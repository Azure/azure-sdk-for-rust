# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
$output = $env:COSMOS_PERF_OUTPUT
$environment = Get-Content -Raw ([System.IO.Path]::Combine($output, 'environment.json')) | ConvertFrom-Json
$mode = Get-Content -Raw ([System.IO.Path]::Combine($output, 'execution-mode.json')) | ConvertFrom-Json
if ($mode.version -ne 1 -or $mode.calibrate -isnot [bool]) {
    throw 'Invalid performance execution mode from preflight.'
}
$arguments = @(
    '--endpoint', $env:AZURE_COSMOS_ENDPOINT,
    '--auth', 'key',
    '--application-region', $environment.region,
    '--commit-sha', $environment.source_commit,
    '--workload-id', $env:BUILD_BUILDID,
    '--perf-profile', ([System.IO.Path]::Combine($output, 'perf-profile.json')),
    '--perf-environment', ([System.IO.Path]::Combine($output, 'environment.json')),
    '--perf-output', $output
)
if ($mode.calibrate) {
    $arguments += '--calibrate'
    Write-Host 'CALIBRATION ONLY: this run cannot produce a performance-gate pass.'
} else {
    $arguments += @('--thresholds', ([System.IO.Path]::Combine($output, 'perf-thresholds.json')))
}
# Capture native stderr alongside stdout, then preserve the native exit status.
& ./target/release/azure_data_cosmos_perf @arguments 2>&1 |
    Tee-Object -FilePath ([System.IO.Path]::Combine($output, 'run.log'))
$exitCode = $LASTEXITCODE
if ($exitCode -eq 0 -and $mode.calibrate) {
    Write-Host '##vso[task.complete result=SucceededWithIssues;]Calibration collected; not a performance pass.'
}
exit $exitCode
