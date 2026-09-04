# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0

<#
.SYNOPSIS
    Rehearses the Rust driver native interface supply chain on a local machine.

.DESCRIPTION
    Produces a deliberately non-production release bundle containing the native
    libraries, C header, Cargo-derived metadata, an SPDX SBOM,
    cargo-auditable read-back evidence, SHA256SUMS, and local test-signing
    evidence.

    The script also generates the Azure/azure-cosmos-driver module layout. With
    PrepareGoPr enabled, it clones that repository, creates a local branch,
    overlays the generated files, runs the target Go test, and creates a
    local-only commit and PR preview. It never pushes or opens a remote PR.

    Local SBOMs and signatures demonstrate mechanics only. They do
    not claim Microsoft 1ES, ESRP, Azure Trusted Signing, or Apple trust.

.PARAMETER TargetId
    Matrix target to rehearse. The default is windows-amd64.

.PARAMETER PrepareGoPr
    Prepare a local-only Azure/azure-cosmos-driver branch and commit.

.PARAMETER SkipTestSigning
    Do not apply a disposable self-signed certificate to the Windows DLL.

.EXAMPLE
    ./Invoke-LocalSupplyChain.ps1
#>
[CmdletBinding()]
param(
    [string] $TargetId = 'windows-amd64',
    [bool] $PrepareGoPr = $true,
    [switch] $SkipTestSigning
)

Set-StrictMode -Version 3.0
$ErrorActionPreference = 'Stop'

$PipelineDir = $PSScriptRoot
$CrateDir = Split-Path -Parent $PipelineDir
$RepoRoot = (Resolve-Path ([System.IO.Path]::Combine($CrateDir, '..', '..', '..'))).Path
$MatrixPath = ([System.IO.Path]::Combine($PipelineDir, 'build-matrix.json'))
$RunId = (Get-Date).ToUniversalTime().ToString('yyyyMMddTHHmmssZ')
$RunRoot = ([System.IO.Path]::Combine($PipelineDir, 'artifacts', 'local-rehearsal', $RunId))
$ArtifactRoot = ([System.IO.Path]::Combine($RunRoot, 'native'))
$TargetArtifactDir = ([System.IO.Path]::Combine($ArtifactRoot, $TargetId))
$GoOutputRoot = ([System.IO.Path]::Combine($RunRoot, 'azure-cosmos-driver-output'))
$GoCheckout = ([System.IO.Path]::Combine($RunRoot, 'azure-cosmos-driver-pr'))
$MetadataFilename = 'rust-driver-native-interface-metadata.json'

function Assert-Command([string] $Name) {
    if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
        throw "Required command is not installed or not on PATH: $Name"
    }
}

function Invoke-Checked([string] $Command, [string[]] $Arguments) {
    & $Command @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "$Command failed with exit code $LASTEXITCODE"
    }
}

function Get-RelativePath([string] $BasePath, [string] $Path) {
    ([System.IO.Path]::GetRelativePath($BasePath, $Path)) -replace '\\', '/'
}

function Get-SourceTreeEvidence([string] $Root) {
    $statusLines = @(& git -C $Root status --porcelain=v1 --untracked-files=all)
    if ($LASTEXITCODE -ne 0) { throw 'Unable to inspect the source tree status.' }

    $sourceFiles = @(& git -C $Root ls-files --cached --others --exclude-standard)
    if ($LASTEXITCODE -ne 0) { throw 'Unable to enumerate the source tree.' }

    $entries = @($sourceFiles | Sort-Object -Unique | ForEach-Object {
        $relativePath = $_ -replace '\\', '/'
        $fullPath = [System.IO.Path]::Combine($Root, $_)
        if (Test-Path -LiteralPath $fullPath -PathType Leaf) {
            "$((Get-FileHash -LiteralPath $fullPath -Algorithm SHA256).Hash.ToLowerInvariant())  $relativePath"
        } else {
            "deleted  $relativePath"
        }
    })
    $snapshotBytes = [System.Text.Encoding]::UTF8.GetBytes(
        (($entries -join "`n") + "`n")
    )

    [ordered]@{
        clean = ($statusLines.Count -eq 0)
        file_count = $entries.Count
        sha256 = [System.Convert]::ToHexString(
            [System.Security.Cryptography.SHA256]::HashData($snapshotBytes)
        ).ToLowerInvariant()
    }
}

Assert-Command 'cargo'
Assert-Command 'git'
Assert-Command 'go'
Assert-Command 'rust-audit-info'
Assert-Command 'sbom-tool'

$matrix = Get-Content $MatrixPath -Raw | ConvertFrom-Json
$row = $matrix.targets | Where-Object id -EQ $TargetId | Select-Object -First 1
if (-not $row) { throw "Unknown target: $TargetId" }
$cCompiler = $row.c_compiler
$sourceTreeEvidence = Get-SourceTreeEvidence $RepoRoot

