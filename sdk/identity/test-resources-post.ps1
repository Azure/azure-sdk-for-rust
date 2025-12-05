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

$repoRoot = git rev-parse --show-toplevel
$testAppDir = [System.IO.Path]::Combine($repoRoot, "sdk", "identity", "azure_identity", "tests", "tools", "deployed_live_test")
$targetDir = [System.IO.Path]::Combine($testAppDir, "target")

Write-Host "##[group]Building test app"
cargo install --path $testAppDir --root $targetDir
Write-Host "##[endgroup]"

Write-Host "##[group]Building container image"
az acr login -n $DeploymentOutputs['IDENTITY_ACR_NAME']
$image = "$($DeploymentOutputs['IDENTITY_ACR_LOGIN_SERVER'])/live-test"
$dockerfilePath = [System.IO.Path]::Combine($testAppDir, "Dockerfile")
Set-Content -Path $dockerfilePath -Value @"
FROM mcr.microsoft.com/mirror/docker/library/ubuntu:24.04
RUN apt update && apt install ca-certificates --no-install-recommends -y
COPY target/bin/deployed_live_test .
CMD ["./deployed_live_test"]
"@
docker build -t $image $testAppDir
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
$aciIP = az container show -g $rg -n $containerName --query ipAddress.ip -o tsv
Write-Host "##vso[task.setvariable variable=IDENTITY_ACI_IP_USER_ASSIGNED;]$aciIP"
Write-Host "##[endgroup]"

$aksName = $DeploymentOutputs['IDENTITY_AKS_NAME']
$serviceAccountName = "workload-identity-sa"

Write-Host "##[group]Creating federated identity"
$idName = $DeploymentOutputs['IDENTITY_USER_ASSIGNED_IDENTITY_NAME']
$issuer = az aks show -g $rg -n $aksName --query "oidcIssuerProfile.issuerUrl" -otsv
az identity federated-credential create -g $rg --identity-name $idName --issuer $issuer --name $idName --subject system:serviceaccount:default:$serviceAccountName --audiences api://AzureADTokenExchange
Write-Host "##[endgroup]"

Write-Host "##[group]Deploying to AKS"
az aks get-credentials -g $rg -n $aksName
az aks update --attach-acr $DeploymentOutputs['IDENTITY_ACR_NAME'] -g $rg -n $aksName
Set-Content -Path "$PSScriptRoot/k8s.yaml" -Value @"
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
    ports:
    - containerPort: 8080
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
kubectl apply -f "$PSScriptRoot/k8s.yaml" --wait=true

$timeout = [TimeSpan]::FromMinutes(2)
$interval = 20
$startTime = Get-Date
do {
  $serviceIP = kubectl get service "$($containerName)-service" -o jsonpath='{.status.loadBalancer.ingress[0].ip}'
  if ($serviceIP) { break }
  Start-Sleep -Seconds $interval
} while ((Get-Date) - $startTime -lt $timeout)
if (-not $serviceIP) {
  Write-Error "Timed out waiting for AKS test pod's external IP"
  exit 1
}
Write-Host "##vso[task.setvariable variable=IDENTITY_AKS_IP;]$serviceIP"
Write-Host "##[endgroup]"
