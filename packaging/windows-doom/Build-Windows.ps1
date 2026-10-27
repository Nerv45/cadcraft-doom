param([string]$CandidateDirectory = 'release-artifacts/local-build')
$ErrorActionPreference = 'Stop'
$root = (Resolve-Path (Join-Path $PSScriptRoot '../..')).Path
Push-Location $root
try {
    if (!(Get-Command cl.exe -ErrorAction SilentlyContinue)) {
        $vswhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio/Installer/vswhere.exe'
        if (!(Test-Path -LiteralPath $vswhere)) { throw 'Install Visual Studio C++ Build Tools and Windows SDK, or use x64 Developer PowerShell.' }
        $install = & $vswhere -latest -products '*' -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
        if (!$install) { throw 'No Visual Studio x64 C++ toolchain found.' }
        & (Join-Path $install 'Common7/Tools/Launch-VsDevShell.ps1') -Arch amd64 -HostArch amd64 -SkipAutomaticLocation
    }
    Remove-Item Env:CRAFT_FONTS_DIR -ErrorAction SilentlyContinue
    Remove-Item Env:CRAFT_FONTS_REQUIRED -ErrorAction SilentlyContinue
    $env:CADCRAFT_REQUIRE_WINRES = '1'
    & cargo build --locked --release --target x86_64-pc-windows-msvc -p cadcraft --features doom
    if ($LASTEXITCODE -ne 0) { throw "Cargo build failed: $LASTEXITCODE" }
    $target = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $root 'target' }
    $exe = Join-Path $target 'x86_64-pc-windows-msvc/release/cadcraft.exe'
    if (!(Test-Path -LiteralPath $exe -PathType Leaf)) { throw "Built EXE missing: $exe" }
    New-Item -ItemType Directory -Force -Path $CandidateDirectory | Out-Null
    Copy-Item -LiteralPath $exe -Destination (Join-Path $CandidateDirectory 'CADCraft-DOOM.exe')
    Copy-Item -Path (Join-Path $PSScriptRoot 'Launch-CADCraft-DOOM.*') -Destination $CandidateDirectory
    Write-Host "Local EXE built: $CandidateDirectory/CADCraft-DOOM.exe"
    Write-Host 'Not a cleared public release. Read LICENSE and run the packaging gate.'
} finally { Pop-Location }
