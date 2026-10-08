# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
#Requires -Version 7.4

[CmdletBinding()]
param(
    [string] $Profile = $env:AZURE_COSMOS_E2E_PROFILE,
    [string] $Backend = $env:AZURE_COSMOS_E2E_BACKEND,
    [string] $Account = $env:AZURE_COSMOS_E2E_ACCOUNT,
    [string] $Runtime = $env:AZURE_COSMOS_E2E_RUNTIME,
    [string] $Client = $env:AZURE_COSMOS_E2E_CLIENT,
    [string] $ActualTransport = $env:AZURE_COSMOS_E2E_ACTUAL_TRANSPORT,
    [string] $JUnitDirectory,
    [string] $OutputDirectory
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$repoRoot = (Resolve-Path ([System.IO.Path]::Combine($PSScriptRoot, '..', '..', '..', '..'))).Path
$e2eRoot = ([System.IO.Path]::Combine($repoRoot, 'sdk', 'cosmos', 'e2e_tests'))
if ([string]::IsNullOrWhiteSpace($JUnitDirectory)) {
    $JUnitDirectory = ([System.IO.Path]::Combine($repoRoot, 'test-results', 'junit'))
}
if ([string]::IsNullOrWhiteSpace($OutputDirectory)) {
    $OutputDirectory = ([System.IO.Path]::Combine($repoRoot, 'test-results', 'e2e'))
}
if ([string]::IsNullOrWhiteSpace($Profile) -or [string]::IsNullOrWhiteSpace($Backend)) {
    Write-Host 'AZURE_COSMOS_E2E_PROFILE and AZURE_COSMOS_E2E_BACKEND are required.' -ForegroundColor Red
    exit 1
}
if (-not (Test-Path $JUnitDirectory)) {
    Write-Host "JUnit directory '$JUnitDirectory' does not exist." -ForegroundColor Red
    exit 1
}

$knownBackends = @('hostedEmulatorGatewayV1', 'hostedEmulatorGatewayV2', 'azureLive')
if ($Backend -notin $knownBackends) {
    Write-Host "Unsupported E2E backend '$Backend'." -ForegroundColor Red
    exit 1
}
if ([string]::IsNullOrWhiteSpace($ActualTransport)) {
    $ActualTransport = switch ($Backend) {
        'hostedEmulatorGatewayV1' { 'gatewayV1' }
        'hostedEmulatorGatewayV2' { 'gatewayV2' }
        'azureLive' { 'gatewayV1' }
    }
}

$profilePath = ([System.IO.Path]::Combine($e2eRoot, 'profiles', "$Profile.json"))
if (-not (Test-Path $profilePath)) {
    Write-Host "E2E profile '$Profile' does not exist." -ForegroundColor Red
    exit 1
}
$implementation = Get-Content ([System.IO.Path]::Combine($e2eRoot, 'implementations', 'rust.json')) -Raw | ConvertFrom-Json
$implementationById = @{}
foreach ($entry in @($implementation.scenarios)) {
    $implementationById[[string]$entry.id] = $entry
}
$quarantine = Get-Content ([System.IO.Path]::Combine($e2eRoot, 'quarantine.json')) -Raw | ConvertFrom-Json
$shards = Get-Content ([System.IO.Path]::Combine($e2eRoot, 'shards.v1.json')) -Raw | ConvertFrom-Json
$maximumDurationMinutes = [double]$shards.maximumDurationMinutes

$testCases = @()
foreach ($resultFile in @(Get-ChildItem $JUnitDirectory -Recurse -Filter '*.xml')) {
    [xml]$document = Get-Content $resultFile.FullName -Raw
    foreach ($testCase in @($document.SelectNodes('//testcase'))) {
        $testCases += [pscustomobject]@{
            Name      = [string]$testCase.name
            ClassName = [string]$testCase.classname
            Time      = if ($testCase.time) { [double]::Parse([string]$testCase.time, [Globalization.CultureInfo]::InvariantCulture) } else { 0.0 }
            Failed    = $null -ne $testCase.SelectSingleNode('./failure') -or $null -ne $testCase.SelectSingleNode('./error')
            Skipped   = $null -ne $testCase.SelectSingleNode('./skipped')
            File      = $resultFile.Name
        }
    }
}

$records = @()
$gateErrors = @()
$scenarioFiles = @(Get-ChildItem ([System.IO.Path]::Combine($e2eRoot, 'scenarios')) -Recurse -Filter '*.json')
foreach ($scenarioFile in $scenarioFiles) {
    $scenario = Get-Content $scenarioFile.FullName -Raw | ConvertFrom-Json
    if ($Profile -notin @($scenario.profiles)) {
        continue
    }
    $backendMetadata = $scenario.backends.$Backend
    if ($backendMetadata.applicability -eq 'notApplicable') {
        continue
    }
    $mapping = $implementationById[[string]$scenario.id]
    $match = @($testCases | Where-Object {
            $qualifiedName = "$($_.ClassName)::$($_.Name)"
            $qualifiedName.Contains([string]$mapping.test) -or $_.Name -eq [string]$mapping.test
        })
    $status = if ($match.Count -eq 0) {
        'missing'
    }
    elseif (@($match | Where-Object Failed).Count -gt 0) {
        'failed'
    }
    elseif (@($match | Where-Object Skipped).Count -gt 0) {
        'skipped'
    }
    else {
        'passed'
    }
    $durationSeconds = 0.0
    foreach ($testCase in $match) {
        $durationSeconds += $testCase.Time
    }
    $durationSeconds = [Math]::Round($durationSeconds, 3)
    $quarantineEntry = @($quarantine.entries | Where-Object {
            $_.scenario -eq $scenario.id -and
            $Backend -in @($_.backends) -and
            $Profile -in @($_.profiles)
        }) | Select-Object -First 1
    if ($null -ne $quarantineEntry) {
        $expiry = [DateTimeOffset]::ParseExact(
            [string]$quarantineEntry.expires,
            'yyyy-MM-dd',
            [Globalization.CultureInfo]::InvariantCulture)
        if ($expiry -lt [DateTimeOffset]::UtcNow.Date) {
            $gateErrors += "quarantine for '$($scenario.id)' expired on $($quarantineEntry.expires)"
        }
        elseif ($status -ne 'passed') {
            $status = 'quarantined'
        }
    }
    if ($backendMetadata.applicability -eq 'required' -and $status -notin @('passed', 'quarantined')) {
        $gateErrors += "required scenario '$($scenario.id)' is $status"
    }
    $records += [pscustomobject][ordered]@{
        scenario        = [string]$scenario.id
        maturity        = [string]$scenario.maturity
        applicability   = [string]$backendMetadata.applicability
        profile         = $Profile
        account         = $Account
        runtime         = $Runtime
        client          = $Client
        backend         = $Backend
        actualTransport = $ActualTransport
        status          = $status
        durationSeconds = $durationSeconds
        attempts        = 1
        test             = [string]$mapping.test
    }
}

$totalDurationSeconds = 0.0
foreach ($record in $records) {
    $totalDurationSeconds += $record.durationSeconds
}
$totalDurationSeconds = [Math]::Round($totalDurationSeconds, 3)
if ($totalDurationSeconds -gt ($maximumDurationMinutes * 60)) {
    $gateErrors += "shard duration $totalDurationSeconds seconds exceeds the $maximumDurationMinutes-minute limit"
}
if ($records.Count -eq 0) {
    $gateErrors += "profile '$Profile' selected no applicable scenarios for '$Backend'"
}

New-Item -ItemType Directory -Path $OutputDirectory -Force | Out-Null
$report = [ordered]@{
    specVersion           = '1.0'
    sdk                   = 'rust'
    generatedAtUtc        = [DateTimeOffset]::UtcNow.ToString('O')
    profile               = $Profile
    account               = $Account
    runtime               = $Runtime
    client                = $Client
    backend               = $Backend
    actualTransport       = $ActualTransport
    maximumDurationMinutes = $maximumDurationMinutes
    durationSeconds       = $totalDurationSeconds
    gate                  = if ($gateErrors.Count -eq 0) { 'passed' } else { 'failed' }
    errors                = $gateErrors
    scenarios             = $records
}
$jsonPath = ([System.IO.Path]::Combine($OutputDirectory, 'coverage.json'))
$markdownPath = ([System.IO.Path]::Combine($OutputDirectory, 'coverage.md'))
$report | ConvertTo-Json -Depth 10 | Set-Content $jsonPath

$markdown = @(
    '# Cosmos SDK E2E coverage',
    '',
    "- Profile: ``$Profile``",
    "- Backend: ``$Backend``",
    "- Actual transport: ``$ActualTransport``",
    "- Runtime: $totalDurationSeconds seconds (limit: $maximumDurationMinutes minutes)",
    "- Gate: **$($report.gate)**",
    '',
    '| Scenario | Applicability | Status | Duration (s) |',
    '| --- | --- | --- | ---: |'
)
foreach ($record in $records) {
    $markdown += "| ``$($record.scenario)`` | $($record.applicability) | $($record.status) | $($record.durationSeconds) |"
}
if ($gateErrors.Count -gt 0) {
    $markdown += @('', '## Gate errors', '')
    foreach ($gateError in $gateErrors) {
        $markdown += "- $gateError"
    }
}
$markdown | Set-Content $markdownPath
Write-Host "Wrote Cosmos E2E coverage reports to '$OutputDirectory'."

if ($gateErrors.Count -gt 0) {
    foreach ($gateError in $gateErrors) {
        Write-Host "ERROR: $gateError" -ForegroundColor Red
    }
    exit 1
}
