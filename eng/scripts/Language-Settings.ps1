$Language = "rust"
$LanguageDisplayName = "Rust"
$PackageRepository = "crates.io"
$packagePattern = "Cargo.toml"
$MetadataUri = "https://raw.githubusercontent.com/Azure/azure-sdk/main/_data/releases/latest/rust-packages.csv"
$GithubUri = "https://github.com/Azure/azure-sdk-for-rust"
$PackageRepositoryUri = "https://crates.io/crates"

function Get-AllPackageInfoFromRepo ([string] $ServiceDirectory) {
    $allPackageProps = @()
    Push-Location $RepoRoot
    try {
        $searchPath = Join-Path $RepoRoot 'sdk' -Resolve

        if ($ServiceDirectory -and $ServiceDirectory -ne 'auto') {
            $searchPath = Join-Path 'sdk' $ServiceDirectory
        }

        $packages = cargo metadata --format-version 1
        | ConvertFrom-Json -AsHashtable
        | Select-Object -ExpandProperty packages
        | Where-Object { $_.manifest_path.StartsWith($searchPath) }

        $packageManifests = @{}
        foreach ($package in $packages) {
            if ($package.manifest_path -replace '\\', '/' -match '/sdk/([^/]+)/') {
                $package.ServiceDirectoryName = $Matches[1]
            }
            else {
                # ignore manifests that are not in a service directory
                continue
            }

            $package.RelativePath = (Split-Path $package.manifest_path -Parent).Replace($RepoRoot, "").SubString(1)
            $package.DependentPackages = @()
            $packageManifests[$package.name] = $package
        }
    }
    catch {
        # This is soft error and failure is expected for python metapackages
        LogError "Failed to get all package properties"
    }
    finally {
        Pop-Location
    }

    # Invert the manifest dependency graph
    foreach ($package in $packageManifests.Values) {
        foreach ($dependency in $package.dependencies) {
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
        $absolutePath = Split-Path $package.manifest_path -Parent -Resolve
        $pkgProp = [PackageProps]::new($package.name, $package.version, $absolutePath, $package.ServiceDirectoryName)
        $pkgProp.IsNewSdk = $true
        $pkgProp.ArtifactName = $package.name

        if ($package.name -match "mgmt") {
            $pkgProp.SdkType = "mgmt"
        }
        else {
            $pkgProp.SdkType = "client"
        }

        $pkgProp.DependentPackages = GetDependentPackages $package | Select-Object -ExpandProperty RelativePath

        $allPackageProps += $pkgProp
    }

    return $allPackageProps
}
