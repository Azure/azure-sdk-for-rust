# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

BeforeAll {
  Import-Module powershell-yaml -MinimumVersion 0.4.7 -ErrorAction Stop
  $cosmosRoot = [IO.Path]::GetFullPath([IO.Path]::Combine($PSScriptRoot, '..', '..'))
  $repoRoot = [IO.Path]::GetFullPath([IO.Path]::Combine($cosmosRoot, '..', '..'))
  $template = Get-Content ([IO.Path]::Combine($PSScriptRoot, 'prepare-test-resource-modules.yml')) -Raw | ConvertFrom-Yaml
  $step = $template.steps[0]
  $prepare = [scriptblock]::Create($step.pwsh)
}

Describe 'Cosmos test-resource module preparation' {
  BeforeAll {
    function Register-PSRepository {
      [CmdletBinding()]
      param([switch] $Default)
      throw 'Register-PSRepository must be mocked.'
    }
  }

  BeforeEach {
    $global:CosmosAzModule = [pscustomobject]@{ Version = [version]'16.3.0'; Path = '/modules/Az/16.3.0/Az.psd1' }
    $global:CosmosAzImported = $false
    $global:CosmosGallery = [pscustomobject]@{ Name = 'PSGallery'; SourceLocation = 'https://www.powershellgallery.com/api/v2' }
    Mock Get-Module { if ($global:CosmosAzModule) { $global:CosmosAzModule } } -ParameterFilter { $Name -eq 'Az' -and $ListAvailable }
    Mock Get-PSRepository { if ($global:CosmosGallery) { $global:CosmosGallery } }
    Mock Register-PSRepository {
      $global:CosmosGallery = [pscustomobject]@{ Name = 'PSGallery'; SourceLocation = 'https://www.powershellgallery.com/api/v2' }
    }
    Mock Install-Module {
      if ($global:CosmosAzImported) { throw 'Module assemblies are in use' }
      $global:CosmosAzModule = [pscustomobject]@{ Version = [version]'16.3.0'; Path = '/modules/Az/16.3.0/Az.psd1' }
    }
    Mock Import-Module { $global:CosmosAzImported = $true }
    Mock Get-Command { [pscustomobject]@{ Name = $Name } } -ParameterFilter { $Name -like 'Az.*\*' }
    Mock Write-Host {}
  }

  AfterAll {
    Remove-Variable CosmosAzModule, CosmosAzImported, CosmosGallery -Scope Global
  }

  It 'reuses installed modules without a repository and verifies provisioning and cleanup commands' {
    & $prepare
    Should -Invoke Import-Module -Times 1 -Exactly -ParameterFilter { $Name -eq '/modules/Az/16.3.0/Az.psd1' -and $ErrorAction -eq 'Stop' }
    Should -Invoke Install-Module -Times 0 -Exactly
    Should -Invoke Get-PSRepository -Times 0 -Exactly
    foreach ($expectedCommand in @(
      'Az.Resources\Get-AzResource',
      'Az.Resources\New-AzResourceGroupDeployment',
      'Az.Resources\Remove-AzResourceGroup',
      'Az.StorageSync\Get-AzStorageSyncService'
    )) {
      Should -Invoke Get-Command -Times 1 -Exactly -ParameterFilter { $Name -eq $expectedCommand -and $ErrorAction -eq 'Stop' }
    }
    Should -Invoke Write-Host -Times 1 -Exactly -ParameterFilter { $Object -eq 'Verified Az 16.3.0 for Cosmos test-resource provisioning.' }
  }

  It 'installs the compatible bundle before imports and is idempotent' {
    $global:CosmosAzModule = $null
    & $prepare
    & $prepare
    Should -Invoke Install-Module -Times 1 -Exactly -ParameterFilter {
      $Name -eq 'Az' -and $RequiredVersion -eq '16.3.0' -and $Repository -eq 'PSGallery' -and
      $Scope -eq 'CurrentUser' -and $Force -and $ErrorAction -eq 'Stop'
    }
    Should -Invoke Import-Module -Times 2 -Exactly
  }

  It 'replaces an older bundle that cannot meet current deployment requirements' {
    $global:CosmosAzModule.Version = [version]'5.7.0'
    & $prepare
    Should -Invoke Install-Module -Times 1 -Exactly
  }

  It 'registers missing PSGallery before installation' {
    $global:CosmosAzModule = $null
    $global:CosmosGallery = $null
    & $prepare
    Should -Invoke Register-PSRepository -Times 1 -Exactly -ParameterFilter { $Default -and $ErrorAction -eq 'Stop' }
    Should -Invoke Install-Module -Times 1 -Exactly
  }

  It 'rejects an unexpected repository source' {
    $global:CosmosAzModule = $null
    $global:CosmosGallery.SourceLocation = 'https://example.invalid'
    { & $prepare } | Should -Throw '*PSGallery must use*'
    Should -Invoke Install-Module -Times 0 -Exactly
  }

  It 'fails when repository registration fails' {
    $global:CosmosAzModule = $null
    $global:CosmosGallery = $null
    Mock Register-PSRepository { throw 'Repository unavailable' }
    { & $prepare } | Should -Throw '*Repository unavailable*'
    Should -Invoke Install-Module -Times 0 -Exactly
  }

  It 'fails before importing when installation fails' {
    $global:CosmosAzModule = $null
    Mock Install-Module { throw 'Module installation failed' }
    { & $prepare } | Should -Throw '*Module installation failed*'
    Should -Invoke Import-Module -Times 0 -Exactly
    Should -Invoke Get-Command -Times 0 -Exactly -ParameterFilter { $Name -like 'Az.*\*' }
  }

  It 'fails if installation leaves the bundle undiscoverable' {
    $global:CosmosAzModule = $null
    Mock Install-Module {}
    { & $prepare } | Should -Throw '*not discoverable after installation*'
    Should -Invoke Import-Module -Times 0 -Exactly
  }

  It 'fails before command verification when import fails' {
    Mock Import-Module { throw 'Module import failed' }
    { & $prepare } | Should -Throw '*Module import failed*'
    Should -Invoke Get-Command -Times 0 -Exactly -ParameterFilter { $Name -like 'Az.*\*' }
    Should -Invoke Write-Host -Times 0 -Exactly
  }

  It 'fails if <Command> remains unavailable after loading modules' -ForEach @(
    @{ Command = 'Az.Resources\Get-AzResource' }
    @{ Command = 'Az.StorageSync\Get-AzStorageSyncService' }
  ) {
    Mock Get-Command { throw 'Required command unavailable' } -ParameterFilter { $Name -eq $Command }
    { & $prepare } | Should -Throw '*Required command unavailable*'
    Should -Invoke Write-Host -Times 0 -Exactly
  }
}

