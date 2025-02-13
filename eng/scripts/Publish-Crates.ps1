#!/usr/bin/env pwsh

#Requires -Version 7.0
param(
  [string]$PackagesPath,
  [string[]]$CrateNames,
  [string[]]$AdditionalOwners,
  [string]$Token
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

$env:CARGO_REGISTRY_TOKEN = $Token

foreach ($crateName in $CrateNames) {
  Write-Host "Publishing packae: '$crateName'"
  $manifestPath = "$PackagesPath/$crateName/Cargo.toml"
  # https://doc.rust-lang.org/cargo/reference/registry-web-api.html#publish
  Write-Host "> cargo publish --manifest-path `"$manifestPath`""
  cargo publish --manifest-path $manifestPath
  if (!$?) {
    Write-Error "Failed to publish package: '$crateName'"
    exit 1
  }

  $existingOwners = (cargo owner --list $crateName) -replace " \(.*", ""
  $missingOwners = $AdditionalOwners | Where-Object { $existingOwners -notcontains $_ }

  foreach ($owner in $missingOwners) {
    Write-Host "> cargo owner --add $owner $crateName"
    cargo owner --add $owner $crateName
  }
}
