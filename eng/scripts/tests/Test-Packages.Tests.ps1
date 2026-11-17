# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License.

BeforeAll {
  . ([System.IO.Path]::Combine($PSScriptRoot, '..', 'shared', 'Cargo.ps1'))
  $packages = @(
    [pscustomobject]@{ Name = 'azure_data_cosmos_driver' }
    [pscustomobject]@{ Name = 'azure_data_cosmos' }
    [pscustomobject]@{ Name = 'azure_data_cosmos_macros' }
  )
}

Describe 'Targeted integration package selection' {
  It 'preserves the full baseline when no target is requested' {
    @(Select-CargoTestPackages $packages '' '').Name | Should -Be $packages.Name
  }

  It 'runs only the explicitly selected package rather than repeating other package suites' {
    $selected = @(Select-CargoTestPackages $packages 'azure_data_cosmos' 'e2e_tests')
    $selected.Count | Should -Be 1
    $selected[0].Name | Should -Be 'azure_data_cosmos'
  }

  It 'fails on incomplete selection rather than silently dropping coverage' {
    { Select-CargoTestPackages $packages '' 'e2e_tests' } | Should -Throw '*both*'
    { Select-CargoTestPackages $packages 'azure_data_cosmos' '' } | Should -Throw '*both*'
  }

  It 'rejects missing and ambiguous selected packages' {
    { Select-CargoTestPackages $packages 'missing' 'e2e_tests' } | Should -Throw '*exactly once*'
    { Select-CargoTestPackages @($packages + $packages[1]) 'azure_data_cosmos' 'e2e_tests' } |
      Should -Throw '*exactly once*'
  }

  It 'does not permit test-selection arguments to inject additional command options' {
    { Select-CargoTestPackages $packages 'azure_data_cosmos' "e2e_tests' --ignored" } |
      Should -Throw '*only letters*'
  }
}

Describe 'Targeted integration runner' {
  BeforeAll {
    $runner = ([System.IO.Path]::Combine($PSScriptRoot, '..', 'Test-Packages.ps1'))
    $fixture = ([System.IO.Path]::Combine($TestDrive, 'crate'))
    $info = ([System.IO.Path]::Combine($TestDrive, 'info'))
    New-Item -ItemType Directory -Path $fixture, $info,
      ([System.IO.Path]::Combine($fixture, 'tests')) | Out-Null
    @'
[package]
name = "targeted-runner-fixture"
version = "0.0.0"
edition = "2021"

[lib]
path = "lib.rs"

[workspace]
'@ | Set-Content ([System.IO.Path]::Combine($fixture, 'Cargo.toml'))
    @'
// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
/// ```
/// panic!("unselected documentation test ran");
/// ```
pub fn documentation() {}
#[cfg(test)]
mod tests {
    #[test]
    fn broad_suite_must_not_run_in_targeted_mode() {
        panic!("unselected unit test ran");
    }
}
'@ | Set-Content ([System.IO.Path]::Combine($fixture, 'lib.rs'))
    @'
// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#[test]
fn selected_integration() {
    std::fs::write(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("selected.marker"),
        "selected integration executed",
    ).unwrap();
}
'@ | Set-Content ([System.IO.Path]::Combine($fixture, 'tests', 'selected.rs'))
    @{ Name = 'targeted-runner-fixture'; DirectoryPath = $fixture } |
      ConvertTo-Json | Set-Content ([System.IO.Path]::Combine($info, 'fixture.json'))
  }

  It 'executes the chosen integration target without running unrelated unit or doc suites' {
    $previousPackage = $env:AZURE_SDK_TEST_PACKAGE
    $previousTarget = $env:AZURE_SDK_TEST_TARGET
    try {
      $env:AZURE_SDK_TEST_PACKAGE = 'targeted-runner-fixture'
      $env:AZURE_SDK_TEST_TARGET = 'selected'
      $output = & pwsh -NoProfile -File $runner -PackageInfoDirectory $info 2>&1
    }
    finally {
      $env:AZURE_SDK_TEST_PACKAGE = $previousPackage
      $env:AZURE_SDK_TEST_TARGET = $previousTarget
    }
    $LASTEXITCODE | Should -Be 0 -Because ($output -join "`n")
    Get-Content ([System.IO.Path]::Combine($fixture, 'selected.marker')) |
      Should -Be 'selected integration executed'
  }

  It 'fails when Cargo cannot find the requested target rather than reporting a skipped pass' {
    $output = & pwsh -NoProfile -File $runner -PackageInfoDirectory $info `
      -PackageName 'targeted-runner-fixture' -TestTarget 'missing' 2>&1
    $LASTEXITCODE | Should -Not -Be 0
    ($output -join "`n") | Should -Match "no test target named .missing"
  }
}
