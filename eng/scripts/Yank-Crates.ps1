#!/usr/bin/env pwsh

#Requires -Version 7.0
param(
  [string]$PackagesPath,
  [string[]]$CrateNames,
  [string]$Token
)

$ErrorActionPreference = 'Stop'
#Set-StrictMode -Version 2.0

$hasErrors = $false
foreach ($crateName in $crateNames) {
  $crate = Get-Content "$PackagesPath/$crateName/cargo-metadata.json" -Raw | ConvertFrom-Json
  $crateVersion = $crate.version

  Write-Host "Yanking crate: '$crateName@$crateVersion'"

  # https://doc.rust-lang.org/cargo/reference/registry-web-api.html#yank
  $response = Invoke-WebRequest -Method Delete -Uri "https://crates.io/api/v1/crates/$crateName/$crateVersion/yank" `
    -Headers @{ Accept = 'application/json'; Authorization = $Token } `
    -SkipHttpErrorCheck
  
  if ($response.StatusCode -ge 400) {
    Write-Host "Failed to yank crate: '$crateName@$crateVersion'"
    Write-Host "Response: $($response.Content)"
    $hasErrors = $true
  }
}

if ($hasErrors) {
  exit 1
}
