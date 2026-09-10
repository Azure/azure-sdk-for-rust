// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore enableaadauthentication

// Registers custom cfgs used by this crate's integration tests.
//
// Some CI/build setups enable `-W unexpected-cfgs`, and in newer Rust toolchains
// unknown cfg names are warned/denied unless explicitly declared via check-cfg.
fn main() {
    // Allow `#[cfg_attr(not(test_category = "..."), ignore)]` in `tests/*.rs`.
    println!(
        "cargo:rustc-check-cfg=cfg(test_category, values(\"emulator\", \"emulator_vnext\", \"emulator_inmemory\", \"emulator_inmemory_gateway_v2\", \"multi_write\", \"split\", \"binary_encoding\", \"gateway_v2\", \"gateway_v2_multi_region\"))"
    );
    // Marker cfg set by test setups where the target Cosmos account is provisioned
    // for AAD data-plane access (local emulator started with /enableaadauthentication,
    // or a live account whose bicep deployment created the data-plane role assignment).
    // Fixed self-owned live accounts do not currently have that role assignment and
    // therefore do not set this cfg, which causes AAD data-plane tests to be ignored
    // on those legs.
    println!("cargo:rustc-check-cfg=cfg(cosmos_aad_supported)");
}
