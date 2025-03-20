#!/usr/bin/env pwsh

#Requires -Version 7.0
param(
  [string]$PackageInfoDirectory,
  [string[]]$CrateNames
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

$hasErrors = $false
foreach ($crateName in $crateNames) {
  $crate = Get-Content "$PackageInfoDirectory/$crateName.json" -Raw | ConvertFrom-Json
  $crateVersion = $crate.Version

  Write-Host "> cargo yank $crateName --version $crateVersion"
  cargo yank $crateName --version $crateVersion 2>&1 | Tee-Object -Variable output

  if ($LASTEXITCODE -ne 0) {
    if ($output -match 'status 404 Not Found') {
      Write-Host "Crate '$crateName@$crateVersion' not found. Skipping yank."
    }
    else {
      Write-Host "Error yanking crate: '$crateName@$crateVersion'"
      $hasErrors = $true
    }
  }
  else {
    Write-Host "Successfully yanked crate: '$crateName@$crateVersion'"
  }
}

if ($hasErrors) {
  exit 1
}
