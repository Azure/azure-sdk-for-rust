// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-level tuning for cross-region hedging.

/// Driver-wide limits on cross-region hedging.
///
/// Whether an individual operation hedges at all is decided per operation by
/// [`AvailabilityStrategy`]. These options are the driver-wide ceiling that sits
/// above that decision: they cap how many metadata operations may make simultaneous
/// cross-region attempts, no matter how many operations ask for hedging.
///
/// Read **once** when the driver is constructed; later mutation has no effect.
///
/// # Example
///
/// ```rust
/// use azure_data_cosmos_driver::models::AccountReference;
/// use azure_data_cosmos_driver::options::{DriverOptions, HedgingOptions};
///
/// let hedging = HedgingOptions::builder()
///     .with_max_concurrent_metadata_attempts(64)
///     .build();
///
/// let account = AccountReference::with_master_key(
///     "https://my-account.documents.azure.com/".parse()?,
///     "my-key",
/// );
/// let options = DriverOptions::builder(account)
///     .with_hedging_options(hedging)
///     .build();
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// [`AvailabilityStrategy`]: crate::options::AvailabilityStrategy
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct HedgingOptions {
    max_concurrent_metadata_attempts: usize,
}

impl Default for HedgingOptions {
    fn default() -> Self {
        Self {
            max_concurrent_metadata_attempts: DEFAULT_MAX_CONCURRENT_METADATA_ATTEMPTS,
        }
    }
}

/// Default ceiling on metadata operations making simultaneous cross-region attempts.
///
/// Metadata reads are cache misses, so the steady-state rate is low and a burst
/// is nearly always a cold start or a cache-wide invalidation. 32 is high enough
/// that those bursts still hedge, and low enough that a pathological storm cannot
/// double the driver's whole metadata load against the service.
pub const DEFAULT_MAX_CONCURRENT_METADATA_ATTEMPTS: usize = 32;

impl HedgingOptions {
    /// Creates a builder for [`HedgingOptions`].
    pub fn builder() -> HedgingOptionsBuilder {
        HedgingOptionsBuilder::new()
    }

    /// Returns the maximum number of metadata operations that may make
    /// simultaneous cross-region attempts at one time.
    ///
    /// `0` disables metadata hedging entirely. See
    /// [`HedgingOptionsBuilder::with_max_concurrent_metadata_attempts`] for what
    /// the limit does and does not cover.
    pub fn max_concurrent_metadata_attempts(&self) -> usize {
        self.max_concurrent_metadata_attempts
    }
}

/// Builder for [`HedgingOptions`].
///
/// # Example
///
/// ```rust
/// use azure_data_cosmos_driver::options::HedgingOptions;
///
/// // Turn metadata hedging off without touching the per-operation strategy.
/// let options = HedgingOptions::builder()
///     .with_max_concurrent_metadata_attempts(0)
///     .build();
/// ```
#[non_exhaustive]
#[derive(Clone, Debug, Default)]
pub struct HedgingOptionsBuilder {
    max_concurrent_metadata_attempts: Option<usize>,
}

impl HedgingOptionsBuilder {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum number of metadata operations that may make
    /// simultaneous cross-region attempts at one time.
    ///
    /// Default: [`DEFAULT_MAX_CONCURRENT_METADATA_ATTEMPTS`]. `0` disables
    /// metadata hedging entirely, which is the supported way to turn the
    /// feature off driver-wide.
    ///
    /// The limit counts metadata operations admitted to make concurrent regional
    /// attempts, not individual request legs. An admitted operation may issue a
    /// second request; one that is refused simply proceeds as an ordinary
    /// single-region request, so reaching the limit degrades latency back to the
    /// non-hedged baseline rather than failing anything.
    ///
    /// This governs **metadata** reads only — container and partition-key-range
    /// lookups. Data-plane hedging is not yet budgeted; see
    /// <https://github.com/Azure/azure-sdk-for-rust/issues/4916>.
    pub fn with_max_concurrent_metadata_attempts(mut self, value: usize) -> Self {
        self.max_concurrent_metadata_attempts = Some(value);
        self
    }

    /// Builds the [`HedgingOptions`], filling unset values with their defaults.
    pub fn build(self) -> HedgingOptions {
        let defaults = HedgingOptions::default();
        HedgingOptions {
            max_concurrent_metadata_attempts: self
                .max_concurrent_metadata_attempts
                .unwrap_or(defaults.max_concurrent_metadata_attempts),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_uses_the_documented_metadata_limit() {
        assert_eq!(
            HedgingOptions::default().max_concurrent_metadata_attempts(),
            DEFAULT_MAX_CONCURRENT_METADATA_ATTEMPTS
        );
    }

    #[test]
    fn builder_without_overrides_matches_default() {
        assert_eq!(
            HedgingOptions::builder()
                .build()
                .max_concurrent_metadata_attempts(),
            HedgingOptions::default().max_concurrent_metadata_attempts()
        );
    }

    #[test]
    fn builder_override_is_honored() {
        let options = HedgingOptions::builder()
            .with_max_concurrent_metadata_attempts(7)
            .build();
        assert_eq!(options.max_concurrent_metadata_attempts(), 7);
    }

    #[test]
    fn zero_is_a_valid_disable_value() {
        let options = HedgingOptions::builder()
            .with_max_concurrent_metadata_attempts(0)
            .build();
        assert_eq!(options.max_concurrent_metadata_attempts(), 0);
    }
}
