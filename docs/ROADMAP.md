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

### Phase 1 — Rust Foundation (IN PROGRESS)
- [ ] Initialize Cargo workspace root (`Cargo.toml`) and toolchain config (`rust-toolchain.toml`).
- [ ] Establish crate boundaries (`crates/common`, `crates/localization`, `crates/app`).
- [ ] Configure workspace lints (`clippy`), formatting rules (`rustfmt.toml`), and CI checks.
- [ ] Implement error handling infrastructure (`thiserror`, `anyhow`).
- [ ] Implement application entry point in `crates/app` with CLI argument parsing (supporting `--selftest`).
- [ ] Configure structured logging with `tracing` and `tracing-subscriber`.
- [ ] Implement basic bilingual string lookup in `crates/localization`.
- [ ] Verify workspace compilation, formatting, clippy, and unit test execution.

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

### Phase 5 — Maintenance and Backup
- [ ] Implement temporary and junk file detection in `crates/cleanup`.
- [ ] Implement old large unused file detection with age thresholds.
- [ ] Implement empty directory pruning.
- [ ] Implement one-way differential backup with destination safety in `crates/backup`.
- [ ] Add unit tests verifying non-destructive backup invariants.

### Phase 6 — System and Network
- [ ] Implement system resource metrics (CPU %, RAM, disk stats) in `crates/system_monitor` using `sysinfo`.
- [ ] Implement top process listing (read-only, no kill capability).
- [ ] Implement local IPv4 interface detection in `crates/network`.
- [ ] Implement ping tool with DNS resolution timing.
- [ ] Implement TCP port check tool with latency measurement.

### Phase 7 — Desktop UI (egui / eframe)
- [ ] Initialize `eframe` desktop application with Premium Light Violet theme.
- [ ] Configure Thai and Latin font stacks (`Instrument Sans` + `Leelawadee UI` / `Noto Sans Thai`).
- [ ] Build slim icon sidebar rail with 14 page views and Ctrl+K command palette.
- [ ] Implement custom widgets: tidy score `Ring`, `BarList` chart, `Card`, `StatCard`, `Toast`.
- [ ] Implement feature pages: Dashboard, Organizer, Disk, Duplicates, Photos, Cleanup, Search, Renamer, Backup, Processes, Network, System, Logs, Settings.
- [ ] Implement modal auth and account profile dialogs.

### Phase 8 — Database and Authentication
- [ ] Implement SQLite schema and WAL mode in `crates/database` using `rusqlite`.
- [ ] Implement legacy `sfm.db` data preservation and migrations.
- [ ] Implement undo batch persistence and restart recovery.
- [ ] Implement hash cache persistence and 90-day pruning.
- [ ] Implement Supabase Auth client (`crates/auth`) using `reqwest`.
- [ ] Implement local loopback email confirmation HTTP server on `127.0.0.1:8765`.
- [ ] Implement device registration and profile display name update.

### Phase 9 — Full Testing and Optimization
- [ ] Run full workspace test suite (`cargo test --workspace`).
- [ ] Run clippy and format checks (`cargo clippy`, `cargo fmt`).
- [ ] Execute regression test suite comparing Rust engine results against Python baseline fixtures.
- [ ] Run filesystem safety and path boundary tests.
- [ ] Profile memory consumption and record performance benchmarks.
- [ ] Verify headless `--selftest` in CI.

### Phase 10 — Release Preparation
- [ ] Configure Windows native build profile (LTO, strip, codegen units).
- [ ] Package Windows application with icon and metadata.
- [ ] Verify clean installation and startup/shutdown lifecycle.
- [ ] Prepare migration handoff and release documentation.
- [ ] Await explicit user approval before publishing or merging into `main`.
