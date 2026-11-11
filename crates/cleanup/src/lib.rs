//! System cleanup and space reclamation engine for Smart-Filee.
//!
//! Identifies temporary/junk files, large unused files, and empty directory trees.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use glob::Pattern;
use serde::{Deserialize, Serialize};
use smart_filee_common::{CancellationToken, SYSTEM_DIRS};
use smart_filee_file_engine::traversal::walk_files;
use walkdir::WalkDir;

/// Known temporary file patterns with their required minimum ages in days.
pub const TEMP_PATTERNS: &[(&str, f64)] = &[
    ("*.tmp", 1.0),
    ("*.temp", 1.0),
    ("~$*", 1.0),
    ("thumbs.db", 1.0),
    (".ds_store", 1.0),
    ("*.dmp", 1.0),
    ("*.chk", 1.0),
    ("*.crdownload", 7.0),
    ("*.part", 7.0),
];

/// The category of a cleanup candidate item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CleanupKind {
    #[serde(rename = "temp")]
    Temp,
    #[serde(rename = "old_large")]
    OldLarge,
    #[serde(rename = "empty_dir")]
    EmptyDir,
}

impl CleanupKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Temp => "temp",
            Self::OldLarge => "old_large",
            Self::EmptyDir => "empty_dir",
        }
    }
}

/// A candidate item identified for cleanup.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CleanupItem {
    pub kind: CleanupKind,
    pub path: PathBuf,
    pub size: u64,
    pub mtime: f64,
}

/// Outcome of the cleanup scan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupResult {
    pub items: Vec<CleanupItem>,
    pub scanned: usize,
    pub cancelled: bool,
    pub unreadable: usize,
}

