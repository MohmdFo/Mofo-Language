[Setup]
AppName=Mofo Language
AppVersion=0.1.0
DefaultDirName={pf}\MofoLanguage
DefaultGroupName=Mofo Language
OutputBaseFilename=setup
Compression=lzma
SolidCompression=yes

[Files]
; Copy the binary to the installation directory
Source: "target\release\mofo.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
; Create a desktop shortcut
Name: "{userdesktop}\Mofo Language"; Filename: "{app}\mofo.exe"
; Create a Start Menu shortcut
Name: "{group}\Mofo Language"; Filename: "{app}\mofo.exe"

[Run]
; Run a test command to verify installation
Filename: "{app}\mofo.exe"; Description: "Run Mofo Language to test installation"; Flags: shellexec
