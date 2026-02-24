// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! User agent string for HTTP requests to Cosmos DB.

use std::fmt;

use crate::options::{CorrelationId, UserAgentSuffix, WorkloadId};

/// Maximum length for the full user agent string (HTTP header limit).
const MAX_USER_AGENT_LENGTH: usize = 255;

/// Azure SDK user agent prefix.
const AZSDK_USER_AGENT_PREFIX: &str = "azsdk-rust-";

/// SDK name used in the user agent.
const SDK_NAME: &str = "cosmos-driver";

/// SDK version, retrieved from Cargo.toml at compile time.
const SDK_VERSION: &str = env!("CARGO_PKG_VERSION");

/// User agent string for HTTP requests.
///
/// The user agent is automatically computed with a static prefix containing:
/// - Azure SDK identifier (`azsdk-rust-`)
/// - SDK name and version
/// - OS name and architecture
/// - Rust version (compile time)
///
/// An optional suffix can be appended (typically from [`UserAgentSuffix`],
/// [`WorkloadId`], or [`CorrelationId`]).
///
/// # Example
///
/// Without suffix: `azsdk-rust-cosmos-driver/0.1.0 windows/x86_64 rustc/1.85.0`
/// With suffix: `azsdk-rust-cosmos-driver/0.1.0 windows/x86_64 rustc/1.85.0 myapp-westus2`
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserAgent {
    /// The full computed user agent string.
    full_user_agent: String,
    /// The suffix that was appended (if any).
    suffix: Option<String>,
}

impl Default for UserAgent {
    fn default() -> Self {
        Self::new(None::<&str>)
    }
}

impl UserAgent {
    /// Returns the base user agent prefix (without suffix).
    ///
    /// Format: `azsdk-rust-{sdk-name}/{version} {os}/{arch} rustc/{rust-version}`
    fn base_user_agent() -> String {
        let os_name = std::env::consts::OS;
        let os_arch = std::env::consts::ARCH;

        // `RUSTC_VERSION` is an optional compile-time environment variable that should
        // contain the Rust compiler version (e.g., "1.85.0"). It can be set by:
        // - CI/CD pipelines (e.g., `RUSTC_VERSION=$(rustc --version | cut -d' ' -f2)`)
        // - A build script (`build.rs`) that captures `rustc --version` output
        // - Manual export before building
        // If not set, falls back to "unknown" in the user agent string.
        let rust_version = option_env!("RUSTC_VERSION").unwrap_or("unknown");

        let mut value = String::with_capacity(
            AZSDK_USER_AGENT_PREFIX.len()
                + SDK_NAME.len()
                + 1
                + SDK_VERSION.len()
                + 1
                + os_name.len()
                + 1
                + os_arch.len()
                + 7
                + rust_version.len(),
        );
        value.push_str(AZSDK_USER_AGENT_PREFIX);
        value.push_str(SDK_NAME);
        value.push('/');
        value.push_str(SDK_VERSION);
        value.push(' ');
        value.push_str(os_name);
        value.push('/');
        value.push_str(os_arch);
        value.push_str(" rustc/");
        value.push_str(rust_version);
        value
    }

    /// Creates a new user agent with an optional suffix.
    ///
    /// The suffix is appended to the base user agent, separated by a space.
    /// If the resulting string exceeds 255 characters, the suffix is truncated.
    fn new(suffix: Option<impl Into<String>>) -> Self {
        // Normalize to ASCII once; this makes byte-length checks safe and avoids
        // reprocessing after we build the final string.
        let base = strip_non_ascii(&Self::base_user_agent());
        let normalized_suffix = suffix.map(Into::into).map(|s| strip_non_ascii(&s));

        let max_suffix_len = MAX_USER_AGENT_LENGTH.saturating_sub(base.len() + 1);
        let effective_suffix = normalized_suffix.and_then(|s| {
            if s.is_empty() || max_suffix_len == 0 {
                None
            } else {
                Some(s[..s.len().min(max_suffix_len)].to_string())
            }
        });

        let mut full_user_agent = String::with_capacity(
            base.len() + effective_suffix.as_ref().map_or(0, |s| 1 + s.len()),
        );
        full_user_agent.push_str(&base);
        if let Some(s) = &effective_suffix {
            full_user_agent.push(' ');
            full_user_agent.push_str(s);
        }

        Self {
            full_user_agent,
            suffix: effective_suffix,
        }
    }

    /// Creates a user agent from a [`UserAgentSuffix`].
    pub(crate) fn from_suffix(suffix: &UserAgentSuffix) -> Self {
        Self::new(Some(suffix.as_str()))
    }

    /// Creates a user agent from a [`WorkloadId`].
    pub(crate) fn from_workload_id(workload_id: WorkloadId) -> Self {
        Self::new(Some(format!("w{}", workload_id.value())))
    }

    /// Creates a user agent from a [`CorrelationId`].
    pub(crate) fn from_correlation_id(correlation_id: &CorrelationId) -> Self {
        Self::new(Some(correlation_id.as_str()))
    }

    /// Returns the full user agent string.
    pub fn as_str(&self) -> &str {
        &self.full_user_agent
    }

    /// Returns the suffix that was used, if any.
    pub fn suffix(&self) -> Option<&str> {
        self.suffix.as_deref()
    }
}

impl fmt::Display for UserAgent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_user_agent)
    }
}

/// Strips non-ASCII characters from a string, replacing them with underscores.
fn strip_non_ascii(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii() && !c.is_ascii_control() {
                c
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_agent_default_has_base_prefix() {
        let ua = UserAgent::default();
        assert!(ua.as_str().starts_with("azsdk-rust-cosmos-driver/"));
        assert!(ua.suffix().is_none());
    }

    #[test]
    fn user_agent_with_suffix() {
        let ua = UserAgent::new(Some("my-app"));
        assert!(ua.as_str().contains("my-app"));
        assert_eq!(ua.suffix(), Some("my-app"));
    }

    #[test]
    fn user_agent_from_user_agent_suffix() {
        let suffix = UserAgentSuffix::new("myapp-westus2");
        let ua = UserAgent::from_suffix(&suffix);
        assert!(ua.as_str().contains("myapp-westus2"));
    }

    #[test]
    fn user_agent_from_workload_id() {
        let workload_id = WorkloadId::new(25);
        let ua = UserAgent::from_workload_id(workload_id);
        assert!(ua.as_str().contains("w25"));
    }

    #[test]
    fn user_agent_from_correlation_id() {
        let correlation_id = CorrelationId::new("aks-prod-eastus");
        let ua = UserAgent::from_correlation_id(&correlation_id);
        assert!(ua.as_str().contains("aks-prod-eastus"));
    }

    #[test]
    fn user_agent_strips_non_ascii() {
        // Non-ASCII characters should be replaced with underscores
        let input = "test café";
        let stripped = strip_non_ascii(input);
        assert!(stripped.is_ascii());
    }
}
