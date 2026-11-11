//! Differential one-way backup engine for Smart-Filee.
//!
//! Synchronizes new and modified files safely without deleting destination files,
//! enforcing destination safety and timestamp preservation.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use smart_filee_common::{protected_reason, CancellationToken, SYSTEM_DIRS};
use smart_filee_file_engine::traversal::walk_files;

/// The reason a file is included in the backup plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackupReason {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "changed")]
    Changed,
}

impl BackupReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::New => "new",
            Self::Changed => "changed",
        }
    }
}

/// A single planned file copy operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupPlanItem {
    pub src: PathBuf,
    pub target: PathBuf,
    pub size: u64,
    pub reason: BackupReason,
}

/// Outcome of planning a differential backup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupPlanResult {
    pub plan: Vec<BackupPlanItem>,
    pub scanned: usize,
    pub bytes: u64,
    pub cancelled: bool,
    pub unreadable: usize,
}

/// Execution outcome of running a differential backup plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupRunResult {
    pub copied: usize,
    pub bytes: u64,
    pub errors: Vec<String>,
    pub cancelled: bool,
}

/// Plans a one-way differential backup from `src` to `dst`.
///
/// Refuses to write to protected directories or if `dst` is the same as or inside `src`.
pub fn plan_backup<F>(
    src: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    cancel: &CancellationToken,
    mut progress: F,
) -> anyhow::Result<BackupPlanResult>
where
    F: FnMut(&str),
{
    let src_raw = src.as_ref();
    let dst_raw = dst.as_ref();

    if !src_raw.is_dir() {
        anyhow::bail!("Source folder does not exist.");
    }

    let src = src_raw
        .canonicalize()
        .unwrap_or_else(|_| src_raw.to_path_buf());
    let dst = dst_raw
        .canonicalize()
        .unwrap_or_else(|_| dst_raw.to_path_buf());

    if src == dst || dst.starts_with(&src) {
        anyhow::bail!("The destination must not be the source folder or inside it.");
    }

    if let Some(reason) = protected_reason(&dst) {
        anyhow::bail!("Refusing to write a backup into {reason}.");
    }

    let mut plan = Vec::new();
    let mut scanned = 0;
    let mut total_bytes: u64 = 0;
    let mut errs = Vec::new();

    let items = walk_files(&src, cancel, Some(SYSTEM_DIRS), &mut errs, |_| {});

    for item in items {
        scanned += 1;
        let p = item.path;
        let Ok(rel) = p.strip_prefix(&src) else {
            continue;
        };
        let target = dst.join(rel);

        let mut reason = None;
        if !target.exists() {
            reason = Some(BackupReason::New);
        } else if let Ok(ts) = fs::metadata(&target) {
            let target_mtime = ts
                .modified()
                .ok()
                .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                .map_or(0.0, |d| d.as_secs_f64());

            // Modified if size differs or source mtime is newer by more than 2.0 seconds
            if ts.len() != item.size || item.mtime > target_mtime + 2.0 {
                reason = Some(BackupReason::Changed);
            }
        } else {
            reason = Some(BackupReason::New);
        }

        if let Some(r) = reason {
            total_bytes = total_bytes.saturating_add(item.size);
            plan.push(BackupPlanItem {
                src: p,
                target,
                size: item.size,
                reason: r,
            });
        }

        if scanned % 500 == 0 {
            progress(&format!("Comparing… {scanned} files"));
        }
    }

    Ok(BackupPlanResult {
        plan,
        scanned,
        bytes: total_bytes,
        cancelled: cancel.is_cancelled(),
        unreadable: errs.len(),
    })
}

/// Executes a backup plan by copying new and modified files.
/// Preserves file modification times and never deletes anything.
pub fn run_backup<F>(
    plan: &[BackupPlanItem],
    cancel: &CancellationToken,
    mut progress: F,
) -> BackupRunResult
where
    F: FnMut(&str),
{
    let mut copied = 0;
    let mut nbytes = 0;
    let mut errors = Vec::new();

    for (i, item) in plan.iter().enumerate() {
        if cancel.is_cancelled() {
            break;
        }

        if let Some(parent) = item.target.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                errors.push(format!("{}: {e}", item.src.display()));
                continue;
            }
        }

        match fs::copy(&item.src, &item.target) {
            Ok(_) => {
                // Preserve modification timestamp
                if let Ok(meta) = fs::metadata(&item.src) {
                    if let Ok(mtime) = meta.modified() {
                        let ft = filetime::FileTime::from_system_time(mtime);
                        let _ = filetime::set_file_mtime(&item.target, ft);
                    }
                }
                copied += 1;
                nbytes += item.size;
            }
            Err(e) => {
                let name = item.src.file_name().unwrap_or_default().to_string_lossy();
                errors.push(format!("{name}: {e}"));
            }
        }

        if (i + 1) % 20 == 0 || i + 1 == plan.len() {
            progress(&format!("Copying {}/{}…", i + 1, plan.len()));
        }
    }

    BackupRunResult {
        copied,
        bytes: nbytes,
        errors,
        cancelled: cancel.is_cancelled(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_backup_refuses_self_or_nested_destination() {
        let dir = tempdir().unwrap();
        let src = dir.path();
        let cancel = CancellationToken::new();

        // Target == source
        assert!(plan_backup(src, src, &cancel, |_| {}).is_err());

        // Target inside source
        let sub = src.join("NestedBackup");
        fs::create_dir_all(&sub).unwrap();
        assert!(plan_backup(src, &sub, &cancel, |_| {}).is_err());
    }

    #[test]
    fn test_backup_plan_and_run_differential() {
        let src_dir = tempdir().unwrap();
        let dst_dir = tempdir().unwrap();
        let src = src_dir.path();
        let dst = dst_dir.path();

        let f1 = src.join("doc1.txt");
        let f2 = src.join("doc2.txt");
        fs::write(&f1, "file one").unwrap();
        fs::write(&f2, "file two").unwrap();

        let cancel = CancellationToken::new();
        let plan_res = plan_backup(src, dst, &cancel, |_| {}).unwrap();

        assert_eq!(plan_res.plan.len(), 2);
        assert_eq!(plan_res.plan[0].reason, BackupReason::New);

        // Run backup
        let run_res = run_backup(&plan_res.plan, &cancel, |_| {});
        assert_eq!(run_res.copied, 2);
        assert!(dst.join("doc1.txt").exists());
        assert!(dst.join("doc2.txt").exists());

        // Second plan: files are unchanged, so plan should be empty
        let plan_res2 = plan_backup(src, dst, &cancel, |_| {}).unwrap();
        assert_eq!(plan_res2.plan.len(), 0);
    }
}
