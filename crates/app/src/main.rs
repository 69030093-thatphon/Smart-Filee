mod cli;
mod logging;

use smart_filee_common::{fmt_bytes, fmt_duration, protected_reason, CancellationToken};
use smart_filee_file_engine::{is_illegal_name, move_files, undo_moves, unique_dest, walk_files};
use smart_filee_localization::{current_language, set_language, tr, Language};
use smart_filee_organizer::{categorize, plan_organize, CustomRule, OrganizeMode};
use smart_filee_renamer::{apply_rename, plan_rename, RenameOptions, RenameStatus};
use smart_filee_search::search_files;
use std::fs;
use std::path::Path;
use tracing::info;

fn main() -> anyhow::Result<()> {
    let args = cli::parse_args();

    if args.help {
        cli::print_help();
        return Ok(());
    }

    if args.version {
        cli::print_version();
        return Ok(());
    }

    logging::init_logging();
    info!("Initializing Smart-Filee v{}", env!("CARGO_PKG_VERSION"));

    // Attempt to load .env if present
    if let Ok(path) = dotenvy::dotenv() {
        info!("Loaded environment from {}", path.display());
    }

    // Set configured language
    if let Some(ref lang_code) = args.lang {
        let lang = Language::from_code(lang_code);
        set_language(lang);
        info!("Language set to: {}", lang.display_name());
    }

    // Automated headless self-test for CI and startup validation
    if args.selftest {
        info!("Running automated headless self-test...");
        run_selftest()?;
        info!("All self-tests passed successfully!");
        return Ok(());
    }

    info!("Smart-Filee foundation initialized successfully.");
    info!(
        "Current active language: {}",
        current_language().display_name()
    );
    info!("Translated greeting: {}", tr("Dashboard"));

    Ok(())
}

