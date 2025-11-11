# Test Results Reporting for Azure DevOps

This document describes the implementation of test results reporting for Azure DevOps pipelines in the azure-sdk-for-rust repository.

## Overview

The testing infrastructure now supports capturing test results from `cargo test` and converting them to JUnit XML format for display in Azure DevOps test results tabs.

## Implementation

### Scripts

1. **Test-Packages.ps1** - Enhanced with `-CI` switch parameter
   - When `-CI` is specified, captures cargo test output to text files in `test-results/` directory
   - Parses test output and displays human-readable summaries with pass/fail/ignored counts
   - Maintains backward compatibility: runs in standard mode when `-CI` is not specified
   - Generates uniquely named output files per test run (format: `{package}-{testtype}-{timestamp}.txt`)

2. **Convert-TestResults.ps1** - Converts plain text test results to JUnit XML
   - Reads text files from `test-results/` directory
   - Parses cargo test output format (test names, status, summaries)
   - Generates JUnit XML files in `test-results/junit/` directory
   - No external dependencies - pure PowerShell implementation

### Usage

#### Running Tests with Result Capture (CI Mode)

```powershell
./eng/scripts/Test-Packages.ps1 -CI -PackageInfoDirectory ./PackageInfo
```

#### Converting Results to JUnit XML

```powershell
./eng/scripts/Convert-TestResults.ps1
```

Or with custom directories:

```powershell
./eng/scripts/Convert-TestResults.ps1 -TestResultsDirectory ./test-results -OutputDirectory ./junit-output
```

#### Publishing to Azure DevOps

In your pipeline YAML:

```yaml
- task: Powershell@2
  displayName: "Test Packages"
  inputs:
    pwsh: true
    filePath: $(Build.SourcesDirectory)/eng/scripts/Test-Packages.ps1
    arguments: >
      -PackageInfoDirectory '$(Build.ArtifactStagingDirectory)/PackageInfo'
      -CI

- task: Powershell@2
  displayName: "Convert Test Results to JUnit XML"
  condition: succeededOrFailed()
  inputs:
    pwsh: true
    filePath: $(Build.SourcesDirectory)/eng/scripts/Convert-TestResults.ps1

- task: PublishTestResults@2
  displayName: "Publish Test Results"
  condition: succeededOrFailed()
  inputs:
    testResultsFormat: 'JUnit'
    testResultsFiles: '**/test-results/junit/*.xml'
    testRunTitle: 'Rust Tests - $(Agent.JobName)'
    mergeTestResults: true
    failTaskOnFailedTests: false
```

## Alternative Approaches Considered

### 1. cargo2junit (Not Chosen)

**Description**: A Rust tool that converts cargo test output to JUnit XML.

**Pros**:
- Purpose-built for cargo test output
- Mentioned in the original issue

**Cons**:
- **Does not work with stable Rust**: cargo2junit expects JSON test output which is only available in nightly Rust with `--format json` flag
- On stable Rust, `cargo test --message-format=json` only provides build compilation messages, not test execution results
- Test results are emitted as plain text even when --message-format=json is used
- Requires `cargo install cargo2junit` step in pipeline
- External dependency to maintain

**Why not chosen**: Fundamental incompatibility with stable Rust's test output format.

### 2. Nightly Rust with --format json/junit (Not Chosen)

**Description**: Use nightly Rust compiler to access test harness's native JSON and JUnit output formats.

```bash
cargo +nightly test -- --format json
cargo +nightly test -- -Z unstable-options --format junit
```

**Pros**:
- Native support from Rust test harness
- Clean JSON or JUnit output

**Cons**:
- Requires nightly Rust toolchain
- May introduce instability from nightly features
- Not suitable for production CI/CD using stable Rust
- Requires additional toolchain installation in pipeline

**Why not chosen**: Repository uses stable Rust; nightly is not appropriate for production testing.

### 3. cargo-nextest (Alternative Option)

