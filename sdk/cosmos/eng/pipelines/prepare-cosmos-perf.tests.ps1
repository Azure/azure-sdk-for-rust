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
