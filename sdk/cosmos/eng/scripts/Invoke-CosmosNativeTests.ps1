# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0
[CmdletBinding()]
param(
    [string]$BuildDirectory = ([System.IO.Path]::Combine($PSScriptRoot, '..', '..', '..', '..', 'target', 'cosmos-native-tests'))
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0
. ([System.IO.Path]::Combine($PSScriptRoot, '..', '..', '..', '..', 'eng', 'common', 'scripts', 'common.ps1'))

$global:LASTEXITCODE = 0
$BuildDirectory = [System.IO.Path]::GetFullPath($BuildDirectory)
$nativeCrateDirectory = ([System.IO.Path]::Combine($RepoRoot, 'sdk', 'cosmos', 'azure_data_cosmos_driver_native'))
$commands = @(
    'cargo test -p azure_data_cosmos_driver_native --all-features'
    "cmake -S '$nativeCrateDirectory' -B '$BuildDirectory' -DCMAKE_BUILD_TYPE=Debug"
    "cmake --build '$BuildDirectory' --config Debug --parallel 2"
)

foreach ($command in $commands) {
    Invoke-LoggedCommand $command -ExecutePath $RepoRoot -GroupOutput -DoNotExitOnFailedExitCode
    if ($LASTEXITCODE -ne 0) {
        throw "Native test setup failed: $command"
    }
}

$nativeTestResults = ([System.IO.Path]::Combine($BuildDirectory, 'results.xml'))
if (Test-Path $nativeTestResults) {
    Remove-Item $nativeTestResults
}
Invoke-LoggedCommand `
    "ctest --test-dir '$BuildDirectory' --build-config Debug --output-on-failure --no-tests=error --timeout 120 --output-junit '$nativeTestResults'" `
    -ExecutePath $RepoRoot -GroupOutput -DoNotExitOnFailedExitCode
$nativeTestExitCode = $LASTEXITCODE

if (-not (Test-Path $nativeTestResults -PathType Leaf)) {
    throw "CTest did not produce the native test results: $nativeTestResults"
}
if ($env:TF_BUILD -eq 'true') {
    # Publish directly because the shared conversion step only handles Cargo JSON.
    Write-Host "##vso[results.publish type=JUnit;runTitle=Cosmos native C ABI;publishRunAttachments=true;]$nativeTestResults"
}
if ($nativeTestExitCode -ne 0) {
    throw "Native C ABI tests failed with exit code $nativeTestExitCode."
}
