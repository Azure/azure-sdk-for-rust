# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
# cspell: ignore JOBID

. "$PSScriptRoot\..\..\..\eng\common\scripts\common.ps1"

if ($IsMacOS) {
  Write-Host "AMQP tests are not supported on macOS. Skipping test setup."
  exit 0
}

# Test-Setup.ps1 clears TEST_BROKER_JOBID when it stops the broker itself, so
# there is nothing to clean up.
if ([string]::IsNullOrWhiteSpace($env:TEST_BROKER_JOBID)) {
  Write-Host "TEST_BROKER_JOBID is not set. The test broker is not running."
  exit 0
}

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
