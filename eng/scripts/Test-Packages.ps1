#!/usr/bin/env pwsh

#Requires -Version 7.0
param(
  [string]$PackageInfoDirectory,
  [switch]$CI
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0
. "$PSScriptRoot/../common/scripts/common.ps1"

# Helper function to parse cargo test JSON output and extract test results
function Parse-TestResults {
  param(
    [string]$JsonFile
  )
  
  $testResults = @{
    Passed = 0
    Failed = 0
    Ignored = 0
    FailedTests = @()
  }
  
  if (!(Test-Path $JsonFile)) {
    return $testResults
  }
  
  # Parse each JSON line (cargo outputs newline-delimited JSON)
  Get-Content $JsonFile | ForEach-Object {
    try {
      $line = $_ | ConvertFrom-Json -ErrorAction SilentlyContinue
      if ($line.reason -eq "test") {
        switch ($line.event) {
          "ok" { $testResults.Passed++ }
          "failed" { 
            $testResults.Failed++
            $testResults.FailedTests += $line.name
          }
          "ignored" { $testResults.Ignored++ }
        }
      }
    }
    catch {
      # Ignore lines that aren't valid JSON
    }
  }
  
  return $testResults
}

# Helper function to output human-readable test summary
function Write-TestSummary {
  param(
    [hashtable]$TestResults,
    [string]$PackageName
  )
  
  Write-Host "`n========================================" -ForegroundColor Cyan
  Write-Host "Test Summary for: $PackageName" -ForegroundColor Cyan
  Write-Host "========================================" -ForegroundColor Cyan
  Write-Host "Passed:  $($TestResults.Passed)" -ForegroundColor Green
  Write-Host "Failed:  $($TestResults.Failed)" -ForegroundColor $(if ($TestResults.Failed -gt 0) { "Red" } else { "Green" })
  Write-Host "Ignored: $($TestResults.Ignored)" -ForegroundColor Yellow
  
  if ($TestResults.Failed -gt 0) {
    Write-Host "`nFailed Tests:" -ForegroundColor Red
    foreach ($failedTest in $TestResults.FailedTests) {
      Write-Host "  - $failedTest" -ForegroundColor Red
    }
    Write-Host "`nℹ️ Additional details are available in the test tab for this build." -ForegroundColor Yellow
  }
  Write-Host "========================================`n" -ForegroundColor Cyan
}

# Helper function to run cargo test and capture output
function Invoke-CargoTest {
  param(
    [string]$Command,
    [string]$OutputFile,
    [bool]$InCI
  )
  
  if ($InCI) {
    # In CI mode, capture JSON output
    $cargoCommand = "$Command --message-format=json"
    Write-Host "Running: $cargoCommand"
    Write-Host "Output will be captured to: $OutputFile"
    
    # Run the command and capture both stdout and stderr
    $output = & { Invoke-Expression $cargoCommand 2>&1 }
    $exitCode = $LASTEXITCODE
    
    # Write output to file
    $output | Out-File -FilePath $OutputFile -Encoding utf8
    
    # Also display output to console for real-time feedback
    $output | ForEach-Object { Write-Host $_ }
    
    return $exitCode
  }
  else {
    # In non-CI mode, use the original Invoke-LoggedCommand
    Invoke-LoggedCommand $Command -GroupOutput
    return $LASTEXITCODE
  }
}

Write-Host @"
Testing packages with
    PackageInfoDirectory: '$PackageInfoDirectory'
    CI Mode: $CI
    RUSTFLAGS: '$env:RUSTFLAGS'
    RUSTDOCFLAGS: '$env:RUSTDOCFLAGS'
    RUST_LOG: '$env:RUST_LOG'
    AZURE_TEST_MODE: '$env:AZURE_TEST_MODE'
    SYSTEM_ACCESSTOKEN: $($env:SYSTEM_ACCESSTOKEN ? 'present' : 'not present')
    ARM_OIDC_TOKEN: $($env:ARM_OIDC_TOKEN ? 'present' : 'not present')
"@

# Create directory for test results if in CI mode
if ($CI) {
  $testResultsDir = Join-Path $RepoRoot "test-results"
  if (!(Test-Path $testResultsDir)) {
    New-Item -ItemType Directory -Path $testResultsDir | Out-Null
  }
  Write-Host "Test results will be saved to: $testResultsDir"
}

if ($PackageInfoDirectory) {
  if (!(Test-Path $PackageInfoDirectory)) {
    Write-Error "Package info path '$PackageInfoDirectory' does not exist."
    exit 1
  }

  $packagesToTest = Get-ChildItem $PackageInfoDirectory -Filter "*.json" -Recurse
  | Get-Content -Raw
  | ConvertFrom-Json
}
else {
  $packagesToTest = Get-AllPackageInfoFromRepo
}

Write-Host "Testing packages:"
foreach ($package in $packagesToTest) {
  Write-Host "  '$($package.Name)' in '$($package.DirectoryPath)'"
}

$allTestResults = @()
$hasFailures = $false

foreach ($package in $packagesToTest) {
  Push-Location ([System.IO.Path]::Combine($RepoRoot, $package.DirectoryPath))
  try {
    $packageDirectory = ([System.IO.Path]::Combine($RepoRoot, $package.DirectoryPath))

    $setupScript = Join-Path $packageDirectory "Test-Setup.ps1"
    if (Test-Path $setupScript) {
      Write-Host "`n`nRunning test setup script for package: '$($package.Name)'`n"
      Invoke-LoggedCommand $setupScript -GroupOutput
      if (!$? -ne 0) {
        Write-Error "Test setup script failed for package: '$($package.Name)'"
        exit 1
      }
    }

    Write-Host "`n`nTesting package: '$($package.Name)'`n"

    # Build step - always use Invoke-LoggedCommand
    Invoke-LoggedCommand "cargo build --keep-going" -GroupOutput
    Write-Host "`n`n"

    # Generate unique filenames for test outputs if in CI mode
    $timestamp = Get-Date -Format "yyyyMMdd-HHmmss-fff"
    $sanitizedPackageName = $package.Name -replace '[^a-zA-Z0-9_-]', '_'
    
    if ($CI) {
      $docTestOutput = Join-Path $testResultsDir "$sanitizedPackageName-doctest-$timestamp.json"
      $allTargetsTestOutput = Join-Path $testResultsDir "$sanitizedPackageName-alltargets-$timestamp.json"
    }

    # Run doc tests
    if ($CI) {
      $exitCode = Invoke-CargoTest -Command "cargo test --doc --no-fail-fast" -OutputFile $docTestOutput -InCI $true
      $docTestResults = Parse-TestResults -JsonFile $docTestOutput
      Write-TestSummary -TestResults $docTestResults -PackageName "$($package.Name) (doc tests)"
      if ($exitCode -ne 0) { $hasFailures = $true }
      $allTestResults += @{ Package = $package.Name; Type = "doc"; Results = $docTestResults }
    }
    else {
      Invoke-LoggedCommand "cargo test --doc --no-fail-fast" -GroupOutput
    }
    Write-Host "`n`n"

    # Run all-targets tests
    if ($CI) {
      $exitCode = Invoke-CargoTest -Command "cargo test --all-targets --no-fail-fast" -OutputFile $allTargetsTestOutput -InCI $true
      $allTargetsTestResults = Parse-TestResults -JsonFile $allTargetsTestOutput
      Write-TestSummary -TestResults $allTargetsTestResults -PackageName "$($package.Name) (all targets)"
      if ($exitCode -ne 0) { $hasFailures = $true }
      $allTestResults += @{ Package = $package.Name; Type = "all-targets"; Results = $allTargetsTestResults }
    }
    else {
      Invoke-LoggedCommand "cargo test --all-targets --no-fail-fast" -GroupOutput
    }
    Write-Host "`n`n"

    $cleanupScript = Join-Path $packageDirectory "Test-Cleanup.ps1"
    if (Test-Path $cleanupScript) {
      Write-Host "`n`nRunning test cleanup script for package: '$($package.Name)'`n"
      Invoke-LoggedCommand $cleanupScript -GroupOutput
      # We ignore the exit code of the cleanup script.
    }
  }
  finally {
    Pop-Location
  }
}

# Print overall summary if in CI mode
if ($CI -and $allTestResults.Count -gt 0) {
  Write-Host "`n`n" 
  Write-Host "========================================" -ForegroundColor Cyan
  Write-Host "OVERALL TEST SUMMARY" -ForegroundColor Cyan
  Write-Host "========================================" -ForegroundColor Cyan
  
  $totalPassed = 0
  $totalFailed = 0
  $totalIgnored = 0
  
  foreach ($result in $allTestResults) {
    $totalPassed += $result.Results.Passed
    $totalFailed += $result.Results.Failed
    $totalIgnored += $result.Results.Ignored
  }
  
  Write-Host "Total Passed:  $totalPassed" -ForegroundColor Green
  Write-Host "Total Failed:  $totalFailed" -ForegroundColor $(if ($totalFailed -gt 0) { "Red" } else { "Green" })
  Write-Host "Total Ignored: $totalIgnored" -ForegroundColor Yellow
  
  if ($totalFailed -gt 0) {
    Write-Host "`nℹ️ Additional details are available in the test tab for this build." -ForegroundColor Yellow
  }
  
  Write-Host "========================================`n" -ForegroundColor Cyan
  
  # Exit with error if there were failures
  if ($hasFailures) {
    exit 1
  }
}
