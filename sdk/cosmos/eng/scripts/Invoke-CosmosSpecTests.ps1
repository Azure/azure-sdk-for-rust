# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
# cSpell:ignore TEAMPROJECTID JOBNAME JOBATTEMPT ARTIFACTSTAGINGDIRECTORY artifactname logissue

# Runs the cosspec spec tests once per job and publishes their results as a
# pipeline artifact. Dot-sourced from the Cosmos crates' Test-Setup.ps1 after
# Invoke-CosmosTestSetup.ps1, so a legacy emulator started there is available.
#
# * Runs in Azure Pipelines by default; set AZURE_COSMOS_SPEC=false to opt out.
#   Locally it runs only with AZURE_COSMOS_SPEC=true.
# * Uses cosspec's in-memory default config, or configs/emulator.toml when the
#   legacy Azure Cosmos DB Emulator is running.
# * Skips legs that run a specialized mode (fuzz, in-memory emulator flavors,
#   E2E profiles, the vNext emulator, live accounts).
# * A spec failure fails the task without aborting setup, so the package's own
#   tests still run. Results are published whether specs pass or fail.

. ([System.IO.Path]::Combine($PSScriptRoot, '..', '..', '..', '..', 'eng', 'common', 'scripts', 'common.ps1'))

# Work around a temporary issue where Invoke-LoggedCommand, which calls us, needs LASTEXITCODE to be set
$global:LASTEXITCODE = 0

function Get-CosmosSpecSkipReason {
    if ($env:AZURE_COSMOS_SPEC_RAN) { return 'already ran in this job' }
    if ($env:AZURE_COSMOS_SPEC -eq 'false') { return 'AZURE_COSMOS_SPEC=false' }
    if (-not $env:TF_BUILD -and $env:AZURE_COSMOS_SPEC -ne 'true') {
        return 'not in CI; set AZURE_COSMOS_SPEC=true to run locally'
    }
    if ($env:AZURE_COSMOS_FUZZ -eq '1') { return 'fuzz leg' }
    if ($env:AZURE_COSMOS_EMULATOR_FLAVOR) { return "emulator flavor '$env:AZURE_COSMOS_EMULATOR_FLAVOR'" }
    if ($env:AZURE_COSMOS_E2E_PROFILE) { return "E2E profile '$env:AZURE_COSMOS_E2E_PROFILE'" }
    if ($env:AZURE_COSMOS_AUTH_MODE -eq 'aad') { return 'live account leg' }
    if ($env:AZURE_COSMOS_CONNECTION_STRING -and $env:AZURE_COSMOS_CONNECTION_STRING -ne 'emulator') {
        return 'live account leg'
    }
    return $null
}

$skipReason = Get-CosmosSpecSkipReason
if ($skipReason) {
    Write-Host "Skipping cosspec spec tests: $skipReason."
    return
}
$env:AZURE_COSMOS_SPEC_RAN = '1'

$repoRoot = (Resolve-Path ([System.IO.Path]::Combine($PSScriptRoot, '..', '..', '..', '..'))).Path
$inAzurePipelines = [bool]$env:SYSTEM_TEAMPROJECTID

$harnessArgs = @()
if ($env:AZURE_COSMOS_LEGACY_EMULATOR_READY -eq '1') {
    $config = [System.IO.Path]::Combine($repoRoot, 'sdk', 'cosmos', 'cosspec', 'configs', 'emulator.toml')
    $harnessArgs += @('--config', $config)
}
$resultsDir = $null
if ($inAzurePipelines -and $env:BUILD_ARTIFACTSTAGINGDIRECTORY) {
    $resultsDir = [System.IO.Path]::Combine($env:BUILD_ARTIFACTSTAGINGDIRECTORY, 'cosspec-results')
    $harnessArgs += @('--results-dir', $resultsDir)
}

LogGroupStart 'Running cosspec spec tests'
Push-Location $repoRoot
try {
    Write-Host "> cargo test -p cosspec --test spec -- $($harnessArgs -join ' ')"
    & cargo test -p cosspec --test spec -- @harnessArgs
    $specExitCode = $LASTEXITCODE
}
finally {
    Pop-Location
    LogGroupEnd
}

if ($inAzurePipelines -and $resultsDir -and (Test-Path $resultsDir)) {
    $leg = "$env:AGENT_JOBNAME-$env:SYSTEM_JOBATTEMPT" -replace '[^A-Za-z0-9_.-]', '_'
    Write-Host "Publishing cosspec results as 'cosspec-results-$leg'."
    Write-Host "##vso[artifact.upload artifactname=cosspec-results-$leg]$resultsDir"
}

if ($specExitCode -ne 0) {
    if (-not $inAzurePipelines) {
        throw "cosspec spec tests failed with exit code $specExitCode."
    }
    # Fail the task, but let setup finish so the package's own tests still run.
    Write-Host "##vso[task.logissue type=error]cosspec spec tests failed with exit code $specExitCode."
    Write-Host '##vso[task.complete result=Failed;]cosspec spec tests failed.'
    $global:LASTEXITCODE = 0
}
