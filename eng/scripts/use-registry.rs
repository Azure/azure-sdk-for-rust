#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2021"

[dependencies]
toml_edit = "0.25.10"
---

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{fs, path::PathBuf, process};
use toml_edit::{value, DocumentMut, InlineTable, Item, Table, Value};

const REGISTRY: &str = "azure-sdk-for-rust";
const INDEX: &str =
    "sparse+https://pkgs.dev.azure.com/azure-sdk/_packaging/azure-sdk-for-rust/Cargo/index/";

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

    let cargo_home = std::env::var("CARGO_HOME").unwrap_or_else(|_| {
        eprintln!("error: CARGO_HOME is not set");
        process::exit(1);
    });
    let config_path = PathBuf::from(cargo_home).join("config.toml");

    let content = if config_path.exists() {
        fs::read_to_string(&config_path).unwrap_or_else(|err| {
            eprintln!("error: failed to read {}: {err}", config_path.display());
            process::exit(1);
        })
    } else {
        String::new()
    };

    let mut doc: DocumentMut = content.parse().unwrap_or_else(|err| {
        eprintln!("error: failed to parse {}: {err}", config_path.display());
        process::exit(1);
    });

    match mode {
        Mode::Azure => {
            ensure_registry(&mut doc);
            set_replace_with(&mut doc);
        }
        Mode::CratesIo => {
            remove_replace_with(&mut doc);
        }
    }

    fs::write(&config_path, doc.to_string()).unwrap_or_else(|err| {
        eprintln!("error: failed to write {}: {err}", config_path.display());
        process::exit(1);
    });

    eprintln!("updated {}", config_path.display());
}

/// Add the azure-sdk-for-rust registry to [registries] if not already present.
fn ensure_registry(doc: &mut DocumentMut) {
    let registries = doc
        .entry("registries")
        .or_insert_with(|| Item::Table(Table::new()))
        .as_table_mut()
        .expect("[registries] should be a table");

    if registries.get(REGISTRY).is_none() {
        let mut entry = InlineTable::new();
        entry.insert("index", Value::from(INDEX));
        registries.insert(REGISTRY, Item::Value(Value::InlineTable(entry)));
    }
}

/// Add [source.crates-io] if needed and set replace-with.
fn set_replace_with(doc: &mut DocumentMut) {
    let source = doc
        .entry("source")
        .or_insert_with(|| {
            let mut t = Table::new();
            t.set_implicit(true);
            Item::Table(t)
        })
        .as_table_mut()
        .expect("[source] should be a table");

    let crates_io = source
        .entry("crates-io")
        .or_insert_with(|| Item::Table(Table::new()))
        .as_table_mut()
        .expect("[source.crates-io] should be a table");

    crates_io.insert("replace-with", value(REGISTRY));
}

/// Remove replace-with from [source.crates-io] if present.
fn remove_replace_with(doc: &mut DocumentMut) {
    if let Some(source) = doc.get_mut("source").and_then(|s| s.as_table_mut()) {
        if let Some(crates_io) = source.get_mut("crates-io").and_then(|c| c.as_table_mut()) {
            crates_io.remove("replace-with");
        }
    }
}

enum Mode {
    Azure,
    CratesIo,
}
