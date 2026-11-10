# Smart File Manager Pro — Quality Assurance Test Plan

**Document Version:** 1.0.0
**Date:** 2026-11-10
**Lead QA Engineer:** ภานุพงษ์ บำรุงศรี (`team/tester`)
**Target Application Version:** 4.2.0 (v4.3 Milestone)

---

## 1. Test Objectives

1. Verify functional correctness of all file operations (Organize, Duplicate Detection, Cleanup, Rename, Backup).
2. Validate non-destructive safety mechanisms (System folder blocking, collision-avoiding unique destination naming, undo transactions).
3. Validate persistence integrity of SQLite WAL database (`undo_batches`, `hash_cache`, `activity_log`, `rules`).
4. Validate headless execution across Linux, macOS, and Windows runners without display server dependencies.
5. Verify 100% internationalization coverage (Thai & English) across all dialogs and widgets.

---

## 2. Test Scope & Categorization

| Category | Modules Tested | Framework / Tool | Execution Mode |
|---|---|---|---|
| **Unit Testing** | `core.py`, `i18n.py`, `i18n_th.py` | `unittest` | Headless / Automated |
| **Integration Testing** | `db.py` (SQLite), `membership_client.py` (Loopback HTTP) | `unittest` | Headless / Automated |
| **UI Smoke Testing** | `app.py` (`MainWindow`), `widgets.py`, `themes.py` | PySide6 (`QT_QPA_PLATFORM=offscreen`) | Headless / Automated |
| **Regression Testing** | Undo journals, edge-case file paths, custom organizer rules | `unittest` | Headless / Automated |
| **Build & Packaging Testing** | `smartFileManagerPro.spec`, `build_exe.bat`, `installer.iss` | PyInstaller, Inno Setup | Automated / CI |

---

## 3. Test Environment Specifications

- **Operating Systems Tested:** macOS (Darwin 24 / Apple Silicon), Linux (Ubuntu 22.04 LTS via GitHub Actions), Windows 11 (Windows runner via GitHub Actions).
- **Python Version:** 3.12.9
- **GUI Engine:** PySide6 6.12.0
- **Virtual Display:** `QT_QPA_PLATFORM=offscreen`

---

## 4. Entry and Exit Criteria

### Entry Criteria
- Clean Git working tree on task branch.
- Dependencies installed in isolated `.venv`.
- Core application compiles without syntax or import errors.

### Exit Criteria
- 100% of automated unit, integration, and UI smoke tests execute and pass without failures or errors.
- Zero uncaught exceptions written to `crash.log`.
- No lingering test files or directories in temporary storage.
- All translation keys in UI components match `TH` dictionary entries.
