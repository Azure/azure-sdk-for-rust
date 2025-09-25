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
  [string]$Path = "."
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

  Write-Host "`nRunning tsp-client update in: $dir" -ForegroundColor Cyan

  try {
    Push-Location $dir
    tsp-client update

    if ($LASTEXITCODE -ne 0) {
      Write-Warning "tsp-client update failed in directory: $dir (exit code: $LASTEXITCODE)"
    }
    else {
      Write-Host "Successfully updated: $dir" -ForegroundColor Green
    }
  }
  catch {
    Write-Error "Error running tsp-client update in directory: $dir - $_"
  }
  finally {
    Pop-Location
  }
}

Write-Host "`nCompleted updating all TypeSpec clients." -ForegroundColor Green
