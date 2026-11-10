# Changelog

All notable changes to Smart File Manager Pro will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [4.3.0] - 2026-11-10

### Added
- **Multi-Platform Continuous Integration:** GitHub Actions CI workflow (`.github/workflows/ci.yml`) validating unit, integration, and UI smoke tests across Ubuntu and Windows.
- **Headless `--selftest` Mode:** Non-interactive startup verification in `app.py` enabling fast automated smoke tests in CI environments.
- **Full Thai Localization Parity:** Extended Thai translation dictionary in `i18n_th.py` covering account management, profile editing, and device revocation dialogs.
- **Automated i18n Test Inspection:** Enhanced AST scanner in `tests/test_i18n.py` verifying translation coverage across `account_dialog.py` and `auth_dialog.py`.
- **Architectural Documentation:** System architecture document (`docs/development/ARCHITECTURE.md`) with comprehensive Mermaid diagrams.
- **Quality Assurance Artifacts:** QA Test Plan, Test Report (58/58 passing tests), and Regression Test Report.
- **Security & Performance Guides:** Security Policy, Security Audit Report, Performance Baseline, and Benchmarking Report.
- **DevOps Documentation:** Comprehensive guide for CI/CD, local building, and Inno Setup compilation (`docs/devops/CI_CD_GUIDE.md`).

### Fixed
- **PyInstaller Bundling Assets (`smartFileManagerPro.spec`):** Added `fonts/` directory and `icon.ico` to binary datas, resolving missing font assets in standalone builds.
- **Build Script Alignment (`build_exe.bat`):** Standardized local Windows packaging around the spec file to produce the directory layout required by `installer.iss`.
- **Test Isolation State Leakage (`tests/test_ui.py`):** Resolved `QSettings` persistent state mutation on macOS `CFPreferences`, guaranteeing deterministic test execution order.
- **Repository Safety:** Added root `.gitignore` to prevent secret and artifact leaks.

---

## [4.2.0] - 2026-10-15

### Added
- **Redesigned Slim Icon Sidebar:** 7 core rail tools with a collapsible "More" tools tile.
- **Tidiness Score Gauge:** Visual ring display (0–100) on the dashboard reflecting folder clutter.
- **Dynamic Quick Switcher:** Universal Ctrl+K command palette for fast page and file search navigation.
- **Card-Based Management Layout:** Responsive card layouts for Disk Analyzer, Duplicate Finder, Cleanup, and Bulk Renamer.
