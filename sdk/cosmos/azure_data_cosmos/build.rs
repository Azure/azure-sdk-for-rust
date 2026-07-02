// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
// Registers custom cfgs used by this crate's integration tests.
//
// Some CI/build setups enable `-W unexpected-cfgs`, and in newer Rust toolchains
// unknown cfg names are warned/denied unless explicitly declared via check-cfg.
fn main() {
    // Allow `#[cfg_attr(not(test_category = "..."), ignore)]` in `tests/*.rs`.
    println!(
        "cargo:rustc-check-cfg=cfg(test_category, values(\"emulator\", \"emulator_vnext\", \"multi_write\", \"split\", \"gateway_v2\", \"gateway_v2_multi_region\"))"
    );
}
