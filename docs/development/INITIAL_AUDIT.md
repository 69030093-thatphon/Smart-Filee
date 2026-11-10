# Smart File Manager Pro — Initial Codebase Audit

**Date:** 2026-11-10
**Auditor:** Autonomous AI Agent (Role: Software Architect / Senior Software Engineer)
**Project Phase:** Phase 1 — Repository Understanding
**Target Repository:** `https://github.com/69030093-thatphon/Smart-Filee.git`
**Current Branch:** `docs/initial-audit-and-roadmap`

---

## 1. Repository Overview

Smart File Manager Pro is a local-first desktop file management and optimization suite designed primarily for Windows 10/11 (with cross-platform capabilities on Linux and macOS), developed in Python using PySide6 (Qt 6). It aims to provide users with file organization, duplicate finding, storage analysis, automated disk cleanup, bulk renaming, backup synchronization, and basic system/network monitoring without unnecessary network telemetry.

In recent iterations (v4.2 - v4.3), optional online cloud authentication and multi-device identity features via Supabase Auth were introduced while keeping the local SQLite storage (`~/.smart_file_manager_pro/sfm.db`) strictly local for file paths and operations.

---

## 2. Technology Stack & Dependencies

### 2.1 Core Technologies
- **Language:** Python 3.11+ (Runtime evaluated: Python 3.12.9)
- **GUI Framework:** PySide6 (Qt 6.7+)
- **Local Persistence:** SQLite3 (standard library), thread-safe WAL mode
- **Cloud Identity / Backend:** Supabase Auth (`supabase==2.24.0`, `python-dotenv==1.0.1`, `gotrue`, `postgrest`, `httpx`)
- **System & File Operations:** `psutil>=5.9`, `send2trash>=1.8`, `shutil`, `pathlib`
- **Media & Image Processing:** `Pillow>=10.0`
- **Packaging & Distribution:** PyInstaller 6.0+, Inno Setup (Windows setup compiler)

### 2.2 Dependency Review (`requirements.txt` & `requirements-dev.txt`)
- `PySide6>=6.7`: Powers the desktop GUI.
- `psutil>=5.9`: System process and hardware metrics (CPU, RAM, disk, network).
- `send2trash>=1.8`: Native OS recycle bin integration.
- `Pillow>=10.0`: Image thumbnail generation and perceptual hash calculation for duplicate/similar image detection.
- `python-dotenv==1.0.1`: Loads environment configurations (`.env`).
- `supabase==2.24.0`: Python SDK for Supabase auth and profile/device tracking.
- `pyinstaller>=6.0`: Compiles source into standalone binary executables.

---

## 3. Directory Structure & Application Entry Points

```text
Smart-Filee/
├── .git/                               # Git version control directory
├── .gitattributes                      # Line endings & text normalization
├── .github/
│   └── workflows/
│       └── build-windows.yml           # CI workflow for Windows executable & Inno Setup
├── .gitignore                          # Ignored patterns (virtual environments, caches, secrets)
├── AGENT.md                            # Autonomous agent development specification
├── MEMBERSHIP_SETUP.md                 # Supabase membership setup and deployment documentation
├── README.md                           # Main user and developer documentation (Thai & English)
├── account_dialog.py                   # PySide6 dialog for profile viewing and device revocation
├── app.py                              # Main application entry point & GUI window definition
├── auth_dialog.py                      # PySide6 sign-in, registration, and email confirmation dialog
├── build_exe.bat                       # Windows batch script for building PyInstaller standalone EXE
├── core.py                             # Pure-Python business logic and file operations (Qt-independent)
├── crash.py                            # Unhandled exception capture and disk crash logging
├── db.py                               # SQLite storage wrapper (undo batches, logs, hash cache, rules)
├── fonts/                              # Bundled Instrument Sans font assets
│   ├── InstrumentSans-Bold.ttf
│   ├── InstrumentSans-Medium.ttf
│   ├── InstrumentSans-Regular.ttf
│   └── InstrumentSans-SemiBold.ttf
├── i18n.py                             # Translation helper and string formatting engine
├── i18n_th.py                          # Thai language translation dictionary and regex patterns
├── icon.ico                            # Application icon
├── installer.iss                       # Inno Setup Windows installer script
├── membership_client.py                # Supabase SDK integration & loopback email confirmation server
├── requirements.txt                    # Production runtime dependencies
├── requirements-dev.txt                # Development and packaging dependencies
├── run.bat                             # Windows launcher script
├── smartFileManagerPro.spec            # PyInstaller build specification
├── tests/                              # Automated test suite
│   ├── test_core.py                    # Unit tests for core file algorithms
│   ├── test_db.py                      # Unit tests for SQLite persistence & migration
│   ├── test_email_callback.py          # Unit tests for HTTP loopback callback server
│   ├── test_i18n.py                    # Translation coverage and integrity tests
│   └── test_ui.py                      # Headless PySide6 UI workflow smoke tests
├── themes.py                           # Application color palettes and global Qt Style Sheet (QSS)
└── widgets.py                          # Custom Qt widgets (Ring score, BarList, Card, StatCard, Toast)
```

