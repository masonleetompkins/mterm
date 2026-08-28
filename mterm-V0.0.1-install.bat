@echo off
REM mterm installer — Windows (double-click to install)
REM No admin needed. Downloads mterm-windows-x64.exe to %USERPROFILE%\.local\bin

setlocal
set REPO=masonleetompkins/mterm
set TARGET=mterm-windows-x64.exe
set INSTALL_DIR=%USERPROFILE%\.local\bin
set URL=https://github.com/%REPO%/releases/latest/download/%TARGET%
set DEST=%INSTALL_DIR%\%TARGET:mterm-windows-=% 
set DEST=%INSTALL_DIR%\mterm.exe

REM handle --dry-run
if "%~1"=="--dry-run" goto :dry
if "%~1"=="-n" goto :dry

echo   ███╗   ███╗████████╗███████╗██████╗ ███╗   ███╗
echo   ████╗ ████║╚══██╔══╝██╔════╝██╔══██╗████╗ ████║
echo   ██╔████╔██║   ██║   █████╗  ██████╔╝██╔████╔██║
echo   ██║╚██╔╝██║   ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║
echo   ██║ ╚═╝ ██║   ██║   ███████╗██║  ██║██║ ╚═╝ ██║
echo   ╚═╝     ╚═╝   ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝
echo    ── installer · Windows · double-click to install ──
echo.
echo [detect] Windows x64
echo [target] %TARGET%
echo [download] %URL%
echo [install]  -^> %DEST%

where mterm >nul 2>&1
if %ERRORLEVEL%==0 (
  echo [found] mterm already installed
  set /p ANS="reinstall? [y/N] "
  if /I not "%ANS%"=="y" exit /b 0
)

if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

echo [downloading]...
powershell -ExecutionPolicy Bypass -Command "try { Invoke-WebRequest -Uri '%URL%' -OutFile '%DEST%' -UseBasicParsing; exit 0 } catch { Write-Error $_; exit 1 }"
if %ERRORLEVEL% neq 0 (
  echo download failed. Try manual: %URL%
  pause
  exit /b 1
)

echo [done] installed to %DEST%
echo.
echo [hint] add to PATH if needed:
echo   setx PATH "%%PATH%%;%INSTALL_DIR%"
echo   or for current session: set PATH=%%PATH%%;%INSTALL_DIR%
echo.
echo try: mterm app examples\runbook.md
pause
exit /b 0

:dry
echo [dry-run] would download %URL% to %DEST%
pause
exit /b 0
