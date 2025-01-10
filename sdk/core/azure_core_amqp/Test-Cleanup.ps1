# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
# cspell: ignore JOBID

param (
  [string]$PackageName
)

. "$PSScriptRoot\..\..\..\eng\common\scripts\common.ps1"

if (-not $PackageName) {
  Write-Host "Please provide a package name."
  exit 1
}

Write-Host Currently running jobs:
Get-Job

Write-Host Job output:
Receive-Job -Id $env:TEST_BROKER_JOBID

if (-not((Get-Job.State -Id $env:TEST_BROKER_JOBID) -eq "Running")) {
  Write-Host "Test broker terminated unexpectedly."
  exit 1
}


# Stop the test broker job started in Test-Setup.ps1
Write-Host "Stopping test broker with Job ID: $env:TEST_BROKER_JOBID"

Write-Host Stopping job...
Stop-Job -Id $env:TEST_BROKER_JOBID

Write-Host Removing job...
Remove-Job -Id $env:TEST_BROKER_JOBID

Write-Host Currently running jobs:
Get-Job

Write-Host "Test broker stopped."
