# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
. ([System.IO.Path]::Combine($PSScriptRoot, 'prepare-cosmos-perf.ps1')) -FunctionsOnly

function New-Fixture {
    return @{
        Account = @{
            id = 'perf'
            readableLocations = @(@{ name = 'East US 2' })
            writableLocations = @(@{ name = 'East US 2' })
            userConsistencyPolicy = @{ defaultConsistencyLevel = 'Session' }
        }
        Vm = @{ location = 'eastus2'; vmSize = 'Standard_D8s_v5'; osType = 'Linux' }
        Profile = @{ vm_size = 'Standard_D8s_v5'; consistency = 'Session' }
        Endpoint = 'https://perf.documents.azure.com:443/'
    }
}

$fixture = New-Fixture
$placement = Get-CosmosPerfPlacement @fixture
if ($placement.region -ne 'East US 2' -or $placement.excluded_regions.Count -ne 0) {
    throw 'Single-region placement differs from the account metadata.'
}
Write-Host 'PASS: canonical and display region names match'

$fixture.Account.readableLocations += @{ name = 'West US 3' }
$placement = Get-CosmosPerfPlacement @fixture
if ($placement.region -ne 'East US 2' -or ($placement.excluded_regions -join ',') -ne 'West US 3') {
    throw 'Single-write multi-region placement did not exclude the remote read region.'
}
$fixture.Account.writableLocations = @(@{ name = 'West US 3' }, @{ name = 'East US 2' })
$placement = Get-CosmosPerfPlacement @fixture
if ($placement.region -ne 'East US 2' -or ($placement.excluded_regions -join ',') -ne 'West US 3') {
    throw 'Multi-write placement did not select the agent region independently of account ordering.'
}
Write-Host 'PASS: multi-region accounts select a local writable region and exclude every other region'

$cases = @{
    'remote VM' = { param($f) $f.Vm.location = 'westus3' }
    'wrong SKU' = { param($f) $f.Vm.vmSize = 'Standard_D2s_v5' }
    'wrong OS' = { param($f) $f.Vm.osType = 'Windows' }
    'wrong consistency' = { param($f) $f.Account.userConsistencyPolicy.defaultConsistencyLevel = 'Eventual' }
    'wrong account' = { param($f) $f.Endpoint = 'https://another.documents.azure.com/' }
    'HTTP endpoint' = { param($f) $f.Endpoint = 'http://perf.documents.azure.com/' }
    'read-only local region' = { param($f) $f.Account.readableLocations += @{ name = 'West US 3' }; $f.Vm.location = 'westus3' }
    'write-only local region' = { param($f) $f.Account.writableLocations += @{ name = 'West US 3' }; $f.Vm.location = 'westus3' }
    'different write region' = { param($f) $f.Account.writableLocations[0].name = 'West US 3' }
    'missing locations' = { param($f) $f.Account.readableLocations = @() }
    'empty location names' = { param($f) $f.Account.writableLocations[0].name = ''; $f.Account.readableLocations[0].name = ''; $f.Vm.location = '' }
    'wrong metadata identity' = { param($f) $f.Account.id = 'another' }
    'missing metadata identity' = { param($f) $f.Account.id = '' }
    'unexpected port' = { param($f) $f.Endpoint = 'https://perf.documents.azure.com:8443/' }
    'endpoint credentials' = { param($f) $f.Endpoint = 'https://user@perf.documents.azure.com/' }
    'endpoint query' = { param($f) $f.Endpoint = 'https://perf.documents.azure.com/?query=1' }
    'endpoint fragment' = { param($f) $f.Endpoint = 'https://perf.documents.azure.com/#fragment' }
    'endpoint path' = { param($f) $f.Endpoint = 'https://perf.documents.azure.com/dbs' }
}
foreach ($name in $cases.Keys) {
    $fixture = New-Fixture
    & $cases[$name] $fixture
    $rejected = $false
    try { Get-CosmosPerfPlacement @fixture | Out-Null } catch { $rejected = $true }
    if (-not $rejected) { throw "Preflight accepted invalid fixture: $name" }
    Write-Host "PASS: rejects $name"
}

