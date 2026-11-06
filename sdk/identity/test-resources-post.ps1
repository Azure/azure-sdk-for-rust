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

  [ValidateRange(1, 1800)]
  [int] $EndpointReadinessTimeoutSeconds = 900,

  # Captures any arguments from eng/New-TestResources.ps1 not declared here (no parameter errors).
  [Parameter(ValueFromRemainingArguments = $true)]
  $RemainingArguments
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

if (!$DeploymentOutputs.ContainsKey('IDENTITY_ACR_NAME')) {
  throw 'Missing deployment output IDENTITY_ACR_NAME'
}
if ([string]::IsNullOrWhiteSpace($DeploymentOutputs['IDENTITY_ACR_NAME'])) {
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

function Test-IdentityTcpConnection {
  param([uri] $Uri, [int] $TimeoutSeconds)

  $client = [System.Net.Sockets.TcpClient]::new()
  try {
    $null = $client.ConnectAsync($Uri.DnsSafeHost, $Uri.Port).
      WaitAsync([TimeSpan]::FromSeconds($TimeoutSeconds)).GetAwaiter().GetResult()
  }
  finally {
    $client.Dispose()
  }
}

function Wait-IdentityEndpoint {
  param([string] $Name, [uri] $Uri)

  $started = Get-Date
  $deadline = $started.AddSeconds($EndpointReadinessTimeoutSeconds)
  $failure = 'no response'
  $lastHttpFailure = $null
  $tcpReported = $false
  $httpReachable = $false
  $probeUri = [System.UriBuilder]::new($Uri)
  $probeUri.Path = '/'
  $probeUri.Query = ''
  $probeUri.Fragment = ''
  while (($remaining = ($deadline - (Get-Date)).TotalSeconds) -ge 1) {
    $phase = 'TCP connectivity'
    try {
      Test-IdentityTcpConnection -Uri $Uri -TimeoutSeconds ([int] [Math]::Min(5, [Math]::Floor($remaining)))
      if (!$tcpReported) {
        Write-Host "$Name TCP connection to $($Uri.Host):$($Uri.Port) succeeded from this agent"
        $tcpReported = $true
      }

      if (!$httpReachable) {
        $remaining = ($deadline - (Get-Date)).TotalSeconds
        $phase = 'HTTP reachability'
        if ($remaining -lt 1) {
          $failure = "$phase was not checked before the endpoint deadline"
          break
        }
        $probeTimeout = [int] [Math]::Min(10, [Math]::Floor($remaining))
        $probe = Invoke-WebRequest -Uri $probeUri.Uri -TimeoutSec $probeTimeout -SkipHttpErrorCheck
        $httpReachable = $true
        Write-Host "$Name HTTP endpoint responded with status $($probe.StatusCode); checking authentication and storage access"
      }

      $remaining = ($deadline - (Get-Date)).TotalSeconds
      $phase = 'Authentication and storage access'
      if ($remaining -lt 1) {
        $failure = "$phase was not checked before the endpoint deadline"
        break
      }
      $requestTimeout = [int] [Math]::Min(60, [Math]::Floor($remaining))
      $response = Invoke-WebRequest -Uri $Uri -TimeoutSec $requestTimeout -SkipHttpErrorCheck
      if ($response.StatusCode -eq 200) {
        return
      }
      $body = ([string] $response.Content).Trim()
      if ($body.Length -gt 2048) {
        $body = $body.Substring(0, 2048) + ' [truncated]'
      }
      $failure = "${phase}: HTTP $($response.StatusCode): $body"
      $lastHttpFailure = $failure
    }
    catch [System.Net.Http.HttpRequestException], [System.Threading.Tasks.TaskCanceledException], [System.TimeoutException], [System.Net.WebException], [System.Net.Sockets.SocketException] {
      $failure = "${phase}: $($_.Exception.Message)"
    }
    $remaining = ($deadline - (Get-Date)).TotalSeconds
    if ($remaining -gt 0) {
      Start-Sleep -Seconds ([Math]::Min(5, $remaining))
    }
  }
  $elapsed = [Math]::Round(((Get-Date) - $started).TotalSeconds, 1)
  $httpDetails = if ($lastHttpFailure -and $lastHttpFailure -ne $failure) { " Last HTTP failure: $lastHttpFailure" } else { '' }
  throw "Timed out waiting for $Name authentication and storage access at $Uri after ${elapsed}s: $failure$httpDetails"
}

function Write-IdentityAksDiagnostics {
  param([string] $Kubeconfig, [string] $PodName)

  $serviceName = "$PodName-service"
  foreach ($arguments in @(
    @('get', 'pod', $PodName, '-o', 'wide'),
    @('get', 'service', $serviceName, '-o', 'wide'),
    @('get', 'endpointslices', '-l', "kubernetes.io/service-name=$serviceName", '-o', 'wide'),
    @('logs', $PodName, '--tail=100'),
    @('get', 'events', '--field-selector', "involvedObject.name=$PodName", '--sort-by=.metadata.creationTimestamp'),
    @('get', 'events', '--field-selector', "involvedObject.name=$serviceName", '--sort-by=.metadata.creationTimestamp')
  )) {
    try {
      Write-Host "AKS diagnostic: kubectl $($arguments -join ' ')"
      kubectl --kubeconfig $Kubeconfig --namespace default --request-timeout=10s @arguments
    }
    catch {
      Write-Warning "AKS diagnostic failed: $($_.Exception.Message)"
    }
  }
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
  try {
    Wait-IdentityEndpoint -Name AKS -Uri "http://${serviceIP}:8080/api?test=workload-identity&storage-name=$storageName"
  }
  catch {
    Write-IdentityAksDiagnostics -Kubeconfig $kubeconfig -PodName $containerName
    throw
  }
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
