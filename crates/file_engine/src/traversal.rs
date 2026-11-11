use crate::metadata::FileItem;
use smart_filee_common::{
    should_skip_dir, should_skip_file, CancellationToken, SKIP_DIRS, SYSTEM_DIRS,
};
use std::path::Path;
use walkdir::WalkDir;

/// Walk a directory tree and collect regular files matching the filtering policy.
/// Skips symlinks, protected/system directories, and transient junk files.
/// Unreadable file errors are accumulated in `errors` instead of aborting.
pub fn walk_files<F>(
    root: &Path,
    cancel: &CancellationToken,
    custom_skip_dirs: Option<&[&str]>,
    errors: &mut Vec<String>,
    mut progress: F,
) -> Vec<FileItem>
where
    F: FnMut(usize),
{
    let mut items = Vec::new();
    let skip_set = custom_skip_dirs.unwrap_or(SKIP_DIRS);

    let walker = WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|entry| {
            if entry.depth() == 0 {
                return true;
            }
            let name = entry.file_name().to_string_lossy();
            if entry.file_type().is_dir() {
                if skip_set.contains(&name.as_ref()) || SYSTEM_DIRS.contains(&name.as_ref()) {
                    return false;
                }
                if should_skip_dir(&name) {
                    return false;
                }
            }
            true
        });

    for entry_result in walker {
        if cancel.is_cancelled() {
            break;
        }

        let entry = match entry_result {
            Ok(e) => e,
            Err(err) => {
                errors.push(err.to_string());
                continue;
            }
        };

        // Skip root itself and directories
        if entry.depth() == 0 || entry.file_type().is_dir() {
            continue;
        }

        // Never follow or process symlinks
        if entry.path_is_symlink() {
            continue;
        }

        let name = entry.file_name().to_string_lossy();
        if should_skip_file(&name) {
            continue;
        }

        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(e) => {
                errors.push(format!("{}: {e}", entry.path().display()));
                continue;
            }
        };

        if !meta.is_file() {
            continue;
        }

        items.push(FileItem::from_path_and_metadata(
            entry.path().to_path_buf(),
            &meta,
        ));

        if items.len() % 500 == 0 {
            progress(items.len());
        }
    }

    items
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_walk_files_basics() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        fs::write(root.join("file1.txt"), "hello").unwrap();
        fs::write(root.join("file2.pdf"), "pdf content").unwrap();
        fs::write(root.join("desktop.ini"), "junk").unwrap();
        fs::write(root.join(".hidden"), "hidden").unwrap();

        let sub = root.join("subfolder");
        fs::create_dir(&sub).unwrap();
        fs::write(sub.join("subfile.doc"), "document").unwrap();

        let skip_dir = root.join("_Duplicates");
        fs::create_dir(&skip_dir).unwrap();
        fs::write(skip_dir.join("dup.txt"), "duplicate").unwrap();

        let cancel = CancellationToken::new();
        let mut errors = Vec::new();
        let files = walk_files(root, &cancel, None, &mut errors, |_| {});

        assert_eq!(errors.len(), 0);
        let names: Vec<String> = files.iter().map(FileItem::file_name).collect();

        assert!(names.contains(&"file1.txt".to_string()));
        assert!(names.contains(&"file2.pdf".to_string()));
        assert!(names.contains(&"subfile.doc".to_string()));

        // Check that skipped files/dirs are excluded
        assert!(!names.contains(&"desktop.ini".to_string()));
        assert!(!names.contains(&".hidden".to_string()));
        assert!(!names.contains(&"dup.txt".to_string()));
    }

    #[test]
    fn test_walk_files_cancellation() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        for i in 0..100 {
            fs::write(root.join(format!("file_{i}.txt")), "data").unwrap();
        }

        let cancel = CancellationToken::new();
        cancel.cancel(); // Pre-cancelled

        let mut errors = Vec::new();
        let files = walk_files(root, &cancel, None, &mut errors, |_| {});

        assert_eq!(files.len(), 0);
    }
}
