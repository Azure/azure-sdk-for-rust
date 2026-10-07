# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# Run with Invoke-Pester ./eng/scripts/tests/Install-VcpkgCacheModules.Tests.ps1.
# Requires Pester 5 and powershell-yaml >= 0.4.7 for the template contract tests.

BeforeAll {
  $scriptPath = [System.IO.Path]::Combine($PSScriptRoot, '..', 'Install-VcpkgCacheModules.ps1')
}

Describe 'Vcpkg cache Az module bootstrap' {
  BeforeAll {
    # Avoid PowerShellGet's provider-dependent dynamic parameters during mocking.
    function Register-PSRepository {
      [CmdletBinding()]
      param([switch] $Default)
      throw 'Register-PSRepository must be mocked.'
    }
  }

  AfterAll {
    Remove-Variable VcpkgTestAvailable, VcpkgTestRepository, VcpkgTestInstalled, VcpkgTestImported, VcpkgTestOutput -Scope Global
  }

  BeforeEach {
    $global:LASTEXITCODE = 0
    $global:VcpkgTestOutput = @()
    $global:VcpkgTestAvailable = @{
      'Az.Accounts' = @([pscustomobject]@{
        Name = 'Az.Accounts'; Version = [version]'5.5.3'; Path = '/modules/Az.Accounts/5.5.3/Az.Accounts.psd1'
      })
      'Az.Storage' = @([pscustomobject]@{
        Name = 'Az.Storage'; Version = [version]'9.7.2'; Path = '/modules/Az.Storage/9.7.2/Az.Storage.psd1'
      })
    }
    $global:VcpkgTestRepository = [pscustomobject]@{ SourceLocation = 'https://www.powershellgallery.com/api/v2'; Name = 'PSGallery' }
    $global:VcpkgTestInstalled = @()
    $global:VcpkgTestImported = @()
    Mock Get-Module {
      if ($global:VcpkgTestAvailable.ContainsKey($Name[0])) {
        $global:VcpkgTestAvailable[$Name[0]]
      }
    } -ParameterFilter { $ListAvailable -and $Name -like 'Az.*' }
    Mock Get-PSRepository {
      if ($global:VcpkgTestRepository) { $global:VcpkgTestRepository }
    }
    Mock Register-PSRepository {
      $global:VcpkgTestRepository = [pscustomobject]@{ SourceLocation = 'https://www.powershellgallery.com/api/v2'; Name = 'PSGallery' }
    }
    Mock Install-Module {
      if ($global:VcpkgTestImported.Count -gt 0) {
        throw 'Installing with loaded modules can lock dependency assemblies on Windows.'
      }
      $global:VcpkgTestInstalled += $Name
      $global:VcpkgTestAvailable[$Name[0]] = @([pscustomobject]@{
        Name = $Name; Version = [version]$RequiredVersion; Path = "/modules/$Name/$RequiredVersion/$Name.psd1"
      })
    }
    Mock Import-Module { $global:VcpkgTestImported += $Name }
    Mock Get-Command {
      [pscustomobject]@{ Parameters = @{
        ServicePrincipal = @{}; Tenant = @{}; FederatedToken = @{}; Scope = @{}; Force = @{}
        Subscription = @{}; StorageAccountName = @{}; UseConnectedAccount = @{}
        Name = @{}; Permission = @{}; Context = @{}; ExpiryTime = @{}
      } }
    } -ParameterFilter { $Name -like 'Az.*\*' }
    Mock Write-Host { $global:VcpkgTestOutput += ($Object -join ' ') }
  }

  It 'imports existing compatible modules without contacting a repository' {
    & $scriptPath
    $LASTEXITCODE | Should -Be 0
    Should -Invoke Install-Module -Times 0 -Exactly
    Should -Invoke Get-PSRepository -Times 0 -Exactly
    Should -Invoke Import-Module -Times 2 -Exactly -ParameterFilter { $Global -and $ErrorAction -eq 'Stop' }
    Should -Invoke Get-Command -Times 5 -Exactly -ParameterFilter { $Name -like 'Az.*\*' }
  }

  It 'selects the highest available Accounts version as AzurePowerShell does' {
    $global:VcpkgTestAvailable['Az.Accounts'] += [pscustomobject]@{
      Name = 'Az.Accounts'; Version = [version]'5.6.0'; Path = '/modules/Az.Accounts/5.6.0/Az.Accounts.psd1'
    }
    & $scriptPath
    $LASTEXITCODE | Should -Be 0
    Should -Invoke Import-Module -Times 1 -Exactly -ParameterFilter { $Name -eq '/modules/Az.Accounts/5.6.0/Az.Accounts.psd1' }
    Should -Invoke Install-Module -Times 0 -Exactly
  }

  It 'installs both missing modules in dependency order with pinned versions' {
    $global:VcpkgTestAvailable.Clear()
    & $scriptPath
    $LASTEXITCODE | Should -Be 0
    $global:VcpkgTestInstalled | Should -Be @('Az.Accounts', 'Az.Storage')
    Should -Invoke Install-Module -Times 1 -Exactly -ParameterFilter {
      $Name -eq 'Az.Accounts' -and $RequiredVersion -eq '5.5.3' -and
      $Repository -eq 'PSGallery' -and $Scope -eq 'CurrentUser' -and $Force -and $ErrorAction -eq 'Stop'
    }
    Should -Invoke Install-Module -Times 1 -Exactly -ParameterFilter {
      $Name -eq 'Az.Storage' -and $RequiredVersion -eq '9.7.2' -and
      $Repository -eq 'PSGallery' -and $Scope -eq 'CurrentUser' -and $Force -and $ErrorAction -eq 'Stop'
    }
    & $scriptPath
    $LASTEXITCODE | Should -Be 0
    Should -Invoke Install-Module -Times 2 -Exactly
  }

  It 'installs only the missing <Module>' -ForEach @(
    @{ Module = 'Az.Accounts' }
    @{ Module = 'Az.Storage' }
  ) {
    $global:VcpkgTestAvailable.Remove($Module)
    & $scriptPath
    $LASTEXITCODE | Should -Be 0
    $global:VcpkgTestInstalled | Should -Be @($Module)
  }

  It 'rediscovers Accounts if Storage installation upgrades its dependency' {
    $global:VcpkgTestAvailable.Remove('Az.Storage')
    Mock Install-Module {
      $global:VcpkgTestImported.Count | Should -Be 0
      $global:VcpkgTestAvailable['Az.Accounts'] = @([pscustomobject]@{
        Name = 'Az.Accounts'; Version = [version]'5.6.0'; Path = '/modules/Az.Accounts/5.6.0/Az.Accounts.psd1'
      })
      $global:VcpkgTestAvailable['Az.Storage'] = @([pscustomobject]@{
        Name = 'Az.Storage'; Version = [version]'9.7.2'; Path = '/modules/Az.Storage/9.7.2/Az.Storage.psd1'
      })
    }
    & $scriptPath
    $LASTEXITCODE | Should -Be 0
    Should -Invoke Import-Module -Times 1 -Exactly -ParameterFilter { $Name -eq '/modules/Az.Accounts/5.6.0/Az.Accounts.psd1' }
  }

  It 'upgrades modules below the compatible baseline' {
    $global:VcpkgTestAvailable['Az.Accounts'][0].Version = [version]'3.0.3'
    $global:VcpkgTestAvailable['Az.Storage'][0].Version = [version]'7.2.0'
    & $scriptPath
    $LASTEXITCODE | Should -Be 0
    Should -Invoke Install-Module -Times 2 -Exactly
  }

  It 'registers the default repository when PSGallery is absent' {
    $global:VcpkgTestAvailable.Clear()
    $global:VcpkgTestRepository = $null
    & $scriptPath
    $LASTEXITCODE | Should -Be 0 -Because ($global:VcpkgTestOutput -join '; ')
    Should -Invoke Register-PSRepository -Times 1 -Exactly -ParameterFilter { $Default -and $ErrorAction -eq 'Stop' }
  }

  It 'fails if repository registration does not make PSGallery available' {
    $global:VcpkgTestAvailable.Clear()
    $global:VcpkgTestRepository = $null
    Mock Register-PSRepository {}
    & $scriptPath
    $LASTEXITCODE | Should -Be 1
    Should -Invoke Install-Module -Times 0 -Exactly
    Should -Invoke Write-Host -Times 1 -Exactly -ParameterFilter { $Object -like 'Failed while preparing PSGallery:*' }
  }

  It 'fails if repository registration throws' {
    $global:VcpkgTestAvailable.Clear()
    $global:VcpkgTestRepository = $null
    Mock Register-PSRepository { throw 'Repository unavailable' }
    & $scriptPath
    $LASTEXITCODE | Should -Be 1
    Should -Invoke Register-PSRepository -Times 1 -Exactly
    Should -Invoke Install-Module -Times 0 -Exactly
  }

  It 'rejects a PSGallery name registered to a different source' {
    $global:VcpkgTestAvailable.Clear()
    $global:VcpkgTestRepository.SourceLocation = 'https://example.invalid/api/v2'
    & $scriptPath
    $LASTEXITCODE | Should -Be 1
    Should -Invoke Install-Module -Times 0 -Exactly
  }

  It 'fails immediately if installation fails' {
    $global:VcpkgTestAvailable.Clear()
    Mock Install-Module { throw 'Download failed' }
    & $scriptPath
    $LASTEXITCODE | Should -Be 1
    Should -Invoke Install-Module -Times 1 -Exactly
    Should -Invoke Import-Module -Times 0 -Exactly
    Should -Invoke Write-Host -Times 1 -Exactly -ParameterFilter {
      $Object -like 'Failed while installing Az.Accounts 5.5.3 from PSGallery:*'
    }
  }

  It 'rejects a successful installer that leaves a module undiscoverable' {
    $global:VcpkgTestAvailable.Clear()
    Mock Install-Module {}
    & $scriptPath
    $LASTEXITCODE | Should -Be 1
    Should -Invoke Import-Module -Times 0 -Exactly
  }

  It 'fails on an import error instead of accepting presence alone' {
    Mock Import-Module { throw 'Dependency incompatible' }
    & $scriptPath
    $LASTEXITCODE | Should -Be 1
    Should -Invoke Write-Host -Times 1 -Exactly -ParameterFilter { $Object -like 'Failed while importing Az.Accounts*' }
  }

  It 'fails when a required cmdlet is missing' {
    Mock Get-Command { throw 'Cmdlet unavailable' } -ParameterFilter { $Name -eq 'Az.Storage\New-AzStorageContainerSASToken' }
    & $scriptPath
    $LASTEXITCODE | Should -Be 1
  }

  It 'fails when a required cmdlet parameter is missing' {
    Mock Get-Command { [pscustomobject]@{ Parameters = @{} } } -ParameterFilter { $Name -eq 'Az.Storage\New-AzStorageContext' }
    & $scriptPath
    $LASTEXITCODE | Should -Be 1
    Should -Invoke Write-Host -Times 1 -Exactly -ParameterFilter { $Object -like '*missing required parameter*' }
  }

  It 'logs only module status without environment values or pipeline variables' {
    & $scriptPath
    $LASTEXITCODE | Should -Be 0
    Should -Invoke Write-Host -Times 3 -Exactly
    Should -Invoke Write-Host -Times 0 -Exactly -ParameterFilter {
      $Object -notmatch '^(Verified import: Az\.(Accounts|Storage) [\d.]+\.|Az modules are ready for the vcpkg write-mode cache task\.)$'
    }
    $source = Get-Content $scriptPath -Raw
    $source | Should -Not -Match '\$env:|##vso|Connect-AzAccount\s+-|New-AzStorageContainerSASToken\s+-'
  }
}

