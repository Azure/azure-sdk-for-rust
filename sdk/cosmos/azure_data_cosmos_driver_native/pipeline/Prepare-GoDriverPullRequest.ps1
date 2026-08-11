# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

<#
.SYNOPSIS
Validates generated Go modules and stages them in an azure-cosmos-driver checkout.

.DESCRIPTION
Verifies SHA256SUMS, rejects unexpected generated paths, replaces only the module
paths declared in build-matrix.json, validates the Go files, and stages the
resulting downstream changes. The script does not push or open a pull request.
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$GeneratedRoot,

    [Parameter(Mandatory = $true)]
    [string]$CheckoutRoot,

    [string]$MatrixPath = (Join-Path $PSScriptRoot 'build-matrix.json')
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Invoke-CheckedCommand {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Command,

        [Parameter(Mandatory = $true)]
        [string[]]$Arguments
    )

    & $Command @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "'$Command $($Arguments -join ' ')' failed with exit code $LASTEXITCODE."
    }
}

function Get-NormalizedRelativePath {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Root,

        [Parameter(Mandatory = $true)]
        [string]$Path
    )

    [IO.Path]::GetRelativePath($Root, $Path).Replace('\', '/')
}

$generatedRootPath = (Resolve-Path $GeneratedRoot).Path
$checkoutRootPath = (Resolve-Path $CheckoutRoot).Path
$matrix = Get-Content -Raw $MatrixPath | ConvertFrom-Json

if (-not (Test-Path (Join-Path $checkoutRootPath '.git'))) {
    throw "CheckoutRoot is not a Git checkout: $checkoutRootPath"
}

