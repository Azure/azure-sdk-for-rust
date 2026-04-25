#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2021"

[dependencies]
serde = { version = "1.0.228", features = ["derive"] }
toml = "1.1.2"
---

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf, process};

fn main() {
    let mode = match std::env::args().nth(1).as_deref() {
        Some("azure") => Mode::Azure,
        Some("crates.io") => Mode::CratesIo,
        Some(other) => {
            eprintln!("error: unknown registry \"{other}\", expected \"azure\" or \"crates.io\"");
            process::exit(1);
        }
        None => {
            eprintln!("usage: use-registry.rs <azure|crates.io>");
            process::exit(1);
        }
    };

    let config_path = find_config();
    let content = std::fs::read_to_string(&config_path).unwrap_or_else(|err| {
        eprintln!("error: failed to read {}: {err}", config_path.display());
        process::exit(1);
    });

    let mut config: CargoConfig = toml::from_str(&content).unwrap_or_else(|err| {
        eprintln!("error: failed to parse {}: {err}", config_path.display());
        process::exit(1);
    });

    // Find the name of the other registry defined under [registries].
    let registry_name = match config.registries.keys().next() {
        Some(name) => name.clone(),
        None => {
            eprintln!("error: no registries defined in {}", config_path.display());
            process::exit(1);
        }
    };

    let crates_io = config
        .source
        .entry("crates-io".to_string())
        .or_insert_with(Source::default);

    match mode {
        Mode::Azure => {
            if crates_io.replace_with.as_deref() == Some(&registry_name) {
                // Already pointing to the registry; nothing to do.
                return;
            }
            crates_io.replace_with = Some(registry_name);
        }
        Mode::CratesIo => {
            if crates_io.replace_with.is_none() {
                // Already using crates.io; nothing to do.
                return;
            }
            crates_io.replace_with = None;
        }
    }

    let output = toml::to_string(&config).unwrap_or_else(|err| {
        eprintln!("error: failed to serialize config: {err}");
        process::exit(1);
    });

    std::fs::write(&config_path, output).unwrap_or_else(|err| {
        eprintln!("error: failed to write {}: {err}", config_path.display());
        process::exit(1);
    });

    eprintln!("updated {}", config_path.display());
}

fn find_config() -> PathBuf {
    let dir = std::env::current_dir().expect("current directory");
    for ancestor in dir.ancestors() {
        let path = ancestor.join(".cargo/config.toml");
        if path.exists() {
            return path;
        }
    }
    eprintln!("error: .cargo/config.toml not found");
    process::exit(1);
}

enum Mode {
    Azure,
    CratesIo,
}

#[derive(Deserialize, Serialize)]
struct CargoConfig {
    #[serde(default)]
    registries: BTreeMap<String, Registry>,
    #[serde(default)]
    source: BTreeMap<String, Source>,
}

#[derive(Deserialize, Serialize)]
struct Registry {
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<String>,
}

#[derive(Default, Deserialize, Serialize)]
struct Source {
    #[serde(rename = "replace-with", skip_serializing_if = "Option::is_none")]
    replace_with: Option<String>,
}
