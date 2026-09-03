# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0

<#
.SYNOPSIS
    Generate the per-target Go modules for the Azure/azure-cosmos-driver
    repository layout.

.DESCRIPTION
    Reads build-matrix.json and the per-artifact
    rust-driver-native-interface-metadata.json files produced by
    Build-NativeMatrix.ps1, then emits — for each matrix row — the m x n files:

        <module_path>/go.mod
        <module_path>/link_<goos>_<goarch>.go
        <module_path>/native/libazurecosmosdriver.a
        <module_path>/native/azurecosmosdriver.h

    The captured `native-static-libs` from each metadata file and any
    target-specific static runtime flags from build-matrix.json are spliced into
    the cgo `#cgo LDFLAGS:` line.

    Linux glibc and musl use distinct modules. The unmarked linux/<arch> module
    contains glibc, while linux/<arch>-musl contains musl. This follows the Go
    distribution design and avoids requiring consumers to select the native
    archive with a custom build tag.

.PARAMETER ArtifactRoot
    Directory containing <target-id>/ produced by Build-NativeMatrix.ps1.
    Default: ./artifacts under this folder.

.PARAMETER OutputRoot
    Root of the generated Azure/azure-cosmos-driver layout.
    Default: ./generated/azure-cosmos-driver under this folder.

.PARAMETER TargetId
    Optional filter: generate the matching matrix row id(s).

.PARAMETER SkipNativeCopy
    Skip copying the static archive (e.g. when it is fetched separately in CI).
    The header and per-target metadata remain required and are still copied.
#>
[CmdletBinding()]
param(
    [string]   $ArtifactRoot,
    [string]   $OutputRoot,
    [string[]] $TargetId,
    [switch]   $SkipNativeCopy,
    [string]   $MatrixPath = (Join-Path $PSScriptRoot 'build-matrix.json')
)

Set-StrictMode -Version 3.0
$ErrorActionPreference = 'Stop'

$PipelineDir = $PSScriptRoot
$MetadataFilename = 'rust-driver-native-interface-metadata.json'
if (-not $ArtifactRoot) { $ArtifactRoot = Join-Path $PipelineDir 'artifacts' }
if (-not $OutputRoot)   { $OutputRoot   = Join-Path $PipelineDir 'generated' 'azure-cosmos-driver' }

$matrix = Get-Content $MatrixPath -Raw | ConvertFrom-Json
$allRows = @($matrix.targets)
$rows = $allRows
if ($TargetId) {
    $rows = @($allRows | Where-Object { $TargetId -contains $_.id })
}
if (-not $rows) { throw "No matching targets for filter: $($TargetId -join ', ')" }

function Test-Property {
    param(
        [Parameter(Mandatory = $true)]
        [object]$Object,

        [Parameter(Mandatory = $true)]
        [string]$Name
    )

    $null -ne $Object -and $Object.PSObject.Properties.Name -contains $Name
}

function Assert-MetadataValue {
    param(
        [Parameter(Mandatory = $true)]
        [string]$TargetId,

        [Parameter(Mandatory = $true)]
        [object]$Metadata,

        [Parameter(Mandatory = $true)]
        [string]$Name,

        [AllowNull()]
        [object]$Expected
    )

    if (-not (Test-Property -Object $Metadata -Name $Name)) {
        throw "[$TargetId] metadata is missing '$Name'."
    }

    $actual = $Metadata.$Name
    if ($null -eq $Expected) {
        if ($null -ne $actual) {
            throw "[$TargetId] metadata '$Name' mismatch: expected null, found '$actual'."
        }
        return
    }

    if ([string]$actual -cne [string]$Expected) {
        throw "[$TargetId] metadata '$Name' mismatch: expected '$Expected', found '$actual'."
    }
}

function Get-FileSha256 {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path
    )

    (Get-FileHash $Path -Algorithm SHA256).Hash.ToLowerInvariant()
}

function Assert-FileHash {
    param(
        [Parameter(Mandatory = $true)]
        [string]$TargetId,

        [Parameter(Mandatory = $true)]
        [string]$Description,

        [Parameter(Mandatory = $true)]
        [string]$Path,

        [Parameter(Mandatory = $true)]
        [string]$ExpectedHash
    )

    $actualHash = Get-FileSha256 -Path $Path
    if ($actualHash -cne $ExpectedHash.ToLowerInvariant()) {
        throw "[$TargetId] $Description SHA256 mismatch: metadata records '$ExpectedHash', file is '$actualHash'."
    }
}

function Write-GeneratedTextFile {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path,

        [Parameter(Mandatory = $true)]
        [string]$Content
    )

    $normalizedContent = $Content.Replace("`r`n", "`n").Replace("`r", "`n")
    if (-not $normalizedContent.EndsWith("`n")) {
        $normalizedContent += "`n"
    }
    [IO.File]::WriteAllText($Path, $normalizedContent, [Text.UTF8Encoding]::new($false))
}

