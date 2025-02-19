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
  [boolean] $ReplaceLatestEntryTitle = $false
)

. (Join-Path $PSScriptRoot '../common/scripts/common.ps1')

Write-Host "Getting package properties for $PackageName in $ServiceDirectory."
$pkgProperties = Get-PkgProperties -PackageName $PackageName -ServiceDirectory $ServiceDirectory

Write-Host "Found package:"
Write-Host "  Name: $($pkgProperties.Name)"
Write-Host "  Version: $($pkgProperties.Version)"
Write-Host "  Directory: $($pkgProperties.DirectoryPath)"
Write-Host "  ChangeLogPath: $($pkgProperties.ChangeLogPath)"

if ($NewVersionString) {
  $newSemVer = [AzureEngSemanticVersion]::new($NewVersionString)
}
else {
  $newSemVer = [AzureEngSemanticVersion]::new($pkgProperties.Version)
  $newSemVer.IncrementAndSetToPrerelease();
}

#If we're just bumping the version with no release date, we want to set the changelog entry to unreleased
$setChangeLogEntryToUnreleased = !$ReleaseDate -and !$NewVersionString

if ($newSemVer.HasValidPrereleaseLabel() -ne $true) {
  Write-Error "Invalid prerelease label: $newSemVer"
  exit 1
}

if ($pkgProperties.ChangeLogPath) {
  Write-Host "Updating changelog for $PackageName in $ServiceDirectory."
  & "$EngCommonScriptsDir/Update-ChangeLog.ps1" -Version $newSemVer.ToString() `
    -ChangelogPath $pkgProperties.ChangeLogPath -Unreleased $setChangeLogEntryToUnreleased `
    -ReplaceLatestEntryTitle $ReplaceLatestEntryTitle -ReleaseDate $ReleaseDate
}

$tomlPath = Join-Path $pkgProperties.DirectoryPath "Cargo.toml"
$content = Get-Content -Path $tomlPath -Raw
$updated = $content -replace '([package](.|\n)+?version\s*=\s*)"(.+?)"', "`$1`"$newSemVer`""

if ($content -ne $updated) {
  $updated | Set-Content -Path $tomlPath  -Encoding utf8 -NoNewLine
  Write-Host "Updated version in $tomlPath from $($pkgProperties.Version) to $newSemVer."

  cargo metadata --format-version 1 | Out-Null
  Write-Host "Updated Cargo.lock using 'cargo metadata'."
}
else {
  Write-Host "$tomlPath already contains version $newSemVer"
}
