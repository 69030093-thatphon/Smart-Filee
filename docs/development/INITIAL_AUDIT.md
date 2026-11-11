# Smart-Filee — Initial Codebase & Rust Migration Audit

**Project:** Smart-Filee (Smart File Manager Pro)  
**Date:** 2026-11-12  
**Auditor:** Autonomous Senior Rust Software Architect & Systems Engineer  
**Specification Reference:** `AGENT.md` (Autonomous Rust Migration Specification)  
**Base Repository Commit:** `6be403d` / `f38ae5b` (Branch: `feat/rust-migration`)  
**Status:** Audit Complete — Ready for Rust Workspace Initialization (Phase 1)  

---

## 1. Executive Summary

Smart-Filee is a local-first desktop file management suite originally authored in Python 3.12 with PySide6 (Qt for Python). The application provides 14 integrated tools encompassing file organization, storage analysis, duplicate file identification, perceptual image similarity matching, temporary and unreferenced file cleanup, high-speed file search, bulk renaming, differential backup, process inspection, network diagnostics, persistent undo journaling, and SQLite metadata caching.

The application also features hybrid cloud user identity via Supabase Auth (email/password registration, login, and a local loopback callback server for email confirmation on port 8765) and profile/device tracking (`sfm_profiles`, `sfm_user_roles`, `sfm_user_devices`, `sfm_login_logs`), while strictly enforcing local-first data isolation (no local file paths, file contents, or filesystem logs are ever transmitted to the cloud).

This audit documents the legacy Python application structure, establishes the verified functional baseline via automated test execution (58/58 unit tests passing), examines the SQLite schema and Supabase authentication integration, and details the technical strategy, risks, and performance targets for rewriting the suite into a high-performance, safe, native Rust desktop application powered by `egui`/`eframe`.

---

## 2. Legacy Codebase Architecture & File Inventory

The Python codebase is divided into modular, decoupled layers:

| File | Size (Bytes) | Lines | Purpose & Responsibility |
|---|---|---|---|
| `core.py` | 32,203 | 821 | Pure-Python business logic, file scanning, hashing, organizing, renamer, backup, network checks, tidy score calculation (zero Qt dependencies, fully testable). |
| `db.py` | 13,646 | 315 | SQLite persistence wrapper: WAL mode, schema management, undo batches, activity logging, custom rules, hash caching, scan history, and legacy JSON/log migration. |
| `app.py` | 115,716 | 2,601 | PySide6 main application entry point, window management, sidebar rail navigation, 14 page builders, asynchronous worker threads, and event coordination. |
| `widgets.py` | 15,237 | 407 | Custom Qt GUI widgets: `Ring` (tidy score gauge), `BarList` (category/folder size bars), `Card`, `StatCard`, `TwoLineDelegate`, `PathBox` (drag-and-drop), and `Toast`. |
| `themes.py` | 12,993 | 187 | Six color theme palettes (Premium Light Violet, Light, Neon Violet, Steam, Cyber Cyan, Crimson Night) and dynamic QSS generator. |
| `i18n.py` | 1,118 | 38 | Translation manager with string interpolation `tr()` and regex substitution patterns for engine progress messages. |
| `i18n_th.py` | 48,631 | 540 | Comprehensive Thai dictionary covering all UI labels, dialogs, error messages, and regex progress patterns. |
| `membership_client.py` | 19,572 | 505 | Supabase client integration (Auth REST API, postgREST tables, local HTTP loopback server on `127.0.0.1:8765` for email verification). |
| `auth_dialog.py` | 9,127 | 211 | PySide6 modal dialog for email/password authentication, registration, and asynchronous email confirmation polling. |
| `account_dialog.py` | 5,532 | 114 | Profile management (display name editing) and multi-device listing and revocation dialog. |
| `crash.py` | 1,577 | 40 | Global unhandled exception hook writing structured stack traces to `~/.smart_file_manager_pro/crash.log`. |

---

## 3. Database Schema & SQLite Persistence Analysis

The local SQLite database resides at `~/.smart_file_manager_pro/sfm.db` and is operated in Write-Ahead Logging (`WAL`) mode with foreign key constraints enabled.

### 3.1 Tables and Constraints

