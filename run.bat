@echo off
cd /d "%~dp0"
if not exist .venv (
    py -m venv .venv
    call .venv\Scripts\activate.bat
    python -m pip install --upgrade pip
) else (
    call .venv\Scripts\activate.bat
)
python -c "import supabase" >nul 2>&1
if errorlevel 1 (
    python -m pip install -r requirements.txt
)
python app.py
