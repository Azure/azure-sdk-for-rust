// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Process-global **diagnostics preamble**: the SDK/driver version + User-Agent provenance.
//!
//! These values are constant for the lifetime of the process, so they must never be stored
//! per-attempt or per-operation. The capture log records a single 1-byte [`PREAMBLE_ID`]
//! referencing this table; the full strings are only rehydrated at *build* time (and only when
//! the gate decides the diagnostics are worth building). On a self-contained `AZD1` blob the
//! preamble is expanded into the blob at build time, so the 1-byte id is an internal hot-path
//! optimization only — it never appears on the wire.
//!
//! This mirrors what .NET does: Azure.Core builds the User-Agent once
//! (`azsdk-net-<pkg>/<ver> (<runtime>; <os>)`) and Cosmos's `UserAgentContainer` appends a
//! feature suffix; diagnostics record it once, not per request. The Rust SDK's own User-Agent
//! has the same shape — `azsdk-rust-<crate>/<ver> (<os>; <arch>)` (see
//! `azure_core`'s `user_agent.rs`) — so this preamble is the diagnostics-side analog.

use std::sync::OnceLock;

/// The id every capture log uses to reference the single process-global preamble.
pub const PREAMBLE_ID: u8 = 0;

/// The driver crate version, sourced at compile time.
const DRIVER_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Optional override of the public SDK name + version, set once by the consuming SDK.
///
/// The driver does not know the version of the public crate built on top of it
/// (e.g. `azure_data_cosmos`). The public crate may call [`set_sdk_provenance`] once at startup
/// to record its own name/version; otherwise the driver's own identity is used.
static SDK_OVERRIDE: OnceLock<(String, String)> = OnceLock::new();

/// Records the public SDK name and version for diagnostics provenance.
///
/// Idempotent: only the first call takes effect (returns `true`); later calls are ignored
/// (`false`). Call once at client construction, before any diagnostics are built.
pub fn set_sdk_provenance(name: impl Into<String>, version: impl Into<String>) -> bool {
    SDK_OVERRIDE.set((name.into(), version.into())).is_ok()
}

/// The constant version/User-Agent provenance for this process.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Preamble {
    /// SDK crate name (the public crate when set via [`set_sdk_provenance`], else the driver).
    pub sdk_name: String,
    /// SDK version string (`major.minor.patch`).
    pub sdk_version: String,
    /// Cosmos driver version string (`major.minor.patch`).
    pub driver_version: String,
    /// Optional User-Agent feature suffix (e.g. enabled-feature flags).
    pub ua_suffix: String,
    /// Target OS.
    pub os: &'static str,
    /// Target architecture.
    pub arch: &'static str,
}

impl Preamble {
    /// Rehydrates the full User-Agent string in the SDK's canonical shape.
    ///
    /// Matches `azure_core`'s `UserAgentPolicy` format: `azsdk-rust-<crate>/<ver> (<os>; <arch>)`,
    /// with an optional feature suffix.
    pub fn user_agent(&self) -> String {
        let base = format!(
            "azsdk-rust-{}/{} ({}; {})",
            self.sdk_name, self.sdk_version, self.os, self.arch
        );
        if self.ua_suffix.is_empty() {
            base
        } else {
            format!("{base} {}", self.ua_suffix)
        }
    }
}

/// Returns the process-global preamble, building it once on first use.
pub fn get() -> &'static Preamble {
    static PREAMBLE: OnceLock<Preamble> = OnceLock::new();
    PREAMBLE.get_or_init(|| {
        let (sdk_name, sdk_version) = match SDK_OVERRIDE.get() {
            Some((name, version)) => (name.clone(), version.clone()),
            None => (
                "azure_data_cosmos_driver".to_string(),
                DRIVER_VERSION.to_string(),
            ),
        };
        Preamble {
            sdk_name,
            sdk_version,
            driver_version: DRIVER_VERSION.to_string(),
            ua_suffix: String::new(),
            os: std::env::consts::OS,
            arch: std::env::consts::ARCH,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_agent_has_canonical_shape() {
        let p = Preamble {
            sdk_name: "azure_data_cosmos".to_string(),
            sdk_version: "1.2.3".to_string(),
            driver_version: "0.1.0".to_string(),
            ua_suffix: String::new(),
            os: "windows",
            arch: "x86_64",
        };
        assert_eq!(
            p.user_agent(),
            "azsdk-rust-azure_data_cosmos/1.2.3 (windows; x86_64)"
        );
    }

    #[test]
    fn user_agent_includes_suffix() {
        let p = Preamble {
            sdk_name: "azure_data_cosmos".to_string(),
            sdk_version: "1.2.3".to_string(),
            driver_version: "0.1.0".to_string(),
            ua_suffix: "feat=hedging".to_string(),
            os: "linux",
            arch: "aarch64",
        };
        assert_eq!(
            p.user_agent(),
            "azsdk-rust-azure_data_cosmos/1.2.3 (linux; aarch64) feat=hedging"
        );
    }

    #[test]
    fn process_preamble_uses_driver_version_by_default() {
        // `get()` is process-global; assert it reports the compiled driver version.
        let p = get();
        assert_eq!(p.driver_version, env!("CARGO_PKG_VERSION"));
        assert!(!p.user_agent().is_empty());
    }
}
