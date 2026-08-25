#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

#Requires -Version 7.0
param(
  # Stable channel version e.g., "1.95" or "1.95.0".
  # If omitted, the latest release is fetched from GitHub.
  [string] $Version,

  # Nightly channel date e.g., "2026-04-14" or "nightly-2026-04-14".
  # If omitted, the date is extracted from 'rustc --version' output.
  [string] $NightlyVersion
)

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version 2.0

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))

# Determine the stable version to pin.
if (-not $Version) {
  Write-Host "Detecting latest stable Rust version from GitHub..."
  $Version = Invoke-RestMethod 'https://api.github.com/repos/rust-lang/rust/releases/latest' |
    Select-Object -ExpandProperty tag_name
  Write-Host "  Detected version: $Version"
}

# Determine the starting date to search for a working nightly toolchain.
if ($NightlyVersion) {
  # Accept "nightly-2026-04-14" or just "2026-04-14".
  $nightlyDate = [datetime]::ParseExact(
    ($NightlyVersion -replace '^nightly-', ''), 'yyyy-MM-dd', $null
  )
} else {
  Write-Host "Detecting nightly date from 'rustc --version'..."
  $rustcOutput = rustc --version 2>&1
  Write-Host "  $rustcOutput"
  if ($rustcOutput -match '\([\w\d]+ (\d{4}-\d{2}-\d{2})\)') {
    $nightlyDate = [datetime]::ParseExact($Matches[1], 'yyyy-MM-dd', $null)
    Write-Host "  Detected date: $($nightlyDate.ToString('yyyy-MM-dd'))"
  } else {
    LogError "Could not parse a date from: $rustcOutput"
    exit 1
  }
}

# Install a nightly toolchain. If a given date is unavailable, advance one day at a time.
$nightlyChannel = $null
$maxDaysAhead = 30
for ($i = 0; $i -le $maxDaysAhead; $i++) {
  $candidateDate = $nightlyDate.AddDays($i).ToString('yyyy-MM-dd')
  $candidateChannel = "nightly-$candidateDate"
  Write-Host "Trying to install '$candidateChannel'..."
  Invoke-LoggedCommand "rustup toolchain install --no-self-update $candidateChannel" -GroupOutput -DoNotExitOnFailedExitCode
  if ($LASTEXITCODE -eq 0) {
    $nightlyChannel = $candidateChannel
    Write-Host "Successfully installed '$nightlyChannel'."
    break
  }
  LogWarning "Could not install '$candidateChannel', trying next day..."
}

if (-not $nightlyChannel) {
  LogError "Failed to install any nightly toolchain within $maxDaysAhead days of $($nightlyDate.ToString('yyyy-MM-dd'))."
  exit 1
}

$toolsRustdocTypesVersion = Get-RustdocTypesVersionForToolchain -Toolchain $nightlyChannel

# Update rust-toolchain.toml
$toolchainTomlPath = [System.IO.Path]::Combine($RepoRoot, 'rust-toolchain.toml')
Write-Host "Updating '$toolchainTomlPath'..."
$content = Get-Content -Raw $toolchainTomlPath
$updated = $content -replace 'channel = "[^"]+"', "channel = `"$Version`""
if ($updated -eq $content) {
  LogWarning "No 'channel' value was updated in '$toolchainTomlPath'."
}
Set-Content -Path $toolchainTomlPath -Value $updated -NoNewline

# Update the pinned nightly channel in Language-Settings.ps1
$languageSettingsPath = [System.IO.Path]::Combine($RepoRoot, 'eng', 'scripts', 'Language-Settings.ps1')
Write-Host "Updating '$languageSettingsPath'..."
$content = Get-Content -Raw $languageSettingsPath
$updated = $content -replace "'nightly-\d{4}-\d{2}-\d{2}'", "'$nightlyChannel'"
if ($updated -eq $content) {
  LogWarning "No nightly channel was updated in '$languageSettingsPath'."
}
Set-Content -Path $languageSettingsPath -Value $updated -NoNewline

# Update the nightly channel used by the engineering tools.
$toolsToolchainPath = [System.IO.Path]::Combine($RepoRoot, 'eng', 'tools', 'rust-toolchain.toml')
Write-Host "Updating '$toolsToolchainPath'..."
$content = Get-Content -Raw $toolsToolchainPath
$updated = $content -replace 'channel = "nightly-\d{4}-\d{2}-\d{2}"', "channel = `"$nightlyChannel`""
if ($updated -eq $content) {
  LogWarning "No nightly channel was updated in '$toolsToolchainPath'."
}
Set-Content -Path $toolsToolchainPath -Value $updated -NoNewline

