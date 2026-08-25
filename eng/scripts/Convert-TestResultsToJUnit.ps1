#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0
<#
.SYNOPSIS
Converts cargo test JSON output to JUnit XML format using cargo2junit.

.DESCRIPTION
This script converts the JSON output files from cargo test (captured by Test-Packages.ps1 in CI mode)
to JUnit XML format suitable for publishing to Azure DevOps test results using the cargo2junit tool.

.PARAMETER TestResultsDirectory
The directory containing JSON test result files. Defaults to test-results in the repo root.

.PARAMETER OutputDirectory
The directory where JUnit XML files should be written. Defaults to test-results/junit in the repo root.

.EXAMPLE
./eng/scripts/Convert-TestResultsToJUnit.ps1

.EXAMPLE
./eng/scripts/Convert-TestResultsToJUnit.ps1 -TestResultsDirectory ./test-results -OutputDirectory ./junit-results
#>

param(
  [string]$TestResultsDirectory = ([System.IO.Path]::Combine($PSScriptRoot, '..', '..', 'test-results')),
  [string]$OutputDirectory = ([System.IO.Path]::Combine($PSScriptRoot, '..', '..', 'test-results', 'junit'))
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0
. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'Cargo.ps1'))

Write-Host "##vso[task.setvariable variable=HasJUnitTestResults]false"
Write-Host "Converting test results from JSON to JUnit XML using cargo2junit"
Write-Host "  Input directory:  $TestResultsDirectory"
Write-Host "  Output directory: $OutputDirectory"

if (!(Test-Path $TestResultsDirectory)) {
  Write-Host "Skipping JUnit conversion because the test results directory does not exist."
  exit 0
}

$allJsonFiles = @(Get-ChildItem -Path $TestResultsDirectory -Filter "*.json" -File)
if ($allJsonFiles.Count -eq 0) {
  Write-Host "Skipping JUnit conversion because no JSON test result files were produced."
  exit 0
}

$jsonFiles = @($allJsonFiles | Where-Object { $_.Length -gt 0 })
if ($jsonFiles.Count -eq 0) {
  Write-Host "Skipping JUnit conversion because all JSON test result files are empty."
  exit 0
}

Write-Host "##vso[task.setvariable variable=HasJUnitTestResults]true"

if (!(Test-Path $OutputDirectory)) {
  New-Item -ItemType Directory -Path $OutputDirectory | Out-Null
  Write-Host "Created output directory: $OutputDirectory"
}

$cargo2junitPath = Get-Command cargo2junit -ErrorAction SilentlyContinue
if (!$cargo2junitPath) {
  $cargo2junitVersionParams = Get-VersionParamsFromCgManifest cargo2junit
  Invoke-LoggedCommand "cargo install cargo2junit --locked $($cargo2junitVersionParams -join ' ')" -GroupOutput
}

$succeeded = $true
$commandOutputDir = ([System.IO.Path]::Combine($OutputDirectory, "cargo2junit-errors"))
if (!(Test-Path $commandOutputDir)) {
  New-Item -ItemType Directory -Path $commandOutputDir | Out-Null
}

Write-Host "`nConverting $($jsonFiles.Count) JSON file(s) to JUnit XML..."
foreach ($jsonFile in $jsonFiles) {
  $baseName = [System.IO.Path]::GetFileNameWithoutExtension($jsonFile.Name)
  $junitFile = ([System.IO.Path]::Combine($OutputDirectory, "$baseName.xml"))
  $stderrFile = ([System.IO.Path]::Combine($commandOutputDir, "$baseName-stderr.txt"))

  Write-Host "  Converting: $($jsonFile.Name) -> $([System.IO.Path]::GetFileName($junitFile))"

  $proc = Start-Process cargo2junit `
    -Wait `
    -PassThru `
    -RedirectStandardInput $jsonFile.FullName `
    -RedirectStandardOutput $junitFile `
    -RedirectStandardError $stderrFile
  $exitCode = $proc.ExitCode
  $stderr = @(Get-Content $stderrFile)

  # Always print stderr so it appears in CI logs for debugging
  foreach ($line in $stderr) {
    Write-Host "    stderr: $line"
  }

  if ($exitCode) {
    # cargo2junit exits non-zero when tests fail, not just on conversion errors.
    # Filter out the known "One or more tests failed." message and only treat
    # remaining stderr lines as actual conversion failures.
    $otherErrors = @($stderr | Where-Object { "$_" -notlike '*One or more tests failed.*' })
    if ($otherErrors.Count -gt 0) {
      LogError "Failure during conversion of $($jsonFile.Name) to JUnit XML."
      $succeeded = $false
    }
  }
}

if (-not $succeeded) {
  exit 1
}
exit 0