$verifiedArtifacts = @{}
$releaseIdentity = $null
$releaseHeaderHash = $null

foreach ($row in $rows) {
    $targetRoot = Join-Path $ArtifactRoot $row.id
    $manifestPath = Join-Path $targetRoot $MetadataFilename
    if (-not (Test-Path $manifestPath -PathType Leaf)) {
        throw "[$($row.id)] missing $MetadataFilename; run Build-NativeMatrix.ps1 first."
    }

    $metadata = Get-Content $manifestPath -Raw | ConvertFrom-Json
    Assert-MetadataValue -TargetId $row.id -Metadata $metadata -Name 'schema_version' -Expected 3
    Assert-MetadataValue -TargetId $row.id -Metadata $metadata -Name 'artifact_id' -Expected $row.id
    Assert-MetadataValue -TargetId $row.id -Metadata $metadata -Name 'triple' -Expected $row.triple
    Assert-MetadataValue -TargetId $row.id -Metadata $metadata -Name 'goos' -Expected $row.goos
    Assert-MetadataValue -TargetId $row.id -Metadata $metadata -Name 'goarch' -Expected $row.goarch
    Assert-MetadataValue -TargetId $row.id -Metadata $metadata -Name 'libc' -Expected $row.libc
    Assert-MetadataValue -TargetId $row.id -Metadata $metadata -Name 'native_interface_crate' -Expected $matrix.native_interface_crate
    Assert-MetadataValue -TargetId $row.id -Metadata $metadata -Name 'rust_driver_crate' -Expected $matrix.rust_driver_crate

    foreach ($name in @(
        'source_commit',
        'native_interface_version',
        'rust_driver_version',
        'rustc_native_static_libs',
        'native_static_libs',
        'static_library',
        'header'
    )) {
        if (-not (Test-Property -Object $metadata -Name $name)) {
            throw "[$($row.id)] metadata is missing '$name'."
        }
    }
    if (-not $metadata.source_commit) {
        throw "[$($row.id)] metadata 'source_commit' must not be empty."
    }
    if (-not $metadata.native_interface_version -or -not $metadata.rust_driver_version) {
        throw "[$($row.id)] metadata package versions must not be empty."
    }

    if (-not (Test-Property -Object $metadata.static_library -Name 'sha256')) {
        throw "[$($row.id)] metadata static library is missing 'sha256'."
    }
    if (-not (Test-Property -Object $metadata.header -Name 'sha256')) {
        throw "[$($row.id)] metadata header is missing 'sha256'."
    }
    Assert-MetadataValue -TargetId $row.id -Metadata $metadata.static_library -Name 'filename' -Expected $matrix.static_lib_filename
    Assert-MetadataValue -TargetId $row.id -Metadata $metadata.header -Name 'filename' -Expected $matrix.header_filename
    if (-not $metadata.header.sha256) {
        throw "[$($row.id)] metadata header SHA256 must not be empty."
    }

    $headerPath = Join-Path $targetRoot $matrix.header_filename
    if (-not (Test-Path $headerPath -PathType Leaf)) {
        throw "[$($row.id)] missing $($matrix.header_filename); build the complete module first."
    }
    Assert-FileHash -TargetId $row.id -Description 'header' -Path $headerPath -ExpectedHash $metadata.header.sha256

    $staticLibraryPath = Join-Path $targetRoot $matrix.static_lib_filename
    if (-not $SkipNativeCopy) {
        if (-not (Test-Path $staticLibraryPath -PathType Leaf)) {
            throw "[$($row.id)] missing $($matrix.static_lib_filename); build the complete module first."
        }
        if (-not $metadata.static_library.sha256) {
            throw "[$($row.id)] metadata static-library SHA256 must not be empty."
        }
        Assert-FileHash -TargetId $row.id -Description 'static library' -Path $staticLibraryPath -ExpectedHash $metadata.static_library.sha256
    }

    $identity = [ordered]@{
        source_commit = [string]$metadata.source_commit
        native_interface_version = [string]$metadata.native_interface_version
        rust_driver_version = [string]$metadata.rust_driver_version
    }
    if ($null -eq $releaseIdentity) {
        $releaseIdentity = $identity
    } else {
        foreach ($name in $identity.Keys) {
            if ($identity[$name] -cne $releaseIdentity[$name]) {
                throw "[$($row.id)] release identity '$name' mismatch: expected '$($releaseIdentity[$name])', found '$($identity[$name])'."
            }
        }
    }

    $headerHash = ([string]$metadata.header.sha256).ToLowerInvariant()
    if ($null -eq $releaseHeaderHash) {
        $releaseHeaderHash = $headerHash
    } elseif ($headerHash -cne $releaseHeaderHash) {
        throw "[$($row.id)] header SHA256 differs from the other selected targets: expected '$releaseHeaderHash', found '$headerHash'."
    }

    $verifiedArtifacts[$row.id] = [pscustomobject]@{
        Metadata = $metadata
        HeaderPath = $headerPath
        StaticLibraryPath = $staticLibraryPath
    }
}