**Description**: A modern test runner for Rust with built-in JUnit XML support on stable Rust.

```bash
cargo install cargo-nextest
cargo nextest run --profile ci
```

**Pros**:
- Works on stable Rust
- Native JUnit XML output via `--message-format`
- Faster test execution (parallel by default)
- Better test output formatting
- Actively maintained

**Cons**:
- Requires `cargo install cargo-nextest` in pipeline
- Changes test execution behavior (parallel by default)
- External dependency
- Need to ensure compatibility with existing tests

**Why not chosen**: Adds external dependency and changes test execution model. Could be reconsidered in future.

### 4. Custom PowerShell Parser (Chosen)

**Description**: Parse cargo test plain text output and generate JUnit XML using PowerShell.

**Pros**:
- Works with stable Rust out of the box
- No external dependencies beyond PowerShell (already required)
- Full control over parsing and XML generation
- Easy to maintain and customize
- Handles all cargo test output scenarios

**Cons**:
- Custom code to maintain
- Parsing logic must handle cargo test format changes
- Less feature-rich than specialized tools

**Why chosen**: 
- Most pragmatic solution for stable Rust
- Leverages existing PowerShell infrastructure
- No additional dependencies
- Maintainable and customizable

## cargo test Output Format

The implementation parses standard cargo test plain text output:

```
Running unittests src/lib.rs (target/debug/deps/crate_name-hash)

running 30 tests
test module::test_name_1 ... ok
test module::test_name_2 ... FAILED
test module::test_name_3 ... ignored

test result: ok. 28 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

### Parsed Elements

- **Test count**: Extracted from "running N tests"
- **Individual results**: Pattern `test {name} ... {status}` where status is `ok`, `FAILED`, or `ignored`
- **Summary**: Pattern `test result: {result}. {passed} passed; {failed} failed; {ignored} ignored`
- **Duration**: Extracted from "finished in {time}s"

## Test Results Directory Structure

```
test-results/
├── {package}-doctest-{timestamp}.txt       # Plain text output from doc tests
├── {package}-alltargets-{timestamp}.txt    # Plain text output from all-targets tests
└── junit/
    ├── {package}-doctest-{timestamp}.xml   # JUnit XML for doc tests
    └── {package}-alltargets-{timestamp}.xml # JUnit XML for all-targets tests
```

The `test-results/` directory is gitignored and should not be committed.

## Features

### Human-Readable Summaries

When running in CI mode, Test-Packages.ps1 displays:
- Per-package test summaries with color-coded results
- List of failed tests
- Overall summary across all packages
- Note about additional details being available in the test tab

### Error Handling

- Non-zero exit codes preserved for failed tests
- Warnings displayed for parsing issues
- Overall test run fails if any package has failures
- Conversion errors reported but don't fail the conversion step

### Unique Filenames

Each test run generates uniquely timestamped files to prevent conflicts in concurrent or repeated runs:
- Format: `{package}-{testtype}-{timestamp}.txt`
- Timestamp: `yyyyMMdd-HHmmss-fff` (includes milliseconds)

## Future Enhancements

1. **Capture test output logs**: Include stdout/stderr from failed tests in JUnit XML
2. **Performance metrics**: Add timing data for individual tests
3. **Trend analysis**: Track test performance over time
4. **Consider cargo-nextest**: Re-evaluate as it matures and if test execution changes are acceptable
5. **Parallel processing**: Convert multiple test result files concurrently
6. **Test  attachments**: Attach full test logs as artifacts in Azure DevOps

## References

- [Azure DevOps PublishTestResults Task](https://learn.microsoft.com/en-us/azure/devops/pipelines/tasks/reference/publish-test-results-v2)
- [JUnit XML Format](https://www.ibm.com/docs/en/developer-for-zos/14.1?topic=formats-junit-xml-format)
- [cargo test documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [cargo2junit](https://crates.io/crates/cargo2junit)
- [cargo-nextest](https://nexte.st/)
