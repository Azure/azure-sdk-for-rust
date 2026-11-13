# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

function Initialize-CosmosPerfCredentials {
    try {
        $config = $env:COSMOS_TEST_ACCOUNTS_JSON | ConvertFrom-Json -AsHashtable -ErrorAction Stop
    } catch {
        throw 'rust-ci must contain valid JSON; credential contents are omitted.'
    }
    if ($config -isnot [System.Collections.IDictionary] -or $config.version -ne 1 -or
        $config.accounts -isnot [System.Collections.IDictionary] -or
        $config.accounts['perf-gateway-v2'] -isnot [System.Collections.IDictionary]) {
        throw 'rust-ci version 1 must contain accounts.perf-gateway-v2; no functional-account fallback is allowed.'
    }
    $account = $config.accounts['perf-gateway-v2']
    if ($account.endpoint -isnot [string] -or $account.key -isnot [string] -or
        [string]::IsNullOrWhiteSpace($account.key) -or $account.key -notmatch '^[A-Za-z0-9+/]+={0,2}$') {
        throw 'rust-ci accounts.perf-gateway-v2 requires an HTTPS endpoint and a base64 account key.'
    }
    Get-CosmosPerfEndpoint $account.endpoint | Out-Null
    try {
        [Convert]::FromBase64String($account.key) | Out-Null
    } catch {
        throw 'rust-ci accounts.perf-gateway-v2 has an invalid base64 account key.'
    }
    # Mask the extracted key separately: ADO does not mask substrings of JSON secrets.
    if ($env:TF_BUILD -eq 'True') { Write-Host "##vso[task.setsecret]$($account.key)" }
    $env:AZURE_COSMOS_ENDPOINT = $account.endpoint
    $env:AZURE_COSMOS_KEY = $account.key
    $baseline = $config['perfBaselineCommit']
    if ($null -ne $baseline -and ($baseline -isnot [string] -or $baseline -notmatch '^[a-fA-F0-9]{40}$')) {
        throw 'rust-ci perfBaselineCommit must be a full reviewed commit SHA when present.'
    }
    $env:COSMOS_PERF_BASELINE_COMMIT = $baseline
}

function Get-CosmosPerfEndpoint([string] $Endpoint) {
    $uri = $null
    if (-not [Uri]::TryCreate($Endpoint, [UriKind]::Absolute, [ref]$uri) -or
        $uri.Scheme -ne 'https' -or $uri.Port -ne 443 -or
        $uri.Host -notmatch '^[a-z0-9-]+\.documents\.azure\.com$' -or
        $uri.AbsolutePath -ne '/' -or $uri.Query -or $uri.Fragment -or $uri.UserInfo) {
        throw 'Perf requires a public Azure Cosmos account HTTPS endpoint without a path, query, or credentials.'
    }
    return $uri
}

function New-CosmosPerfHeaders(
    [ValidateSet('get', 'delete')][string] $Method,
    [AllowEmptyString()][ValidateSet('', 'dbs')][string] $ResourceType,
    [AllowEmptyString()][string] $ResourceLink,
    [string] $Key,
    [DateTime] $Date = [DateTime]::UtcNow
) {
    $dateText = $Date.ToUniversalTime().ToString('r', [Globalization.CultureInfo]::InvariantCulture)
    $text = "$($Method.ToLowerInvariant())`n$ResourceType`n$ResourceLink`n$($dateText.ToLowerInvariant())`n`n"
    $hmac = [Security.Cryptography.HMACSHA256]::new([Convert]::FromBase64String($Key))
    try {
        $signature = [Convert]::ToBase64String($hmac.ComputeHash([Text.Encoding]::UTF8.GetBytes($text)))
    } finally {
        $hmac.Dispose()
    }
    return @{
        authorization = [Uri]::EscapeDataString("type=master&ver=1.0&sig=$signature")
        'x-ms-date' = $dateText
        'x-ms-version' = '2018-12-31'
    }
}

function Get-CosmosPerfAccount([string] $Endpoint, [string] $Key) {
    $uri = Get-CosmosPerfEndpoint $Endpoint
    $headers = New-CosmosPerfHeaders -Method get -ResourceType '' -ResourceLink '' -Key $Key
    $headers['x-ms-cosmos-use-thinclient'] = 'true'
    $response = Invoke-WebRequest -Method Get -Uri $uri -Headers $headers `
        -TimeoutSec 30 -MaximumRedirection 0 -NoProxy -SkipHttpErrorCheck
    if ($response.StatusCode -ne 200) {
        throw "Cosmos account metadata request failed with HTTP $($response.StatusCode)."
    }
    return ($response.Content | ConvertFrom-Json -ErrorAction Stop)
}
