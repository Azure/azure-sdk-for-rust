# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

param([switch] $FunctionsOnly)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
. ([System.IO.Path]::Combine($PSScriptRoot, 'cosmos-perf-data-plane.ps1'))

function ConvertTo-CanonicalRegion([string] $Region) {
    return ($Region -replace '\s', '').ToLowerInvariant()
}

function Assert-CosmosPerfSourceRef([string] $SourceRef) {
    if ($SourceRef -like 'refs/pull/*' -and $SourceRef -notmatch '^refs/pull/[0-9]+/head$') {
        throw 'Queue PR validation on its exact head commit, not a synthetic pull-request merge ref.'
    }
}

function Get-CosmosPerfCalibration([bool] $Weekly, [bool] $Requested, [bool] $HasBaseline, $Thresholds) {
    if ($Requested) { return $true }
    if (-not $HasBaseline) {
        if ($Weekly) { return $true }
        throw 'A reviewed baseline commit is required for manual performance validation.'
    }
    if ($Thresholds.version -ne 1 -or $Thresholds.approved -isnot [bool]) {
        throw 'Invalid performance threshold document version or approval flag.'
    }
    if ($Thresholds.approved) { return $false }
    if ($Weekly -and $null -eq $Thresholds.profile -and
        $Thresholds.thresholds -is [array] -and $Thresholds.thresholds.Count -eq 0) {
        return $true
    }
    throw 'Performance thresholds are not approved. Use explicit manual calibration or the initial empty baseline.'
}

function Get-CosmosPerfPlacement($Account, $Vm, $Profile, [string] $Endpoint) {
    $uri = Get-CosmosPerfEndpoint $Endpoint
    if ([string]::IsNullOrWhiteSpace($Account.id) -or $uri.Host -ne "$($Account.id).documents.azure.com") {
        throw 'Fixed-account endpoint does not match the authenticated account metadata.'
    }
    if ($Account.readableLocations -isnot [array] -or $Account.writableLocations -isnot [array] -or
        $Account.readableLocations.Count -eq 0 -or $Account.writableLocations.Count -eq 0) {
        throw 'Account metadata must contain readable and writable regions.'
    }
    $region = ConvertTo-CanonicalRegion $Vm.location
    $regions = @{}
    foreach ($location in @($Account.readableLocations) + @($Account.writableLocations)) {
        if ([string]::IsNullOrWhiteSpace($location.name)) { throw 'Account metadata contains an empty region name.' }
        $regions[(ConvertTo-CanonicalRegion $location.name)] = $location.name
    }
    $reads = @($Account.readableLocations | Where-Object { (ConvertTo-CanonicalRegion $_.name) -eq $region })
    $writes = @($Account.writableLocations | Where-Object { (ConvertTo-CanonicalRegion $_.name) -eq $region })
    if ([string]::IsNullOrWhiteSpace($region) -or $reads.Count -eq 0 -or $writes.Count -eq 0) {
        throw "Agent region '$($Vm.location)' must be both readable and writable. Account reads: $($Account.readableLocations.name -join ', '); writes: $($Account.writableLocations.name -join ', ')."
    }
    if ($Vm.vmSize -ne $Profile.vm_size -or $Vm.osType -ne 'Linux') {
        throw "Agent SKU '$($Vm.vmSize)' / OS '$($Vm.osType)' does not match profile SKU '$($Profile.vm_size)' / Linux."
    }
    if ($Account.userConsistencyPolicy.defaultConsistencyLevel -ne $Profile.consistency) {
        throw 'Account consistency differs from the performance profile.'
    }
    return [PSCustomObject]@{
        region = $writes[0].name
        excluded_regions = [string[]]@($regions.Keys | Where-Object { $_ -ne $region } | ForEach-Object { $regions[$_] } | Sort-Object)
    }
}

if ($FunctionsOnly) { return }

