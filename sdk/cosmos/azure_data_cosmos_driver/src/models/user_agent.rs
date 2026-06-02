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
/// - Optional wrapping-SDK identifier (e.g., `azsdk-rust-cosmos/0.34.0`),
///   prepended when the driver is used through a higher-level SDK
/// - Azure SDK identifier (`azsdk-rust-`)
/// - Driver name and version
/// - OS name and architecture
/// - Rust version (compile time)
///
/// An optional suffix can be appended (typically from [`UserAgentSuffix`],
/// [`WorkloadId`], or [`CorrelationId`]).
///
/// # Example
///
/// Driver used directly, no suffix:
/// `azsdk-rust-cosmos-driver/0.1.0 windows/x86_64 rustc/1.85.0`
///
/// Driver used directly, with suffix:
/// `azsdk-rust-cosmos-driver/0.1.0 windows/x86_64 rustc/1.85.0 myapp-westus2`
///
/// Wrapped by a higher-level SDK:
/// `azsdk-rust-cosmos/0.34.0 azsdk-rust-cosmos-driver/0.1.0 windows/x86_64 rustc/1.85.0 myapp-westus2`
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
        Self::new(None::<&str>, None::<&str>)
    }
}

impl UserAgent {
    /// Returns the driver-owned base user agent (without wrapping SDK or suffix).
    ///
    /// Format: `azsdk-rust-{sdk-name}/{version} {os}/{arch} rustc/{rust-version}`
    fn driver_base_user_agent() -> String {
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

    /// Builds the user agent prefix, optionally prepending a wrapping-SDK
    /// identifier (e.g., `azsdk-rust-cosmos/0.34.0`).
    ///
    /// The wrapping identifier is ASCII-stripped and trimmed; an empty or
    /// whitespace-only value is treated as absent.
    fn base_user_agent(wrapping_sdk_identifier: Option<&str>) -> String {
        let driver = Self::driver_base_user_agent();
        let wrapping = wrapping_sdk_identifier
            .map(strip_non_ascii)
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        match wrapping {
            Some(mut w) => {
                // Reserve room for the driver base + a separator and, when
                // possible, also reserve the maximum-allowed suffix length
                // (`UserAgentSuffix::MAX_LENGTH` + separator). The suffix
                // typically carries the operator-supplied telemetry tag, so
                // we prefer to truncate a pathologically long wrapping
                // identifier rather than silently drop the suffix. The driver
                // portion is required and is never truncated.
                let reserved_for_suffix = UserAgentSuffix::MAX_LENGTH + 1;
                let driver_with_sep = driver.len() + 1;
                let preferred_max_wrap =
                    MAX_USER_AGENT_LENGTH.saturating_sub(driver_with_sep + reserved_for_suffix);
                let absolute_max_wrap = MAX_USER_AGENT_LENGTH.saturating_sub(driver_with_sep);
                // Fall back to the absolute cap if the preferred cap would be
                // zero (e.g., a future driver base big enough to leave no
                // suffix headroom). This still guarantees the final string
                // fits in `MAX_USER_AGENT_LENGTH`.
                let max_wrap = if preferred_max_wrap == 0 {
                    absolute_max_wrap
                } else {
                    preferred_max_wrap
                };
                if w.len() > max_wrap {
                    w.truncate(max_wrap);
                }
                if w.is_empty() {
                    return driver;
                }
                let mut value = String::with_capacity(w.len() + 1 + driver.len());
                value.push_str(&w);
                value.push(' ');
                value.push_str(&driver);
                value
            }
            None => driver,
        }
    }

    /// Creates a new user agent with optional wrapping-SDK identifier and suffix.
    ///
    /// The wrapping identifier is prepended to the driver's base prefix; the
    /// suffix is appended after the base, separated by a space. If the
    /// resulting string exceeds 255 characters, the suffix is truncated.
    fn new(
        wrapping_sdk_identifier: Option<impl AsRef<str>>,
        suffix: Option<impl Into<String>>,
    ) -> Self {
        // Normalize to ASCII once; this makes byte-length checks safe and avoids
        // reprocessing after we build the final string.
        let base = strip_non_ascii(&Self::base_user_agent(
            wrapping_sdk_identifier.as_ref().map(AsRef::as_ref),
        ));
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

    /// Creates a user agent with only a wrapping-SDK identifier (no suffix).
    pub(crate) fn from_wrapping_sdk_identifier(wrapping_sdk_identifier: Option<&str>) -> Self {
        Self::new(wrapping_sdk_identifier, None::<&str>)
    }

    /// Creates a user agent from a [`UserAgentSuffix`].
    pub(crate) fn from_suffix(
        wrapping_sdk_identifier: Option<&str>,
        suffix: &UserAgentSuffix,
    ) -> Self {
        Self::new(wrapping_sdk_identifier, Some(suffix.as_str()))
    }

    /// Creates a user agent from a [`WorkloadId`].
    pub(crate) fn from_workload_id(
        wrapping_sdk_identifier: Option<&str>,
        workload_id: WorkloadId,
    ) -> Self {
        Self::new(
            wrapping_sdk_identifier,
            Some(format!("w{}", workload_id.value())),
        )
    }

    /// Creates a user agent from a [`CorrelationId`].
    pub(crate) fn from_correlation_id(
        wrapping_sdk_identifier: Option<&str>,
        correlation_id: &CorrelationId,
    ) -> Self {
        Self::new(wrapping_sdk_identifier, Some(correlation_id.as_str()))
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

/// Normalizes a wrapping-SDK identifier the same way [`UserAgent`] would when
/// rendering the prefix: strips non-ASCII, trims surrounding whitespace, and
/// returns `None` for empty / whitespace-only input.
///
/// Used at builder set-time so a runtime accessor like
/// `CosmosDriverRuntime::wrapping_sdk_identifier()` returns the same value
/// that ultimately appears in the `User-Agent` header.
pub(crate) fn normalize_wrapping_sdk_identifier(value: &str) -> Option<String> {
    // Trim whitespace (including \t, \n) before ASCII normalization so a
    // whitespace-only input collapses to `None` instead of a string of
    // underscores produced by `strip_non_ascii`.
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    let normalized = strip_non_ascii(trimmed);
    if normalized.is_empty() {
        None
    } else {
        Some(normalized)
    }
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
        let ua = UserAgent::new(None::<&str>, Some("my-app"));
        assert!(ua.as_str().contains("my-app"));
        assert_eq!(ua.suffix(), Some("my-app"));
    }

    #[test]
    fn user_agent_from_user_agent_suffix() {
        let suffix = UserAgentSuffix::new("myapp-westus2");
        let ua = UserAgent::from_suffix(None, &suffix);
        assert!(ua.as_str().contains("myapp-westus2"));
    }

    #[test]
    fn user_agent_from_workload_id() {
        let workload_id = WorkloadId::new(25);
        let ua = UserAgent::from_workload_id(None, workload_id);
        assert!(ua.as_str().contains("w25"));
    }

    #[test]
    fn user_agent_from_correlation_id() {
        let correlation_id = CorrelationId::new("aks-prod-eastus");
        let ua = UserAgent::from_correlation_id(None, &correlation_id);
        assert!(ua.as_str().contains("aks-prod-eastus"));
    }

    #[test]
    fn user_agent_strips_non_ascii() {
        // Non-ASCII characters should be replaced with underscores
        let input = "test café";
        let stripped = strip_non_ascii(input);
        assert!(stripped.is_ascii());
    }

    #[test]
    fn user_agent_with_wrapping_sdk_identifier_prepends() {
        let ua = UserAgent::from_wrapping_sdk_identifier(Some("azsdk-rust-cosmos/0.34.0"));
        assert!(
            ua.as_str()
                .starts_with("azsdk-rust-cosmos/0.34.0 azsdk-rust-cosmos-driver/"),
            "unexpected user agent: {}",
            ua.as_str()
        );
        assert!(ua.suffix().is_none());
    }

    #[test]
    fn user_agent_wrapping_plus_suffix() {
        let suffix = UserAgentSuffix::new("myapp-westus2");
        let ua = UserAgent::from_suffix(Some("azsdk-rust-cosmos/0.34.0"), &suffix);
        let s = ua.as_str();
        assert!(
            s.starts_with("azsdk-rust-cosmos/0.34.0 azsdk-rust-cosmos-driver/"),
            "missing wrapping prefix in: {s}"
        );
        assert!(s.ends_with(" myapp-westus2"), "missing suffix in: {s}");
    }

    #[test]
    fn user_agent_wrapping_identifier_strips_non_ascii() {
        let ua = UserAgent::from_wrapping_sdk_identifier(Some("azsdk-rust-café/0.1.0"));
        assert!(ua.as_str().is_ascii());
        assert!(ua.as_str().starts_with("azsdk-rust-caf_/0.1.0 "));
    }

    #[test]
    fn user_agent_empty_wrapping_identifier_treated_as_absent() {
        let ua_empty = UserAgent::from_wrapping_sdk_identifier(Some(""));
        let ua_ws = UserAgent::from_wrapping_sdk_identifier(Some("   "));
        let ua_default = UserAgent::default();
        assert_eq!(ua_empty.as_str(), ua_default.as_str());
        assert_eq!(ua_ws.as_str(), ua_default.as_str());
    }

    #[test]
    fn user_agent_respects_max_length_with_wrapping_and_suffix() {
        // Force a long wrapping identifier and a long suffix; total must still be capped.
        let long_wrap = format!("azsdk-rust-{}", "x".repeat(200));
        let suffix = UserAgentSuffix::new("a".repeat(25));
        let ua = UserAgent::from_suffix(Some(&long_wrap), &suffix);
        assert!(
            ua.as_str().len() <= MAX_USER_AGENT_LENGTH,
            "len={} value={}",
            ua.as_str().len(),
            ua.as_str()
        );
    }

    #[test]
    fn user_agent_preserves_suffix_when_wrapping_is_pathological() {
        // Regression: a pathologically long wrapping identifier must not
        // silently displace the operator-supplied suffix, which is the
        // primary telemetry-tag carrier. The wrapping identifier is
        // truncated instead.
        let long_wrap = format!("azsdk-rust-{}", "x".repeat(500));
        let suffix = UserAgentSuffix::new("myapp-westus2");
        let ua = UserAgent::from_suffix(Some(&long_wrap), &suffix);
        assert!(
            ua.as_str().len() <= MAX_USER_AGENT_LENGTH,
            "exceeded cap: {}",
            ua.as_str()
        );
        assert_eq!(ua.suffix(), Some("myapp-westus2"));
        assert!(
            ua.as_str().ends_with(" myapp-westus2"),
            "suffix lost: {}",
            ua.as_str()
        );
    }
}
