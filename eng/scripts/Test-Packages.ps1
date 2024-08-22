#Requires -Version 7.0

param(
    [string]$TargetingString,
    [string]$Toolchain = 'stable',
    [bool]$UnitTests = $true,
    [bool]$FunctionalTests = $true
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. "$PSScriptRoot\..\common\scripts\common.ps1"
. (Join-Path $EngCommonScriptsDir "Helpers" CommandInvocation-Helpers.ps1)

Write-Host "Testing packages with
  TargetingString: '$TargetingString'
  Toolchain: '$Toolchain'"

$allPackages = Get-AllPackageInfoFromRepo

if (!$TargetingString) {
    $pacakgesToTest = $allPackages
}
else {
    $targetPattern = [Regex]::Escape($TargetingString.Trim())
    $targetPattern = $targetPattern.Replace('\*', '.*').Replace(",", "|")
    $targetPattern = "^($targetPattern)$"

    $pacakgesToTest = $allPackages | Where-Object { $_.Name -match $targetPattern }
}

Write-Host "Testing packages:"
foreach ($package in $pacakgesToTest) {
    Write-Host "  '$($package.Name)'"
}

#TODO: filter packages to those that match the targeting string
# the targeting string is built by comparing the PR diff to the list of packages
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
