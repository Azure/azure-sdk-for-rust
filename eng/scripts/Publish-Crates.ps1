#!/usr/bin/env pwsh

#Requires -Version 7.0
param(
  [string]$PackagesPath,
  [string[]]$CrateNames,
  [string[]]$AdditionalOwners,
  [switch]$DryRun,
  [switch]$Verify
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

foreach ($crateName in $CrateNames) {
  Write-Host "Publishing packae: '$crateName'"
  $manifestPath = "$PackagesPath/$crateName/Cargo.toml"
  # https://doc.rust-lang.org/cargo/reference/registry-web-api.html#publish
  $command = "cargo publish --locked --manifest-path '$manifestPath'"
  if ($DryRun) { $command += ' --dry-run' }
  if (!$Verify) { $command += ' --no-verify' }

  Write-Host "> $command"
  Invoke-Expression $command

  if (!$?) {
    Write-Error "Failed to publish package: '$crateName'"
    exit 1
  }

  if ($AdditionalOwners) {
    $existingOwners = (cargo owner --list $crateName) -replace " \(.*", ""
    $missingOwners = $AdditionalOwners | Where-Object { $existingOwners -notcontains $_ }

    foreach ($owner in $missingOwners) {
      Write-Host "> cargo owner --add $owner $crateName"
      cargo owner --add $owner $crateName
    }
  }
}
