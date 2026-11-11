use std::path::{Path, PathBuf};

pub const SYSTEM_DIRS: &[&str] = &["System Volume Information", "$Recycle.Bin", "$RECYCLE.BIN"];
pub const SKIP_DIRS: &[&str] = &[
    "System Volume Information",
    "$Recycle.Bin",
    "$RECYCLE.BIN",
    "_Duplicates",
    "_Similar",
];
pub const SKIP_FILES: &[&str] = &["desktop.ini", "thumbs.db", ".DS_Store"];
pub const PARTIAL_EXTS: &[&str] = &["crdownload", "part", "tmp", "download", "partial"];

/// Check if a path is protected against modification (system root or system folder).
/// Returns Some("reason") if protected, or None if safe.
pub fn protected_reason(path: &Path) -> Option<String> {
    let resolved = match path.canonicalize() {
        Ok(p) => p,
        Err(_) => path.to_path_buf(),
    };

    // Check if path is a filesystem or drive root
    if let Some(parent) = resolved.parent() {
        if parent == resolved {
            return Some("a drive or filesystem root".to_string());
        }
    } else {
        return Some("a drive or filesystem root".to_string());
    }

    // Windows system folder checks
    #[cfg(windows)]
    {
        let vars = [
            "WINDIR",
            "SystemRoot",
            "ProgramFiles",
            "ProgramFiles(x86)",
            "ProgramData",
        ];
        for var in vars {
            if let Ok(val) = std::env::var(var) {
                let cand = PathBuf::from(val);
                let cand_resolved = cand.canonicalize().unwrap_or(cand);
                if resolved == cand_resolved || resolved.starts_with(&cand_resolved) {
                    return Some(format!("a system folder ({})", cand_resolved.display()));
                }
            }
        }
    }

    // Unix / macOS system folder checks
    #[cfg(not(windows))]
    {
        let cands = [
            "/bin",
            "/boot",
            "/dev",
            "/etc",
            "/lib",
            "/lib64",
            "/proc",
            "/sbin",
            "/sys",
            "/usr",
            "/System",
            "/Library",
            "/Applications",
        ];
        for cand in cands {
            let cand_path = PathBuf::from(cand);
            let cand_resolved = cand_path.canonicalize().unwrap_or(cand_path);
            if resolved == cand_resolved || resolved.starts_with(&cand_resolved) {
                return Some(format!("a system folder ({})", cand_resolved.display()));
            }
        }
    }

    None
}

/// Check if a directory name should be skipped during recursive traversal.
pub fn should_skip_dir(name: &str) -> bool {
    SKIP_DIRS.contains(&name) || name.starts_with('.')
}

/// Check if a file name should be skipped during scanning.
pub fn should_skip_file(name: &str) -> bool {
    let lower = name.to_lowercase();
    SKIP_FILES.contains(&lower.as_str()) || name.starts_with('.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filesystem_root_protected() {
        #[cfg(not(windows))]
        {
            assert!(protected_reason(Path::new("/")).is_some());
            assert!(protected_reason(Path::new("/etc")).is_some());
            assert!(protected_reason(Path::new("/usr/bin")).is_some());
        }

        #[cfg(windows)]
        {
            assert!(protected_reason(Path::new("C:\\")).is_some());
        }
    }

    #[test]
    fn test_skip_lists() {
        assert!(should_skip_dir("System Volume Information"));
        assert!(should_skip_dir("$Recycle.Bin"));
        assert!(should_skip_dir("_Duplicates"));
        assert!(should_skip_dir(".git"));
        assert!(!should_skip_dir("Documents"));

        assert!(should_skip_file("desktop.ini"));
        assert!(should_skip_file("thumbs.db"));
        assert!(should_skip_file(".DS_Store"));
        assert!(!should_skip_file("report.pdf"));
    }
}