### Application Entry Point
- **`app.py::main()`**:
  1. Initializes `QApplication` with Fusion style.
  2. Sets up `crash.install()` for global exception trapping.
  3. Loads bundled fonts via `load_bundled_fonts()`.
  4. Reads persistent settings from `QSettings("SmartFileManagerPro", "SmartFileManagerPro")`.
  5. Validates Supabase configuration via `membership_client.check_server()`.
  6. Opens `AuthDialog` for user sign-in / registration (blocking modal).
  7. Upon successful authentication, instantiates and shows `MainWindow(user=login.user)`.
  8. Handles optional `--selftest` flag for automated headless startup testing.

---

## 4. Architecture Overview

The system is decoupled into three primary layers:

```
┌────────────────────────────────────────────────────────────────────────┐
│                          Presentation Layer                            │
│   app.py (MainWindow), auth_dialog.py, account_dialog.py, widgets.py   │
│         Themes & Styling (themes.py), Localization (i18n.py)           │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │ Worker Threads (QThread)
┌───────────────────────────────────▼────────────────────────────────────┐
│                         Core Business Logic                            │
│       core.py (File scanning, organizing, hashing, cleanup, backup)    │
│           membership_client.py (Supabase auth & session state)         │
└───────────────────┬────────────────────────────────┬───────────────────┘
                    │                                │
┌───────────────────▼──────────────┐   ┌─────────────▼───────────────────┐
│       Local Storage Layer        │   │       Cloud Identity Layer      │
│   db.py (SQLite3 - WAL mode)     │   │      Supabase PostgreSQL        │
│   • undo_batches / undo_moves    │   │      • sfm_profiles             │
│   • activity_log / rules         │   │      • sfm_user_devices         │
│   • hash_cache / scan_history    │   │      • sfm_login_logs           │
│   • Local only; zero telemetry   │   │      • RLS protected via JWT    │
└──────────────────────────────────┘   └─────────────────────────────────┘
```

1. **Presentation Layer:**
   - Built on PySide6 widgets.
   - Decoupled from long-running operations via `Worker(QThread)` with `progress`, `done`, and `failed` Qt signals.
   - Adaptive themes (Steam, Neon Violet, Cyber Cyan, Crimson Night, Premium Light Violet, Light) injected via dynamic QSS.
   - Dual-language support (English and Thai).

2. **Core Domain Layer (`core.py`):**
   - Strictly free of Qt dependencies, enabling direct unit test execution without an active X11/Wayland/Quartz display server.
   - Cancellation tokens (`cancel` callable) and progress callbacks (`progress` callable) accepted across all batch operations.
   - Strict safety guarantees: system directory blacklisting, non-destructive renaming conflicts (`name (n).ext`), and unreadable file tracking.

3. **Persistence Layer (`db.py` & `membership_client.py`):**
   - **Local DB:** SQLite3 (`sfm.db`) with foreign key constraints, indexation, and WAL mode. Manages transactional undo stacks, persistent hash caches, and activity logs.
   - **Cloud Identity:** Supabase Auth for identity management with loopback local HTTP verification (`127.0.0.1:8765`), keeping user file data local.

---

## 5. Existing Functionality

1. **Dashboard:**
   - Real-time CPU, RAM, and network I/O monitors.
   - Tidiness Score ring gauge (0–100) calculated from clutter algorithms (`core.tidy_score`).
   - Quick action suggestions and recent activity logs.
   - Universal quick-switcher (Ctrl+K palette) with fuzzy action searching.

2. **File Organizer:**
   - Organization modes: "By type", "By type + month", "By month", "Photos by date taken" (EXIF DateTimeOriginal).
   - Custom ordering rules with wildcard patterns (`*.psd`, `invoice_*`).
   - Live preview, selective file checkboxes, and transaction-backed multi-level Undo.
   - Background directory watcher (`QFileSystemWatcher`) with configurable settling debounce (5s).

3. **Disk Analyzer:**
   - Multi-threaded file size indexing.
   - Top 100 largest files listing with numeric sorting.
   - Category and directory space breakdown charts with CSV export.

