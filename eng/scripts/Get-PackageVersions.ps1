[CmdletBinding(DefaultParameterSetName = 'none')]
param(
  [Parameter(ParameterSetName = "versions")]
  [switch]$Versions,
  [Parameter(ParameterSetName = "dependencies")]
  [switch]$Dependencies
)

. $PSScriptRoot/../common/scripts/common.ps1

$metadata = cargo metadata --format-version 1 --no-deps --all-features | ConvertFrom-Json -AsHashtable
$packages = $metadata.packages
foreach ($package in $packages) {
  try {
    $name = $package.name
    $resp = Invoke-WebRequest "https://index.crates.io/$($name.Substring(0,2))/$($name.Substring(2,2))/$name"
    $packageVersions = $resp.Content.Trim().Split("`n") | ConvertFrom-Json | Select-Object -ExpandProperty vers
    $package.indexVersion = $packageVersions | Sort-Object { [AzureEngSemanticVersion]::ParseVersionString($_) } | Select-Object -Last 1
  }
  catch {
  }
  $package.publish = $null -eq $package.publish

  foreach ($dependency in $package.dependencies) {
    $dependencyPackage = $packages | Where-Object -Property name -EQ -Value $dependency.name | Select-Object -First 1

    if ($dependencyPackage) {
      $ordered = [ordered]@{
        dependant    = $package.name
        pathVersion  = $dependencyPackage.version
        indexVersion = $dependencyPackage.indexVersion
      }

      foreach ($key in $dependency.Keys) {
        $ordered[$key] = $dependency[$key]
      }

      $dependencyPackage.dependantPackages ??= @()
      $package.packageDependencies ??= @()

      $dependencyPackage.dependantPackages += [hashtable]$ordered
      $package.packageDependencies += [hashtable]$ordered
    }
  }
}

if ($Versions) {
  $packages | Select-Object name, version, publish, indexVersion
}
elseif ($Dependencies) {
  $packages.packageDependencies | Select-Object -Property @(
    @{ Name = 'from'; Expression = { $_.name } },
    @{ Name = 'to'; Expression = { $_.dependant } },
    'kind',
    'req',
    @{ Name = 'version'; Expression = { $_.pathVersion } },
    @{ Name = 'index'; Expression = { $_.indexVersion } },
    @{ Name = 'byPath'; Expression = { !!$_.path } }
  )
}
else {
  $packages | Select-Object name, version, publish, indexVersion, packageDependencies, dependantPackages
}
