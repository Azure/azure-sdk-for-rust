#!/usr/bin/env pwsh

#Requires -Version 7.0
param(
  [string]$PackageInfoDirectory,
  [switch]$CI
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0
. "$PSScriptRoot/../common/scripts/common.ps1"

# Helper function to parse test results from JSON and output human-readable summary
function Write-TestSummary {
  param(
    [string]$JsonFile,
    [string]$PackageName
  )
  
  if (!(Test-Path $JsonFile)) {
    Write-Warning "Test results file not found: $JsonFile"
    return
  }
  
  $passed = 0
  $failed = 0
  $ignored = 0
  $failedTests = @()
  
  # Parse JSON output (newline-delimited JSON)
  Get-Content $JsonFile | ForEach-Object {
    try {
      $event = $_ | ConvertFrom-Json -ErrorAction SilentlyContinue
      if ($event.type -eq "test" -and $event.event) {
        switch ($event.event) {
          "ok" { $passed++ }
          "failed" { 
            $failed++
            $failedTests += $event.name
          }
          "ignored" { $ignored++ }
        }
      }
    }
    catch {
      # Ignore lines that aren't valid JSON
    }
  }
  
  Write-Host "`n========================================" -ForegroundColor Cyan
  Write-Host "Test Summary: $PackageName" -ForegroundColor Cyan
  Write-Host "========================================" -ForegroundColor Cyan
  Write-Host "Passed:  $passed" -ForegroundColor Green
  Write-Host "Failed:  $failed" -ForegroundColor $(if ($failed -gt 0) { "Red" } else { "Green" })
  Write-Host "Ignored: $ignored" -ForegroundColor Yellow
  
  if ($failed -gt 0) {
    Write-Host "`nFailed tests:" -ForegroundColor Red
    foreach ($test in $failedTests) {
      Write-Host "  - $test" -ForegroundColor Red
    }
    Write-Host "`nℹ️  Additional details are available in the test tab for the build." -ForegroundColor Yellow
  }
  Write-Host "========================================`n" -ForegroundColor Cyan
  
  return @{
    Passed = $passed
    Failed = $failed
    Ignored = $ignored
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
$testResultsDir = $null
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

    Invoke-LoggedCommand "cargo build --keep-going" -GroupOutput
    Write-Host "`n`n"

    # Generate unique filenames for test outputs if in CI mode
    $timestamp = Get-Date -Format "yyyyMMdd-HHmmss-fff"
    $sanitizedPackageName = $package.Name -replace '[^a-zA-Z0-9_-]', '_'
    
    # Run doc tests
    if ($CI) {
      $docTestOutput = Join-Path $testResultsDir "$sanitizedPackageName-doctest-$timestamp.json"
      Write-Host "Running doc tests with JSON output to: $docTestOutput"
      
      # Use cargo +nightly test with --format json and -Z unstable-options
      $output = & cargo +nightly test --doc --no-fail-fast -- --format json -Z unstable-options 2>&1
      $exitCode = $LASTEXITCODE
      
      # Write JSON output to file
      $output | Out-File -FilePath $docTestOutput -Encoding utf8
      
      # Also display the output
      $output | ForEach-Object { Write-Host $_ }
      
      # Parse and display summary
      $docResults = Write-TestSummary -JsonFile $docTestOutput -PackageName "$($package.Name) (doc tests)"
      if ($exitCode -ne 0) { $hasFailures = $true }
      $allTestResults += @{ Package = $package.Name; Type = "doc"; Results = $docResults }
    }
    else {
      Invoke-LoggedCommand "cargo test --doc --no-fail-fast" -GroupOutput
    }
    Write-Host "`n`n"

    # Run all-targets tests
    if ($CI) {
      $allTargetsOutput = Join-Path $testResultsDir "$sanitizedPackageName-alltargets-$timestamp.json"
      Write-Host "Running all-targets tests with JSON output to: $allTargetsOutput"
      
      # Use cargo +nightly test with --format json and -Z unstable-options
      $output = & cargo +nightly test --all-targets --no-fail-fast -- --format json -Z unstable-options 2>&1
      $exitCode = $LASTEXITCODE
      
      # Write JSON output to file
      $output | Out-File -FilePath $allTargetsOutput -Encoding utf8
      
      # Also display the output
      $output | ForEach-Object { Write-Host $_ }
      
      # Parse and display summary
      $allTargetsResults = Write-TestSummary -JsonFile $allTargetsOutput -PackageName "$($package.Name) (all targets)"
      if ($exitCode -ne 0) { $hasFailures = $true }
      $allTestResults += @{ Package = $package.Name; Type = "all-targets"; Results = $allTargetsResults }
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
    if ($result.Results) {
      $totalPassed += $result.Results.Passed
      $totalFailed += $result.Results.Failed
      $totalIgnored += $result.Results.Ignored
    }
  }
  
  Write-Host "Total Passed:  $totalPassed" -ForegroundColor Green
  Write-Host "Total Failed:  $totalFailed" -ForegroundColor $(if ($totalFailed -gt 0) { "Red" } else { "Green" })
  Write-Host "Total Ignored: $totalIgnored" -ForegroundColor Yellow
  
  if ($totalFailed -gt 0) {
    Write-Host "`nℹ️  Additional details are available in the test tab for the build." -ForegroundColor Yellow
  }
  
  Write-Host "========================================`n" -ForegroundColor Cyan
  
  # Exit with error if there were failures
  if ($hasFailures) {
    exit 1
  }
}
