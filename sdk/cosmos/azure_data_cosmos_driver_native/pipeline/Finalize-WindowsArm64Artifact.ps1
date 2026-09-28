# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.
# cspell:ignore advapi Dumpbin msvcp ncrypt ntdll secur ucrtbase userenv vcruntime

#Requires -Version 7.0

<#
.SYNOPSIS
    Validates and finalizes the signed Windows ARM64 native-driver payload.

.DESCRIPTION
    This script must run after Authenticode signing. It verifies the PE machine,
    exports, imports, and signature before hashing the final DLL bytes. It then
    updates the native-interface metadata and writes SHA256SUMS. The PDB remains
    a diagnostics asset and is never treated as a runtime dependency.
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string] $ArtifactRoot,

    [string] $Dumpbin = 'dumpbin.exe'
)

Set-StrictMode -Version 3.0
$ErrorActionPreference = 'Stop'

$metadataFilename = 'rust-driver-native-interface-metadata.json'
$metadataPath = Join-Path $ArtifactRoot $metadataFilename
if (-not (Test-Path $metadataPath -PathType Leaf)) {
    throw "Required metadata was not found: $metadataPath"
}

$metadata = Get-Content $metadataPath -Raw | ConvertFrom-Json
if (
    $metadata.publication_kind -ne 'windows-dll' -or
    $metadata.triple -ne 'aarch64-pc-windows-msvc' -or
    $metadata.goos -ne 'windows' -or
    $metadata.goarch -ne 'arm64'
) {
    throw 'Metadata does not describe the Windows ARM64 MSVC DLL payload.'
}

$dllPath = Join-Path $ArtifactRoot ([string]$metadata.dynamic_library.filename)
$headerPath = Join-Path $ArtifactRoot ([string]$metadata.header.filename)
$pdbPath = Join-Path $ArtifactRoot ([string]$metadata.symbols.filename)
foreach ($requiredPath in @($dllPath, $headerPath, $pdbPath)) {
    if (-not (Test-Path $requiredPath -PathType Leaf)) {
        throw "Required Windows ARM64 artifact was not found: $requiredPath"
    }
}

$dumpbinCommand = Get-Command $Dumpbin -ErrorAction SilentlyContinue
if (-not $dumpbinCommand) {
    throw "Required PE inspection tool is unavailable: $Dumpbin"
}