if ($row.goos -eq 'windows') {
    if ($row.triple -ne 'x86_64-pc-windows-gnu') {
        throw "The local rehearsal has no validated MinGW linker for $($row.triple). Keep windows-arm64 gated until its GNU or MSVC fallback toolchain is ratified."
    }
    $mingwBin = 'C:\msys64\mingw64\bin'
    if (-not (Get-Command 'gcc' -ErrorAction SilentlyContinue)) {
        if (-not (Test-Path ([System.IO.Path]::Combine($mingwBin, 'gcc.exe')))) {
            throw "MinGW GCC was not found for $TargetId."
        }
        $env:PATH = "$mingwBin;$env:PATH"
    }
    $cCompiler = 'gcc'
}

Write-Host "Building $TargetId with cargo-auditable"
& ([System.IO.Path]::Combine($PipelineDir, 'Build-NativeMatrix.ps1')) `
    -TargetId $TargetId `
    -OutputRoot $ArtifactRoot `
    -CCompiler $cCompiler
if ($LASTEXITCODE -ne 0) {
    throw "Build-NativeMatrix.ps1 failed with exit code $LASTEXITCODE"
}

$metadataPath = ([System.IO.Path]::Combine($TargetArtifactDir, $MetadataFilename))
$metadata = Get-Content $metadataPath -Raw | ConvertFrom-Json
$staticLibraryPath = ([System.IO.Path]::Combine(
    $TargetArtifactDir,
    $metadata.static_library.filename
))
$dynamicLibraryPath = ([System.IO.Path]::Combine(
    $TargetArtifactDir,
    $metadata.dynamic_library.filename
))
$headerPath = ([System.IO.Path]::Combine($TargetArtifactDir, $matrix.header_filename))

$signingDir = ([System.IO.Path]::Combine($TargetArtifactDir, 'signing'))
$auditDir = ([System.IO.Path]::Combine($TargetArtifactDir, 'audit'))
$validationDir = ([System.IO.Path]::Combine($TargetArtifactDir, 'validation'))
New-Item -ItemType Directory -Force -Path @(
    $signingDir,
    $auditDir,
    $validationDir
) | Out-Null

$unsignedDynamicSha = (Get-FileHash $dynamicLibraryPath -Algorithm SHA256).Hash.ToLowerInvariant()
$signingEvidence = [ordered]@{
    mode = 'not-applicable'
    production_trust = $false
    artifact = Get-RelativePath $TargetArtifactDir $dynamicLibraryPath
    unsigned_sha256 = $unsignedDynamicSha
}

if ($row.goos -eq 'windows' -and -not $SkipTestSigning) {
    Write-Host 'Applying a disposable local test signature to the Windows DLL'
    $certificate = New-SelfSignedCertificate `
        -Type CodeSigningCert `
        -Subject 'CN=Azure Cosmos Driver Local Rehearsal - NOT FOR RELEASE' `
        -CertStoreLocation 'Cert:\CurrentUser\My' `
        -NotAfter (Get-Date).AddDays(1)
    try {
        $signature = Set-AuthenticodeSignature `
            -FilePath $dynamicLibraryPath `
            -Certificate $certificate `
            -HashAlgorithm SHA256
        if (-not $signature.SignerCertificate) {
            throw 'The DLL did not receive a test Authenticode signature.'
        }
        $signingEvidence = [ordered]@{
            mode = 'local-self-signed-authenticode'
            production_trust = $false
            artifact = Get-RelativePath $TargetArtifactDir $dynamicLibraryPath
            certificate_subject = $signature.SignerCertificate.Subject
            certificate_thumbprint = $signature.SignerCertificate.Thumbprint
            signature_attached = $true
            verification_status = [string]$signature.Status
            trust_validation_expected = $false
            unsigned_sha256 = $unsignedDynamicSha
            signed_sha256 = (Get-FileHash $dynamicLibraryPath -Algorithm SHA256).Hash.ToLowerInvariant()
            explanation = 'Signature attachment only. Trust validation is not expected for the disposable self-signed certificate; production uses Microsoft-managed signing.'
        }
    }
    finally {
        Remove-Item "Cert:\CurrentUser\My\$($certificate.Thumbprint)" -Force
    }
}
elseif ($row.goos -eq 'linux') {
    $signingEvidence.explanation = 'Linux has no expected platform-native artifact signing standard.'
}
elseif ($row.goos -eq 'darwin') {
    $signingEvidence.explanation = 'Production requires Apple Developer ID signing and notarization on macOS.'
}
else {
    $signingEvidence.explanation = 'Test signing was explicitly skipped.'
}

$signingEvidencePath = ([System.IO.Path]::Combine($signingDir, 'local-signing-evidence.json'))
$signingEvidence | ConvertTo-Json -Depth 6 |
    Set-Content -Path $signingEvidencePath -Encoding utf8

