# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0

<#
.SYNOPSIS
    Deliverable 2 (SKELETON): generate the per-target Go modules for the
    Azure/azure-cosmos-driver repository layout.

.DESCRIPTION
    Reads build-matrix.json and the per-artifact
    rust-driver-native-interface-metadata.json files produced by
    Build-NativeMatrix.ps1, then emits — for each matrix row — the m x n files:

        <module_path>/go.mod
        <module_path>/link_<goos>_<goarch>[_<libc>].go
        <module_path>/native[/<libc>]/libazurecosmosdriver.a
        <module_path>/native[/<libc>]/azurecosmosdriver.h

    The captured `native-static-libs` from each metadata file are spliced verbatim
    into the cgo `#cgo LDFLAGS:` line (never hand-guessed).

    ------------------------------------------------------------------------
    KEY DESIGN DECISION — glibc vs musl collision (the core Deliverable-2 output)
    ------------------------------------------------------------------------
    Rows linux-amd64-glibc & linux-amd64-musl are BOTH GOOS=linux GOARCH=amd64
    (same for arm64). Go's build system has no standard "musl" tag, so both rows
    map to the SAME module path (e.g. linux/amd64). We disambiguate WITHIN the
    module using a custom, consumer-set build tag:

        glibc (default): link_linux_amd64_glibc.go  //go:build cgo && linux && amd64 && !cosmos_musl
                         LDFLAGS -L ${SRCDIR}/native/glibc ...
        musl  (opt-in) : link_linux_amd64_musl.go   //go:build cgo && linux && amd64 && cosmos_musl
                         LDFLAGS -L ${SRCDIR}/native/musl  ...

    Consumer selection:
        * glibc distros (default): `go build`                 (no tag)
        * Alpine / musl:           `go build -tags cosmos_musl`

    Trade-off (flagged as an OPEN decision to ratify): forgetting -tags cosmos_musl
    on a musl host produces a LOUD link/runtime error, not silent corruption.
    The considered alternative — a distinct module path `linux_musl/amd64` — is
    documented in NATIVE_SUPPLY_CHAIN.md; it removes the tag footgun but forces
    the azcosmos consumer into build-tag-switched import files. Do not change the
    scheme here without updating that doc and the azure-sdk-for-go consumer.

.PARAMETER ArtifactRoot
    Directory containing <target-id>/ produced by Build-NativeMatrix.ps1.
    Default: ./artifacts under this folder.

.PARAMETER OutputRoot
    Root of the generated Azure/azure-cosmos-driver layout.
    Default: ./generated/azure-cosmos-driver under this folder.

.PARAMETER TargetId
    Optional filter: generate the matching matrix row id(s). A selected Linux
    row also selects its sibling libc row because both share one Go module.

.PARAMETER SkipNativeCopy
    Skip copying the static archive (e.g. when it is fetched separately in CI).
    The header and per-target metadata remain required and are still copied.
#>
[CmdletBinding()]
param(
    [string]   $ArtifactRoot,
    [string]   $OutputRoot,
    [string[]] $TargetId,
    [switch]   $SkipNativeCopy
)

Set-StrictMode -Version 3.0
$ErrorActionPreference = 'Stop'

$PipelineDir = $PSScriptRoot
$MatrixPath  = Join-Path $PipelineDir 'build-matrix.json'
$MetadataFilename = 'rust-driver-native-interface-metadata.json'
if (-not $ArtifactRoot) { $ArtifactRoot = Join-Path $PipelineDir 'artifacts' }
if (-not $OutputRoot)   { $OutputRoot   = Join-Path $PipelineDir 'generated' 'azure-cosmos-driver' }

$matrix = Get-Content $MatrixPath -Raw | ConvertFrom-Json
$allRows = @($matrix.targets)
$rows = $allRows
if ($TargetId) {
    $selectedRows = @($allRows | Where-Object { $TargetId -contains $_.id })
    $selectedModulePaths = @($selectedRows.module_path | Sort-Object -Unique)
    $rows = @($allRows | Where-Object { $selectedModulePaths -contains $_.module_path })
}
if (-not $rows) { throw "No matching targets for filter: $($TargetId -join ', ')" }

