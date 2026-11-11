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

---

## 2. Rust Migration Regression Verification (Phase 9)

### 2.1 Python Reference Suite Zero-Regression
- **Execution Command:** `/Users/x/Desktop/Smart-Filee/.venv/bin/python -m unittest discover -s tests`
- **Result:** **58/58 tests passed** (100% pass rate in 3.22s).
- **Status:** Complete behavioral parity confirmed with Python baseline.

### 2.2 Rust Workspace Regression Suite
- **Execution Command:** `cargo test --workspace`
- **Result:** **50 unit tests + 3 benchmark tests passed** (100% pass rate).
- **Status:** Zero regressions across all 15 crates.

### 2.3 Strict Lint & Static Analysis Audit
- **Execution Command:** `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- **Result:** 0 warnings / 0 errors.

### 2.4 Headless Self-Test Automated Validation
- **Execution Command:** `cargo run -p smart_filee_app -- --selftest`
- **Result:** Process exits with code 0 in 0.11s. All 17 internal component self-tests verified cleanly.
