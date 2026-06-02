#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0
param(
  # Stable channel version e.g., "1.95" or "1.95.0".
  # If omitted, the latest release is fetched from GitHub.
  [string] $Version,

  # Nightly channel date e.g., "2026-04-14" or "nightly-2026-04-14".
  # If omitted, the date is extracted from 'rustc --version' output.
  [string] $NightlyVersion
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))

# Determine the stable version to pin.
if (-not $Version) {
  Write-Host "Detecting latest stable Rust version from GitHub..."
  $Version = Invoke-RestMethod 'https://api.github.com/repos/rust-lang/rust/releases/latest' |
    Select-Object -ExpandProperty tag_name
  Write-Host "  Detected version: $Version"
}

# Determine the starting date to search for a working nightly toolchain.
if ($NightlyVersion) {
  # Accept "nightly-2026-04-14" or just "2026-04-14".
  $nightlyDate = [datetime]::ParseExact(
    ($NightlyVersion -replace '^nightly-', ''), 'yyyy-MM-dd', $null
  )
} else {
  Write-Host "Detecting nightly date from 'rustc --version'..."
  $rustcOutput = rustc --version 2>&1
  Write-Host "  $rustcOutput"
  if ($rustcOutput -match '\([\w\d]+ (\d{4}-\d{2}-\d{2})\)') {
    $nightlyDate = [datetime]::ParseExact($Matches[1], 'yyyy-MM-dd', $null)
    Write-Host "  Detected date: $($nightlyDate.ToString('yyyy-MM-dd'))"
  } else {
    LogError "Could not parse a date from: $rustcOutput"
    exit 1
  }
}

# Install a nightly toolchain. If a given date is unavailable, advance one day at a time.
$nightlyChannel = $null
$maxDaysAhead = 30
for ($i = 0; $i -le $maxDaysAhead; $i++) {
  $candidateDate = $nightlyDate.AddDays($i).ToString('yyyy-MM-dd')
  $candidateChannel = "nightly-$candidateDate"
  Write-Host "Trying to install '$candidateChannel'..."
  Invoke-LoggedCommand "rustup toolchain install --no-self-update $candidateChannel" -GroupOutput -DoNotExitOnFailedExitCode
  if ($LASTEXITCODE -eq 0) {
    $nightlyChannel = $candidateChannel
    Write-Host "Successfully installed '$nightlyChannel'."
    break
  }
  LogWarning "Could not install '$candidateChannel', trying next day..."
}

if (-not $nightlyChannel) {
  LogError "Failed to install any nightly toolchain within $maxDaysAhead days of $($nightlyDate.ToString('yyyy-MM-dd'))."
  exit 1
}

# Update rust-toolchain.toml
$toolchainTomlPath = [System.IO.Path]::Combine($RepoRoot, 'rust-toolchain.toml')
Write-Host "Updating '$toolchainTomlPath'..."
$content = Get-Content -Raw $toolchainTomlPath
$updated = $content -replace 'channel = "[^"]+"', "channel = `"$Version`""
if ($updated -eq $content) {
  LogWarning "No 'channel' value was updated in '$toolchainTomlPath'."
}
Set-Content -Path $toolchainTomlPath -Value $updated -NoNewline

# Update the pinned nightly channel in Language-Settings.ps1
$languageSettingsPath = [System.IO.Path]::Combine($RepoRoot, 'eng', 'scripts', 'Language-Settings.ps1')
Write-Host "Updating '$languageSettingsPath'..."
$content = Get-Content -Raw $languageSettingsPath
$updated = $content -replace "'nightly-\d{4}-\d{2}-\d{2}'", "'$nightlyChannel'"
if ($updated -eq $content) {
  LogWarning "No nightly channel was updated in '$languageSettingsPath'."
}
Set-Content -Path $languageSettingsPath -Value $updated -NoNewline

# Update shebang lines in all *.rs scripts under eng/scripts/
$rsScriptsDir = [System.IO.Path]::Combine($RepoRoot, 'eng', 'scripts')
foreach ($rsFile in Get-ChildItem -Path $rsScriptsDir -Filter '*.rs') {
  Write-Host "Updating '$($rsFile.FullName)'..."
  $content = Get-Content -Raw $rsFile.FullName
  $updated = $content -replace 'cargo \+nightly-\d{4}-\d{2}-\d{2}', "cargo +$nightlyChannel"
  if ($updated -eq $content) {
    LogWarning "No nightly channel was updated in '$($rsFile.FullName)'."
  }
  Set-Content -Path $rsFile.FullName -Value $updated -NoNewline
}

Write-Host "Done. Stable: '$Version', nightly: '$nightlyChannel'."
