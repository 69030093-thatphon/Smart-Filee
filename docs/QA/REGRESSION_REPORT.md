# Smart File Manager Pro — Regression Test Report

**Execution Date:** 2026-11-10
**Evaluator:** ภานุพงษ์ บำรุงศรี (`team/tester`)
**Scope:** Re-validation of critical safety guards, packaging specs, and OS isolation

---

## 1. Regression Risk Matrix & Verification

### 1.1 Non-Destructive File Safety Regression
- **Verification Area:** Does the application ever delete or overwrite a user file unintentionally?
- **Checks:**
  1. `unique_dest()` ensures that moving `report.pdf` into a directory that already contains `report.pdf` produces `report (1).pdf` rather than overwriting.
  2. `trash_files()` explicitly calls OS recycle bin APIs (`send2trash`) and blocks deletion on system roots.
  3. `undo_moves()` reverts file moves in reverse order and prunes empty intermediate folders created by the operation.
- **Result:** **PASSED.** Zero regressions found.

### 1.2 System Path Traversal Regression
- **Verification Area:** Can files in system directories be manipulated via custom organizer rules or bulk renamer?
- **Checks:**
  1. `core.protected_reason()` evaluated against Windows (`WINDIR`, `ProgramFiles`, `ProgramData`) and Unix (`/usr`, `/System`, `/Library`, `/Applications`, `/bin`, `/etc`).
  2. Submitting `/usr` or `C:\Windows` to `scan_organizer()` immediately halts execution and surfaces a user warning.
- **Result:** **PASSED.** Zero regressions found.

### 1.3 `QSettings` Cross-Test State Leakage
- **Verification Area:** Does running tests modify the host user's preferences?
- **Root Cause Analyzed:** macOS `QSettings` uses `CFPreferences`, bypassing `os.environ["HOME"]`.
- **Fix Implemented:** `setUp()` and `tearDown()` in `tests/test_ui.py` explicitly reset all mutated settings (`org_mode`, `theme`, `lang`).
- **Result:** **PASSED.** Full test suite runs deterministically in any execution order.

### 1.4 Headless `--selftest` in Automated CI
- **Verification Area:** Can the application be launched headlessly without a user entering credentials?
- **Fix Implemented:** Handled `--selftest` in `app.py::main()` to initialize `MainWindow` with a mock session and exit with code 0 after 1.5 seconds.
- **Verification Command:** `QT_QPA_PLATFORM=offscreen .venv/bin/python app.py --selftest`
- **Result:** **PASSED.** Process exits with code 0.
