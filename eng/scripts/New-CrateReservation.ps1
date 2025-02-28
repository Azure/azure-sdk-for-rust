#!/usr/bin/env pwsh

#Requires -Version 7.0

[CmdletBinding()]
param(
  [Parameter(Mandatory = $true, ValueFromPipelineByPropertyName = $true)]
  [string] $ServiceDirectory,

  [Parameter(Mandatory = $true, ValueFromPipelineByPropertyName = $true)]
  [string] $Name,

  [Parameter(ValueFromPipelineByPropertyName = $true)]
  [string] $Description,

  [Parameter()]
  [switch] $Force
)

Begin {
  $ErrorActionPreference = 'Stop'
}

Process {
  Write-Verbose "Emitting $ServiceDirectory/$Name"
  if (!$Description) {
    $Description = "Microsoft Azure SDK for Rust: $Name"
  }

  $path = "$PSScriptRoot/../../sdk/$ServiceDirectory/$Name"
  if (!$Force -and (Test-Path "$path/Cargo.toml")) {
    Write-Verbose "$path already exists; skipping"
    return
  }
  New-Item -Path "$path/src" -ItemType Directory -Force -ErrorAction Ignore | Out-Null

  @"
// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

fn main() {
    println!("Coming soon: $Description");
}
"@ | Set-Content -Path "$path/src/main.rs"

  @"
[package]
name = "$Name"
version = "0.0.1"
description = "$Description"
readme = "README.md"
authors.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
homepage = "https://github.com/azure/azure-sdk-for-rust"
documentation = "https://docs.rs/$Name"
categories = ["api-bindings"]

[lints]
workspace = true
"@ | Set-Content -Path "$path/Cargo.toml"

  @"
# Microsoft Azure Key Vault management client library for Rust

Coming soon.

If this package is important to you, we'd love to hear from you! Please let us know by [leaving an issue in our repo](https://github.com/Azure/azure-sdk-for-rust/issues/new/choose). Your input helps us improve and prioritize features that matter most to you.
"@ | Set-Content -Path "$path/README.md"
}
