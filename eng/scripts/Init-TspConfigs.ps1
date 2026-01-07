#!/usr/bin/env pwsh

# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

<#
.SYNOPSIS
    Runs 'tsp-client init' for every directory containing a tspconfig file.

.DESCRIPTION
  Searches recursively for tspconfig files (defaults to tspconfig.yaml) beneath the
  provided path, filters them to those referencing the typespec-rust emitter, and
  runs 'tsp-client init -c <path>' from the azure-sdk-for-rust repository root for
  every remaining file. Successful runs also ensure each crate's service directory
  is listed in the root Cargo.toml workspace members array.

.PARAMETER Path
    Root path that contains TypeSpec projects. Defaults to ~/src/azure-rest-api-specs.

.PARAMETER ConfigNames
    One or more filenames to search for. Defaults to 'tspconfig.yaml'.

.EXAMPLE
    .\Init-TspConfigs.ps1
    Runs tsp-client init for every tspconfig.yaml under ~/src/azure-rest-api-specs.

.EXAMPLE
    .\Init-TspConfigs.ps1 -Path "C:\repos\azure-rest-api-specs" -ConfigNames "tspconfig.yaml","tspconfig.yml"
    Runs tsp-client init for matching files under the provided repo path.
#>

[CmdletBinding(SupportsShouldProcess = $true)]
param(
  [Parameter(Mandatory = $false)]
  [string]$Path = (Join-Path $HOME "src/azure-rest-api-specs"),

  [Parameter(Mandatory = $false)]
  [string[]]$ConfigNames = @("tspconfig.yaml")
)

$script:YamlSupportChecked = $false

function Ensure-YamlSupport {
  if ($script:YamlSupportChecked) {
    return
  }

  $script:YamlSupportChecked = $true

  if (-not (Get-Command -Name ConvertFrom-Yaml -ErrorAction SilentlyContinue)) {
    try {
      Import-Module -Name Microsoft.PowerShell.Utility -ErrorAction SilentlyContinue | Out-Null
    }
    catch {
      # ignore and try the next option
    }
  }

  if (-not (Get-Command -Name ConvertFrom-Yaml -ErrorAction SilentlyContinue)) {
    try {
      Import-Module -Name powershell-yaml -ErrorAction Stop | Out-Null
    }
    catch {
      $installMessage = "ConvertFrom-Yaml cmdlet is unavailable. Install PowerShell 7+ or run 'Install-Module -Name powershell-yaml -Scope CurrentUser' and retry."
      throw $installMessage
    }
  }
}

function ConvertFrom-TspYaml {
  [CmdletBinding()]
  param(
    [Parameter(Mandatory = $true)]
    [string]$Content,

    [Parameter(Mandatory = $true)]
    [string]$SourcePath
  )

  Ensure-YamlSupport

  try {
    return $Content | ConvertFrom-Yaml
  }
  catch {
    throw "Unable to parse YAML for '$SourcePath': $_"
  }
}