$metadata.dynamic_library | Add-Member `
    -NotePropertyName unsigned_sha256 `
    -NotePropertyValue $metadata.dynamic_library.sha256 `
    -Force
$metadata.dynamic_library.sha256 = (
    Get-FileHash $dynamicLibraryPath -Algorithm SHA256
).Hash.ToLowerInvariant()
$metadata.dynamic_library | Add-Member `
    -NotePropertyName signing `
    -NotePropertyValue $signingEvidence.mode `
    -Force
$metadata | Add-Member -NotePropertyName release_trust -NotePropertyValue 'local-rehearsal' -Force
$metadata | Add-Member -NotePropertyName source_tree -NotePropertyValue $sourceTreeEvidence -Force
$metadata | ConvertTo-Json -Depth 8 |
    Set-Content -Path $metadataPath -Encoding utf8

Write-Host 'Reading cargo-auditable metadata from the dynamic library'
$dynamicAuditPath = ([System.IO.Path]::Combine($auditDir, 'dynamic-library-auditable.json'))
$dynamicAuditOutput = & rust-audit-info $dynamicLibraryPath 2>&1
if ($LASTEXITCODE -ne 0) {
    throw "rust-audit-info failed for $dynamicLibraryPath`n$($dynamicAuditOutput -join "`n")"
}
$dynamicAuditOutput | ConvertFrom-Json | ConvertTo-Json -Depth 100 -Compress |
    Set-Content -Path $dynamicAuditPath -Encoding utf8

$staticAuditOutput = & rust-audit-info $staticLibraryPath 2>&1
$staticAuditEvidence = [ordered]@{
    artifact = Get-RelativePath $TargetArtifactDir $staticLibraryPath
    readback_supported = ($LASTEXITCODE -eq 0)
    tool = 'rust-audit-info'
    result = ($staticAuditOutput -join "`n")
    explanation = if ($LASTEXITCODE -eq 0) {
        'Embedded dependency metadata was recovered from the static archive.'
    } else {
        'rust-audit-info treats the static archive as non-executable; verify the published archive with SHA256SUMS and the signed 1ES SPDX manifest.'
    }
}
$staticAuditPath = ([System.IO.Path]::Combine($auditDir, 'static-library-auditable-status.json'))
$staticAuditEvidence | ConvertTo-Json -Depth 6 |
    Set-Content -Path $staticAuditPath -Encoding utf8

$primaryArtifacts = @(
    $staticLibraryPath,
    $dynamicLibraryPath,
    $headerPath,
    $metadataPath,
    $dynamicAuditPath,
    $staticAuditPath,
    $signingEvidencePath
)
$hashFiles = @($primaryArtifacts)
$sha256Path = ([System.IO.Path]::Combine($TargetArtifactDir, 'SHA256SUMS'))
$hashFiles |
    Sort-Object |
    ForEach-Object {
        $hash = (Get-FileHash $_ -Algorithm SHA256).Hash.ToLowerInvariant()
        "$hash  $(Get-RelativePath $TargetArtifactDir $_)"
    } |
    Set-Content -Path $sha256Path -Encoding utf8

Write-Host 'Generating and validating the local Microsoft SPDX 2.2 SBOM'
Invoke-Checked 'sbom-tool' @(
    'generate',
    '-b', $TargetArtifactDir,
    '-bc', $RepoRoot,
    '-pn', 'azurecosmosdriver',
    '-pv', $metadata.native_interface_version,
    '-ps', 'Microsoft',
    '-nsb', 'https://azure.github.io/azure-sdk',
    '-D', 'true'
)
$spdxValidationPath = ([System.IO.Path]::Combine($validationDir, 'spdx-validation.json'))
Invoke-Checked 'sbom-tool' @(
    'validate',
    '-b', $TargetArtifactDir,
    '-o', $spdxValidationPath,
    '-mi', 'SPDX:2.2'
)

Write-Host 'Generating the Azure/azure-cosmos-driver module layout'
& ([System.IO.Path]::Combine($PipelineDir, 'New-GoModules.ps1')) `
    -ArtifactRoot $ArtifactRoot `
    -OutputRoot $GoOutputRoot `
    -TargetId $TargetId
if ($LASTEXITCODE -ne 0) {
    throw "New-GoModules.ps1 failed with exit code $LASTEXITCODE"
}

$releaseMetadataDir = ([System.IO.Path]::Combine(
    $GoOutputRoot,
    'release-metadata',
    "v$($metadata.native_interface_version)",
    $TargetId
))
New-Item -ItemType Directory -Force -Path $releaseMetadataDir | Out-Null
Copy-Item $metadataPath, $sha256Path, $signingEvidencePath `
    -Destination $releaseMetadataDir -Force
