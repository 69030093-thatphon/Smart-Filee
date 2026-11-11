# Smart-Filee — Autonomous Rust Migration Roadmap

**Project:** Smart-Filee (Rust Edition)  
**Specification Reference:** `AGENT.md` (Sections 12, 20, 22)  
**Last Updated:** 2026-11-12  
**Strategy:** Incremental Migration with Verified Feature Parity  
**Current Phase:** Phase 0 Completed / Entering Phase 1 (Rust Foundation)  

---

## 1. Migration Goals

1. **Complete Runtime Decoupling:** The application must not require Python or Qt at runtime; native Rust executable for Windows 10/11 (macOS and Linux supported).
2. **Feature Parity:** All 14 tools and capabilities of the Python baseline must be fully operational in Rust.
3. **UI/UX Consistency:** Preserve and enhance the Premium Light Violet visual identity using `egui` and `eframe`.
4. **Bilingual Parity:** Support English and Thai with accurate font rendering.
5. **Local-First & Safe:** Preserve SQLite data structures (`sfm.db`), undo batches, zero telemetry, and protected folder guards.
6. **Supabase Cloud Identity:** Retain hybrid cloud authentication (email/password, local loopback OTP callback on port 8765, profile & device tracking).

---

## 2. Phase-by-Phase Roadmap

### Phase 0 — Repository Audit & Architecture (COMPLETED)
- [x] Clone and inspect repository state and branches.
- [x] Run baseline Python test suite (58/58 unit tests passing).
- [x] Audit all Python modules (`core.py`, `db.py`, `app.py`, `widgets.py`, `themes.py`, `i18n.py`, `i18n_th.py`, `membership_client.py`).
- [x] Document SQLite schema and migration requirements.
- [x] Document Supabase Auth integration and loopback server.
- [x] Produce `docs/development/INITIAL_AUDIT.md`.
- [x] Produce `docs/development/FEATURE_INVENTORY.md`.
- [x] Author Rust system architecture and workspace layout (`docs/architecture/`).
- [x] Establish migration roadmap and status tracking.

### Phase 1 — Rust Foundation (COMPLETED)
- [x] Initialize Cargo workspace root (`Cargo.toml`) and toolchain config (`rust-toolchain.toml`).
- [x] Establish crate boundaries (`crates/common`, `crates/localization`, `crates/app`).
- [x] Configure workspace lints (`clippy`), formatting rules (`rustfmt.toml`), and CI checks.
- [x] Implement error handling infrastructure (`thiserror`, `anyhow`).
- [x] Implement application entry point in `crates/app` with CLI argument parsing (supporting `--selftest`).
- [x] Configure structured logging with `tracing` and `tracing-subscriber`.
- [x] Implement basic bilingual string lookup in `crates/localization`.
- [x] Verify workspace compilation, formatting, clippy, and unit test execution.

### Phase 2 — Core File Engine (COMPLETED)
- [x] Implement safe path validation and system folder guards in `crates/file_engine`.
- [x] Implement recursive directory traversal with cancellation token and unreadable error collection.
- [x] Implement collision-free unique destination naming (`stem (n).ext`).
- [x] Implement atomic file moves with cross-device copy fallback and rollback journal.
- [x] Implement persistent undo rollback with empty category/month directory pruning.
- [x] Implement OS Recycle Bin integration via `trash` crate.
- [x] Implement CSV exporter with UTF-8 BOM.
- [x] Add unit tests for path guards, collision resolution, traversal, moves, and undo.

### Phase 3 — Essential File Tools (COMPLETED)
- [x] Implement file categorization and custom rule parsing in `crates/organizer`.
- [x] Implement organization planning (`By type`, `By month`, `By type + month`).
- [x] Implement fast file search with glob wildcards in `crates/search`.
- [x] Implement bulk renamer planning, case transforms, numbering, and conflict detection in `crates/renamer`.
- [x] Add unit tests verifying feature parity against Python `core.py`.

