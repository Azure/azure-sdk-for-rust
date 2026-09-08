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

  $targetsMatch = [regex]::Match(
    $content,
    '(?ms)^\s*targets\s*=\s*\[(.*?)\]'
  )
  if (!$targetsMatch.Success) {
    throw "Microsoft Rust toolchain configuration must declare targets."
  }

  $targets = @(
    [regex]::Matches($targetsMatch.Groups[1].Value, '"([^"]+)"') |
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

function Test-MicrosoftRustActiveToolchain(
  [Parameter(Mandatory = $true)]
  [string] $ActiveToolchain,

  [Parameter(Mandatory = $true)]
  [string] $Channel
) {
  return (
    $ActiveToolchain -ceq $Channel -or
    $ActiveToolchain.StartsWith("$Channel-", [StringComparison]::Ordinal)
  )
}
