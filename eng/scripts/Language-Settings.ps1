$Language = "rust"
$LanguageDisplayName = "Rust"
$PackageRepository = "crates.io"
$packagePattern = "*.crate"
#$MetadataUri = "https://raw.githubusercontent.com/Azure/azure-sdk/main/_data/releases/latest/rust-packages.csv"
$GithubUri = "https://github.com/Azure/azure-sdk-for-rust"
$PackageRepositoryUri = "https://crates.io/crates"
$SupportsTestResourcesDotenv = $true

. (Join-Path $EngCommonScriptsDir "Helpers" "PSModule-Helpers.ps1")

function SetPackageVersion ($PackageName, $Version, $ServiceDirectory, $ReleaseDate, $ReplaceLatestEntryTitle = $true) {
  if ($null -eq $ReleaseDate) {
    $ReleaseDate = Get-Date -Format "yyyy-MM-dd"
  }
  & "$EngDir/scripts/Update-PackageVersion.ps1" -ServiceDirectory $ServiceDirectory -PackageName $PackageName `
    -NewVersionString $Version -ReleaseDate $ReleaseDate -ReplaceLatestEntryTitle $ReplaceLatestEntryTitle
}

function GetExistingPackageVersions ($PackageName, $GroupId = $null) {
  try {
    $PackageName = $PackageName.ToLower()
    $response = Invoke-RestMethod -Method GET -Uri "https://crates.io/api/v1/crates/${PackageName}/versions"
    $existingVersions = $response.versions `
    | Sort-Object { [AzureEngSemanticVersion]::new($_.num) } `
    | Select-Object -ExpandProperty num
    return $existingVersions
  }
  catch {
    if ($_.Exception.Response.StatusCode -ne 404) {
      LogError "Failed to retrieve package versions for ${PackageName}. $($_.Exception.Message)"
    }
    return $null
  }
}

function Get-AllPackageInfoFromRepo ([string] $ServiceDirectory) {
  $allPackageProps = @()
  Push-Location $RepoRoot
  try {
    $searchPath = Join-Path $RepoRoot 'sdk' -Resolve

    if ($ServiceDirectory -and $ServiceDirectory -ne 'auto') {
      $searchPath = Join-Path $searchPath $ServiceDirectory -Resolve
    }

    # when a package is marked `publish = false` in the Cargo.toml, `cargo metadata` returns an empty array for
    # `publish`, otherwise it returns null. We only want to include packages where `publish` is null.
    $packages = Invoke-LoggedCommand "cargo metadata --format-version 1 --no-deps" -GroupOutput
    | ConvertFrom-Json -AsHashtable
    | Select-Object -ExpandProperty packages
    | Where-Object { $_.manifest_path.StartsWith($searchPath) -and ($null -eq $_.publish -or $_.publish.Count -gt 0) }

    $packageManifests = @{}
    foreach ($package in $packages) {
      if ($package.manifest_path -replace '\\', '/' -match '/sdk/([^/]+)/') {
        $package.ServiceDirectoryName = $Matches[1]
      }
      else {
        # ignore manifests that are not in a service directory
        continue
      }

      $package.DirectoryPath = Split-Path $package.manifest_path -Parent
      $package.DependentPackages = @()

      # Collect the crate types available in this package
      $package.CrateTypes = $package.targets | Select-Object -ExpandProperty crate_types | Select-Object -Unique

      $packageManifests[$package.name] = $package
    }
  }
  finally {
    Pop-Location
  }

  # Invert the manifest dependency graph
  foreach ($package in $packageManifests.Values) {
    foreach ($dependency in $package.dependencies | Where-Object { $null -eq $_.kind }) {
      $dependencyManifest = $packageManifests[$dependency.name]
      if ($dependencyManifest) {
        $dependencyManifest.DependentPackages += $package
      }
    }
  }

  # Flatten the dependency graph recursively
  function GetDependentPackages($package, $dependantPackages) {
    if (!$dependantPackages) {
      $dependantPackages = @()
    }

    foreach ($dependency in $package.DependentPackages) {
      if ($dependantPackages.Contains($dependency)) {
        continue
      }
      $dependantPackages += $dependency
      [array]$dependantPackages = GetDependentPackages $dependency $dependantPackages
    }

    return $dependantPackages;
  }

  foreach ($package in $packageManifests.Values) {
    $pkgProp = [PackageProps]::new($package.name, $package.version, $package.DirectoryPath, $package.ServiceDirectoryName)
    $pkgProp.IsNewSdk = $true
    $pkgProp.ArtifactName = $package.name
    $pkgProp.CrateTypes = $package.CrateTypes

    if ($package.name -match "mgmt") {
      $pkgProp.SdkType = "mgmt"
    }
    else {
      $pkgProp.SdkType = "client"
    }

    $pkgProp.AdditionalValidationPackages = GetDependentPackages $package | Select-Object -ExpandProperty DirectoryPath

    $allPackageProps += $pkgProp
  }

  return $allPackageProps
}

function Get-rust-AdditionalValidationPackagesFromPackageSet ($packagesWithChanges, $diff, $allPackageProperties) {
  # if the change was in a service directory, but not in a package directory, test all the packages in the service directory
  [array]$serviceFiles = ($diff.ChangedFiles + $diff.DeletedFiles) | ForEach-Object { $_ -replace '\\', '/' } | Where-Object { $_ -match "^sdk/.+/" }

  # remove files that target any specific package
  foreach ($package in $allPackageProperties) {
    $packagePathPattern = "^$( [Regex]::Escape($package.DirectoryPath.Replace('\', '/')) )/"
    $serviceFiles = $serviceFiles | Where-Object { "$RepoRoot/$_".Replace('\', '/') -notmatch $packagePathPattern }
  }

  $affectedServiceDirectories = $serviceFiles | ForEach-Object { $_ -replace '^sdk/(.+?)/.*', '$1' } | Sort-Object -Unique

  $affectedPackages = @($allPackageProperties | Where-Object { $affectedServiceDirectories -contains $_.ServiceDirectory })
  $additionalPackages = @($affectedPackages | Where-Object { $packagesWithChanges -notcontains $_ })

  foreach ($package in $additionalPackages) {
    $package.IncludedForValidation = $true
  }

  return $additionalPackages ?? @()
}

# $GetPackageInfoFromPackageFileFn = "Get-${Language}-PackageInfoFromPackageFile"
function Get-rust-PackageInfoFromPackageFile([IO.FileInfo]$pkg, [string]$workingDirectory) {
  # Create a temporary folder for extraction
  $extractionPath = [System.IO.Path]::Combine([System.IO.Path]::GetTempPath(), [System.IO.Path]::GetRandomFileName())
  New-Item -ItemType Directory -Path $extractionPath | Out-Null

  # Extract the .crate file (which is a tarball) to the temporary folder
  tar -xvf $pkg.FullName -C $extractionPath
  $cargoTomlPath = [System.IO.Path]::Combine($extractionPath, $pkg.BaseName, 'Cargo.toml')

  Write-Host "Reading package info from $cargoTomlPath"
  if (!(Test-Path $cargoTomlPath)) {
    $message = "The Cargo.toml file was not found in the package artifact at $cargoTomlPath"
    LogError $message
    throw $message
  }

  $package = cargo read-manifest --manifest-path $cargoTomlPath | ConvertFrom-Json

  $packageName = $package.name
  $packageVersion = $package.version

  $packageAssetPath = [System.IO.Path]::Combine($extractionPath, "$packageName-$packageVersion")

  $changeLogLoc = Get-ChildItem -Path $packageAssetPath -Filter "CHANGELOG.md" | Select-Object -First 1
  $readmeContentLoc = Get-ChildItem -Path $packageAssetPath -Filter "README.md" | Select-Object -First 1

  if ($changeLogLoc) {
    $releaseNotes = Get-ChangeLogEntryAsString -ChangeLogLocation $changeLogLoc -VersionString $packageVersion
  }

  if ($readmeContentLoc) {
    $readmeContent = Get-Content -Raw $readmeContentLoc
  }

  $existingVersions = GetExistingPackageVersions -PackageName $packageName

  return @{
    PackageId      = $packageName
    PackageVersion = $packageVersion
    ReleaseTag     = "$packageName@$packageVersion"
    Deployable     = $existingVersions -notcontains $packageVersion
    ReleaseNotes   = $releaseNotes
    ReadmeContent  = $readmeContent
  }
}

function Find-rust-Artifacts-For-Apireview([string]$ArtifactPath, [string]$packageName) {
  [array]$files = Get-ChildItem -Path $ArtifactPath -Recurse -Filter "$packageName.rust.json"

  if (!$files) {
    Write-Host "$($packageName) does not have api review json"
    return $null
  }
  elseif ($files.Count -ne 1) {
    Write-Host "$($artifactPath) should contain only one api review for $($packageName)"
    Write-Host "Number of files $($files.Count)"
    return $null
  }
  $packages = @{
    $files[0].Name = $files[0].FullName
  }
  return $packages
}
