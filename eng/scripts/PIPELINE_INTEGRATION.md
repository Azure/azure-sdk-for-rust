# Pipeline Integration for Test Results Reporting

This document describes how to integrate the test results reporting into Azure DevOps pipelines.

## Changes Required in Pipeline YAML

The following changes need to be made to `eng/pipelines/templates/jobs/ci.tests.yml` and `eng/pipelines/templates/jobs/live.tests.yml`:

### 1. Update Test-Packages.ps1 Task

Add the `-CI` parameter to enable test result capture:

**Before:**
```yaml
- task: Powershell@2
  displayName: "Test Packages"
  condition: and(succeeded(), ne(variables['NoPackagesChanged'],'true'))
  timeoutInMinutes: ${{ parameters.TimeoutInMinutes }}
  env:
    CIBW_BUILD_VERBOSITY: 3
  inputs:
    pwsh: true
    filePath: $(Build.SourcesDirectory)/eng/scripts/Test-Packages.ps1
    arguments: >
      -PackageInfoDirectory '$(Build.ArtifactStagingDirectory)/PackageInfo'
```

**After:**
```yaml
- task: Powershell@2
  displayName: "Test Packages"
  condition: and(succeeded(), ne(variables['NoPackagesChanged'],'true'))
  timeoutInMinutes: ${{ parameters.TimeoutInMinutes }}
  env:
    CIBW_BUILD_VERBOSITY: 3
  inputs:
    pwsh: true
    filePath: $(Build.SourcesDirectory)/eng/scripts/Test-Packages.ps1
    arguments: >
      -PackageInfoDirectory '$(Build.ArtifactStagingDirectory)/PackageInfo'
      -CI
```

### 2. Add Convert-TestResults.ps1 Task

Add a new task after the test task to convert results to JUnit XML:

```yaml
- task: Powershell@2
  displayName: "Convert Test Results to JUnit XML"
  condition: succeededOrFailed()
  inputs:
    pwsh: true
    filePath: $(Build.SourcesDirectory)/eng/scripts/Convert-TestResults.ps1
    arguments: >
      -TestResultsDirectory '$(Build.SourcesDirectory)/test-results'
      -OutputDirectory '$(Build.SourcesDirectory)/test-results/junit'
```

### 3. Add PublishTestResults Task

Add a task to publish the JUnit XML results to Azure DevOps:

```yaml
- task: PublishTestResults@2
  displayName: "Publish Test Results"
  condition: succeededOrFailed()
  inputs:
    testResultsFormat: 'JUnit'
    testResultsFiles: '**/test-results/junit/*.xml'
    testRunTitle: 'Rust Tests - $(Agent.JobName)'
    mergeTestResults: true
    failTaskOnFailedTests: false
    publishRunAttachments: true
```

## Complete Example

Here's a complete example showing the test tasks section with all changes:

```yaml
steps:
  # ... previous setup steps ...

  - task: Powershell@2
    displayName: "Test Packages"
    condition: and(succeeded(), ne(variables['NoPackagesChanged'],'true'))
    timeoutInMinutes: ${{ parameters.TimeoutInMinutes }}
    env:
      CIBW_BUILD_VERBOSITY: 3
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
      publishRunAttachments: true

  # ... remaining steps ...
```

## Files to Update

The following pipeline files need to be updated:

1. `eng/pipelines/templates/jobs/ci.tests.yml` - CI test jobs
2. `eng/pipelines/templates/jobs/live.tests.yml` - Live test jobs (both PowerShell and AzurePowerShell tasks)

## Testing the Changes

After updating the pipeline files:

1. Run a test pipeline to verify test results are captured
2. Check the Azure DevOps "Tests" tab to ensure results are displayed
3. Verify that failed tests show up correctly with failure details
4. Confirm that test trends are visible across multiple runs

## Important Notes

### Task Conditions

- **Convert-TestResults task**: Uses `condition: succeededOrFailed()` to ensure conversion runs even if tests fail
- **PublishTestResults task**: Uses `condition: succeededOrFailed()` to ensure results are published regardless of test outcome
- **failTaskOnFailedTests**: Set to `false` to let the test task itself control the build outcome

### Test Result Files

- Test output files are stored in `$(Build.SourcesDirectory)/test-results/`
- JUnit XML files are stored in `$(Build.SourcesDirectory)/test-results/junit/`
- These directories are automatically created by the scripts
- Files are automatically cleaned up between builds (gitignored)

### Backward Compatibility

- The `-CI` parameter is optional
- Without `-CI`, Test-Packages.ps1 behaves exactly as before
- This allows for gradual rollout and testing

## Troubleshooting

### No test results appear in Azure DevOps

1. Check that the `-CI` parameter was added to Test-Packages.ps1
2. Verify that test-results/junit/*.xml files were created
3. Check the PublishTestResults task log for errors
4. Ensure the testResultsFiles pattern matches the actual file locations

### Test results show but counts are wrong

1. Check the conversion script output for warnings
2. Review the plain text output files in test-results/
3. Verify cargo test output format hasn't changed
4. Check for parsing errors in the Convert-TestResults.ps1 log

### Tests pass but task fails

1. Check the exit code handling in Test-Packages.ps1
2. Verify AllowedExitCodes in pipeline task configuration
3. Review the overall test summary output

## Future Enhancements

1. **Test attachments**: Attach full test logs for failed tests
2. **Performance tracking**: Add test duration trending
3. **Flaky test detection**: Identify intermittently failing tests
4. **Parallel conversion**: Process multiple result files concurrently
5. **Custom test categorization**: Group tests by type or component
