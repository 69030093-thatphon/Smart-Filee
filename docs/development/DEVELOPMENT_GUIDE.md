# Smart File Manager Pro — Developer Guide

**Document Version:** 1.0.0
**Date:** 2026-11-10
**Target Audience:** Core Developers, Contributors, QA Engineers

---

## 1. Development Environment Setup

### 1.1 Prerequisites
- **Python:** Version 3.11 or 3.12 (Python 3.12 recommended).
- **Git:** Version 2.30+.
- **Operating System:** Windows 10/11, macOS 12+, or Linux (Ubuntu 22.04+).
- **Inno Setup (Windows only):** Required for compiling the `SmartFileManagerPro-Setup.exe` installer.

### 1.2 Initial Workspace Setup
Clone the repository and set up a Python virtual environment:

```bash
# Clone the repository
git clone https://github.com/69030093-thatphon/Smart-Filee.git
cd Smart-Filee

# Create virtual environment
python3 -m venv .venv

# Activate virtual environment
# On macOS/Linux:
source .venv/bin/activate
# On Windows (Command Prompt):
.venv\Scripts\activate.bat
# On Windows (PowerShell):
.\.venv\Scripts\Activate.ps1

# Upgrade pip and install dependencies
pip install --upgrade pip
pip install -r requirements.txt
pip install -r requirements-dev.txt
```

---

## 2. Running the Application

### Normal Mode (with Cloud Identity)
Ensure `.env` exists with valid Supabase public keys:
```bash
python app.py
```

### Headless Smoke Test / Self-Test
To verify application initialization without opening interactive modal windows:
```bash
python app.py --selftest
```

---

## 3. Running Automated Tests

Run all unit and integration tests using Python's standard `unittest` runner:

```bash
# Run all tests
python -m unittest discover -s tests -v

# Run core domain tests only (no GUI required)
python -m unittest tests/test_core.py -v

# Run database persistence tests
python -m unittest tests/test_db.py -v

# Run loopback callback tests
python -m unittest tests/test_email_callback.py -v

# Run translation integrity tests
python -m unittest tests/test_i18n.py -v

# Run headless UI tests (uses offscreen QPA platform)
python -m unittest tests/test_ui.py -v
```

---

## 4. Coding & Architecture Conventions

### 4.1 Separation of Concerns
1. **Never import PySide6 in `core.py`:** Core domain functions must remain pure Python to guarantee headless testability without display drivers.
2. **Accept Cancellation & Progress Callbacks:** Long-running domain functions must accept `cancel: Callable[[], bool]` and `progress: Callable[[str], None]`.
3. **Thread Safety in `db.py`:** Always use `with self.lock:` around SQLite cursor operations.
4. **Never Hardcode Secrets:** Only public publishable keys belong in `.env`.

### 4.2 Internationalization (i18n)
All user-facing strings in UI components must be routed through the translation system:
```python
from i18n import tr

# Plain string
label = QLabel(tr("Scan duplicates"))

# Formatted string with named placeholders
message = tr("Moving {current}/{total} files…", current=i, total=count)
```
When adding a new UI string:
1. Wrap the English template with `tr("...")` or use custom widgets `TLabel`, `TButton`, `TCheck`.
2. Add the corresponding Thai translation in `i18n_th.py` under the `TH` dictionary.
3. Run `python -m unittest tests/test_i18n.py -v` to verify translation parity.

### 4.3 Git Commit Conventions
Follow Conventional Commits:
- `feat:` New user functionality.
- `fix:` Bug fix.
- `refactor:` Code change that neither fixes a bug nor adds a feature.
- `test:` Adding or updating tests.
- `docs:` Documentation additions or updates.
- `style:` Formatting or UI styling changes.
- `perf:` Performance improvements.
- `ci:` Changes to CI/CD workflows and packaging.

---

## 5. Building and Packaging

### 5.1 Building Directory Package with PyInstaller
```bash
pyinstaller --noconfirm smartFileManagerPro.spec
```
Output files will be generated in `dist/SmartFileManagerPro/`.

### 5.2 Building Inno Setup Windows Installer
Run Inno Setup Compiler against `installer.iss`:
```powershell
iscc installer.iss
```
Output executable: `Output/SmartFileManagerPro-Setup.exe`.
