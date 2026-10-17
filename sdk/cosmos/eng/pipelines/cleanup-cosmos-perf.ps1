# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
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
$date = [DateTime]::UtcNow.ToString('r', [Globalization.CultureInfo]::InvariantCulture)
$resource = "dbs/$($owned.database)"
$text = "delete`ndbs`n$resource`n$($date.ToLowerInvariant())`n`n"
$hmac = [Security.Cryptography.HMACSHA256]::new([Convert]::FromBase64String($env:AZURE_COSMOS_KEY))
try {
    $signature = [Convert]::ToBase64String($hmac.ComputeHash([Text.Encoding]::UTF8.GetBytes($text)))
} finally {
    $hmac.Dispose()
}
$headers = @{
    authorization = [Uri]::EscapeDataString("type=master&ver=1.0&sig=$signature")
    'x-ms-date' = $date
    'x-ms-version' = '2018-12-31'
}
$response = Invoke-WebRequest -Method Delete `
    -Uri "$($env:AZURE_COSMOS_ENDPOINT.TrimEnd('/'))/$resource" `
    -Headers $headers -TimeoutSec 60 -SkipHttpErrorCheck
if ($response.StatusCode -notin @(204, 404)) {
    throw "Owned database cleanup failed with HTTP $($response.StatusCode)."
}
Write-Host 'Owned benchmark database removed (or already removed by the perf crate).'