```sql
CREATE TABLE IF NOT EXISTS meta (
    key   TEXT PRIMARY KEY,
    value TEXT
);

CREATE TABLE IF NOT EXISTS undo_batches (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    label      TEXT    NOT NULL,
    created_at TEXT    NOT NULL,
    status     TEXT    NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'undone', 'stale'))
);

CREATE TABLE IF NOT EXISTS undo_moves (
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    batch_id INTEGER NOT NULL REFERENCES undo_batches(id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    src      TEXT    NOT NULL,
    dst      TEXT    NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_undo_moves_batch ON undo_moves(batch_id);

CREATE TABLE IF NOT EXISTS activity_log (
    id      INTEGER PRIMARY KEY AUTOINCREMENT,
    ts      TEXT NOT NULL,
    message TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS rules (
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    position INTEGER NOT NULL,
    pattern  TEXT    NOT NULL,
    folder   TEXT    NOT NULL
);

CREATE TABLE IF NOT EXISTS hash_cache (
    path    TEXT    NOT NULL,
    kind    TEXT    NOT NULL CHECK (kind IN ('head', 'full')),
    size    INTEGER NOT NULL,
    mtime   REAL    NOT NULL,
    hash    TEXT    NOT NULL,
    updated REAL    NOT NULL,
    PRIMARY KEY (path, kind)
);

CREATE TABLE IF NOT EXISTS scan_history (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    ts           TEXT    NOT NULL,
    kind         TEXT    NOT NULL,
    root         TEXT    NOT NULL,
    scanned      INTEGER NOT NULL DEFAULT 0,
    groups       INTEGER NOT NULL DEFAULT 0,
    dup_files    INTEGER NOT NULL DEFAULT 0,
    wasted_bytes INTEGER NOT NULL DEFAULT 0,
    cache_hits   INTEGER NOT NULL DEFAULT 0,
    cache_misses INTEGER NOT NULL DEFAULT 0,
    seconds      REAL    NOT NULL DEFAULT 0
);
```

### 3.2 Data Integrity & Migration Requirements for Rust

1. **Schema Versioning:** The Rust database layer (`crates/database`) must inspect `meta.key = 'schema_version'`. If existing tables exist, they must be preserved without data loss.
2. **Path Encoding:** Python wrote paths using `str(Path)`. On Windows, these contain backslashes (`C:\Users\...`); on Unix, forward slashes. Rust must handle platform-native paths cleanly using normalized string serialization or canonical representation.
3. **MTime Precision:** `mtime` in `hash_cache` is stored as `REAL` (floating-point seconds since Unix epoch from `os.stat().st_mtime`). Rust `std::time::SystemTime` conversion must preserve fractional seconds or safely match within timestamp tolerances.
4. **Maintenance Triggers:** Startup maintenance must retain:
   - Up to 5,000 recent log lines in `activity_log`.
   - Hash cache entries updated within the last 90 days.
   - Up to 20 active batches in the in-memory undo stack (`limit = 20`), while persisting all historical batches in `undo_batches` with statuses `active`, `undone`, or `stale`.

---

## 4. Supabase Authentication & Hybrid Cloud Architecture

### 4.1 Cloud vs. Local Boundaries

```
┌────────────────────────────────────────────────────────┐
│                   SUPABASE CLOUD                       │
│  (Auth API, sfm_profiles, sfm_user_devices, logs)      │
└───────────────────────────▲────────────────────────────┘
                            │ HTTPS (REST API)
                            │ Publishable Key Only (sb_publishable_*)
┌───────────────────────────▼────────────────────────────┐
│              SMART-FILEE DESKTOP APP                   │
│                                                        │
│  ┌──────────────────────┐    ┌──────────────────────┐  │
│  │   Auth / Account     │    │   Local HTTP Server  │  │
│  │   (reqwest / memory) │    │   (127.0.0.1:8765)   │  │
│  └──────────────────────┘    └──────────────────────┘  │
│                                                        │
│  ┌──────────────────────────────────────────────────┐  │
│  │               LOCAL SQLITE (sfm.db)              │  │
│  │  (Undo, Rules, Logs, Hash Cache, Scan History)   │  │
│  └──────────────────────────────────────────────────┘  │
│                                                        │
│  ┌──────────────────────────────────────────────────┐  │
│  │              LOCAL USER FILESYSTEM               │  │
│  │  (Never uploaded, zero telemetry, local-first)   │  │
│  └──────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────┘
```

### 4.2 Security Rules Enforced

