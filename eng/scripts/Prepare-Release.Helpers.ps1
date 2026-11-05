# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# cspell:ignore pathversions Zscript
#Requires -Version 7.0
<#
.SYNOPSIS
Deterministic operations for the prepare-release skill, not a standalone release workflow.
.DESCRIPTION
Analyze writes a decision file without changing tracked files. The skill reviews
Release and DecisionReason for each package, then calls Apply. Validate checks
prepared versions and changelogs. No operation increases package versions.
#>
[CmdletBinding()]
param(
  [switch] $FromSkill,
  [ValidateSet('Analyze', 'Apply', 'Validate')]
  [string] $Operation = 'Analyze',
  [string[]] $ServiceDirectory,
  [Alias('PackageNames')]
  [string[]] $PackageName,
  [ValidateSet('Stable', 'Beta')]
  [string] $ReleaseType,
  [string] $ReleaseDate = (Get-Date -Format 'yyyy-MM-dd'),
  [string] $SessionDirectory,
  [string] $PlanPath
)

$ErrorActionPreference = 'Stop'
if (!$FromSkill) {
  throw 'Use the prepare-release skill. Its helper requires -FromSkill.'
}

. ([System.IO.Path]::Combine($PSScriptRoot, '..', 'common', 'scripts', 'common.ps1'))
. ([System.IO.Path]::Combine($PSScriptRoot, 'shared', 'common.ps1'))

function Invoke-ReleaseCommand([string[]] $Arguments, [int[]] $AllowedExitCodes = @(0)) {
  $command = ($Arguments | ForEach-Object { "'" + $_.Replace("'", "''") + "'" }) -join ' '
  $output = @(Invoke-LoggedCommand "& $command" -ExecutePath $RepoRoot `
      -AllowedExitCodes $AllowedExitCodes -DoNotExitOnFailedExitCode)
  if ($LASTEXITCODE -notin $AllowedExitCodes) {
    throw "Release command failed: $($Arguments -join ' ')"
  }
  return $output
}

function Invoke-ReleaseToml([string] $Mode, [hashtable] $InputObject) {
  $inputPath = [System.IO.Path]::GetTempFileName()
  try {
    $InputObject | ConvertTo-Json -Depth 100 | Set-Content $inputPath
    $toolchain = Get-ResolvedRustToolchain -Toolchain 'nightly'
    $scriptPath = [System.IO.Path]::Combine($PSScriptRoot, 'update-pathversions.rs')
    $output = Invoke-ReleaseCommand @('cargo', "+$toolchain", '-Zscript', $scriptPath, $Mode, $inputPath)
    return ($output -join "`n") | ConvertFrom-Json -AsHashtable
  }
  finally {
    Remove-Item -LiteralPath $inputPath -Force
  }
}

function Initialize-ReleaseTags([string] $Directory) {
  if (!$Directory) { throw 'Pass the session directory used by the prepare-release skill.' }
  $identity = [Convert]::ToHexString([System.Security.Cryptography.SHA256]::HashData(
      [System.Text.Encoding]::UTF8.GetBytes([string] $RepoRoot)))
  $marker = [System.IO.Path]::Combine($Directory, "prepare-release-tags-$identity")
  if (!(Test-Path -LiteralPath $marker)) {
    Invoke-ReleaseCommand @('git', 'fetch', '--tags', '--no-recurse-submodules',
      'https://github.com/Azure/azure-sdk-for-rust.git') | Out-Host
    [void] (New-Item -ItemType Directory -Path $Directory -Force)
    Set-Content -LiteralPath $marker -Value 'https://github.com/Azure/azure-sdk-for-rust.git'
  }
}

