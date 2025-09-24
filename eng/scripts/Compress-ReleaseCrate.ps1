#!/usr/bin/env pwsh

#Requires -Version 7.0
[CmdletBinding(DefaultParameterSetName = "none")]
param(
  [string]$ArtifactName,
  [string]$ArtifactRootPath,
  [string]$OutFile
)

Write-Host "Artifact name: $ArtifactName"

$packageMetadataPath = "$ArtifactRootPath/PackageInfo/$ArtifactName.json"
if (!(Test-Path $packageMetadataPath)) {
  Write-Error "Package metadata file not found: $packageMetadataPath"
  exit 1
}

$packageMetadata = Get-Content -Raw $packageMetadataPath | ConvertFrom-Json
$packageVersion = $packageMetadata.version
Write-Host "Package version: $packageVersion"

New-Item -ItemType Directory -Path '$(Pipeline.Workspace)/release' -Force | Out-Null
Compress-Archive `
  -Path "$ArtifactRootPath/$ArtifactName/$ArtifactName-$packageVersion.crate" `
  -DestinationPath $OutFile

Write-Host "Created archive: $OutFile"
