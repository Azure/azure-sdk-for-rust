# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# IMPORTANT: Do not invoke this file directly. Please instead run
# eng/common/TestResources/New-TestResources.ps1 from the repository root.
#
# The AllVersionsAndDeletes (full-fidelity) change feed cannot be enabled at
# account-create time: the service rejects it with a BadRequest. Instead we
# deploy a continuous-backup account normally and PATCH it here, after
# deployment, to turn the feature on. This runs for every Cosmos job but is a
# no-op unless the deployment signalled AVAD via the AZURE_COSMOS_ENABLE_AVAD
# output (only the dedicated 'avad' matrix job does).

param (
    [hashtable] $DeploymentOutputs,

    # Captures any arguments from New-TestResources.ps1 not declared here so no
    # parameter binding errors occur.
    [Parameter(ValueFromRemainingArguments = $true)]
    $RemainingArguments
)

if (-not $DeploymentOutputs -or $DeploymentOutputs['AZURE_COSMOS_ENABLE_AVAD'] -ne 'true') {
    Write-Host "Skipping AllVersionsAndDeletes activation (not an AVAD deployment)."
    return
}

$resourceId = $DeploymentOutputs['AZURE_COSMOS_ACCOUNT_RESOURCE_ID']
if (-not $resourceId) {
    Write-Host "AZURE_COSMOS_ACCOUNT_RESOURCE_ID output is missing; cannot enable AVAD." -ForegroundColor Red
    exit 1
}

# Preview API version that exposes the enableAllVersionsAndDeletesChangeFeed
# account property.
$apiVersion = '2024-12-01-preview'
$payload = @{ properties = @{ enableAllVersionsAndDeletesChangeFeed = $true } } | ConvertTo-Json -Depth 5

Write-Host "Enabling AllVersionsAndDeletes change feed on $resourceId"
$response = Invoke-AzRestMethod -Method PATCH -Path "${resourceId}?api-version=$apiVersion" -Payload $payload
if ($response.StatusCode -ge 400) {
    Write-Host "PATCH to enable AllVersionsAndDeletes failed ($($response.StatusCode)): $($response.Content)" -ForegroundColor Red
    exit 1
}

# The PATCH is a long-running operation. Poll the account until the property is
# reflected and provisioning has settled so the live tests do not race the
# activation.
$deadline = (Get-Date).AddMinutes(30)
do {
    Start-Sleep -Seconds 30
    $get = Invoke-AzRestMethod -Method GET -Path "${resourceId}?api-version=$apiVersion"
    $account = $get.Content | ConvertFrom-Json
    $provisioningState = $account.properties.provisioningState
    $enabled = $account.properties.enableAllVersionsAndDeletesChangeFeed
    Write-Host "  provisioningState = $provisioningState; enableAllVersionsAndDeletesChangeFeed = $enabled"
} while ((($provisioningState -ne 'Succeeded') -or (-not $enabled)) -and ((Get-Date) -lt $deadline))

if ($provisioningState -ne 'Succeeded' -or -not $enabled) {
    Write-Host "AllVersionsAndDeletes change feed did not become active in time (provisioningState = $provisioningState, enabled = $enabled)." -ForegroundColor Red
    exit 1
}

Write-Host "AllVersionsAndDeletes change feed is enabled and active."
