// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use serde::Deserialize;
use std::{fs, path::Path};

fn main() {
    let toolchain_file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("rust-toolchain.toml");
    let toolchain_content =
        fs::read_to_string(&toolchain_file).expect("read rust-toolchain.toml from workspace root");
    let manifest: Manifest =
        toml::from_str(&toolchain_content).expect("deserialize rust-toolchain.toml");
    println!(
        "cargo::rustc-env=TOOLCHAIN_CHANNEL={}",
        manifest.toolchain.channel
    );
    println!("cargo::rerun-if-changed=../rust-toolchain.toml");
}

#[derive(Debug, Deserialize)]
struct Manifest {
    toolchain: Toolchain,
}

#[derive(Debug, Deserialize)]
struct Toolchain {
    channel: String,
}
