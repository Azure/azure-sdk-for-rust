# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
. ([System.IO.Path]::Combine($PSScriptRoot, 'prepare-cosmos-perf.ps1'))

function New-Fixture {
    return @{
        Account = @{
            documentEndpoint = 'https://perf.documents.azure.com/'
            readLocations = @(@{ locationName = 'East US 2' })
            writeLocations = @(@{ locationName = 'East US 2' })
            consistencyPolicy = @{ defaultConsistencyLevel = 'Session' }
        }
        Vm = @{ location = 'eastus2'; vmSize = 'Standard_D8s_v5'; osType = 'Linux' }
        Profile = @{ vm_size = 'Standard_D8s_v5'; consistency = 'Session' }
        Endpoint = 'https://perf.documents.azure.com:443/'
    }
}

$fixture = New-Fixture
Assert-CosmosPerfEnvironment @fixture
Write-Host 'PASS: canonical and display region names match'

$cases = @{
    'remote VM' = { param($f) $f.Vm.location = 'westus3' }
    'wrong SKU' = { param($f) $f.Vm.vmSize = 'Standard_D2s_v5' }
    'wrong OS' = { param($f) $f.Vm.osType = 'Windows' }
    'wrong consistency' = { param($f) $f.Account.consistencyPolicy.defaultConsistencyLevel = 'Eventual' }
    'wrong account' = { param($f) $f.Endpoint = 'https://another.documents.azure.com/' }
    'HTTP endpoint' = { param($f) $f.Endpoint = 'http://perf.documents.azure.com/' }
    'multi-region account' = { param($f) $f.Account.readLocations += @{ locationName = 'West US 3' } }
    'different write region' = { param($f) $f.Account.writeLocations[0].locationName = 'West US 3' }
    'missing locations' = { param($f) $f.Account.readLocations = @() }
}
foreach ($name in $cases.Keys) {
    $fixture = New-Fixture
    & $cases[$name] $fixture
    $rejected = $false
    try { Assert-CosmosPerfEnvironment @fixture } catch { $rejected = $true }
    if (-not $rejected) { throw "Preflight accepted invalid fixture: $name" }
    Write-Host "PASS: rejects $name"
}

$files = @('prepare-cosmos-perf.ps1', 'run-cosmos-perf.ps1', 'cleanup-cosmos-perf.ps1')
foreach ($file in $files) {
    $tokens = $null
    $errors = $null
    [System.Management.Automation.Language.Parser]::ParseFile(
        ([System.IO.Path]::Combine($PSScriptRoot, $file)), [ref]$tokens, [ref]$errors
    ) | Out-Null
    if ($errors.Count) { throw "PowerShell parse errors in ${file}: $errors" }
}
Write-Host 'PASS: all performance scripts parse'

$initialBaseline = '{"version":1,"approved":false,"profile":null,"thresholds":[]}' | ConvertFrom-Json
$approvedBaseline = '{"version":1,"approved":true,"profile":null,"thresholds":[]}' | ConvertFrom-Json
$modes = @(
    @{ Name = 'weekly bootstrap'; Weekly = $true; Requested = $false; HasBaseline = $false; Thresholds = $initialBaseline; Expected = $true },
    @{ Name = 'weekly uncalibrated baseline'; Weekly = $true; Requested = $false; HasBaseline = $true; Thresholds = $initialBaseline; Expected = $true },
    @{ Name = 'weekly reviewed baseline'; Weekly = $true; Requested = $false; HasBaseline = $true; Thresholds = $approvedBaseline; Expected = $false },
    @{ Name = 'manual reviewed baseline'; Weekly = $false; Requested = $false; HasBaseline = $true; Thresholds = $approvedBaseline; Expected = $false },
    @{ Name = 'explicit manual calibration'; Weekly = $false; Requested = $true; HasBaseline = $true; Thresholds = $approvedBaseline; Expected = $true },
    @{ Name = 'manual calibration bootstrap'; Weekly = $false; Requested = $true; HasBaseline = $false; Thresholds = $initialBaseline; Expected = $true },
    @{ Name = 'candidate approval cannot enable gating'; Weekly = $true; Requested = $false; HasBaseline = $false; Thresholds = $approvedBaseline; Expected = $true }
)
foreach ($case in $modes) {
    $actual = Get-CosmosPerfCalibration $case.Weekly $case.Requested $case.HasBaseline $case.Thresholds
    if ($actual -ne $case.Expected) { throw "Incorrect mode for $($case.Name)" }
    Write-Host "PASS: $($case.Name)"
}

$rejections = @(
    @{ Name = 'manual gate without baseline'; Weekly = $false; HasBaseline = $false; Thresholds = $initialBaseline },
    @{ Name = 'manual gate with unapproved baseline'; Weekly = $false; HasBaseline = $true; Thresholds = $initialBaseline },
    @{ Name = 'invalid baseline version'; Weekly = $true; HasBaseline = $true; Thresholds = ('{"version":2,"approved":false,"profile":null,"thresholds":[]}' | ConvertFrom-Json) },
    @{ Name = 'string approval flag'; Weekly = $true; HasBaseline = $true; Thresholds = ('{"version":1,"approved":"false","profile":null,"thresholds":[]}' | ConvertFrom-Json) },
    @{ Name = 'missing approval flag'; Weekly = $true; HasBaseline = $true; Thresholds = ('{"version":1,"profile":null,"thresholds":[]}' | ConvertFrom-Json) },
    @{ Name = 'partial unapproved baseline'; Weekly = $true; HasBaseline = $true; Thresholds = ('{"version":1,"approved":false,"profile":{},"thresholds":[]}' | ConvertFrom-Json) },
    @{ Name = 'malformed empty thresholds'; Weekly = $true; HasBaseline = $true; Thresholds = ('{"version":1,"approved":false,"profile":null,"thresholds":null}' | ConvertFrom-Json) }
)
foreach ($case in $rejections) {
    $rejected = $false
    try {
        Get-CosmosPerfCalibration $case.Weekly $false $case.HasBaseline $case.Thresholds | Out-Null
    } catch { $rejected = $true }
    if (-not $rejected) { throw "Invalid baseline silently selected a mode: $($case.Name)" }
    Write-Host "PASS: rejects $($case.Name)"
}

Assert-CosmosPerfSourceRef 'refs/heads/main'
Assert-CosmosPerfSourceRef 'refs/pull/5469/head'
foreach ($sourceRef in @('refs/pull/5469/merge', 'refs/pull/5469/unknown')) {
    $rejected = $false
    try { Assert-CosmosPerfSourceRef $sourceRef } catch { $rejected = $true }
    if (-not $rejected) { throw "Invalid PR ref accepted: $sourceRef" }
}
Write-Host 'PASS: exact PR heads are accepted and synthetic merge refs are rejected'
