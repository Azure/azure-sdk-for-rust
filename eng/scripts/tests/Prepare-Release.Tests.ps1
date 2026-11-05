# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

# cspell:ignore gpgsign
BeforeAll {
  $helper = [System.IO.Path]::Combine($PSScriptRoot, '..', 'Prepare-Release.Helpers.ps1')
  . $helper -FromSkill

  function New-ReleaseFixture([string] $Name = 'azure_example', [string] $Service = 'example') {
    [PSCustomObject]@{
      Name = $Name; Version = '1.1.0-beta.1'; Service = $Service
      Directory = "sdk/$Service/$Name"; Publishable = $true; Dependencies = @()
    }
  }
}

Describe 'Skill helper entry point' {
  It 'rejects accidental invocation before doing work' {
    { & $helper } | Should -Throw '*requires -FromSkill*'
  }

  It 'has no executable shebang' {
    (Get-Content $helper -First 1) | Should -Not -Match '^#!'
  }
}

Describe 'Package selection' {
  It 'finds names across services and deduplicates service selections' {
    $inventory = @((New-ReleaseFixture), (New-ReleaseFixture 'azure_other' 'other'))
    @(Select-ReleasePackages $inventory @() @('azure_other'))[0].Service | Should -Be 'other'
    @(Select-ReleasePackages $inventory @('example', 'example') @()).Count | Should -Be 1
  }

  It 'rejects missing and private explicit packages' {
    $private = New-ReleaseFixture
    $private.Publishable = $false
    { Select-ReleasePackages @($private) @() @('azure_example') } | Should -Throw '*publishable*'
    { Select-ReleasePackages @() @() @('missing') } | Should -Throw '*publishable*'
  }

  It 'filters private packages from a service and checks directory names' {
    $private = New-ReleaseFixture 'private'
    $private.Publishable = $false
    @(Select-ReleasePackages @((New-ReleaseFixture), $private) @('example') @()).Count | Should -Be 1
    $mismatch = New-ReleaseFixture
    $mismatch.Directory = 'sdk/example/other'
    { Select-ReleasePackages @($mismatch) @('example') @() } | Should -Throw '*does not match*'
  }
}

Describe 'Session tag fetch' {
  BeforeEach {
    $RepoRoot = $TestDrive
    Mock Invoke-ReleaseCommand {}
  }

  It 'fetches from canonical upstream only once across calls' {
    $session = [System.IO.Path]::Combine($TestDrive, 'session')
    Initialize-ReleaseTags $session
    Initialize-ReleaseTags $session
    Should -Invoke Invoke-ReleaseCommand -Times 1 -Exactly -ParameterFilter {
      $Arguments -contains 'https://github.com/Azure/azure-sdk-for-rust.git'
    }
  }

  It 'retries after a failed fetch' {
    $session = [System.IO.Path]::Combine($TestDrive, 'retry')
    Mock Invoke-ReleaseCommand { throw 'fetch failed' }
    { Initialize-ReleaseTags $session } | Should -Throw '*fetch failed*'
    Test-Path $session | Should -BeFalse
    Mock Invoke-ReleaseCommand {}
    Initialize-ReleaseTags $session
    @(Get-ChildItem $session).Count | Should -Be 1
  }
}

Describe 'Semantic release baselines' {
  BeforeEach {
    Mock Invoke-ReleaseCommand {
      @('azure_example@1.9.0', 'azure_example@1.10.0', 'azure_example@2.0.0-beta.2',
        'azure_example@2.0.0-beta.10', 'azure_example@invalid')
    }
  }

  It 'uses the latest stable across major versions' {
    (Get-ReleaseBaseline 'azure_example' 'Stable').Version | Should -Be '1.10.0'
  }

  It 'orders prerelease numbers numerically' {
    (Get-ReleaseBaseline 'azure_example' 'Beta').Version | Should -Be '2.0.0-beta.10'
  }

  It 'treats unlabeled zero-major tags as stable semver' {
    Mock Invoke-ReleaseCommand { @('azure_example@0.9.0', 'azure_example@0.10.0-beta.1') }
    (Get-ReleaseBaseline 'azure_example' 'Stable').Version | Should -Be '0.9.0'
  }

  It 'returns no baseline for a first release' {
    Mock Invoke-ReleaseCommand { @() }
    Get-ReleaseBaseline 'azure_example' 'Beta' | Should -BeNullOrEmpty
  }

  It 'ignores malformed semantic versions' {
    Mock Invoke-ReleaseCommand {
      @('azure_example@01.2.3', 'azure_example@1.2.3-beta.01', 'azure_example@1.2.3')
    }
    (Get-ReleaseBaseline 'azure_example' 'Beta').Version | Should -Be '1.2.3'
  }
}

