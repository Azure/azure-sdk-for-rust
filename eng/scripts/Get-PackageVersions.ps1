[CmdletBinding(DefaultParameterSetName = 'none')]
param(
  [Parameter(ParameterSetName = "versions")]
  [switch]$Versions,
  [Parameter(ParameterSetName = "dependencies")]
  [switch]$Dependencies
)

$metadata = cargo metadata --format-version 1 --no-deps --all-features | ConvertFrom-Json -AsHashtable
$packages = $metadata.packages
foreach ($package in $packages) {
  try {
    $resp = Invoke-RestMEthod "https://crates.io/api/v1/crates/$($package.name)"
    $package.released_version = $resp.crate.max_stable_version
  }
  catch {
  }
  $package.publish = $null -eq $package.publish

  foreach ($dependency in $package.dependencies) {
    $dependencyPackage = $packages | Where-Object -Property name -EQ -Value $dependency.name | Select-Object -First 1

    if ($dependencyPackage) {
      $ordered = [ordered]@{
        dependant        = $package.name
        pathVersion      = $dependencyPackage.version
        released_version = $dependencyPackage.released_version
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
  $packages | Select-Object name, version, publish, released_version
}
elseif ($Dependencies) {
  $packages.packageDependencies | Select-Object dependant, @{Name = 'dependency'; Expression = { $_.name } }, kind, req, @{Name = 'released'; Expression = { $_.released_version } }, pathVersion, @{Name = 'byPath'; Expression = { !!$_.path } }
}
else {
  $packages | Select-Object name, version, publish, released_version, packageDependencies, dependantPackages
}
