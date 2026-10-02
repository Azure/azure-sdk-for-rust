// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! User agent string for HTTP requests to Cosmos DB.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use std::{
    fmt,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign},
};

use crate::options::UserAgentProperty;

/// Maximum length for the full user agent string (HTTP header limit).
const MAX_USER_AGENT_LENGTH: usize = 255;

/// Bitmask of client-side features advertised in the `User-Agent` header.
///
/// The Cosmos SDKs share a cross-language contract: enabled client features are
/// encoded as an `ft=<B64>` entry in the parenthesized `User-Agent` metadata
/// segment. `<B64>` is the OR-ed bit value as a big-endian unsigned integer
/// with leading zero bytes removed, encoded with the standard URL-safe base64
/// alphabet and no padding, so any base64 tool can decode it (for example
/// `Eg` is the single byte `0x12`). This lets backend telemetry bucket traffic by feature regardless of which
/// language SDK produced the request.
///
/// **The bit values below MUST stay consistent with the other Cosmos SDKs**
/// (.NET `UserAgentFeatureFlags`, Java `UserAgentFeatureFlags`). Do not
/// renumber existing bits — only append new ones.
///
/// # Example
///
/// ```ignore
/// let flags = UserAgentFeatureFlags::PER_PARTITION_CIRCUIT_BREAKER
///     | UserAgentFeatureFlags::HTTP2;
/// assert_eq!(flags.to_string(), "ft=Eg"); // 0x2 | 0x10 == 0x12 == bytes [0x12]
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) struct UserAgentFeatureFlags(u32);

impl UserAgentFeatureFlags {
    /// No features advertised. Renders to an empty token.
    pub(crate) const NONE: Self = Self(0);

    /// Per-partition automatic failover (PPAF). Cross-SDK bit value `0x1`.
    ///
    /// Reserved to keep Rust's bit assignments aligned with the .NET and Java
    /// Cosmos SDKs; this driver does not advertise it yet (PPAF is server-driven
    /// and resolved per-partition at request time, so it is unknown when the
    /// shared header value is computed).
    #[allow(dead_code)] // Reserved cross-SDK bit; not advertised by this driver yet.
    pub(crate) const PER_PARTITION_AUTOMATIC_FAILOVER: Self = Self(1);

    /// Per-partition circuit breaker (PPCB). Cross-SDK bit value `0x2`.
    pub(crate) const PER_PARTITION_CIRCUIT_BREAKER: Self = Self(2);

    /// Thin client mode. Cross-SDK bit value `0x4`.
    ///
    /// Reserved for cross-SDK parity; not advertised by this driver yet.
    #[allow(dead_code)] // Reserved cross-SDK bit; not advertised by this driver yet.
    pub(crate) const THIN_CLIENT: Self = Self(4);

    /// Cosmos binary encoding. Cross-SDK bit value `0x8`.
    ///
    /// Reserved for cross-SDK parity; not advertised by this driver yet.
    #[allow(dead_code)] // Reserved cross-SDK bit; not advertised by this driver yet.
    pub(crate) const BINARY_ENCODING: Self = Self(8);

    /// HTTP/2 transport. Cross-SDK bit value `0x10`.
    pub(crate) const HTTP2: Self = Self(16);

    /// Returns the raw bitmask value.
    pub(crate) const fn bits(self) -> u32 {
        self.0
    }

    /// Returns `true` when no feature bits are set.
    pub(crate) const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns the union of two flag sets (every bit set in either operand).
    pub(crate) const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Maps the statically-known client configuration to feature flags.
    ///
    /// Only features whose enablement is known at `User-Agent` construction
    /// time are advertised. PPAF is intentionally excluded: it is server-driven
    /// and resolved per-partition at request time, so it is not known when the
    /// shared header value is computed.
    pub(crate) fn from_client_config(is_http2_allowed: bool, ppcb_enabled: bool) -> Self {
        let mut flags = Self::NONE;
        if ppcb_enabled {
            flags |= Self::PER_PARTITION_CIRCUIT_BREAKER;
        }
        if is_http2_allowed {
            flags |= Self::HTTP2;
        }
        flags
    }
}

impl BitOr for UserAgentFeatureFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        self.union(rhs)
    }
}

