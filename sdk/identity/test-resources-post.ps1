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

Write-Host "##[group]Building test app"
Set-Location "$(git rev-parse --show-toplevel)/sdk/identity/azure_identity/tools/deployed_live_test"
cargo install --path . --root .
Write-Host "##[endgroup]"

Write-Host "##[group]Building container image"
az acr login -n $DeploymentOutputs['IDENTITY_ACR_NAME']
$image = "$($DeploymentOutputs['IDENTITY_ACR_LOGIN_SERVER'])/live-test"
Set-Content -Path Dockerfile -Value @"
FROM mcr.microsoft.com/mirror/docker/library/ubuntu:24.04
RUN apt update && apt install ca-certificates --no-install-recommends -y
COPY bin/deployed_live_test .
CMD ["./deployed_live_test"]
"@
docker build -t $image .
docker push $image
Write-Host "##[endgroup]"

$rg = $DeploymentOutputs['IDENTITY_RESOURCE_GROUP']

Write-Host "##[group]Deploying Azure Container Instance with user-assigned identity"
$aciName = "azure-identity-test-user-assigned"
az container create -g $rg -n $aciName --image $image `
  --acr-identity $($DeploymentOutputs['IDENTITY_USER_ASSIGNED_IDENTITY']) `
  --assign-identity $($DeploymentOutputs['IDENTITY_USER_ASSIGNED_IDENTITY']) `
  --cpu 1 `
  --ip-address Public `
  --memory 1.0 `
  --os-type Linux `
  --ports 8080
$aciIP = az container show -g $rg -n $aciName --query ipAddress.ip -o tsv
Write-Host "##vso[task.setvariable variable=IDENTITY_ACI_IP_USER_ASSIGNED;]$aciIP"
Write-Host "##[endgroup]"
