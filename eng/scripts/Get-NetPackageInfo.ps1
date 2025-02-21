#!/usr/bin/env pwsh

#Requires -Version 7.0

[CmdletBinding()]
param(
  [Parameter(Mandatory = $true, Position = 0)]
  [string] $SdkRoot
)

foreach ($serviceDirectory in (Join-Path $SdkRoot 'sdk' -Resolve | Get-ChildItem -Directory)) {
  foreach ($packageDirectory in (Get-ChildItem $serviceDirectory -Directory)) {
    foreach ($projectFile in (Join-Path $packageDirectory 'src' -Resolve -ErrorAction SilentlyContinue | Get-ChildItem -Filter *.csproj)) {
      $packageName = $projectFile.BaseName.ToLowerInvariant().Replace('.', '_')
      if (-not $packageName.StartsWith("azure")) {
        continue
      }
      $existingPackageName = $packageName

      [xml] $projectXml = Get-Content $projectFile
      $description = (([string]($projectXml.Project.PropertyGroup.Description -join '') -split "`n").Trim() -join ' ').Trim()

      $idx0 = $packageName.Substring(0, 2)
      $idx1 = $packageName.Substring(2, 2)

      $resp = Invoke-WebRequest "https://index.crates.io/$idx0/$idx1/$packageName" -SkipHttpErrorCheck
      if ($resp.StatusCode -eq 404) {
        $existingPackageName = $packageName -replace '_', '-'
        $resp = Invoke-WebRequest "https://index.crates.io/$idx0/$idx1/$existingPackageName" -SkipHttpErrorCheck
      }

      $exists = $resp.StatusCode -eq 200
      $microsoftOwned = $false
      [string[]] $owners = $()

      if ($exists) {
        # If we found a package, get the publisher. There is a 1s per request rate limit.
        Start-Sleep -Seconds 1

        $resp = Invoke-WebRequest "https://crates.io/api/v1/crates/$existingPackageName/owners" -SkipHttpErrorCheck
        if ($resp.StatusCode -ne 200) {
          Write-Error "No owner information for existing package $existingPackageName"
          continue
        }

        $json = $resp.Content | ConvertFrom-Json
        [string[]] $owners = $json.users.login
        $microsoftOwned = $owners -contains 'azure-sdk' -or $owners -contains 'heaths' -or $owners -contains 'github:azure:azure-sdk-publish-rust'

      }
      else {
        $existingPackageName = ''
      }

      [pscustomobject] @{
        ServiceDirectory    = $serviceDirectory.BaseName.ToLowerInvariant()
        Name                = $packageName
        Description         = $description
        Exists              = $exists
        ExistingPackageName = $existingPackageName
        MicrosoftOwned      = $microsoftOwned -or $false
        Owners              = $owners -join ';'
        Publish             = $true
      }
    }
  }
}
