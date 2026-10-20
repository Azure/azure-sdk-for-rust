# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

<#
.SYNOPSIS
    Deletes stale per-run test databases left behind on the fixed Cosmos live-test
    accounts (resource group 'sdk-ci') by cancelled or crashed test runs.

.DESCRIPTION
    The live tests create a per-run database named 'test-db-<run-id>' (see
    azure_data_cosmos_driver/tests/framework/test_client.rs) and delete it at the end
    of the run. That in-process cleanup never runs if the agent is cancelled or the
    process crashes, so with fixed accounts (no resource-group teardown after each run,
    unlike the old per-run ARM-deployed accounts) these databases can otherwise
    accumulate indefinitely. This script deletes any database matching
    -DatabaseNamePrefix that is older than -MaxAgeHours on every Cosmos account in
    -ResourceGroupName whose name starts with -AccountNamePrefix.

    Age is read from each database's Cosmos resource '_ts' (server-assigned creation/
    last-modified timestamp, seconds since epoch), fetched via the ARM data-plane
    proxy (Invoke-AzRestMethod) since Az.CosmosDB does not surface it directly.

    Uses the Az PowerShell modules (Az.Accounts, Az.CosmosDB).

.PARAMETER SubscriptionId
    Subscription hosting the resource group. Must be the same permanent, self-owned
    subscription used by New-CosmosLiveTestAccounts.ps1 - NOT the ephemeral test tenant.

.PARAMETER ResourceGroupName
    Resource group containing the fixed Cosmos accounts. Defaults to 'sdk-ci'.

.PARAMETER AccountNamePrefix
    Only accounts whose name starts with "<prefix>-" are scanned. Defaults to 'sdkci'.

.PARAMETER DatabaseNamePrefix
    Only databases whose name starts with this prefix are considered for deletion.
    Defaults to 'test-db-' (see azure_data_cosmos_driver/tests/framework/test_client.rs).

.PARAMETER MaxAgeHours
    Minimum age, in hours, before a matching database is considered stale and deleted.
    Defaults to 6, comfortably longer than the ~90-minute live-test job timeout, to
    tolerate clock skew and slow-finishing runs without deleting an in-progress test's
    database.

.EXAMPLE
    # Dry run - lists what would be deleted, deletes nothing
    ./Remove-StaleCosmosTestDatabases.ps1 -SubscriptionId <sub> -WhatIf

.EXAMPLE
    # Delete test-db-* databases older than 6 hours on every sdkci-* account
    ./Remove-StaleCosmosTestDatabases.ps1 -SubscriptionId <sub>

.NOTES
    Requires: PowerShell 7+, Az modules, and Contributor on the subscription.
    Safe to run repeatedly; only ever deletes databases matching both the name prefix
    and the age threshold.
#>
[CmdletBinding(SupportsShouldProcess = $true)]
param(
    [Parameter(Mandatory = $true)]
    [string] $SubscriptionId,

    [string] $ResourceGroupName = 'sdk-ci',

    [ValidatePattern('^[a-z0-9]{1,10}$')]
    [string] $AccountNamePrefix = 'sdkci',

    [string] $DatabaseNamePrefix = 'test-db-',

    [ValidateRange(1, [int]::MaxValue)]
    [int] $MaxAgeHours = 6
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

function Write-Info([string]$msg) { Write-Host "==> $msg" -ForegroundColor Cyan }

foreach ($m in @('Az.Accounts', 'Az.CosmosDB')) {
    if (-not (Get-Module -ListAvailable -Name $m)) {
        throw "Required module '$m' is not installed. Install with: Install-Module $m -Scope CurrentUser"
    }
}

Write-Info "Selecting subscription $SubscriptionId"
$null = Set-AzContext -Subscription $SubscriptionId

$armApiVersion = '2024-05-15'
$cutoffUnix = [DateTimeOffset]::UtcNow.AddHours(-$MaxAgeHours).ToUnixTimeSeconds()

$accounts = @(Get-AzCosmosDBAccount -ResourceGroupName $ResourceGroupName |
        Where-Object { $_.Name -like "$AccountNamePrefix-*" })
if ($accounts.Count -eq 0) {
    Write-Info "No accounts matching '$AccountNamePrefix-*' found in resource group '$ResourceGroupName'."
    return
}

$deletedCount = 0
$scannedCount = 0

foreach ($account in $accounts) {
    $accountName = $account.Name
    $listUri = "https://management.azure.com/subscriptions/$SubscriptionId/resourceGroups/$ResourceGroupName" +
    "/providers/Microsoft.DocumentDB/databaseAccounts/$accountName/sqlDatabases?api-version=$armApiVersion"
    $response = Invoke-AzRestMethod -Method GET -Uri $listUri
    if ($response.StatusCode -ge 300) {
        Write-Warning "Failed to list databases on '$accountName' (HTTP $($response.StatusCode)): $($response.Content)"
        continue
    }
    $databases = @(($response.Content | ConvertFrom-Json).value)

    foreach ($db in $databases) {
        $dbName = $db.name
        if ($dbName -notlike "$DatabaseNamePrefix*") { continue }
        $scannedCount++

        $ts = $db.properties.resource._ts
        if (-not $ts) {
            Write-Warning "Database '$accountName/$dbName' has no '_ts'; skipping (cannot determine age)."
            continue
        }
        if ($ts -ge $cutoffUnix) { continue }

        $ageHours = [math]::Round(([DateTimeOffset]::UtcNow.ToUnixTimeSeconds() - $ts) / 3600.0, 1)
        if ($PSCmdlet.ShouldProcess("$accountName/$dbName", "Delete stale test database (age ${ageHours}h)")) {
            $deleteUri = "https://management.azure.com/subscriptions/$SubscriptionId/resourceGroups/$ResourceGroupName" +
            "/providers/Microsoft.DocumentDB/databaseAccounts/$accountName/sqlDatabases/$dbName?api-version=$armApiVersion"
            $deleteResponse = Invoke-AzRestMethod -Method DELETE -Uri $deleteUri
            if ($deleteResponse.StatusCode -ge 300) {
                Write-Warning "Failed to delete '$accountName/$dbName' (HTTP $($deleteResponse.StatusCode)): $($deleteResponse.Content)"
                continue
            }
            Write-Info "Deleted stale database '$accountName/$dbName' (age ${ageHours}h)"
            $deletedCount++
        }
    }
}

Write-Info "Scanned $scannedCount database(s) matching '$DatabaseNamePrefix*' across $($accounts.Count) account(s); deleted $deletedCount stale database(s) (>${MaxAgeHours}h old)."
