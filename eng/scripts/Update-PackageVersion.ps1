<#
.SYNOPSIS
Bumps up package versions after release

.DESCRIPTION
This script bumps up package versions following conventions defined at https://github.com/Azure/azure-sdk/blob/main/docs/policies/releases.md#incrementing-after-release-net

.PARAMETER ServiceDirectory
The Name of the Service Directory

.PARAMETER PackageName
The Name of the Package

.PARAMETER NewVersionString
Use this to overide version incement logic and set a version specified by this parameter

.EXAMPLE
Updating package version for Azure.Core
Update-PackageVersion.ps1 -ServiceDirectory core -PackageName azure_core

Updating package version for Azure.Core with a specified verion
Update-PackageVersion.ps1 -ServiceDirectory core -PackageName azure_core -NewVersionString 2.0.5

Updating package version for Azure.Core with a specified verion and release date
Update-PackageVersion.ps1 -ServiceDirectory core -PackageName azure_core -NewVersionString 2.0.5 -ReleaseDate "2020-05-01"
#>

[CmdletBinding()]
Param (
  [Parameter(Mandatory = $True)]
  [string] $ServiceDirectory,
  [Parameter(Mandatory = $True)]
  [string] $PackageName,
  [string] $NewVersionString,
  [string] $ReleaseDate,
  [boolean] $ReplaceLatestEntryTitle = $true
)

. (Join-Path $PSScriptRoot '../common/scripts/common.ps1')

$pkgProperties = Get-PkgProperties -PackageName $PackageName -ServiceDirectory $ServiceDirectory
$packageVersion = $pkgProperties.Version

$packageSemVer = [AzureEngSemanticVersion]::new($packageVersion)
Write-Host "Current Version: ${PackageVersion}"

if ([System.String]::IsNullOrEmpty($NewVersionString)) {
  $packageSemVer.IncrementAndSetToPrerelease();

  & "$EngCommonScriptsDir/Update-ChangeLog.ps1" -Version $packageSemVer.ToString() `
    -ChangelogPath $pkgProperties.ChangeLogPath -Unreleased $True
}
else {
  $packageSemVer = [AzureEngSemanticVersion]::new($NewVersionString)

  & "$EngCommonScriptsDir/Update-ChangeLog.ps1" -Version $packageSemVer.ToString() `
    -ChangelogPath $pkgProperties.ChangeLogPath -Unreleased $False `
    -ReplaceLatestEntryTitle $ReplaceLatestEntryTitle -ReleaseDate $ReleaseDate
}

Write-Host "New Version: $packageSemVer"

if ($packageSemVer.HasValidPrereleaseLabel() -ne $true) {
  Write-Error "Invalid prerelease label"
  exit 1
}

cargo set-version --package $PackageName $packageSemVer.ToString()

$tomlPath = Join-Path $pkgProperties.DirectoryPath "Cargo.toml"
Write-Host "Updated version in $tomlPath to $packageSemVer"
