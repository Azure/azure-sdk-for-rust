# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
# cspell:ignore SOURCEBRANCH SOURCEVERSION

#Requires -Version 7.0

<#
.SYNOPSIS
    Validates the native package version and release tag.

.DESCRIPTION
    Ensures Cargo.toml, the generated C header, and CHANGELOG.md agree on the
    native package version. For tagged releases, also requires the expected
    annotated tag to point to the built commit. A pre-1.0 patch release is
    rejected when the checked-in C header differs from the previous release.
#>
[CmdletBinding()]
param(
    [string] $RepositoryRoot = (
        [System.IO.Path]::GetFullPath(
            [System.IO.Path]::Combine($PSScriptRoot, '..', '..', '..', '..')
        )
    ),
    [string] $SourceBranch = $env:BUILD_SOURCEBRANCH,
    [string] $SourceVersion = $env:BUILD_SOURCEVERSION,
    [scriptblock] $GitInvoker
)

Set-StrictMode -Version 3.0
$ErrorActionPreference = 'Stop'

$packageName = 'azure_data_cosmos_driver_native'
$crateRelativePath = 'sdk/cosmos/azure_data_cosmos_driver_native'
$cratePath = [System.IO.Path]::Combine(
    $RepositoryRoot,
    'sdk',
    'cosmos',
    'azure_data_cosmos_driver_native'
)
$cargoPath = [System.IO.Path]::Combine($cratePath, 'Cargo.toml')
$changelogPath = [System.IO.Path]::Combine($cratePath, 'CHANGELOG.md')
$headerPath = [System.IO.Path]::Combine(
    $cratePath,
    'include',
    'azurecosmosdriver.h'
)

function Get-RequiredMatch {
    param(
        [Parameter(Mandatory = $true)]
        [string] $Content,
        [Parameter(Mandatory = $true)]
        [string] $Pattern,
        [Parameter(Mandatory = $true)]
        [string] $Description
    )

    $match = [regex]::Match($Content, $Pattern)
    if (-not $match.Success) {
        throw "Unable to read $Description."
    }
    $match.Groups[1].Value
}

function Invoke-Git {
    param(
        [Parameter(Mandatory = $true)]
        [string[]] $Arguments
    )

    if ($GitInvoker) {
        return @(& $GitInvoker $RepositoryRoot $Arguments)
    }

    $output = @(& git -C $RepositoryRoot @Arguments 2>&1)
    if ($LASTEXITCODE -ne 0) {
        throw "git $($Arguments -join ' ') failed: $($output -join [Environment]::NewLine)"
    }
    @($output)
}

$cargoContent = Get-Content $cargoPath -Raw
$packageSection = Get-RequiredMatch `
    -Content $cargoContent `
    -Pattern '(?ms)^\[package\]\s*(.*?)(?=^\[|\z)' `
    -Description "$cargoPath [package] section"
$cargoVersion = Get-RequiredMatch `
    -Content $packageSection `
    -Pattern '(?m)^\s*version\s*=\s*"([^"]+)"\s*$' `
    -Description "$cargoPath package version"

$parsedVersion = $null
if (-not [version]::TryParse($cargoVersion, [ref]$parsedVersion) -or
    $parsedVersion.ToString(3) -ne $cargoVersion) {
    throw "Native package version '$cargoVersion' must use X.Y.Z format."
}

$headerContent = Get-Content $headerPath -Raw
$headerVersion = Get-RequiredMatch `
    -Content $headerContent `
    -Pattern '(?m)^#define AZURECOSMOSDRIVER_H_VERSION "([^"]+)"\s*$' `
    -Description "$headerPath version"
if ($headerVersion -ne $cargoVersion) {
    throw "Header version '$headerVersion' does not match Cargo version '$cargoVersion'."
}

$changelogContent = Get-Content $changelogPath -Raw
$changelogMatch = [regex]::Match(
    $changelogContent,
    "(?m)^## $([regex]::Escape($cargoVersion)) \((Unreleased|\d{4}-\d{2}-\d{2})\)\s*$"
)
if (-not $changelogMatch.Success) {
    throw "CHANGELOG.md must contain a '$cargoVersion' release heading."
}

$tagPrefix = "refs/tags/$packageName@"
$isRelease = $SourceBranch -and $SourceBranch.StartsWith(
    $tagPrefix,
    [StringComparison]::Ordinal
)
if (-not $isRelease) {
    Write-Host "Validated native development version $cargoVersion."
    return
}

$expectedTag = "$packageName@$cargoVersion"
$actualTag = $SourceBranch.Substring('refs/tags/'.Length)
if ($actualTag -cne $expectedTag) {
    throw "Release tag '$actualTag' does not match expected tag '$expectedTag'."
}
if ($changelogMatch.Groups[1].Value -eq 'Unreleased') {
    throw "CHANGELOG.md version '$cargoVersion' must have a release date."
}
if ($parsedVersion.Major -ge 1) {
    throw "Stable releases require an additive/breaking FFI classifier before publication."
}

$tagTypeOutput = @(Invoke-Git -Arguments @('cat-file', '-t', "refs/tags/$actualTag"))
$tagType = $tagTypeOutput[0]
if ($tagType -cne 'tag') {
    throw "Release tag '$actualTag' must be annotated."
}

$tagCommitOutput = @(Invoke-Git -Arguments @('rev-list', '-n', '1', $actualTag))
$tagCommit = $tagCommitOutput[0]
if (-not $SourceVersion) {
    $sourceVersionOutput = @(Invoke-Git -Arguments @('rev-parse', 'HEAD'))
    $SourceVersion = $sourceVersionOutput[0]
}
if ($tagCommit -cne $SourceVersion) {
    throw "Release tag '$actualTag' points to '$tagCommit', not '$SourceVersion'."
}

$previousReleases = @(
    Invoke-Git -Arguments @('tag', '--list', "$packageName@*") |
        ForEach-Object {
            $tag = [string]$_
            $versionText = $tag.Substring("$packageName@".Length)
            $version = $null
            if ([version]::TryParse($versionText, [ref]$version) -and
                $version -lt $parsedVersion) {
                [pscustomobject]@{
                    Tag = $tag
                    Version = $version
                }
            }
        } |
        Sort-Object Version -Descending
)

if ($previousReleases.Count -gt 0) {
    $previous = $previousReleases[0]
    $isPatch = $previous.Version.Major -eq $parsedVersion.Major -and
        $previous.Version.Minor -eq $parsedVersion.Minor
    if ($isPatch) {
        $headerGitPath = "$crateRelativePath/include/azurecosmosdriver.h"
        $previousHeader = (
            Invoke-Git -Arguments @('show', "$($previous.Tag):$headerGitPath")
        ) -join "`n"
        $versionPattern = '(?m)^(#define AZURECOSMOSDRIVER_H_VERSION) "[^"]+"'
        $previousHeader = $previousHeader -replace $versionPattern, '$1 "<VERSION>"'
        $currentHeader = (
            ($headerContent -replace "`r`n", "`n") -replace $versionPattern, '$1 "<VERSION>"'
        ).TrimEnd("`n")
        if ($previousHeader.TrimEnd("`n") -cne $currentHeader) {
            throw "Patch release '$actualTag' changes the C FFI header; use a minor version."
        }
    }
}

Write-Host "Validated native release $actualTag at $SourceVersion."