$files = @('prepare-cosmos-perf.ps1', 'run-cosmos-perf.ps1', 'cleanup-cosmos-perf.ps1', 'cosmos-perf-data-plane.ps1')
foreach ($file in $files) {
    $tokens = $null
    $errors = $null
    [System.Management.Automation.Language.Parser]::ParseFile(
        ([System.IO.Path]::Combine($PSScriptRoot, $file)), [ref]$tokens, [ref]$errors
    ) | Out-Null
    if ($errors.Count) { throw "PowerShell parse errors in ${file}: $errors" }
}
Write-Host 'PASS: all performance scripts parse'

$stageYaml = Get-Content -Raw ([System.IO.Path]::Combine($PSScriptRoot, 'perf-stage.yml'))
foreach ($mapping in @(
    'AZURE_COSMOS_ENDPOINT: $(cosmos-rust-perf-endpoint)',
    'AZURE_COSMOS_KEY: $(cosmos-rust-perf-key)'
)) {
    if ([regex]::Matches($stageYaml, [regex]::Escape($mapping)).Count -ne 3) {
        throw "Preflight, workload, and cleanup must all use the dedicated perf mapping: $mapping"
    }
}
if ($stageYaml.Contains('$(thinclient-test-')) {
    throw 'The perf stage must not consume functional-test account credentials.'
}
$ciYaml = Get-Content -Raw ([System.IO.Path]::Combine($PSScriptRoot, '..', '..', 'ci.yml'))
foreach ($mapping in @(
    'AZURE_COSMOS_GW_V2_ENDPOINT: $(thinclient-test-endpoint)',
    'AZURE_COSMOS_GW_V2_KEY: $(thinclient-test-key)'
)) {
    if (-not $ciYaml.Contains($mapping)) { throw 'Functional-test credentials must remain unchanged.' }
}
Write-Host 'PASS: dedicated perf credentials are consistent across all steps and separate from functional tests'

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

$key = 'dGVzdGtleQ=='
$date = [DateTime]::Parse('2026-10-19T22:00:00Z', [Globalization.CultureInfo]::InvariantCulture).ToUniversalTime()
$headers = New-CosmosPerfHeaders -Method get -ResourceType '' -ResourceLink '' -Key $key -Date $date
if ($headers.authorization -cne 'type%3Dmaster%26ver%3D1.0%26sig%3D38svEf2gDSQS3c2FLy0%2Bs5xj8U7eMSeCUsRA0ZJJdMs%3D' -or
    $headers['x-ms-date'] -cne 'Mon, 19 Oct 2026 22:00:00 GMT') {
    throw 'Account metadata signature or RFC 7231 date does not match the independent fixture.'
}
$database = 'cosmos-perf-12345678-1234-1234-1234-123456789abc'
$headers = New-CosmosPerfHeaders -Method delete -ResourceType dbs -ResourceLink "dbs/$database" -Key $key -Date $date
if ($headers.authorization -cne 'type%3Dmaster%26ver%3D1.0%26sig%3DwaXAgU44rymsJn2rLHBa4YPiMosgfDYAebkT4NvAyRo%3D') {
    throw 'Database cleanup signature does not match the independent fixture.'
}
Write-Host 'PASS: account reads and database cleanup use correct key-authenticated signatures'

