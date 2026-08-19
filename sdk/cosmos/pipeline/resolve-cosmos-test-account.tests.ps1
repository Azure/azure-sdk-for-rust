# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
#
# Local tests for resolve-cosmos-test-account.ps1 (no ADO required).
# Run: pwsh sdk/cosmos/pipeline/resolve-cosmos-test-account.tests.ps1
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$here = $PSScriptRoot
$script = [System.IO.Path]::Combine($here, 'resolve-cosmos-test-account.ps1')
$samplePath = [System.IO.Path]::Combine($here, 'live-test-accounts.sample.json')
$sampleJson = Get-Content -Raw -Path $samplePath

$script:pass = 0
$script:fail = 0

function Test-Ok([string]$name) {
    Write-Host "  PASS: $name" -ForegroundColor Green
    $script:pass++
}

function Test-Fail([string]$name, [string]$detail = '') {
    Write-Host "  FAIL: $name $detail" -ForegroundColor Red
    $script:fail++
}

function Invoke-Resolver([string]$selector, [string]$json) {
    $env:COSMOS_ACCOUNTS_LOCAL = 'true'
    $env:COSMOS_ACCOUNT_SELECTOR = $selector
    $env:COSMOS_TEST_ACCOUNTS_JSON = $json
    $output = & pwsh -NoProfile -File $script 2>$null
    $rc = $LASTEXITCODE
    Remove-Item Env:\COSMOS_ACCOUNTS_LOCAL, Env:\COSMOS_ACCOUNT_SELECTOR, Env:\COSMOS_TEST_ACCOUNTS_JSON -ErrorAction SilentlyContinue
    return @{ Output = ($output -join "`n"); ExitCode = $rc }
}

Write-Host "Test 1: resolves a valid selector and exports connection string + rustflags"
$result = Invoke-Resolver 'session-multiwrite' $sampleJson
if ($result.ExitCode -eq 0 -and
    $result.Output -match 'AZURE_COSMOS_CONNECTION_STRING=AccountEndpoint=https://REPLACE-session-multiwrite' -and
    $result.Output -match 'COSMOS_RUSTFLAGS=--cfg=test_category="multi_write"' -and
    $result.Output -match 'AZURE_COSMOS_DEFAULT_CONSISTENCY=Session') {
    Test-Ok "resolved connection string + rustflags + consistency"
}
else {
    Test-Fail "resolve valid selector" "(rc=$($result.ExitCode)): $($result.Output)"
}

Write-Host "Test 2: exports AZURE_COSMOS_SECONDARY_KEY when present"
$result = Invoke-Resolver 'session-split' $sampleJson
if ($result.ExitCode -eq 0 -and $result.Output -match 'AZURE_COSMOS_SECONDARY_KEY=REPLACE_SECONDARY_KEY') {
    Test-Ok "secondary key exported"
}
else {
    Test-Fail "secondary key" "(rc=$($result.ExitCode)): $($result.Output)"
}

Write-Host "Test 3: unknown selector fails with non-zero"
$result = Invoke-Resolver 'does-not-exist' $sampleJson
if ($result.ExitCode -ne 0) { Test-Ok "unknown selector rejected" } else { Test-Fail "unknown selector should fail" $result.Output }

Write-Host "Test 4: invalid JSON fails"
$result = Invoke-Resolver 'session-singlewrite' '{not json'
if ($result.ExitCode -ne 0) { Test-Ok "invalid json rejected" } else { Test-Fail "invalid json should fail" }

Write-Host "Test 5: unsupported version fails"
$result = Invoke-Resolver 'session-singlewrite' '{"version":99,"accounts":{"session-singlewrite":{"endpoint":"https://x","key":"k","consistency":"Session","testCategory":"emulator"}}}'
if ($result.ExitCode -ne 0) { Test-Ok "bad version rejected" } else { Test-Fail "bad version should fail" }

Write-Host "Test 6: missing key fails"
$result = Invoke-Resolver 'x' '{"version":1,"accounts":{"x":{"endpoint":"https://x","consistency":"Session","testCategory":"emulator"}}}'
if ($result.ExitCode -ne 0) { Test-Ok "missing key rejected" } else { Test-Fail "missing key should fail" }

Write-Host "Test 7: non-https endpoint fails"
$result = Invoke-Resolver 'x' '{"version":1,"accounts":{"x":{"endpoint":"http://x","key":"k","consistency":"Session","testCategory":"emulator"}}}'
if ($result.ExitCode -ne 0) { Test-Ok "non-https rejected" } else { Test-Fail "non-https should fail" }

Write-Host "Test 8: empty selector fails"
$result = Invoke-Resolver '' $sampleJson
if ($result.ExitCode -ne 0) { Test-Ok "empty selector rejected" } else { Test-Fail "empty selector should fail" }

Write-Host "Test 9: missing testCategory fails"
$result = Invoke-Resolver 'x' '{"version":1,"accounts":{"x":{"endpoint":"https://x","key":"k","consistency":"Session"}}}'
if ($result.ExitCode -ne 0) { Test-Ok "missing testCategory rejected" } else { Test-Fail "missing testCategory should fail" }

Write-Host ""
Write-Host "Results: $script:pass passed, $script:fail failed"
if ($script:fail -ne 0) { exit 1 }
exit 0