Describe 'Analysis and build scope' {
  BeforeEach {
    Mock Get-ReleaseBaseline { $null }
    Mock Invoke-ReleaseCommand { @() }
    Mock Invoke-ReleaseToml { @{} }
    Mock Get-ReleaseSnapshot { [ordered]@{} }
  }

  It 'includes a first release and requires a workspace build for core or identity' {
    foreach ($service in @('core', 'identity')) {
      $package = New-ReleaseFixture 'azure_example' $service
      $analysis = Get-ReleaseAnalysis @($package) @($package) 'Beta' '2026-11-05'
      $analysis.WorkspaceBuild | Should -BeTrue
      $analysis.Packages[0].Release | Should -BeTrue
      $analysis.Packages[0].Changes | Should -Contain 'First release for this channel'
    }
  }

  It 'uses targeted builds for other services' {
    $package = New-ReleaseFixture
    (Get-ReleaseAnalysis @($package) @($package) 'Beta' '2026-11-05').WorkspaceBuild | Should -BeFalse
  }

  It 'compares dependencies to their last stable release even when a newer preview exists' {
    Mock Invoke-ReleaseToml {
      param($Mode, $InputObject)
      $parsed = @{}
      foreach ($id in $InputObject.Keys) { $parsed[$id] = @{} }
      $parsed
    }
    Mock Get-ReleaseBaseline {
      param($Name, $Channel)
      $version = if ($Channel -eq 'Stable') { '1.0.0' } else { '1.1.0-beta.1' }
      @{ Tag = "$Name@$version"; Version = $version }
    }
    Mock Invoke-ReleaseCommand {
      param($Arguments)
      if ($Arguments -contains 'diff' -and $Arguments -contains 'azure_dependency@1.0.0..HEAD') {
        'sdk/example/azure_dependency/src/lib.rs'
      }
    }
    $packages = @((New-ReleaseFixture), (New-ReleaseFixture 'azure_dependency'))
    $analysis = Get-ReleaseAnalysis $packages $packages 'Stable' '2026-11-05'
    $dependency = $analysis.Packages | Where-Object Name -EQ 'azure_dependency'
    $dependency.BaselineVersion | Should -Be '1.0.0'
    $dependency.Release | Should -BeTrue
    Should -Invoke Get-ReleaseBaseline -Times 2 -Exactly -ParameterFilter { $Channel -eq 'Stable' }
  }
}

Describe 'Substantive change normalization' {
  It 'ignores empty next-version scaffolding and release dates' {
    $before = "# Release History`n`n## 1.0.0 (2026-01-01)`n`n### Features Added`n`n- Existing feature."
    $after = "# Release History`n`n## 1.1.0-beta.1 (Unreleased)`n`n### Features Added`n`n### Bugs Fixed`n`n## 1.0.0 (2026-02-01)`n`n### Features Added`n`n- Existing feature."
    (ConvertTo-ReleaseChangelog $before) | Should -BeExactly (ConvertTo-ReleaseChangelog $after)
  }

  It 'retains substantive changelog content' {
    $before = "# Release History`n`n## 1.0.0 (2026-01-01)`n`n### Features Added`n`n- Existing feature."
    $after = $before.Replace('Existing feature.', 'Fixed behavior.')
    (ConvertTo-ReleaseChangelog $before) | Should -Not -BeExactly (ConvertTo-ReleaseChangelog $after)
  }

  It 'detects effective inherited dependency changes including target tables' {
    $document = @{ target = @{ 'cfg(unix)' = @{ dependencies = @{ core = @{ workspace = $true } } } } }
    $before = @{ workspace = @{ dependencies = @{ core = @{ version = '1.0.0' } } } }
    $after = @{ workspace = @{ dependencies = @{ core = @{ version = '1.1.0' } } } }
    (Get-ReleaseDependencySignature $document $before) | Should -Not -BeExactly (Get-ReleaseDependencySignature $document $after)
  }
}

