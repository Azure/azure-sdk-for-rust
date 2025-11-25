#!/usr/bin/env pwsh

#Requires -Version 7.0
param(
  [string]$PackageName,
  [string]$DirectoryPath,
  [string[]]$CrateTypes
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0
. "$PSScriptRoot/../common/scripts/common.ps1"

$DirectoryPath = ([System.IO.Path]::Combine($RepoRoot, $DirectoryPath))

Write-Host "Testing package: '$PackageName' in directory: '$DirectoryPath'"

Set-Location $DirectoryPath

$env:THEENV = "INNER"
Write-Host "Current THEENV: '$env:THEENV"

$setupScript = Join-Path $DirectoryPath "Test-Setup.ps1"
if (Test-Path $setupScript) {
  Write-Host "`n`nRunning test setup script for package: '$PackageName'`n"
  Invoke-LoggedCommand $setupScript -GroupOutput
  if (!$? -ne 0) {
    Write-Error "Test setup script failed for package: '$PackageName'"
    exit 1
  }
}

Write-Host "`n`nTesting package: '$PackageName'`n"

Invoke-LoggedCommand "cargo build --keep-going" -GroupOutput
Write-Host "`n`n"

# Doc tests are only applicable for library crates.
if ($CrateTypes -contains 'lib') {
  Invoke-LoggedCommand "cargo test --doc --no-fail-fast" -GroupOutput
  Write-Host "`n`n"
}

Invoke-LoggedCommand "cargo test --all-targets --no-fail-fast" -GroupOutput
Write-Host "`n`n"

# If the package has an additional test script, run it now.
$additionalTestScript = Join-Path $DirectoryPath "Test-Additional.ps1"
if (Test-Path $additionalTestScript) {
  Write-Host "`n`nRunning additional test script for package: '$PackageName'`n"
  Invoke-LoggedCommand $additionalTestScript -GroupOutput
  if (!$? -ne 0) {
    Write-Error "Additional test script failed for package: '$PackageName'"
    exit 1
  }
}

$cleanupScript = Join-Path $DirectoryPath "Test-Cleanup.ps1"
if (Test-Path $cleanupScript) {
  Write-Host "`n`nRunning test cleanup script for package: '$PackageName'`n"
  Invoke-LoggedCommand $cleanupScript -GroupOutput
  # We ignore the exit code of the cleanup script.
}
