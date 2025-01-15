# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
# cspell: ignore JOBID

. "$PSScriptRoot\..\..\..\eng\common\scripts\common.ps1"

Write-Host "Test Broker output:"
Receive-Job -Id $env:TEST_BROKER_JOBID

# Check if the test broker job is still running
$job = Get-Job -Id $env:TEST_BROKER_JOBID
if ($job.State -ne "Running") {
  Write-Host "Test broker terminated unexpectedly."
  exit 1
}

# Stop the test broker job started in Test-Setup.ps1
Write-Host "Stopping test broker"
Stop-Job -Id $env:TEST_BROKER_JOBID
Remove-Job -Id $env:TEST_BROKER_JOBID
Write-Host "Test broker stopped."
