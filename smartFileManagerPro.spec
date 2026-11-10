
# SmartFileManagerPro.spec

from PyInstaller.utils.hooks import (
    collect_all,
    collect_submodules,
)

datas = [("fonts", "fonts"), ("icon.ico", "."), (".env", ".")]
binaries = []
hiddenimports = []

for package in ["PySide6", "supabase", "httpx"]:
    package_datas, package_binaries, package_hidden = (
        collect_all(package)
    )
    datas += package_datas
    binaries += package_binaries
    hiddenimports += package_hidden

hiddenimports += collect_submodules("PySide6")

a = Analysis(
    ["app.py"],
    pathex=["."],
    binaries=binaries,
    datas=datas,
    hiddenimports=hiddenimports,
    hookspath=[],
    hooksconfig={},
    runtime_hooks=[],
    excludes=[],
    noarchive=False,
)

pyz = PYZ(a.pure)

exe = EXE(
    pyz,
    a.scripts,
    [],
    exclude_binaries=True,
    name="SmartFileManagerPro",
    icon="icon.ico",
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=False,
    console=False,
    disable_windowed_traceback=False,
)

coll = COLLECT(
    exe,
    a.binaries,
    a.datas,
    strip=False,
    upx=False,
    name="SmartFileManagerPro",
)
