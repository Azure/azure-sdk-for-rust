// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use rustc_version::version;
use std::env;
use std::io::{self, Write};
use std::process::Command;

fn generate_typespec() {
    println!("Generating typespec...");

    let output = Command::new("tsp-client.cmd")
        .args(&["update"])
        //.stdout(Stdio::piped())
        .output()
        .unwrap();

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());

    println!("Generating typespec Completed.");
}

fn main() {
    let version = match version() {
        Ok(version) => version.to_string(),
        Err(_) => "unknown".to_owned(),
    };
    println!("cargo:rustc-env=AZSDK_RUSTC_VERSION={version}");

    let generate_enabled = env::var("GENERATE_TYPESPEC").unwrap_or("1".to_string());

    if generate_enabled == "1" {
        generate_typespec();
    }
}
