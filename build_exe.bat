@echo off
REM Builds Windows native release executable: target\release\smart_filee.exe
cd /d "%~dp0"
echo [1/3] Building native Rust release executable with LTO and optimizations...
cargo build --release -p smart_filee_app
if %ERRORLEVEL% neq 0 (
    echo [ERROR] Cargo release build failed!
    pause
    exit /b %ERRORLEVEL%
)

echo [2/3] Verifying release executable self-test...
target\release\smart_filee.exe --selftest
if %ERRORLEVEL% neq 0 (
    echo [ERROR] Headless self-test failed!
    pause
    exit /b %ERRORLEVEL%
)

echo.
echo [3/3] Build completed successfully!
echo Executable: target\release\smart_filee.exe
echo Run installer.iss with Inno Setup to generate SmartFileManagerPro-Setup-v4.3.1.exe
pause
