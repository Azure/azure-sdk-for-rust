#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2021"

[dependencies]
toml_edit = "^0.22"
---

use std::{env, fs, path::PathBuf};
use std::io::Write;
use toml_edit::{DocumentMut, value};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let script_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let repo_root = script_dir.join("../../").canonicalize()?;

    // read the Cargo.toml file
    let cargo_toml_path = repo_root.join("Cargo.toml");
    let cargo_toml_content = fs::read_to_string(&cargo_toml_path).expect("Failed to read Cargo.toml");
    let mut cargo_toml: DocumentMut = cargo_toml_content.parse().expect("Failed to parse Cargo.toml");

    cargo_toml["workspace"]["dependencies"]["azure_core"]["version"] = value("0.99.0");

    // reserialize the Cargo.toml file
    let mut cargo_toml_file = fs::File::create(&cargo_toml_path)?;
    let cargo_toml_content = cargo_toml.to_string();
    cargo_toml_file
        .write_all(cargo_toml_content.as_bytes())
        .expect("Failed to write Cargo.toml");

    return Ok(());
}
