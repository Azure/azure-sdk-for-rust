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

  Write-Host "Yanking crate: '$crateName@$crateVersion'"

  Write-Host "cargo yank $crateName --version $crateVersion"
  cargo yank $crateName --version $crateVersion

  if (!$?) {
    Write-Host "Failed to yank crate: '$crateName@$crateVersion'"
    $hasErrors = $true
  }
}

if ($hasErrors) {
  exit 1
}
