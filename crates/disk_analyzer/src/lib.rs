//! Disk space analyzer for Smart-Filee.
//!
//! Provides file category breakdown, top-level folder totals,
//! and bounded top-N largest file inspection.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use smart_filee_common::CancellationToken;
use smart_filee_file_engine::traversal::walk_files;
use smart_filee_organizer::categorize;

/// Item representing a large file found during disk analysis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiskFileItem {
    pub size: u64,
    pub path: PathBuf,
    pub mtime: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct HeapEntry {
    size: u64,
    path: PathBuf,
    mtime: f64,
}

impl Eq for HeapEntry {}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse comparison on size for min-heap behavior
        other
            .size
            .cmp(&self.size)
            .then_with(|| self.path.cmp(&other.path))
    }
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Result of disk analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskAnalysisResult {
    /// Top largest files, sorted descending by size.
    pub top: Vec<DiskFileItem>,
    /// Aggregated file size by category.
    pub categories: HashMap<String, u64>,
    /// Aggregated file size by top-level folder name or `"(files in folder)"`.
    pub folders: HashMap<String, u64>,
    /// Total number of files scanned.
    pub scanned: usize,
    /// Total byte count of all scanned files.
    pub total: u64,
    /// Whether analysis was cancelled prematurely.
    pub cancelled: bool,
    /// Number of unreadable files/folders encountered.
    pub unreadable: usize,
}

/// Analyzes disk usage under `root`, gathering category distribution, folder totals, and the top `limit` largest files.
pub fn analyze_disk<F>(
    root: impl AsRef<Path>,
    limit: usize,
    cancel: &CancellationToken,
    mut progress: F,
) -> DiskAnalysisResult
where
    F: FnMut(&str),
{
    let root = root.as_ref();
    let mut errs = Vec::new();
    let mut scanned = 0;
    let mut total: u64 = 0;
    let mut categories: HashMap<String, u64> = HashMap::new();
    let mut folders: HashMap<String, u64> = HashMap::new();

    // Min-heap to retain the top `limit` largest files.
    let mut heap: BinaryHeap<HeapEntry> = BinaryHeap::new();

    let items = walk_files(root, cancel, None, &mut errs, |c| {
        if c % 500 == 0 {
            progress(&format!("Scanning… {c} files"));
        }
    });

    for item in items {
        scanned += 1;
        let size = item.size;
        total = total.saturating_add(size);
        let p = item.path;

        let cat = categorize(&p).to_string();
        *categories.entry(cat).or_default() += size;

        let key = if let Ok(rel) = p.strip_prefix(root) {
            let mut comps = rel.components();
            let first = comps.next();
            let second = comps.next();
            if second.is_some() {
                first.map_or_else(
                    || "(files in folder)".to_string(),
                    |c| c.as_os_str().to_string_lossy().to_string(),
                )
            } else {
                "(files in folder)".to_string()
            }
        } else {
            "(files in folder)".to_string()
        };
        *folders.entry(key).or_default() += size;

        let mtime = item.mtime;
        if limit > 0 {
            if heap.len() < limit {
                heap.push(HeapEntry {
                    size,
                    path: p,
                    mtime,
                });
            } else if let Some(min_top) = heap.peek() {
                if size > min_top.size {
                    heap.pop();
                    heap.push(HeapEntry {
                        size,
                        path: p,
                        mtime,
                    });
                }
            }
        }
    }

    let mut top: Vec<DiskFileItem> = heap
        .into_iter()
        .map(|entry| DiskFileItem {
            size: entry.size,
            path: entry.path,
            mtime: entry.mtime,
        })
        .collect();

    // Sort descending by size
    top.sort_by_key(|a| std::cmp::Reverse(a.size));

    DiskAnalysisResult {
        top,
        categories,
        folders,
        scanned,
        total,
        cancelled: cancel.is_cancelled(),
        unreadable: errs.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_disk_analyzer_aggregates_correctly() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        // Direct files in root
        fs::write(root.join("root_doc.txt"), "hello world").unwrap(); // 11 bytes, Documents
        fs::write(root.join("root_img.png"), vec![0u8; 100]).unwrap(); // 100 bytes, Images

        // Subfolders
        let sub = root.join("SubDir");
        fs::create_dir_all(&sub).unwrap();
        fs::write(sub.join("nested.mp4"), vec![0u8; 500]).unwrap(); // 500 bytes, Videos

        let cancel = CancellationToken::new();
        let res = analyze_disk(root, 10, &cancel, |_| {});

        assert_eq!(res.scanned, 3);
        assert_eq!(res.total, 11 + 100 + 500);
        assert_eq!(res.categories.get("Documents").copied().unwrap_or(0), 11);
        assert_eq!(res.categories.get("Images").copied().unwrap_or(0), 100);
        assert_eq!(res.categories.get("Videos").copied().unwrap_or(0), 500);

        assert_eq!(
            res.folders.get("(files in folder)").copied().unwrap_or(0),
            111
        );
        assert_eq!(res.folders.get("SubDir").copied().unwrap_or(0), 500);

        assert_eq!(res.top.len(), 3);
        assert_eq!(res.top[0].size, 500);
        assert_eq!(res.top[1].size, 100);
        assert_eq!(res.top[2].size, 11);
    }
}