/// Scans `root` for temporary files, old large files, and empty directories without modifying anything.
pub fn find_cleanup<F>(
    root: impl AsRef<Path>,
    min_mb: u64,
    old_days: f64,
    cancel: &CancellationToken,
    mut progress: F,
) -> CleanupResult
where
    F: FnMut(&str),
{
    let root = root.as_ref();
    let mut errs = Vec::new();
    let mut items = Vec::new();
    let mut scanned = 0;

    let now_secs = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_or(0.0, |d| d.as_secs_f64());

    // Compile glob patterns once
    let compiled_patterns: Vec<(Pattern, f64)> = TEMP_PATTERNS
        .iter()
        .filter_map(|(pat, age)| Pattern::new(pat).ok().map(|p| (p, *age)))
        .collect();

    let files = walk_files(root, cancel, None, &mut errs, |_| {});

    for file in files {
        scanned += 1;
        let p = file.path;
        let file_name = p
            .file_name()
            .and_then(|s| s.to_str())
            .map(str::to_ascii_lowercase)
            .unwrap_or_default();

        // Calculate age using max(atime, mtime)
        let atime = fs::metadata(&p)
            .ok()
            .and_then(|m| m.accessed().ok())
            .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
            .map_or(file.mtime, |d| d.as_secs_f64());

        let max_time = file.mtime.max(atime);
        let age_days = if now_secs > max_time {
            (now_secs - max_time) / 86_400.0
        } else {
            0.0
        };

        let is_temp = compiled_patterns
            .iter()
            .any(|(pat, min_age)| pat.matches(&file_name) && age_days >= *min_age);

        if is_temp {
            items.push(CleanupItem {
                kind: CleanupKind::Temp,
                path: p,
                size: file.size,
                mtime: file.mtime,
            });
        } else if file.size >= min_mb * 1024 * 1024 && age_days >= old_days {
            items.push(CleanupItem {
                kind: CleanupKind::OldLarge,
                path: p,
                size: file.size,
                mtime: file.mtime,
            });
        }

        if scanned % 1000 == 0 {
            progress(&format!("Scanning… {scanned} files"));
        }
    }

    // Identify empty folders using post-order (bottom-up) traversal
    let mut nonempty: HashSet<PathBuf> = HashSet::new();
    let mut empty: HashSet<PathBuf> = HashSet::new();

    let walker = WalkDir::new(root)
        .contents_first(true)
        .follow_links(false)
        .into_iter();

    for entry_res in walker {
        if cancel.is_cancelled() {
            break;
        }

        match entry_res {
            Ok(entry) => {
                if !entry.file_type().is_dir() {
                    continue;
                }
                let cur = entry.path().to_path_buf();
                let dir_name = entry.file_name().to_string_lossy();

                if SYSTEM_DIRS.contains(&dir_name.as_ref()) {
                    nonempty.insert(cur);
                    continue;
                }

                // Check directory contents
                let mut has_files = false;
                let mut has_busy_kids = false;

                if let Ok(rd) = fs::read_dir(&cur) {
                    for child in rd.flatten() {
                        let child_path = child.path();
                        if child.file_type().is_ok_and(|ft| ft.is_symlink()) {
                            has_busy_kids = true;
                            break;
                        }
                        if child.file_type().is_ok_and(|ft| ft.is_file()) {
                            has_files = true;
                            break;
                        }
                        if nonempty.contains(&child_path) {
                            has_busy_kids = true;
                            break;
                        }
                    }
                } else {
                    // Unreadable folder: treat as nonempty
                    nonempty.insert(cur.clone());
                    errs.push(format!("Unreadable directory: {}", cur.display()));
                    continue;
                }

                if has_files || has_busy_kids {
                    nonempty.insert(cur);
                } else if cur != root {
                    empty.insert(cur);
                }
            }
            Err(e) => {
                errs.push(e.to_string());
            }
        }
    }

    // Report only the top-most empty folder (i.e. parent is not in empty)
    let mut sorted_empty: Vec<PathBuf> = empty.iter().cloned().collect();
    sorted_empty.sort();

    for d in &sorted_empty {
        let parent_is_empty = d.parent().is_some_and(|p| empty.contains(p));
        if !parent_is_empty {
            let mtime = fs::metadata(d)
                .ok()
                .and_then(|m| m.modified().ok())
                .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                .map_or(0.0, |dur| dur.as_secs_f64());

            items.push(CleanupItem {
                kind: CleanupKind::EmptyDir,
                path: d.clone(),
                size: 0,
                mtime,
            });
        }
    }

    CleanupResult {
        items,
        scanned,
        cancelled: cancel.is_cancelled(),
        unreadable: errs.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_find_cleanup_empty_dir() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        let empty_sub = root.join("EmptyFolder");
        fs::create_dir_all(&empty_sub).unwrap();

        let filled_sub = root.join("FilledFolder");
        fs::create_dir_all(&filled_sub).unwrap();
        fs::write(filled_sub.join("active.txt"), "content").unwrap();

        let cancel = CancellationToken::new();
        let res = find_cleanup(root, 100, 180.0, &cancel, |_| {});

        assert_eq!(res.items.len(), 1);
        assert_eq!(res.items[0].kind, CleanupKind::EmptyDir);
        assert_eq!(res.items[0].path, empty_sub);
    }

    #[test]
    fn test_find_cleanup_temp_and_old_large() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        let tmp_file = root.join("test.tmp");
        fs::write(&tmp_file, "temporary data").unwrap();

        // Artificially age the file to 2 days ago (both atime and mtime)
        let two_days_ago = SystemTime::now() - std::time::Duration::from_secs(2 * 86_400);
        let ft = filetime::FileTime::from_system_time(two_days_ago);
        filetime::set_file_times(&tmp_file, ft, ft).unwrap();

        let cancel = CancellationToken::new();
        let res = find_cleanup(root, 100, 180.0, &cancel, |_| {});

        assert!(res
            .items
            .iter()
            .any(|it| it.kind == CleanupKind::Temp && it.path == tmp_file));
    }

    #[test]
    fn test_cleanup_parity_with_python() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        let age_fn = |path: &Path, days: u64| {
            let past = SystemTime::now() - std::time::Duration::from_secs(days * 86_400);
            let ft = filetime::FileTime::from_system_time(past);
            filetime::set_file_times(path, ft, ft).unwrap();
        };

        let old_tmp = root.join("a.tmp");
        fs::write(&old_tmp, "old").unwrap();
        age_fn(&old_tmp, 3);

        let fresh_tmp = root.join("b.tmp");
        fs::write(&fresh_tmp, "fresh").unwrap();

        let big = root.join("movie.bin");
        fs::write(&big, vec![0u8; 2_000_000]).unwrap();
        age_fn(&big, 400);

        let recent_big = root.join("recent.bin");
        fs::write(&recent_big, vec![0u8; 2_000_000]).unwrap();

        let e1 = root.join("e1");
        let e2 = e1.join("e2");
        fs::create_dir_all(&e2).unwrap();

        let full = root.join("full");
        fs::create_dir_all(&full).unwrap();
        fs::write(full.join("keep.txt"), "keep").unwrap();

        let cancel = CancellationToken::new();
        let res = find_cleanup(root, 1, 180.0, &cancel, |_| {});

        let mut got: Vec<(&str, String)> = res
            .items
            .iter()
            .map(|it| {
                (
                    it.kind.as_str(),
                    it.path.file_name().unwrap().to_string_lossy().to_string(),
                )
            })
            .collect();
        got.sort();

        let mut expected = vec![
            ("empty_dir", "e1".to_string()),
            ("old_large", "movie.bin".to_string()),
            ("temp", "a.tmp".to_string()),
        ];
        expected.sort();

        assert_eq!(got, expected);
    }
}
