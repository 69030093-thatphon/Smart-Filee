use crate::paths::unique_dest;
use smart_filee_common::{protected_reason, CancellationToken};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

pub const DEFAULT_CATEGORY_DIRS: &[&str] = &[
    "Images",
    "Documents",
    "Archives",
    "Installers",
    "Audio",
    "Videos",
    "Code",
    "Others",
    "_Duplicates",
    "_Similar",
];

#[derive(Debug, Default, Clone)]
pub struct MoveResult {
    pub moves: Vec<(PathBuf, PathBuf)>,
    pub skipped: usize,
    pub errors: Vec<String>,
}

#[derive(Debug, Default, Clone)]
pub struct UndoResult {
    pub restored: usize,
    pub skipped: usize,
    pub errors: Vec<String>,
}

/// Safely moves files into target directories without overwriting.
/// Revalidates protected paths immediately before execution.
pub fn move_files<F>(
    pairs: &[(PathBuf, PathBuf)],
    cancel: &CancellationToken,
    mut progress: F,
) -> MoveResult
where
    F: FnMut(usize, usize),
{
    let mut result = MoveResult::default();
    let total = pairs.len();

    for (i, (src, dest_dir)) in pairs.iter().enumerate() {
        if cancel.is_cancelled() {
            break;
        }

        if (i + 1) % 10 == 0 || i + 1 == total {
            progress(i + 1, total);
        }

        // 1. Path safety revalidation
        if let Some(src_parent) = src.parent() {
            if let Some(reason) = protected_reason(src_parent) {
                result
                    .errors
                    .push(format!("{}: refused, source is in {reason}", src.display()));
                continue;
            }
        }
        if let Some(reason) = protected_reason(dest_dir) {
            result.errors.push(format!(
                "{}: refused, destination is in {reason}",
                src.display()
            ));
            continue;
        }

        // 2. Source existence check
        if !src.exists() {
            result.skipped += 1;
            continue;
        }

        // 3. Prepare destination directory
        if let Err(e) = fs::create_dir_all(dest_dir) {
            result.errors.push(format!(
                "{}: cannot create destination folder: {e}",
                src.display()
            ));
            continue;
        }

        // 4. Resolve unique destination path
        let file_name = src
            .file_name()
            .map_or_else(String::new, |n| n.to_string_lossy().to_string());
        let dest = unique_dest(dest_dir, &file_name);

        // 5. Execute move with cross-device fallback
        match fs::rename(src, &dest) {
            Ok(()) => {
                result.moves.push((src.clone(), dest));
            }
            Err(_) => {
                // Fallback for cross-device moves: copy then remove
                match fs::copy(src, &dest) {
                    Ok(_) => match fs::remove_file(src) {
                        Ok(()) => {
                            result.moves.push((src.clone(), dest));
                        }
                        Err(e) => {
                            // If source removal failed, clean up copy to avoid duplication
                            let _ = fs::remove_file(&dest);
                            result.errors.push(format!(
                                "{}: failed to remove source after copy: {e}",
                                src.display()
                            ));
                        }
                    },
                    Err(e) => {
                        result.errors.push(format!("{}: {e}", src.display()));
                    }
                }
            }
        }
    }

    result
}

