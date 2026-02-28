#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

<#
.SYNOPSIS
    Updates eng/emitter-package.json to the latest @azure-tools/typespec-rust version.

.DESCRIPTION
    Queries npmjs.org for the latest version of @azure-tools/typespec-rust, fetches
    its package.json from GitHub, and updates any matching devDependencies in our
    eng/emitter-package.json. Runs 'tsp-client generate-lock-file' afterward.

.PARAMETER Regenerate
    If set, recursively finds each tsp-location.yaml under sdk/ and runs
    'tsp-client update' in its parent directory.

.EXAMPLE
    .\Update-Emitter.ps1
    Updates emitter-package.json and regenerates the lock file.

.EXAMPLE
    .\Update-Emitter.ps1 -Regenerate
    Updates emitter-package.json, regenerates the lock file, then regenerates all SDK clients.
#>

param(
  [switch]$Regenerate
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))

$EmitterPackagePath = ([System.IO.Path]::Combine($RepoRoot, 'eng', 'emitter-package.json'))
$TspClientDir = ([System.IO.Path]::Combine($RepoRoot, 'eng', 'common', 'tsp-client'))

# Install tsp-client if not already in PATH.
if (-not (Get-Command tsp-client -ErrorAction SilentlyContinue)) {
  Invoke-LoggedCommand "npm ci --no-fund --no-audit --prefix $TspClientDir" -GroupOutput
}

function Invoke-TspClient {
  param([string[]]$Arguments)

  $cmd = if (Get-Command tsp-client -ErrorAction SilentlyContinue) {
    "tsp-client $($Arguments -join ' ')"
  } else {
    "npm exec --prefix $TspClientDir --no -- tsp-client $($Arguments -join ' ')"
  }

  Invoke-LoggedCommand $cmd -GroupOutput
}

# Query npmjs.org for the latest version of @azure-tools/typespec-rust.
Write-Host "Querying npmjs.org for latest @azure-tools/typespec-rust version"
$npmData = Invoke-RestMethod -Uri 'https://registry.npmjs.org/@azure-tools/typespec-rust/latest'
$latestVersion = $npmData.version
Write-Host "Latest version: $latestVersion"

# Fetch the upstream package.json from the corresponding GitHub tag.
$packageJsonUrl = "https://raw.githubusercontent.com/Azure/typespec-rust/v$latestVersion/packages/typespec-rust/package.json"
Write-Host "Fetching upstream package.json from v$latestVersion tag"
$upstreamPackage = Invoke-RestMethod -Uri $packageJsonUrl
$upstreamDevDeps = $upstreamPackage.devDependencies

# Read our emitter-package.json and check if an update is needed.
$emitterPackage = Get-Content $EmitterPackagePath -Raw | ConvertFrom-Json
$currentVersion = $emitterPackage.dependencies.'@azure-tools/typespec-rust'

if ($currentVersion -eq $latestVersion) {
  Write-Host "Already up to date ($currentVersion)"
} else {
  Write-Host "Updating @azure-tools/typespec-rust: $currentVersion -> $latestVersion"
  $emitterPackage.dependencies.'@azure-tools/typespec-rust' = $latestVersion

  # Update intersection of upstream devDependencies with our devDependencies.
  foreach ($prop in $emitterPackage.devDependencies.PSObject.Properties) {
    $upstreamProp = $upstreamDevDeps.PSObject.Properties[$prop.Name]
    if ($upstreamProp -and $prop.Value -ne $upstreamProp.Value) {
      Write-Host "Updating devDependency $($prop.Name): $($prop.Value) -> $($upstreamProp.Value)"
      $prop.Value = $upstreamProp.Value
    }
  }

  Write-Host "Writing updated emitter-package.json"
  $emitterPackage | ConvertTo-Json -Depth 10 | Set-Content -Path $EmitterPackagePath

  # Regenerate the lock file after updating.
  Invoke-TspClient -Arguments 'generate-lock-file'
}

# Regenerate all SDK clients if requested.
if ($Regenerate) {
  $sdkPath = ([System.IO.Path]::Combine($RepoRoot, 'sdk'))
  $tspFiles = Get-ChildItem -Path $sdkPath -Recurse -Filter 'tsp-location.yaml'

  if ($tspFiles.Count -eq 0) {
    Write-Host "No tsp-location.yaml files found under sdk/"
  } else {
    Write-Host "Found $($tspFiles.Count) tsp-location.yaml file(s) to regenerate"
    foreach ($file in $tspFiles) {
      $dir = $file.DirectoryName
      Push-Location $dir
      try {
        Invoke-TspClient -Arguments 'update'
      } finally {
        Pop-Location
      }
    }
  }
}

Write-Host "Done"
