# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

[CmdletBinding()]
param(
  [ValidatePattern('^\d+$')]
  [string] $BuildId = $env:BUILD_BUILDID,

  [string] $BuildJob = $env:AGENT_JOBNAME
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $false

if ([string]::IsNullOrWhiteSpace($BuildId) -or [string]::IsNullOrWhiteSpace($BuildJob)) {
  throw 'BuildId and BuildJob are required to scope identity deployment diagnostics'
}

function Invoke-IdentityAzDiagnostic {
  param([string] $Label, [string[]] $Arguments)

  Write-Host "Identity deployment diagnostic: $Label"
  try {
    $output = & az @Arguments --only-show-errors --output json 2>&1
    if ($LASTEXITCODE -ne 0) {
      $detail = ($output -join "`n")
      if ($detail.Length -gt 2048) {
        $detail = $detail.Substring(0, 2048) + ' [truncated]'
      }
      Write-Warning "$Label failed (exit $LASTEXITCODE): $detail"
      return
    }
    ($output -join "`n") | ConvertFrom-Json
  }
  catch {
    Write-Warning "$Label unavailable: $($_.Exception.Message)"
  }
}

function Write-IdentityDiagnosticRecords {
  param([string] $Label, [object[]] $Records)

  if (!$Records.Count) {
    Write-Host "No records returned for $Label; resources or deployment history may already have been removed. See any diagnostic warnings above."
    return
  }
  foreach ($record in $Records) {
    # Activity Log status messages may contain serialized provider error details.
    if ($null -ne $record.message) {
      $message = if ($record.message -is [string]) { $record.message } else { $record.message | ConvertTo-Json -Depth 10 -Compress }
      if ($message.Length -gt 2048) {
        $record.message = $message.Substring(0, 2048) + ' [truncated]'
      }
    }
    Write-Host ($record | ConvertTo-Json -Depth 10)
  }
}

function Write-IdentityActivityFailures {
  param([string] $ResourceGroup)

  $records = @(Invoke-IdentityAzDiagnostic -Label "Activity Log for $ResourceGroup" -Arguments @(
    'monitor', 'activity-log', 'list', '--resource-group', $ResourceGroup,
    '--offset', '2h', '--max-events', '50', '--status', 'Failed',
    '--query', '[].{time:eventTimestamp,operation:operationName.value,status:status.value,resource:resourceId,correlationId:correlationId,message:properties.statusMessage}'
  ))
  Write-IdentityDiagnosticRecords -Label "failed operations in $ResourceGroup" -Records $records
}

$groups = @(Invoke-IdentityAzDiagnostic -Label 'Find resource group for this build and job' -Arguments @(
  'group', 'list', '--tag', "BuildId=$BuildId",
  '--query', '[].{name:name,state:properties.provisioningState,tags:tags}'
))
$matchingGroups = @($groups | Where-Object {
  $_.tags.BuildId -ceq $BuildId -and $_.tags.BuildJob -ceq $BuildJob -and $_.tags.ServiceDirectory -ceq 'identity'
})
if ($matchingGroups.Count -ne 1) {
  Write-Warning "Expected one identity resource group for build $BuildId and job $BuildJob, found $($matchingGroups.Count). Discovery may be unavailable or cleanup may have removed it; no other resource groups will be inspected."
  return
}

$group = $matchingGroups[0]
Write-Host "Identity resource group: $($group.name), state: $($group.state)"
$deployments = @(Invoke-IdentityAzDiagnostic -Label 'Deployment state' -Arguments @(
  'deployment', 'group', 'list', '--resource-group', $group.name,
  '--query', '[].{name:name,state:properties.provisioningState,time:properties.timestamp,correlationId:properties.correlationId,errorCode:properties.error.code,message:properties.error.message}'
))
Write-IdentityDiagnosticRecords -Label 'deployments' -Records $deployments
foreach ($deployment in $deployments) {
  $operations = @(Invoke-IdentityAzDiagnostic -Label "Operations for $($deployment.name)" -Arguments @(
    'deployment', 'operation', 'group', 'list', '--resource-group', $group.name, '--name', $deployment.name,
    '--query', '[?properties.provisioningState!=`Succeeded`].{time:properties.timestamp,state:properties.provisioningState,resource:properties.targetResource.id,message:properties.statusMessage}'
  ))
  Write-IdentityDiagnosticRecords -Label 'incomplete deployment operations' -Records $operations
}
Write-IdentityActivityFailures -ResourceGroup $group.name

$clusters = @(Invoke-IdentityAzDiagnostic -Label 'AKS provisioning state' -Arguments @(
  'aks', 'list', '--resource-group', $group.name,
  '--query', '[].{name:name,state:provisioningState,nodeResourceGroup:nodeResourceGroup,pools:agentPoolProfiles[].{name:name,state:provisioningState}}'
))
Write-IdentityDiagnosticRecords -Label 'AKS clusters' -Records $clusters
foreach ($cluster in $clusters) {
  if ([string]::IsNullOrWhiteSpace($cluster.nodeResourceGroup)) {
    Write-Warning "No node resource group is available for $($cluster.name)"
    continue
  }
  $scaleSets = @(Invoke-IdentityAzDiagnostic -Label "Worker scale sets for $($cluster.name)" -Arguments @(
    'vmss', 'list', '--resource-group', $cluster.nodeResourceGroup,
    '--query', '[].{name:name,state:provisioningState,capacity:sku.capacity}'
  ))
  Write-IdentityDiagnosticRecords -Label 'worker scale sets' -Records $scaleSets
  foreach ($scaleSet in $scaleSets) {
    $instances = @(Invoke-IdentityAzDiagnostic -Label "Instances in $($scaleSet.name)" -Arguments @(
      'vmss', 'list-instances', '--resource-group', $cluster.nodeResourceGroup, '--name', $scaleSet.name,
      '--expand', 'instanceView',
      '--query', '[].{instance:instanceId,state:provisioningState,statuses:instanceView.statuses[].{code:code,message:message}}'
    ))
    Write-IdentityDiagnosticRecords -Label 'worker instances' -Records $instances
  }
  Write-IdentityActivityFailures -ResourceGroup $cluster.nodeResourceGroup
}