# Update the stable Rust version and rustdoc-types version required by the engineering tools.
$toolsManifestPath = [System.IO.Path]::Combine($RepoRoot, 'eng', 'tools', 'Cargo.toml')
Update-ToolsRustdocTypesVersion -ManifestPath $toolsManifestPath -RustVersion $Version -RustdocTypesVersion $toolsRustdocTypesVersion

# Update shebang lines in all *.rs scripts under eng/scripts/
$rsScriptsDir = [System.IO.Path]::Combine($RepoRoot, 'eng', 'scripts')
foreach ($rsFile in Get-ChildItem -Path $rsScriptsDir -Filter '*.rs') {
  Write-Host "Updating '$($rsFile.FullName)'..."
  $content = Get-Content -Raw $rsFile.FullName
  $updated = $content -replace 'cargo \+nightly-\d{4}-\d{2}-\d{2}', "cargo +$nightlyChannel"
  if ($updated -eq $content) {
    LogWarning "No nightly channel was updated in '$($rsFile.FullName)'."
  }
  Set-Content -Path $rsFile.FullName -Value $updated -NoNewline
}

Write-Host "Done. Stable: '$Version', nightly: '$nightlyChannel'."

# Nightly toolchain dates identify snapshots from Rust's moving development branch rather than the
# eventual stable release date. To keep eng/tools aligned, resolve the nightly's rust commit, ask
# rust-lang/rust which commit last changed src/rustdoc-json-types/lib.rs at that snapshot, then map
# that upstream commit to the minimum rustdoc-types release listed in rust-lang/rustdoc-types'
# CHANGELOG.md. For background on why nightly dates can look far behind stable release numbers, see
# https://doc.rust-lang.org/book/appendix-07-nightly-rust.html
function Get-RustCommitHashForToolchain(
  [string] $Toolchain
) {
  $verboseOutput = & rustc "+$Toolchain" --version --verbose 2>&1
  if ($LASTEXITCODE -ne 0) {
    LogError "Failed to get verbose rustc version for '$Toolchain'."
    exit 1
  }

  foreach ($line in $verboseOutput) {
    if ($line -match '^commit-hash:\s+([0-9a-f]{40})$') {
      return $matches[1]
    }
  }

  $versionOutput = & rustc "+$Toolchain" --version 2>&1
  if ($LASTEXITCODE -ne 0) {
    LogError "Failed to get rustc version for '$Toolchain'."
    exit 1
  }

  foreach ($line in $versionOutput) {
    if ($line -match '\(([0-9a-f]{9,40}) \d{4}-\d{2}-\d{2}\)') {
      return $matches[1]
    }
  }

  LogError "Could not determine the rust commit hash for '$Toolchain'."
  exit 1
}

