//! Image processing engine for Smart-Filee.
//!
//! Provides perceptual image fingerprinting (dHash + aHash),
//! visual similarity clustering (pigeonhole + union-find),
//! and EXIF date extraction.

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use chrono::NaiveDateTime;
use image::GenericImageView;
use serde::{Deserialize, Serialize};
use smart_filee_common::CancellationToken;
use smart_filee_file_engine::traversal::walk_files;

pub const IMAGE_EXTS: &[&str] = &["jpg", "jpeg", "png", "bmp", "webp", "tif", "tiff"];
pub const EXIF_EXTS: &[&str] = &["jpg", "jpeg", "tif", "tiff", "webp"];
pub const MAX_IMAGE_BYTES: u64 = 60 * 1024 * 1024; // 60MB

/// 3-way perceptual image fingerprint and dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Fingerprint {
    pub h_dhash: u64,
    pub v_dhash: u64,
    pub ahash: u64,
    pub mean_brightness: f64,
}

/// A photo scanned during similarity analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimilarPhotoItem {
    pub path: PathBuf,
    pub size: u64,
    pub mtime: f64,
    pub width: u32,
    pub height: u32,
}

/// A cluster of similar images, sorted with the best copy (largest resolution, then size) first.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarGroup {
    pub items: Vec<SimilarPhotoItem>,
    pub wasted_bytes: u64,
}

/// Output of `find_similar_images`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarImagesResult {
    pub groups: Vec<SimilarGroup>,
    pub scanned: usize,
    pub cancelled: bool,
    pub unreadable: usize,
}

/// Computes a 3-way perceptual fingerprint for an image at `path`.
pub fn fingerprint(path: impl AsRef<Path>) -> anyhow::Result<(Fingerprint, (u32, u32))> {
    let path = path.as_ref();
    let img = image::open(path)?;
    let (width, height) = img.dimensions();
    let gray = img.into_luma8();
    let resized = image::imageops::resize(&gray, 9, 9, image::imageops::FilterType::Lanczos3);
    let raw = resized.as_raw();

    // 8x8 block mean
    let mut sum: f64 = 0.0;
    for y in 0..8 {
        for x in 0..8 {
            sum += f64::from(raw[y * 9 + x]);
        }
    }
    let mean = sum / 64.0;

    let mut hh: u64 = 0;
    let mut hv: u64 = 0;
    let mut ah: u64 = 0;

    for y in 0..8 {
        for x in 0..8 {
            let p = raw[y * 9 + x];
            let p_right = raw[y * 9 + (x + 1)];
            let p_down = raw[(y + 1) * 9 + x];

            hh = (hh << 1) | u64::from(u8::from(p > p_right));
            hv = (hv << 1) | u64::from(u8::from(p > p_down));
            ah = (ah << 1) | u64::from(u8::from(f64::from(p) > mean));
        }
    }

    Ok((
        Fingerprint {
            h_dhash: hh,
            v_dhash: hv,
            ahash: ah,
            mean_brightness: mean,
        },
        (width, height),
    ))
}

/// Determines whether two fingerprints are close enough to be considered similar.
#[inline]
pub fn is_close(a: &Fingerprint, b: &Fingerprint, max_distance: u32) -> bool {
    (a.h_dhash ^ b.h_dhash).count_ones() <= max_distance
        && (a.v_dhash ^ b.v_dhash).count_ones() <= max_distance
        && (a.ahash ^ b.ahash).count_ones() <= max_distance
        && (a.mean_brightness - b.mean_brightness).abs() <= 40.0
}