Copy-Item $auditDir, ([System.IO.Path]::Combine($TargetArtifactDir, '_manifest')) `
    -Destination $releaseMetadataDir -Recurse -Force

$moduleDir = ([System.IO.Path]::Combine(
    $GoOutputRoot,
    ($row.module_path -replace '/', [System.IO.Path]::DirectorySeparatorChar)
))
$goTestArgs = @('test', './...')
Push-Location $moduleDir
try {
    $env:CGO_ENABLED = '1'
    Invoke-Checked 'go' $goTestArgs
}
finally { Pop-Location }

$prPreviewPath = ([System.IO.Path]::Combine($RunRoot, 'LOCAL_PR_PREVIEW.md'))
if ($PrepareGoPr) {
    Write-Host 'Cloning Azure/azure-cosmos-driver for a local-only PR rehearsal'
    Invoke-Checked 'git' @(
        '-c', 'core.longpaths=true',
        'clone',
        '--depth', '1',
        'https://github.com/Azure/azure-cosmos-driver.git',
        $GoCheckout
    )
    Invoke-Checked 'git' @('-C', $GoCheckout, 'config', 'core.longpaths', 'true')
    $branchName = "local/native-interface-v$($metadata.native_interface_version)-$TargetId"
    Invoke-Checked 'git' @('-C', $GoCheckout, 'switch', '-c', $branchName)
    Copy-Item ([System.IO.Path]::Combine($GoOutputRoot, '*')) `
        -Destination $GoCheckout -Recurse -Force

    Push-Location ([System.IO.Path]::Combine(
        $GoCheckout,
        ($row.module_path -replace '/', [System.IO.Path]::DirectorySeparatorChar)
    ))
    try {
        Invoke-Checked 'go' $goTestArgs
    }
    finally { Pop-Location }

    Invoke-Checked 'git' @('-C', $GoCheckout, 'add', '--all')
    $stagedFiles = & git -C $GoCheckout diff --cached --name-only
    if ($LASTEXITCODE -ne 0) { throw 'Unable to inspect the staged Go repository files.' }
    if (-not $stagedFiles) { throw 'The generated Go repository produced no changes.' }

    $existingUserName = & git -C $GoCheckout config user.name
    if (-not $existingUserName) {
        Invoke-Checked 'git' @('-C', $GoCheckout, 'config', 'user.name', 'Local supply-chain rehearsal')
    }
    $existingUserEmail = & git -C $GoCheckout config user.email
    if (-not $existingUserEmail) {
        Invoke-Checked 'git' @('-C', $GoCheckout, 'config', 'user.email', 'noreply@localhost')
    }

    $commitTitle = "Publish native interface v$($metadata.native_interface_version)"
    $commitBody = @"
Prepare the $TargetId native module and local supply-chain evidence so the
generated PR can be reviewed before trusted 1ES production wiring.

Co-authored-by: Copilot App <223556219+Copilot@users.noreply.github.com>
Copilot-Session: 6657175b-2e8e-4f7c-bb91-6d9edab5fa79
"@
    Invoke-Checked 'git' @('-C', $GoCheckout, 'commit', '-m', $commitTitle, '-m', $commitBody)

    $changedFiles = & git -C $GoCheckout diff HEAD^ --name-only
    if ($LASTEXITCODE -ne 0) { throw 'Unable to inspect the local PR commit.' }
    @"
# Local PR preview — do not publish

**Repository:** Azure/azure-cosmos-driver
**Branch:** $branchName
**Title:** $commitTitle

## Purpose

Publish the **$TargetId** artifacts for **rust_driver_native_interface**
v$($metadata.native_interface_version), with locally generated supply-chain
evidence for workflow validation.

> This commit contains locally built and self-signed rehearsal artifacts. It
> must not be pushed or released. Production artifacts must be rebuilt,
> attested, and signed by the trusted 1ES/ESRP pipeline.

## Validation

- Rust native static and dynamic libraries built with cargo-auditable
- Dynamic-library auditable metadata recovered successfully
- Microsoft SPDX 2.2 SBOM generated
- SPDX SBOM validated with Microsoft sbom-tool
- Go/cgo module linked and tested for **$TargetId**

## Changed files

$(($changedFiles | ForEach-Object { "- ``$_``" }) -join "`n")
"@ | Set-Content -Path $prPreviewPath -Encoding utf8
}

Write-Host ''
Write-Host 'Local supply-chain rehearsal completed.'
Write-Host "Release bundle: $TargetArtifactDir"
Write-Host "Generated Go layout: $GoOutputRoot"
if ($PrepareGoPr) {
    Write-Host "Local Go PR checkout: $GoCheckout"
    Write-Host "PR preview: $prPreviewPath"
}
Write-Host 'No branch was pushed and no remote PR was opened.'
