# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
# cspell:ignore GOHOSTARCH GOHOSTOS Println

#Requires -Version 7.0

<#
.SYNOPSIS
Builds ordinary and vendored Go consumers of one generated native-driver module.

.DESCRIPTION
Creates a temporary Go module that requires and imports a generated target
module through a local replace directive. The consumer calls cosmos_version()
through cgo, so both builds must resolve a real symbol from the target archive.
It first builds directly from the replacement module, then runs go mod vendor,
verifies the archive was copied to the vendored module root, and builds with
-mod=vendor.

.PARAMETER GeneratedRoot
Root containing the output from New-GoModules.ps1.

.PARAMETER TargetId
Host-compatible target to consume. The default is linux-amd64-glibc.

.PARAMETER CCompiler
C compiler used by cgo. Defaults to the target's build-matrix compiler.
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$GeneratedRoot,

    [string]$TargetId = 'linux-amd64-glibc',

    [string]$CCompiler,

    [string]$MatrixPath = ([System.IO.Path]::Combine($PSScriptRoot, 'build-matrix.json'))
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Invoke-Go {
    param(
        [Parameter(Mandatory = $true)]
        [string[]]$Arguments
    )

    $output = & go @Arguments 2>&1
    if ($LASTEXITCODE -ne 0) {
        throw "go $($Arguments -join ' ') failed with exit code ${LASTEXITCODE}:`n$($output -join "`n")"
    }
    @($output)
}

if (-not (Get-Command go -ErrorAction SilentlyContinue)) {
    throw 'Go is required for the generated-module consumer smoke test.'
}

$matrix = Get-Content $MatrixPath -Raw | ConvertFrom-Json
$target = $matrix.targets | Where-Object id -EQ $TargetId | Select-Object -First 1
if (-not $target) {
    throw "Target '$TargetId' is not present in build-matrix.json."
}

$compiler = if ($CCompiler) { $CCompiler } else { [string]$target.c_compiler }
if (-not $compiler -or
    -not (Get-Command $compiler -CommandType Application -ErrorAction SilentlyContinue)) {
    throw "C compiler '$compiler' is not available for target '$TargetId'."
}

$hostIdentity = @(Invoke-Go -Arguments @('env', 'GOHOSTOS', 'GOHOSTARCH'))
if ($hostIdentity.Count -ne 2 -or
    [string]$hostIdentity[0] -cne [string]$target.goos -or
    [string]$hostIdentity[1] -cne [string]$target.goarch) {
    throw "Target '$TargetId' is not compatible with Go host '$($hostIdentity -join '/')'."
}

$generatedRootPath = (Resolve-Path $GeneratedRoot).Path
$modulePath = "$($matrix.module_root)/$($target.module_path)"
$moduleDirectory = ([System.IO.Path]::Combine(
    $generatedRootPath,
    ($target.module_path -replace '/', [System.IO.Path]::DirectorySeparatorChar)
))
$archivePath = ([System.IO.Path]::Combine($moduleDirectory, $matrix.static_lib_filename))
foreach ($path in @(
    ([System.IO.Path]::Combine($moduleDirectory, 'go.mod')),
    ([System.IO.Path]::Combine($moduleDirectory, $matrix.header_filename)),
    $archivePath
)) {
    if (-not (Test-Path $path -PathType Leaf)) {
        throw "Generated target module input is missing: $path"
    }
}

$consumerSource = @"
// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

package main

/*
const char *cosmos_version(void);
*/
import "C"

import (
	"fmt"

	_ "$modulePath"
)

func main() {
	version := C.cosmos_version()
	if version == nil {
		panic("cosmos_version returned nil")
	}
	fmt.Println(C.GoString(version))
}
"@

$workRoot = ([System.IO.Path]::Combine(
    [System.IO.Path]::GetTempPath(),
    "cosmos-go-consumer-$TargetId-$([guid]::NewGuid())"
))
$savedEnvironment = @{
    CC = $env:CC
    CGO_ENABLED = $env:CGO_ENABLED
    GOARCH = $env:GOARCH
    GOOS = $env:GOOS
    GOTOOLCHAIN = $env:GOTOOLCHAIN
}
try {
    New-Item -ItemType Directory -Path $workRoot -Force | Out-Null
    [System.IO.File]::WriteAllText(
        ([System.IO.Path]::Combine($workRoot, 'go.mod')),
        "module azure-cosmos-driver-consumer-smoke`n`ngo $($matrix.go_version)`n",
        [System.Text.UTF8Encoding]::new($false)
    )
    [System.IO.File]::WriteAllText(
        ([System.IO.Path]::Combine($workRoot, 'main.go')),
        $consumerSource.Replace("`r`n", "`n").Replace("`r", "`n") + "`n",
        [System.Text.UTF8Encoding]::new($false)
    )

    $env:CC = $compiler
    $env:CGO_ENABLED = '1'
    $env:GOOS = [string]$target.goos
    $env:GOARCH = [string]$target.goarch
    $env:GOTOOLCHAIN = 'local'

    Push-Location $workRoot
    try {
        Invoke-Go -Arguments @('mod', 'edit', "-require=$modulePath@v0.0.0") | Out-Null
        Invoke-Go -Arguments @('mod', 'edit', "-replace=$modulePath=$moduleDirectory") | Out-Null

        $executableSuffix = if ($target.goos -eq 'windows') { '.exe' } else { '' }
        Invoke-Go -Arguments @(
            'build',
            '-mod=mod',
            '-trimpath',
            '-o', "consumer-direct$executableSuffix",
            '.'
        ) | Out-Null

        Invoke-Go -Arguments @('mod', 'vendor') | Out-Null
        $vendoredModule = ([System.IO.Path]::Combine(
            $workRoot,
            'vendor',
            ($modulePath -replace '/', [System.IO.Path]::DirectorySeparatorChar)
        ))
        $vendoredArchive = ([System.IO.Path]::Combine(
            $vendoredModule,
            $matrix.static_lib_filename
        ))
        if (-not (Test-Path $vendoredArchive -PathType Leaf)) {
            throw "go mod vendor did not copy the native archive to '$vendoredArchive'."
        }
        $obsoleteVendoredArchive = ([System.IO.Path]::Combine(
            $vendoredModule,
            'native',
            $matrix.static_lib_filename
        ))
        if (Test-Path $obsoleteVendoredArchive) {
            throw "go mod vendor copied an obsolete nested archive: '$obsoleteVendoredArchive'."
        }
        $sourceHash = (Get-FileHash $archivePath -Algorithm SHA256).Hash
        $vendoredHash = (Get-FileHash $vendoredArchive -Algorithm SHA256).Hash
        if ($sourceHash -cne $vendoredHash) {
            throw 'The vendored native archive differs from the generated module archive.'
        }

        Invoke-Go -Arguments @(
            'build',
            '-mod=vendor',
            '-trimpath',
            '-o', "consumer-vendored$executableSuffix",
            '.'
        ) | Out-Null
    }
    finally {
        Pop-Location
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

Write-Host "Direct and vendored Go consumer builds passed for $TargetId."
