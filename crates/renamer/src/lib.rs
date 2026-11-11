use serde::{Deserialize, Serialize};
use smart_filee_common::{protected_reason, CancellationToken};
use smart_filee_file_engine::is_illegal_name;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaseTransform {
    Keep,
    Lowercase,
    Uppercase,
    TitleCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpaceTransform {
    Keep,
    Underscore,
    Dash,
    Remove,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RenameStatus {
    OK,
    Unchanged,
    Invalid,
    Protected,
    Conflict,
}

impl RenameStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OK => "OK",
            Self::Unchanged => "Unchanged",
            Self::Invalid => "Invalid",
            Self::Protected => "Protected",
            Self::Conflict => "Conflict",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenamePlanItem {
    pub original: PathBuf,
    pub new_name: String,
    pub status: RenameStatus,
}

#[derive(Debug, Clone)]
pub struct RenameOptions<'a> {
    pub find: &'a str,
    pub replace: &'a str,
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub case: CaseTransform,
    pub spaces: SpaceTransform,
    pub number: bool,
    pub start: usize,
    pub pad: usize,
}

impl Default for RenameOptions<'_> {
    fn default() -> Self {
        Self {
            find: "",
            replace: "",
            prefix: "",
            suffix: "",
            case: CaseTransform::Keep,
            spaces: SpaceTransform::Keep,
            number: false,
            start: 1,
            pad: 3,
        }
    }
}

fn to_title_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut capitalize_next = true;

    for c in s.chars() {
        if c.is_alphanumeric() {
            if capitalize_next {
                for upper in c.to_uppercase() {
                    result.push(upper);
                }
                capitalize_next = false;
            } else {
                for lower in c.to_lowercase() {
                    result.push(lower);
                }
            }
        } else {
            result.push(c);
            capitalize_next = true;
        }
    }

    result
}

/// Compute rename plan with case conversion, space replacement, numbering, and conflict detection.
pub fn plan_rename(files: &[PathBuf], opts: &RenameOptions) -> Vec<RenamePlanItem> {
    let mut plan = Vec::new();

    // 1. Initial string transformation & validity check
    for (i, p) in files.iter().enumerate() {
        let original_name = p
            .file_name()
            .map_or_else(String::new, |n| n.to_string_lossy().to_string());
        let stem = p
            .file_stem()
            .map_or_else(String::new, |s| s.to_string_lossy().to_string());
        let ext = p
            .extension()
            .map_or_else(String::new, |e| format!(".{}", e.to_string_lossy()));

        let mut s = stem;

        // Find and replace in stem
        if !opts.find.is_empty() {
            s = s.replace(opts.find, opts.replace);
        }

        // Space transforms
        match opts.spaces {
            SpaceTransform::Underscore => {
                s = s.split_whitespace().collect::<Vec<_>>().join("_");
            }
            SpaceTransform::Dash => {
                s = s.split_whitespace().collect::<Vec<_>>().join("-");
            }
            SpaceTransform::Remove => {
                s = s.split_whitespace().collect::<Vec<_>>().join("");
            }
            SpaceTransform::Keep => {}
        }

        // Case transforms
        match opts.case {
            CaseTransform::Lowercase => {
                s = s.to_lowercase();
            }
            CaseTransform::Uppercase => {
                s = s.to_uppercase();
            }
            CaseTransform::TitleCase => {
                s = to_title_case(&s);
            }
            CaseTransform::Keep => {}
        }

        // Prefix and suffix
        s = format!("{}{s}{}", opts.prefix, opts.suffix);

        // Numbering
        if opts.number {
            s = format!("{s}_{:0width$}", opts.start + i, width = opts.pad);
        }

        let new_name = format!("{s}{ext}");

        let status = if new_name == original_name {
            RenameStatus::Unchanged
        } else if s.trim().is_empty() || is_illegal_name(&new_name) {
            RenameStatus::Invalid
        } else {
            RenameStatus::OK
        };

        plan.push(RenamePlanItem {
            original: p.clone(),
            new_name,
            status,
        });
    }

    // 2. Protected folder check
    for item in &mut plan {
        if item.status == RenameStatus::OK {
            if let Some(parent) = item.original.parent() {
                if protected_reason(parent).is_some() {
                    item.status = RenameStatus::Protected;
                }
            }
        }
    }

    // 3. Conflict detection (intra-batch and on-disk)
    let mut new_keys_count: HashMap<(String, String), usize> = HashMap::new();
    let mut old_keys: HashMap<(String, String), PathBuf> = HashMap::new();

    for item in &plan {
        let parent = item
            .original
            .parent()
            .map_or_else(String::new, |p| p.to_string_lossy().to_string());
        let key = (parent.clone(), item.new_name.to_lowercase());
        *new_keys_count.entry(key).or_insert(0) += 1;

        let orig_name = item
            .original
            .file_name()
            .map_or_else(String::new, |n| n.to_string_lossy().to_lowercase());
        old_keys.insert((parent, orig_name), item.original.clone());
    }

    for item in &mut plan {
        if item.status != RenameStatus::OK {
            continue;
        }

        let parent = item
            .original
            .parent()
            .map_or_else(PathBuf::new, Path::to_path_buf);
        let parent_str = parent.to_string_lossy().to_string();
        let new_key = (parent_str.clone(), item.new_name.to_lowercase());

        let orig_name = item
            .original
            .file_name()
            .map_or_else(String::new, |n| n.to_string_lossy().to_lowercase());
        let own_key = (parent_str, orig_name);

        // Multiple files in batch mapped to the exact same new name
        if new_keys_count.get(&new_key).copied().unwrap_or(0) > 1 {
            item.status = RenameStatus::Conflict;
        } else if new_key != own_key {
            // Clashes with another file's current name in the batch
            if old_keys.contains_key(&new_key) {
                item.status = RenameStatus::Conflict;
            } else if parent.join(&item.new_name).exists() {
                // Clashes with an existing file on disk
                item.status = RenameStatus::Conflict;
            }
        }
    }

    plan
}