impl BitOrAssign for UserAgentFeatureFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitAnd for UserAgentFeatureFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for UserAgentFeatureFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl fmt::Display for UserAgentFeatureFlags {
    /// Renders the cross-SDK `ft=<B64>` entry, or an empty string when no
    /// features are set.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            Ok(())
        } else {
            write!(f, "{FEATURE_FLAGS_KEY}={}", encode_base64url(self.bits()))
        }
    }
}

/// Product name the driver reports when it is used directly (no wrapping SDK).
///
/// The `azsdk-` prefix is required by the Azure SDK guidelines.
const DRIVER_PRODUCT_NAME: &str = "azsdk-rust-cosmos-driver";

/// Driver version, retrieved from Cargo.toml at compile time.
const DRIVER_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Rust compiler version, retrieved from build.rs at compile time.
const RUSTC_VERSION: &str = env!("AZSDK_RUSTC_VERSION");

/// Key of the mandatory first metadata entry, carrying the driver version.
pub(crate) const DRIVER_VERSION_KEY: &str = "drv";

/// Key of the feature-flags metadata entry.
pub(crate) const FEATURE_FLAGS_KEY: &str = "ft";

/// User agent string for HTTP requests.
///
/// Every Cosmos SDK builds its `User-Agent` through this type, so the format
/// is the same regardless of which language SDK wraps the driver:
///
/// ```text
/// {sdk}/{version} (drv={driver}; {os}; {arch}; {rustc}[; ft={B64}][; {key}={value}]...) [{suffix}]
/// ```
///
/// - `{sdk}/{version}` is the SDK name and version. When the driver is wrapped
///   by a higher-level SDK (which may not be written in Rust), this is the
///   identifier supplied by that SDK, e.g. `azsdk-rust-cosmos/0.34.0`.
///   Otherwise it is the driver itself, `azsdk-rust-cosmos-driver/{version}`.
/// - The parenthesized metadata segment is a `; `-separated list. Its first
///   entry is always `drv=<driver version>`, which marks the string as
///   driver-generated. Three positional values follow, always in this order:
///   operating system, CPU architecture, and the Rust compiler version the
///   driver was built with. Then come optional `key=value` entries: `ft` (the
///   cross-SDK client feature flags as a base64url number, omitted when none are
///   enabled) followed by any [`UserAgentProperty`] values supplied by the
///   wrapping SDK, such as its runtime version.
/// - An optional suffix (typically from [`UserAgentSuffix`], [`WorkloadId`], or
///   [`CorrelationId`]) follows the closing parenthesis after a space.
///
/// The string is limited to 255 ASCII characters. Truncation can never break
/// the structure: the wrapping-SDK identifier is shortened first, then custom
/// properties are dropped whole (last first) when they do not fit. The `drv`,
///   positional, and `ft` entries and the closing parenthesis are always present,
/// and the suffix is never altered or truncated.
///
/// The first `)` unambiguously marks the end of the SDK-provided portion; the
/// suffix is opaque and may contain any characters its source type allows.
///
/// # Example
///
/// Driver used directly, no suffix:
/// `azsdk-rust-cosmos-driver/0.1.0 (drv=0.1.0; windows; x86_64; 1.85.0)`
///
/// Driver used directly, with suffix and feature flags:
/// `azsdk-rust-cosmos-driver/0.1.0 (drv=0.1.0; windows; x86_64; 1.85.0; ft=Eg) myapp-westus2`
///
/// Wrapped by a higher-level SDK, with a custom property:
/// `azsdk-dotnet-cosmos/3.40.0 (drv=0.1.0; windows; x86_64; 1.85.0; ft=Eg; dotnet=8.0.1) myapp-westus2`
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
        UserAgentBuilder::default().build()
    }
}

