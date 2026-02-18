// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Configuration options for diagnostics and telemetry.

use super::env_parsing::{parse_from_env, ValidationBounds};

/// Default maximum size for diagnostic summary output (8 KB).
const DEFAULT_MAX_SUMMARY_SIZE_BYTES: usize = 8 * 1024;

/// Minimum allowed size for diagnostic summary output (4 KB).
const MIN_MAX_SUMMARY_SIZE_BYTES: usize = 4 * 1024;

/// Controls the verbosity level of diagnostic output.
///
/// Diagnostics can be output in different levels of detail depending on
/// debugging needs vs. log size constraints.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum DiagnosticsVerbosity {
    /// Use the default verbosity level configured in the runtime.
    #[default]
    Default,

    /// Minimal diagnostic output optimized for log size limits.
    ///
    /// Summary mode:
    /// - Groups requests by region
    /// - Keeps first and last request per region in full detail
    /// - Deduplicates middle requests by (endpoint, status, sub_status, execution_context)
    /// - Shows count and duration statistics (min/max/P50) for deduplicated groups
    /// - Respects `max_summary_size_bytes` limit
    Summary,

    /// Full diagnostic output with all request details.
    ///
    /// Detailed mode includes:
    /// - All individual request diagnostics
    /// - Pipeline stage events with timing
    /// - No deduplication or truncation
    Detailed,
}

impl std::fmt::Display for DiagnosticsVerbosity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiagnosticsVerbosity::Default => write!(f, "default"),
            DiagnosticsVerbosity::Summary => write!(f, "summary"),
            DiagnosticsVerbosity::Detailed => write!(f, "detailed"),
        }
    }
}

impl std::str::FromStr for DiagnosticsVerbosity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "default" => Ok(DiagnosticsVerbosity::Default),
            "summary" | "minimal" => Ok(DiagnosticsVerbosity::Summary),
            "detailed" | "verbose" => Ok(DiagnosticsVerbosity::Detailed),
            _ => Err(format!(
                "Unknown diagnostics verbosity: '{}'. Expected 'default', 'summary', or 'detailed'",
                s
            )),
        }
    }
}

/// Configuration options for diagnostics output.
///
/// Controls how diagnostic information is formatted and truncated.
/// Use [`DiagnosticsOptionsBuilder`] to construct instances.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiagnosticsOptions {
    /// Maximum size in bytes for summary mode diagnostic output.
    ///
    /// When `to_json_string` is called with `Summary` verbosity, the output
    /// will be truncated to fit within this limit. Default is 8 KB.
    pub(crate) max_summary_size_bytes: usize,

    /// Default verbosity level when none is specified.
    ///
    /// Used when `to_json_string` is called with `None` for verbosity.
    pub(crate) default_verbosity: DiagnosticsVerbosity,
}

impl Default for DiagnosticsOptions {
    fn default() -> Self {
        DiagnosticsOptionsBuilder::new()
            .build()
            .expect("Default DiagnosticsOptions should always be valid")
    }
}

impl DiagnosticsOptions {
    /// Creates a new builder for `DiagnosticsOptions`.
    pub fn builder() -> DiagnosticsOptionsBuilder {
        DiagnosticsOptionsBuilder::new()
    }

    /// Returns the maximum size in bytes for summary mode output.
    pub fn max_summary_size_bytes(&self) -> usize {
        self.max_summary_size_bytes
    }

    /// Returns the default verbosity level.
    pub fn default_verbosity(&self) -> DiagnosticsVerbosity {
        self.default_verbosity
    }
}

/// Builder for [`DiagnosticsOptions`].
///
/// Default values are read from environment variables when available,
/// and can be overridden using builder methods.
///
/// # Environment Variables
///
/// - `AZURE_COSMOS_DIAGNOSTICS_MAX_SUMMARY_SIZE_BYTES`: Maximum size in bytes for
///   summary mode output (default: `8192`, min: `4096`)
/// - `AZURE_COSMOS_DIAGNOSTICS_DEFAULT_VERBOSITY`: Default verbosity level.
///   Valid values: `default`, `summary`, `detailed` (default: `detailed`)
///
/// # Example
///
/// ```rust
/// use azure_data_cosmos_driver::options::{DiagnosticsOptions, DiagnosticsVerbosity};
///
/// let options = DiagnosticsOptions::builder()
///     .with_max_summary_size_bytes(16 * 1024)  // 16 KB
///     .with_default_verbosity(DiagnosticsVerbosity::Summary)
///     .build()
///     .expect("valid options");
/// ```
#[non_exhaustive]
#[derive(Clone, Debug, Default)]
pub struct DiagnosticsOptionsBuilder {
    max_summary_size_bytes: Option<usize>,
    default_verbosity: Option<DiagnosticsVerbosity>,
}

