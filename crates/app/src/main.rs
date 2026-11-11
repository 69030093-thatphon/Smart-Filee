mod cli;
mod logging;

use smart_filee_common::{fmt_bytes, fmt_duration, protected_reason, CancellationToken};
use smart_filee_localization::{current_language, set_language, tr, Language};
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
