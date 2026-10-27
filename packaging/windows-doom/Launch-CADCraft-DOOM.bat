@echo off
setlocal
cd /d "%~dp0"
if not exist "%~dp0Launch-CADCraft-DOOM.ps1" (
  echo ERROR: Launch-CADCraft-DOOM.ps1 is missing. Keep the launcher files together.
  pause
  exit /b 1
)
rem Execution policy applies only to this process; no machine/user settings change.
"%SystemRoot%\System32\WindowsPowerShell\v1.0\powershell.exe" -NoLogo -NoProfile -ExecutionPolicy Bypass -STA -File "%~dp0Launch-CADCraft-DOOM.ps1" -NoPause %*
set "result=%errorlevel%"
if not "%result%"=="0" (
  echo.
  echo CADCraft DOOM failed to start. See Launch-CADCraft-DOOM.log and profile\logs.
  pause
)
exit /b %result%
