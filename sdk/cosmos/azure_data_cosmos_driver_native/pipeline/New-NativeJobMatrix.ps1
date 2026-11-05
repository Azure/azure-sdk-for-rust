# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0

<#
.SYNOPSIS
    Converts the native driver build matrix into the shared Azure Pipelines
    matrix-generator format.

.DESCRIPTION
    Keeps build-matrix.json as the single source of target metadata while
    producing the configuration consumed by
    eng/common/scripts/job-matrix/Create-JobMatrix.ps1.
#>
[CmdletBinding()]
param(
    [string] $MatrixPath = ([System.IO.Path]::Combine($PSScriptRoot, 'build-matrix.json')),

    [Parameter(Mandatory = $true)]
    [string] $OutputPath
)

Set-StrictMode -Version 3.0
$ErrorActionPreference = 'Stop'

$matrix = Get-Content $MatrixPath -Raw | ConvertFrom-Json
$targets = [ordered]@{}

foreach ($target in $matrix.targets) {
    $agent = switch ($target.goos) {
        'windows' {
            @{
                Pool = 'env:WINDOWSPOOL'
                OSVmImage = 'env:WINDOWSVMIMAGE'
            }
        }
        'linux' {
            @{
                Pool = 'env:LINUXPOOL'
                OSVmImage = 'env:LINUXVMIMAGE'
            }
        }
        'darwin' {
            @{
                Pool = 'env:MACPOOL'
                OSVmImage = 'env:MACVMIMAGEM1'
            }
        }
        default {
            throw "Unsupported GOOS '$($target.goos)' for target '$($target.id)'."
        }
    }

    $targets[$target.id] = [ordered]@{
        TargetId = $target.id
        Triple = $target.triple
        CCompiler = $target.c_compiler
        GoToolchainVersion = $matrix.go_toolchain_version
        Pool = $agent.Pool
        OSVmImage = $agent.OSVmImage
    }
}

if ($targets.Count -eq 0) {
    throw "No targets were found in '$MatrixPath'."
}

$outputDirectory = [System.IO.Path]::GetDirectoryName(
    [System.IO.Path]::GetFullPath($OutputPath)
)
if ($outputDirectory) {
    New-Item -ItemType Directory -Force -Path $outputDirectory | Out-Null
}

[ordered]@{
    displayNames = [ordered]@{}
    matrix = [ordered]@{
        Target = $targets
    }
    include = @()
    exclude = @()
} |
    ConvertTo-Json -Depth 8 |
    Set-Content -Path $OutputPath -Encoding utf8

Write-Host "Native job matrix configuration written to: $OutputPath"
