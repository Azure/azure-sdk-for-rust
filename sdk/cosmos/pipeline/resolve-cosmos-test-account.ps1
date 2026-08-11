# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
#
# Resolves a single fixed Cosmos live-test account from the one JSON secret and
# exports AZURE_COSMOS_CONNECTION_STRING (+ ACCOUNT_HOST, DATABASE_NAME,
# AZURE_COSMOS_DEFAULT_CONSISTENCY, COSMOS_RUSTFLAGS, and optionally
# AZURE_COSMOS_SECONDARY_KEY) for the tests.
#
# The Rust Cosmos live-test matrix runs on ubuntu, windows, and macOS agents, so
# this resolver is PowerShell (pwsh), not bash+jq.
#
# Inputs (environment variables, unless passed as parameters):
#   COSMOS_TEST_ACCOUNTS_JSON  Raw JSON matching live-test-accounts.schema.json
#                              (the value of the fixed-accounts ADO secret).
#   COSMOS_ACCOUNT_SELECTOR    Logical account name to select (e.g. session-multiwrite).
#   COSMOS_ACCOUNTS_LOCAL      Optional. When "true", prints KEY=VALUE to stdout instead of
#                              emitting Azure DevOps ##vso logging commands (used for local tests).
#
# Exit codes: 0 on success; non-zero on any validation failure.
[CmdletBinding()]
param(
    [string] $AccountSelector = $env:COSMOS_ACCOUNT_SELECTOR,
    [string] $AccountsJson = $env:COSMOS_TEST_ACCOUNTS_JSON,
    [switch] $Local = ($env:COSMOS_ACCOUNTS_LOCAL -eq 'true')
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

function Fail([string]$message) {
    Write-Host "ERROR: $message" -ForegroundColor Red
    exit 1
}

if ([string]::IsNullOrWhiteSpace($AccountsJson)) {
    Fail "COSMOS_TEST_ACCOUNTS_JSON is empty. Wire the fixed-accounts secret to this variable."
}
if ([string]::IsNullOrWhiteSpace($AccountSelector)) {
    Fail "COSMOS_ACCOUNT_SELECTOR is empty. Set it to a logical account name."
}

try {
    $config = $AccountsJson | ConvertFrom-Json -ErrorAction Stop
}
catch {
    Fail "COSMOS_TEST_ACCOUNTS_JSON is not valid JSON: $($_.Exception.Message)"
}

if (-not ($config.PSObject.Properties.Name -contains 'version') -or $config.version -ne 1) {
    Fail "Unsupported or missing schema version '$($config.version)' (parser supports: 1)."
}

if (-not ($config.PSObject.Properties.Name -contains 'accounts') -or
    -not ($config.accounts.PSObject.Properties.Name -contains $AccountSelector)) {
    $available = if ($config.PSObject.Properties.Name -contains 'accounts') {
        ($config.accounts.PSObject.Properties.Name -join ', ')
    } else { '<none>' }
    Fail "Account selector '$AccountSelector' not found. Available: $available"
}

$account = $config.accounts.$AccountSelector

$endpoint = [string]$account.endpoint
$key = [string]$account.key
$secondaryKey = if ($account.PSObject.Properties.Name -contains 'secondaryKey') { [string]$account.secondaryKey } else { '' }
$database = if ($account.PSObject.Properties.Name -contains 'database' -and $account.database) { [string]$account.database } else { 'shared-test-db' }
$consistency = [string]$account.consistency
$testCategory = [string]$account.testCategory

if ([string]::IsNullOrWhiteSpace($endpoint)) { Fail "Account '$AccountSelector' is missing required 'endpoint'." }
if ([string]::IsNullOrWhiteSpace($key)) { Fail "Account '$AccountSelector' is missing required 'key'." }
if (-not $endpoint.StartsWith('https://')) { Fail "Account '$AccountSelector' endpoint must start with https:// (got '$endpoint')." }
if ([string]::IsNullOrWhiteSpace($consistency)) { Fail "Account '$AccountSelector' is missing required 'consistency'." }
if ([string]::IsNullOrWhiteSpace($testCategory)) { Fail "Account '$AccountSelector' is missing required 'testCategory'." }

$connectionString = "AccountEndpoint=$endpoint;AccountKey=$key;"
$rustFlags = "--cfg=test_category=`"$testCategory`""

function Emit-Public([string]$name, [string]$value) {
    if ($Local) {
        Write-Output "$name=$value"
    }
    else {
        Write-Host "##vso[task.setvariable variable=$name;issecret=false]$value"
    }
}

# Emit a secret using the azure-sdk double-set convention (see
# eng/common/TestResources/TestResources-Helpers.ps1): register the literal value
# as a secret (variable _NAME) so the log scrubber masks it everywhere, AND set a
# plain variable NAME so it still auto-exports as an environment variable to the
# cargo test task. Marking a variable issecret=true alone would prevent env
# propagation to subsequent tasks.
function Emit-Secret([string]$name, [string]$value) {
    if ($Local) {
        Write-Output "$name=$value"
    }
    else {
        Write-Host "##vso[task.setvariable variable=_$name;issecret=true]$value"
        Write-Host "##vso[task.setvariable variable=$name;issecret=false]$value"
    }
}

# AZURE_COSMOS_CONNECTION_STRING carries the account key, so it must go through
# Emit-Secret. The remaining values are non-sensitive test configuration.
Emit-Secret 'AZURE_COSMOS_CONNECTION_STRING' $connectionString
Emit-Public 'ACCOUNT_HOST' $endpoint
Emit-Public 'DATABASE_NAME' $database
Emit-Public 'AZURE_COSMOS_DEFAULT_CONSISTENCY' $consistency
Emit-Public 'COSMOS_RUSTFLAGS' $rustFlags
if (-not [string]::IsNullOrWhiteSpace($secondaryKey)) {
    Emit-Secret 'AZURE_COSMOS_SECONDARY_KEY' $secondaryKey
}

# Masked, secret-free summary for logs.
Write-Host "Resolved Cosmos test account '$AccountSelector': endpoint=$endpoint consistency=$consistency testCategory=$testCategory key=***"
