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

function TryAddOwners($crateName) {
  foreach ($owner in $AdditionalOwners) {
    Write-Host "Adding owner: '$owner' to crate: '$crateName'"
    # https://doc.rust-lang.org/cargo/reference/registry-web-api.html#owners-add
    # ignore errors is owner already exists
    $body = @{ users = @($owner) } | ConvertTo-Json

    $response = Invoke-WebRequest -Method Put -Uri "https://crates.io/api/v1/crates/$crateName/owners" `
      -Headers @{ Accept = 'application/json'; Authorization = $Token } `
      -ContentType 'application/json' `
      -Body $body`
      -SkipHttpErrorCheck

    if ($response.StatusCode -ge 400 -and $response.Content -notmatch 'already an owner') {
      Write-Host "Failed to add owner: '$owner' to crate: '$crateName'"
      Write-Host "Response: $($response.Content)"
      exit 1
    }
  }
}

foreach ($crateName in $CrateNames) {
  Write-Host "Publishing packae: '$crateName'"
  # https://doc.rust-lang.org/cargo/reference/registry-web-api.html#publish
  Invoke-WebRequest -Method Put -Uri 'https://crates.io/api/v1/crates/new' `
    -Headers @{ Accept = 'application/json'; Authorization = $Token } `
    -ContentType 'application/json' `
    -InFile "$PackagesPath/$crateName/cargo-put.bin"

  TryAddOwners $crateName
}
