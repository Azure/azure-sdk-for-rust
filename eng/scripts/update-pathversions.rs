#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2021"

[dependencies]
toml = "0.8.10"
---

use std::{env, fs, path::PathBuf};
use std::io::Write;
use toml::Table;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let script_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let repo_root = script_dir.join("../../").canonicalize()?;

    // read the Cargo.toml file
    let cargo_toml_path = repo_root.join("Cargo.toml");
    let cargo_toml_content = fs::read_to_string(&cargo_toml_path).expect("Failed to read Cargo.toml");
    let cargo_toml: Table = cargo_toml_content.parse().expect("Failed to parse Cargo.toml");

    // reserialize the Cargo.toml file
    let mut cargo_toml_file = fs::File::create(&cargo_toml_path)?;
    let mut cargo_toml_content = toml::to_string_pretty(&cargo_toml)?;
    cargo_toml_content.push_str("\n");
    cargo_toml_file
        .write_all(cargo_toml_content.as_bytes())
        .expect("Failed to write Cargo.toml");

    return Ok(());
}