function Invoke-Dumpbin([string[]] $Arguments, [string] $Description) {
    $output = @(& $Dumpbin @Arguments 2>&1)
    if ($LASTEXITCODE -ne 0) {
        throw "$Description failed with exit code $LASTEXITCODE`n$($output -join "`n")"
    }
    $output | ForEach-Object { [string]$_ }
}

$headers = @(Invoke-Dumpbin @('/headers', $dllPath) 'PE header inspection')
if (-not ($headers -match '(?i)\bAA64 machine \(ARM64\)')) {
    throw 'azurecosmosdriver.dll is not a PE ARM64 (0xAA64) image.'
}
if (-not ($headers -match '(?i)\bazurecosmosdriver\.pdb\b')) {
    throw 'The PE debug directory does not reference azurecosmosdriver.pdb.'
}

$exportOutput = @(Invoke-Dumpbin @('/exports', $dllPath) 'PE export inspection')
$exports = @(
    $exportOutput |
        ForEach-Object {
            if ($_ -match '^\s+\d+\s+[0-9A-F]+\s+[0-9A-F]+\s+(\S+)\s*$') {
                $Matches[1]
            }
        } |
        Sort-Object -Unique
)
$requiredExports = @(
    'COSMOS_BUILD_IDENTIFIER'
    'cosmos_abi_version'
    'cosmos_bytes_free_ref'
    'cosmos_driver_options_config_default_init'
    'cosmos_operation_options_default_init'
    'cosmos_runtime_options_default_init'
    'cosmos_version'
)
foreach ($requiredExport in $requiredExports) {
    if ($requiredExport -notin $exports) {
        throw "Required DLL export is missing: $requiredExport"
    }
}
$headerText = Get-Content $headerPath -Raw
$headerExports = @(
    [regex]::Matches($headerText, '\b(cosmos_[a-z0-9_]+)\s*\(') |
        ForEach-Object { $_.Groups[1].Value } |
        Sort-Object -Unique
)
$expectedExports = @('COSMOS_BUILD_IDENTIFIER') + $headerExports
$unexpectedExports = @($exports | Where-Object { $_ -notin $expectedExports })
if ($unexpectedExports) {
    throw "Unexpected DLL exports: $($unexpectedExports -join ', ')"
}
$missingHeaderExports = @($headerExports | Where-Object { $_ -notin $exports })
if ($missingHeaderExports) {
    throw "Generated-header exports missing from DLL: $($missingHeaderExports -join ', ')"
}

$importOutput = @(Invoke-Dumpbin @('/imports', $dllPath) 'PE import inspection')
$imports = @(
    $importOutput |
        ForEach-Object {
            if ($_ -match '^\s+([A-Za-z0-9._-]+\.dll)\s*$') {
                $Matches[1].ToLowerInvariant()
            }
        } |
        Sort-Object -Unique
)
if (-not $imports) {
    throw 'No imported DLL inventory was discovered.'
}
$nonSystemImports = @(
    $imports | Where-Object {
        $_ -notmatch '^(api-ms-win-|ext-ms-win-)' -and
        $_ -notin @(
            'advapi32.dll', 'bcrypt.dll', 'crypt32.dll', 'kernel32.dll',
            'ncrypt.dll', 'ntdll.dll', 'secur32.dll', 'userenv.dll',
            'ws2_32.dll', 'vcruntime140.dll', 'vcruntime140_1.dll',
            'msvcp140.dll', 'ucrtbase.dll'
        )
    }
)
if ($nonSystemImports) {
    throw "Unexpected non-system DLL imports: $($nonSystemImports -join ', ')"
}

$signature = Get-AuthenticodeSignature -LiteralPath $dllPath
if ($signature.Status -ne [System.Management.Automation.SignatureStatus]::Valid) {
    throw "Authenticode verification failed: $($signature.Status) - $($signature.StatusMessage)"
}
if (-not $signature.TimeStamperCertificate) {
    throw 'Authenticode signature does not contain an approved timestamp.'
}

$dllSha = (Get-FileHash $dllPath -Algorithm SHA256).Hash.ToLowerInvariant()
$headerSha = (Get-FileHash $headerPath -Algorithm SHA256).Hash.ToLowerInvariant()
$pdbSha = (Get-FileHash $pdbPath -Algorithm SHA256).Hash.ToLowerInvariant()
if ($headerSha -ne $metadata.header.sha256) {
    throw 'Generated header hash changed after the native build.'
}
if ($pdbSha -ne $metadata.symbols.sha256) {
    throw 'PDB hash changed after the native build.'
}

$metadata.dynamic_library.sha256 = $dllSha
$metadata.dynamic_library.authenticode_verified = $true
$metadata.dynamic_library | Add-Member -NotePropertyName pe_machine -NotePropertyValue 'ARM64'
$metadata.dynamic_library | Add-Member -NotePropertyName pe_machine_code -NotePropertyValue '0xAA64'
$metadata.dynamic_library | Add-Member -NotePropertyName exports -NotePropertyValue $exports
$metadata.dynamic_library | Add-Member -NotePropertyName header_exports -NotePropertyValue $headerExports
$metadata.dynamic_library | Add-Member -NotePropertyName required_exports -NotePropertyValue $requiredExports
$metadata.dynamic_library | Add-Member -NotePropertyName imported_dlls -NotePropertyValue $imports
$metadata.dynamic_library | Add-Member -NotePropertyName signing -NotePropertyValue ([ordered]@{
    status = [string]$signature.Status
    signer_subject = [string]$signature.SignerCertificate.Subject
    timestamp_subject = [string]$signature.TimeStamperCertificate.Subject
})
$metadata | Add-Member -NotePropertyName finalized_utc `
    -NotePropertyValue (Get-Date).ToUniversalTime().ToString('o')
$metadata | ConvertTo-Json -Depth 10 | Set-Content $metadataPath -Encoding utf8

$checksums = @(
    "$dllSha  $([IO.Path]::GetFileName($dllPath))"
    "$headerSha  $([IO.Path]::GetFileName($headerPath))"
    "$pdbSha  $([IO.Path]::GetFileName($pdbPath))"
    "$((Get-FileHash $metadataPath -Algorithm SHA256).Hash.ToLowerInvariant())  $metadataFilename"
)
$checksums | Set-Content (Join-Path $ArtifactRoot 'SHA256SUMS') -Encoding utf8

Write-Host "Finalized signed Windows ARM64 payload: $ArtifactRoot"