Describe 'Release decisions' {
  BeforeEach {
    Mock Get-ReleaseBaseline { $null }
    $fixture = New-ReleaseFixture
    $plan = @{
      ReleaseType = 'Beta'
      Packages = @(@{
          Name = $fixture.Name; Version = $fixture.Version; Release = $true
          BaselineVersion = $null; BaselineTag = $null; Changes = @(); DecisionReason = ''
        })
    }
  }

  It 'retains the existing next-release version' {
    $result = Get-ReleaseDecisions $plan @($fixture)
    $result.ReleaseOrder | Should -Be 'azure_example'
    $fixture.Version | Should -Be '1.1.0-beta.1'
  }

  It 'blocks wrong-channel and already-released versions without increasing them' {
    $plan.ReleaseType = 'Stable'
    { Get-ReleaseDecisions $plan @($fixture) } | Should -Throw '*Do not bump*'
    $plan.ReleaseType = 'Beta'
    Mock Get-ReleaseBaseline { @{ Parsed = [semver]'1.1.0-beta.1' } }
    { Get-ReleaseDecisions $plan @($fixture) } | Should -Throw '*Do not bump*'
  }

  It 'requires agreement to expand dependency scope' {
    $fixture.Dependencies = @(@{ name = 'azure_dependency'; path = '/local'; kind = $null; req = '^1.0.0' })
    { Get-ReleaseDecisions $plan @($fixture) } | Should -Throw '*ask before expanding*'
  }

  It 'rejects preview normal and build dependencies for stable releases' {
    $plan.ReleaseType = 'Stable'
    $fixture.Version = $plan.Packages[0].Version = '1.1.0'
    foreach ($kind in @($null, 'build')) {
      $fixture.Dependencies = @(@{ name = 'azure_dependency'; kind = $kind; req = '^1.1.0-beta.1' })
      { Get-ReleaseDecisions $plan @($fixture) } | Should -Throw '*cannot depend on preview*last stable*'
    }
    $fixture.Dependencies[0].kind = 'dev'
    (Get-ReleaseDecisions $plan @($fixture)).ReleaseOrder | Should -Be 'azure_example'
  }

  It 'allows a preview requirement only when it will be replaced with an unchanged stable dependency' {
    $plan.ReleaseType = 'Stable'
    $fixture.Version = $plan.Packages[0].Version = '1.1.0'
    $dependency = New-ReleaseFixture 'azure_dependency'
    $fixture.Dependencies = @(@{ name = $dependency.Name; path = '/local'; kind = $null; req = '^1.1.0-beta.1' })
    $plan.Packages += @{
      Name = $dependency.Name; Version = $dependency.Version; Release = $false
      BaselineVersion = '1.0.0'; BaselineTag = 'azure_dependency@1.0.0'; Changes = @()
    }
    Mock Get-ReleaseBaseline {
      param($Name, $Channel)
      if ($Name -eq 'azure_dependency' -and $Channel -eq 'Stable') {
        @{ Tag = 'azure_dependency@1.0.0'; Version = '1.0.0' }
      }
    }
    (Get-ReleaseDecisions $plan @($fixture, $dependency)).Versions.azure_dependency | Should -Be '1.0.0'
  }

  It 'orders dependencies before dependents' {
    $dependency = New-ReleaseFixture 'azure_dependency'
    $fixture.Dependencies = @(@{ name = $dependency.Name; path = '/local'; kind = $null; req = '^1.1.0-beta.1' })
    $plan.Packages += @{ Name = $dependency.Name; Version = $dependency.Version; Release = $true }
    (Get-ReleaseDecisions $plan @($fixture, $dependency)).ReleaseOrder |
      Should -Be @('azure_dependency', 'azure_example')
  }

  It 'does not skip a first release or an unexplained changed crate' {
    $plan.Packages[0].Release = $false
    { Get-ReleaseDecisions $plan @($fixture) } | Should -Throw '*first release*'
    $plan.Packages[0].BaselineVersion = '1.0.0'
    $plan.Packages[0].Changes = @('src/lib.rs')
    { Get-ReleaseDecisions $plan @($fixture) } | Should -Throw '*DecisionReason*'
  }
}

Describe 'Final release checks' {
  It 'fails on invalid changelogs even when preparation only warned' {
    Mock Confirm-ChangeLogEntry { $false }
    $fixture = New-ReleaseFixture
    $plan = @{ Packages = @(@{ Name = $fixture.Name; Version = $fixture.Version; Release = $true; Directory = $fixture.Directory }) }
    { Confirm-PreparedRelease $plan @($fixture) } | Should -Throw '*Changelog is not ready*'
  }

  It 'checks the requested date and unchanged version' {
    Mock Confirm-ChangeLogEntry { $true }
    Mock Get-ChangeLogEntry { @{ ReleaseStatus = '(2026-11-06)' } }
    $fixture = New-ReleaseFixture
    $plan = @{ ReleaseDate = '2026-11-05'; Packages = @(@{ Name = $fixture.Name; Version = $fixture.Version; Release = $true; Directory = $fixture.Directory }) }
    { Confirm-PreparedRelease $plan @($fixture) } | Should -Throw '*Unexpected release date*'
    $plan.Packages[0].Version = '2.0.0'
    { Confirm-PreparedRelease $plan @($fixture) } | Should -Throw '*changed package version*'
  }
}

