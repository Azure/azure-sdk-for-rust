#!/usr/bin/env pwsh

#Requires -Version 7.0
[CmdletBinding(DefaultParameterSetName = "none")]
param(
  [string]$OutputPath,
  [string[]]$PackageNames,
  [string]$OutBuildOrderFile = 'build-order.json'
)

. ([System.IO.Path]::Combine($PSScriptRoot, 'Pack-Common.ps1'))

# TODO: Ensure this works
$RepoRoot = [System.IO.Path]::Combine($PSScriptRoot, '../..')

$metadata = Get-CargoMetadata
$packagesToBuild = $metadata.packages | Where-Object { $PackageNames.Contains($_.name) }

$packageParams = @()
foreach ($package in $packagesToBuild) {
  $packageParams += "--package", $package.name
}

Write-Host "cargo publish --dry-run $($packageParams -join ' ') --target-dir $OutputPath --allow-dirty"
cargo publish --dry-run @packageParams --target-dir $OutputPath --allow-dirty 2>&1 | Tee-Object -Variable result

if ($LASTEXITCODE) {
  Write-Host "cargo publish failed with exit code $LASTEXITCODE"
  exit $LASTEXITCODE
}

foreach ($package in $packagesToBuild) {
  $sourcePath = [System.IO.Path]::Combine($RepoRoot, "target", "package", "$($package.name)-$($package.version)")
  $targetPath = [System.IO.Path]::Combine($OutputPath, $package.name)
  $targetApiReviewFile = [System.IO.Path]::Combine($targetPath, "$($package.name).rust.json")

  Write-Host "Copying package '$($package.name)' to '$targetPath'"
  New-Item -ItemType Directory -Path $targetPath -Force | Out-Null
  Copy-Item -Path "$sourcePath.crate" -Destination $targetPath

  Write-Host "Creating API review file"
  $apiReviewFile = Create-ApiViewFile $package
  
  Write-Host "Copying API review file to '$targetApiReviewFile'"
  Copy-Item -Path $apiReviewFile -Destination $targetApiReviewFile -Force
}

if ($OutBuildOrderFile) {
  $buildOrder = @()
  foreach ($line in $result) { 
    if ($line -match '^\s*Packaging (\w*) ([\w\d\.-]*)') {
      $buildOrder += $matches[1]
    }
  }

  Write-Host "Build Order: $($buildOrder -join ', ')"
  $buildOrder | ConvertTo-Json -Depth 100 | Set-Content $OutBuildOrderFile
}
