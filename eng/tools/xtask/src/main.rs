// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{env, process::exit};

mod check_editorconfig;
#[cfg(not(target_arch = "wasm32"))]
mod update_cratenames;
#[cfg(not(target_arch = "wasm32"))]
mod verify_dependencies;

fn main() {
    let mut args = env::args();
    args.next().expect("expected executable");

    let Some(command) = args.next() else {
        eprintln!("Error: missing command.");
        usage();

        exit(1);
    };

    match command.as_str() {
        "check-editorconfig" => check_editorconfig::run(args),
        #[cfg(not(target_arch = "wasm32"))]
        "update-cratenames" => update_cratenames::run(),
        #[cfg(not(target_arch = "wasm32"))]
        "verify-dependencies" => verify_dependencies::run(args),
        "--help" | "-h" => usage(),
        _ => {
            eprintln!("Error: unknown command: {}", command);
            usage();

            exit(1);
        }
    }
}

fn usage() {
    eprintln!("Usage: cargo xtask <command> [options]\n");
    eprintln!("Commands:\n");
    eprintln!("  check-editorconfig: Enforce (some) rules of .editorconfig");
    eprintln!("  update-cratenames: Update eng/dict/crates.txt with names of crates used throughout the workspace.");
    eprintln!("  verify-dependencies: Verify that all dependencies are centralized in the workspace Cargo.toml.");
}

fn find_file(dir: impl AsRef<std::path::Path>, name: &str) -> Option<String> {
    for dir in dir.as_ref().ancestors() {
        let path = dir.join(name);
        if path.exists() {
            return Some(path.to_str().unwrap().into());
        }
    }
    None
}