function Get-RustdocTypesGraphQlData(
  [string] $RustCommitHash
) {
  $query = 'query($rustExpr:String!, $rustPath:String!, $changelogExpr:String!) { rust: repository(owner:"rust-lang", name:"rust") { object(expression:$rustExpr) { ... on Commit { history(first: 1, path: $rustPath) { nodes { oid } } } } } rustdocTypes: repository(owner:"rust-lang", name:"rustdoc-types") { object(expression:$changelogExpr) { ... on Blob { text } } } }'
  $json = & gh api graphql `
    -f "query=$query" `
    -F "rustExpr=$RustCommitHash" `
    -F 'rustPath=src/rustdoc-json-types/lib.rs' `
    -F 'changelogExpr=trunk:CHANGELOG.md' 2>&1
  if ($LASTEXITCODE -ne 0) {
    LogError "Failed to query GitHub for rustdoc schema metadata."
    exit 1
  }

  try {
    return ($json | ConvertFrom-Json).data
  } catch {
    LogError "Failed to parse GitHub GraphQL response for rustdoc schema metadata."
    exit 1
  }
}

function Get-RustdocJsonTypesCommit(
  [object] $GraphQlData
) {
  $commit = $GraphQlData.rust.object.history.nodes | Select-Object -First 1 -ExpandProperty oid
  if (-not $commit) {
    LogError "Could not determine the rustdoc JSON schema commit from GitHub."
    exit 1
  }
  return $commit
}

function Get-RustdocTypesReleaseMap(
  [object] $GraphQlData
) {
  $changelog = $GraphQlData.rustdocTypes.object.text
  if (-not $changelog) {
    LogError "Could not load rustdoc-types CHANGELOG.md from GitHub."
    exit 1
  }

  $releases = New-Object System.Collections.Generic.List[object]
  $currentTag = $null
  foreach ($line in ($changelog -split "`r?`n")) {
    if ($line -match '^# \[(v[0-9][^]]+)\]') {
      $currentTag = $matches[1]
      continue
    }

    if ($currentTag -and $line -match '^- Upstream Commit: \[`([0-9a-f]{40})`\]') {
      $releases.Add([pscustomobject]@{
          Tag = $currentTag
          UpstreamCommit = $matches[1]
        })
      $currentTag = $null
    }
  }

  if ($releases.Count -eq 0) {
    LogError "Could not parse any rustdoc-types releases from CHANGELOG.md."
    exit 1
  }

  return $releases
}

function Get-RustdocTypesVersionForRustdocCommit(
  [string] $RustdocCommit,
  [object[]] $ReleaseMap
) {
  $matches = $ReleaseMap |
    Where-Object { $_.UpstreamCommit -eq $RustdocCommit } |
    Sort-Object { [version]($_.Tag.TrimStart('v')) }
  $release = $matches | Select-Object -First 1
  if (-not $release) {
    LogError "Could not find a rustdoc-types release for rust commit '$RustdocCommit'."
    exit 1
  }

  return $release.Tag.TrimStart('v')
}

function Get-RustdocTypesVersionForToolchain(
  [string] $Toolchain
) {
  $rustCommitHash = Get-RustCommitHashForToolchain -Toolchain $Toolchain
  $graphQlData = Get-RustdocTypesGraphQlData -RustCommitHash $rustCommitHash
  $rustdocCommit = Get-RustdocJsonTypesCommit -GraphQlData $graphQlData
  $releaseMap = Get-RustdocTypesReleaseMap -GraphQlData $graphQlData
  return Get-RustdocTypesVersionForRustdocCommit -RustdocCommit $rustdocCommit -ReleaseMap $releaseMap
}

function Update-ToolsRustdocTypesVersion(
  [string] $ManifestPath,
  [string] $RustVersion,
  [string] $RustdocTypesVersion
) {
  Write-Host "Updating '$ManifestPath'..."
  $content = Get-Content -Raw $ManifestPath
  $updated = $content -replace 'rust-version = "[^"]+"', "rust-version = `"$RustVersion`""
  $updated = $updated -replace 'rustdoc-types = "[^"]+"', "rustdoc-types = `"$RustdocTypesVersion`""
  if ($updated -eq $content) {
    LogWarning "No engineering tools version values were updated in '$ManifestPath'."
  }
  Set-Content -Path $ManifestPath -Value $updated -NoNewline
}
