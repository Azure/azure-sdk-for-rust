// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration tests for the perf CLI's `--user-agent-suffix` flag.
//!
//! Driving the real `azure_data_cosmos_perf` binary (via `CARGO_BIN_EXE_*`)
//! exercises end-to-end clap parsing, environment-variable resolution, and
//! the `UserAgentSuffix` validation that lives in `main`. These paths sit
//! outside any library API in this crate, so a `tests/` integration test is
//! the appropriate coverage layer.

use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_azure_data_cosmos_perf");

/// `--help` should advertise both the flag and its default value so it shows
/// up in operator-facing tooling. Asserts on the flag name, the
/// `AZURE_COSMOS_USER_AGENT_SUFFIX` env binding, and the `rust-perf` default
/// to catch silent renames or default-drift.
#[test]
fn help_documents_user_agent_suffix_flag() {
    let output = Command::new(BIN)
        .arg("--help")
        .output()
        .expect("perf binary runs");
    assert!(
        output.status.success(),
        "--help exited non-zero: status={:?} stderr={}",
        output.status,
        String::from_utf8_lossy(&output.stderr),
    );
    let stdout = String::from_utf8(output.stdout).expect("utf-8 help output");
    for needle in [
        "--user-agent-suffix",
        "AZURE_COSMOS_USER_AGENT_SUFFIX",
        "rust-perf",
    ] {
        assert!(
            stdout.contains(needle),
            "--help output missing {needle:?}; full output:\n{stdout}",
        );
    }
}

/// An invalid suffix (too long for `UserAgentSuffix::MAX_LENGTH`) must be
/// rejected at startup before any network activity. We pass otherwise-valid
/// CLI args so the failure can only come from the `UserAgentSuffix::try_new`
/// validation step in `main`.
#[test]
fn invalid_user_agent_suffix_is_rejected_before_network() {
    // 26 characters: exceeds UserAgentSuffix::MAX_LENGTH (25).
    let too_long = "a".repeat(26);
    let output = Command::new(BIN)
        .args([
            "--endpoint",
            "https://example.documents.azure.com:443/",
            "--auth",
            "key",
            "--key",
            "dGVzdGtleQ==",
            "--application-region",
            "East US 2",
            "--user-agent-suffix",
            &too_long,
        ])
        .output()
        .expect("perf binary runs");
    assert!(
        !output.status.success(),
        "expected non-zero exit for invalid suffix; stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--user-agent-suffix") && stderr.contains("invalid"),
        "stderr should explain the --user-agent-suffix validation failure; got: {stderr}",
    );
}
