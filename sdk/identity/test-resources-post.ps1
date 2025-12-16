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

Set-Location "$(git rev-parse --show-toplevel)/sdk/identity/azure_identity/tests/tools/deployed_live_test"

Write-Host "##[group]Building test app"
cargo install --path . --root target
Write-Host "##[endgroup]"

if ($DeploymentOutputs['IDENTITY_FUNCTIONAPP_NAME']) {
  Write-Host "##[group]Deploy Azure Function App"

  $functionAppName = $DeploymentOutputs['IDENTITY_FUNCTIONAPP_NAME']
  $functionAppHostname = $DeploymentOutputs['IDENTITY_FUNCTIONAPP_DEFAULT_HOSTNAME']

  # Build the probe for Linux
  Push-Location "$PSScriptRoot/azure_identity/tests/tools/deployed_live_test"
  try {
    Write-Host "Building probe binary for Linux..."
    cargo build --release --target x86_64-unknown-linux-musl

    $tempDir = New-Item -ItemType Directory -Path ([System.IO.Path]::GetTempPath()) -Name "func-deploy-$(New-Guid)"
    try {
      Copy-Item "target/x86_64-unknown-linux-musl/release/deployed_live_test" -Destination $tempDir
      Copy-Item "host.json" -Destination $tempDir
      Copy-Item "probe" -Destination $tempDir -Recurse

      $zipPath = Join-Path ([System.IO.Path]::GetTempPath()) "func-deploy-$(New-Guid).zip"
      Write-Host "Creating deployment package..."
      Compress-Archive -Path "$tempDir/*" -DestinationPath $zipPath -Force

      # Deploy to Function App
      Write-Host "Deploying to Function App..."
      az functionapp deployment source config-zip `
          --resource-group $ResourceGroupName `
          --name $functionAppName `
          --src $zipPath

      Remove-Item $zipPath -Force

      $functionAppUrl = "https://$functionAppHostname"
      Write-Host "##vso[task.setvariable variable=IDENTITY_FUNCTIONAPP_URL;]$functionAppUrl"

      Write-Host "Function App deployed successfully: $functionAppUrl"
    }
    finally {
      Remove-Item $tempDir -Recurse -Force
    }
  }
  finally {
    Pop-Location
  }

  Write-Host "##[endgroup]"
}