function Get-ReleaseInventory {
  # Keep metadata behind this boundary so manifest-only discovery can replace it.
  $metadata = Get-CargoMetadata
  if ($LASTEXITCODE) { throw 'Could not read Cargo metadata.' }
  $paths = @(Get-ChildItem -Path ([System.IO.Path]::Combine($RepoRoot, 'sdk', '*', '*', 'Cargo.toml')))
  $documents = @{}
  foreach ($path in $paths) { $documents[$path.FullName] = Get-Content -LiteralPath $path.FullName -Raw }
  $parsed = Invoke-ReleaseToml 'release-inspect' $documents
  foreach ($path in $paths) {
    $document = $parsed[$path.FullName]
    $name = $document.package.name
    $publishable = $document.package.publish -ne $false -and @($document.package.publish).Count -ne 0
    if (!$document.package.Contains('publish')) { $publishable = $true }
    $package = @($metadata.packages | Where-Object { $_.manifest_path -eq $path.FullName })
    if ($publishable -and $package.Count -ne 1) {
      throw "Publishable SDK manifest is not a unique workspace member: $path"
    }
    [PSCustomObject]@{
      Name = $name
      Version = if ($package.Count) { $package[0].version } else { $null }
      Service = $path.Directory.Parent.Name
      Directory = [System.IO.Path]::GetRelativePath($RepoRoot, $path.DirectoryName).Replace('\', '/')
      Manifest = $path.FullName
      Publishable = $publishable
      Dependencies = if ($package.Count) { @($package[0].dependencies) } else { @() }
      Document = $document
    }
  }
}

function Select-ReleasePackages($Inventory, [string[]] $Services, [string[]] $Names) {
  if ((!$Services -and !$Names) -or ($Services -and $Names)) {
    throw 'Specify service directories or package names, but not both.'
  }
  $selected = @()
  foreach ($service in (Resolve-CargoPackageNames $Services)) {
    if ($service -notmatch '^[\w-]+$') { throw "Invalid service directory: $service" }
    $matches = @($Inventory | Where-Object { $_.Service -eq $service -and $_.Publishable })
    if (!$matches.Count) { throw "No publishable crates in service directory: $service" }
    $selected += $matches
  }
  foreach ($name in (Resolve-CargoPackageNames $Names)) {
    $matches = @($Inventory | Where-Object { $_.Name -ceq $name })
    if ($matches.Count -ne 1 -or !$matches[0].Publishable) {
      throw "Package must identify one publishable SDK crate: $name"
    }
    $selected += $matches
  }
  foreach ($package in $selected) {
    if ((Split-Path $package.Directory -Leaf) -cne $package.Name) {
      throw "Package name does not match directory: $($package.Directory) ($($package.Name))"
    }
  }
  return @($selected | Sort-Object Name -Unique)
}

function Get-ReleaseBaseline([string] $Name, [string] $Channel) {
  $versions = @(foreach ($tag in (Invoke-ReleaseCommand @('git', 'tag', '-l', "$Name@*"))) {
      $text = $tag.Substring($Name.Length + 1)
      try { $version = [System.Management.Automation.SemanticVersion]::new($text) }
      catch { continue }
      if (($text -split '[-+]', 2)[0] -cne "$($version.Major).$($version.Minor).$($version.Patch)" -or
        @($version.PreReleaseLabel -split '\.' | Where-Object { $_ -match '^0[0-9]+$' }).Count) { continue }
      if ($Channel -eq 'Stable' -and $version.PreReleaseLabel) { continue }
      [PSCustomObject]@{ Tag = $tag; Version = $text; Parsed = $version }
    })
  return $versions | Sort-Object Parsed -Descending | Select-Object -First 1
}

function ConvertTo-ReleaseChangelog([string] $Content) {
  $entries = Get-ChangeLogEntriesFromContent $Content
  if (!$entries) { throw 'Could not parse changelog.' }
  $lines = @($entries.HeaderBlock.Trim())
  foreach ($entry in $entries.Values) {
    $body = @($entry.ReleaseContent)
    for ($i = 0; $i -lt $body.Count; $i++) {
      $line = $body[$i].TrimEnd()
      if (!$line.Trim()) { continue }
      if ($line -match '^###\s') {
        $next = $i + 1
        while ($next -lt $body.Count -and !$body[$next].Trim()) { $next++ }
        if ($next -eq $body.Count -or $body[$next] -match '^###\s') { continue }
      }
      $lines += $line
    }
  }
  return $lines -join "`n"
}

function Get-ReleaseDependencySignature($Document, $WorkspaceDocument) {
  $result = [ordered]@{}
  $tables = @(@{ Prefix = ''; Table = $Document })
  foreach ($target in @($Document.target.Keys | Sort-Object)) {
    $tables += @{ Prefix = "$target/"; Table = $Document.target[$target] }
  }
  foreach ($table in $tables) {
    foreach ($kind in @('dependencies', 'build-dependencies')) {
      foreach ($key in @($table.Table[$kind].Keys | Sort-Object)) {
        $dependency = $table.Table[$kind][$key]
        if ($dependency -is [System.Collections.IDictionary] -and $dependency.workspace) {
          $result["$($table.Prefix)$kind/$key"] = $WorkspaceDocument.workspace.dependencies[$key]
        }
      }
    }
  }
  return ConvertTo-Json -InputObject $result -Depth 100 -Compress
}

function Get-ReleaseSnapshot([string[]] $Directories) {
  $paths = @('Cargo.toml', 'Cargo.lock')
  $paths += @(Get-ChildItem -Path ([System.IO.Path]::Combine($RepoRoot, 'sdk', '*', '*', 'Cargo.toml')) |
      ForEach-Object { [System.IO.Path]::GetRelativePath($RepoRoot, $_.FullName).Replace('\', '/') })
  $paths += @(Invoke-ReleaseCommand (@('git', 'ls-files', '--cached', '--others', '--exclude-standard', '--') + $Directories))
  $snapshot = [ordered]@{}
  foreach ($path in ($paths | Sort-Object -Unique)) {
    $absolute = [System.IO.Path]::Combine($RepoRoot, $path)
    $snapshot[$path] = if (Test-Path -LiteralPath $absolute -PathType Leaf) {
      (Get-FileHash -LiteralPath $absolute -Algorithm SHA256).Hash
    } else { $null }
  }
  return $snapshot
}

function Get-ReleaseAnalysis($Inventory, $Selected, [string] $Channel, [string] $Date) {
  $documents = @{ 'HEAD:Cargo.toml' = (Invoke-ReleaseCommand @('git', 'show', 'HEAD:Cargo.toml')) -join "`n" }
  $baselines = @{}
  foreach ($package in $Selected) {
    $baseline = Get-ReleaseBaseline $package.Name $Channel
    $baselines[$package.Name] = $baseline
    $manifest = "$($package.Directory)/Cargo.toml"
    $documents["HEAD:$manifest"] = (Invoke-ReleaseCommand @('git', 'show', "HEAD:$manifest")) -join "`n"
    if ($baseline) {
      foreach ($path in @('Cargo.toml', $manifest)) {
        $id = "$($baseline.Tag):$path"
        $documents[$id] = (Invoke-ReleaseCommand @('git', 'show', $id)) -join "`n"
      }
    }
  }
  $parsed = Invoke-ReleaseToml 'release-inspect' $documents
  $packages = @(foreach ($package in $Selected) {
      $baseline = $baselines[$package.Name]
      $reasons = @()
      if (!$baseline) { $reasons += 'First release for this channel' }
      else {
        $changed = @(Invoke-ReleaseCommand @('git', 'diff', '--name-only', "$($baseline.Tag)..HEAD", '--', $package.Directory))
        foreach ($path in $changed) {
          if ($path -eq "$($package.Directory)/Cargo.toml") {
            $before = $parsed["$($baseline.Tag):$path"] | ConvertTo-Json -Depth 100 -Compress
            $after = $parsed["HEAD:$path"] | ConvertTo-Json -Depth 100 -Compress
            if ($before -ceq $after) { continue }
          }
          if ($path -eq "$($package.Directory)/CHANGELOG.md") {
            $before = Invoke-ReleaseCommand @('git', 'show', "$($baseline.Tag):$path") @(0, 128)
            $after = Invoke-ReleaseCommand @('git', 'show', "HEAD:$path") @(0, 128)
            if ($before.Count -and $after.Count -and
              (ConvertTo-ReleaseChangelog ($before -join "`n")) -ceq
              (ConvertTo-ReleaseChangelog ($after -join "`n"))) { continue }
          }
          $reasons += $path
        }
        $manifest = "$($package.Directory)/Cargo.toml"
        $before = Get-ReleaseDependencySignature $parsed["$($baseline.Tag):$manifest"] $parsed["$($baseline.Tag):Cargo.toml"]
        $after = Get-ReleaseDependencySignature $parsed["HEAD:$manifest"] $parsed['HEAD:Cargo.toml']
        if ($before -cne $after) { $reasons += 'Inherited workspace dependencies changed' }
      }
      [ordered]@{
        Name = $package.Name; Directory = $package.Directory; Service = $package.Service
        Version = $package.Version; BaselineTag = $baseline.Tag; BaselineVersion = $baseline.Version
        Release = $reasons.Count -gt 0; DecisionReason = ''; Changes = $reasons
        LocalChanges = @(Invoke-ReleaseCommand @('git', 'status', '--porcelain', '--untracked-files=all', '--',
            $package.Directory, 'Cargo.toml', 'Cargo.lock'))
        LocalDependencies = @($package.Dependencies | Where-Object { $_.path -and $_.kind -ne 'dev' } |
            Select-Object name, req, kind, path)
      }
    })
  return [ordered]@{
    Repository = [string] $RepoRoot
    Head = (Invoke-ReleaseCommand @('git', 'rev-parse', 'HEAD')) -join ''
    ReleaseType = $Channel; ReleaseDate = $Date
    WorkspaceBuild = @($Selected | Where-Object { $_.Service -in @('core', 'identity') }).Count -gt 0
    Packages = $packages
    Snapshot = Get-ReleaseSnapshot @($Selected.Directory)
  }
}

function Get-ReleaseDecisions($Plan, $Inventory) {
  if ($Plan.ReleaseType -notin @('Stable', 'Beta')) { throw 'Invalid release type in decision file.' }
  $versions = @{}
  $release = @($Plan.Packages | Where-Object Release)
  foreach ($package in $Plan.Packages) {
    $actual = @($Inventory | Where-Object { $_.Name -ceq $package.Name -and $_.Publishable })
    if ($actual.Count -ne 1 -or $actual[0].Version -cne $package.Version) {
      throw "Package/version no longer matches analysis: $($package.Name)"
    }
    if (!$package.Release) {
      if (!$package.BaselineVersion) { throw "Cannot skip first release: $($package.Name)" }
      if ($package.Changes.Count -and !$package.DecisionReason) {
        throw "Record DecisionReason before overriding changed crate: $($package.Name)"
      }
      $baseline = Get-ReleaseBaseline $package.Name $Plan.ReleaseType
      if ($baseline.Tag -cne $package.BaselineTag -or $baseline.Version -cne $package.BaselineVersion) {
        throw "Release baseline changed: $($package.Name)"
      }
      $versions[$package.Name] = $baseline.Version
      continue
    }
    $version = [System.Management.Automation.SemanticVersion]::new($package.Version)
    if (($Plan.ReleaseType -eq 'Stable' -and $version.PreReleaseLabel) -or
      ($Plan.ReleaseType -eq 'Beta' -and !$version.PreReleaseLabel -and $version.Major -ne 0)) {
      throw "Manifest version does not match release type: $($package.Name) $version. Do not bump it automatically."
    }
    $latest = Get-ReleaseBaseline $package.Name 'Beta'
    if ($latest -and $version -le $latest.Parsed) {
      throw "Manifest version is not newer than released tags: $($package.Name). Do not bump it automatically."
    }
  }
  $ordered = @()
  $pending = @($release.Name)
  while ($pending.Count) {
    $ready = @($pending | Where-Object {
        $name = $_
        $dependencies = @($Inventory | Where-Object Name -CEQ $name | Select-Object -ExpandProperty Dependencies)
        @($dependencies | Where-Object { $_.kind -ne 'dev' -and $_.name -in $pending }).Count -eq 0
      })
    if (!$ready.Count) { throw 'Release dependencies contain a cycle.' }
    foreach ($name in $ready) {
      $package = $Inventory | Where-Object Name -CEQ $name
      if ($Plan.ReleaseType -eq 'Stable') {
        foreach ($dependency in ($package.Dependencies | Where-Object { $_.kind -ne 'dev' })) {
          if (!$versions.ContainsKey($dependency.name) -and $dependency.req -match '\d+\.\d+\.\d+-[0-9A-Za-z]') {
            throw "Stable release '$name' cannot depend on preview '$($dependency.name)'. Analyze it against its last stable release."
          }
        }
      }
      foreach ($dependency in ($package.Dependencies | Where-Object { $_.path -and $_.kind -ne 'dev' })) {
        if ($versions.ContainsKey($dependency.name)) { continue }
        if ($dependency.name -notin $release.Name) {
          throw "Analyze dependency '$($dependency.name)' of '$name'; ask before expanding release scope."
        }
        if ($dependency.req -eq '*') {
          throw "Unreleased dependency '$($dependency.name)' of '$name' needs path + version."
        }
      }
      $ordered += $name
    }
    $pending = @($pending | Where-Object { $_ -notin $ready })
  }
  return @{ Versions = $versions; ReleaseOrder = $ordered }
}

function Confirm-PreparedRelease($Plan, $Inventory) {
  foreach ($package in ($Plan.Packages | Where-Object Release)) {
    $actual = $Inventory | Where-Object Name -CEQ $package.Name
    if ($actual.Version -cne $package.Version) { throw "Preparation changed package version: $($package.Name)" }
    $path = [System.IO.Path]::Combine($RepoRoot, $package.Directory, 'CHANGELOG.md')
    if (!(Confirm-ChangeLogEntry -ChangeLogLocation $path -VersionString $package.Version -ForRelease $true)) {
      throw "Changelog is not ready: $($package.Name)"
    }
    $entry = Get-ChangeLogEntry -ChangeLogLocation $path -VersionString $package.Version
    if ($entry.ReleaseStatus -ne "($($Plan.ReleaseDate))") { throw "Unexpected release date: $($package.Name)" }
  }
}

if ($MyInvocation.InvocationName -eq '.') { return }
if (!$PlanPath) {
  if (!$SessionDirectory) { throw 'Specify -SessionDirectory or -PlanPath.' }
  $PlanPath = [System.IO.Path]::Combine($SessionDirectory, 'prepare-release.json')
}
if ($Operation -eq 'Analyze') {
  if (!$ReleaseType) { throw 'The skill must ask for Stable or Beta and pass -ReleaseType.' }
  $date = [datetime]::ParseExact($ReleaseDate, 'yyyy-MM-dd', [CultureInfo]::InvariantCulture)
  Initialize-ReleaseTags $SessionDirectory
  $inventory = @(Get-ReleaseInventory)
  $selected = Select-ReleasePackages $inventory $ServiceDirectory $PackageName
  $plan = Get-ReleaseAnalysis $inventory $selected $ReleaseType $date.ToString('yyyy-MM-dd')
  $plan | ConvertTo-Json -Depth 100 | Set-Content -LiteralPath $PlanPath
  Write-Host "Review release decisions in $PlanPath"
}
else {
  $plan = Get-Content -LiteralPath $PlanPath -Raw | ConvertFrom-Json -AsHashtable
  if ($plan.Repository -cne [string] $RepoRoot) { throw 'Decision file belongs to another repository.' }
  $inventory = @(Get-ReleaseInventory)
  if ($Operation -eq 'Apply') {
    $head = (Invoke-ReleaseCommand @('git', 'rev-parse', 'HEAD')) -join ''
    $snapshot = Get-ReleaseSnapshot @($plan.Packages.Directory)
    if ($head -cne $plan.Head -or
      ($snapshot | ConvertTo-Json -Compress) -cne ($plan.Snapshot | ConvertTo-Json -Compress)) {
      throw 'Analysis is stale; analyze again before applying edits.'
    }
    if (@($plan.Packages | Where-Object { $_.LocalChanges.Count }).Count) {
      throw 'Review local changes and analyze again once they are resolved; HEAD-only decisions cannot verify them.'
    }
    $decisions = Get-ReleaseDecisions $plan $inventory
    $inputObject = @{ root = [string] $RepoRoot; versions = $decisions.Versions; apply = $true }
    $result = Invoke-ReleaseToml 'release-dependencies' $inputObject
    [ordered]@{
      ReleaseOrder = $decisions.ReleaseOrder; WorkspaceBuild = $plan.WorkspaceBuild
      ChangedManifests = $result.changed_manifests
      BuildPackages = @(@($decisions.ReleaseOrder) + @($result.affected_packages) | Sort-Object -Unique)
    } | ConvertTo-Json -Depth 100
  }
  else {
    Confirm-PreparedRelease $plan $inventory
    $decisions = Get-ReleaseDecisions $plan $inventory
    $remaining = Invoke-ReleaseToml 'release-dependencies' @{
      root = [string] $RepoRoot; versions = $decisions.Versions; apply = $false
    }
    if ($remaining.changed_manifests.Count) {
      throw 'Prepared dependency declarations no longer match the approved release decisions.'
    }
    Write-Host 'Prepared versions, release dates, and changelogs are valid.'
  }
}