foreach ($name in @('COSMOS_PERF_OUTPUT',
    'AZURE_COSMOS_ENDPOINT', 'AZURE_COSMOS_KEY', 'BUILD_SOURCEVERSION')) {
    $value = [Environment]::GetEnvironmentVariable($name)
    if (-not $value -or $value.StartsWith('$(')) { throw "Missing required configuration: $name" }
}
$output = $env:COSMOS_PERF_OUTPUT
New-Item -ItemType Directory -Force -Path $output | Out-Null
$baseline = $env:COSMOS_PERF_BASELINE_COMMIT
$source = $env:BUILD_SOURCEVERSION
$weekly = $env:COSMOS_PERF_WEEKLY -eq 'True'
$requested = $env:COSMOS_PERF_CALIBRATE -eq 'True'
$hasBaseline = -not [string]::IsNullOrWhiteSpace($baseline) -and -not $baseline.StartsWith('$(')
if (-not $hasBaseline) {
    if (-not ($weekly -or $requested)) {
        throw 'COSMOS_PERF_BASELINE_COMMIT is required for manual performance validation.'
    }
    $baseline = $source
}
if ($baseline -notmatch '^[a-fA-F0-9]{40}$' -or $source -notmatch '^[a-fA-F0-9]{40}$') {
    throw 'Source and reviewed baseline must be full commit SHAs.'
}
$head = & git rev-parse HEAD
if ($LASTEXITCODE -ne 0 -or $head.Trim() -ne $source) { throw 'Checkout does not match Build.SourceVersion.' }
Assert-CosmosPerfSourceRef $env:BUILD_SOURCEBRANCH
foreach ($file in @('perf-profile.json', 'perf-thresholds.json')) {
    $content = & git show "${baseline}:sdk/cosmos/eng/pipelines/$file"
    if ($LASTEXITCODE -ne 0) { throw "Cannot read reviewed $file from the pinned baseline commit." }
    $content | Set-Content -Encoding utf8 -Path ([System.IO.Path]::Combine($output, $file))
}
$profile = Get-Content -Raw ([System.IO.Path]::Combine($output, 'perf-profile.json')) | ConvertFrom-Json
$thresholds = Get-Content -Raw ([System.IO.Path]::Combine($output, 'perf-thresholds.json')) | ConvertFrom-Json
if ($profile.duration_seconds -ne 2700 -or $profile.warmup_seconds -ne 300) {
    throw 'Weekly runs require exactly 2700 seconds with 300 seconds of discarded warm-up.'
}
$calibrate = Get-CosmosPerfCalibration $weekly $requested $hasBaseline $thresholds
if ($calibrate) {
    $reason = if (-not $hasBaseline) {
        'No reviewed baseline commit is configured; collecting calibration using the source profile.'
    } elseif ($requested) {
        'Manual calibration requested.'
    } else {
        'The reviewed baseline is the initial uncalibrated template; collecting calibration.'
    }
    Write-Host "##vso[task.logissue type=warning;]$reason No performance-gate pass will be reported."
}
@{ version = 1; calibrate = $calibrate } | ConvertTo-Json |
    Set-Content -Encoding utf8 ([System.IO.Path]::Combine($output, 'execution-mode.json'))
$account = Get-CosmosPerfAccount $env:AZURE_COSMOS_ENDPOINT $env:AZURE_COSMOS_KEY
$vm = Invoke-RestMethod -Uri 'http://169.254.169.254/metadata/instance/compute?api-version=2021-02-01' `
    -Headers @{ Metadata = 'true' } -NoProxy -TimeoutSec 10
$placement = Get-CosmosPerfPlacement $account $vm $profile $env:AZURE_COSMOS_ENDPOINT
@{
    source_commit = $source
    baseline_commit = $baseline
    account_name = $account.id
    endpoint = $env:AZURE_COSMOS_ENDPOINT
    region = $placement.region
    excluded_regions = $placement.excluded_regions
    vm_size = $vm.vmSize
    consistency = $account.userConsistencyPolicy.defaultConsistencyLevel
} | ConvertTo-Json | Set-Content -Encoding utf8 ([System.IO.Path]::Combine($output, 'environment.json'))
Write-Host "Verified source SHA and baseline policy. Perf region: $($placement.region); VM SKU: $($vm.vmSize); excluded regions: $($placement.excluded_regions -join ', ')."
