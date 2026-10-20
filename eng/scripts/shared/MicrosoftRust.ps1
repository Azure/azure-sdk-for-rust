# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

function Get-MicrosoftRustToolchainConfiguration(
  [Parameter(Mandatory = $true)]
  [string] $Path
) {
  if (!(Test-Path -Path $Path -PathType Leaf)) {
    throw "Microsoft Rust toolchain configuration was not found: $Path"
  }

  $content = Get-Content -Path $Path -Raw
  $channelMatches = [regex]::Matches(
    $content,
    '(?m)^\s*channel\s*=\s*"([^"]+)"\s*$'
  )
  if ($channelMatches.Count -ne 1) {
    throw "Microsoft Rust toolchain configuration must declare exactly one channel."
  }

  $channel = $channelMatches[0].Groups[1].Value
  if ($channel -notmatch '^ms-prod-\d+(?:\.\d+)+$') {
    throw "Microsoft Rust channel '$channel' is not an explicit pinned ms-prod channel."
  }

  $targetsMatches = [regex]::Matches(
    $content,
    '(?ms)^\s*targets\s*=\s*\[(.*?)\]'
  )
  if ($targetsMatches.Count -ne 1) {
    throw "Microsoft Rust toolchain configuration must declare exactly one targets array."
  }

  $targets = @(
    [regex]::Matches($targetsMatches[0].Groups[1].Value, '"([^"]+)"') |
      ForEach-Object { $_.Groups[1].Value }
  )
  if ($targets.Count -eq 0) {
    throw "Microsoft Rust toolchain configuration must declare at least one target."
  }

  return [pscustomobject]@{
    Channel = $channel
    Targets = $targets
  }
}

function New-MicrosoftRustInstallerConfiguration(
  [Parameter(Mandatory = $true)]
  [string] $Path,

  [Parameter(Mandatory = $true)]
  [string] $Target,

  [Parameter(Mandatory = $true)]
  [string] $OutputPath
) {
  $configuration = Get-MicrosoftRustToolchainConfiguration -Path $Path
  if ($Target -notin $configuration.Targets) {
    throw "Microsoft Rust target '$Target' is not declared in '$Path'."
  }

  $content = Get-Content -Path $Path -Raw
  $targetsMatches = [regex]::Matches(
    $content,
    '(?ms)^\s*targets\s*=\s*\[(.*?)\]\s*'
  )
  if ($targetsMatches.Count -ne 1) {
    throw "Microsoft Rust toolchain configuration must declare exactly one targets array."
  }

  $installerContent = $content.Remove(
    $targetsMatches[0].Index,
    $targetsMatches[0].Length
  )
  Set-Content -Path $OutputPath -Value $installerContent -Encoding utf8
}
