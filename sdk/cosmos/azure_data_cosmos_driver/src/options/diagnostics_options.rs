// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Configuration options for diagnostics output.

use super::env_parsing::{parse_from_env, ValidationBounds};
use std::sync::{Arc, OnceLock};

/// Default maximum size for summary-mode diagnostic output (8 KB).
const DEFAULT_MAX_SUMMARY_SIZE_BYTES: usize = 8 * 1024;

/// Minimum allowed size for summary-mode diagnostic output (4 KB).
const MIN_MAX_SUMMARY_SIZE_BYTES: usize = 4 * 1024;

/// Controls the verbosity level of diagnostic output.
///
/// Diagnostics can be rendered at different levels of detail depending on debugging needs versus
/// log-size constraints. Pass a value to
/// [`DiagnosticsContext::to_json_string`](crate::diagnostics::DiagnosticsContext::to_json_string).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum DiagnosticsVerbosity {
    /// Use the default verbosity configured on the [`DiagnosticsOptions`].
    #[default]
    Default,

    /// Minimal output optimized for log-size limits: operation-level roll-up only, no per-request
    /// detail.
    Summary,

    /// Full output including every individual [`RequestDiagnostics`](crate::diagnostics::RequestDiagnostics).
    Detailed,
}

impl DiagnosticsVerbosity {
    /// Returns the string representation of this verbosity level.
    pub fn as_str(&self) -> &'static str {
        match self {
            DiagnosticsVerbosity::Default => "default",
            DiagnosticsVerbosity::Summary => "summary",
            DiagnosticsVerbosity::Detailed => "detailed",
        }
    }
}

impl AsRef<str> for DiagnosticsVerbosity {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DiagnosticsVerbosity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
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
                "unknown diagnostics verbosity: '{s}'; expected 'default', 'summary', or 'detailed'"
            )),
        }
    }
}

/// Configuration options for diagnostics output.
///
/// Controls the default verbosity and the size budget applied when rendering a
/// [`DiagnosticsContext`](crate::diagnostics::DiagnosticsContext) to a string. Use
/// [`DiagnosticsOptions::builder`] to construct instances.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiagnosticsOptions {
    /// Maximum size in bytes for summary-mode diagnostic output.
    pub(crate) max_summary_size_bytes: usize,

    /// Default verbosity used when [`DiagnosticsVerbosity::Default`] is requested.
    pub(crate) default_verbosity: DiagnosticsVerbosity,
}

impl Default for DiagnosticsOptions {
    fn default() -> Self {
        DiagnosticsOptionsBuilder::new()
            .build()
            .expect("default DiagnosticsOptions should always be valid")
    }
}

impl DiagnosticsOptions {
    /// Creates a new builder for [`DiagnosticsOptions`].
    pub fn builder() -> DiagnosticsOptionsBuilder {
        DiagnosticsOptionsBuilder::new()
    }

    /// Returns the maximum size in bytes for summary-mode output.
    pub fn max_summary_size_bytes(&self) -> usize {
        self.max_summary_size_bytes
    }

    /// Returns the default verbosity level.
    pub fn default_verbosity(&self) -> DiagnosticsVerbosity {
        self.default_verbosity
    }

    /// Returns a process-wide shared default, avoiding repeated environment reads on the hot path.
    pub(crate) fn shared_default() -> Arc<DiagnosticsOptions> {
        static DEFAULT: OnceLock<Arc<DiagnosticsOptions>> = OnceLock::new();
        DEFAULT
            .get_or_init(|| Arc::new(DiagnosticsOptions::default()))
            .clone()
    }
}

/// Builder for [`DiagnosticsOptions`].
///
/// Unset values fall back to environment variables and then to sensible defaults.
///
/// # Environment Variables
///
/// - `AZURE_COSMOS_DIAGNOSTICS_MAX_SUMMARY_SIZE_BYTES`: summary-mode size budget in bytes
///   (default: `8192`, min: `4096`).
/// - `AZURE_COSMOS_DIAGNOSTICS_DEFAULT_VERBOSITY`: default verbosity — one of `default`, `summary`,
///   `detailed` (default: `detailed`).
///
/// # Example
///
/// ```
/// use azure_data_cosmos_driver::options::{DiagnosticsOptions, DiagnosticsVerbosity};
///
/// let options = DiagnosticsOptions::builder()
///     .with_max_summary_size_bytes(16 * 1024)
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

    /// Sets the summary-mode size budget in bytes (must be at least 4096).
    pub fn with_max_summary_size_bytes(mut self, size: usize) -> Self {
        self.max_summary_size_bytes = Some(size);
        self
    }

    /// Sets the default verbosity level.
    pub fn with_default_verbosity(mut self, verbosity: DiagnosticsVerbosity) -> Self {
        self.default_verbosity = Some(verbosity);
        self
    }

    /// Builds the [`DiagnosticsOptions`], filling unset values from the environment or defaults.
    ///
    /// # Errors
    ///
    /// Returns an error if `max_summary_size_bytes` is below the minimum, or if an environment
    /// variable fails to parse.
    pub fn build(self) -> azure_core::Result<DiagnosticsOptions> {
        let max_summary_size_bytes = parse_from_env(
            self.max_summary_size_bytes,
            "AZURE_COSMOS_DIAGNOSTICS_MAX_SUMMARY_SIZE_BYTES",
            DEFAULT_MAX_SUMMARY_SIZE_BYTES,
            ValidationBounds::range(MIN_MAX_SUMMARY_SIZE_BYTES, usize::MAX),
        )?;

        let default_verbosity = match self.default_verbosity {
            Some(v) => v,
            None => match std::env::var("AZURE_COSMOS_DIAGNOSTICS_DEFAULT_VERBOSITY") {
                Ok(v) => v.parse().map_err(|e: String| {
                    azure_core::Error::with_message(
                        azure_core::error::ErrorKind::DataConversion,
                        format!("Failed to parse AZURE_COSMOS_DIAGNOSTICS_DEFAULT_VERBOSITY: {e}"),
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
        assert_eq!(options.max_summary_size_bytes(), 8 * 1024);
        assert_eq!(options.default_verbosity(), DiagnosticsVerbosity::Detailed);
    }

    #[test]
    fn custom_values() {
        let options = DiagnosticsOptions::builder()
            .with_max_summary_size_bytes(16 * 1024)
            .with_default_verbosity(DiagnosticsVerbosity::Summary)
            .build()
            .unwrap();

        assert_eq!(options.max_summary_size_bytes(), 16 * 1024);
        assert_eq!(options.default_verbosity(), DiagnosticsVerbosity::Summary);
    }

    #[test]
    fn max_summary_size_too_small_is_rejected() {
        let result = DiagnosticsOptions::builder()
            .with_max_summary_size_bytes(2 * 1024)
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
            "minimal".parse::<DiagnosticsVerbosity>().unwrap(),
            DiagnosticsVerbosity::Summary
        );
        assert_eq!(
            "VERBOSE".parse::<DiagnosticsVerbosity>().unwrap(),
            DiagnosticsVerbosity::Detailed
        );
        assert!("nonsense".parse::<DiagnosticsVerbosity>().is_err());
    }

    #[test]
    fn shared_default_is_stable() {
        let a = DiagnosticsOptions::shared_default();
        let b = DiagnosticsOptions::shared_default();
        assert!(Arc::ptr_eq(&a, &b));
    }
}
