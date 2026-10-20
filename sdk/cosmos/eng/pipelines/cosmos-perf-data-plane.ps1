# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

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