fn run_selftest() -> anyhow::Result<()> {
    // 1. Validate common formatters
    anyhow::ensure!(fmt_bytes(0) == "0.0 B", "fmt_bytes(0) failed");
    anyhow::ensure!(fmt_bytes(1024) == "1.0 KB", "fmt_bytes(1024) failed");
    anyhow::ensure!(
        fmt_bytes(1024 * 1024 * 1024) == "1.0 GB",
        "fmt_bytes(1GB) failed"
    );

    let dur = fmt_duration(3665, ("d", "h", "m"));
    anyhow::ensure!(dur == "1h 1m", "fmt_duration failed: got {dur}");

    // 2. Validate cancellation token
    let token = CancellationToken::new();
    anyhow::ensure!(!token.is_cancelled(), "New token should not be cancelled");
    token.cancel();
    anyhow::ensure!(token.is_cancelled(), "Cancelled token should be true");
    token.reset();
    anyhow::ensure!(!token.is_cancelled(), "Reset token should be false");

    // 3. Validate path guards
    #[cfg(not(windows))]
    {
        anyhow::ensure!(
            protected_reason(Path::new("/")).is_some(),
            "Filesystem root must be protected"
        );
        anyhow::ensure!(
            protected_reason(Path::new("/etc")).is_some(),
            "System folder /etc must be protected"
        );
    }
    #[cfg(windows)]
    {
        anyhow::ensure!(
            protected_reason(Path::new("C:\\")).is_some(),
            "Windows drive root C:\\ must be protected"
        );
    }

    // 4. Validate localization
    set_language(Language::English);
    anyhow::ensure!(tr("Dashboard") == "Dashboard", "EN translation failed");

    set_language(Language::Thai);
    anyhow::ensure!(tr("Dashboard") == "แดชบอร์ด", "TH translation failed");
    anyhow::ensure!(
        tr("File Organizer") == "จัดระเบียบไฟล์",
        "TH translation failed"
    );

    // 5. Validate file engine (traversal, unique_dest, moves, undo)
    let temp_dir = std::env::temp_dir().join(format!("sfm_selftest_{}", std::process::id()));
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir)?;

    let test_file = temp_dir.join("test.txt");
    fs::write(&test_file, "selftest content")?;

    anyhow::ensure!(
        !is_illegal_name("valid.txt"),
        "valid.txt should be recognized as legal"
    );
    anyhow::ensure!(
        is_illegal_name("invalid?.txt"),
        "invalid?.txt should be recognized as illegal"
    );

    let cand1 = unique_dest(&temp_dir, "test.txt");
    anyhow::ensure!(
        cand1 == temp_dir.join("test (1).txt"),
        "unique_dest should resolve to test (1).txt"
    );

    let cancel = CancellationToken::new();
    let mut errs = Vec::new();
    let found = walk_files(&temp_dir, &cancel, None, &mut errs, |_| {});
    anyhow::ensure!(
        found.len() == 1,
        "walk_files should find exactly 1 file, found {}",
        found.len()
    );

    let target_dir = temp_dir.join("Documents");
    let pairs = vec![(test_file.clone(), target_dir.clone())];
    let move_res = move_files(&pairs, &cancel, |_, _| {});
    anyhow::ensure!(move_res.moves.len() == 1, "move_files failed to move file");
    anyhow::ensure!(!test_file.exists(), "Source file should no longer exist");

    let undo_res = undo_moves(&move_res.moves, &[], &cancel, |_, _| {});
    anyhow::ensure!(undo_res.restored == 1, "undo_moves failed to restore file");
    anyhow::ensure!(test_file.exists(), "Source file should be restored");

    // 6. Validate Organizer, Search, Renamer
    anyhow::ensure!(
        categorize(Path::new("pic.jpg")) == "Images",
        "categorize failed"
    );
    anyhow::ensure!(
        categorize(Path::new("doc.pdf")) == "Documents",
        "categorize failed"
    );

    let doc_file = temp_dir.join("invoice_may.pdf");
    fs::write(&doc_file, "invoice")?;

    let rules = vec![CustomRule::new("invoice*", "Finance")];
    let org_plan = plan_organize(&temp_dir, OrganizeMode::ByType, &rules)?;
    anyhow::ensure!(
        org_plan.iter().any(|i| i.category == "Finance"),
        "Organizer custom rule matching failed"
    );

    let search_res = search_files(&temp_dir, "*.pdf", 0, 10, &cancel, |_, _| {});
    anyhow::ensure!(
        search_res.results.len() == 1,
        "search_files wildcard failed"
    );

    let mut rn_opts = RenameOptions::default();
    rn_opts.find = "may";
    rn_opts.replace = "june";
    let rn_plan = plan_rename(std::slice::from_ref(&doc_file), &rn_opts);
    anyhow::ensure!(
        rn_plan[0].status == RenameStatus::OK,
        "Rename planning status failed"
    );
    anyhow::ensure!(
        rn_plan[0].new_name == "invoice_june.pdf",
        "Rename new name failed"
    );

    let (rn_moves, _) = apply_rename(&rn_plan, &cancel, |_, _| {});
    anyhow::ensure!(rn_moves.len() == 1, "apply_rename failed");
    anyhow::ensure!(
        temp_dir.join("invoice_june.pdf").exists(),
        "Renamed file does not exist"
    );

    // 7. Validate Duplicate Finder
    let dup_a = temp_dir.join("dup_a.bin");
    let dup_b = temp_dir.join("dup_b.bin");
    fs::write(&dup_a, b"identical duplicate content for hashing")?;
    fs::write(&dup_b, b"identical duplicate content for hashing")?;

    let dup_res = smart_filee_duplicate::find_duplicates(&temp_dir, 1, None, &cancel, |_, _| {});
    anyhow::ensure!(
        dup_res.groups.len() == 1,
        "find_duplicates should detect 1 group, got {}",
        dup_res.groups.len()
    );
    anyhow::ensure!(
        dup_res.groups[0].files.len() == 2,
        "find_duplicates group should have 2 files"
    );

    // 8. Validate Disk Analyzer
    let disk_res = smart_filee_disk_analyzer::analyze_disk(&temp_dir, 10, &cancel, |_| {});
    anyhow::ensure!(
        disk_res.scanned >= 2,
        "analyze_disk should have scanned at least 2 files, scanned {}",
        disk_res.scanned
    );
    anyhow::ensure!(
        disk_res.total > 0,
        "analyze_disk total bytes should be greater than 0"
    );

    // 9. Validate Image Engine (Fingerprint & Distance)
    let fp_a = smart_filee_image_engine::Fingerprint {
        h_dhash: 0xAAAA_BBBB_CCCC_DDDD,
        v_dhash: 0x1111_2222_3333_4444,
        ahash: 0x5555_6666_7777_8888,
        mean_brightness: 120.0,
    };
    let mut fp_b = fp_a;
    fp_b.h_dhash ^= 0b11; // 2 bit difference
    anyhow::ensure!(
        smart_filee_image_engine::is_close(&fp_a, &fp_b, 6),
        "Fingerprints with 2 bit difference should be close"
    );

    // 10. Validate Cleanup Engine
    let empty_dir = temp_dir.join("EmptyFolder");
    fs::create_dir_all(&empty_dir)?;
    let clean_res = smart_filee_cleanup::find_cleanup(&temp_dir, 100, 180.0, &cancel, |_| {});
    anyhow::ensure!(
        clean_res
            .items
            .iter()
            .any(|it| it.kind == smart_filee_cleanup::CleanupKind::EmptyDir),
        "find_cleanup should detect empty directory"
    );

    // 11. Validate Differential Backup
    let backup_src = temp_dir.join("BackupSrc");
    let backup_dst = temp_dir.join("BackupDst");
    fs::create_dir_all(&backup_src)?;
    fs::create_dir_all(&backup_dst)?;
    fs::write(backup_src.join("data.txt"), "backup data")?;

    let backup_plan = smart_filee_backup::plan_backup(&backup_src, &backup_dst, &cancel, |_| {})?;
    anyhow::ensure!(
        backup_plan.plan.len() == 1,
        "plan_backup should plan 1 file, planned {}",
        backup_plan.plan.len()
    );
    let backup_run = smart_filee_backup::run_backup(&backup_plan.plan, &cancel, |_| {});
    anyhow::ensure!(
        backup_run.copied == 1,
        "run_backup should have copied 1 file, copied {}",
        backup_run.copied
    );
    anyhow::ensure!(
        backup_dst.join("data.txt").exists(),
        "Backup destination file should exist"
    );

    // 12. Validate System Monitor
    let mut sys_mon = smart_filee_system_monitor::SystemMonitor::new();
    let snap = sys_mon.snapshot();
    anyhow::ensure!(!snap.os_name.is_empty(), "OS name should be detected");
    anyhow::ensure!(
        snap.memory_total_bytes > 0,
        "Total memory should be positive"
    );
    let procs = sys_mon.list_processes(None, smart_filee_system_monitor::ProcessSort::Memory);
    anyhow::ensure!(!procs.is_empty(), "Process list should not be empty");

    // 13. Validate Network Tools
    let my_ip = smart_filee_network::local_ip();
    anyhow::ensure!(!my_ip.is_empty(), "Local IP should not be empty");
    anyhow::ensure!(
        smart_filee_network::valid_host("127.0.0.1"),
        "127.0.0.1 should be valid host"
    );
    anyhow::ensure!(
        !smart_filee_network::valid_host("-flag"),
        "-flag should be invalid host"
    );

    // 14. Validate Tidy Score
    anyhow::ensure!(
        smart_filee_common::tidy_score(0, 0, 0, 0, 0, 0) == 100,
        "Clean tidy score should be 100"
    );

    // 15. Validate Database Engine
    let db = smart_filee_database::Database::open(":memory:")?;
    db.add_log("Selftest database initialized");
    let logs = db.recent_logs(10, None);
    anyhow::ensure!(logs.len() == 1, "Database should contain 1 log entry");
    let stats = db.stats();
    anyhow::ensure!(
        stats.log_lines == 1,
        "Database stats should reflect 1 log entry"
    );

    // 16. Validate Auth Config & Loopback
    let auth_cfg = smart_filee_auth::SupabaseConfig::new(
        smart_filee_auth::DEFAULT_SUPABASE_URL,
        smart_filee_auth::DEFAULT_SUPABASE_PUBLISHABLE_KEY,
    )?;
    anyhow::ensure!(
        !auth_cfg.publishable_key.is_empty(),
        "Publishable key must be valid"
    );

    let _ = fs::remove_dir_all(&temp_dir);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selftest_executes_cleanly() {
        assert!(run_selftest().is_ok());
    }
}
