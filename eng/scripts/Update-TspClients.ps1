#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

<#
.SYNOPSIS
    Runs 'tsp-client update' on every directory containing a tsp-location.yml file.

.DESCRIPTION
    This script searches recursively for all tsp-location.yml files and runs
    'tsp-client update' in each directory that contains one.

.PARAMETER Path
    The root path to search for tsp-location.yml files. Defaults to current directory.

.EXAMPLE
    .\Update-TspClients.ps1
    Runs tsp-client update in all directories with tsp-location.yml files under the current directory.

.EXAMPLE
    .\Update-TspClients.ps1 -Path "C:\my-project"
    Runs tsp-client update in all directories with tsp-location.yml files under C:\my-project.
#>

param(
  [Parameter(Mandatory = $false)]
  [string]$Path = (Join-Path $PSScriptRoot "../../sdk" | Resolve-Path)
)

Write-Host "Searching for tsp-location.yaml files in: $Path" -ForegroundColor Green

$tspFiles = Get-ChildItem -Path $Path -Recurse -Name "tsp-location.yaml"

if ($tspFiles.Count -eq 0) {
  Write-Host "No tsp-location.yaml files found." -ForegroundColor Yellow
  exit 0
}

Write-Host "Found $($tspFiles.Count) tsp-location.yaml file(s)" -ForegroundColor Green
foreach ($file in $tspFiles) {
  $dir = Split-Path $file -Parent
  if ([string]::IsNullOrEmpty($dir)) {
    $dir = "."
  }
  $fullDir = Join-Path $Path $dir

  Write-Host "`nRunning tsp-client update in: $fullDir" -ForegroundColor Cyan

  try {
    tsp-client update --output-dir $fullDir

    if ($LASTEXITCODE -ne 0) {
      Write-Warning "tsp-client update failed in directory: $fullDir (exit code: $LASTEXITCODE)"
    }
    else {
      Write-Host "Successfully updated: $fullDir" -ForegroundColor Green
    }
  }
  catch {
    Write-Error "Error running tsp-client update in directory: $fullDir - $_"
  }
}

Write-Host "`nCompleted updating all TypeSpec clients." -ForegroundColor Green
