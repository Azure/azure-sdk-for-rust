# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# Azure Cosmos DB refuses `enableAllVersionsAndDeletesChangeFeed` while an account is
# being created, so test-resources.bicep can only provision the continuous backup
# prerequisite. This script turns the account level feature on afterwards and waits for
# the account to settle. Without it, creating a container with a full fidelity
# `changeFeedPolicy` fails with HTTP 400 and the AllVersionsAndDeletes change feed tests
# cannot pass.
#
# See https://learn.microsoft.com/azure/cosmos-db/change-feed-modes
#
# Deliberately declared as a simple (non-advanced) script: New-TestResources.ps1 splats
# its own bound parameters onto this script, and unknown named arguments only land
# harmlessly in $args when the script has no [Parameter()] attributes.

param (
    [hashtable] $DeploymentOutputs
)

$ErrorActionPreference = 'Stop'

if ($DeploymentOutputs['ENABLE_ALL_VERSIONS_AND_DELETES_CHANGE_FEED'] -ne 'true') {
    Write-Host 'All versions and deletes change feed mode was not requested for this account. Nothing to do.'
    return
}

$accountId = $DeploymentOutputs['COSMOS_ACCOUNT_RESOURCE_ID']
if (!$accountId) {
    throw "Deployment output 'COSMOS_ACCOUNT_RESOURCE_ID' is missing. Make sure sdk/cosmos/test-resources.bicep still emits it."
}

# `enableAllVersionsAndDeletesChangeFeed` is only exposed by preview API versions.
$apiVersion = '2025-05-01-preview'
$accountUri = "${accountId}?api-version=$apiVersion"
$pollIntervalSeconds = 30
# Enabling the feature can take up to 30 minutes, during which no other account changes
# are accepted. Leave headroom, but still fail well inside the live test job timeout.
$timeout = [timespan]::FromMinutes(40)

function Get-CosmosDbAccount {
    $response = Invoke-AzRestMethod -Method GET -Path $accountUri
    if ($response.StatusCode -ne 200) {
        throw "GET $accountUri returned HTTP $($response.StatusCode): $($response.Content)"
    }

    return $response.Content | ConvertFrom-Json
}

function Wait-CosmosDbAccount([string] $activity, [scriptblock] $isDone) {
    $deadline = (Get-Date).Add($timeout)
    while ($true) {
        $account = Get-CosmosDbAccount
        if (& $isDone $account) {
            return $account
        }

        if ((Get-Date) -gt $deadline) {
            throw "Timed out after $($timeout.TotalMinutes) minutes waiting for $activity. Last provisioning state: '$($account.properties.provisioningState)'."
        }

        Write-Host "  Waiting for $activity. Provisioning state is '$($account.properties.provisioningState)'; checking again in $pollIntervalSeconds seconds."
        Start-Sleep -Seconds $pollIntervalSeconds
    }
}

Write-Host "Preparing '$accountId' for all versions and deletes change feed mode."

$account = Wait-CosmosDbAccount 'the account to finish provisioning' {
    param($a) $a.properties.provisioningState -eq 'Succeeded'
}

if ($account.properties.enableAllVersionsAndDeletesChangeFeed) {
    Write-Host 'All versions and deletes change feed mode is already enabled.'
    return
}

$payload = @{ properties = @{ enableAllVersionsAndDeletesChangeFeed = $true } } | ConvertTo-Json -Depth 3 -Compress
Write-Host 'Enabling all versions and deletes change feed mode. This can take up to 30 minutes.'

$response = Invoke-AzRestMethod -Method PATCH -Path $accountUri -Payload $payload
if ($response.StatusCode -notin 200, 201, 202) {
    throw "PATCH $accountUri returned HTTP $($response.StatusCode): $($response.Content)"
}

Wait-CosmosDbAccount 'all versions and deletes change feed mode to be enabled' {
    param($a) ($a.properties.provisioningState -eq 'Succeeded') -and $a.properties.enableAllVersionsAndDeletesChangeFeed
} | Out-Null

Write-Host 'All versions and deletes change feed mode is enabled.'

# The feature lights up in the control plane slightly ahead of the data plane, so give
# the account a moment before the tests start creating full fidelity containers.
Start-Sleep -Seconds 60
