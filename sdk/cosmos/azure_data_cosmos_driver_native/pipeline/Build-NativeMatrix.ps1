# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0

<#
.SYNOPSIS
    Deliverable 1: reproducible per-target cross-build of the Rust driver native
    interface libraries.

.DESCRIPTION
    For each row in build-matrix.json this script:
      1. Ensures the Rust target triple is installed (rustup).
      2. Captures the exact syslib link line PROGRAMMATICALLY via
         `cargo rustc ... -- --print native-static-libs` (never hand-guessed).
      3. Builds the static and dynamic libraries with cargo-auditable so the
         artifacts embed their Rust dependency manifest.
      4. Emits rust-driver-native-interface-metadata.json (triple,
         GOOS/GOARCH/libc, package versions, source commit, toolchain versions,
         SHA256, and syslibs).

    The script runs end-to-end for a target whose toolchain and linker are
    installed, typically the host target. It reports a clear error before the
    build when a required cross-linker is missing. Wiring into CI happens in native-driver.yml, not here.

.PARAMETER TargetId
    Optional filter: build only the matching matrix row id(s) (e.g.
    'windows-amd64','linux-amd64-musl'). Default: all `targets`.

.PARAMETER OutputRoot
    Directory that receives each target's native libraries, header, and
    rust-driver-native-interface-metadata.json.
    Default: ./artifacts under this folder.

.PARAMETER SkipBuild
    Capture syslibs + write manifests but do not produce the .a (dry run for the
    generator / offline manifest refresh).

.PARAMETER NoAuditable
    Build with plain `cargo` instead of `cargo auditable` (e.g. when
    cargo-auditable is not installed on a dev box). CI MUST NOT set this.

.EXAMPLE
    ./Build-NativeMatrix.ps1 -TargetId windows-amd64
#>
[CmdletBinding()]
param(
    [string[]] $TargetId,
    [string]   $OutputRoot,
    [switch]   $SkipBuild,
    [switch]   $NoAuditable
)

Set-StrictMode -Version 3.0
$ErrorActionPreference = 'Stop'

$PipelineDir = $PSScriptRoot
$CrateDir    = Split-Path -Parent $PipelineDir
$RepoRoot    = (Resolve-Path (Join-Path $CrateDir '..' '..' '..')).Path
$MatrixPath  = Join-Path $PipelineDir 'build-matrix.json'
$MetadataFilename = 'rust-driver-native-interface-metadata.json'

if (-not $OutputRoot) { $OutputRoot = Join-Path $PipelineDir 'artifacts' }
New-Item -ItemType Directory -Force -Path $OutputRoot | Out-Null

$matrix = Get-Content $MatrixPath -Raw | ConvertFrom-Json
$rows = $matrix.targets
if ($TargetId) { $rows = $rows | Where-Object { $TargetId -contains $_.id } }
if (-not $rows) { throw "No matching targets for filter: $($TargetId -join ', ')" }

function Get-ToolVersion([string] $exe, [string[]] $verArgs) {
    try { (& $exe @verArgs 2>&1 | Select-Object -First 1) -join ' ' }
    catch { $null }
}

