# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

<#
.SYNOPSIS
Validates generated Go modules and stages them in an azure-cosmos-driver checkout.

.DESCRIPTION
Verifies SHA256SUMS, requires the 1ES SPDX manifest, rejects unexpected generated
paths, synchronizes the pipeline-owned platform and manifest roots to the exact
generated artifact, validates the Go files, and stages the resulting downstream
changes. The script does not push or open a pull request.
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

$managedRoots = @('windows', 'linux', 'darwin', '_manifest')

# Root-level files (no directory prefix) that the pipeline owns end-to-end. They
# are generated fresh every run, replace any prior copy, and are the only
# non-directory paths permitted at the checkout root.
$managedRootFiles = @('SHA256SUMS', 'provenance.json')

function Test-IsManagedPath {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path
    )

    foreach ($root in $managedRoots) {
        if ($Path.StartsWith("$root/", [StringComparison]::Ordinal)) {
            return $true
        }
    }
    return $managedRootFiles -contains $Path
}

function Test-Is1EsEvidencePath {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path
    )

    $Path.StartsWith('_manifest/', [StringComparison]::Ordinal)
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
$requiredLinkerFlags = @{}
foreach ($managedRootFile in $managedRootFiles) {
    [void]$expectedFiles.Add($managedRootFile)
}

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
    if ($target.PSObject.Properties.Name -contains 'static_runtime_ldflags') {
        $requiredLinkerFlags[$linkPath] = @($target.static_runtime_ldflags) -join ' '
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
$requiredManifestPath = '_manifest/spdx_2.2/manifest.spdx.json'
$manifestFiles = @($actualFiles | Where-Object { Test-Is1EsEvidencePath -Path $_ })
if ($requiredManifestPath -notin $manifestFiles) {
    throw "Generated artifact is missing the required 1ES manifest: $requiredManifestPath"
}
foreach ($manifestFile in $manifestFiles) {
    [void]$expectedFiles.Add($manifestFile)
}

$missingFiles = @($expectedFiles | Where-Object { $_ -notin $actualFiles })
$unexpectedFiles = @($actualFiles | Where-Object { -not $expectedFiles.Contains($_) })
if ($missingFiles.Count -gt 0 -or $unexpectedFiles.Count -gt 0) {
    throw "Generated artifact layout does not match build-matrix.json. Missing: [$($missingFiles -join ', ')]; unexpected: [$($unexpectedFiles -join ', ')]."
}

foreach ($entry in $requiredLinkerFlags.GetEnumerator()) {
    $linkFile = Join-Path $generatedRootPath $entry.Key
    $linkFileContents = Get-Content $linkFile -Raw
    if (-not $linkFileContents.Contains($entry.Value)) {
        throw "Generated linker file '$($entry.Key)' is missing required static runtime flags: $($entry.Value)"
    }
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

foreach ($root in $managedRoots) {
    $managedRootPath = Join-Path $checkoutRootPath $root
    if (-not (Test-Path $managedRootPath)) {
        continue
    }

    $managedRootItem = Get-Item $managedRootPath -Force
    if (-not $managedRootItem.PSIsContainer) {
        throw "Managed generated root is not a directory: '$root'."
    }
    if ($managedRootItem.LinkType -or ($managedRootItem.Attributes -band [IO.FileAttributes]::ReparsePoint)) {
        throw "Managed generated root is a link or reparse point: '$root'."
    }
    $linkedManagedPaths = @(
        Get-ChildItem $managedRootPath -Recurse -Force |
            Where-Object { $_.LinkType -or ($_.Attributes -band [IO.FileAttributes]::ReparsePoint) } |
            ForEach-Object { Get-NormalizedRelativePath -Root $checkoutRootPath -Path $_.FullName }
    )
    if ($linkedManagedPaths.Count -gt 0) {
        throw "Managed generated root contains links or reparse points: $($linkedManagedPaths -join ', ')"
    }
    Remove-Item $managedRootPath -Recurse -Force
}

foreach ($managedRootFile in $managedRootFiles) {
    $checkoutManagedFilePath = Join-Path $checkoutRootPath $managedRootFile
    if (Test-Path $checkoutManagedFilePath) {
        $managedFileItem = Get-Item $checkoutManagedFilePath -Force
        if (-not $managedFileItem.PSIsContainer -and
            -not $managedFileItem.LinkType -and
            -not ($managedFileItem.Attributes -band [IO.FileAttributes]::ReparsePoint)) {
            Remove-Item $checkoutManagedFilePath -Force
        }
        else {
            throw "Managed generated path is not a regular file: '$managedRootFile'."
        }
    }
}

foreach ($relativePath in $expectedFiles) {
    $source = Join-Path $generatedRootPath $relativePath
    $destination = Join-Path $checkoutRootPath $relativePath
    $destinationDirectory = Split-Path -Parent $destination
    New-Item -ItemType Directory -Force -Path $destinationDirectory | Out-Null
    Copy-Item $source $destination -Force
}

$destinationFiles = @(
    foreach ($root in $managedRoots) {
        $managedRootPath = Join-Path $checkoutRootPath $root
        if (Test-Path $managedRootPath) {
            Get-ChildItem $managedRootPath -Recurse -File |
                ForEach-Object { Get-NormalizedRelativePath -Root $checkoutRootPath -Path $_.FullName }
        }
    }
    foreach ($managedRootFile in $managedRootFiles) {
        if (Test-Path (Join-Path $checkoutRootPath $managedRootFile) -PathType Leaf) {
            $managedRootFile
        }
    }
)
$missingDestinationFiles = @($expectedFiles | Where-Object { $_ -notin $destinationFiles })
$unexpectedDestinationFiles = @($destinationFiles | Where-Object { -not $expectedFiles.Contains($_) })
if ($missingDestinationFiles.Count -gt 0 -or $unexpectedDestinationFiles.Count -gt 0) {
    throw "Downstream generated layout does not match the artifact. Missing: [$($missingDestinationFiles -join ', ')]; unexpected: [$($unexpectedDestinationFiles -join ', ')]."
}
foreach ($relativePath in $expectedFiles) {
    $sourceHash = (Get-FileHash (Join-Path $generatedRootPath $relativePath) -Algorithm SHA256).Hash
    $destinationHash = (Get-FileHash (Join-Path $checkoutRootPath $relativePath) -Algorithm SHA256).Hash
    if ($sourceHash -cne $destinationHash) {
        throw "Downstream generated file differs from the artifact: '$relativePath'."
    }
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
        Where-Object { -not (Test-IsManagedPath -Path $_) }
)
if ($unexpectedChanges.Count -gt 0) {
    throw "Downstream checkout contains changes outside the generated file allowlist: $($unexpectedChanges -join ', ')"
}

Write-Host "Prepared $($modulePaths.Count) Go modules and staged $($changedPaths.Count) downstream paths."
