#Requires -Version 7.0

param(
    [string]$Toolchain = 'stable',
    [bool]$UnitTests = $true,
    [bool]$FunctionalTests = $true,
    [string]$PackageInfoPath
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. "$PSScriptRoot\..\common\scripts\common.ps1"
. (Join-Path $EngCommonScriptsDir "Helpers" CommandInvocation-Helpers.ps1)

Write-Host "Testing packages with
    Toolchain: '$Toolchain'
    UnitTests: '$UnitTests'
    FunctionalTests: '$FunctionalTests'
    PackageInfoPath: '$PackageInfoPath'"

if ($PackageInfoPath) {
    if (!(Test-Path $PackageInfoPath)) {
        Write-Error "Package info path '$PackageInfoPath' does not exist."
        exit 1
    }

    $pacakgesToTest = Get-ChildItem $PackageInfoPath -Filter "*.json" -Recurse
    | ConvertFrom-Json
}
else {
    $pacakgesToTest = Get-AllPackagesInRepo
}

Write-Host "Testing packages:"
foreach ($package in $pacakgesToTest) {
    Write-Host "  '$($package.Name)'"
}

Write-Host "Setting RUSTFLAGS to '-Dwarnings'"
$env:RUSTFLAGS = "-Dwarnings"

foreach ($package in $pacakgesToTest) {
    Push-Location $package.DirectoryPath
    try {
        Write-Host "`n`nTesting package: '$($package.Name)' in directory: '$($package.DirectoryPath)'`n"

        Invoke-LoggedCommand "cargo +$Toolchain build --keep-going"
        Write-Host "`n`n"
        Invoke-LoggedCommand "cargo +$Toolchain test --lib --no-fail-fast"
        Write-Host "`n`n"
        Invoke-LoggedCommand "cargo +$Toolchain test --doc --no-fail-fast"
    }
    finally {
        Pop-Location
    }
}
