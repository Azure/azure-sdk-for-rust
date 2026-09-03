# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# Every other build, lint, and test step in CI compiles the Cosmos crates with
# `--all-features`, so an item that a default-feature code path uses but a cargo
# feature gates only fails once it reaches a consumer on the default feature set.

. ([System.IO.Path]::Combine($PSScriptRoot, '..', '..', '..', '..', 'eng', 'common', 'scripts', 'common.ps1'))

# Work around a temporary issue where Invoke-LoggedCommand, which calls us, needs LASTEXITCODE to be set
$global:LASTEXITCODE = 0

# Test-Setup.ps1 runs once per package, but this check already covers every
# Cosmos crate, so only the first invocation in a job does the work.
if ($env:AZURE_COSMOS_DEFAULT_FEATURE_CHECK_COMPLETE -eq 'true') {
  Write-Host 'Default-feature check already ran for this job; skipping.'
  return
}

$packages = @(
  'azure_data_cosmos'
  'azure_data_cosmos_driver'
  'azure_data_cosmos_driver_native'
)
$packageArgs = '--package ' + ($packages -join ' --package ')

LogGroupStart 'Checking Cosmos crates with default features'
Invoke-LoggedCommand "cargo check $packageArgs --all-targets --keep-going"
LogGroupEnd

$env:AZURE_COSMOS_DEFAULT_FEATURE_CHECK_COMPLETE = 'true'
