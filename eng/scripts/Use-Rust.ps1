#!/usr/bin/env pwsh

#Requires -Version 7.0
[CmdletBinding(DefaultParameterSetName = "none")]
param(
    [string]$Toolchain,
    [int]$MaxAttempts = 3,
    [switch]$SetDefault
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0
. "$PSScriptRoot/../common/scripts/common.ps1"

Write-Host $RepoRoot

$rustupVersion = Invoke-LoggedCommand "rustup --version"
if ($rustupVersion -match 'rustup (\S+)') {
    $rustupVersion = [AzureEngSemanticVersion]::new($matches[1])
    if ($rustupVersion -lt [AzureEngSemanticVersion]::new('1.28.0')) {
        Invoke-LoggedCommand "rustup self update"
    }
}

if ($Toolchain -eq 'msrv') {
    Write-Host "Reading MSRV from azure_core"
    $toolchainArg = cargo read-manifest --manifest-path "$RepoRoot/sdk/core/azure_core/Cargo.toml" | ConvertFrom-Json | Select-Object -ExpandProperty rust_version
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

    Invoke-LoggedCommand "rustup install $toolchainArg --no-self-update"

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

if ($SetDefault) {
    if ($Toolchain -eq 'default') {
        $toolchainArg = rustup show active-toolchain -v | Select-Object -First 1
    }

    Write-Host "Setting default toolchain to $toolchainArg`n"
    Invoke-LoggedCommand "rustup default $toolchainArg"
}

Invoke-LoggedCommand "rustup show"