Describe 'Release helper integration' {
  It 'analyzes bookkeeping-only commits and applies only released dependency edits' {
    $RepoRoot = [System.IO.Path]::Combine($TestDrive, 'repository')
    [void] (New-Item -ItemType Directory -Path $RepoRoot)
    foreach ($name in @('azure_example', 'azure_consumer')) {
      $directory = [System.IO.Path]::Combine($RepoRoot, 'sdk', 'example', $name)
      [void] (New-Item -ItemType Directory -Path ([System.IO.Path]::Combine($directory, 'src')) -Force)
      Set-Content ([System.IO.Path]::Combine($directory, 'src', 'lib.rs')) -Value ''
      Set-Content ([System.IO.Path]::Combine($directory, 'Cargo.toml')) -Value @"
[package]
name = "$name"
version = "1.0.0"
edition = "2021"
"@
      Set-Content ([System.IO.Path]::Combine($directory, 'CHANGELOG.md')) -Value @"
# Release History

## 1.0.0 (2026-01-01)

### Features Added

- Initial release.
"@
    }
    Set-Content ([System.IO.Path]::Combine($RepoRoot, 'Cargo.toml')) -Value @'
[workspace]
members = ["sdk/example/*"]
resolver = "2"
'@
    $consumerPath = [System.IO.Path]::Combine($RepoRoot, 'sdk', 'example', 'azure_consumer', 'Cargo.toml')
    Add-Content $consumerPath -Value @'

[dependencies]
azure_example = { path = "../azure_example", version = "1.0.0", default-features = false }

[dev-dependencies]
azure_example = { path = "../azure_example", version = "1.0.0", features = [] }
'@
    Invoke-ReleaseCommand @('git', 'init', '--quiet') | Out-Null
    Invoke-ReleaseCommand @('git', 'add', '.') | Out-Null
    Invoke-ReleaseCommand @('git', '-c', 'commit.gpgsign=false',
      '-c', "core.hooksPath=$([System.IO.Path]::Combine($RepoRoot, '.git', 'no-hooks'))",
      '-c', 'user.name=Release Test', '-c', 'user.email=release@example.invalid',
      'commit', '--quiet', '-m', 'Initial fixture') | Out-Null
    Invoke-ReleaseCommand @('git', '-c', 'tag.gpgsign=false', 'tag', 'azure_example@1.0.0') | Out-Null
    $manifestPath = [System.IO.Path]::Combine($RepoRoot, 'sdk', 'example', 'azure_example', 'Cargo.toml')
    (Get-Content $manifestPath -Raw).Replace('1.0.0', '1.1.0') | Set-Content $manifestPath
    (Get-Content $consumerPath -Raw).Replace('1.0.0', '1.1.0') | Set-Content $consumerPath
    Invoke-ReleaseCommand @('git', 'add', '.') | Out-Null
    Invoke-ReleaseCommand @('git', '-c', 'commit.gpgsign=false',
      '-c', "core.hooksPath=$([System.IO.Path]::Combine($RepoRoot, '.git', 'no-hooks'))",
      '-c', 'user.name=Release Test', '-c', 'user.email=release@example.invalid',
      'commit', '--quiet', '-m', 'Post-release versions') | Out-Null

    $inventory = @(Get-ReleaseInventory)
    $selected = Select-ReleasePackages $inventory @() @('azure_example')
    $plan = Get-ReleaseAnalysis $inventory $selected 'Stable' '2026-11-05'
    $plan.Packages[0].Release | Should -BeFalse
    $plan.Packages[0].Changes.Count | Should -Be 0
    $plan.Packages[0].LocalChanges.Count | Should -Be 0
    $decisions = Get-ReleaseDecisions $plan $inventory
    $inputObject = @{ root = [string] $RepoRoot; versions = $decisions.Versions; apply = $false }
    $preview = Invoke-ReleaseToml 'release-dependencies' $inputObject
    $preview.affected_packages | Should -Contain 'azure_consumer'
    (Get-Content $consumerPath -Raw) | Should -Match 'path ='
    $inputObject.apply = $true
    $applied = Invoke-ReleaseToml 'release-dependencies' $inputObject
    $applied.changed_manifests | Should -Contain 'sdk/example/azure_consumer/Cargo.toml'
    (Get-Content $consumerPath -Raw) | Should -Not -Match 'path ='
    (Get-Content $consumerPath -Raw) | Should -Match 'default-features = false'
    (Get-Content $manifestPath -Raw) | Should -Match 'version = "1.1.0"'
    (Invoke-ReleaseToml 'release-dependencies' $inputObject).changed_manifests.Count | Should -Be 0
  }
}