$modulePaths = @(
    $matrix.targets.module_path |
        Sort-Object -Unique |
        ForEach-Object { $_.Replace('\', '/') }
)
$expectedFiles = [Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
$expectedLibraries = [Collections.Generic.List[string]]::new()
[void]$expectedFiles.Add('SHA256SUMS')

foreach ($modulePath in $modulePaths) {
    [void]$expectedFiles.Add("$modulePath/go.mod")
    [void]$expectedFiles.Add("$modulePath/$($matrix.header_filename)")
}

foreach ($target in $matrix.targets) {
    $modulePath = $target.module_path.Replace('\', '/')
    $nativeSubdir = [string]$target.native_subdir
    $nativePath = if ($nativeSubdir) { "$modulePath/native/$nativeSubdir" } else { "$modulePath/native" }
    $linkSuffix = if ($nativeSubdir) { "_$nativeSubdir" } else { '' }

    $libraryPath = "$nativePath/$($matrix.static_lib_filename)"
    $headerPath = "$nativePath/$($matrix.header_filename)"
    $linkPath = "$modulePath/link_$($target.goos)_$($target.goarch)$linkSuffix.go"
    foreach ($path in @($libraryPath, $headerPath, $linkPath)) {
        [void]$expectedFiles.Add($path)
    }
    $expectedLibraries.Add($libraryPath)
}

$linkedArtifactPaths = @(
    Get-ChildItem $generatedRootPath -Recurse -Force |
        Where-Object { $_.LinkType -or ($_.Attributes -band [IO.FileAttributes]::ReparsePoint) } |
        ForEach-Object { Get-NormalizedRelativePath -Root $generatedRootPath -Path $_.FullName }
)
if ($linkedArtifactPaths.Count -gt 0) {
    throw "Generated artifact contains links or reparse points: $($linkedArtifactPaths -join ', ')"
}

$actualFiles = @(
    Get-ChildItem $generatedRootPath -Recurse -File |
        ForEach-Object { Get-NormalizedRelativePath -Root $generatedRootPath -Path $_.FullName }
)
$missingFiles = @($expectedFiles | Where-Object { $_ -notin $actualFiles })
$unexpectedFiles = @($actualFiles | Where-Object { -not $expectedFiles.Contains($_) })
if ($missingFiles.Count -gt 0 -or $unexpectedFiles.Count -gt 0) {
    throw "Generated artifact layout does not match build-matrix.json. Missing: [$($missingFiles -join ', ')]; unexpected: [$($unexpectedFiles -join ', ')]."
}

$checksumPath = Join-Path $generatedRootPath 'SHA256SUMS'
$verifiedLibraries = [Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
foreach ($line in Get-Content $checksumPath) {
    if ($line -notmatch '^([0-9a-fA-F]{64})  (.+)$') {
        throw "Invalid SHA256SUMS entry: '$line'"
    }

    $expectedHash = $Matches[1].ToLowerInvariant()
    $relativePath = $Matches[2].Replace('\', '/')
    if ([IO.Path]::IsPathRooted($relativePath) -or $relativePath.Split('/') -contains '..') {
        throw "SHA256SUMS contains an unsafe path: '$relativePath'"
    }
    if (-not $verifiedLibraries.Add($relativePath)) {
        throw "SHA256SUMS contains a duplicate path: '$relativePath'"
    }

    $libraryPath = Join-Path $generatedRootPath $relativePath
    if (-not (Test-Path $libraryPath -PathType Leaf)) {
        throw "SHA256SUMS references a missing file: '$relativePath'"
    }

    $actualHash = (Get-FileHash $libraryPath -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($actualHash -ne $expectedHash) {
        throw "SHA256 mismatch for '$relativePath'."
    }
}

$missingChecksums = @($expectedLibraries | Where-Object { -not $verifiedLibraries.Contains($_) })
$unexpectedChecksums = @($verifiedLibraries | Where-Object { $_ -notin $expectedLibraries })
if ($missingChecksums.Count -gt 0 -or $unexpectedChecksums.Count -gt 0) {
    throw "SHA256SUMS does not match the generated libraries. Missing: [$($missingChecksums -join ', ')]; unexpected: [$($unexpectedChecksums -join ', ')]."
}

foreach ($relativePath in $expectedFiles) {
    $source = Join-Path $generatedRootPath $relativePath
    $destination = Join-Path $checkoutRootPath $relativePath
    $destinationDirectory = Split-Path -Parent $destination
    New-Item -ItemType Directory -Force -Path $destinationDirectory | Out-Null
    Copy-Item $source $destination -Force
}

$goFiles = @(
    $modulePaths |
        ForEach-Object { Get-ChildItem (Join-Path $checkoutRootPath $_) -Recurse -File -Filter '*.go' }
)
$formatDiff = [Collections.Generic.List[string]]::new()
foreach ($goFile in $goFiles) {
    $fileDiff = @(& gofmt -d $goFile.FullName)
    if ($LASTEXITCODE -ne 0) {
        throw "gofmt failed for '$($goFile.FullName)' with exit code $LASTEXITCODE."
    }
    $formatDiff.AddRange([string[]]$fileDiff)
}
if ($formatDiff.Count -gt 0) {
    throw "Generated Go files are not formatted:`n$($formatDiff -join [Environment]::NewLine)"
}

foreach ($modulePath in $modulePaths) {
    Push-Location (Join-Path $checkoutRootPath $modulePath)
    try {
        Invoke-CheckedCommand -Command 'go' -Arguments @('mod', 'edit', '-json')
    }
    finally {
        Pop-Location
    }
}

$hostModule = Join-Path $checkoutRootPath 'linux/amd64'
if (-not (Test-Path $hostModule -PathType Container)) {
    throw "The Linux AMD64 module required for host validation is missing."
}
Push-Location $hostModule
try {
    Invoke-CheckedCommand -Command 'go' -Arguments @('test', './...')
}
finally {
    Pop-Location
}

Invoke-CheckedCommand -Command 'git' -Arguments @('-C', $checkoutRootPath, 'add', '-A')
$changedPaths = @(
    & git -C $checkoutRootPath diff --cached --name-only |
        ForEach-Object { $_.Replace('\', '/') }
)
if ($LASTEXITCODE -ne 0) {
    throw "Unable to inspect staged downstream changes."
}

$unexpectedChanges = @(
    $changedPaths |
        Where-Object { -not $expectedFiles.Contains($_) }
)
if ($unexpectedChanges.Count -gt 0) {
    throw "Downstream checkout contains changes outside the generated file allowlist: $($unexpectedChanges -join ', ')"
}

Write-Host "Prepared $($modulePaths.Count) Go modules and staged $($changedPaths.Count) downstream paths."
