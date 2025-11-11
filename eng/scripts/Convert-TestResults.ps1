#!/usr/bin/env pwsh

#Requires -Version 7.0
<#
.SYNOPSIS
Converts cargo test plain text output to JUnit XML format.

.DESCRIPTION
This script converts the plain text output files from cargo test (captured by Test-Packages.ps1 in CI mode)
to JUnit XML format suitable for publishing to Azure DevOps test results.

.PARAMETER TestResultsDirectory
The directory containing text test result files. Defaults to test-results in the repo root.

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

# Helper function to escape XML special characters
function Escape-Xml {
  param([string]$Text)
  if (!$Text) { return "" }
  return $Text.Replace("&", "&amp;").Replace("<", "&lt;").Replace(">", "&gt;").Replace('"', "&quot;").Replace("'", "&apos;")
}

# Helper function to parse cargo test output and generate JUnit XML
function Convert-CargoTestToJUnit {
  param(
    [string]$InputFile,
    [string]$OutputFile
  )
  
  $content = Get-Content $InputFile
  $testSuiteName = "cargo-test"
  $testCases = @()
  $totalTests = 0
  $failures = 0
  $errors = 0
  $skipped = 0
  $time = 0.0
  
  foreach ($line in $content) {
    # Extract test suite name from "Running" line
    if ($line -match 'Running (unittests|tests|integration test).*\(([^)]+)\)') {
      $testSuiteName = [System.IO.Path]::GetFileNameWithoutExtension($Matches[2])
    }
    
    # Parse individual test results
    if ($line -match '^test (.+) \.\.\. (ok|FAILED|ignored)(?:\s+\(([0-9.]+)s\))?') {
      $testName = $Matches[1].Trim()
      $status = $Matches[2]
      $duration = if ($Matches[3]) { [double]$Matches[3] } else { 0.0 }
      
      $testCase = @{
        Name = $testName
        ClassName = $testSuiteName
        Time = $duration
        Status = $status
        Message = ""
      }
      
      $totalTests++
      $time += $duration
      
      switch ($status) {
        "ok" { }
        "FAILED" { 
          $failures++
          $testCase.Message = "Test failed"
        }
        "ignored" { 
          $skipped++
        }
      }
      
      $testCases += $testCase
    }
    
    # Parse summary line to extract total time if available
    if ($line -match 'finished in ([0-9.]+)s') {
      $time = [double]$Matches[1]
    }
  }
  
  # Generate JUnit XML
  $xml = New-Object System.Text.StringBuilder
  [void]$xml.AppendLine('<?xml version="1.0" encoding="UTF-8"?>')
  [void]$xml.AppendLine("<testsuites>")
  [void]$xml.AppendLine("  <testsuite name=`"$(Escape-Xml $testSuiteName)`" tests=`"$totalTests`" failures=`"$failures`" errors=`"$errors`" skipped=`"$skipped`" time=`"$time`">")
  
  foreach ($testCase in $testCases) {
    $escapedName = Escape-Xml $testCase.Name
    $escapedClass = Escape-Xml $testCase.ClassName
    
    if ($testCase.Status -eq "ignored") {
      [void]$xml.AppendLine("    <testcase name=`"$escapedName`" classname=`"$escapedClass`" time=`"$($testCase.Time)`">")
      [void]$xml.AppendLine("      <skipped />")
      [void]$xml.AppendLine("    </testcase>")
    }
    elseif ($testCase.Status -eq "FAILED") {
      [void]$xml.AppendLine("    <testcase name=`"$escapedName`" classname=`"$escapedClass`" time=`"$($testCase.Time)`">")
      [void]$xml.AppendLine("      <failure message=`"$(Escape-Xml $testCase.Message)`" />")
      [void]$xml.AppendLine("    </testcase>")
    }
    else {
      [void]$xml.AppendLine("    <testcase name=`"$escapedName`" classname=`"$escapedClass`" time=`"$($testCase.Time)`" />")
    }
  }
  
  [void]$xml.AppendLine("  </testsuite>")
  [void]$xml.AppendLine("</testsuites>")
  
  # Write to file
  $xml.ToString() | Out-File -FilePath $OutputFile -Encoding utf8
}

# Get repo root
$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot .. ..)

# Set default directories
if (!$TestResultsDirectory) {
  $TestResultsDirectory = Join-Path $RepoRoot "test-results"
}

if (!$OutputDirectory) {
  $OutputDirectory = Join-Path $RepoRoot "test-results" "junit"
}

Write-Host "Converting test results from plain text to JUnit XML"
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

# Get all text files in the test results directory
$textFiles = @(Get-ChildItem -Path $TestResultsDirectory -Filter "*.txt" -File)

if ($textFiles.Count -eq 0) {
  Write-Warning "No text files found in $TestResultsDirectory"
  Write-Host "No test results to convert."
  exit 0
}

Write-Host "`nConverting $($textFiles.Count) text file(s) to JUnit XML..."

$convertedCount = 0
$failedCount = 0

foreach ($textFile in $textFiles) {
  $baseName = [System.IO.Path]::GetFileNameWithoutExtension($textFile.Name)
  $junitFile = Join-Path $OutputDirectory "$baseName.xml"
  
  Write-Host "  Converting: $($textFile.Name) -> $([System.IO.Path]::GetFileName($junitFile))"
  
  try {
    Convert-CargoTestToJUnit -InputFile $textFile.FullName -OutputFile $junitFile
    $convertedCount++
  }
  catch {
    Write-Warning "    Failed to convert $($textFile.Name): $_"
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
