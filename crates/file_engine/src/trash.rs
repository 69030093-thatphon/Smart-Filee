use smart_filee_common::{protected_reason, CancellationToken};
use std::path::PathBuf;

/// Send files to the OS Recycle Bin / Trash.
/// Refuses to recycle files inside protected or system directories.
pub fn trash_files<F>(
    paths: &[PathBuf],
    cancel: &CancellationToken,
    mut progress: F,
) -> (Vec<PathBuf>, Vec<String>)
where
    F: FnMut(usize, usize),
{
    let mut done = Vec::new();
    let mut errors = Vec::new();
    let total = paths.len();

    for (i, path) in paths.iter().enumerate() {
        if cancel.is_cancelled() {
            break;
        }

        if (i + 1) % 10 == 0 || i + 1 == total {
            progress(i + 1, total);
        }

        // Validate parent directory is not protected
        if let Some(parent) = path.parent() {
            if let Some(reason) = protected_reason(parent) {
                let name = path.file_name().map_or_else(
                    || path.display().to_string(),
                    |n| n.to_string_lossy().to_string(),
                );
                errors.push(format!("{name}: refused, {reason}"));
                continue;
            }
        }

        match trash::delete(path) {
            Ok(()) => {
                done.push(path.clone());
            }
            Err(e) => {
                let name = path.file_name().map_or_else(
                    || path.display().to_string(),
                    |n| n.to_string_lossy().to_string(),
                );
                errors.push(format!("{name}: {e}"));
            }
        }
    }

    (done, errors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_trash_files_safe() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("to_delete.txt");
        fs::write(&file, "temporary content").unwrap();

        let cancel = CancellationToken::new();
        let (done, errors) = trash_files(std::slice::from_ref(&file), &cancel, |_, _| {});

        // On headless / CI without a trash bin, trash::delete may succeed or return an error,
        // but it must never panic.
        if done.is_empty() {
            assert_eq!(errors.len(), 1);
        } else {
            assert_eq!(done.len(), 1);
            assert_eq!(done[0], file);
            assert!(!file.exists());
        }
    }
}
