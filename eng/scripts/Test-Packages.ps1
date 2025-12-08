#!/usr/bin/env pwsh

#Requires -Version 7.0
param(
  [string]$PackageInfoDirectory
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
  
  LogGroupStart "Test Summary: $PackageName"
  Write-Host "Passed:  $passed" -ForegroundColor Green
  Write-Host "Failed:  $failed" -ForegroundColor $(if ($failed -gt 0) { "Red" } else { "Green" })
  Write-Host "Ignored: $ignored" -ForegroundColor Yellow
  
  if ($failed -gt 0) {
    Write-Host "`nFailed tests:" -ForegroundColor Red
    foreach ($test in $failedTests) {
      Write-Host "  - $test" -ForegroundColor Red
    }
    Write-Host "`nAdditional details are available in the test tab for the build." -ForegroundColor Yellow
  }
  LogGroupEnd
  
  return @{
    Passed = $passed
    Failed = $failed
    Ignored = $ignored
  }
}

# Helper function to run cargo test with JSON output
function Invoke-CargoTestWithJsonOutput {
  param(
    [string]$TestType,
    [string]$PackageName,
    [string]$OutputFile
  )
  
  Write-Host "Running $TestType tests with JSON output to: $OutputFile"
  
  # Use cargo +nightly test with --format json and -Z unstable-options
  $testCommand = if ($TestType -eq "doc") {
    "cargo +nightly test --doc --no-fail-fast -- --format json -Z unstable-options"
  } else {
    "cargo +nightly test --all-targets --no-fail-fast -- --format json -Z unstable-options"
  }
  
  # Redirect output to file
  Invoke-LoggedCommand "$testCommand > `"$OutputFile`"" -GroupOutput -DoNotExitOnFailedExitCode
  $exitCode = $LASTEXITCODE
  
  # Parse and display summary
  $results = Write-TestSummary -JsonFile $OutputFile -PackageName "$PackageName ($TestType tests)"
  
  # Exit immediately if tests failed
  if ($exitCode -ne 0) {
    Write-Error "Tests failed for package '$PackageName' ($TestType tests)"
    exit $exitCode
  }
  
  return $results
}

Write-Host @"
Testing packages with
    PackageInfoDirectory: '$PackageInfoDirectory'
    RUSTFLAGS: '$env:RUSTFLAGS'
    RUSTDOCFLAGS: '$env:RUSTDOCFLAGS'
    RUST_LOG: '$env:RUST_LOG'
    AZURE_TEST_MODE: '$env:AZURE_TEST_MODE'
    SYSTEM_ACCESSTOKEN: $($env:SYSTEM_ACCESSTOKEN ? 'present' : 'not present')
    ARM_OIDC_TOKEN: $($env:ARM_OIDC_TOKEN ? 'present' : 'not present')
"@

# Create directory for test results
$testResultsDir = ([System.IO.Path]::Combine($RepoRoot, 'test-results'))
if (!(Test-Path $testResultsDir)) {
  New-Item -ItemType Directory -Path $testResultsDir | Out-Null
}
Write-Host "Test results will be saved to: $testResultsDir"

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

foreach ($package in $packagesToTest) {
  Push-Location ([System.IO.Path]::Combine($RepoRoot, $package.DirectoryPath))
  try {
    $packageDirectory = ([System.IO.Path]::Combine($RepoRoot, $package.DirectoryPath))

    $setupScript = ([System.IO.Path]::Combine($packageDirectory, 'Test-Setup.ps1'))
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

    # Generate unique filenames for test outputs
    $timestamp = Get-Date -Format "yyyyMMdd-HHmmss-fff"
    $sanitizedPackageName = $package.Name -replace '[^a-zA-Z0-9_-]', '_'
    
    # Run doc tests
    $docTestOutput = ([System.IO.Path]::Combine($testResultsDir, "$sanitizedPackageName-doctest-$timestamp.json"))
    Invoke-CargoTestWithJsonOutput -TestType "doc" -PackageName $package.Name -OutputFile $docTestOutput
    
    # Run all-targets tests
    $allTargetsOutput = ([System.IO.Path]::Combine($testResultsDir, "$sanitizedPackageName-alltargets-$timestamp.json"))
    Invoke-CargoTestWithJsonOutput -TestType "all-targets" -PackageName $package.Name -OutputFile $allTargetsOutput

    $cleanupScript = ([System.IO.Path]::Combine($packageDirectory, 'Test-Cleanup.ps1'))
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
