#!/usr/bin/env pwsh
# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

<#
.SYNOPSIS
    Runs the coverage-guided cargo-fuzz targets for the Cosmos binary-JSON codec.

.DESCRIPTION
    Installs (idempotently) the nightly toolchain + cargo-fuzz, seeds each
    target's corpus from the committed golden vectors, then runs every fuzz
    target under `azure_data_cosmos_driver/fuzz` for a bounded wall-clock budget.

    This is a byte-level *protocol* fuzzer: it feeds arbitrary/mutated bytes
    straight into the decoder (decode / from_slice / transcode_to_text) plus a
    differential decode->encode->decode idempotence check. It is the complement
    to the value-space live round-trip fuzzer in
    azure_data_cosmos/tests/binary_roundtrip_fuzzer.rs.

    LINUX ONLY. libFuzzer (the -fsanitize=fuzzer backend) is not supported on the
    Windows MSVC target, so this script no-ops with a warning off Linux.

.PARAMETER MaxTotalTimeSeconds
    Wall-clock budget PER TARGET passed to libFuzzer as `-max_total_time`.
    PR smoke runs use ~90s; weekly deep runs use ~1800s (30 min). Ignored when
    -ValidateOnly is set.

.PARAMETER ValidateOnly
    Regression mode: replay ONLY the seeded golden-vector corpus through each
    target once (libFuzzer `-runs=0`, no mutation, no time budget) and assert no
    crash. This is a fast, deterministic gate suitable for the Build stage — it
    proves the committed golden vectors still decode without panicking, without
    the multi-minute coverage-guided soak (which belongs on a live/weekly leg).

.PARAMETER Toolchain
    Nightly toolchain to use. Defaults to the repo's pinned nightly if the
    RUST_NIGHTLY_TOOLCHAIN env var is set, else plain `nightly`.

.PARAMETER Targets
    Which fuzz targets to run. Defaults to all four.

.EXAMPLE
    ./Run-BinaryJsonFuzz.ps1 -MaxTotalTimeSeconds 90            # PR smoke
    ./Run-BinaryJsonFuzz.ps1 -MaxTotalTimeSeconds 1800          # weekly deep run
#>
[CmdletBinding()]
param(
    [int] $MaxTotalTimeSeconds = 90,
    [switch] $ValidateOnly,
    [string] $Toolchain = $(if ($env:RUST_NIGHTLY_TOOLCHAIN) { $env:RUST_NIGHTLY_TOOLCHAIN } else { 'nightly' }),
    [string[]] $Targets = @('decode', 'from_slice', 'transcode_to_text', 'decode_reencode_roundtrip'),
    [int] $Workers = 0  # 0 => libFuzzer default (single); CI can raise it
)

$ErrorActionPreference = 'Stop'

if (-not $IsLinux) {
    Write-Warning "cargo-fuzz / libFuzzer is only supported on Linux; skipping on this OS. Use WSL or a Linux CI leg."
    exit 0
}

$driverDir = Resolve-Path "$PSScriptRoot/../../azure_data_cosmos_driver"
$fuzzDir = Join-Path $driverDir 'fuzz'
$vectorsPath = Join-Path $driverDir 'testdata/binary_json_vectors.json'

Write-Host "==> Ensuring nightly toolchain '$Toolchain' + cargo-fuzz"
rustup toolchain install $Toolchain --profile minimal --component rust-src
# Detect cargo-fuzz via the installed-binaries list (always exits 0), since
# `cargo fuzz --help` would exit non-zero when missing and abort under
# $ErrorActionPreference='Stop'.
$fuzzInstalled = (cargo "+$Toolchain" install --list 2>$null) -match 'cargo-fuzz'
if (-not $fuzzInstalled) {
    Write-Host "==> cargo-fuzz not found; installing"
    cargo "+$Toolchain" install cargo-fuzz --locked
}

# Seed each target's corpus from the committed golden vectors so libFuzzer
# mutates outward from real wire frames instead of blind byte flips.
if (Test-Path $vectorsPath) {
    Write-Host "==> Seeding corpora from golden vectors"
    $vectors = Get-Content $vectorsPath -Raw | ConvertFrom-Json
    foreach ($t in $Targets) {
        $corpus = Join-Path $fuzzDir "corpus/$t"
        New-Item -ItemType Directory -Force $corpus | Out-Null
        foreach ($v in $vectors) {
            $bytes = ($v.binary -split '\s+' | ForEach-Object { [Convert]::ToByte($_, 16) })
            [IO.File]::WriteAllBytes((Join-Path $corpus $v.name), [byte[]]$bytes)
        }
    }
}

Push-Location $driverDir
try {
    $failed = @()
    foreach ($t in $Targets) {
        if ($ValidateOnly) {
            # `-runs=0` replays the golden-vector-seeded corpus once with no
            # mutation: a fast, deterministic regression gate.
            Write-Host "==> Validating '$t' against the golden-vector corpus (-runs=0)"
            $runArgs = @("+$Toolchain", 'fuzz', 'run', $t, '--', '-runs=0', '-print_final_stats=1')
        }
        else {
            Write-Host "==> Fuzzing '$t' for ${MaxTotalTimeSeconds}s"
            $runArgs = @("+$Toolchain", 'fuzz', 'run', $t, '--', "-max_total_time=$MaxTotalTimeSeconds", '-print_final_stats=1')
            if ($Workers -gt 0) { $runArgs += "-workers=$Workers"; $runArgs += "-jobs=$Workers" }
        }
        & cargo @runArgs
        if ($LASTEXITCODE -ne 0) {
            $failed += $t
            Write-Host "##vso[task.logissue type=error]Fuzz target '$t' found a crash. Minimize with: cargo +$Toolchain fuzz tmin $t <artifact>"
        }
    }

    # Publish any crash inputs so a failure can be reproduced off-agent.
    $artifactsDir = Join-Path $fuzzDir 'artifacts'
    if (Test-Path $artifactsDir) {
        $crashes = Get-ChildItem $artifactsDir -Recurse -File -ErrorAction SilentlyContinue
        if ($crashes -and $env:SYSTEM_TEAMPROJECTID) {
            Write-Host "##vso[task.logissue type=warning]Fuzz crash artifacts found; published as 'fuzz-crashes'."
            Write-Host "##vso[artifact.upload artifactname=fuzz-crashes]$((Resolve-Path $artifactsDir).Path)"
        }
    }

    if ($failed.Count -gt 0) {
        throw "Fuzz targets reported crashes: $($failed -join ', ')"
    }
    Write-Host "==> All fuzz targets clean."
}
finally {
    Pop-Location
}
