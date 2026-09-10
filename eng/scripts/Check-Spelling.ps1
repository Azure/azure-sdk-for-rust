#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0
[CmdletBinding()]
param(
  [string]$CspellConfigPath = ([System.IO.Path]::Combine($PSScriptRoot, '..', '..', '.vscode', 'cspell.json')),
  [string]$SpellCheckRoot = ([System.IO.Path]::Combine($PSScriptRoot, '..', '..')),
  [switch]$ExitWithError,
  [string]$SourceCommittish = $env:SYSTEM_PULLREQUEST_SOURCECOMMITID,
  [string]$TargetCommittish = ("origin/$($env:SYSTEM_PULLREQUEST_TARGETBRANCH)" -replace 'refs/heads/')
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'common.ps1'))

if (!(Test-Path -Path $CspellConfigPath -PathType Leaf)) {
  Write-PipelineIssue -Type error -Message "Could not locate CSpell config file '$CspellConfigPath'."
  exit 1
}

$getChangedFilesScript = ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'get-changedfiles.ps1'))
$invokeCspellScript = ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'spelling', 'Invoke-Cspell.ps1'))
$changedFiles = @(
  & $getChangedFilesScript `
    -SourceCommittish $SourceCommittish `
    -TargetCommittish $TargetCommittish |
    ForEach-Object { Resolve-Path -Path $_ }
)

Write-Host "Git detected $($changedFiles.Count) changed file(s). Files checked by CSpell may exclude files according to cspell.json."
if ($changedFiles.Count -eq 0) {
  Write-Host 'No changes detected.'
  exit 0
}

$spellingOutput = @(
  & $invokeCspellScript `
    -CSpellConfigPath $CspellConfigPath `
    -SpellCheckRoot $SpellCheckRoot `
    -FileList $changedFiles.Path
)
$cspellExitCode = $LASTEXITCODE
$issueBudget = New-PipelineIssueBudget
$parsedIssues = 0

foreach ($line in $spellingOutput) {
  $text = "$line"
  Write-Host $text
  $issue = ConvertFrom-CSpellIssue $text
  if ($issue) {
    Write-BudgetedPipelineIssue `
      -Budget $issueBudget `
      -Type $(if ($ExitWithError) { 'error' } else { 'warning' }) `
      -Message $issue.Message `
      -SourcePath $issue.SourcePath `
      -LineNumber $issue.LineNumber `
      -ColumnNumber $issue.ColumnNumber `
      -Code 'cspell'
    $parsedIssues++
  }
}

Complete-PipelineIssueBudget $issueBudget

if ($parsedIssues -gt 0) {
  Write-Host 'Spelling errors detected. To correct false positives or learn about spell checking, see https://aka.ms/azsdk/engsys/spellcheck.'
  if ($ExitWithError) {
    exit 1
  }
}
elseif ($cspellExitCode -ne 0) {
  Write-PipelineIssue -Type error -Message "CSpell exited with code $cspellExitCode. This may indicate a configuration or tool failure."
  exit $cspellExitCode
}
else {
  Write-Host 'No spelling errors detected.'
}

exit 0
