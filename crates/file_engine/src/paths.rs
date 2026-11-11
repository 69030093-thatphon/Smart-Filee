use std::path::{Path, PathBuf};

/// Generate a unique destination path in `dest_dir` that does not exist on disk.
/// If `name` already exists, appends ` (1)`, ` (2)`, etc. before the extension.
pub fn unique_dest(dest_dir: &Path, name: &str) -> PathBuf {
    let dest = dest_dir.join(name);
    if !dest.exists() {
        return dest;
    }

    let p = Path::new(name);
    let stem = p
        .file_stem()
        .map_or_else(|| name.to_string(), |s| s.to_string_lossy().to_string());
    let suffix = p
        .extension()
        .map_or_else(String::new, |e| format!(".{}", e.to_string_lossy()));

    let mut n = 1;
    loop {
        let cand_name = format!("{stem} ({n}){suffix}");
        let cand = dest_dir.join(&cand_name);
        if !cand.exists() {
            return cand;
        }
        n += 1;
    }
}

/// Check if a filename contains illegal filesystem characters.
pub fn is_illegal_name(name: &str) -> bool {
    if name.is_empty() || name == "." || name == ".." {
        return true;
    }
    if name.ends_with(' ') || name.ends_with('.') {
        return true;
    }
    name.chars().any(|c| match c {
        '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => true,
        c if c < ' ' => true,
        _ => false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_unique_dest_no_collision() {
        let dir = tempdir().unwrap();
        let path = unique_dest(dir.path(), "test.txt");
        assert_eq!(path, dir.path().join("test.txt"));
    }

    #[test]
    fn test_unique_dest_with_collisions() {
        let dir = tempdir().unwrap();
        let original = dir.path().join("report.pdf");
        std::fs::write(&original, "content").unwrap();

        let path1 = unique_dest(dir.path(), "report.pdf");
        assert_eq!(path1, dir.path().join("report (1).pdf"));

        std::fs::write(&path1, "content2").unwrap();
        let path2 = unique_dest(dir.path(), "report.pdf");
        assert_eq!(path2, dir.path().join("report (2).pdf"));
    }

    #[test]
    fn test_illegal_names() {
        assert!(is_illegal_name(""));
        assert!(is_illegal_name("."));
        assert!(is_illegal_name(".."));
        assert!(is_illegal_name("file<name>.txt"));
        assert!(is_illegal_name("file:name.txt"));
        assert!(is_illegal_name("folder/sub"));
        assert!(is_illegal_name("folder\\sub"));
        assert!(is_illegal_name("trailing "));
        assert!(is_illegal_name("trailing."));
        assert!(!is_illegal_name("valid_filename.txt"));
        assert!(!is_illegal_name("My Report (2026).pdf"));
    }
}
