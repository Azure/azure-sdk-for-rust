#!/usr/bin/env pwsh

#Requires -Version 7.0
<#
.SYNOPSIS
Converts cargo test JSON output to JUnit XML format using cargo2junit.

.DESCRIPTION
This script converts the JSON output files from cargo test (captured by Test-Packages.ps1 in CI mode)
to JUnit XML format suitable for publishing to Azure DevOps test results.

.PARAMETER TestResultsDirectory
The directory containing JSON test result files. Defaults to test-results in the repo root.

.PARAMETER OutputDirectory
The directory where JUnit XML files should be written. Defaults to test-results/junit in the repo root.

.EXAMPLE
./eng/scripts/Convert-TestResults.ps1

.EXAMPLE
./eng/scripts/Convert-TestResults.ps1 -TestResultsDirectory ./test-results -OutputDirectory ./junit-results
#>

param(
  [string]$TestResultsDirectory,
  [string]$OutputDirectory
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

# Get repo root
$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot .. ..)

# Set default directories
if (!$TestResultsDirectory) {
  $TestResultsDirectory = Join-Path $RepoRoot "test-results"
}

if (!$OutputDirectory) {
  $OutputDirectory = Join-Path $RepoRoot "test-results" "junit"
}

Write-Host "Converting test results from JSON to JUnit XML"
Write-Host "  Input directory:  $TestResultsDirectory"
Write-Host "  Output directory: $OutputDirectory"

# Check if test results directory exists
if (!(Test-Path $TestResultsDirectory)) {
  Write-Warning "Test results directory not found: $TestResultsDirectory"
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
  cargo install cargo2junit
  if ($LASTEXITCODE -ne 0) {
    Write-Error "Failed to install cargo2junit"
    exit 1
  }
  Write-Host "cargo2junit installed successfully"
}

# Get all JSON files in the test results directory
$jsonFiles = Get-ChildItem -Path $TestResultsDirectory -Filter "*.json" -File

if ($jsonFiles.Count -eq 0) {
  Write-Warning "No JSON files found in $TestResultsDirectory"
  Write-Host "No test results to convert."
  exit 0
}

Write-Host "`nConverting $($jsonFiles.Count) JSON file(s) to JUnit XML..."

$convertedCount = 0
$failedCount = 0

foreach ($jsonFile in $jsonFiles) {
  $baseName = [System.IO.Path]::GetFileNameWithoutExtension($jsonFile.Name)
  $junitFile = Join-Path $OutputDirectory "$baseName.xml"
  
  Write-Host "  Converting: $($jsonFile.Name) -> $([System.IO.Path]::GetFileName($junitFile))"
  
  try {
    # Convert JSON to JUnit XML using cargo2junit
    Get-Content $jsonFile.FullName | cargo2junit > $junitFile
    
    if ($LASTEXITCODE -ne 0) {
      Write-Warning "    cargo2junit returned exit code $LASTEXITCODE for $($jsonFile.Name)"
      $failedCount++
    }
    else {
      $convertedCount++
    }
  }
  catch {
    Write-Warning "    Failed to convert $($jsonFile.Name): $_"
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
