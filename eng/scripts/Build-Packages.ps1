#Requires -Version 7.0

param(
    [string]$TargetingString,
    [string]$RustToolchain = 'stable'
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. "$PSScriptRoot\..\common\scripts\common.ps1"
. (Join-Path $EngCommonScriptsDir "Helpers" CommandInvocation-Helpers.ps1)

Write-Host "Building packages with
  TargetingString: '$TargetingString'
  RustToolchain: '$RustToolchain'"

$allPackages = Get-AllPackageInfoFromRepo

if (!$TargetingString) {
    $packagesToBuild = $allPackages
}
else {
    $targetPattern = [Regex]::Escape($TargetingString.Trim())
    $targetPattern = $targetPattern.Replace('\*', '.*').Replace(",", "|")
    $targetPattern = "^($targetPattern)$"

    $packagesToBuild = $allPackages | Where-Object { $_.Name -match $targetPattern }
}

Write-Host "Building packages:"
foreach ($package in $packagesToBuild) {
    Write-Host "  '$($package.Name)'"
}

#TODO: filter packages to those that match the targeting string
# the targeting string is built by comparing the PR diff to the list of packages

foreach ($package in $packagesToBuild) {
    Push-Location $package.DirectoryPath
    try {
        Write-Host "Building package: '$($package.Name)' in directory: '$($package.DirectoryPath)'"
        Invoke-LoggedCommand "cargo +$RustToolchain build --keep-going"
    }
    finally {
        Pop-Location
    }
}
