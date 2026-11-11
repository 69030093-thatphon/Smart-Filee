use crate::categories::categorize;
use crate::rules::{rule_folder, CustomRule};
use chrono::{DateTime, Local};
use smart_filee_common::{PARTIAL_EXTS, SKIP_FILES};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrganizeMode {
    ByType,
    ByTypeAndMonth,
    ByMonth,
    PhotosByDateTaken,
}

impl OrganizeMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ByType => "By type",
            Self::ByTypeAndMonth => "By type + month",
            Self::ByMonth => "By month",
            Self::PhotosByDateTaken => "Photos by date taken",
        }
    }
}

impl std::str::FromStr for OrganizeMode {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "By type + month" => Self::ByTypeAndMonth,
            "By month" => Self::ByMonth,
            "Photos by date taken" => Self::PhotosByDateTaken,
            _ => Self::ByType,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizePlanItem {
    pub src: PathBuf,
    pub category: String,
    pub size: u64,
    pub dest_dir: PathBuf,
}

/// Plan file organization for top-level regular files of a folder.
/// Returns plan items sorted case-insensitively by filename.
pub fn plan_organize(
    folder: &Path,
    mode: OrganizeMode,
    rules: &[CustomRule],
) -> Result<Vec<OrganizePlanItem>, std::io::Error> {
    let mut plan = Vec::new();
    let entries = fs::read_dir(folder)?;

    let mut paths: Vec<PathBuf> = Vec::new();
    for entry in entries.flatten() {
        paths.push(entry.path());
    }

    // Sort case-insensitively by filename matching Python sorted(folder.iterdir(), key=lambda x: x.name.lower())
    paths.sort_by_cached_key(|p| {
        p.file_name()
            .map_or_else(String::new, |n| n.to_string_lossy().to_lowercase())
    });

    for p in paths {
        // Skip symlinks and non-files
        let meta = match p.symlink_metadata() {
            Ok(m) => {
                if m.file_type().is_symlink() || !m.is_file() {
                    continue;
                }
                m
            }
            Err(_) => continue,
        };

        let file_name = p
            .file_name()
            .map_or_else(String::new, |n| n.to_string_lossy().to_string());
        let lower_name = file_name.to_lowercase();

        // Skip hidden files, system files, and in-progress download extensions
        if file_name.starts_with('.') || SKIP_FILES.contains(&lower_name.as_str()) {
            continue;
        }
        if let Some(ext) = p.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            if PARTIAL_EXTS.contains(&ext_str.as_str()) {
                continue;
            }
        }

        // Determine category (custom rule priority over built-in category)
        let cat = rule_folder(&file_name, rules).unwrap_or_else(|| categorize(&p).to_string());

        // Determine month YYYY-MM from mtime
        let mtime = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        let dt: DateTime<Local> = mtime.into();
        let month = dt.format("%Y-%m").to_string();

        let dest = match mode {
            OrganizeMode::ByMonth => folder.join(&month),
            OrganizeMode::ByTypeAndMonth => folder.join(&cat).join(&month),
            OrganizeMode::PhotosByDateTaken if cat == "Images" => {
                let taken = smart_filee_image_engine::exif_date(&p);
                let photo_month = if let Some(dt) = taken {
                    dt.format("%Y-%m").to_string()
                } else {
                    month
                };
                folder.join("Images").join(&photo_month)
            }
            _ => folder.join(&cat),
        };

        plan.push(OrganizePlanItem {
            src: p,
            category: cat,
            size: meta.len(),
            dest_dir: dest,
        });
    }

    Ok(plan)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_plan_organize_modes() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        fs::write(root.join("photo.jpg"), "jpg data").unwrap();
        fs::write(root.join("report.pdf"), "pdf data").unwrap();
        fs::write(root.join("temp.crdownload"), "partial").unwrap();
        fs::write(root.join(".DS_Store"), "junk").unwrap();

        let rules = vec![CustomRule::new("report*", "Reports")];

        // Mode: ByType
        let plan_by_type = plan_organize(root, OrganizeMode::ByType, &rules).unwrap();
        assert_eq!(plan_by_type.len(), 2);

        let photo_plan = plan_by_type
            .iter()
            .find(|i| i.category == "Images")
            .unwrap();
        assert_eq!(photo_plan.dest_dir, root.join("Images"));

        let report_plan = plan_by_type
            .iter()
            .find(|i| i.category == "Reports")
            .unwrap();
        assert_eq!(report_plan.dest_dir, root.join("Reports"));

        // Mode: ByMonth
        let plan_by_month = plan_organize(root, OrganizeMode::ByMonth, &rules).unwrap();
        assert_eq!(plan_by_month.len(), 2);
        assert!(plan_by_month[0].dest_dir.to_string_lossy().contains('-'));
    }
}
