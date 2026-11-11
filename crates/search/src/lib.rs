use glob::Pattern;
use serde::{Deserialize, Serialize};
use smart_filee_common::CancellationToken;
use smart_filee_file_engine::walk_files;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResultItem {
    pub path: PathBuf,
    pub size: u64,
    pub mtime: f64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub results: Vec<SearchResultItem>,
    pub scanned: usize,
    pub truncated: bool,
    pub unreadable: usize,
}

/// Search files by substring or wildcard pattern with minimum size filter.
pub fn search_files<F>(
    root: &Path,
    query: &str,
    min_size: u64,
    max_results: usize,
    cancel: &CancellationToken,
    mut progress: F,
) -> SearchResult
where
    F: FnMut(usize, usize),
{
    let q = query.trim().to_lowercase();
    let has_wildcard = q.chars().any(|c| c == '*' || c == '?' || c == '[');
    let pattern = if has_wildcard {
        Pattern::new(&q).ok()
    } else {
        None
    };

    let mut errors = Vec::new();
    let items = walk_files(root, cancel, None, &mut errors, |_| {});
    let scanned = items.len();

    let mut results = Vec::new();
    let mut truncated = false;

    for item in items {
        if cancel.is_cancelled() {
            break;
        }

        if item.size < min_size {
            continue;
        }

        let name = item.file_name().to_lowercase();
        let matches = if q.is_empty() {
            true
        } else if let Some(ref pat) = pattern {
            pat.matches(&name)
        } else {
            name.contains(&q)
        };

        if matches {
            results.push(SearchResultItem {
                path: item.path,
                size: item.size,
                mtime: item.mtime,
            });

            if results.len() % 50 == 0 {
                progress(scanned, results.len());
            }

            if results.len() >= max_results {
                truncated = true;
                break;
            }
        }
    }

    SearchResult {
        results,
        scanned,
        truncated,
        unreadable: errors.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_search_substring_and_wildcard() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        fs::write(root.join("quarterly_report.pdf"), "content").unwrap();
        fs::write(root.join("annual_report.xlsx"), "content").unwrap();
        fs::write(root.join("notes.txt"), "content").unwrap();

        let cancel = CancellationToken::new();

        // 1. Substring search "report"
        let res1 = search_files(root, "report", 0, 100, &cancel, |_, _| {});
        assert_eq!(res1.results.len(), 2);
        assert!(!res1.truncated);

        // 2. Wildcard search "*.pdf"
        let res2 = search_files(root, "*.pdf", 0, 100, &cancel, |_, _| {});
        assert_eq!(res2.results.len(), 1);
        assert_eq!(
            res2.results[0].path.file_name().unwrap(),
            "quarterly_report.pdf"
        );

        // 3. Size filter
        let res3 = search_files(root, "", 1000, 100, &cancel, |_, _| {});
        assert_eq!(res3.results.len(), 0);
    }
}
