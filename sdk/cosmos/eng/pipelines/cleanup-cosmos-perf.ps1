# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
. ([System.IO.Path]::Combine($PSScriptRoot, 'cosmos-perf-data-plane.ps1'))
$manifest = [System.IO.Path]::Combine($env:COSMOS_PERF_OUTPUT, 'owned-database.json')
if (-not (Test-Path $manifest)) {
    Write-Host 'No owned database manifest; no cleanup required.'
    exit 0
}
$owned = Get-Content -Raw $manifest | ConvertFrom-Json
if ($owned.database -notmatch '^cosmos-perf-[a-f0-9-]{36}$' -or
    $owned.workload_id -ne $env:BUILD_BUILDID) {
    throw 'Refusing cleanup of a database not owned by this pipeline run.'
}
$endpoint = Get-CosmosPerfEndpoint $env:AZURE_COSMOS_ENDPOINT
$resource = "dbs/$($owned.database)"
$headers = New-CosmosPerfHeaders -Method delete -ResourceType dbs -ResourceLink $resource -Key $env:AZURE_COSMOS_KEY
$response = Invoke-WebRequest -Method Delete `
    -Uri ([Uri]::new($endpoint, $resource)) `
    -Headers $headers -TimeoutSec 60 -MaximumRedirection 0 -NoProxy -SkipHttpErrorCheck
if ($response.StatusCode -notin @(204, 404)) {
    throw "Owned database cleanup failed with HTTP $($response.StatusCode)."
}
Write-Host 'Owned benchmark database removed (or already removed by the perf crate).'
