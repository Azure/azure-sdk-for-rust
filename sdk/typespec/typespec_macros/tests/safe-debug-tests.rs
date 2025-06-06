// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

#[test]
fn debug_tests() {
    let test_root = {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests");
        p.push("data");
        p.push("safe-debug-tests");
        p
    };

    let output = Command::new(env!("CARGO"))
        .arg("test")
        .arg("--no-fail-fast")
        .arg("--test")
        .arg("debug")
        .arg("--features")
        .arg("debug")
        .current_dir(test_root.clone())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("cargo to start running");
    assert!(
        output.status.success(),
        "tests failed:\n{}",
        String::from_utf8_lossy(output.stdout.as_ref())
    );
}

#[test]
fn safe_debug_tests() {
    let test_root = {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests");
        p.push("data");
        p.push("safe-debug-tests");
        p
    };

    let output = Command::new(env!("CARGO"))
        .arg("test")
        .arg("--no-fail-fast")
        .arg("--test")
        .arg("safe-debug")
        .current_dir(test_root.clone())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("cargo to start running");
    assert!(
        output.status.success(),
        "tests failed:\n{}",
        String::from_utf8_lossy(output.stdout.as_ref())
    );
}