$http = @{
    Requests = @()
    Response = @{ StatusCode = 200; Content = ((New-Fixture).Account | ConvertTo-Json -Depth 5) }
}
$mockRequest = {
    param($Method, [Uri]$Uri, $Headers, $TimeoutSec, $MaximumRedirection, [switch]$NoProxy, [switch]$SkipHttpErrorCheck)
    if ($MaximumRedirection -ne 0 -or -not $NoProxy -or -not $SkipHttpErrorCheck -or $TimeoutSec -le 0) {
        throw 'Data-plane requests must be direct, bounded, and must not forward credentials through redirects.'
    }
    $http.Requests += @{ Method = $Method; Uri = $Uri; Headers = $Headers }
    return [PSCustomObject]$http.Response
}.GetNewClosure()
Set-Item Function:\Invoke-WebRequest $mockRequest
$mockMetadata = {
    param([Uri]$Uri, $Headers, $TimeoutSec, [switch]$NoProxy)
    if ($Uri.Host -ne '169.254.169.254' -or $Headers.Metadata -ne 'true' -or -not $NoProxy -or $TimeoutSec -le 0) {
        throw 'Expected a bounded direct IMDS query.'
    }
    $http.MetadataCalls++
    return [PSCustomObject]$http.Vm
}.GetNewClosure()
Set-Item Function:\Invoke-RestMethod $mockMetadata
$temporary = [System.IO.Path]::Combine([System.IO.Path]::GetTempPath(), "cosmos-perf-script-$([Guid]::NewGuid())")
$savedEnvironment = @{}
foreach ($name in @('COSMOS_PERF_OUTPUT', 'AZURE_COSMOS_ENDPOINT', 'AZURE_COSMOS_KEY', 'BUILD_BUILDID',
    'COSMOS_PERF_BASELINE_COMMIT', 'COSMOS_PERF_CALIBRATE', 'COSMOS_PERF_WEEKLY', 'BUILD_SOURCEVERSION', 'BUILD_SOURCEBRANCH')) {
    $savedEnvironment[$name] = [Environment]::GetEnvironmentVariable($name)
}
try {
    $account = Get-CosmosPerfAccount 'https://perf.documents.azure.com/' $key
    if ($account.id -ne 'perf' -or $http.Requests.Count -ne 1 -or
        $http.Requests[0].Method -ne 'Get' -or $http.Requests[0].Uri.AbsolutePath -ne '/' -or
        $http.Requests[0].Headers['x-ms-cosmos-use-thinclient'] -ne 'true') {
        throw 'Preflight did not read account metadata through the authenticated root endpoint.'
    }
    foreach ($status in @(301, 401, 403, 429, 500)) {
        $http.Response.StatusCode = $status
        $rejected = $false
        try { Get-CosmosPerfAccount 'https://perf.documents.azure.com/' $key | Out-Null } catch { $rejected = $true }
        if (-not $rejected) { throw "Metadata HTTP $status was accepted." }
    }
    $http.Response.StatusCode = 200
    $http.Response.Content = '{invalid json'
    $rejected = $false
    try { Get-CosmosPerfAccount 'https://perf.documents.azure.com/' $key | Out-Null } catch { $rejected = $true }
    if (-not $rejected) { throw 'Invalid metadata JSON was accepted.' }
    Write-Host 'PASS: metadata errors and redirects fail instead of providing success-shaped defaults'

    New-Item -ItemType Directory -Path $temporary | Out-Null
    $env:COSMOS_PERF_OUTPUT = $temporary
    $env:AZURE_COSMOS_ENDPOINT = 'https://perf.documents.azure.com/'
    $env:AZURE_COSMOS_KEY = $key
    $env:BUILD_BUILDID = 'owned-run'
    $manifest = [System.IO.Path]::Combine($temporary, 'owned-database.json')
    @{ database = $database; workload_id = 'owned-run' } | ConvertTo-Json | Set-Content $manifest
    $http.Requests = @()
    $http.Response = @{ StatusCode = 204; Content = '' }
    & ([System.IO.Path]::Combine($PSScriptRoot, 'cleanup-cosmos-perf.ps1'))
    if ($http.Requests.Count -ne 1 -or $http.Requests[0].Method -ne 'Delete' -or
        $http.Requests[0].Uri.AbsolutePath -ne "/dbs/$database") {
        throw 'Cleanup did not target exactly the owned database.'
    }
    foreach ($ownership in @(
        @{ database = $database; workload_id = 'another-run' },
        @{ database = 'shared-test-db'; workload_id = 'owned-run' }
    )) {
        $ownership | ConvertTo-Json | Set-Content $manifest
        $http.Requests = @()
        $rejected = $false
        try { & ([System.IO.Path]::Combine($PSScriptRoot, 'cleanup-cosmos-perf.ps1')) } catch { $rejected = $true }
        if (-not $rejected -or $http.Requests.Count -ne 0) { throw 'Cleanup accepted unowned resources.' }
    }
    Write-Host 'PASS: cleanup deletes only this run database, never the account or other databases'

    $env:COSMOS_PERF_BASELINE_COMMIT = '$(cosmos-perf-baseline-commit)'
    $env:COSMOS_PERF_CALIBRATE = 'True'
    $env:COSMOS_PERF_WEEKLY = 'False'
    $env:BUILD_SOURCEVERSION = (& git rev-parse HEAD)
    if ($LASTEXITCODE -ne 0) { throw 'Cannot resolve the test checkout commit.' }
    $sourceProfileJson = & git show "$($env:BUILD_SOURCEVERSION):sdk/cosmos/eng/pipelines/perf-profile.json"
    if ($LASTEXITCODE -ne 0) { throw 'Cannot resolve the source workload profile.' }
    $sourceProfile = ($sourceProfileJson -join "`n") | ConvertFrom-Json
    $env:BUILD_SOURCEBRANCH = 'refs/pull/5469/head'
    $http.Requests = @()
    $multiRegion = (New-Fixture).Account
    $multiRegion.readableLocations += @{ name = 'West US 3' }
    $multiRegion.writableLocations = @(@{ name = 'West US 3' }, @{ name = 'East US 2' })
    $http.Response = @{ StatusCode = 200; Content = ($multiRegion | ConvertTo-Json -Depth 5) }
    $http.Vm = (New-Fixture).Vm
    $http.Vm.vmSize = $sourceProfile.vm_size
    $http.MetadataCalls = 0
    # PowerShell@2 dot-sources filePath scripts; this must execute preflight, not only load functions.
    . ([System.IO.Path]::Combine($PSScriptRoot, 'prepare-cosmos-perf.ps1'))
    $recordedEnvironment = Get-Content -Raw ([System.IO.Path]::Combine($temporary, 'environment.json')) | ConvertFrom-Json
    $recordedMode = Get-Content -Raw ([System.IO.Path]::Combine($temporary, 'execution-mode.json')) | ConvertFrom-Json
    if ($http.Requests.Count -ne 1 -or $http.MetadataCalls -ne 1 -or
        $recordedEnvironment.account_name -ne 'perf' -or $recordedEnvironment.region -ne 'East US 2' -or
        $recordedEnvironment.source_commit -ne $env:BUILD_SOURCEVERSION -or
        ($recordedEnvironment.excluded_regions -join ',') -ne 'West US 3' -or
        $recordedEnvironment.PSObject.Properties.Name -contains 'account_resource_id' -or
        $recordedMode.calibrate -ne $true) {
        throw 'Pipeline-style invocation did not execute and persist verified preflight.'
    }
    Write-Host 'PASS: PowerShell task-style dot-sourcing executes preflight and writes required manifests'
} finally {
    Remove-Item Function:\Invoke-WebRequest
    Remove-Item Function:\Invoke-RestMethod
    foreach ($name in $savedEnvironment.Keys) {
        [Environment]::SetEnvironmentVariable($name, $savedEnvironment[$name])
    }
    if (Test-Path -LiteralPath $temporary) { Remove-Item -LiteralPath $temporary -Recurse }
}
