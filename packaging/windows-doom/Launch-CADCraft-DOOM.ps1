param(
    [string]$WadPath,
    [switch]$ValidateOnly,
    [switch]$NoDialog,
    [switch]$NoPause
)
$ErrorActionPreference = 'Stop'
$executable = Join-Path $PSScriptRoot 'CADCraft-DOOM.exe'
$logPath = Join-Path $PSScriptRoot 'Launch-CADCraft-DOOM.log'

function Test-CompatibleIwad([string]$Path) {
    $stream = [IO.File]::OpenRead($Path)
    $reader = [IO.BinaryReader]::new($stream)
    try {
        if ($stream.Length -lt 12 -or $stream.Length -gt 134217728) { throw 'Invalid IWAD size.' }
        if ([Text.Encoding]::ASCII.GetString($reader.ReadBytes(4)) -ne 'IWAD') { throw 'The file is not an IWAD.' }
        $count = $reader.ReadInt32()
        $directory = $reader.ReadInt32()
        if ($count -lt 1 -or $count -gt 65536 -or $directory -lt 12 -or ([long]$directory + [long]$count * 16) -gt $stream.Length) { throw 'Invalid IWAD directory.' }
        $stream.Position = $directory
        $names = @{}
        for ($i=0; $i -lt $count; $i++) {
            $offset = $reader.ReadInt32(); $size = $reader.ReadInt32()
            if ($offset -lt 0 -or $size -lt 0 -or ([long]$offset + $size) -gt $stream.Length) { throw "Invalid IWAD lump $i." }
            $name = [Text.Encoding]::ASCII.GetString($reader.ReadBytes(8)).TrimEnd([char]0).ToUpperInvariant()
            $names[$name] = $true
        }
        if (!$names.ContainsKey('E1M1') -and !$names.ContainsKey('MAP01')) {
            throw 'IWAD has no supported first map (E1M1 or MAP01).'
        }
    } finally { $reader.Dispose() }
}

try {
    if (!(Test-Path -LiteralPath $executable -PathType Leaf)) {
        throw "CADCraft-DOOM.exe is missing. Keep the EXE and launcher in the same folder. No compilation is performed by this launcher. Expected: $executable"
    }
    if (!$WadPath -and (Test-Path -LiteralPath (Join-Path $PSScriptRoot 'DOOM1.WAD') -PathType Leaf)) {
        $WadPath = Join-Path $PSScriptRoot 'DOOM1.WAD'
    }
    if (!$WadPath) { $WadPath = $env:CADCRAFT_DOOM_WAD }
    if ($WadPath -and (Test-Path -LiteralPath $WadPath -PathType Container)) { $WadPath = Join-Path $WadPath 'DOOM1.WAD' }
    if ($WadPath -and !(Test-Path -LiteralPath $WadPath -PathType Leaf)) {
        throw "The selected IWAD does not exist: $WadPath"
    }
    if (!$WadPath -and !$NoDialog -and !$ValidateOnly) {
        Add-Type -AssemblyName System.Windows.Forms
        $dialog = [Windows.Forms.OpenFileDialog]::new()
        $dialog.Title = 'Select your own Doom IWAD (Cancel starts CADCraft without Doom)'
        $dialog.Filter = 'Doom IWAD (*.wad)|*.wad|All files (*.*)|*.*'
        $dialog.InitialDirectory = $PSScriptRoot
        try {
            if ($dialog.ShowDialog() -eq [Windows.Forms.DialogResult]::OK) { $WadPath = $dialog.FileName }
        } finally { $dialog.Dispose() }
    }
    if ($WadPath) {
        $WadPath = (Resolve-Path -LiteralPath $WadPath).ProviderPath
        Test-CompatibleIwad $WadPath
        $env:CADCRAFT_DOOM_WAD = $WadPath
        Add-Content -LiteralPath $logPath -Value "$(Get-Date -Format o) User-selected IWAD=$WadPath"
    } else {
        Remove-Item Env:CADCRAFT_DOOM_WAD -ErrorAction SilentlyContinue
        Write-Host 'No IWAD selected. CADCraft is available; DOOM explains how to supply game data.'
    }
    if ($ValidateOnly) {
        Write-Output 'PASS: EXE exists; supplied IWAD checked; CAD-only startup permitted without game data.'
        exit 0
    }
    $env:CADCRAFT_DOOM_DATA = Join-Path $PSScriptRoot 'run-data'
    $env:CADCRAFT_CONFIG_DIR = Join-Path $PSScriptRoot 'profile'
    Write-Host 'Starting CADCraft DOOM (unofficial community modification).'
    Write-Host 'Type DOOM then Enter inside CADCraft. Click the game to capture the keyboard.'
    Write-Host 'Arrows/WASD move/turn; Ctrl fires; Space uses; Shift runs; Esc opens the game menu.'
    Write-Host 'Exit game: F12, Exit DOOM button, or type DOOMEXIT in its Command field.'
    Write-Host 'Keep this launcher open. It records startup failures and waits until CADCraft closes.'
    $process = Start-Process -FilePath $executable -WorkingDirectory $PSScriptRoot -ArgumentList '--sample' -PassThru
    Add-Content -LiteralPath $logPath -Value "$(Get-Date -Format o) Started PID=$($process.Id)"
    $process.WaitForExit()
    $code = $process.ExitCode
    Add-Content -LiteralPath $logPath -Value "$(Get-Date -Format o) ExitCode=$code"
    if ($code -ne 0) { throw "CADCraft exited with code $code. See profile\logs\cadcraft.log. If Windows reports a missing VCRUNTIME140.dll, install Microsoft's current Visual C++ v14 x64 Redistributable." }
    Write-Host 'CADCraft closed normally. You can double-click the BAT file to launch it again.'
    exit 0
} catch {
    $message = $_.Exception.Message
    Write-Host "ERROR: $message" -ForegroundColor Red
    try { Add-Content -LiteralPath $logPath -Value "$(Get-Date -Format o) ERROR: $message" } catch { }
    if (!$NoPause) { [void](Read-Host 'Press Enter to close this error message') }
    exit 1
}