impl DiagnosticsOptionsBuilder {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum size in bytes for summary mode diagnostic output.
    ///
    /// Must be at least 4096 bytes (4 KB).
    /// Default: 8192 bytes (8 KB).
    pub fn with_max_summary_size_bytes(mut self, size: usize) -> Self {
        self.max_summary_size_bytes = Some(size);
        self
    }

    /// Sets the default verbosity level.
    ///
    /// Default: `DiagnosticsVerbosity::Detailed`.
    pub fn with_default_verbosity(mut self, verbosity: DiagnosticsVerbosity) -> Self {
        self.default_verbosity = Some(verbosity);
        self
    }

    /// Builds the `DiagnosticsOptions` with configured values.
    ///
    /// Unset values are populated from environment variables or use sensible defaults.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `max_summary_size_bytes` is less than 4096
    /// - Environment variable parsing fails
    pub fn build(self) -> azure_core::Result<DiagnosticsOptions> {
        let max_summary_size_bytes = parse_from_env(
            self.max_summary_size_bytes,
            "AZURE_COSMOS_DIAGNOSTICS_MAX_SUMMARY_SIZE_BYTES",
            DEFAULT_MAX_SUMMARY_SIZE_BYTES,
            ValidationBounds::min(MIN_MAX_SUMMARY_SIZE_BYTES),
        )?;

        let default_verbosity = match self.default_verbosity {
            Some(v) => v,
            None => match std::env::var("AZURE_COSMOS_DIAGNOSTICS_DEFAULT_VERBOSITY") {
                Ok(v) => v.parse().map_err(|e: String| {
                    azure_core::Error::with_message(
                        azure_core::error::ErrorKind::DataConversion,
                        format!(
                            "Failed to parse AZURE_COSMOS_DIAGNOSTICS_DEFAULT_VERBOSITY: {}",
                            e
                        ),
                    )
                })?,
                Err(_) => DiagnosticsVerbosity::Detailed,
            },
        };

        Ok(DiagnosticsOptions {
            max_summary_size_bytes,
            default_verbosity,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        let options = DiagnosticsOptions::default();
        assert_eq!(options.max_summary_size_bytes, 8 * 1024);
        assert_eq!(options.default_verbosity, DiagnosticsVerbosity::Detailed);
    }

    #[test]
    fn custom_values() {
        let options = DiagnosticsOptionsBuilder::new()
            .with_max_summary_size_bytes(16 * 1024)
            .with_default_verbosity(DiagnosticsVerbosity::Summary)
            .build()
            .unwrap();

        assert_eq!(options.max_summary_size_bytes, 16 * 1024);
        assert_eq!(options.default_verbosity, DiagnosticsVerbosity::Summary);
    }

    #[test]
    fn max_summary_size_too_small() {
        let result = DiagnosticsOptionsBuilder::new()
            .with_max_summary_size_bytes(2 * 1024) // 2 KB, below minimum
            .build();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("must be at least 4096"));
    }

    #[test]
    fn verbosity_from_str() {
        assert_eq!(
            "default".parse::<DiagnosticsVerbosity>().unwrap(),
            DiagnosticsVerbosity::Default
        );
        assert_eq!(
            "summary".parse::<DiagnosticsVerbosity>().unwrap(),
            DiagnosticsVerbosity::Summary
        );
        assert_eq!(
            "minimal".parse::<DiagnosticsVerbosity>().unwrap(),
            DiagnosticsVerbosity::Summary
        );
        assert_eq!(
            "detailed".parse::<DiagnosticsVerbosity>().unwrap(),
            DiagnosticsVerbosity::Detailed
        );
        assert_eq!(
            "verbose".parse::<DiagnosticsVerbosity>().unwrap(),
            DiagnosticsVerbosity::Detailed
        );
        assert_eq!(
            "DETAILED".parse::<DiagnosticsVerbosity>().unwrap(),
            DiagnosticsVerbosity::Detailed
        );

        assert!("invalid".parse::<DiagnosticsVerbosity>().is_err());
    }

    #[test]
    fn verbosity_display() {
        assert_eq!(DiagnosticsVerbosity::Default.to_string(), "default");
        assert_eq!(DiagnosticsVerbosity::Summary.to_string(), "summary");
        assert_eq!(DiagnosticsVerbosity::Detailed.to_string(), "detailed");
    }
}