New-Item -ItemType Directory -Force -Path $OutputRoot | Out-Null

function Get-SyslibsForRow($row) {
    $manifestPath = Join-Path $ArtifactRoot $row.id $MetadataFilename
    if (Test-Path $manifestPath) {
        $m = Get-Content $manifestPath -Raw | ConvertFrom-Json
        if ($m.native_static_libs) { return @($m.native_static_libs) }
    }
    if ($matrix.reference_syslibs.PSObject.Properties.Name -contains $row.triple) {
        Write-Warning "[$($row.id)] no manifest syslibs; using reference_syslibs."
        return @($matrix.reference_syslibs.$($row.triple))
    }
    throw "[$($row.id)] no syslibs available; run Build-NativeMatrix.ps1 first."
}

$writtenGoMods = @{}
foreach ($row in $rows) {
    $moduleDir = Join-Path $OutputRoot ($row.module_path -replace '/', [IO.Path]::DirectorySeparatorChar)
    New-Item -ItemType Directory -Force -Path $moduleDir | Out-Null

    # go.mod is written once per module_path (glibc + musl rows share it).
    if (-not $writtenGoMods.ContainsKey($row.module_path)) {
        $modulePathFull = "$($matrix.module_root)/$($row.module_path)"
        @"
module $modulePathFull

go $($matrix.go_version)
"@ | Set-Content -Path (Join-Path $moduleDir 'go.mod') -Encoding utf8 -NoNewline
        $writtenGoMods[$row.module_path] = $true
    }

    # Native destination: native/ for single-libc rows, native/<libc>/ for linux.
    $nativeRel = if ($row.native_subdir) { "native/$($row.native_subdir)" } else { 'native' }
    $nativeDir = Join-Path $moduleDir ($nativeRel -replace '/', [IO.Path]::DirectorySeparatorChar)
    New-Item -ItemType Directory -Force -Path $nativeDir | Out-Null

    $aSrc = Join-Path $ArtifactRoot $row.id $matrix.static_lib_filename
    $hSrc = Join-Path $ArtifactRoot $row.id $matrix.header_filename
    if (-not (Test-Path $hSrc)) {
        throw "[$($row.id)] missing $($matrix.header_filename); build the complete module first."
    }
    Copy-Item $hSrc $nativeDir -Force
    Copy-Item $hSrc $moduleDir -Force

    if (-not $SkipNativeCopy) {
        if (-not (Test-Path $aSrc)) {
            throw "[$($row.id)] missing $($matrix.static_lib_filename); build the complete module first."
        }
        Copy-Item $aSrc $nativeDir -Force
    }

    # Build constraint + cgo link file.
    $tag = "cgo && $($row.goos) && $($row.goarch)"
    if ($row.build_tag_extra) { $tag += " && $($row.build_tag_extra)" }

    $syslibs = (Get-SyslibsForRow $row) -join ' '
    $ldflags = "-L`${SRCDIR}/$nativeRel -l$($matrix.lib_basename) $syslibs".TrimEnd()

    $suffix = if ($row.native_subdir) { "_$($row.native_subdir)" } else { '' }
    $linkName = "link_$($row.goos)_$($row.goarch)$suffix.go"

    @"
// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Code generated by New-GoModules.ps1; DO NOT EDIT.
// Target: $($row.id)  triple: $($row.triple)

//go:build $tag

package $($matrix.go_package)

// #cgo LDFLAGS: $ldflags
// #include "$($matrix.header_filename)"
import "C"
"@ | Set-Content -Path (Join-Path $moduleDir $linkName) -Encoding utf8

    Write-Host "[$($row.id)] -> $($row.module_path)/$linkName  (tag: $tag)"
}

Write-Host ''
Write-Host "Generated Go modules under: $OutputRoot"
Write-Host "musl consumers must build with: go build -tags $($matrix.musl_build_tag)"
