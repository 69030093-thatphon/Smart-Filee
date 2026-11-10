# Architectural Decision Records (ADR) — Smart File Manager Pro

**Project:** Smart File Manager Pro
**Documentation Owner:** นายปฏิภาณ พิพัฒน์พิทยากุล (`team/docs`)
**Lead Developer:** ทัตพร สีโส (`team/dev`)

---

## ADR-001: Git Branching Hierarchy and Remote Management

### Context
The repository needs a clean, multi-contributor workflow supporting distinct team roles (Developer, UI Designer, QA Tester, Documentation) without directly modifying `main` or causing integration chaos.

### Problem
Direct commits to `main` risk introducing untested changes and violating release control policies. Single-branch development creates conflicts between documentation, tests, and application logic.

### Options Considered
1. **Trunk-Based Development:** All commits directly to `main` with feature flags.
2. **Git Flow:** Strict `main`, `develop`, feature, release, and hotfix branches.
3. **Role-Based Team Branching Strategy (Specified in AGENT.md):** `main` -> `develop` -> `team/{dev, ui-design, tester, docs}` with task branches.

### Chosen Solution
Option 3: Role-based team branching strategy.
- `main`: Production-ready release branch (merges strictly require human approval).
- `develop`: Central integration branch.
- `team/dev`, `team/ui-design`, `team/tester`, `team/docs`: Dedicated long-lived branches for each team member.
- Short-lived task branches: `docs/*`, `feature/*`, `fix/*`, `test/*`.

### Consequences
- Unambiguous change ownership.
- Clear audit trail for Pull Requests into `develop`.
- Zero risk to `main`.

---

## ADR-002: Test Suite Isolation for Qt Settings (`QSettings`)

### Context
`MainWindow` reads and persists settings (such as `org_mode`, `theme`, `lang`, `font_size`) into `QSettings("SmartFileManagerPro", "SmartFileManagerPro")`. On macOS, `QSettings` uses `CFPreferences` (system defaults in `~/Library/Preferences/`), ignoring `os.environ["HOME"]`.

### Problem
Tests that modify settings (e.g., `test_organizer_preview_...` setting `org_mode` to "By month") mutate the developer's system preferences, causing subsequent test cases (`test_organize_then_undo`, `test_custom_rule_applies...`, `test_auto_organize...`) to fail unpredictably.

### Options Considered
1. **Manual Reset in Tests:** Explicitly reset settings in `tearDown()` of each test.
2. **File-Based INI Format in Temp Directory:** Configure `QSettings` to use `QSettings.IniFormat` with a temporary file path during tests or initialize `QSettings.setDefaultFormat(QSettings.IniFormat)`.
3. **Dependency Injection:** Allow `MainWindow` to accept an explicit settings object or in-memory dictionary.

### Chosen Solution
Combine Option 1 and Option 2:
- Configure `test_ui.py` to reset `QSettings` keys to known defaults in `setUp()` and `tearDown()`.
- Ensure clean teardown so test order independence is guaranteed across Linux, Windows, and macOS.

### Consequences
- Guarantees 100% deterministic test execution across all OS platforms.
- Prevents development machine preference pollution.

---

## ADR-003: Packaging and Asset Bundling Harmonization

### Context
The Windows executable can be built as a single file (`--onefile` in `build_exe.bat`) or a directory build (`COLLECT` in `smartFileManagerPro.spec`). Inno Setup (`installer.iss`) expects directory contents in `dist\SmartFileManagerPro\*`.

### Problem
`smartFileManagerPro.spec` was missing the `fonts/` directory and `icon.ico` in `datas`, causing missing font assets when launched from the directory build. Running `build_exe.bat` produces a single file that Inno Setup cannot find.

### Options Considered
1. **Single-file executable only:** Drop Inno Setup and distribute only `dist\SmartFileManagerPro.exe`.
2. **Directory build + Inno Setup installer:** Update `smartFileManagerPro.spec` to bundle `fonts/` and `icon.ico`, aligning with `installer.iss`. Update `build_exe.bat` to invoke PyInstaller using the spec file.

### Chosen Solution
Option 2: Harmonize around `smartFileManagerPro.spec` and Inno Setup.
- Update `smartFileManagerPro.spec` to add `("fonts", "fonts")` and `("icon.ico", ".")` to `datas`.
- Set `icon="icon.ico"` in `EXE()`.
- Update `build_exe.bat` to run `pyinstaller --noconfirm smartFileManagerPro.spec`.

### Consequences
- Fast application launch (no temporary extraction overhead on every launch).
- Professional Windows setup experience with desktop shortcut and uninstaller via Inno Setup.
- Identical build behavior between local developer scripts and GitHub Actions CI.
