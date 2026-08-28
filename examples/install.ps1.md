<# 
  mterm installer — polyglot PowerShell + markdown
  This file is BOTH a markdown document and an executable installer.
  - Open in mterm: renders as doc with [Run] buttons
  - Run as PowerShell: powershell -ExecutionPolicy Bypass -File examples/install.ps1.md
    or double-click (after associating .ps1.md with PowerShell)
#>

$ErrorActionPreference = "Stop"
$Repo = "masonleetompkins/mterm"  # <-- change to your github org/repo
$BinName = "mterm.exe"
$InstallDir = if ($env:MTERM_INSTALL_DIR) { $env:MTERM_INSTALL_DIR } else { "$env:USERPROFILE\.local\bin" }
$DryRun = $args -contains "--dry-run" -or $args -contains "-n"

Write-Host "  ███╗   ███╗████████╗███████╗██████╗ ███╗   ███╗"
Write-Host "  ████╗ ████║╚══██╔══╝██╔════╝██╔══██╗████╗ ████║"
Write-Host "  ██╔████╔██║   ██║   █████╗  ██████╔╝██╔████╔██║"
Write-Host "  ██║╚██╔╝██║   ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║"
Write-Host "  ██║ ╚═╝ ██║   ██║   ███████╗██║  ██║██║ ╚═╝ ██║"
Write-Host "  ╚═╝     ╚═╝   ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝"
Write-Host "   ── installer · Windows · checks system and installs mterm ──"
Write-Host ""

$Arch = if ([Environment]::Is64BitOperatingSystem) { "x64" } else { "x86" }
$Os = "windows"
$Target = "mterm-windows-$Arch.exe"
Write-Host "[detect] OS=$Os ARCH=$Arch"
Write-Host "[target] $Target"

$Url = "https://github.com/$Repo/releases/latest/download/$Target"
$Dest = Join-Path $InstallDir $BinName
Write-Host "[download] $Url"
Write-Host "[install]  -> $Dest"

if ($DryRun) {
  Write-Host "[dry-run] would Invoke-WebRequest $Url -OutFile $Dest"
  exit 0
}

# check existing
try { $Existing = Get-Command mterm -ErrorAction Stop; Write-Host "[found] $($Existing.Source) -> $(mterm --version 2>$null)" ; $Ans = Read-Host "reinstall? [y/N]"; if ($Ans -ne "y" -and $Ans -ne "Y") { Write-Host "abort"; exit 0 } } catch {}

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
try {
  Invoke-WebRequest -Uri $Url -OutFile $Dest -UseBasicParsing
} catch {
  Write-Error "download failed: $_`nTry: winget install mterm or download manually from https://github.com/$Repo/releases"
  exit 1
}

Write-Host "[done] installed to $Dest"

# PATH hint
if ($env:PATH -notlike "*$InstallDir*") {
  Write-Host "[hint] add to PATH: setx PATH `"%PATH%;$InstallDir`"  (restart terminal)"
  Write-Host "       or: `$env:PATH += `";$InstallDir`"  (current session)"
}

Write-Host ""
Write-Host "try: mterm app examples\runbook.md"
exit 0

# __MTERM_MD_BELOW__  markdown starts here, PowerShell has already exited
---
mterm:
  permissions:
    shell: ["powershell *", "pwsh *"]
---

# Install mterm — Windows

This file *is* the installer.

**How to install — pick one:**

> [!BUTTON] Install mterm now (Windows)

```ps1 :run id=install-win
powershell -ExecutionPolicy Bypass -File examples/install.ps1.md
```

> [!BUTTON] Dry run (check what it would do)

```ps1 :run id=dry-win
powershell -ExecutionPolicy Bypass -File examples/install.ps1.md --dry-run
```

**What it does:**
1. Detects `Windows x64` (arm64 coming)
2. Downloads `mterm-windows-x64.exe` from GitHub Releases
3. Installs to `%USERPROFILE%\.local\bin\mterm.exe` (no admin)
4. Hints to add to PATH if needed

**Manual fallback (PowerShell):**
```powershell
Invoke-WebRequest https://github.com/masonleetompkins/mterm/releases/latest/download/mterm-windows-x64.exe -OutFile $env:USERPROFILE\.local\bin\mterm.exe
```

> [!NOTE]
> Polyglot PowerShell + markdown — open this in `mterm` for buttons, or run as PowerShell for install.
