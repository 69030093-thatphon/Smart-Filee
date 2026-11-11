@echo off
cd /d "%~dp0"
if exist target\release\smart_filee.exe (
    start "" target\release\smart_filee.exe
) else if exist target\debug\smart_filee.exe (
    start "" target\debug\smart_filee.exe
) else (
    echo Launching Smart-Filee via Cargo...
    cargo run -p smart_filee_app
)
