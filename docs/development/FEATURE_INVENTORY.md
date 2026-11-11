# Smart-Filee — Feature Inventory & Migration Tracking

**Project:** Smart-Filee (Rust Edition)  
**Specification Reference:** `AGENT.md` (Sections 3.3, 7, 21)  
**Last Updated:** 2026-11-12  
**Baseline Acceptance Criteria:** 58 passing Python reference tests and 100% feature parity.  

### Status Definitions
- `NOT_STARTED`: Feature mapped from legacy code; no Rust implementation started.
- `ANALYZING`: Architecture and Rust crate interface designed.
- `IMPLEMENTING`: Active Rust code development in progress.
- `TESTING`: Implementation complete; unit/integration/regression tests running.
- `COMPLETED`: Verified by automated tests passing; acceptance criteria met.
- `BLOCKED`: Development halted due to unresolved external dependency or platform blocker.

---

## 1. Feature Catalog & Status Matrix

| Module / Area | Feature Item | Legacy Location | Target Rust Crate | Status | Acceptance Verification |
|---|---|---|---|---|---|
| **Foundation** | Cargo Workspace & Toolchain Config | — | Workspace root | COMPLETED | `cargo check --workspace`, `rust-toolchain.toml`, `rustfmt.toml` verified. |
| **Foundation** | Application Entry Point & Logging | `app.py::main`, `crash.py` | `crates/app`, `crates/common` | COMPLETED | Clean startup, structured tracing initialized, `--selftest` verified. |
| **Foundation** | Localization & Bilingual Strings | `i18n.py`, `i18n_th.py` | `crates/localization` | COMPLETED | EN & TH string interpolation, placeholder validation tested. |
| **Foundation** | Database Schema & SQLite Persistence | `db.py` | `crates/database` | COMPLETED | Schema tables, WAL mode, legacy DB compatibility tests pass. |
| **Foundation** | Theme System & Color Palettes | `themes.py` | `crates/ui` | NOT_STARTED | 6 themes defined, Premium Light Violet default colors match. |
| **File Engine** | Safe Path Validation & System Directory Guards | `core.py::protected_reason` | `crates/file_engine` | COMPLETED | Refuses root & system dirs; symlinks skipped; safe bounds verified. |
| **File Engine** | Directory Traversal & Metadata Collection | `core.py::walk_files` | `crates/file_engine` | COMPLETED | Handles unreadable files, ignores junk, respects cancellation. |
| **File Engine** | Collision-Free Path Resolution | `core.py::unique_dest` | `crates/file_engine` | COMPLETED | Generates `stem (n).ext` deterministically without overwriting. |
| **File Engine** | Atomic File Operations & Journaling | `core.py::move_files`, `app.py` | `crates/file_engine` | COMPLETED | Operations recorded in batch journal; undo rollback verified. |
| **File Engine** | Persistent Undo Stack & History | `core.py::undo_moves`, `db.py` | `crates/database`, `crates/file_engine` | COMPLETED | Reverts moved files; cleans empty dirs; prevents overwrite. |
| **File Engine** | OS Recycle Bin Integration | `core.py::trash_files` | `crates/file_engine` | COMPLETED | Safely trashes files using native OS recycle bin APIs. |
| **File Engine** | CSV Export Utility | `core.py::export_csv` | `crates/file_engine` | COMPLETED | Exports UTF-8 with BOM for Excel compatibility. |
| **Organizer** | File Categorization by Extension | `core.py::categorize` | `crates/organizer` | COMPLETED | 7 categories match `CATEGORY_MAP` ("Images", "Documents", etc.). |
| **Organizer** | Custom Rules Matching | `core.py::rule_folder`, `parse_rules` | `crates/organizer` | COMPLETED | Glob wildcard & substring rules evaluated in priority order. |
| **Organizer** | EXIF Date Extraction for Photos | `core.py::exif_date` | `crates/image_engine` | COMPLETED | Reads `DateTimeOriginal` / `DateTime` from JPEG/TIFF/WebP via `kamadak-exif`. |
| **Organizer** | Organization Planning (`By type`, `By month`, etc.) | `core.py::plan_organize` | `crates/organizer` | COMPLETED | Accurately generates planned moves without disk side effects; integrates EXIF photo routing. |
| **Organizer** | Automatic Watch Folder (Debounced) | `app.py::auto_organize`, `watcher` | `crates/organizer` | NOT_STARTED | Monitored folder moves settled files; debounced 5 seconds. |
| **Disk Analyzer** | Space Usage by Category & Subfolder | `core.py::analyze_disk` | `crates/disk_analyzer` | COMPLETED | Aggregates category byte totals and top-level folder sizes. |
| **Disk Analyzer** | Top 100 Largest Files Ranking | `core.py::analyze_disk` | `crates/disk_analyzer` | COMPLETED | Bounded min-heap preserves top 100 files by size in O(N log K). |
| **Duplicate Finder** | Staged Duplicate Detection (Size → Head → Full) | `core.py::find_duplicates` | `crates/duplicate` | COMPLETED | 64 KB head hash + 1 MB chunk full SHA-256; oldest copy first. |
| **Duplicate Finder** | Persistent Hash Caching | `core.py::_cached_hash`, `db.py` | `crates/database`, `crates/duplicate` | COMPLETED | DuplicateHashCache trait defined and wired into find_duplicates. |
| **Similar Photos** | Perceptual Fingerprinting (dHash + aHash) | `core.py::_fingerprint` | `crates/image_engine` | COMPLETED | 9x9 Lanczos grayscale resize, 64-bit dhash (H+V), ahash, mean brightness. |
| **Similar Photos** | Pigeonhole Bucketing & Union-Find Clustering | `core.py::find_similar_images` | `crates/image_engine` | COMPLETED | Hamming distance <= 6, brightness diff <= 40; largest first. |
| **Cleanup** | Temporary & Junk File Identification | `core.py::find_cleanup` | `crates/cleanup` | COMPLETED | Matches `TEMP_PATTERNS` with minimum age thresholds (1d/7d). |
| **Cleanup** | Old Large Unused Files Identification | `core.py::find_cleanup` | `crates/cleanup` | COMPLETED | Identifies files >= `min_mb` untouched for >= `old_days`. |
| **Cleanup** | Empty Directory Tree Pruning | `core.py::find_cleanup` | `crates/cleanup` | COMPLETED | Post-order walk; reports top-most empty directory nodes. |
| **Search** | Fast Filename & Wildcard Search | `core.py::search_files` | `crates/search` | COMPLETED | Supports glob `*?[` and substring search; min size filter. |
| **Bulk Renamer** | Rename Planning & Case/Space Transforms | `core.py::plan_rename` | `crates/renamer` | COMPLETED | Find/replace, prefix/suffix, lower/upper/title, numbering. |
| **Bulk Renamer** | In-Batch & Disk Collision Detection | `core.py::plan_rename` | `crates/renamer` | COMPLETED | Detects duplicate new names and existing file clashes. |
| **Backup** | Differential Backup Planning | `core.py::plan_backup` | `crates/backup` | COMPLETED | One-way sync of new & modified files; protects destination. |
| **Backup** | Safe Backup Execution | `core.py::run_backup` | `crates/backup` | COMPLETED | Preserves file timestamps via `filetime`; never deletes destination. |
| **System & Network** | System Resource Metrics (CPU, RAM, Disk) | `app.py::refresh_dashboard` | `crates/system_monitor` | COMPLETED | Reads CPU %, RAM used/total, disk free/total with sysinfo. |
| **System & Network** | Top Process Inspection (Read-Only) | `app.py::refresh_procs` | `crates/system_monitor` | COMPLETED | Lists top processes by memory/CPU without kill actions. |
| **System & Network** | Network Interfaces & Local IP Detection | `core.py::local_ip` | `crates/network` | COMPLETED | Detects active local network interface IPv4 address. |
| **System & Network** | Ping & DNS Resolution Tool | `core.py::ping_host` | `crates/network` | COMPLETED | DNS lookup timing + system ping execution with timeout. |
| **System & Network** | TCP Port Connectivity Check | `core.py::check_port` | `crates/network` | COMPLETED | TCP connection test with latency measurement. |
| **Dashboard** | Tidy Score Algorithm (0–100) | `core.py::tidy_score` | `crates/common` | COMPLETED | Mathematical score based on loose, dupes, temps, empty dirs. |
| **Dashboard** | Time-Budgeted Quick Health Summary | `core.py::health_summary` | `crates/core` | NOT_STARTED | Read-only scan within 8.0s budget; generates recommendations. |
| **Supabase Auth** | Client Initialization & Publishable Key Guard | `membership_client.py` | `crates/auth` | COMPLETED | Rejects secret keys; reads `.env` configuration safely. |
| **Supabase Auth** | Email/Password Registration & Login | `membership_client.py` | `crates/auth` | COMPLETED | Calls Supabase GoTrue REST API; establishes memory session. |
| **Supabase Auth** | Local Loopback Email Confirmation Server | `membership_client.py` | `crates/auth` | COMPLETED | Listens on `127.0.0.1:8765`, verifies one-time `token_hash`. |
| **Supabase Auth** | Profile Display Name & Device Tracking | `membership_client.py` | `crates/auth` | COMPLETED | Updates `sfm_profiles`, records `sfm_user_devices`, logs in. |
| **Desktop UI** | `egui`/`eframe` Application Shell | `app.py` | `crates/ui` | NOT_STARTED | Slim icon sidebar rail, 14 pages stack, Ctrl+K launcher. |
| **Desktop UI** | Custom Visual Widgets (Ring, BarList, Toast) | `widgets.py` | `crates/ui` | NOT_STARTED | Custom painter rendering matching Premium Light Violet mockup. |
| **Desktop UI** | Modal Auth & Account Dialogs | `auth_dialog.py`, `account_dialog.py` | `crates/ui` | NOT_STARTED | Registration, login, profile edit, device revocation UI. |
| **Packaging & CI** | Cross-Platform CI & Windows Release Workflow | `.github/workflows/` | `.github/workflows/` | NOT_STARTED | Formatting, Clippy, unit tests, Windows native release build. |

---

## 2. Migration Status Summary

- **Total Tracked Features:** 46
- **NOT_STARTED:** 11
- **ANALYZING:** 0
- **IMPLEMENTING:** 0
- **TESTING:** 0
- **COMPLETED:** 35 (Foundation 4, File Engine 7, Organizer & Search & Renamer 6, Analysis Engines 4, Maintenance & Backup 4, System & Network 5, Dashboard 1, Supabase Auth 4)
- **BLOCKED:** 0