4. **Duplicate Finder:**
   - Multi-stage pipeline: file size match -> head 64KB hash -> full SHA-256 hash.
   - Persistent `hash_cache` table to skip re-hashing unchanged files.
   - Safe isolation into `_Duplicates` or native OS Recycle Bin.

5. **Similar Photos:**
   - Perceptual image hashing (horizontal dHash, vertical dHash, aHash, mean brightness).
   - Pigeonhole bucket indexing and Hamming distance clustering.
   - Thumbnail preview and automatic retention of the highest-resolution copy.

6. **Cleanup:**
   - Identifies temporary files (`*.tmp`, `thumbs.db`, `.ds_store`), unused large files older than N days, and empty directories.
   - Safe pre-selection (large unused files are left unchecked by default).

7. **Bulk Renamer:**
   - String find & replace, prefix/suffix insertion, case conversion, whitespace normalization, and sequential zero-padded numbering.
   - Conflict detection and system path protection preview before renaming.

8. **Backup:**
   - One-way differential file synchronization.
   - Never deletes files from source or destination.

9. **System Info & Processes:**
   - Hardware, OS, and network adapter details.
   - Real-time read-only process listing with PID, CPU %, and memory usage.

10. **Network Tools:**
    - Local IP lookup, ping latency, DNS resolution time, and TCP port reachability checking.

11. **Activity Log & Settings:**
    - Searchable SQLite activity log with log retention limits (5,000 lines).
    - Database statistics viewer (actions, moves, scans, cached hashes).
    - Theme, font family, font size, and language selection.

12. **Membership & Identity:**
    - Supabase Auth integration with local loopback callback verification.
    - Profile management and multi-device registration records.

---

## 6. Testing Infrastructure

### Current Test Suite (`tests/`):
- `test_core.py`: 25 test cases covering disk analysis, organizer, backup, duplicates, EXIF date extraction, safety guards, undo journal roundtrips, and tidy scores.
- `test_db.py`: 7 test cases covering schema creation, undo batch transactions, log filtering, rule persistence, hash cache hits/misses, and legacy JSON/log migration.
- `test_email_callback.py`: 3 test cases for loopback HTTP callback handling, state validation, and token hash extraction.
- `test_i18n.py`: 4 test cases validating AST-extracted UI strings, placeholder format parity, Thai dictionary completeness, and translation fallback behavior.
- `test_ui.py`: 17 headless PySide6 smoke tests using `offscreen` Qt QPA platform.

### Test Execution Assessment:
- Unit tests for pure-Python modules (`core.py`, `db.py`, `i18n.py`) execute smoothly without GUI dependencies.
- `test_email_callback.py` requires `dotenv` and `supabase` packages installed.
- `test_ui.py` requires `PySide6` and `psutil`.

---

## 7. Build and Packaging Process

1. **PyInstaller Spec (`smartFileManagerPro.spec`):**
   - Configured for directory build (`COLLECT`).
   - Collects `PySide6`, `supabase`, `httpx`.
   - **Discrepancy:** Does NOT bundle `fonts/` directory or `icon.ico` in `datas`.

2. **Windows Batch Builder (`build_exe.bat`):**
   - Invokes `pyinstaller --noconsole --onefile --name SmartFileManagerPro --icon icon.ico --add-data "fonts;fonts" app.py`.
   - Produces a single-file executable `dist\SmartFileManagerPro.exe`.
   - **Discrepancy:** Conflicts with `smartFileManagerPro.spec` and `installer.iss`.

3. **Inno Setup Script (`installer.iss`):**
   - Packages `dist\SmartFileManagerPro\*` (directory output) into `SmartFileManagerPro-Setup.exe`.

4. **CI Workflow (`.github/workflows/build-windows.yml`):**
   - Runs on Windows runner for tags `v*`.
   - Uses PyInstaller with `SmartFileManagerPro.spec`, then builds installer with Inno Setup.
   - **Missing Workflow:** `ci.yml` (automated PR testing on Linux + Windows) mentioned in README.md is absent from the repository.

---

## 8. Security Observations

1. **Zero Secret Leakage:**
   - Repository contains no hardcoded API secrets, database passwords, or private service keys.
   - Supabase client requires only public publishable keys and strictly enforces HTTPS.
   - Runtime security check in `membership_client.py` explicitly rejects secret/service_role keys (`sb_secret_` or `service_role`).

