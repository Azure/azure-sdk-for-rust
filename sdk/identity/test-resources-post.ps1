# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# IMPORTANT: Do not invoke this file directly. Please instead run eng/common/TestResources/New-TestResources.ps1 from the repository root.

param (
  [hashtable] $AdditionalParameters = @{},
  [hashtable] $ArmTemplateParameters = @{},

  [Parameter(Mandatory = $true)]
  [hashtable] $DeploymentOutputs,

  [Parameter(Mandatory = $true)]
  [ValidateNotNullOrEmpty()]
  [string] $SubscriptionId,

  [string] $TenantId,

  [Parameter()]
  [ValidatePattern('^[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}$')]
  [string] $TestApplicationId,

  [string] $Environment = 'AzureCloud',

  [switch] $CI = ($null -ne $env:SYSTEM_TEAMPROJECTID),

  [ValidateRange(1, 1800)]
  [int] $ReadinessTimeoutSeconds = 180,

  # Captures any arguments from eng/New-TestResources.ps1 not declared here (no parameter errors).
  [Parameter(ValueFromRemainingArguments = $true)]
  $RemainingArguments
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

$deployed = $false
if (![bool]::TryParse($DeploymentOutputs['IDENTITY_DEPLOY_RESOURCES'], [ref] $deployed)) {
  throw 'Missing or invalid deployment output IDENTITY_DEPLOY_RESOURCES'
}
if (!$deployed) {
  Write-Host "Skipping post-provisioning script because resources weren't deployed"
  return
}

foreach ($name in @(
  'IDENTITY_RESOURCE_GROUP',
  'IDENTITY_ACR_NAME',
  'IDENTITY_ACR_LOGIN_SERVER',
  'IDENTITY_AKS_NAME',
  'IDENTITY_STORAGE_NAME_USER_ASSIGNED',
  'IDENTITY_USER_ASSIGNED_IDENTITY',
  'IDENTITY_USER_ASSIGNED_IDENTITY_CLIENT_ID',
  'IDENTITY_USER_ASSIGNED_IDENTITY_NAME'
)) {
  if ([string]::IsNullOrWhiteSpace($DeploymentOutputs[$name])) {
    throw "Missing deployment output $name"
  }
}

function Wait-IdentityAddress {
  param([string] $Name, [scriptblock] $GetAddress)

  $deadline = (Get-Date).AddSeconds($ReadinessTimeoutSeconds)
  do {
    $address = & $GetAddress
    $parsed = $null
    if ([System.Net.IPAddress]::TryParse([string] $address, [ref] $parsed)) {
      return $address
    }
    Start-Sleep -Seconds 5
  } while ((Get-Date) -lt $deadline)
  throw "Timed out waiting for $Name IP address"
}

function Wait-IdentityEndpoint {
  param([string] $Name, [uri] $Uri)

  $deadline = (Get-Date).AddSeconds($ReadinessTimeoutSeconds)
  $failure = 'no response'
  do {
    try {
      $response = Invoke-WebRequest -Uri $Uri -TimeoutSec 10 -SkipHttpErrorCheck
      if ($response.StatusCode -eq 200) {
        return
      }
      $failure = "HTTP $($response.StatusCode)"
    }
    catch [System.Net.Http.HttpRequestException], [System.Threading.Tasks.TaskCanceledException] {
      $failure = $_.Exception.Message
    }
    Start-Sleep -Seconds 5
  } while ((Get-Date) -lt $deadline)
  throw "Timed out waiting for $Name authentication and storage access: $failure"
}

function Publish-IdentityVariable {
  param([string] $Name, [string] $Value)

  [Environment]::SetEnvironmentVariable($Name, $Value, 'Process')
  if ($CI) {
    Write-Host "##vso[task.setvariable variable=$Name;]$Value"
  } else {
    Write-Host "PowerShell: `$env:$Name = '$Value'"
    Write-Host "sh: export $Name='$Value'"
  }
}

if ($CI) {
  if (!$TenantId -or !$TestApplicationId -or !$env:ARM_OIDC_TOKEN) {
    throw 'CI provisioning requires TenantId, TestApplicationId, and ARM_OIDC_TOKEN'
  }
  az cloud set -n $Environment
  az login --federated-token $env:ARM_OIDC_TOKEN --service-principal -t $TenantId -u $TestApplicationId
}
az account set --subscription $SubscriptionId

Push-Location ([System.IO.Path]::Combine($PSScriptRoot, 'azure_identity', 'tests', 'tools', 'deployed_live_test'))
$dockerfile = $null
$manifest = $null
$kubeconfig = $null
try {
  Write-Host "##[group]Building test app"
  cargo install --path . --root target
  Write-Host "##[endgroup]"

  Write-Host "##[group]Building container image"
  az acr login -n $DeploymentOutputs['IDENTITY_ACR_NAME']
  $image = "$($DeploymentOutputs['IDENTITY_ACR_LOGIN_SERVER'])/live-test"
  $dockerfile = New-TemporaryFile
  Set-Content -LiteralPath $dockerfile -Value @"
FROM mcr.microsoft.com/mirror/docker/library/ubuntu:24.04
RUN apt update && apt install ca-certificates --no-install-recommends -y
COPY target/bin/deployed_live_test .
CMD ["./deployed_live_test"]
"@
  docker build --file $dockerfile -t $image .
  docker push $image
  Write-Host "##[endgroup]"

  $rg = $DeploymentOutputs['IDENTITY_RESOURCE_GROUP']

  Write-Host "##[group]Deploying Azure Container Instance with user-assigned identity"
  $containerName = "azure-identity-test-user-assigned"
  az container create -g $rg -n $containerName --image $image `
    --acr-identity $($DeploymentOutputs['IDENTITY_USER_ASSIGNED_IDENTITY']) `
    --assign-identity $($DeploymentOutputs['IDENTITY_USER_ASSIGNED_IDENTITY']) `
    --cpu 1 `
    --ip-address Public `
    --memory 1.0 `
    --os-type Linux `
    --ports 8080
  $aciIP = Wait-IdentityAddress -Name ACI -GetAddress {
    az container show -g $rg -n $containerName --query ipAddress.ip -o tsv
  }
  $storageName = $DeploymentOutputs['IDENTITY_STORAGE_NAME_USER_ASSIGNED']
  $clientId = $DeploymentOutputs['IDENTITY_USER_ASSIGNED_IDENTITY_CLIENT_ID']
  Wait-IdentityEndpoint -Name ACI -Uri "http://${aciIP}:8080/api?test=managed-identity&client-id=$clientId&storage-name=$storageName"
  Publish-IdentityVariable -Name IDENTITY_ACI_IP_USER_ASSIGNED -Value $aciIP
  Write-Host "##[endgroup]"

  $aksName = $DeploymentOutputs['IDENTITY_AKS_NAME']
  $serviceAccountName = "workload-identity-sa"

  Write-Host "##[group]Creating federated identity"
  $idName = $DeploymentOutputs['IDENTITY_USER_ASSIGNED_IDENTITY_NAME']
  $issuer = az aks show -g $rg -n $aksName --query "oidcIssuerProfile.issuerUrl" -otsv
  az identity federated-credential create -g $rg --identity-name $idName --issuer $issuer --name $idName --subject system:serviceaccount:default:$serviceAccountName --audiences api://AzureADTokenExchange
  Write-Host "##[endgroup]"

  Write-Host "##[group]Deploying to AKS"
  $kubeconfig = New-TemporaryFile
  az aks get-credentials -g $rg -n $aksName --file $kubeconfig --overwrite-existing
  az aks update --attach-acr $DeploymentOutputs['IDENTITY_ACR_NAME'] -g $rg -n $aksName
  $manifest = New-TemporaryFile
  Set-Content -LiteralPath $manifest -Value @"
apiVersion: v1
kind: ServiceAccount
metadata:
  annotations:
    azure.workload.identity/client-id: $($DeploymentOutputs['IDENTITY_USER_ASSIGNED_IDENTITY_CLIENT_ID'])
  name: $serviceAccountName
  namespace: default
---
apiVersion: v1
kind: Pod
metadata:
  name: $containerName
  namespace: default
  labels:
    app: $containerName
    azure.workload.identity/use: "true"
spec:
  serviceAccountName: $serviceAccountName
  containers:
  - name: $containerName
    image: $image
    imagePullPolicy: Always
    ports:
    - containerPort: 8080
    readinessProbe:
      tcpSocket:
        port: 8080
  nodeSelector:
    kubernetes.io/os: linux
---
apiVersion: v1
kind: Service
metadata:
  name: $containerName-service
  namespace: default
spec:
  selector:
    app: $containerName
  ports:
  - protocol: TCP
    port: 8080
    targetPort: 8080
  type: LoadBalancer
"@
  kubectl --kubeconfig $kubeconfig delete pod $containerName --ignore-not-found=true --wait=true "--timeout=${ReadinessTimeoutSeconds}s"
  kubectl --kubeconfig $kubeconfig apply -f $manifest --wait=true
  kubectl --kubeconfig $kubeconfig wait --for=condition=Ready "pod/$containerName" "--timeout=${ReadinessTimeoutSeconds}s"
  $serviceIP = Wait-IdentityAddress -Name AKS -GetAddress {
    kubectl --kubeconfig $kubeconfig get service "$($containerName)-service" -o jsonpath='{.status.loadBalancer.ingress[0].ip}'
  }
  Wait-IdentityEndpoint -Name AKS -Uri "http://${serviceIP}:8080/api?test=workload-identity&storage-name=$storageName"
  Publish-IdentityVariable -Name IDENTITY_AKS_IP -Value $serviceIP
  Write-Host "##[endgroup]"
}
finally {
  try {
    foreach ($file in @($dockerfile, $manifest, $kubeconfig)) {
      if ($file -and (Test-Path -LiteralPath $file)) {
        Remove-Item -LiteralPath $file
      }
    }
  }
  finally {
    Pop-Location
  }
}