/// Revert previously performed moves in reverse order.
/// Refuses to overwrite files if source already exists.
pub fn undo_moves<F>(
    moves: &[(PathBuf, PathBuf)],
    extra_dirs: &[&str],
    cancel: &CancellationToken,
    mut progress: F,
) -> UndoResult
where
    F: FnMut(usize, usize),
{
    let mut result = UndoResult::default();
    let total = moves.len();

    // Revert in reverse order
    for (i, (src, dest)) in moves.iter().rev().enumerate() {
        if cancel.is_cancelled() {
            break;
        }

        if (i + 1) % 10 == 0 || i + 1 == total {
            progress(i + 1, total);
        }

        // Skip if destination file vanished or source file already exists
        if !dest.exists() || src.exists() {
            result.skipped += 1;
            continue;
        }

        if let Some(src_parent) = src.parent() {
            if let Err(e) = fs::create_dir_all(src_parent) {
                result.errors.push(format!(
                    "{}: cannot recreate source directory: {e}",
                    dest.display()
                ));
                continue;
            }
        }

        // Revert move
        match fs::rename(dest, src) {
            Ok(()) => {
                result.restored += 1;
                if let Some(dest_parent) = dest.parent() {
                    cleanup_empty_dirs(dest_parent, extra_dirs);
                }
            }
            Err(_) => {
                // Cross-device copy fallback
                match fs::copy(dest, src) {
                    Ok(_) => match fs::remove_file(dest) {
                        Ok(()) => {
                            result.restored += 1;
                            if let Some(dest_parent) = dest.parent() {
                                cleanup_empty_dirs(dest_parent, extra_dirs);
                            }
                        }
                        Err(e) => {
                            result.errors.push(format!("{}: {e}", dest.display()));
                        }
                    },
                    Err(e) => {
                        result.errors.push(format!("{}: {e}", dest.display()));
                    }
                }
            }
        }
    }

    result
}

/// Clean up empty parent directories created for categories or months.
pub fn cleanup_empty_dirs(dir: &Path, extra_dirs: &[&str]) {
    let mut allowed_set: HashSet<&str> = DEFAULT_CATEGORY_DIRS.iter().copied().collect();
    for extra in extra_dirs {
        allowed_set.insert(extra);
    }

    let mut current = dir.to_path_buf();
    while let Some(name) = current.file_name() {
        let name_str = name.to_string_lossy();
        let is_month = name_str.len() == 7
            && name_str.chars().nth(4) == Some('-')
            && name_str[..4].chars().all(|c| c.is_ascii_digit())
            && name_str[5..].chars().all(|c| c.is_ascii_digit());

        if allowed_set.contains(name_str.as_ref()) || is_month {
            // Attempt remove_dir (only succeeds if directory is completely empty)
            if fs::remove_dir(&current).is_err() {
                break;
            }
            if let Some(parent) = current.parent() {
                current = parent.to_path_buf();
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_move_and_undo_lifecycle() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        let src_file = root.join("document.pdf");
        fs::write(&src_file, "sample content").unwrap();

        let dest_folder = root.join("Documents");
        let pairs = vec![(src_file.clone(), dest_folder.clone())];

        let cancel = CancellationToken::new();
        let move_res = move_files(&pairs, &cancel, |_, _| {});

        assert_eq!(move_res.moves.len(), 1);
        assert_eq!(move_res.skipped, 0);
        assert_eq!(move_res.errors.len(), 0);
        assert!(!src_file.exists());

        let moved_file = dest_folder.join("document.pdf");
        assert!(moved_file.exists());

        // Now undo the move
        let undo_res = undo_moves(&move_res.moves, &[], &cancel, |_, _| {});
        assert_eq!(undo_res.restored, 1);
        assert_eq!(undo_res.skipped, 0);
        assert_eq!(undo_res.errors.len(), 0);

        assert!(src_file.exists());
        assert!(!moved_file.exists());
        // Verify empty category folder was tidied up
        assert!(!dest_folder.exists());
    }

    #[test]
    fn test_undo_refuses_to_overwrite_existing_file() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        let src_file = root.join("notes.txt");
        fs::write(&src_file, "original").unwrap();

        let dest_folder = root.join("Documents");
        let pairs = vec![(src_file.clone(), dest_folder.clone())];

        let cancel = CancellationToken::new();
        let move_res = move_files(&pairs, &cancel, |_, _| {});
        assert_eq!(move_res.moves.len(), 1);

        // Simulate a new file being created at the original location before undo
        fs::write(&src_file, "new conflicting file").unwrap();

        // Undo must not overwrite the new file
        let undo_res = undo_moves(&move_res.moves, &[], &cancel, |_, _| {});
        assert_eq!(undo_res.restored, 0);
        assert_eq!(undo_res.skipped, 1);

        // Original new file untouched
        assert_eq!(
            fs::read_to_string(&src_file).unwrap(),
            "new conflicting file"
        );
    }
}