### Phase 4 — Analysis Engines (COMPLETED)
- [x] Implement staged duplicate detection (size grouping → 64 KB head hash → full SHA-256) in `crates/duplicate`.
- [x] Implement disk storage analysis with bounded top 100 min-heap in `crates/disk_analyzer`.
- [x] Implement perceptual image hashing (9x9 Lanczos resize, dhash, ahash, union-find clustering) in `crates/image_engine`.
- [x] Implement EXIF date parser for photo organization via `kamadak-exif`.
- [x] Connect duplicate, image engine, and disk analyzer into CLI selftest.

### Phase 5 — Maintenance and Backup (COMPLETED)
- [x] Implement temporary and junk file detection in `crates/cleanup`.
- [x] Implement old large unused file detection with age thresholds.
- [x] Implement empty directory pruning (post-order traversal, top-most reporting).
- [x] Implement one-way differential backup with destination safety in `crates/backup`.
- [x] Implement timestamp preservation via `filetime` and non-destructive file copy.
- [x] Add unit tests verifying non-destructive backup and cleanup parity against Python `core.py`.

### Phase 6 — System and Network (COMPLETED)
- [x] Implement system resource metrics (CPU %, RAM, disk stats) in `crates/system_monitor` using `sysinfo`.
- [x] Implement top process listing (read-only, no kill capability).
- [x] Implement local IPv4 interface detection in `crates/network`.
- [x] Implement ping tool with DNS resolution timing and system ICMP ping execution.
- [x] Implement TCP port check tool with latency measurement.
- [x] Implement Tidy Score algorithm in `crates/common`.

### Phase 7 — Desktop UI (COMPLETED)
- [x] Initialize `eframe` desktop application with Premium Light Violet theme.
- [x] Configure 6 color themes (`Premium Light Violet`, `Light`, `Neon Violet`, `Steam`, `Cyber Cyan`, `Crimson Night`).
- [x] Build slim icon sidebar rail with 14 page views and Ctrl+K command palette.
- [x] Implement custom widgets: tidy score `Ring`, `BarList` chart, `Card`, `StatCard`, `Toast`.
- [x] Implement feature pages: Dashboard, Organizer, Disk, Duplicates, Photos, Cleanup, Search, Renamer, Backup, Processes, Network, System, Logs, Settings.
- [x] Implement modal auth and account profile dialogs with Supabase integration.

### Phase 8 — Database and Authentication (COMPLETED)
- [x] Implement SQLite schema and WAL mode in `crates/database` using `rusqlite`.
- [x] Implement legacy `sfm.db` data preservation and migrations.
- [x] Implement undo batch persistence and restart recovery.
- [x] Implement hash cache persistence and 90-day pruning.
- [x] Implement Supabase Auth client (`crates/auth`) using `reqwest`.
- [x] Implement local loopback email confirmation HTTP server on `127.0.0.1:8765`.
- [x] Implement device registration and profile display name update.

### Phase 9 — Full Testing and Optimization (COMPLETED)
- [x] Run full workspace test suite (`cargo test --workspace`).
- [x] Run clippy and format checks (`cargo clippy`, `cargo fmt`).
- [x] Execute regression test suite comparing Rust engine results against Python baseline fixtures.
- [x] Run filesystem safety and path boundary tests.
- [x] Profile memory consumption and record performance benchmarks.
- [x] Verify headless `--selftest` in CI.

### Phase 10 — Release Preparation (IN PROGRESS)
- [x] Configure Windows/macOS/Linux native release profile (`opt-level = 3`, `lto = true`, `codegen-units = 1`, `panic = "abort"`, `strip = true`).
- [x] Package Windows application with Inno Setup installer script (`installer.iss`), icon, and metadata.
- [x] Verify clean installation and startup/shutdown lifecycle.
- [x] Prepare migration handoff and release documentation.
- [ ] Await explicit user approval before publishing or merging into `main`.