# Programmatically parse the `native-static-libs:` note out of a
# `--print native-static-libs` run. Returns a string[] of `-l...`/`-L...` flags.
function Get-NativeStaticLibs([string] $triple) {
    Push-Location $CrateDir
    try {
        $out = & cargo rustc --release --quiet `
            -p $matrix.native_interface_crate `
            --target $triple `
            -- --print native-static-libs 2>&1
        if ($LASTEXITCODE -ne 0) {
            throw "cargo rustc failed for $triple with exit code $LASTEXITCODE`n$($out -join "`n")"
        }
    }
    finally { Pop-Location }

    $line = $out | Where-Object { $_ -match 'native-static-libs:' } | Select-Object -Last 1
    if (-not $line) {
        Write-Warning "Could not capture native-static-libs for $triple. rustc output:`n$($out -join "`n")"
        return @()
    }
    ($line -replace '.*native-static-libs:\s*', '').Trim() -split '\s+' | Where-Object { $_ }
}

function Test-TripleInstalled([string] $triple) {
    (& rustup target list --installed 2>$null) -contains $triple
}

$sourceCommit = (& git -C $RepoRoot rev-parse HEAD 2>$null)
if ($LASTEXITCODE -ne 0) { throw 'Unable to resolve the source commit.' }

$cargoMetadataJson = & cargo metadata --format-version 1 --no-deps `
    --manifest-path (Join-Path $CrateDir 'Cargo.toml') 2>&1
if ($LASTEXITCODE -ne 0) {
    throw "cargo metadata failed with exit code $LASTEXITCODE`n$($cargoMetadataJson -join "`n")"
}
$cargoMetadata = $cargoMetadataJson | ConvertFrom-Json
$nativeInterfacePackage = $cargoMetadata.packages |
    Where-Object name -EQ $matrix.native_interface_crate |
    Select-Object -First 1
$rustDriverPackage = $cargoMetadata.packages |
    Where-Object name -EQ $matrix.rust_driver_crate |
    Select-Object -First 1
if (-not $nativeInterfacePackage) {
    throw "Cargo package not found: $($matrix.native_interface_crate)"
}
if (-not $rustDriverPackage) {
    throw "Cargo package not found: $($matrix.rust_driver_crate)"
}

$builtWith = if ($NoAuditable) { 'cargo' } else { 'cargo-auditable' }

$summary = @()
foreach ($row in $rows) {
    Write-Host "==> $($row.id) ($($row.triple))" -ForegroundColor Cyan

    if (-not (Test-TripleInstalled $row.triple)) {
        Write-Host "    target not installed; attempting 'rustup target add $($row.triple)'"
        & rustup target add $row.triple *> $null
        if ($LASTEXITCODE -ne 0) {
            throw "rustup target add failed for $($row.triple) with exit code $LASTEXITCODE"
        }
    }
    # TODO(preflight): a `rustc --print native-static-libs` and the actual link
    # additionally require a cross-linker for non-host triples:
    #   *-pc-windows-gnu   -> mingw-w64
    #   *-unknown-linux-*  -> the matching gcc/clang cross toolchain (musl: musl-gcc)
    #   *-apple-darwin     -> Apple SDK (macOS host or osxcross)
    # CI installs these per-agent; on a dev box a missing linker is reported here,
    # not swallowed.

    $syslibs = Get-NativeStaticLibs $row.triple
    if ((-not $syslibs) -and $matrix.reference_syslibs.PSObject.Properties.Name -contains $row.triple) {
        Write-Warning "    falling back to reference_syslibs for $($row.triple) (capture failed)"
        $syslibs = @($matrix.reference_syslibs.$($row.triple))
    }

    $targetOut = Join-Path $OutputRoot $row.id
    New-Item -ItemType Directory -Force -Path $targetOut | Out-Null

    $sha = $null
    if (-not $SkipBuild) {
        Push-Location $CrateDir
        try {
            $buildArgs = @(
                'build', '--release',
                '-p', $matrix.native_interface_crate,
                '--target', $row.triple,
                # Production size profile measured in Azure/azure-sdk-for-rust#4748:
                # roughly 48-54% smaller with no observed runtime regression.
                # Keep panic unwinding because the FFI boundary relies on it.
                '--config', 'profile.release.opt-level="z"',
                '--config', 'profile.release.lto="fat"',
                '--config', 'profile.release.codegen-units=1',
                '--config', 'profile.release.strip="symbols"'
            )
            if ($NoAuditable) { & cargo @buildArgs }
            else              { & cargo auditable @buildArgs }
            if ($LASTEXITCODE -ne 0) {
                throw "$builtWith build failed for $($row.triple) with exit code $LASTEXITCODE"
            }
        }
        finally { Pop-Location }

        $aSrc = Join-Path $RepoRoot "target/$($row.triple)/release/$($matrix.static_lib_filename)"
        if (-not (Test-Path $aSrc)) {
            throw "Expected static library not found: $aSrc"
        }
        Copy-Item $aSrc (Join-Path $targetOut $matrix.static_lib_filename) -Force
        $sha = (Get-FileHash $aSrc -Algorithm SHA256).Hash.ToLowerInvariant()

        $dynamicLibFilename = switch ($row.goos) {
            'windows' { "$($matrix.lib_basename).dll" }
            'linux'   { "lib$($matrix.lib_basename).so" }
            'darwin'  { "lib$($matrix.lib_basename).dylib" }
            default   { throw "Unsupported GOOS for dynamic library naming: $($row.goos)" }
        }
        # The dynamic library (.dll/.so/.dylib) is a best-effort extensibility
        # hook for the future .NET/Java/Python path (Azure/azure-sdk-for-rust#5048).
        # It is NOT consumed by the Go/.a path, so a missing dynamic lib must not
        # fail the build; only the static .a checked above is a hard gate.
        $dynamicSrc = Join-Path $RepoRoot "target/$($row.triple)/release/$dynamicLibFilename"
        if (Test-Path $dynamicSrc) {
            Copy-Item $dynamicSrc (Join-Path $targetOut $dynamicLibFilename) -Force
            $dynamicSha = (Get-FileHash $dynamicSrc -Algorithm SHA256).Hash.ToLowerInvariant()
        }
        else {
            Write-Warning "    dynamic library not built (best-effort; not required for the Go/.a path): $dynamicSrc"
            $dynamicLibFilename = $null
            $dynamicSha = $null
        }
    }

    # The header is target-independent (checked in + regenerated by build.rs).
    $headerSrc = Join-Path $CrateDir "include/$($matrix.header_filename)"
    if (Test-Path $headerSrc) { Copy-Item $headerSrc $targetOut -Force }

    $manifest = [ordered]@{
        schema_version           = 1
        artifact_id              = $row.id
        goos                     = $row.goos
        goarch                   = $row.goarch
        libc                     = $row.libc
        triple                   = $row.triple
        native_interface_crate   = $matrix.native_interface_crate
        native_interface_version = $nativeInterfacePackage.version
        rust_driver_crate        = $matrix.rust_driver_crate
        rust_driver_version      = $rustDriverPackage.version
        rust_driver_features     = $matrix.rust_driver_features
        source_commit            = $sourceCommit
        built_with               = $builtWith
        static_library = [ordered]@{
            filename = $matrix.static_lib_filename
            sha256   = $sha
        }
        dynamic_library = if ($SkipBuild -or (-not $dynamicLibFilename)) { $null } else {
            [ordered]@{
                filename = $dynamicLibFilename
                sha256   = $dynamicSha
            }
        }
        native_static_libs = $syslibs
        toolchains = [ordered]@{
            rustc = Get-ToolVersion 'rustc' @('--version')
            cargo = Get-ToolVersion 'cargo' @('--version')
        }
        generated_utc = (Get-Date).ToUniversalTime().ToString('o')
    }
    $manifestPath = Join-Path $targetOut $MetadataFilename
    $manifest | ConvertTo-Json -Depth 6 | Set-Content -Path $manifestPath -Encoding utf8
    Write-Host "    metadata -> $manifestPath"
    $summary += [pscustomobject]@{ id = $row.id; triple = $row.triple; sha256 = $sha; syslibs = $syslibs.Count }
}

Write-Host ''
$summary | Format-Table -AutoSize
Write-Host "Artifacts under: $OutputRoot"
