#define MyAppName "Smart File Manager Pro"
#define MyAppVersion "4.3.1"
#define MyAppPublisher "Smart File Manager Pro Team"
#define MyAppExeName "SmartFileManagerPro.exe"

[Setup]
AppId={{A7F82D41-91B5-4C92-9E72-3B9A7F420001}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
DefaultDirName={autopf}\Smart File Manager Pro
DefaultGroupName={#MyAppName}
OutputDir=Output
OutputBaseFilename=SmartFileManagerPro-Setup-v4.3.1
SetupIconFile=icon.ico
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern
ArchitecturesInstallIn64BitMode=x64

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"; Flags: unchecked

[Files]
Source: "target\release\smart_filee.exe"; DestDir: "{app}"; DestName: "{#MyAppExeName}"; Flags: ignoreversion
Source: "icon.ico"; DestDir: "{app}"; Flags: ignoreversion
Source: "fonts\*"; DestDir: "{app}\fonts"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{autoprograms}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\icon.ico"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; IconFilename: "{app}\icon.ico"; Tasks: desktopicon

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "Launch Smart File Manager Pro"; Flags: postinstall nowait skipifsilent
