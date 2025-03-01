#!/usr/bin/env pwsh

#Requires -Version 7.0

[CmdletBinding()]
param(
  [Parameter(Mandatory = $true, ValueFromPipelineByPropertyName = $true)]
  [string] $Name,

  [Parameter(HelpMessage = "Number of seconds to sleep")]
  [ValidateRange(0, 600)]
  [int] $Sleep = 0,

  [Parameter()]
  [switch] $WhatIf
)

Begin {
  $ErrorActionPreference = 'Stop'
}

Process {
  [string[]] $additionalArgs = if ($WhatIf) {
    @('--dry-run')
  }
  $idx0 = $Name.Substring(0, 2)
  $idx1 = $Name.Substring(2, 2)

  $resp = Invoke-WebRequest "https://index.crates.io/$idx0/$idx1/$Name" -SkipHttpErrorCheck -Verbose:$false
  if ($resp.StatusCode -eq 404) {
    Write-Verbose "Publishing $Name"
    cargo publish --package $Name $additionalArgs
    if (!$?) {
      exit 1
    }

    if ($Sleep) {
      Start-Sleep -Seconds $Sleep
    }
  }
  else {
    Write-Verbose "$Name is already published; skipping"
  }
}