/// Extracts the date a photo was taken from EXIF `DateTimeOriginal` or `DateTime`.
pub fn exif_date(path: impl AsRef<Path>) -> Option<NaiveDateTime> {
    let path = path.as_ref();
    let ext = path.extension()?.to_str()?.to_ascii_lowercase();
    if !EXIF_EXTS.contains(&ext.as_str()) {
        return None;
    }

    let file = File::open(path).ok()?;
    let mut bufreader = BufReader::new(file);
    let exifreader = exif::Reader::new();
    let exif_data = exifreader.read_from_container(&mut bufreader).ok()?;

    let field = exif_data
        .get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY)
        .or_else(|| exif_data.get_field(exif::Tag::DateTime, exif::In::PRIMARY))?;

    let val_str = field.display_value().to_string();
    let s = val_str.trim();
    if s.len() >= 19 {
        let slice = &s[..19];
        if let Ok(dt) = NaiveDateTime::parse_from_str(slice, "%Y:%m:%d %H:%M:%S") {
            return Some(dt);
        }
        if let Ok(dt) = NaiveDateTime::parse_from_str(slice, "%Y-%m-%d %H:%M:%S") {
            return Some(dt);
        }
    }
    None
}

struct ScannedEntry {
    fp: Fingerprint,
    item: SimilarPhotoItem,
}

fn find_parent(parent: &mut [usize], mut i: usize) -> usize {
    while parent[i] != i {
        parent[i] = parent[parent[i]];
        i = parent[i];
    }
    i
}

