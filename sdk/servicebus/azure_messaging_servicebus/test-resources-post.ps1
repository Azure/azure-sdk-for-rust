# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# IMPORTANT: Do not invoke this file directly. Please instead run eng/common/TestResources/New-TestResources.ps1 from the repository root.

param (
  [hashtable] $AdditionalParameters = @{},
  [hashtable] $DeploymentOutputs,

  [Parameter(Mandatory = $true)]
  [ValidateNotNullOrEmpty()]
  [string] $SubscriptionId,

  [Parameter(ParameterSetName = 'Provisioner', Mandatory = $true)]
  [ValidateNotNullOrEmpty()]
  [string] $TenantId,

  [Parameter()]
  [ValidatePattern('^[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}$')]
  [string] $TestApplicationId,

  [Parameter(Mandatory = $true)]
  [ValidateNotNullOrEmpty()]
  [string] $Environment,

  # Captures any arguments from eng/New-TestResources.ps1 not declared here (no parameter errors).
  [Parameter(ValueFromRemainingArguments = $true)]
  $RemainingArguments
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

if ($CI) {
  if (!$AdditionalParameters['deployResources']) {
    Write-Host "Skipping post-provisioning script because resources weren't deployed"
    return
  }
  az cloud set -n $Environment
  az login --federated-token $env:ARM_OIDC_TOKEN --service-principal -t $TenantId -u $TestApplicationId
  az account set --subscription $SubscriptionId
}

Write-Host "##[group]Service Bus Post-Deployment Setup"

# Extract deployment outputs
$namespaceName = $DeploymentOutputs['SERVICEBUS_NAMESPACE_NAME']
$resourceGroup = $DeploymentOutputs['RESOURCE_GROUP']

Write-Host "Service Bus Namespace: $namespaceName"
Write-Host "Resource Group: $resourceGroup"

# Retrieve connection strings (these contain secrets so aren't in Bicep outputs)
Write-Host "Retrieving Service Bus connection strings..."

try {
  $connectionString = az servicebus namespace authorization-rule keys list `
    --resource-group $resourceGroup `
    --namespace-name $namespaceName `
    --name RootManageSharedAccessKey `
    --query primaryConnectionString `
    --output tsv

  $listenOnlyConnectionString = az servicebus namespace authorization-rule keys list `
    --resource-group $resourceGroup `
    --namespace-name $namespaceName `
    --name ListenOnly `
    --query primaryConnectionString `
    --output tsv

  $sendOnlyConnectionString = az servicebus namespace authorization-rule keys list `
    --resource-group $resourceGroup `
    --namespace-name $namespaceName `
    --name SendOnly `
    --query primaryConnectionString `
    --output tsv

  Write-Host "✅ Connection strings retrieved successfully"

  # Set additional outputs for the test pipeline
  if ($CI) {
    Write-Host "##vso[task.setvariable variable=SERVICEBUS_CONNECTION_STRING;issecret=true]$connectionString"
    Write-Host "##vso[task.setvariable variable=SERVICEBUS_LISTEN_ONLY_CONNECTION_STRING;issecret=true]$listenOnlyConnectionString"
    Write-Host "##vso[task.setvariable variable=SERVICEBUS_SEND_ONLY_CONNECTION_STRING;issecret=true]$sendOnlyConnectionString"
  }
}
catch {
  Write-Warning "Failed to retrieve connection strings: $($_.Exception.Message)"
}

Write-Host "##[endgroup]"

Write-Host "Service Bus post-deployment setup completed successfully."