Describe 'Cosmos live-test prerequisite scope' {
  BeforeAll {
    $ci = Get-Content ([IO.Path]::Combine($cosmosRoot, 'ci.yml')) -Raw | ConvertFrom-Yaml
    $jobText = Get-Content ([IO.Path]::Combine($repoRoot, 'eng', 'pipelines', 'templates', 'jobs', 'live.tests.yml')) -Raw
    $stageText = Get-Content ([IO.Path]::Combine($repoRoot, 'eng', 'pipelines', 'templates', 'stages', 'archetype-sdk-client.yml')) -Raw
  }

  It 'uses the live PreSteps hook before subnet lookup without modifying shared templates' {
    $ci.extends.parameters.PreSteps.Count | Should -Be 1
    $ci.extends.parameters.PreSteps[0].template | Should -Be '/sdk/cosmos/eng/pipelines/prepare-test-resource-modules.yml'
    $jobText.IndexOf('${{ parameters.PreSteps }}') | Should -BeGreaterThan -1
    $jobText.IndexOf('${{ parameters.PreSteps }}') | Should -BeLessThan $jobText.IndexOf('/eng/common/TestResources/build-test-resource-config.yml')
    $stageText.Substring(0, $stageText.IndexOf('- stage: ${{ cloud.key }}')) | Should -Not -Match 'PreSteps: \$\{\{ parameters.PreSteps \}\}'
  }

  It 'is blocking and runs only for internal jobs without a fixed account selector' {
    $step.condition | Should -Be "and(succeeded(), eq(variables['System.TeamProject'], 'internal'), eq(variables['AccountSelector'], ''))"
    $step.ContainsKey('continueOnError') | Should -BeFalse
    $step.workingDirectory | Should -Be '$(Build.SourcesDirectory)'
    $step.pwsh | Should -Not -Match '##vso|\$env:|Connect-AzAccount'
  }

  It 'skips every fixed-account entry and includes every ARM-provisioned entry' {
    foreach ($config in $ci.extends.parameters.FixedAccountMatrixConfigs) {
      $matrix = Get-Content ([IO.Path]::Combine($repoRoot, $config.Path)) -Raw | ConvertFrom-Json -AsHashtable
      foreach ($entry in $matrix.matrix['Account Settings'].Values) {
        $entry.AccountSelector | Should -Not -BeNullOrEmpty
      }
    }
    foreach ($config in $ci.extends.parameters.LiveTestMatrixConfigs) {
      $text = Get-Content ([IO.Path]::Combine($repoRoot, $config.Path)) -Raw
      $text | Should -Not -Match '"AccountSelector"'
    }
  }
}
