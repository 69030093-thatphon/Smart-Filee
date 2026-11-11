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
