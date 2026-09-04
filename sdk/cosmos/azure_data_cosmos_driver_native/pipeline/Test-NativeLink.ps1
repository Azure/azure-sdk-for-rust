# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0

<#
.SYNOPSIS
Links a minimal Go/cgo program against one native-driver target artifact.

.DESCRIPTION
Reads the target metadata emitted by Build-NativeMatrix.ps1, then cross-links a
minimal Go program that calls cosmos_version(). This verifies the archive,
header, captured system libraries, target runtime flags, and C compiler before
the target artifact is published. The linked program is not executed.
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$TargetId,

    [Parameter(Mandatory = $true)]
    [string]$ArtifactRoot,

    [Parameter(Mandatory = $true)]
    [string]$CCompiler,

    [string]$MatrixPath = (Join-Path $PSScriptRoot 'build-matrix.json')
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$matrix = Get-Content $MatrixPath -Raw | ConvertFrom-Json
$target = $matrix.targets | Where-Object id -EQ $TargetId | Select-Object -First 1
if (-not $target) {
    throw "Target '$TargetId' is not present in build-matrix.json."
}

$targetRoot = Join-Path (Resolve-Path $ArtifactRoot).Path $TargetId
$metadataPath = Join-Path $targetRoot 'rust-driver-native-interface-metadata.json'
if (-not (Test-Path $metadataPath -PathType Leaf)) {
    throw "Target metadata is missing: $metadataPath"
}
$metadata = Get-Content $metadataPath -Raw | ConvertFrom-Json
foreach ($property in @(
    'schema_version',
    'artifact_id',
    'goos',
    'goarch',
    'libc',
    'triple',
    'rustc_native_static_libs',
    'native_static_libs'
)) {
    if ($metadata.PSObject.Properties.Name -notcontains $property) {
        throw "[$TargetId] metadata is missing '$property'."
    }
}
if ($metadata.schema_version -ne 3) {
    throw "[$TargetId] metadata 'schema_version' does not match expected version 3."
}
$expectedIdentity = @{
    artifact_id = $target.id
    goos = $target.goos
    goarch = $target.goarch
    libc = $target.libc
    triple = $target.triple
}
foreach ($property in $expectedIdentity.Keys) {
    if ([string]$metadata.$property -cne [string]$expectedIdentity[$property]) {
        throw "[$TargetId] metadata '$property' does not match build-matrix.json."
    }
}

$libraryPath = Join-Path $targetRoot $matrix.static_lib_filename
$headerPath = Join-Path $targetRoot $matrix.header_filename
foreach ($path in @($libraryPath, $headerPath)) {
    if (-not (Test-Path $path -PathType Leaf)) {
        throw "Required native link input is missing: $path"
    }
}
if (-not (Get-Command $CCompiler -ErrorAction SilentlyContinue)) {
    throw "C compiler '$CCompiler' is not available for target '$TargetId'."
}

$runtimeFlags = if ($target.PSObject.Properties.Name -contains 'static_runtime_ldflags') {
    @($target.static_runtime_ldflags)
}
else {
    @()
}
$linkerFlags = @(
    '-L${SRCDIR}/native'
    "-l$($matrix.lib_basename)"
    $runtimeFlags
    @($metadata.native_static_libs)
) | Where-Object { $_ }

$tag = "cgo && $($target.goos) && $($target.goarch)"
if ($target.build_tag_extra) {
    $tag += " && $($target.build_tag_extra)"
}
$source = @"
// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//go:build $tag

package main

// #cgo CFLAGS: -I`${SRCDIR}/native
// #cgo LDFLAGS: $($linkerFlags -join ' ')
// #include "$($matrix.header_filename)"
import "C"

func main() {
	_ = C.cosmos_version()
}
"@

$workRoot = Join-Path ([IO.Path]::GetTempPath()) "cosmos-native-link-$TargetId-$([guid]::NewGuid())"
$savedEnvironment = @{
    CC = $env:CC
    CGO_ENABLED = $env:CGO_ENABLED
    GOARCH = $env:GOARCH
    GOOS = $env:GOOS
}
try {
    $nativeRoot = Join-Path $workRoot 'native'
    New-Item -ItemType Directory -Path $nativeRoot -Force | Out-Null
    Copy-Item $libraryPath, $headerPath -Destination $nativeRoot -Force
    [IO.File]::WriteAllText(
        (Join-Path $workRoot 'go.mod'),
        "module native-link-smoke`n`ngo $($matrix.go_version)`n",
        [Text.UTF8Encoding]::new($false)
    )
    [IO.File]::WriteAllText(
        (Join-Path $workRoot 'main.go'),
        $source.Replace("`r`n", "`n").Replace("`r", "`n") + "`n",
        [Text.UTF8Encoding]::new($false)
    )

    $env:CC = $CCompiler
    $env:CGO_ENABLED = '1'
    $env:GOOS = $target.goos
    $env:GOARCH = $target.goarch
    $outputPath = Join-Path $workRoot $(if ($target.goos -eq 'windows') { 'smoke.exe' } else { 'smoke' })
    $arguments = @('build', '-trimpath', '-o', $outputPath)
    $arguments += '.'

    Push-Location $workRoot
    try {
        $output = & go @arguments 2>&1
        if ($LASTEXITCODE -ne 0) {
            throw "Go/cgo link smoke test failed for '$TargetId' with exit code ${LASTEXITCODE}:`n$($output -join "`n")"
        }
    }
    finally {
        Pop-Location
    }
    if (-not (Test-Path $outputPath -PathType Leaf)) {
        throw "Go/cgo link smoke test did not produce an output for '$TargetId'."
    }
}
finally {
    foreach ($name in $savedEnvironment.Keys) {
        if ($null -eq $savedEnvironment[$name]) {
            Remove-Item "Env:$name" -ErrorAction SilentlyContinue
        }
        else {
            Set-Item "Env:$name" $savedEnvironment[$name]
        }
    }
    if (Test-Path $workRoot) {
        Remove-Item $workRoot -Recurse -Force
    }
}

Write-Host "Go/cgo link smoke test passed for $TargetId."
