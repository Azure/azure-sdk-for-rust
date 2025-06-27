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

Write-Host "Building test app"
cd "$(git rev-parse --show-toplevel)/sdk/identity/azure_identity/tools/managed_identity_test"
cargo install --path . --root .

Write-Host "Building container image"
$image = "$($DeploymentOutputs['AZURE_IDENTITY_ACR_LOGIN_SERVER'])/managed-id-test"
Set-Content -Path Dockerfile -Value @"
FROM mcr.microsoft.com/mirror/docker/library/ubuntu:24.04
RUN apt update && apt install ca-certificates --no-install-recommends -y
COPY bin/managed_identity_test .
CMD ["./managed_identity_test"]
"@
docker build -t $image .
az acr login -n $DeploymentOutputs['AZURE_IDENTITY_ACR_NAME']
docker push $image

$rg = $DeploymentOutputs['IDENTITY_RESOURCE_GROUP']

# ACI is easier to provision here than in the bicep file because the image isn't available before now
Write-Host "Deploying Azure Container Instance"
$aciName = "azure-identity-test"
az container create -g $rg -n $aciName --image $image `
  --acr-identity $($DeploymentOutputs['AZURE_IDENTITY_USER_ASSIGNED_IDENTITY']) `
  --assign-identity $($DeploymentOutputs['AZURE_IDENTITY_USER_ASSIGNED_IDENTITY']) `
  --cpu 1 `
  --ip-address Public `
  --memory 1.0 `
  --os-type Linux `
  -e FUNCTIONS_CUSTOMHANDLER_PORT=80
$aciIP = az container show -g $rg -n $aciName --query ipAddress.ip --output tsv
Write-Host "##vso[task.setvariable variable=AZURE_IDENTITY_ACI_IP;]$aciIP"

az container logs -g $rg -n $aciName
Get-AzContainerInstanceLog -ResourceGroupName $rg -ContainerGroupName $aciName -ContainerName $aciName
