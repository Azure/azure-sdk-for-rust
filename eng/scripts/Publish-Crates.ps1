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

foreach ($crateName in $CrateNames) {
  Write-Host "Publishing packae: '$crateName'"
  $manifestPath = "$PackagesPath/$crateName/Cargo.toml"
  # https://doc.rust-lang.org/cargo/reference/registry-web-api.html#publish
  Write-Host "> cargo publish --manifest-path `"$manifestPath`" --token <TOKEN>"
  cargo publish --manifest-path $manifestPath --token $Token
  if (!$?) {
    Write-Error "Failed to publish package: '$crateName'"
    exit 1
  }

  $existingOwners = (Invoke-LoggedCommand cargo owner --list $crateName) -replace " \(.*", ""
  $missingOwners = $AdditionalOwners | Where-Object { $existingOwners -notcontains $_ }

  foreach ($owner in $missingOwners) {
    Write-Host "> cargo owner --add $owner $crateName --token <TOKEN>"
    cargo owner --add $owner $crateName --token $Token
  }
}
