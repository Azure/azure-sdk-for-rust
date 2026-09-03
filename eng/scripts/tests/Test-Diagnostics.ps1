#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

$global:RepoRoot = [System.IO.Path]::GetFullPath(([System.IO.Path]::Combine($PSScriptRoot, '..', '..', '..')))
. ([System.IO.Path]::Combine($RepoRoot, 'eng', 'common', 'scripts', 'logging.ps1'))
. ([System.IO.Path]::Combine($RepoRoot, 'eng', 'scripts', 'shared', 'Diagnostics.ps1'))
. ([System.IO.Path]::Combine($RepoRoot, 'eng', 'scripts', 'shared', 'Cargo.ps1'))

function Assert-Equal($Expected, $Actual, [string]$Description) {
  if ($Expected -ne $Actual) {
    throw "$Description`nExpected: $Expected`nActual:   $Actual"
  }
}

function Assert-Contains([string]$Expected, [string[]]$Actual, [string]$Description) {
  if (!($Actual | Where-Object { "$_".Contains($Expected) })) {
    throw "$Description`nExpected output containing: $Expected`nActual output:`n$($Actual -join [Environment]::NewLine)"
  }
}

Assert-Equal `
  '100%AZP25%3B%5D%0D%0A' `
  (ConvertTo-AzDevOpsLoggingValue "100%;]`r`n" -Property) `
  'Azure DevOps properties should be escaped.'

$cargoArgs = Get-CargoArgumentsWithJsonMessages @('test', '--all-features', '--', '--nocapture')
Assert-Equal `
  'test --all-features --message-format=json -- --nocapture' `
  ($cargoArgs -join ' ') `
  'Cargo JSON message format should be inserted before test-binary arguments.'

$humanFailures = @(Get-RustTestFailuresFromOutput @(
    'running 1 test',
    'test tests::fails ... FAILED',
    '',
    'failures:',
    '',
    '---- tests::fails stdout ----',
    'thread ''tests::fails'' panicked at src/lib.rs:10:5:',
    'assertion failed',
    '',
    'failures:',
    '    tests::fails'
  ))
Assert-Equal 1 $humanFailures.Count 'One human-format test failure should be parsed.'
Assert-Equal 'tests::fails' $humanFailures[0].Name 'The failed test name should be parsed.'
Assert-Contains 'assertion failed' @($humanFailures[0].Output) 'The failed test output should be retained.'

$unixSpellingIssue = ConvertFrom-CSpellIssue 'sdk/example.rs:8:14 - Unknown word (azur)'
Assert-Equal 'sdk/example.rs' $unixSpellingIssue.SourcePath 'A Unix CSpell path should be parsed.'
Assert-Equal 8 $unixSpellingIssue.LineNumber 'A CSpell line should be parsed.'
Assert-Equal 14 $unixSpellingIssue.ColumnNumber 'A CSpell column should be parsed.'

$windowsSpellingIssue = ConvertFrom-CSpellIssue 'C:\agent\_work\1\s\sdk\example.rs:9:3 - Unknown word (azur)'
Assert-Equal 'C:\agent\_work\1\s\sdk\example.rs' $windowsSpellingIssue.SourcePath 'A Windows CSpell path should be parsed.'

$oldTeamProjectId = $env:SYSTEM_TEAMPROJECTID
try {
  $env:SYSTEM_TEAMPROJECTID = 'test'
  $issueOutput = @(
    (& {
        Write-PipelineIssue `
          -Type error `
          -Message "error[E0001]`nhelp: fix 100%" `
          -SourcePath ([System.IO.Path]::Combine($RepoRoot, 'sdk', 'example.rs')) `
          -LineNumber 12 `
          -ColumnNumber 7 `
          -Code 'E0001'
      } 6>&1) | ForEach-Object { "$_" }
  )

  Assert-Equal 1 $issueOutput.Count 'One Azure DevOps logging command should be emitted.'
  Assert-Contains 'sourcepath=sdk/example.rs;' $issueOutput 'The repository-relative source path should be included.'
  Assert-Contains 'linenumber=12;columnnumber=7;code=E0001;' $issueOutput 'The source location and diagnostic code should be included.'
  Assert-Contains 'error[E0001]%0Ahelp: fix 100%AZP25' $issueOutput 'The multiline diagnostic body should be escaped.'

  $budget = New-PipelineIssueBudget -Maximum 2
  $budgetOutput = @(
    (& {
        1..3 | ForEach-Object {
          Write-BudgetedPipelineIssue -Budget $budget -Type error -Message "failure $_"
        }
        Complete-PipelineIssueBudget $budget
      } 6>&1) | ForEach-Object { "$_" }
  )

  Assert-Equal 2 $budget.Emitted 'The issue budget should emit only its maximum.'
  Assert-Equal 1 $budget.Suppressed 'The issue budget should count suppressed issues.'
  Assert-Contains 'Suppressed 1 additional pipeline issue' $budgetOutput 'Suppressed issues should be summarized.'

  $cargoMessage = @'
{
  "reason": "compiler-message",
  "message": {
    "rendered": "error[E0308]: mismatched types\n  --> sdk/example.rs:4:9\nhelp: use the expected type\n",
    "message": "mismatched types",
    "level": "error",
    "code": { "code": "E0308" },
    "spans": [
      {
        "file_name": "sdk/example.rs",
        "line_start": 4,
        "column_start": 9,
        "is_primary": true
      }
    ]
  }
}
'@ | ConvertFrom-Json -Depth 100
  $cargoBudget = New-PipelineIssueBudget
  $cargoOutput = @(
    (& {
        [void](Write-CargoCompilerDiagnostic -CargoMessage $cargoMessage -IssueBudget $cargoBudget)
      } 6>&1) | ForEach-Object { "$_" }
  )

  Assert-Equal 1 $cargoBudget.Emitted 'An error-level Cargo diagnostic should emit one issue.'
  Assert-Contains 'sourcepath=sdk/example.rs;linenumber=4;columnnumber=9;code=E0308;' $cargoOutput 'Cargo spans should map to issue metadata.'
  Assert-Contains 'help: use the expected type' $cargoOutput 'The complete rendered Cargo diagnostic should be retained.'

  $testEvent = @'
{
  "type": "test",
  "event": "failed",
  "name": "tests::fails",
  "stdout": "thread 'tests::fails' panicked at src/lib.rs:10:5:\nassertion failed"
}
'@ | ConvertFrom-Json
  $testBudget = New-PipelineIssueBudget
  $testOutput = @(
    (& {
        [void](Write-RustJsonTestEvent -TestEvent $testEvent -IssueBudget $testBudget)
      } 6>&1) | ForEach-Object { "$_" }
  )

  Assert-Equal 1 $testBudget.Emitted 'A failed JSON test event should emit one issue.'
  Assert-Contains "Test 'tests::fails' failed." $testOutput 'The failed test name should be included.'
  Assert-Contains 'assertion failed' $testOutput 'Captured test output should be included.'
}
finally {
  $env:SYSTEM_TEAMPROJECTID = $oldTeamProjectId
}

Write-Host 'Diagnostic parser tests passed.'
