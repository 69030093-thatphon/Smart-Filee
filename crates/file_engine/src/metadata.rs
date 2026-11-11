use serde::{Deserialize, Serialize};
use std::fs::Metadata;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileItem {
    pub path: PathBuf,
    pub size: u64,
    pub mtime: f64,
    pub is_symlink: bool,
}

impl FileItem {
    pub fn from_path_and_metadata(path: PathBuf, meta: &Metadata) -> Self {
        let size = meta.len();
        let mtime = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
            .map_or(0.0, |d| d.as_secs_f64());

        Self {
            path,
            size,
            mtime,
            is_symlink: meta.file_type().is_symlink(),
        }
    }

    pub fn file_name(&self) -> String {
        self.path
            .file_name()
            .map_or_else(String::new, |n| n.to_string_lossy().to_string())
    }

    pub fn extension(&self) -> String {
        self.path
            .extension()
            .map_or_else(String::new, |ext| ext.to_string_lossy().to_lowercase())
    }

    pub fn parent(&self) -> Option<&Path> {
        self.path.parent()
    }
}
