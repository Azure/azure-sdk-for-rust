# Test Results Reporting

This directory contains scripts for capturing cargo test results and converting them to JUnit XML format for Azure DevOps.

## Overview

The test results reporting uses:
1. **Nightly Rust's native JSON test output** (`cargo +nightly test -- --format json -Z unstable-options`)
2. **cargo2junit** tool to convert JSON to JUnit XML

## Scripts

### Test-Packages.ps1

Enhanced to support CI mode with `-CI` switch parameter.

**CI Mode (`-CI` flag):**
- Uses `cargo +nightly test -- --format json -Z unstable-options`
- Captures JSON output to uniquely named files in `test-results/` directory
- Parses JSON and displays human-readable summaries
- Shows pass/fail/ignored counts and lists failed tests

**Standard Mode (no `-CI` flag):**
- Original behavior using `Invoke-LoggedCommand`
- Human-readable output directly to console

**Usage:**
```powershell
# CI mode
./eng/scripts/Test-Packages.ps1 -PackageInfoDirectory ./PackageInfo -CI

# Standard mode
./eng/scripts/Test-Packages.ps1 -PackageInfoDirectory ./PackageInfo
```

### Convert-TestResultsToJUnit.ps1

Converts JSON test results to JUnit XML format using cargo2junit.

**Features:**
- Automatically installs cargo2junit if not present
- Processes all JSON files in test-results directory
- Outputs JUnit XML to test-results/junit directory
- Compatible with Azure DevOps PublishTestResults task

**Usage:**
```powershell
./eng/scripts/Convert-TestResultsToJUnit.ps1

# Or with custom directories
./eng/scripts/Convert-TestResultsToJUnit.ps1 -TestResultsDirectory ./test-results -OutputDirectory ./junit
```

## Pipeline Integration

Example Azure DevOps pipeline YAML:

```yaml
# Run tests with JSON output capture
- task: Powershell@2
  displayName: "Test Packages"
  inputs:
    pwsh: true
    filePath: $(Build.SourcesDirectory)/eng/scripts/Test-Packages.ps1
    arguments: >
      -PackageInfoDirectory '$(Build.ArtifactStagingDirectory)/PackageInfo'
      -CI

# Convert JSON to JUnit XML
- task: Powershell@2
  displayName: "Convert Test Results to JUnit XML"
  condition: succeededOrFailed()
  inputs:
    pwsh: true
    filePath: $(Build.SourcesDirectory)/eng/scripts/Convert-TestResultsToJUnit.ps1

# Publish test results to Azure DevOps
- task: PublishTestResults@2
  displayName: "Publish Test Results"
  condition: succeededOrFailed()
  inputs:
    testResultsFormat: 'JUnit'
    testResultsFiles: '**/test-results/junit/*.xml'
    testRunTitle: 'Rust Tests'
    mergeTestResults: true
    failTaskOnFailedTests: false
```

## Requirements

- **PowerShell 7.0+** (already required by existing scripts)
- **Nightly Rust toolchain** (installed automatically by rustup when using `cargo +nightly`)
- **cargo2junit** (installed automatically by Convert-TestResultsToJUnit.ps1 if needed)

## Test Results Format

### Directory Structure
```
test-results/
├── {package}-doctest-{timestamp}.json       # JSON test output from doc tests
├── {package}-alltargets-{timestamp}.json    # JSON test output from all-targets tests
└── junit/
    ├── {package}-doctest-{timestamp}.xml    # JUnit XML for doc tests
    └── {package}-alltargets-{timestamp}.xml # JUnit XML for all-targets tests
```

### JSON Format

Nightly Rust outputs newline-delimited JSON with events like:
```json
{ "type": "test", "event": "started", "name": "test_name" }
{ "type": "test", "name": "test_name", "event": "ok" }
{ "type": "suite", "event": "ok", "passed": 30, "failed": 0, "ignored": 0 }
```

### JUnit XML Format

cargo2junit converts to standard JUnit XML:
```xml
<testsuites>
  <testsuite name="..." tests="30" errors="0" failures="0">
    <testcase name="test_name" classname="module::tests" />
  </testsuite>
</testsuites>
```

## Troubleshooting

### Nightly Rust not installed
If you see errors about nightly not being available:
```bash
rustup toolchain install nightly
```

### cargo2junit not found
The Convert-TestResultsToJUnit.ps1 script automatically installs it, but you can manually install:
```bash
cargo install cargo2junit
```

### No test results generated
Make sure the `-CI` flag is passed to Test-Packages.ps1 when running in CI mode.

## Benefits of This Approach

1. **Native Format**: Uses Rust's native JSON test output format (no custom parsing)
2. **Reliable**: cargo2junit is purpose-built for this conversion
3. **Simple**: Minimal code, leverages existing tools
4. **Maintainable**: Less custom code to maintain
5. **Feature-Rich**: Gets full test metadata from Rust's test harness
