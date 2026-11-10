@echo off
REM Builds Windows application package: dist\SmartFileManagerPro
cd /d "%~dp0"
if not exist .venv py -m venv .venv
call .venv\Scripts\activate.bat
python -m pip install --upgrade pip
pip install -r requirements-dev.txt
pyinstaller --noconfirm smartFileManagerPro.spec
if exist .env copy /Y .env dist\SmartFileManagerPro\.env >nul
echo.
echo Done. Application bundle is in dist\SmartFileManagerPro\
echo Run installer.iss with Inno Setup to create SmartFileManagerPro-Setup.exe
pause
