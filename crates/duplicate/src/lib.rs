use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use smart_filee_common::CancellationToken;
use smart_filee_file_engine::walk_files;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Instant;

pub const HEAD_HASH_LIMIT: usize = 65536; // 64 KB
pub const CHUNK_SIZE: usize = 1024 * 1024; // 1 MB

pub trait DuplicateHashCache {
    fn get(&mut self, path: &Path, size: u64, mtime: f64, kind: &str) -> Option<String>;
    fn put(&mut self, path: &Path, size: u64, mtime: f64, kind: &str, digest: &str);
    fn flush(&mut self);
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DuplicateFileItem {
    pub path: PathBuf,
    pub mtime: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DuplicateGroup {
    pub size: u64,
    /// Oldest file is index 0 (the copy to keep); subsequent items are duplicates
    pub files: Vec<DuplicateFileItem>,
}

impl DuplicateGroup {
    pub fn wasted_bytes(&self) -> u64 {
        if self.files.len() > 1 {
            self.size * (self.files.len() as u64 - 1)
        } else {
            0
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DuplicateResult {
    pub groups: Vec<DuplicateGroup>,
    pub scanned: usize,
    pub unreadable: usize,
    pub cancelled: bool,
    pub seconds: f64,
}

/// Compute SHA-256 hash of a file or its first `limit` bytes.
pub fn hash_file(
    path: &Path,
    limit: Option<usize>,
    cancel: &CancellationToken,
) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0u8; CHUNK_SIZE];

    if let Some(max_bytes) = limit {
        let mut remaining = max_bytes;
        while remaining > 0 {
            if cancel.is_cancelled() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Interrupted,
                    "Operation cancelled",
                ));
            }
            let to_read = remaining.min(buffer.len());
            let n = file.read(&mut buffer[..to_read])?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
            remaining -= n;
        }
    } else {
        loop {
            if cancel.is_cancelled() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Interrupted,
                    "Operation cancelled",
                ));
            }
            let n = file.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }
    }

    Ok(format!("{:x}", hasher.finalize()))
}

fn cached_hash(
    path: &Path,
    size: u64,
    mtime: f64,
    kind: &str,
    limit: Option<usize>,
    cache: &mut Option<&mut dyn DuplicateHashCache>,
    cancel: &CancellationToken,
) -> Option<String> {
    if let Some(ref mut c) = cache {
        if let Some(digest) = c.get(path, size, mtime, kind) {
            return Some(digest);
        }
    }

    match hash_file(path, limit, cancel) {
        Ok(digest) => {
            if let Some(ref mut c) = cache {
                c.put(path, size, mtime, kind, &digest);
            }
            Some(digest)
        }
        Err(_) => None,
    }
}

/// Staged duplicate file detection matching Python `core.py::find_duplicates`:
/// - File size grouping
/// - 64 KB head hash
/// - Full SHA-256 verification (1 MB chunk streaming)
///
/// The oldest copy by mtime is placed first in each group.
pub fn find_duplicates<F>(
    root: &Path,
    min_size: u64,
    mut cache: Option<&mut dyn DuplicateHashCache>,
    cancel: &CancellationToken,
    mut progress: F,
) -> DuplicateResult
where
    F: FnMut(usize, &str),
{
    let start_time = Instant::now();
    let mut errors = Vec::new();

    let items = walk_files(root, cancel, None, &mut errors, |count| {
        progress(count, "Indexing files…");
    });
    let scanned = items.len();

    // Stage 1: Group by file size
    let mut by_size: HashMap<u64, Vec<DuplicateFileItem>> = HashMap::new();
    let threshold = min_size.max(1);

    for item in items {
        if item.size >= threshold {
            by_size
                .entry(item.size)
                .or_default()
                .push(DuplicateFileItem {
                    path: item.path,
                    mtime: item.mtime,
                });
        }
    }

    let candidates: Vec<(u64, Vec<DuplicateFileItem>)> = by_size
        .into_iter()
        .filter(|(_, files)| files.len() >= 2)
        .collect();

    let mut groups = Vec::new();
    let total_candidates = candidates.len();

    for (cand_idx, (size, files)) in candidates.into_iter().enumerate() {
        if cancel.is_cancelled() {
            break;
        }

        if (cand_idx + 1) % 10 == 0 || cand_idx + 1 == total_candidates {
            progress(
                cand_idx + 1,
                &format!(
                    "Comparing candidates {}/{}…",
                    cand_idx + 1,
                    total_candidates
                ),
            );
        }

        // Stage 2: 64 KB head hash
        let mut by_head: HashMap<String, Vec<DuplicateFileItem>> = HashMap::new();
        for file in files {
            if let Some(head_digest) = cached_hash(
                &file.path,
                size,
                file.mtime,
                "head",
                Some(HEAD_HASH_LIMIT),
                &mut cache,
                cancel,
            ) {
                by_head.entry(head_digest).or_default().push(file);
            }
        }

        // Stage 3: Full SHA-256 verification
        for head_files in by_head.into_values() {
            if head_files.len() < 2 {
                continue;
            }

            let mut by_full: HashMap<String, Vec<DuplicateFileItem>> = HashMap::new();
            for file in head_files {
                if let Some(full_digest) = cached_hash(
                    &file.path, size, file.mtime, "full", None, &mut cache, cancel,
                ) {
                    by_full.entry(full_digest).or_default().push(file);
                }
            }

            for mut group_files in by_full.into_values() {
                if group_files.len() >= 2 {
                    // Sort oldest first (smallest mtime = copy to keep)
                    group_files.sort_by(|a, b| {
                        a.mtime
                            .partial_cmp(&b.mtime)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });
                    groups.push(DuplicateGroup {
                        size,
                        files: group_files,
                    });
                }
            }
        }
    }

    if let Some(ref mut c) = cache {
        c.flush();
    }

    // Sort duplicate groups descending by wasted bytes
    groups.sort_by_key(|a| std::cmp::Reverse(a.wasted_bytes()));

    DuplicateResult {
        groups,
        scanned,
        unreadable: errors.len(),
        cancelled: cancel.is_cancelled(),
        seconds: start_time.elapsed().as_secs_f64(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_duplicate_staged_detection() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        let f1 = root.join("file1.bin");
        let f2 = root.join("file2.bin");
        let f3 = root.join("diff.bin");

        let content = vec![42u8; 100_000]; // 100 KB
        fs::write(&f1, &content).unwrap();
        fs::write(&f2, &content).unwrap();

        let mut diff_content = vec![42u8; 100_000];
        diff_content[99_999] = 99; // Same first 64KB head, differs at end
        fs::write(&f3, &diff_content).unwrap();

        let cancel = CancellationToken::new();
        let res = find_duplicates(root, 1, None, &cancel, |_, _| {});

        assert_eq!(res.groups.len(), 1);
        assert_eq!(res.groups[0].size, 100_000);
        assert_eq!(res.groups[0].files.len(), 2);

        let group_paths: Vec<&Path> = res.groups[0]
            .files
            .iter()
            .map(|f| f.path.as_path())
            .collect();
        assert!(group_paths.contains(&f1.as_path()));
        assert!(group_paths.contains(&f2.as_path()));
        assert!(!group_paths.contains(&f3.as_path()));
    }
}
