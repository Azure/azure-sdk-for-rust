#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2021"

[dependencies]
serde = { version = "1.0.197", features = ["derive"] }
toml = "0.8.10"
---

use serde::Deserialize;
use std::{env, fs, path::Path};

fn main() {
    let script_root = env!("CARGO_MANIFEST_DIR");
    let script_root = Path::new(&script_root);
    let workspace_root = script_root
        .join("../..")
        .canonicalize()
        .expect("ancestor expected");

    let toolchain_content = fs::read_to_string(workspace_root.join("rust-toolchain.toml")).expect("read rust-toolchain.toml");
    let toolchain_toml = toml::from_str::<ToolchainManifest>(&toolchain_content).expect("deserialize rust-toolchain.toml");

    let core_content = fs::read_to_string(workspace_root.join("sdk/core/Cargo.toml")).expect("read sdk/core/Cargo.toml");
    let core_toml = toml::from_str::<CargoManifest>(&core_content).expect("deserialize sdk/core/Cargo.toml");

    if toolchain_toml.toolchain.channel != core_toml.package.rust_version {
        eprintln!("error: toolchain channel {} != azure_core's rust_version {}", toolchain_toml.toolchain.channel, core_toml.package.rust_version);
        std::process::exit(1);
    }
}

#[derive(Deserialize)]
struct CargoManifest {
    pub package: CargoPackage,
}

#[derive(Deserialize)]
struct CargoPackage {
    #[serde(rename = "rust-version")]
    pub rust_version: String,
}

#[derive(Deserialize)]
struct ToolchainManifest {
    pub toolchain: Toolchain,
}

#[derive(Deserialize)]
struct Toolchain {
    pub channel: String,
}