2. **Input and Path Validation:**
   - `core.protected_reason()` protects drive roots and system directories (`WINDIR`, `ProgramFiles`, `/usr`, `/System`, etc.) from modification, moving, or deletion.
   - `ILLEGAL_NAME` regex prevents invalid characters and path traversal patterns in custom rules and renaming operations.
   - Unique destination path generation (`core.unique_dest`) prevents accidental file overwrites.
   - Shell command injection vectors are mitigated: `core.valid_host()` strictly regex-validates hostnames before calling `ping`.

3. **Local HTTP Callback Server:**
   - Loopback callback in `EmailConfirmationListener` binds exclusively to `127.0.0.1:8765`.
   - Employs cryptographically secure random per-registration token URLs (`secrets.token_urlsafe(24)`).
   - Suppresses HTTP request logging to prevent token leakage in console or log files.
   - Single-use consumption pattern with automated shutdown upon receipt or dialog closure.

---

## 9. Performance Observations

1. **Duplicate Finder Optimization:**
   - Staged comparison avoids computing expensive full SHA-256 hashes on dissimilar files.
   - SQLite `hash_cache` table avoids re-hashing files with identical file size and modification timestamps.

2. **Similar Photos Performance:**
   - Uses Pillow `.draft("L", (128, 128))` for fast JPEG decode and downsampling.
   - Pigeonhole partitioning into hash segments narrows comparison pairs to $O(M)$ instead of $O(N^2)$.
   - Degenerate buckets (>400 items) are ignored to prevent combinatorial slowdown on flat/blank images.

3. **UI Responsiveness:**
   - File system traversal and hashing tasks run in non-blocking background `QThread` instances.
   - Database operations use SQLite Write-Ahead Logging (WAL) and recursive re-entrant locks (`threading.RLock`) to allow parallel read/write access.

---

## 10. Existing Bugs, Deficiencies, & Technical Debt

| ID | Category | Severity | Description |
|---|---|---|---|
| **BUG-01** | Packaging | **High** | `smartFileManagerPro.spec` does not include `fonts/` directory or `icon.ico` in `datas`, breaking font rendering and icon display in bundled builds. |
| **BUG-02** | Packaging | **Medium** | Build configuration conflict between `build_exe.bat` (`--onefile` output) and `installer.iss` (expects directory `dist\SmartFileManagerPro\*`). |
| **BUG-03** | CI/CD | **High** | `.github/workflows/ci.yml` is missing from the repository despite being documented in `README.md`. No automated PR test validation currently exists. |
| **BUG-04** | i18n | **Medium** | `account_dialog.py` contains hardcoded English strings and does not use `i18n.tr()`. |
| **BUG-05** | i18n | **Low** | Several dialog feedback strings in `auth_dialog.py` and menu actions in `app.py` lack Thai translations in `i18n_th.py`. |
| **BUG-06** | Testing | **Medium** | `tests/test_i18n.py` only parses `app.py`, skipping `account_dialog.py` and `auth_dialog.py`. |
| **BUG-07** | Git Config | **Low** | No `.gitignore` file existed in the initial commit, risking untracked local caches, venvs, and build artifacts. |
| **BUG-08** | App Startup | **Medium** | In headless testing or environments where Supabase is unconfigured, `app.py` blocks at `login.exec()` rather than evaluating `--selftest`. |
| **BUG-09** | Testing / Isolation | **Medium** | Test state leakage via macOS `CFPreferences` in `QSettings`. Tests mutating settings (e.g. `test_organizer_preview...` setting `org_mode` to "By month") cause subsequent tests to fail unless settings are isolated in a temporary scope. |

---

## 11. Development Risks & Mitigation Strategies

1. **Risk:** Unintentional file data loss or corruption during file moving/renaming.
   **Mitigation:** Retain multi-tiered safety checks: `protected_reason()`, `unique_dest()` non-destructive naming, and SQLite undo batch journaling.

2. **Risk:** Platform-dependent behavior across Windows, macOS, and Linux (file paths, separators, line endings, permissions).
   **Mitigation:** Use `pathlib.Path`, normalized path strings, `send2trash` cross-platform trash handling, and multi-OS CI testing.

3. **Risk:** Cloud authentication dependencies interrupting offline file management utility.
   **Mitigation:** Clarify online identity boundaries, safeguard local SQLite data integrity, and ensure smooth fallback handling when network is offline.

---

## 12. Conclusion & Next Steps

The codebase exhibits high structural cleanliness, separation of concerns, and robust defensive design in `core.py` and `db.py`. Immediate priorities for subsequent phases include:
1. Harmonizing PyInstaller specs, installer scripts, and packaging assets.
2. Implementing the complete multi-platform CI workflow (`.github/workflows/ci.yml`).
3. Completing Thai localization across account and auth dialogs.
4. Expanding test coverage for edge cases, error states, and headless automated self-tests.