function Add-CargoMembers {
  [CmdletBinding()]
  param(
    [Parameter(Mandatory = $true)]
    [string]$CargoPath,

    [Parameter(Mandatory = $true)]
    [string[]]$MembersToAdd
  )

  if (-not $MembersToAdd -or $MembersToAdd.Count -eq 0) {
    return @()
  }

  if (-not (Test-Path -Path $CargoPath -PathType Leaf)) {
    Write-Warning "Cargo.toml not found at $CargoPath. Skipping workspace update."
    return @()
  }

  $normalizedMembers = @($MembersToAdd | Where-Object { -not [string]::IsNullOrWhiteSpace($_) } | Sort-Object -Unique)
  if ($normalizedMembers.Count -eq 0) {
    return @()
  }

  $cargoLines = [System.Collections.Generic.List[string]]::new()
  $cargoLines.AddRange([string[]](Get-Content -Path $CargoPath))

  $membersStart = -1
  for ($i = 0; $i -lt $cargoLines.Count; $i++) {
    if ($cargoLines[$i].TrimStart().StartsWith("members = [")) {
      $membersStart = $i
      break
    }
  }

  if ($membersStart -lt 0) {
    Write-Warning "Could not locate 'members = [' block in Cargo.toml."
    return
  }

  $membersEnd = -1
  for ($i = $membersStart + 1; $i -lt $cargoLines.Count; $i++) {
    if ($cargoLines[$i].Trim() -eq "]") {
      $membersEnd = $i
      break
    }
  }

  if ($membersEnd -lt 0) {
    Write-Warning "Unterminated members array in Cargo.toml."
    return
  }

  $existingMembers = [System.Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
  for ($i = $membersStart + 1; $i -lt $membersEnd; $i++) {
    $line = $cargoLines[$i].Trim()
    if ($line -match '"([^\"]+)"') {
      [void]$existingMembers.Add($matches[1])
    }
  }

  $insertLines = @()
  $addedMembers = @()
  foreach ($member in $normalizedMembers) {
    if (-not $existingMembers.Contains($member)) {
      $insertLines += ('  "' + $member + '",')
      $addedMembers += $member
    }
  }

  if ($insertLines.Count -eq 0) {
    Write-Host "No Cargo.toml updates required; all crate paths already present." -ForegroundColor Green
    return @()
  }

  foreach ($line in $insertLines) {
    $cargoLines.Insert($membersEnd, $line)
    $membersEnd++
  }

  Set-Content -Path $CargoPath -Value $cargoLines -Encoding UTF8

  $entryWord = if ($insertLines.Count -eq 1) { "entry" } else { "entries" }
  Write-Host "Added $($insertLines.Count) $entryWord to Cargo.toml members." -ForegroundColor Green

  return ,$addedMembers
}

function Remove-CargoMembers {
  [CmdletBinding()]
  param(
    [Parameter(Mandatory = $true)]
    [string]$CargoPath,

    [Parameter(Mandatory = $true)]
    [string[]]$MembersToRemove
  )

  if (-not $MembersToRemove -or $MembersToRemove.Count -eq 0) {
    return
  }

  if (-not (Test-Path -Path $CargoPath -PathType Leaf)) {
    Write-Warning "Cargo.toml not found at $CargoPath. Unable to remove workspace members."
    return
  }

  $normalizedMembers = @($MembersToRemove | Where-Object { -not [string]::IsNullOrWhiteSpace($_) } | Sort-Object -Unique)
  if ($normalizedMembers.Count -eq 0) {
    return
  }

  $cargoLines = [System.Collections.Generic.List[string]]::new()
  $cargoLines.AddRange([string[]](Get-Content -Path $CargoPath))

  $membersStart = -1
  for ($i = 0; $i -lt $cargoLines.Count; $i++) {
    if ($cargoLines[$i].TrimStart().StartsWith("members = [")) {
      $membersStart = $i
      break
    }
  }

  if ($membersStart -lt 0) {
    Write-Warning "Could not locate 'members = [' block in Cargo.toml."
    return
  }

  $membersEnd = -1
  for ($i = $membersStart + 1; $i -lt $cargoLines.Count; $i++) {
    if ($cargoLines[$i].Trim() -eq "]") {
      $membersEnd = $i
      break
    }
  }

  if ($membersEnd -lt 0) {
    Write-Warning "Unterminated members array in Cargo.toml."
    return
  }

  $removalSet = [System.Collections.Generic.HashSet[string]]::new([StringComparer]::Ordinal)
  foreach ($member in $normalizedMembers) {
    [void]$removalSet.Add($member)
  }

  $removedCount = 0
  for ($i = $membersEnd - 1; $i -gt $membersStart; $i--) {
    $line = $cargoLines[$i].Trim()
    if ($line -match '"([^"]+)"' -and $removalSet.Contains($matches[1])) {
      $cargoLines.RemoveAt($i)
      $removedCount++
    }
  }

  if ($removedCount -eq 0) {
    return
  }

  Set-Content -Path $CargoPath -Value $cargoLines -Encoding UTF8
  $entryWord = if ($removedCount -eq 1) { "entry" } else { "entries" }
  Write-Host "Removed $removedCount $entryWord from Cargo.toml members after failure." -ForegroundColor Yellow
}

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

if (-not (Get-Command tsp-client -ErrorAction SilentlyContinue)) {
  Write-Error "tsp-client CLI not found in PATH. Install it via 'npm install -g @azure-tools/typespec-client' or ensure it's accessible."
  exit 1
}

try {
  $resolvedPath = (Resolve-Path -Path $Path).Path
}
catch {
  Write-Error "Unable to resolve path '$Path'. $_"
  exit 1
}

$specPathCandidate = Join-Path -Path $resolvedPath -ChildPath "specification"
if (Test-Path -Path $specPathCandidate -PathType Container) {
  $searchRoot = (Resolve-Path -Path $specPathCandidate).Path
}
elseif ((Split-Path -Path $resolvedPath -Leaf) -eq "specification") {
  $searchRoot = $resolvedPath
}
else {
  Write-Error "Could not locate a 'specification' directory under '$resolvedPath'. Provide the azure-rest-api-specs repo root or the specifications directory path."
  exit 1
}

Write-Host "Searching for config files ($($ConfigNames -join ', ')) under: $searchRoot" -ForegroundColor Green

$configFiles = @()
foreach ($name in $ConfigNames) {
  $found = Get-ChildItem -Path $searchRoot -Recurse -File -Filter $name -ErrorAction SilentlyContinue
  if ($found) {
    $configFiles += $found
  }
}

$configFiles = $configFiles | Sort-Object -Property FullName -Unique

if (-not $configFiles -or $configFiles.Count -eq 0) {
  Write-Host "No matching tspconfig files found." -ForegroundColor Yellow
  exit 0
}

Write-Host "Found $($configFiles.Count) config file(s)." -ForegroundColor Green

$configFiles = $configFiles | Where-Object {
  try {
    Select-String -Path $_.FullName -Pattern "typespec-rust" -SimpleMatch -Quiet
  }
  catch {
    Write-Warning "Unable to inspect file '$($_.FullName)': $_"
    $false
  }
}

if (-not $configFiles -or $configFiles.Count -eq 0) {
  Write-Host "No tspconfig files referencing the typespec-rust emitter were found." -ForegroundColor Yellow
  exit 0
}

Write-Host "Filtered to $($configFiles.Count) config file(s) referencing typespec-rust." -ForegroundColor Green

$configEntries = @()
foreach ($file in $configFiles) {
  try {
    $rawContent = Get-Content -Path $file.FullName -Raw
  }
  catch {
    Write-Warning "Unable to read '$($file.FullName)': $_"
    continue
  }

  try {
    $configData = ConvertFrom-TspYaml -Content $rawContent -SourcePath $file.FullName
  }
  catch {
    Write-Warning $_
    continue
  }

  if ($configData -is [System.Array]) {
    if ($configData.Count -gt 0) {
      $configData = $configData[0]
    }
    else {
      Write-Warning "Empty YAML document in '$($file.FullName)'. Skipping."
      continue
    }
  }

  $emitterConfig = $configData.options.'@azure-tools/typespec-rust'
  if (-not $emitterConfig) {
    Write-Warning "typespec-rust emitter configuration not found in '$($file.FullName)'. Skipping."
    continue
  }

  $crateName = $emitterConfig.'crate-name'
  if ([string]::IsNullOrWhiteSpace($crateName)) {
    Write-Warning "crate-name missing in '$($file.FullName)'. Skipping."
    continue
  }

  $parametersNode = $configData.parameters
  if (-not $parametersNode) {
    Write-Warning "parameters section missing in '$($file.FullName)'. Skipping."
    continue
  }

  $serviceDirRaw = $parametersNode.'service-dir'
  $serviceDir = $null

  if ($serviceDirRaw -is [string]) {
    $serviceDir = $serviceDirRaw
  }
  elseif ($serviceDirRaw -is [System.Collections.IDictionary]) {
    foreach ($key in @('value', 'default', 'path', 'dir')) {
      if ($serviceDirRaw.Contains($key) -and -not [string]::IsNullOrWhiteSpace($serviceDirRaw[$key])) {
        $serviceDir = [string]$serviceDirRaw[$key]
        break
      }
    }
  }
  elseif ($serviceDirRaw -is [System.Collections.IEnumerable]) {
    foreach ($candidate in $serviceDirRaw) {
      if ($candidate -is [string] -and -not [string]::IsNullOrWhiteSpace($candidate)) {
        $serviceDir = $candidate
        break
      }
    }
  }

  if ([string]::IsNullOrWhiteSpace($serviceDir)) {
    Write-Warning "service-dir missing in '$($file.FullName)'. Skipping."
    continue
  }

  $normalizedServiceDir = ($serviceDir -replace '\\', '/').Trim()
  $normalizedServiceDir = $normalizedServiceDir -replace '^[./]+', ''

  if ([string]::IsNullOrWhiteSpace($normalizedServiceDir)) {
    Write-Warning "Unable to normalize service-dir value in '$($file.FullName)'. Skipping."
    continue
  }

  $crateRelativePath = Join-Path -Path $normalizedServiceDir -ChildPath $crateName
  $crateRelativePath = ($crateRelativePath -replace '\\', '/').Trim()
  $crateRelativePath = $crateRelativePath -replace '^[./]+', ''

  if ([string]::IsNullOrWhiteSpace($crateRelativePath)) {
    Write-Warning "Unable to compute crate path for '$($file.FullName)'. Skipping."
    continue
  }

  $configEntries += [PSCustomObject]@{
    File = $file.FullName
    CrateName = $crateName
    ServiceDir = $normalizedServiceDir
    CratePath = $crateRelativePath
  }
}

if (-not $configEntries -or $configEntries.Count -eq 0) {
  Write-Host "No tspconfig files referencing typespec-rust with crate metadata were found." -ForegroundColor Yellow
  exit 0
}

Write-Host "Loaded emitter metadata for $($configEntries.Count) config file(s)." -ForegroundColor Green

$successes = @()
$failures = @()
$warnings = @()

try {
  $repoRoot = (Resolve-Path -Path (Join-Path $PSScriptRoot "../..")).Path
}
catch {
  Write-Error "Unable to resolve the azure-sdk-for-rust repository root relative to this script. $_"
  exit 1
}

$cargoPath = Join-Path -Path $repoRoot -ChildPath "Cargo.toml"

Write-Host "Running tsp-client init commands from repository root: $repoRoot" -ForegroundColor Green

$locationPushed = $false
try {
  Push-Location $repoRoot
  $locationPushed = $true

  foreach ($config in $configEntries) {
    $target = $config.File
    $action = "Run tsp-client init for crate $($config.CrateName)"

    if (-not $PSCmdlet.ShouldProcess($target, $action)) {
      continue
    }

    $addedMembers = @(Add-CargoMembers -CargoPath $cargoPath -MembersToAdd @($config.CratePath))

    $fullServicePath = Join-Path -Path $repoRoot -ChildPath $config.CratePath
    if (-not (Test-Path -Path $fullServicePath)) {
      Write-Warning "Crate directory '$($config.CratePath)' does not currently exist under the SDK repository."
    }

    Write-Host "\nRunning tsp-client init for crate '$($config.CrateName)' using config: $($config.File)" -ForegroundColor Cyan

    $exitCode = $null
    try {
      npm exec --prefix eng/common/tsp-client/ -- tsp-client init -c $config.File
      $exitCode = $LASTEXITCODE
    }
    catch {
      $failureMessage = "Error running tsp-client init for crate '$($config.CrateName)' (config: $($config.File)) - $_"
      Write-Error $failureMessage
      $failures += $failureMessage
      if ($addedMembers.Count -gt 0) {
        Remove-CargoMembers -CargoPath $cargoPath -MembersToRemove $addedMembers
      }
      continue
    }

    if ($exitCode -ne 0) {
      $failureMessage = "tsp-client init failed for crate '$($config.CrateName)' (config: $($config.File)) (exit code: $exitCode)"
      Write-Warning $failureMessage
      $failures += $failureMessage
      $warnings += $failureMessage
      if ($addedMembers.Count -gt 0) {
        Remove-CargoMembers -CargoPath $cargoPath -MembersToRemove $addedMembers
      }
    }
    else {
      $successMessage = "$($config.CrateName) [$($config.CratePath)]"
      Write-Host "Successfully initialized: $successMessage" -ForegroundColor Green
      $successes += $successMessage
    }
  }
}
finally {
  if ($locationPushed) {
    Pop-Location
  }
}

Write-Host "\nClient generation summary:" -ForegroundColor Green
Write-Host "  Successful ($($successes.Count)):" -ForegroundColor Green
if ($successes.Count -gt 0) {
  $successes | ForEach-Object { Write-Host "    $_" }
}
else {
  Write-Host "    None"
}

Write-Host "  Failed ($($failures.Count)):" -ForegroundColor Yellow
if ($failures.Count -gt 0) {
  $failures | ForEach-Object { Write-Host "    $_" }
}
else {
  Write-Host "    None"
}

Write-Host "  Warnings ($($warnings.Count)):" -ForegroundColor Yellow
if ($warnings.Count -gt 0) {
  $warnings | ForEach-Object { Write-Host "    $_" }
}
else {
  Write-Host "    None"
}

if ($failures.Count -gt 0) {
  Write-Host "\nCompleted initializing TypeSpec configs with failures." -ForegroundColor Yellow
  exit 1
}

Write-Host "\nCompleted initializing all TypeSpec configs." -ForegroundColor Green
