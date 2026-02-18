// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Emulator host detection.
//!
//! This module provides utilities for detecting when an endpoint is pointing
//! to a local Cosmos DB emulator rather than a production endpoint.

use crate::models::AccountEndpoint;

/// Environment variable for custom emulator host.
const AZURE_COSMOS_EMULATOR_HOST: &str = "AZURE_COSMOS_EMULATOR_HOST";

/// Known localhost hostnames that indicate an emulator endpoint.
const EMULATOR_LOCALHOST_HOSTS: &[&str] = &["localhost", "127.0.0.1", "[::1]", "[0:0:0:0:0:0:0:1]"];

/// Determines if the given endpoint is pointing to a Cosmos DB emulator.
///
/// An endpoint is considered an emulator if:
/// 1. The `AZURE_COSMOS_EMULATOR_HOST` environment variable is set and the
///    endpoint's host matches its value (case-insensitive).
/// 2. The endpoint's host is one of the known localhost variants:
///    - `localhost`
///    - `127.0.0.1`
///    - `[::1]`
///    - `[0:0:0:0:0:0:0:1]`
///
/// # Arguments
///
/// * `endpoint` - The account endpoint to check.
///
/// # Returns
///
/// `true` if the endpoint is an emulator, `false` otherwise.
pub(crate) fn is_emulator_host(endpoint: &AccountEndpoint) -> bool {
    let host = endpoint.host();

    // First, check if there's a custom emulator host configured
    if let Ok(custom_emulator_host) = std::env::var(AZURE_COSMOS_EMULATOR_HOST) {
        if !custom_emulator_host.is_empty() && host.eq_ignore_ascii_case(&custom_emulator_host) {
            return true;
        }
    }

    // Fall back to known localhost patterns
    EMULATOR_LOCALHOST_HOSTS
        .iter()
        .any(|h| host.eq_ignore_ascii_case(h))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;

    /// Mutex to serialize tests that modify the AZURE_COSMOS_EMULATOR_HOST env var.
    /// This prevents race conditions when tests run in parallel.
    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn localhost_is_emulator() {
        let endpoint = AccountEndpoint::try_from("https://localhost:8081/").unwrap();
        assert!(is_emulator_host(&endpoint));
    }

    #[test]
    fn localhost_case_insensitive() {
        let endpoint = AccountEndpoint::try_from("https://LOCALHOST:8081/").unwrap();
        assert!(is_emulator_host(&endpoint));

        let endpoint = AccountEndpoint::try_from("https://LocalHost:8081/").unwrap();
        assert!(is_emulator_host(&endpoint));
    }

    #[test]
    fn ipv4_loopback_is_emulator() {
        let endpoint = AccountEndpoint::try_from("https://127.0.0.1:8081/").unwrap();
        assert!(is_emulator_host(&endpoint));
    }

    #[test]
    fn ipv6_loopback_short_is_emulator() {
        let endpoint = AccountEndpoint::try_from("https://[::1]:8081/").unwrap();
        assert!(is_emulator_host(&endpoint));
    }

    #[test]
    fn ipv6_loopback_full_is_emulator() {
        let endpoint = AccountEndpoint::try_from("https://[0:0:0:0:0:0:0:1]:8081/").unwrap();
        assert!(is_emulator_host(&endpoint));
    }

    #[test]
    fn production_endpoint_is_not_emulator() {
        let endpoint =
            AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/").unwrap();
        assert!(!is_emulator_host(&endpoint));
    }

    #[test]
    fn custom_emulator_host_via_env() {
        let _guard = ENV_MUTEX.lock().unwrap();

        // Save and clear any existing value
        let original = env::var(AZURE_COSMOS_EMULATOR_HOST).ok();

        // Set custom emulator host
        env::set_var(AZURE_COSMOS_EMULATOR_HOST, "my-custom-emulator.local");

        let custom = AccountEndpoint::try_from("https://my-custom-emulator.local:8081/").unwrap();
        assert!(is_emulator_host(&custom));

        // Production should still not be emulator
        let prod = AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/").unwrap();
        assert!(!is_emulator_host(&prod));

        // Restore original value
        match original {
            Some(val) => env::set_var(AZURE_COSMOS_EMULATOR_HOST, val),
            None => env::remove_var(AZURE_COSMOS_EMULATOR_HOST),
        }
    }

    #[test]
    fn custom_emulator_host_case_insensitive() {
        let _guard = ENV_MUTEX.lock().unwrap();

        // Save and clear any existing value
        let original = env::var(AZURE_COSMOS_EMULATOR_HOST).ok();

        // Set custom emulator host
        env::set_var(AZURE_COSMOS_EMULATOR_HOST, "MY-EMULATOR.LOCAL");

        let lower = AccountEndpoint::try_from("https://my-emulator.local:8081/").unwrap();
        assert!(is_emulator_host(&lower));

        let upper = AccountEndpoint::try_from("https://MY-EMULATOR.LOCAL:8081/").unwrap();
        assert!(is_emulator_host(&upper));

        // Restore original value
        match original {
            Some(val) => env::set_var(AZURE_COSMOS_EMULATOR_HOST, val),
            None => env::remove_var(AZURE_COSMOS_EMULATOR_HOST),
        }
    }

    #[test]
    fn empty_env_var_uses_default_hosts() {
        let _guard = ENV_MUTEX.lock().unwrap();

        // Save and clear any existing value
        let original = env::var(AZURE_COSMOS_EMULATOR_HOST).ok();

        // Set empty value
        env::set_var(AZURE_COSMOS_EMULATOR_HOST, "");

        // localhost should still work
        let endpoint = AccountEndpoint::try_from("https://localhost:8081/").unwrap();
        assert!(is_emulator_host(&endpoint));

        // Restore original value
        match original {
            Some(val) => env::set_var(AZURE_COSMOS_EMULATOR_HOST, val),
            None => env::remove_var(AZURE_COSMOS_EMULATOR_HOST),
        }
    }
}