New-Item -ItemType Directory -Force -Path $OutputRoot | Out-Null

$writtenGoMods = @{}
$writtenModuleHeaders = @{}
foreach ($row in $rows) {
    $verifiedArtifact = $verifiedArtifacts[$row.id]
    $moduleDir = Join-Path $OutputRoot ($row.module_path -replace '/', [IO.Path]::DirectorySeparatorChar)
    New-Item -ItemType Directory -Force -Path $moduleDir | Out-Null

    # go.mod is written once per module_path.
    if (-not $writtenGoMods.ContainsKey($row.module_path)) {
        $modulePathFull = "$($matrix.module_root)/$($row.module_path)"
        $goModContent = @"
module $modulePathFull

go $($matrix.go_version)
"@
        Write-GeneratedTextFile -Path (Join-Path $moduleDir 'go.mod') -Content $goModContent
        $writtenGoMods[$row.module_path] = $true
    }

    # Native destination is normally native/. native_subdir remains available
    # for any future target that needs multiple archives in one module.
    $nativeRel = if ($row.native_subdir) { "native/$($row.native_subdir)" } else { 'native' }
    $nativeDir = Join-Path $moduleDir ($nativeRel -replace '/', [IO.Path]::DirectorySeparatorChar)
    New-Item -ItemType Directory -Force -Path $nativeDir | Out-Null

    $aSrc = $verifiedArtifact.StaticLibraryPath
    $hSrc = $verifiedArtifact.HeaderPath
    Copy-Item $hSrc $nativeDir -Force
    if (-not $writtenModuleHeaders.ContainsKey($row.module_path)) {
        Copy-Item $hSrc $moduleDir -Force
        $writtenModuleHeaders[$row.module_path] = $true
    }

    if (-not $SkipNativeCopy) {
        Copy-Item $aSrc $nativeDir -Force
    }

    # Build constraint + cgo link file.
    $tag = "cgo && $($row.goos) && $($row.goarch)"
    if ($row.build_tag_extra) { $tag += " && $($row.build_tag_extra)" }

    $syslibs = @($verifiedArtifact.Metadata.native_static_libs) -join ' '
    $staticRuntimeLdflags = if ($row.PSObject.Properties.Name -contains 'static_runtime_ldflags') {
        @($row.static_runtime_ldflags) -join ' '
    } else {
        ''
    }
    $ldflags = @(
        "-L`${SRCDIR}/$nativeRel"
        "-l$($matrix.lib_basename)"
        $staticRuntimeLdflags
        $syslibs
    ) | Where-Object { $_ }
    $ldflags = $ldflags -join ' '

    $suffix = if ($row.native_subdir) { "_$($row.native_subdir)" } else { '' }
    $linkName = "link_$($row.goos)_$($row.goarch)$suffix.go"

    $linkFileContent = @"
// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Code generated by New-GoModules.ps1; DO NOT EDIT.
// Target: $($row.id)  triple: $($row.triple)

//go:build $tag

package $($matrix.go_package)

// #cgo LDFLAGS: $ldflags
// #include "$($matrix.header_filename)"
import "C"
"@
    Write-GeneratedTextFile -Path (Join-Path $moduleDir $linkName) -Content $linkFileContent

    Write-Host "[$($row.id)] -> $($row.module_path)/$linkName  (tag: $tag)"
}

# Consolidated, human-readable release provenance for the whole generated tree.
# It is written at the scan root ($OutputRoot) so the 1ES SBOM COSE signature
# covers it, binding the Rust driver version and source commit into the signed
# artifact. $releaseIdentity was cross-validated across every selected target in
# the verification loop above, so it is the single agreed release identity.
$provenanceTargets = foreach ($row in $rows) {
    $targetMetadata = $verifiedArtifacts[$row.id].Metadata
    [ordered]@{
        id                    = [string]$row.id
        triple                = [string]$row.triple
        module_path           = [string]$row.module_path
        static_library_sha256 = ([string]$targetMetadata.static_library.sha256).ToLowerInvariant()
        header_sha256         = ([string]$targetMetadata.header.sha256).ToLowerInvariant()
    }
}

$provenance = [ordered]@{
    schema_version           = 1
    source_commit            = $releaseIdentity['source_commit']
    native_interface_crate   = [string]$matrix.native_interface_crate
    native_interface_version = $releaseIdentity['native_interface_version']
    rust_driver_crate        = [string]$matrix.rust_driver_crate
    rust_driver_version      = $releaseIdentity['rust_driver_version']
    targets                  = @($provenanceTargets)
}

$provenanceJson = $provenance | ConvertTo-Json -Depth 6
Write-GeneratedTextFile -Path (Join-Path $OutputRoot 'provenance.json') -Content $provenanceJson
Write-Host "Wrote provenance.json (commit $($releaseIdentity['source_commit']), rust_driver v$($releaseIdentity['rust_driver_version']))"

Write-Host ''
Write-Host "Generated Go modules under: $OutputRoot"
