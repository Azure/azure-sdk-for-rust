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
pushd "$(git rev-parse --show-toplevel)/sdk/identity/azure_identity/tools/managed_identity_test"
cargo build --release --target x86_64-unknown-linux-gnu
Copy-Item -Path "target/release/managed_identity_test" -Destination .

# Write-Host "Building container image"
# $image = "$($DeploymentOutputs['AZURE_IDENTITY_ACR_LOGIN_SERVER'])/managed-id-test"
# Set-Content -Path "$PSScriptRoot/Dockerfile" -Value @"
# FROM mcr.microsoft.com/mirror/docker/library/ubuntu:24.04
# COPY target/release/managed_identity_test .
# CMD ["./managed_identity_test"]
# "@
# docker build -t $image .
# az acr login -n $DeploymentOutputs['AZURE_IDENTITY_ACR_NAME']
# docker push $image

$rg = $DeploymentOutputs['AZURE_IDENTITY_RESOURCE_GROUP']

Write-Host "Deploying to Azure Functions"
Get-ChildItem -Path . -Recurse | Where-Object { $_.FullName -notlike "target" } | Compress-Archive -DestinationPath func.zip -Force
az functionapp deploy -g $rg -n $DeploymentOutputs['AZURE_IDENTITY_FUNCTION_NAME'] --src-path func.zip --type zip
