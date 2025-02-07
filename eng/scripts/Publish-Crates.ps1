#!/usr/bin/env pwsh

#Requires -Version 7.0
param(
  [string]$PackagesPath,
  [string[]]$PackageNames,
  [string[]]$AdditionalOwners,
  [string]$Token
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

function TryAddOwners($packageName) {
  foreach ($owner in $AdditionalOwners) {
    Write-Host "Adding owner: '$owner' to package: '$packageName'"
    # https://doc.rust-lang.org/cargo/reference/registry-web-api.html#owners-add
    # ignore errors is owner already exists
    $body = @{ users = @($owner) } | ConvertTo-Json

    $response = Invoke-WebRequest -Method Put -Uri "https://crates.io/api/v1/crates/$packageName/owners" `
      -Headers @{ Accept = 'application/json'; Authorization = $Token } `
      -ContentType 'application/json' `
      -Body $body`
      -SkipHttpErrorCheck

    if ($response.StatusCode -ge 400 -and $response.Content -notmatch 'already an owner') {
      Write-Host "Failed to add owner: '$owner' to package: '$packageName'"
      Write-Host "Response: $($response.Content)"
      exit 1
    }
  }
}

foreach ($packageName in $packageNames) {
  Write-Host "Publishing packae: '$packageName'"
  # https://doc.rust-lang.org/cargo/reference/registry-web-api.html#publish
  Invoke-WebRequest -Method Put -Uri 'https://crates.io/api/v1/crates/new' `
    -Headers @{ Accept = 'application/json'; Authorization = $Token } `
    -ContentType 'application/json' `
    -InFile "$PackagesPath/$packageName/cargo-put.bin"

  TryAddOwners $packageName
}
