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

/// Ensures the endpoint's URL scheme is permitted for the given host.
///
/// Plaintext `http://` (non-HTTPS) endpoints are only permitted when the host is a
/// known emulator host (see [`is_emulator_host`]). Any other scheme (notably `https`)
/// is always permitted. This is the single source of truth for the HTTP-only-for-emulator
/// rule so emulator detection is never duplicated.
///
/// # Arguments
///
/// * `endpoint` - The account endpoint to validate.
///
/// # Errors
///
/// Returns a [`CosmosError`](crate::error::CosmosError) with
/// [`CosmosStatus::CLIENT_INVALID_ACCOUNT_ENDPOINT_URL`](crate::error::CosmosStatus::CLIENT_INVALID_ACCOUNT_ENDPOINT_URL)
/// when the endpoint uses `http://` but does not point to an emulator host.
pub(crate) fn ensure_endpoint_scheme_allowed(
    endpoint: &AccountEndpoint,
) -> crate::error::Result<()> {
    if endpoint.url().scheme() == "http" && !is_emulator_host(endpoint) {
        return Err(crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::CLIENT_INVALID_ACCOUNT_ENDPOINT_URL)
            .with_message(
                "invalid account endpoint: http:// (non-HTTPS) endpoints are only permitted \
                 when connecting to the Cosmos DB emulator; use an https:// endpoint for \
                 production accounts",
            )
            .build());
    }
    Ok(())
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

    #[test]
    fn https_production_endpoint_allowed() {
        let endpoint =
            AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/").unwrap();
        assert!(ensure_endpoint_scheme_allowed(&endpoint).is_ok());
    }

    #[test]
    fn http_emulator_hosts_allowed() {
        for url in [
            "http://localhost:8081/",
            "http://127.0.0.1:8081/",
            "http://[::1]:8081/",
        ] {
            let endpoint = AccountEndpoint::try_from(url).unwrap();
            assert!(
                ensure_endpoint_scheme_allowed(&endpoint).is_ok(),
                "expected {url} to be allowed"
            );
        }
    }

    #[test]
    fn http_custom_emulator_host_via_env_allowed() {
        let _guard = ENV_MUTEX.lock().unwrap();

        let original = env::var(AZURE_COSMOS_EMULATOR_HOST).ok();
        env::set_var(AZURE_COSMOS_EMULATOR_HOST, "my-custom-emulator.local");

        let endpoint = AccountEndpoint::try_from("http://my-custom-emulator.local:8081/").unwrap();
        assert!(ensure_endpoint_scheme_allowed(&endpoint).is_ok());

        match original {
            Some(val) => env::set_var(AZURE_COSMOS_EMULATOR_HOST, val),
            None => env::remove_var(AZURE_COSMOS_EMULATOR_HOST),
        }
    }

    #[test]
    fn http_production_endpoint_rejected() {
        let endpoint = AccountEndpoint::try_from("http://myaccount.documents.azure.com/").unwrap();
        let error = ensure_endpoint_scheme_allowed(&endpoint).unwrap_err();
        assert_eq!(
            error.status(),
            crate::error::CosmosStatus::CLIENT_INVALID_ACCOUNT_ENDPOINT_URL
        );
    }

    #[test]
    fn http_non_emulator_custom_domain_rejected() {
        let endpoint = AccountEndpoint::try_from("http://my.custom.domain/").unwrap();
        assert!(ensure_endpoint_scheme_allowed(&endpoint).is_err());
    }
}
