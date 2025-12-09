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
  [string]$TestResultsDirectory = "$PSScriptRoot../../test-results",
  [string]$OutputDirectory = "$PSScriptRoot../../test-results/junit"
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0
. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))

# Set default directories (must be after sourcing common.ps1 which defines $RepoRoot)
if (!$TestResultsDirectory) {
  $TestResultsDirectory = ([System.IO.Path]::Combine($RepoRoot, 'test-results'))
}

if (!$OutputDirectory) {
  $OutputDirectory = ([System.IO.Path]::Combine($RepoRoot, 'test-results', 'junit'))
}

Write-Host "Converting test results from JSON to JUnit XML using cargo2junit"
Write-Host "  Input directory:  $TestResultsDirectory"
Write-Host "  Output directory: $OutputDirectory"

# Check if test results directory exists
if (!(Test-Path $TestResultsDirectory)) {
  LogWarning "Test results directory not found: $TestResultsDirectory"
  Write-Host "No test results to convert."
  exit 0
}

# Create output directory if it doesn't exist
if (!(Test-Path $OutputDirectory)) {
  New-Item -ItemType Directory -Path $OutputDirectory | Out-Null
  Write-Host "Created output directory: $OutputDirectory"
}

# Check if cargo2junit is installed
$cargo2junitPath = Get-Command cargo2junit -ErrorAction SilentlyContinue
if (!$cargo2junitPath) {
  Write-Host "cargo2junit not found. Installing..."
  Invoke-LoggedCommand "cargo install cargo2junit" -GroupOutput
}

# Get all JSON files in the test results directory
$jsonFiles = @(Get-ChildItem -Path $TestResultsDirectory -Filter "*.json" -File)

if ($jsonFiles.Count -eq 0) {
  LogWarning "No JSON files found in $TestResultsDirectory"
  Write-Host "No test results to convert."
  exit 0
}

Write-Host "`nConverting $($jsonFiles.Count) JSON file(s) to JUnit XML..."

$convertedCount = 0
$failedCount = 0

foreach ($jsonFile in $jsonFiles) {
  $baseName = [System.IO.Path]::GetFileNameWithoutExtension($jsonFile.Name)
  $junitFile = ([System.IO.Path]::Combine($OutputDirectory, "$baseName.xml"))

  Write-Host "  Converting: $($jsonFile.Name) -> $([System.IO.Path]::GetFileName($junitFile))"

  try {
    # Convert JSON to JUnit XML using cargo2junit
    Get-Content $jsonFile.FullName | cargo2junit > $junitFile

    if ($LASTEXITCODE -ne 0) {
      LogWarning "    cargo2junit returned exit code $LASTEXITCODE for $($jsonFile.Name)"
      $failedCount++
    }
    else {
      $convertedCount++
    }
  }
  catch {
    LogWarning "    Failed to convert $($jsonFile.Name): $_"
    $failedCount++
  }
}

Write-Host "`nConversion complete:"
Write-Host "  Successfully converted: $convertedCount"
if ($failedCount -gt 0) {
  Write-Host "  Failed to convert:      $failedCount" -ForegroundColor Yellow
}

Write-Host "`nJUnit XML files are available in: $OutputDirectory"

# Exit with error if any conversions failed
if ($failedCount -gt 0) {
  exit 1
}

exit 0