1. **Zero Secret Keys in Client:** Only the project URL and public publishable key (`sb_publishable_...`) are permitted. Any occurrence of `service_role` or `sb_secret_` triggers immediate rejection.
2. **In-Memory Session:** The active session JWT is kept in process memory; access and refresh tokens are cleared on logout.
3. **Loopback Email Confirmation Listener:**
   - Listens on `127.0.0.1:8765`.
   - Path: `/auth/callback/{random_state_token}`.
   - Accepts only single-use `token_hash` and `type=email`. Rejects URL payloads exceeding 2,048 bytes.
   - Completes OTP verification directly with Supabase via `verify_otp({email, token_hash, type: "email"})`.
   - Never exposes access tokens or refresh tokens in URL parameters or server logs.
4. **Device Metadata Restriction:** Device records register hostname, OS version, and app version. No file paths or filesystem scan information are ever sent to Supabase.

---

## 5. Existing Security Protections & File Safety Mechanisms

The legacy application adheres to strict safety invariants that must be preserved in the Rust engine:

1. **Protected Path Guards (`protected_reason`):**
   - Refuses file movements, renames, and backups if target or source is a filesystem/drive root (`/`, `C:\`).
   - Refuses system folders on Windows (`WINDIR`, `SystemRoot`, `ProgramFiles`, `ProgramFiles(x86)`, `ProgramData`) and Unix (`/bin`, `/boot`, `/dev`, `/etc`, `/lib`, `/lib64`, `/proc`, `/sbin`, `/sys`, `/usr`, `/System`, `/Library`, `/Applications`).
2. **Directory & File Skipping:**
   - Skips system volume folders (`System Volume Information`, `$Recycle.Bin`).
   - Skips application working directories (`_Duplicates`, `_Similar`).
   - Skips system junk (`desktop.ini`, `thumbs.db`, `.DS_Store`).
   - Skips in-progress downloads (`.crdownload`, `.part`, `.tmp`, `.download`, `.partial`).
   - Skips symbolic links during directory traversal to prevent directory loops and symlink hijacking.
3. **Collision Avoidance:**
   - `unique_dest(dest_dir, name)` ensures files never overwrite existing files by appending ` (1)`, ` (2)`, etc.
   - Renamer pre-validates collisions against both on-disk files and sibling rename targets within the same batch.
4. **Execution Safeguards:**
   - Context menus in file tables refuse to launch executables (`.exe`, `.bat`, `.cmd`, `.msi`, `.ps1`, `.sh`, `.vbs`, `.jar`, `.lnk`).
5. **Two-Stage Duplicate Verification:**
   - Step 1: File size grouping.
   - Step 2: 64 KB head hash (`head`) SHA-256 calculation.
   - Step 3: Full-file SHA-256 calculation (`full`) in 1 MB chunks.
   - Retains the oldest file (by modification time) by default.
6. **Non-Destructive Defaults:**
   - Cleanup unchecks large old files by default.
   - Backup is strictly one-way copy; it never deletes destination files.
   - Duplicate finder moves to `_Duplicates` or optional OS Recycle Bin; it never permanently deletes without explicit confirmation.
   - Auto-organize (folder watch) is disabled by default and requires explicit opt-in.

---

## 6. Functional Baseline Verification (Existing Tests)

The test suite in `tests/` was executed against the Python reference implementation:

```bash
/Users/x/Desktop/Smart-Filee/.venv/bin/python -m unittest discover -s tests -v
```

**Results:**
- Total tests executed: **58**
- Failures: **0**
- Errors: **0**
- Skipped: **0**
- Duration: **3.306s**
- Outcome: **100% PASS**

### Test Breakdown

1. **`test_core.py` (25 tests):**
   - Disk analysis, categorizations, file formatting.
   - Duplicate finder with partial and full hash verification.
   - File organizer planning, execution, and rollback.
   - File search, wildcard patterns, and limits.
   - Renamer planning, conflicts, illegal names, and application.
   - Backup planning and differential copy.
   - Path protection against root and system folder modifications.
   - EXIF date extraction and photo organization.
   - Perceptual image similarity hashing and union-find clustering.
   - Temporary file patterns and age thresholding.
   - Tidy score formula calculation and time-budgeted health summary.
2. **`test_db.py` (7 tests):**
   - Schema creation and WAL configuration.
   - Hash cache retrieval, eviction, and mtime/size invalidation.
   - Undo batch pushing, popping, and stale batch pruning.
   - Activity log insertion, filtering, and trimming.
   - Custom organizer rule ordering.
   - Scan history recording.
   - One-time legacy migration from JSON journals and text logs.
3. **`test_email_callback.py` (3 tests):**
   - Local HTTP loopback listener initialization and random state token validation.
   - Single-use OTP token extraction and oversized URL rejection.
   - Email verification verification flow.
4. **`test_i18n.py` (4 tests):**
   - English to Thai translation dictionary lookup.
   - UI string coverage (zero missing keys).
   - Placeholder consistency (`{name}`, `{count}`, `{error}`).
   - Progress regex translation matching.
5. **`test_ui.py` (19 headless tests):**
   - Headless window initialization across all 6 themes.
   - Sidebar navigation and rail button state synchronization.
   - Organizer preview, live updates, and undo roundtrip.
   - Duplicate workflow and selection.
   - Similar photos group thumbnails and selection.
   - Bulk renamer preview and conflict detection.
   - Backup workflow and progress.
   - Settings persistence (fonts, sizes, themes, languages).

---

## 7. Migration Risks & Architectural Considerations

| Risk Area | Severity | Legacy Behavior | Proposed Rust Strategy |
|---|---|---|---|
| **GUI Framework & Rendering** | High | PySide6 / Qt with QSS stylesheets. | `egui` / `eframe`. Immediate-mode rendering is light, fast, and cross-platform. We must replicate the Premium Light Violet theme, custom ring chart, bar chart, and cards using custom `egui::Painter` shapes and widgets. |
| **Font Rendering (Thai Support)** | High | Windows uses `Leelawadee UI` fallback for Thai; Latin uses bundled `Instrument Sans`. | `egui` requires explicit font definitions in `epaint::text::FontDefinitions`. We must bundle `Instrument Sans` and include a high-quality Unicode Thai font (e.g. `Noto Sans Thai` or system fallback) in `FontDefinitions` to ensure crisp rendering on Windows, Linux, and macOS. |
| **Filesystem Safety & Concurrency** | High | Python `os.walk` with manual error list and thread workers. | Rust `walkdir` or parallel `jwalk` + `rayon`. File mutations must be serialized through an operation journal with atomic rollback tracking. |
| **SQLite Concurrency** | Medium | Single SQLite connection with Python `threading.RLock`. | Use `rusqlite` with connection pooling or `r2d2_sqlite`, or Tokio-friendly async access via `tokio::task::spawn_blocking`. WAL mode enabled. |
| **Perceptual Image Hashing** | Medium | Python Pillow `Image.open().draft().convert("L").resize((9, 9))` computing dhash (H+V), ahash, and average brightness. | Implement the exact algorithm in Rust using the `image` crate (9x9 Lanczos grayscale resize, bitwise hash computation, pigeonhole bucketing, and union-find) to ensure 100% mathematical parity with legacy duplicate clusters. |
| **Loopback Email Callback** | Low | Python `http.server.HTTPServer` on `127.0.0.1:8765`. | Lightweight HTTP listener using `hyper` / `axum` or a standard socket listener in `crates/auth`, shutting down immediately after callback receipt. |
| **System Process & Hardware Metrics** | Low | Python `psutil` (CPU %, RAM, disk stats, network I/O, process table). | Use `sysinfo` crate in Rust. It provides low-overhead, native OS metrics across Windows, macOS, and Linux. |
| **OS Recycle Bin** | Low | Python `send2trash`. | Use the `trash` crate in Rust, providing safe, native Recycle Bin integration on Windows, macOS, and Linux. |

---

## 8. Potential Performance Improvements in Rust

1. **Directory Traversal:** Parallel directory traversal using `jwalk` or work-stealing `rayon` achieves up to 5x-10x faster scans on modern SSDs compared to single-threaded Python `os.walk`.
2. **SHA-256 Hashing:** The `sha2` crate with hardware-accelerated SHA extensions (via AVX2 / ARM Crypto) hashes files at multi-gigabyte-per-second throughput with zero Python GIL contention.
3. **Memory Footprint:** PySide6 + Python runtime consumes ~120-180 MB idle RAM. An `eframe`/`egui` native Rust binary typically operates under 25-45 MB idle RAM.
4. **Binary Size & Distribution:** Standalone PyInstaller bundles exceed 70-100 MB and unpack dozens of DLLs to `%TEMP%`. The compiled Rust executable is a single compact binary (~15-25 MB).
5. **Startup Time:** Python cold startup requires 1.5-3.0 seconds to initialize the Python interpreter and load PySide6 dynamic libraries. A native Rust binary launches in under 100 milliseconds.

---

## 9. Conclusion & Audit Sign-Off

The audit confirms that the legacy Python codebase is cleanly architected, thoroughly tested, and well-documented. All algorithms and invariants are understood, and the functional baseline of 58 passing tests provides clear acceptance criteria for the Rust migration.

We are ready to proceed with Phase 1: Rust Workspace Foundation.
