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
        $searchPath = 'sdk'

        if ($ServiceDirectory -and $ServiceDirectory -ne 'auto') {
            $searchPath = Join-Path 'sdk' $ServiceDirectory
        }

        $cargoTomlFiles = Get-ChildItem -Path $searchPath -Recurse -Filter 'Cargo.toml' -ErrorAction SilentlyContinue

        $packageManifests = @{}
        foreach ($file in $cargoTomlFiles) {
            $manifest = cargo read-manifest --manifest-path $file | ConvertFrom-Json -AsHashtable

            if ($manifest.manifest_path -replace '\\', '/' -match '/sdk/([^/]+)/') {
                $manifest.ServiceDirectoryName = $Matches[1]
            }
            else {
                # ignore manifests that are not in a service directory
                continue
            }

            $manifest.RelativePath = (Split-Path $manifest.manifest_path -Parent).Replace($RepoRoot, "").SubString(1)
            $manifest.DependentPackages = @()
            $packageManifests[$manifest.name] = $manifest
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
    foreach ($manifest in $packageManifests.Values) {
        foreach ($dependency in $manifest.dependencies) {
            $dependencyManifest = $packageManifests[$dependency.name]
            if ($dependencyManifest) {
                $dependencyManifest.DependentPackages += $manifest
            }
        }
    }

    # Flatten the dependency graph recursively
    function GetDependentPackages($manifest, $dependantPackages) {
        if (!$dependantPackages) {
            $dependantPackages = @()
        }

        foreach ($dependency in $manifest.DependentPackages) {
            if ($dependantPackages.Contains($dependency)) {
                continue
            }
            $dependantPackages += $dependency
            [array]$dependantPackages = GetDependentPackages $dependency $dependantPackages
        }

        return $dependantPackages;
    }

    foreach ($manifest in $packageManifests.Values) {
        $absolutePath = Split-Path $manifest.manifest_path -Parent -Resolve
        $pkgProp = [PackageProps]::new($manifest.name, $manifest.version, $absolutePath, $manifest.ServiceDirectoryName)
        $pkgProp.IsNewSdk = $true
        $pkgProp.ArtifactName = $manifest.name

        if ($manifest.name -match "mgmt") {
            $pkgProp.SdkType = "mgmt"
        }
        else {
            $pkgProp.SdkType = "client"
        }

        $pkgProp.DependentPackages = GetDependentPackages $manifest | Select-Object -ExpandProperty RelativePath

        $allPackageProps += $pkgProp
    }

    return $allPackageProps
}
