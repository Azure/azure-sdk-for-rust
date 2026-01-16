# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# IMPORTANT: Do not invoke this file directly. Please instead run eng/common/TestResources/New-TestResources.ps1 from the repository root.

param (
  # [hashtable] $AdditionalParameters = @{},
  [hashtable] $DeploymentOutputs,

  # [Parameter(Mandatory = $true)]
  # [ValidateNotNullOrEmpty()]
  # [string] $SubscriptionId,

  # [Parameter(ParameterSetName = 'Provisioner', Mandatory = $true)]
  # [ValidateNotNullOrEmpty()]
  # [string] $TenantId,

  [Parameter()]
  [ValidatePattern('^[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}$')]
  [string] $TestApplicationId,

  # [Parameter(Mandatory = $true)]
  # [ValidateNotNullOrEmpty()]
  # [string] $Environment,

  # Captures any arguments from eng/New-TestResources.ps1 not declared here (no parameter errors).
  [Parameter(ValueFromRemainingArguments = $true)]
  $RemainingArguments
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

# if ($CI) {
#   if (!$AdditionalParameters['deployResources']) {
#     Write-Host "Skipping post-provisioning script because resources weren't deployed"
#     return
#   }
  # az cloud set -n $Environment
  # az login --federated-token $env:ARM_OIDC_TOKEN --service-principal -t $TenantId -u $TestApplicationId
  # az account set --subscription $SubscriptionId
# }

$DeploymentOutputs = @{
  IDENTITY_FUNCTIONAPP_NAME = 'chlowe'
  IDENTITY_RESOURCE_GROUP = 'chlowe'
}

$rg = $DeploymentOutputs['IDENTITY_RESOURCE_GROUP']

Set-Location "$(git rev-parse --show-toplevel)/sdk/identity/azure_identity/tests/tools/deployed_live_test"

Write-Host "##[group]Building test app"
cargo install --path . --root target
Write-Host "##[endgroup]"

Write-Host "##[group]Deploy Azure Function App"
Compress-Archive -Path "target/bin/deployed_live_test.exe", "host.json" -DestinationPath func.zip -Force
az functionapp deploy -g $rg -n $DeploymentOutputs['IDENTITY_FUNCTIONAPP_NAME'] --src-path func.zip --type zip
Write-Host "##vso[task.setvariable variable=IDENTITY_FUNCTIONAPP_URL;]$functionAppUrl"
Write-Host "##[endgroup]"