impl UserAgent {
    /// Returns a [`UserAgentBuilder`] initialized with the driver's built-in
    /// values.
    pub(crate) fn builder() -> UserAgentBuilder {
        UserAgentBuilder::default()
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

/// Collects the inputs for a [`UserAgent`] and renders it.
///
/// The builder starts with the driver's built-in values (driver version,
/// operating system, architecture, and Rust compiler version), any of which
/// can be overridden. [`build`](Self::build) sanitizes the inputs, applies the
/// 255-character limit, and produces the final string; see [`UserAgent`] for
/// the layout and truncation priorities.
#[derive(Clone, Debug)]
pub(crate) struct UserAgentBuilder {
    wrapping_sdk_identifier: Option<String>,
    driver_version: String,
    os: String,
    arch: String,
    rustc_version: String,
    feature_flags: UserAgentFeatureFlags,
    properties: Vec<UserAgentProperty>,
    suffix: Option<String>,
}

impl Default for UserAgentBuilder {
    fn default() -> Self {
        Self {
            wrapping_sdk_identifier: None,
            driver_version: DRIVER_VERSION.to_owned(),
            os: std::env::consts::OS.to_owned(),
            arch: std::env::consts::ARCH.to_owned(),
            rustc_version: RUSTC_VERSION.to_owned(),
            feature_flags: UserAgentFeatureFlags::NONE,
            properties: Vec::new(),
            suffix: None,
        }
    }
}

impl UserAgentBuilder {
    /// Sets the `{sdk}/{version}` product token supplied by a wrapping SDK.
    ///
    /// Without one, the product token is the driver's own
    /// `azsdk-rust-cosmos-driver/{version}`. Empty or whitespace-only values
    /// are treated as unset.
    pub(crate) fn with_wrapping_sdk_identifier(mut self, identifier: impl Into<String>) -> Self {
        self.wrapping_sdk_identifier = Some(identifier.into());
        self
    }

    /// Overrides the driver version reported as `drv=`.
    #[cfg_attr(not(test), allow(dead_code))] // Overrides are exercised by tests and reserved for SDK wrappers.
    pub(crate) fn with_driver_version(mut self, version: impl Into<String>) -> Self {
        self.driver_version = version.into();
        self
    }

    /// Overrides the operating system name.
    #[cfg_attr(not(test), allow(dead_code))] // Overrides are exercised by tests and reserved for SDK wrappers.
    pub(crate) fn with_os(mut self, os: impl Into<String>) -> Self {
        self.os = os.into();
        self
    }

    /// Overrides the CPU architecture.
    #[cfg_attr(not(test), allow(dead_code))] // Overrides are exercised by tests and reserved for SDK wrappers.
    pub(crate) fn with_arch(mut self, arch: impl Into<String>) -> Self {
        self.arch = arch.into();
        self
    }

    /// Overrides the Rust compiler version.
    #[cfg_attr(not(test), allow(dead_code))] // Overrides are exercised by tests and reserved for SDK wrappers.
    pub(crate) fn with_rustc_version(mut self, version: impl Into<String>) -> Self {
        self.rustc_version = version.into();
        self
    }

    /// Sets the cross-SDK client feature flags.
    pub(crate) fn with_feature_flags(mut self, flags: UserAgentFeatureFlags) -> Self {
        self.feature_flags = flags;
        self
    }

    /// Adds a custom property; an existing property with the same key is
    /// replaced in place.
    pub(crate) fn with_property(mut self, property: UserAgentProperty) -> Self {
        match self
            .properties
            .iter_mut()
            .find(|p| p.key() == property.key())
        {
            Some(existing) => *existing = property,
            None => self.properties.push(property),
        }
        self
    }

    /// Adds several custom properties in order.
    pub(crate) fn with_properties(
        self,
        properties: impl IntoIterator<Item = UserAgentProperty>,
    ) -> Self {
        properties
            .into_iter()
            .fold(self, |builder, property| builder.with_property(property))
    }

    /// Sets the suffix appended after the metadata segment.
    ///
    /// The suffix is never altered or truncated, so it must already be
    /// validated (see [`UserAgentSuffix`](crate::options::UserAgentSuffix),
    /// [`WorkloadId`](crate::options::WorkloadId), and
    /// [`CorrelationId`](crate::options::CorrelationId)). Empty values are
    /// treated as unset.
    pub(crate) fn with_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Renders the [`UserAgent`].
    ///
    /// The returned string never exceeds [`MAX_USER_AGENT_LENGTH`] (given a
    /// suffix from a validated source type) and always contains exactly one
    /// balanced `(...)` metadata segment.
    pub(crate) fn build(self) -> UserAgent {
        let wrapping = self
            .wrapping_sdk_identifier
            .as_deref()
            .and_then(normalize_wrapping_sdk_identifier);
        let suffix = self.suffix.filter(|s| !s.is_empty());

        // Required metadata: never truncated or dropped. Built-in values may
        // be overridden, so keep them from breaking the segment.
        let positional = |value: &str| {
            let value = sanitize_token(value.trim());
            if value.is_empty() {
                "unknown".to_owned()
            } else {
                value
            }
        };
        let driver_version = positional(&self.driver_version);
        let mut required = format!(
            "{DRIVER_VERSION_KEY}={driver_version}; {}; {}; {}",
            positional(&self.os),
            positional(&self.arch),
            positional(&self.rustc_version),
        );
        if !self.feature_flags.is_empty() {
            required.push_str("; ");
            required.push_str(&self.feature_flags.to_string());
        }
        // Parentheses plus the separating space before them.
        let required_len = required.len() + 3;

        let suffix_wanted = suffix.as_ref().map_or(0, |s| 1 + s.len());

        // Only a wrapping identifier is ever shortened; the driver's own
        // product token is short and fixed. The suffix is always kept whole,
        // so it is budgeted before the wrapping identifier.
        let mut product = match wrapping {
            Some(mut w) => {
                w.truncate(MAX_USER_AGENT_LENGTH.saturating_sub(required_len + suffix_wanted));
                w
            }
            None => String::new(),
        };
        if product.is_empty() {
            product = format!("{DRIVER_PRODUCT_NAME}/{driver_version}");
        }

        // Custom properties only get what remains after the product, the
        // required metadata, and the suffix.
        let mut property_budget =
            MAX_USER_AGENT_LENGTH.saturating_sub(product.len() + required_len + suffix_wanted);

        let mut metadata = required;
        for property in &self.properties {
            let cost = 2 + property.key().len() + 1 + property.value().len();
            if cost > property_budget {
                break;
            }
            property_budget -= cost;
            metadata.push_str("; ");
            metadata.push_str(property.key());
            metadata.push('=');
            metadata.push_str(property.value());
        }

        let mut full_user_agent = product;
        full_user_agent.push_str(" (");
        full_user_agent.push_str(&metadata);
        full_user_agent.push(')');
        if let Some(s) = &suffix {
            full_user_agent.push(' ');
            full_user_agent.push_str(s);
        }

        UserAgent {
            full_user_agent,
            suffix,
        }
    }
}

impl fmt::Display for UserAgent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_user_agent)
    }
}