/// Execute renaming for planned items whose status is `OK`.
pub fn apply_rename<F>(
    plan: &[RenamePlanItem],
    cancel: &CancellationToken,
    mut progress: F,
) -> (Vec<(PathBuf, PathBuf)>, Vec<String>)
where
    F: FnMut(usize, usize),
{
    let mut moves = Vec::new();
    let mut errors = Vec::new();
    let total = plan.len();

    for (i, item) in plan.iter().enumerate() {
        if item.status != RenameStatus::OK {
            continue;
        }

        if cancel.is_cancelled() {
            break;
        }

        if (i + 1) % 10 == 0 || i + 1 == total {
            progress(i + 1, total);
        }

        let Some(parent) = item.original.parent() else {
            errors.push(format!("{}: missing parent path", item.original.display()));
            continue;
        };

        let dest = parent.join(&item.new_name);

        match fs::rename(&item.original, &dest) {
            Ok(()) => {
                moves.push((item.original.clone(), dest));
            }
            Err(e) => {
                errors.push(format!("{}: {e}", item.original.display()));
            }
        }
    }

    (moves, errors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_plan_rename_transforms() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        let f1 = root.join("my old file.txt");
        let f2 = root.join("sample.pdf");
        fs::write(&f1, "1").unwrap();
        fs::write(&f2, "2").unwrap();

        let mut opts = RenameOptions::default();
        opts.find = "old";
        opts.replace = "new";
        opts.spaces = SpaceTransform::Underscore;
        opts.case = CaseTransform::TitleCase;
        opts.prefix = "PRE_";
        opts.number = true;
        opts.pad = 2;

        let plan = plan_rename(&[f1.clone(), f2.clone()], &opts);
        assert_eq!(plan.len(), 2);

        assert_eq!(plan[0].status, RenameStatus::OK);
        assert_eq!(plan[0].new_name, "PRE_My_New_File_01.txt");

        assert_eq!(plan[1].status, RenameStatus::OK);
        assert_eq!(plan[1].new_name, "PRE_Sample_02.pdf");
    }

    #[test]
    fn test_plan_rename_conflicts() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        let f1 = root.join("fileA.txt");
        let existing = root.join("fixed.txt");
        fs::write(&f1, "a").unwrap();
        fs::write(&existing, "exists").unwrap();

        // 1. Rename to an existing on-disk file -> Conflict
        let mut opts = RenameOptions::default();
        opts.find = "fileA";
        opts.replace = "fixed";
        let plan = plan_rename(std::slice::from_ref(&f1), &opts);
        assert_eq!(plan[0].status, RenameStatus::Conflict);

        // 2. Intra-batch duplicate -> Conflict
        let f2 = root.join("another.txt");
        fs::write(&f2, "b").unwrap();

        let mut opts_same = RenameOptions::default();
        opts_same.find = "another";
        opts_same.replace = "fileA"; // maps to fileA.txt, causing collision
        let plan_dup = plan_rename(&[f1, f2], &opts_same);
        assert!(plan_dup.iter().any(|i| i.status == RenameStatus::Conflict));
    }
}