/// Finds visually similar photos under `root` using perceptual hashes, bucketing, and union-find.
pub fn find_similar_images<F>(
    root: impl AsRef<Path>,
    max_distance: u32,
    cancel: &CancellationToken,
    mut progress: F,
) -> SimilarImagesResult
where
    F: FnMut(&str),
{
    let root = root.as_ref();
    let mut errs = Vec::new();
    let mut scanned_count = 0;

    let mut hashes: Vec<ScannedEntry> = Vec::new();

    let items = walk_files(root, cancel, None, &mut errs, |_| {});

    for item in items {
        let p = item.path;
        let ext = p
            .extension()
            .and_then(|s| s.to_str())
            .map(str::to_ascii_lowercase)
            .unwrap_or_default();

        if !IMAGE_EXTS.contains(&ext.as_str()) || item.size > MAX_IMAGE_BYTES {
            continue;
        }

        match fingerprint(&p) {
            Ok((fp, (w, h))) => {
                hashes.push(ScannedEntry {
                    fp,
                    item: SimilarPhotoItem {
                        path: p,
                        size: item.size,
                        mtime: item.mtime,
                        width: w,
                        height: h,
                    },
                });
                scanned_count += 1;
                if scanned_count % 50 == 0 {
                    progress(&format!("Reading photos… {scanned_count}"));
                }
            }
            Err(e) => {
                errs.push(e.to_string());
            }
        }
    }

    let n = hashes.len();
    if n == 0 {
        return SimilarImagesResult {
            groups: Vec::new(),
            scanned: 0,
            cancelled: cancel.is_cancelled(),
            unreadable: errs.len(),
        };
    }

    // Union-Find disjoint set
    let mut parent: Vec<usize> = (0..n).collect();

    // Pigeonhole bucketing on h_dhash
    let parts = max_distance + 1;
    let width = 64 / parts + 1;
    let mut buckets: HashMap<(u32, u64), Vec<usize>> = HashMap::new();

    for (idx, entry) in hashes.iter().enumerate() {
        for k in 0..parts {
            let chunk = (entry.fp.h_dhash >> (k * width)) & ((1 << width) - 1);
            buckets.entry((k, chunk)).or_default().push(idx);
        }
    }

    for members in buckets.values() {
        if cancel.is_cancelled() {
            break;
        }
        // Skip degenerate buckets (flat / solid-color images)
        if members.len() < 2 || members.len() > 400 {
            continue;
        }

        for a in 0..members.len() {
            for b in (a + 1)..members.len() {
                let i = members[a];
                let j = members[b];
                if is_close(&hashes[i].fp, &hashes[j].fp, max_distance) {
                    let ri = find_parent(&mut parent, i);
                    let rj = find_parent(&mut parent, j);
                    if ri != rj {
                        parent[ri] = rj;
                    }
                }
            }
        }
    }

    // Group items by root parent
    let mut groups_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..n {
        let root_id = find_parent(&mut parent, i);
        groups_map.entry(root_id).or_default().push(i);
    }

    let mut out: Vec<SimilarGroup> = Vec::new();
    for members in groups_map.values() {
        if members.len() < 2 {
            continue;
        }

        let mut items: Vec<SimilarPhotoItem> = members
            .iter()
            .map(|&idx| hashes[idx].item.clone())
            .collect();

        // Sort: best = largest pixel area (width * height), then largest file size
        items.sort_by(|a, b| {
            let pixels_b = u64::from(b.width) * u64::from(b.height);
            let pixels_a = u64::from(a.width) * u64::from(a.height);
            pixels_b.cmp(&pixels_a).then_with(|| b.size.cmp(&a.size))
        });

        let wasted: u64 = items.iter().skip(1).map(|it| it.size).sum();

        out.push(SimilarGroup {
            items,
            wasted_bytes: wasted,
        });
    }

    // Sort groups descending by wasted bytes
    out.sort_by_key(|a| std::cmp::Reverse(a.wasted_bytes));

    SimilarImagesResult {
        groups: out,
        scanned: scanned_count,
        cancelled: cancel.is_cancelled(),
        unreadable: errs.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{GrayImage, Rgb, RgbImage};
    use tempfile::tempdir;

    #[test]
    fn test_fingerprint_and_distance() {
        let dir = tempdir().unwrap();
        let p1 = dir.path().join("dark.png");
        let p2 = dir.path().join("light.png");

        let dark = RgbImage::from_pixel(64, 64, Rgb([20, 20, 20]));
        let light = RgbImage::from_pixel(64, 64, Rgb([230, 230, 230]));
        dark.save(&p1).unwrap();
        light.save(&p2).unwrap();

        let (fp1, (w1, h1)) = fingerprint(&p1).unwrap();
        let (fp2, (w2, h2)) = fingerprint(&p2).unwrap();

        assert_eq!((w1, h1), (64, 64));
        assert_eq!((w2, h2), (64, 64));

        // Dark vs Light: brightness differs by ~210, so is_close must be false
        assert!(!is_close(&fp1, &fp2, 6));
        // Identity must be close
        assert!(is_close(&fp1, &fp1, 6));
    }

    #[test]
    fn test_find_similar_images_clustering() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        // 1. Horizontal gradient image (orig)
        let mut grad = GrayImage::new(240, 240);
        for (x, _y, pixel) in grad.enumerate_pixels_mut() {
            let val = (255.0 * (1.0 - (f64::from(x)) / 240.0)) as u8;
            *pixel = image::Luma([val]);
        }
        grad.save(root.join("orig.png")).unwrap();

        // 2. Resized gradient image (small)
        let small = image::imageops::resize(&grad, 120, 120, image::imageops::FilterType::Triangle);
        small.save(root.join("small.png")).unwrap();

        // 3. Flat dark image
        let flat_dark = RgbImage::from_pixel(300, 300, Rgb([30, 30, 30]));
        flat_dark.save(root.join("flat_dark.png")).unwrap();

        // 4. Flat light image
        let flat_light = RgbImage::from_pixel(300, 300, Rgb([240, 240, 240]));
        flat_light.save(root.join("flat_light.png")).unwrap();

        // 5. Noise image
        let mut noise = GrayImage::new(240, 240);
        let mut seed: u64 = 7;
        for pixel in noise.pixels_mut() {
            seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
            *pixel = image::Luma([(seed >> 32) as u8]);
        }
        noise.save(root.join("noise.png")).unwrap();

        let cancel = CancellationToken::new();
        let res = find_similar_images(root, 6, &cancel, |_| {});

        // Exactly one group containing orig.png and small.png
        assert_eq!(res.groups.len(), 1);
        let names: Vec<String> = res.groups[0]
            .items
            .iter()
            .map(|it| it.path.file_name().unwrap().to_string_lossy().to_string())
            .collect();

        // The best copy (largest pixel count: orig.png 240x240 vs small.png 120x120) must be first
        assert_eq!(names, vec!["orig.png", "small.png"]);
    }
}
