#!/usr/bin/env pwsh

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
  [string]$TestResultsDirectory = "$PSScriptRoot/../../test-results",
  [string]$OutputDirectory = "$PSScriptRoot/../../test-results/junit"
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0
. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'Cargo.ps1'))

Write-Host "Converting test results from JSON to JUnit XML using cargo2junit"
Write-Host "  Input directory:  $TestResultsDirectory"
Write-Host "  Output directory: $OutputDirectory"

if (!(Test-Path $TestResultsDirectory)) {
  LogWarning "Test results directory not found: $TestResultsDirectory"
  Write-Host "No test results to convert."
  exit 0
}

$jsonFiles = @(Get-ChildItem -Path $TestResultsDirectory -Filter "*.json" -File)
if ($jsonFiles.Count -eq 0) {
  LogWarning "No JSON files found in $TestResultsDirectory"
  Write-Host "No test results to convert."
  exit 0
}

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
Write-Host "`nConverting $($jsonFiles.Count) JSON file(s) to JUnit XML..."
foreach ($jsonFile in $jsonFiles) {
  $baseName = [System.IO.Path]::GetFileNameWithoutExtension($jsonFile.Name)
  $junitFile = ([System.IO.Path]::Combine($OutputDirectory, "$baseName.xml"))

  Write-Host "  Converting: $($jsonFile.Name) -> $([System.IO.Path]::GetFileName($junitFile))"
  $output = Get-Content $jsonFile.FullName | cargo2junit 2>&1
  $exitCode = $LASTEXITCODE

  # Separate stdout from stderr (native command stderr lines are ErrorRecord objects)
  $stdout = @($output | Where-Object { $_ -isnot [System.Management.Automation.ErrorRecord] })
  $stderr = @($output | Where-Object { $_ -is [System.Management.Automation.ErrorRecord] })

  # Write converted output to the JUnit file
  $stdout | Set-Content $junitFile

  # Always print stderr so it appears in CI logs for debugging
  foreach ($line in $stderr) {
    Write-Host "  stderr: $line"
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