Describe 'Vcpkg pipeline template contract' {
  BeforeAll {
    Import-Module powershell-yaml -MinimumVersion 0.4.7 -ErrorAction Stop
    $engRoot = [System.IO.Path]::Combine($PSScriptRoot, '..', '..')
    $wrapper = Get-Content ([System.IO.Path]::Combine($engRoot, 'pipelines', 'templates', 'steps', 'vcpkg.yml')) -Raw |
      ConvertFrom-Yaml
    $shared = Get-Content ([System.IO.Path]::Combine($engRoot, 'common', 'pipelines', 'templates', 'steps', 'set-vcpkg-cache-vars.yml')) -Raw |
      ConvertFrom-Yaml
    $internalCondition = '${{ if eq(variables[''System.TeamProject''], ''internal'') }}'
  }

  It 'places a blocking internal-only pwsh bootstrap before the unchanged shared template' {
    @($wrapper.steps[0].Keys) | Should -Be @($internalCondition)
    $step = $wrapper.steps[0][$internalCondition][0]
    $step.task | Should -Be 'PowerShell@2'
    $step.inputs.pwsh | Should -BeTrue
    $step.inputs.filePath | Should -Be '$(Build.SourcesDirectory)/eng/scripts/Install-VcpkgCacheModules.ps1'
    $step.ContainsKey('condition') | Should -BeFalse
    $step.ContainsKey('continueOnError') | Should -BeFalse
    $wrapper.steps[1].template | Should -Be '/eng/common/pipelines/templates/steps/set-vcpkg-cache-vars.yml'
  }

  It 'leaves public steps and Windows-only vcpkg operations unchanged' {
    $wrapper.steps.Count | Should -Be 4
    $wrapper.steps[2].script | Should -Be 'vcpkg --version'
    $wrapper.steps[3].displayName | Should -Be 'vcpkg install'
    foreach ($step in $wrapper.steps[2..3]) {
      $step.condition | Should -Match 'succeeded\(\)'
      $step.condition | Should -Match "eq\(variables\['Agent.OS'\], 'Windows_NT'\)"
    }
    $wrapper.steps[3].env.VCPKG_BINARY_SOURCES | Should -Be '$(VCPKG_BINARY_SOURCES_SECRET)'
  }

  It 'preserves service connection, SAS permissions and secret redaction' {
    $condition = @($shared.steps[1].Keys)[0]
    $condition.Replace(' ', '') | Should -Be $internalCondition.Replace(' ', '')
    $task = $shared.steps[1][$condition][0]
    $task.task | Should -Be 'AzurePowerShell@5'
    $task.inputs.azureSubscription | Should -Be 'Azure SDK Artifacts'
    $task.inputs.azurePowerShellVersion | Should -Be 'LatestVersion'
    $task.inputs.pwsh | Should -BeTrue
    $task.inputs.ScriptPath | Should -Be 'eng/common/scripts/Set-VcpkgWriteModeCache.ps1'
    $source = Get-Content ([System.IO.Path]::Combine($engRoot, 'common', 'scripts', 'Set-VcpkgWriteModeCache.ps1')) -Raw
    $source | Should -Match '-UseConnectedAccount'
    $source | Should -Match '-Permission "rwcl"'
    $source | Should -Match 'variable=VCPKG_BINARY_SAS_TOKEN;issecret=true;'
    $source | Should -Match 'variable=VCPKG_BINARY_SOURCES_SECRET;issecret=true;'
  }
}