/// Encodes `value` as big-endian bytes without leading zeros, using unpadded
/// URL-safe base64. Zero encodes to an empty string.
fn encode_base64url(value: u32) -> String {
    let bytes = value.to_be_bytes();
    let start = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len());
    URL_SAFE_NO_PAD.encode(&bytes[start..])
}

/// Replaces non-ASCII and control characters with underscores.
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

/// Replaces every character that could break parsing of the user agent
/// (non-ASCII, control, whitespace, parentheses, and `;`) with an underscore.
fn sanitize_token(input: &str) -> String {
    strip_non_ascii(input)
        .chars()
        .map(|c| {
            if c.is_whitespace() || matches!(c, '(' | ')' | ';') {
                '_'
            } else {
                c
            }
        })
        .collect()
}

/// Normalizes a wrapping-SDK identifier the same way [`UserAgent`] would when
/// rendering the product token: trims surrounding whitespace, sanitizes
/// characters that would break parsing, and returns `None` for empty /
/// whitespace-only input.
///
/// Used at builder set-time so a runtime accessor like
/// `CosmosDriverRuntime::wrapping_sdk_identifier()` returns the same value
/// that ultimately appears in the `User-Agent` header.
pub(crate) fn normalize_wrapping_sdk_identifier(value: &str) -> Option<String> {
    // Trim before sanitizing so a whitespace-only input collapses to `None`
    // instead of a string of underscores.
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| sanitize_token(trimmed))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options::{CorrelationId, UserAgentSuffix, WorkloadId};

    /// A builder with every built-in value pinned, so output is deterministic.
    fn pinned() -> UserAgentBuilder {
        UserAgent::builder()
            .with_driver_version("1.0.0")
            .with_os("linux")
            .with_arch("x86_64")
            .with_rustc_version("1.98.1")
    }

    fn property(key: &str, value: &str) -> UserAgentProperty {
        UserAgentProperty::try_new(key, value).unwrap()
    }

    /// Asserts the structural invariants every user agent must satisfy.
    fn assert_well_formed(ua: &UserAgent) {
        let s = ua.as_str();
        assert!(s.is_ascii(), "non-ascii: {s}");
        assert!(
            s.len() <= MAX_USER_AGENT_LENGTH,
            "too long ({}): {s}",
            s.len()
        );
        let open = s.find('(').unwrap();
        let close = s.find(')').unwrap();
        assert!(open < close, "bad parens: {s}");
        assert_eq!(s.matches('(').count(), 1, "bad parens: {s}");
        assert_eq!(s.matches(')').count(), 1, "bad parens: {s}");
        assert_eq!(&s[open - 1..open], " ", "missing space before '(': {s}");
        assert!(s[open + 1..close].starts_with("drv="), "missing drv: {s}");
    }

    #[test]
    fn example_driver_used_directly() {
        assert_eq!(
            pinned().build().as_str(),
            "azsdk-rust-cosmos-driver/1.0.0 (drv=1.0.0; linux; x86_64; 1.98.1)"
        );
        // HTTP/2 + PPCB (0x12) with an operator suffix.
        let ua = pinned()
            .with_feature_flags(
                UserAgentFeatureFlags::HTTP2 | UserAgentFeatureFlags::PER_PARTITION_CIRCUIT_BREAKER,
            )
            .with_suffix("myapp-westus2")
            .build();
        assert_eq!(
            ua.as_str(),
            "azsdk-rust-cosmos-driver/1.0.0 (drv=1.0.0; linux; x86_64; 1.98.1; ft=Eg) myapp-westus2"
        );
        assert_eq!(ua.suffix(), Some("myapp-westus2"));
    }

    #[test]
    fn example_rust_cosmos_sdk_wrapper() {
        let ua = pinned()
            .with_wrapping_sdk_identifier("azsdk-rust-cosmos/1.0.0")
            .with_feature_flags(UserAgentFeatureFlags::HTTP2)
            .with_suffix("w25")
            .build();
        assert_eq!(
            ua.as_str(),
            "azsdk-rust-cosmos/1.0.0 (drv=1.0.0; linux; x86_64; 1.98.1; ft=EA) w25"
        );
    }

    #[test]
    fn example_non_rust_wrapper_with_properties() {
        let builder = pinned()
            .with_wrapping_sdk_identifier("azsdk-dotnet-cosmos/3.40.0")
            .with_property(property("rt", ".NET 8.0.1"))
            .with_property(property("host", "aks-prod"));
        assert_eq!(
            builder.clone().build().as_str(),
            "azsdk-dotnet-cosmos/3.40.0 (drv=1.0.0; linux; x86_64; 1.98.1; rt=.NET 8.0.1; host=aks-prod)"
        );
        let ua = builder
            .with_feature_flags(UserAgentFeatureFlags::PER_PARTITION_CIRCUIT_BREAKER)
            .with_suffix("myapp-westus2")
            .build();
        assert_eq!(
            ua.as_str(),
            "azsdk-dotnet-cosmos/3.40.0 (drv=1.0.0; linux; x86_64; 1.98.1; ft=Ag; rt=.NET 8.0.1; host=aks-prod) myapp-westus2"
        );
    }

    #[test]
    fn example_overridden_built_ins() {
        let ua = UserAgent::builder()
            .with_wrapping_sdk_identifier("azsdk-go-cosmos/0.5.0")
            .with_driver_version("2.1.0")
            .with_os("macos")
            .with_arch("aarch64")
            .with_rustc_version("1.99.0-nightly")
            .build();
        assert_eq!(
            ua.as_str(),
            "azsdk-go-cosmos/0.5.0 (drv=2.1.0; macos; aarch64; 1.99.0-nightly)"
        );
    }

    #[test]
    fn example_sanitized_inputs() {
        // Delimiters in the wrapping identifier or overridden built-ins are
        // replaced so the first `)` still ends the SDK-provided portion.
        let ua = pinned()
            .with_wrapping_sdk_identifier("azsdk-py-cosmos/1.0 (x; y)")
            .with_os("Windows 11; (x)")
            .with_arch("")
            .build();
        assert_eq!(
            ua.as_str(),
            "azsdk-py-cosmos/1.0__x__y_ (drv=1.0.0; Windows_11___x_; unknown; 1.98.1)"
        );
        assert_well_formed(&ua);
    }

    #[test]
    fn example_properties_dropped_before_suffix() {
        let big = "v".repeat(UserAgentProperty::MAX_VALUE_LENGTH);
        let ua = pinned()
            .with_wrapping_sdk_identifier("azsdk-x/1")
            .with_suffix("myapp")
            .with_properties((0..10).map(|i| property(&format!("k{i}"), &big)))
            .build();
        assert_well_formed(&ua);
        assert_eq!(ua.as_str().len(), 235);
        assert!(ua.as_str().ends_with(&format!("; k4={big}) myapp")));
        assert!(!ua.as_str().contains("; k5="));
    }

    #[test]
    fn default_uses_built_in_values() {
        let expected = format!(
            "azsdk-rust-cosmos-driver/{DRIVER_VERSION} (drv={DRIVER_VERSION}; {}; {}; {RUSTC_VERSION})",
            std::env::consts::OS,
            std::env::consts::ARCH,
        );
        assert_eq!(UserAgent::default().as_str(), expected);
        assert_ne!(RUSTC_VERSION, "unknown");
    }

    #[test]
    fn suffix_sources_follow_closing_paren() {
        let suffix = UserAgentSuffix::try_from("myapp-westus2").unwrap();
        let cid = CorrelationId::new("aks-prod-eastus");
        for value in [
            suffix.as_str().to_owned(),
            format!("w{}", WorkloadId::new(25).value()),
            cid.as_str().to_owned(),
        ] {
            let ua = pinned().with_suffix(value.clone()).build();
            assert!(ua.as_str().ends_with(&format!(") {value}")));
            assert_eq!(ua.suffix(), Some(value.as_str()));
        }
    }

    #[test]
    fn empty_inputs_are_treated_as_unset() {
        for raw in ["", "   ", "\t\n"] {
            let ua = pinned().with_wrapping_sdk_identifier(raw).build();
            assert_eq!(ua, pinned().build());
        }
        let ua = pinned().with_suffix("").build();
        assert!(ua.suffix().is_none());
        assert!(ua.as_str().ends_with(')'));
    }

    #[test]
    fn with_property_replaces_by_key_in_place() {
        let ua = pinned()
            .with_property(property("a", "1"))
            .with_property(property("b", "2"))
            .with_property(property("a", "3"))
            .build();
        assert!(ua.as_str().ends_with("; a=3; b=2)"), "{ua}");
    }

    #[test]
    fn suffix_is_not_altered() {
        // The suffix is opaque; even delimiter characters pass through, and the
        // first `)` still ends the SDK-provided portion.
        let ua = pinned().with_suffix("a) b; c").build();
        assert!(ua.as_str().ends_with(") a) b; c"));
        assert_eq!(ua.suffix(), Some("a) b; c"));
    }

    #[test]
    fn pathological_wrapping_identifier_keeps_suffix_and_structure() {
        let suffix = "a".repeat(UserAgentSuffix::MAX_LENGTH);
        let ua = pinned()
            .with_wrapping_sdk_identifier(format!("azsdk-rust-{}", "x".repeat(500)))
            .with_feature_flags(UserAgentFeatureFlags::HTTP2)
            .with_property(property("dotnet", "8.0.1"))
            .with_suffix(suffix.clone())
            .build();
        assert_well_formed(&ua);
        assert_eq!(ua.suffix(), Some(suffix.as_str()));
        assert!(ua.as_str().contains("; ft=EA"));
    }

    #[test]
    fn truncation_never_breaks_parentheses() {
        let max_value = "v".repeat(UserAgentProperty::MAX_VALUE_LENGTH);
        let props: Vec<_> = (0..40)
            .map(|i| property(&format!("key{i}"), &max_value))
            .collect();
        let suffix = "s".repeat(UserAgentSuffix::MAX_LENGTH);
        for wrap_len in (0..400).step_by(7) {
            for prop_count in [0, 1, 3, 40] {
                for flags in [UserAgentFeatureFlags::NONE, UserAgentFeatureFlags::HTTP2] {
                    let ua = pinned()
                        .with_wrapping_sdk_identifier(format!("azsdk-{}", "w".repeat(wrap_len)))
                        .with_feature_flags(flags)
                        .with_properties(props[..prop_count].iter().cloned())
                        .with_suffix(suffix.clone())
                        .build();
                    assert_well_formed(&ua);
                    assert_eq!(ua.suffix(), Some(suffix.as_str()));
                }
            }
        }
    }

    #[test]
    fn feature_flags_encoding() {
        assert_eq!(UserAgentFeatureFlags::NONE.to_string(), "");
        for (flags, expected) in [
            (
                UserAgentFeatureFlags::PER_PARTITION_AUTOMATIC_FAILOVER,
                "ft=AQ",
            ),
            (
                UserAgentFeatureFlags::PER_PARTITION_CIRCUIT_BREAKER,
                "ft=Ag",
            ),
            (UserAgentFeatureFlags::THIN_CLIENT, "ft=BA"),
            (UserAgentFeatureFlags::BINARY_ENCODING, "ft=CA"),
            (UserAgentFeatureFlags::HTTP2, "ft=EA"),
            (
                UserAgentFeatureFlags::PER_PARTITION_AUTOMATIC_FAILOVER
                    | UserAgentFeatureFlags::PER_PARTITION_CIRCUIT_BREAKER,
                "ft=Aw",
            ),
            (
                UserAgentFeatureFlags::PER_PARTITION_CIRCUIT_BREAKER | UserAgentFeatureFlags::HTTP2,
                "ft=Eg",
            ),
        ] {
            assert_eq!(flags.to_string(), expected);
            let decoded = URL_SAFE_NO_PAD.decode(&expected[3..]).unwrap();
            assert_eq!(decoded, [flags.bits() as u8]);
        }
    }

    #[test]
    fn base64url_encoding_is_standard_base64_of_minimal_bytes() {
        assert_eq!(encode_base64url(0), "");
        assert_eq!(encode_base64url(0xFF), "_w");
        assert_eq!(encode_base64url(0x100), "AQA");
        assert_eq!(encode_base64url(0x01_00_00), "AQAA");
        for value in [
            1u32,
            0x12,
            0x7F,
            0xFF,
            0x100,
            0xFFFF,
            0x10000,
            0xFF_FFFF,
            0x100_0000,
            u32::MAX,
        ] {
            let expected: Vec<u8> = value
                .to_be_bytes()
                .iter()
                .copied()
                .skip_while(|&b| b == 0)
                .collect();
            assert_eq!(
                URL_SAFE_NO_PAD.decode(encode_base64url(value)).unwrap(),
                expected
            );
        }
    }

    #[test]
    fn feature_flags_bitwise_helpers() {
        let mut flags = UserAgentFeatureFlags::NONE;
        assert!(flags.is_empty());
        flags |= UserAgentFeatureFlags::HTTP2;
        flags |= UserAgentFeatureFlags::PER_PARTITION_CIRCUIT_BREAKER;
        assert!(!flags.is_empty());
        assert_eq!(flags.bits(), 0x12);
        assert_eq!(
            UserAgentFeatureFlags::HTTP2
                .union(UserAgentFeatureFlags::PER_PARTITION_CIRCUIT_BREAKER),
            flags
        );
        assert_eq!(
            flags & UserAgentFeatureFlags::HTTP2,
            UserAgentFeatureFlags::HTTP2
        );
        assert_eq!(
            flags & UserAgentFeatureFlags::PER_PARTITION_AUTOMATIC_FAILOVER,
            UserAgentFeatureFlags::NONE
        );
        let mut masked = flags;
        masked &= UserAgentFeatureFlags::HTTP2;
        assert_eq!(masked, UserAgentFeatureFlags::HTTP2);
    }

    #[test]
    fn feature_flags_from_client_config() {
        assert_eq!(
            UserAgentFeatureFlags::from_client_config(false, false),
            UserAgentFeatureFlags::NONE
        );
        assert_eq!(
            UserAgentFeatureFlags::from_client_config(true, false),
            UserAgentFeatureFlags::HTTP2
        );
        assert_eq!(
            UserAgentFeatureFlags::from_client_config(false, true),
            UserAgentFeatureFlags::PER_PARTITION_CIRCUIT_BREAKER
        );
        assert_eq!(
            UserAgentFeatureFlags::from_client_config(true, true),
            UserAgentFeatureFlags::HTTP2 | UserAgentFeatureFlags::PER_PARTITION_CIRCUIT_BREAKER
        );
    }
}
