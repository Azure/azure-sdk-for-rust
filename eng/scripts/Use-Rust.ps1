#!/usr/bin/env pwsh

#Requires -Version 7.0
[CmdletBinding(DefaultParameterSetName = "none")]
param(
    [string]$Toolchain,
    [int]$MaxAttempts = 3
)

$ErrorActionPreference = 'Stop'

. (Join-Path $PSScriptRoot '..' 'common' 'scripts' 'common.ps1')

Invoke-LoggedCommand "rustup --version"

if ($Toolchain -eq 'msrv') {
    Write-Host "Reading MSRV from azure_core"
    $toolchainArg = cargo read-manifest --manifest-path '$RepoRoot/sdk/core/azure_core/Cargo.toml' | ConvertFrom-Json | Select-Object -ExpandProperty rust_version
}
elseif ($Toolchain -eq 'default') {
    # 'rustup default' will install and set the default toolchain, but unlike `rustup install`, it doesn't work
    # without a toolchain argument.  Even though 'rustup install'
    $toolchainArg = ''
}
else {
    $toolchainArg = $toolchain
}

$attempts = 0

while ($true) {
    $attempts++

    Invoke-LoggedCommand "rustup install $toolchainArg"

    if ($?) { break }

    if ($attempts -lt $MaxAttempts) {
        Write-Host "Install failed, attempt $attempts, retrying..."
    }
    else {
        Write-Host "Install failed after $attempts attempts."
        exit 1
    }

    # Failures to update are usually caused by file locks in Windows.
    # Sleep for a few seconds to give the blocking process a chance to release the lock.
    Start-Sleep -Seconds 3
}

if ($Toolchain -eq 'default') {
    $toolchainArg = rustup show active-toolchain -v | Select-Object -First 1
}

Invoke-LoggedCommand "rustup default $toolchainArg"
Invoke-LoggedCommand "rustup show"
