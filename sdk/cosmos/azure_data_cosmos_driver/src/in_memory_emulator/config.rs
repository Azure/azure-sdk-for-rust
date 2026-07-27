// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore PRNG
//! Virtual account configuration for the in-memory emulator.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use url::Url;

use super::ru_model::RuChargingModel;

/// Configures the emulated Cosmos DB account.
///
/// # Runtime-mutable fields
///
/// Most fields are set at construction time and never change. The
/// per-partition-failover flag (PPAF, exposed in the synthesized account
/// JSON as `enablePerPartitionFailoverBehavior`) is intentionally backed by
/// an `Arc<AtomicBool>` so tests can flip it at runtime through a
/// shared-reference handle. Flipping it through `set_per_partition_failover`
/// changes what the emulator returns on the *next* account-read response,
/// which is what the driver's background refresh loop polls.
#[derive(Clone, Debug)]
pub struct VirtualAccountConfig {
    regions: Vec<VirtualRegion>,
    write_mode: WriteMode,
    consistency: ConsistencyLevel,
    replication: ReplicationConfig,
    replication_overrides: HashMap<(String, String), ReplicationConfig>,
    ru_model: RuChargingModel,
    throttling_enabled: bool,
    /// Server-side per-partition automatic failover flag -- emitted as
    /// `enablePerPartitionFailoverBehavior` in the account JSON. Atomic and
    /// shared so test code can toggle the value after the config has been
    /// moved into `EmulatorStore` and is reachable only by `&self`.
    enable_per_partition_failover: Arc<AtomicBool>,
}

impl VirtualAccountConfig {
    /// Creates a new configuration with the given regions.
    /// The first region is the hub/primary write region in single-write mode.
    pub fn new(mut regions: Vec<VirtualRegion>) -> crate::error::Result<Self> {
        if regions.is_empty() {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message("at least one region is required")
                .build());
        }
        // Auto-assign monotonically increasing region IDs by position for any
        // region that did not have one set explicitly via `with_region_id`.
        // Using `0` as the sentinel means callers that explicitly pass
        // `with_region_id(0)` to the *first* region get the same effective ID
        // they would have been auto-assigned anyway.
        for (idx, r) in regions.iter_mut().enumerate() {
            if r.region_id == 0 {
                r.region_id = idx as u64;
            }
        }
        Ok(Self {
            regions,
            write_mode: WriteMode::Single,
            consistency: ConsistencyLevel::Session,
            replication: ReplicationConfig::default(),
            replication_overrides: HashMap::new(),
            ru_model: RuChargingModel::default(),
            throttling_enabled: false,
            enable_per_partition_failover: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Sets the write mode.
    pub fn with_write_mode(mut self, mode: WriteMode) -> Self {
        self.write_mode = mode;
        self
    }

    /// Sets the default consistency level.
    pub fn with_consistency(mut self, level: ConsistencyLevel) -> Self {
        self.consistency = level;
        self
    }

    /// Sets the global replication config.
    pub fn with_replication_config(mut self, config: ReplicationConfig) -> Self {
        self.replication = config;
        self
    }

    /// Adds a per-direction replication override.
    ///
    /// Validates that both `source` and `target` match the name of a
    /// configured region (case-sensitive). Returns a `Client` error on
    /// either mismatch — silently dropping a typo in the region name (the
    /// previous behavior) made misuse hard to spot in tests.
    pub fn with_replication_override(
        mut self,
        source: &str,
        target: &str,
        config: ReplicationConfig,
    ) -> crate::error::Result<Self> {
        let known: Vec<&str> = self.regions.iter().map(|r| r.name.as_str()).collect();
        if !known.contains(&source) {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message(format!(
                    "replication override source region '{}' is not configured (known: {:?})",
                    source, known
                ))
                .build());
        }
        if !known.contains(&target) {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message(format!(
                    "replication override target region '{}' is not configured (known: {:?})",
                    target, known
                ))
                .build());
        }
        if source == target {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message("replication override source and target must be different regions")
                .build());
        }
        self.replication_overrides
            .insert((source.to_string(), target.to_string()), config);
        Ok(self)
    }

    /// Sets the RU charging model.
    pub fn with_ru_model(mut self, model: RuChargingModel) -> Self {
        self.ru_model = model;
        self
    }

    /// Enables or disables throughput throttling (429/3200).
    /// When enabled, containers with provisioned throughput will enforce RU/s limits.
    pub fn with_throttling_enabled(mut self, enabled: bool) -> Self {
        self.throttling_enabled = enabled;
        self
    }

    /// Returns whether throttling is enabled.
    pub fn throttling_enabled(&self) -> bool {
        self.throttling_enabled
    }

    /// Sets the initial value of the server-side per-partition automatic
    /// failover (PPAF) flag. This is the JSON field
    /// `enablePerPartitionFailoverBehavior` in the synthesized account-read
    /// response and the input the driver consumes to enable PPAF dynamically.
    ///
    /// Most tests should use this builder method for the initial state, and
    /// only fall back to [`Self::set_per_partition_failover`] when they need
    /// to flip the value at runtime to exercise the driver's dynamic
    /// enablement / disablement code path.
    pub fn with_per_partition_failover(self, enabled: bool) -> Self {
        self.enable_per_partition_failover
            .store(enabled, Ordering::SeqCst);
        self
    }

    /// Returns the current value of the server-side PPAF flag.
    pub fn per_partition_failover_enabled(&self) -> bool {
        self.enable_per_partition_failover.load(Ordering::SeqCst)
    }

    /// Flips the server-side PPAF flag at runtime. The next account-read
    /// response served by the emulator will include the new value, and the
    /// driver's background account-refresh loop will pick it up on its next
    /// tick.
    ///
    /// Takes `&self` so test code can call it through the shared
    /// `Arc<EmulatorStore>` handle without needing exclusive access.
    pub fn set_per_partition_failover(&self, enabled: bool) {
        self.enable_per_partition_failover
            .store(enabled, Ordering::SeqCst);
    }

    pub fn regions(&self) -> &[VirtualRegion] {
        &self.regions
    }

    pub fn write_mode(&self) -> WriteMode {
        self.write_mode
    }

    pub fn consistency(&self) -> ConsistencyLevel {
        self.consistency
    }

    pub fn replication(&self) -> &ReplicationConfig {
        &self.replication
    }

    pub fn ru_model(&self) -> &RuChargingModel {
        &self.ru_model
    }

    /// Returns the replication config for a specific source → target pair,
    /// falling back to the global default.
    pub fn replication_for(&self, source: &str, target: &str) -> &ReplicationConfig {
        self.replication_overrides
            .get(&(source.to_string(), target.to_string()))
            .unwrap_or(&self.replication)
    }

    /// Returns the write region name (first region in single-write mode).
    pub fn write_region_name(&self) -> &str {
        &self.regions[0].name
    }

    /// Returns whether a region is allowed to accept writes.
    pub fn is_write_region(&self, region_name: &str) -> bool {
        match self.write_mode {
            WriteMode::Multi => true,
            WriteMode::Single => self.regions[0].name == region_name,
        }
    }

    /// Finds the region name for a given gateway URL.
    ///
    /// Matches on (scheme, host, port) — not host alone — so two regions
    /// that share a hostname but differ in port (or scheme) route correctly.
    /// Useful when adding e.g. `https://localhost:8081` and
    /// `https://localhost:8082` regions for parity tests.
    pub fn region_for_url(&self, url: &Url) -> Option<&str> {
        let host = url.host_str()?;
        let scheme = url.scheme();
        let port = url.port_or_known_default();
        for r in &self.regions {
            let Some(rhost) = r.gateway_url.host_str() else {
                continue;
            };
            if !rhost.eq_ignore_ascii_case(host) {
                continue;
            }
            if r.gateway_url.scheme() != scheme {
                continue;
            }
            if r.gateway_url.port_or_known_default() != port {
                continue;
            }
            return Some(&r.name);
        }
        None
    }

    /// Finds the region ID for a given region name.
    pub fn region_id_for(&self, region_name: &str) -> u64 {
        self.regions
            .iter()
            .find(|r| r.name == region_name)
            .map(|r| r.region_id)
            .unwrap_or(0)
    }
}

/// A virtual region with a name and gateway URL.
#[derive(Clone, Debug)]
pub struct VirtualRegion {
    name: String,
    gateway_url: Url,
    region_id: u64,
}

impl VirtualRegion {
    /// Creates a new region. The `region_id` is auto-assigned monotonically
    /// (0, 1, 2, …) based on position in the regions list when constructing
    /// `VirtualAccountConfig`. To pin an explicit region ID, chain
    /// [`Self::with_region_id`] before passing the region into the config.
    pub fn new(name: &str, gateway_url: Url) -> Self {
        Self {
            name: name.to_string(),
            gateway_url,
            region_id: 0,
        }
    }

    /// Creates a new region with an explicit region ID.
    pub fn with_region_id(mut self, id: u64) -> Self {
        self.region_id = id;
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn gateway_url(&self) -> &Url {
        &self.gateway_url
    }

    pub fn region_id(&self) -> u64 {
        self.region_id
    }
}

/// Write mode for the emulated account.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteMode {
    /// Only the first (hub) region accepts writes.
    Single,
    /// All regions accept writes (multi-master).
    Multi,
}

/// Consistency level for the emulated account.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConsistencyLevel {
    Strong,
    BoundedStaleness,
    Session,
    ConsistentPrefix,
    Eventual,
}

impl ConsistencyLevel {
    /// Returns the Cosmos DB API string representation.
    pub fn as_str(&self) -> &str {
        match self {
            ConsistencyLevel::Strong => "Strong",
            ConsistencyLevel::BoundedStaleness => "BoundedStaleness",
            ConsistencyLevel::Session => "Session",
            ConsistencyLevel::ConsistentPrefix => "ConsistentPrefix",
            ConsistencyLevel::Eventual => "Eventual",
        }
    }

    /// Returns whether this is session consistency.
    pub fn is_session(&self) -> bool {
        matches!(self, ConsistencyLevel::Session)
    }
}

/// Default cap on the number of pending entries the per-region replication
/// buffer will hold while paused. Once the buffer reaches this cap the
/// emulator returns 429/3075 to subsequent writes from the source region
/// (matching the real service's `RetryWith` / `ReplicaTooMuchTimeBehind`
/// behavior) instead of buffering indefinitely.
pub const DEFAULT_MAX_BUFFERED_REPLICATIONS: usize = 10_000;

/// Type alias for the per-replication delay sampling function.
pub type ReplicationDelayFn = std::sync::Arc<dyn Fn() -> Duration + Send + Sync>;

/// Replication delay and back-pressure configuration.
#[derive(Clone)]
pub struct ReplicationConfig {
    min_delay: Duration,
    max_delay: Duration,
    max_buffered_replications: usize,
    delay_fn: Option<ReplicationDelayFn>,
    /// Optional fixed seed for the jitter PRNG. When set, replication delays
    /// within `[min_delay, max_delay]` are sampled from a deterministic
    /// xorshift sequence keyed by this seed instead of the thread-local
    /// time-seeded state. Useful for reproducing flakes from delayed-
    /// replication races without falling back to `immediate()` / `fixed()`.
    jitter_seed: Option<std::sync::Arc<std::sync::Mutex<u64>>>,
}

impl std::fmt::Debug for ReplicationConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReplicationConfig")
            .field("min_delay", &self.min_delay)
            .field("max_delay", &self.max_delay)
            .field("max_buffered_replications", &self.max_buffered_replications)
            .field("delay_fn", &self.delay_fn.as_ref().map(|_| "<custom>"))
            .field(
                "jitter_seed",
                &self.jitter_seed.as_ref().map(|_| "<seeded>"),
            )
            .finish()
    }
}

impl ReplicationConfig {
    /// Zero-delay replication (synchronous).
    pub fn immediate() -> Self {
        Self {
            min_delay: Duration::ZERO,
            max_delay: Duration::ZERO,
            max_buffered_replications: DEFAULT_MAX_BUFFERED_REPLICATIONS,
            delay_fn: None,
            jitter_seed: None,
        }
    }

    /// Fixed delay for deterministic testing.
    pub fn fixed(delay: Duration) -> Self {
        Self {
            min_delay: delay,
            max_delay: delay,
            max_buffered_replications: DEFAULT_MAX_BUFFERED_REPLICATIONS,
            delay_fn: None,
            jitter_seed: None,
        }
    }

    /// Random delay within a range.
    pub fn range(min: Duration, max: Duration) -> crate::error::Result<Self> {
        if min > max {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message("min delay must be <= max delay")
                .build());
        }
        Ok(Self {
            min_delay: min,
            max_delay: max,
            max_buffered_replications: DEFAULT_MAX_BUFFERED_REPLICATIONS,
            delay_fn: None,
            jitter_seed: None,
        })
    }

    /// Sets the maximum number of replication entries that can be buffered
    /// while a target region is paused. Writes that would push the buffer
    /// past this cap are rejected with 429/3075.
    pub fn with_max_buffered_replications(mut self, max: usize) -> Self {
        self.max_buffered_replications = max.max(1);
        self
    }

    /// Overrides the per-replication delay sampling function with a caller-
    /// supplied closure. Useful for tests that want deterministic delays
    /// (e.g. `|| Duration::ZERO`) without depending on the thread-local
    /// xorshift PRNG.
    pub fn with_replication_delay_fn(mut self, f: ReplicationDelayFn) -> Self {
        self.delay_fn = Some(f);
        self
    }

    /// Pins the jitter PRNG to a fixed seed so [`Self::sample_delay`] returns a
    /// deterministic, reproducible sequence within `[min_delay, max_delay]`.
    /// Use this in tests that want to reproduce a flake from delayed-
    /// replication races without resorting to `immediate()` / `fixed()`.
    pub fn with_jitter_seed(mut self, seed: u64) -> Self {
        // Avoid the all-zeros seed, which is a fixed point of xorshift64.
        let s = if seed == 0 { 0xDEAD_BEEF_u64 } else { seed };
        self.jitter_seed = Some(std::sync::Arc::new(std::sync::Mutex::new(s)));
        self
    }

    /// Returns whether this is immediate (zero-delay) replication.
    pub fn is_immediate(&self) -> bool {
        self.delay_fn.is_none() && self.max_delay == Duration::ZERO
    }

    /// Samples a delay duration from the configured range (or the custom
    /// delay function, if set).
    pub fn sample_delay(&self) -> Duration {
        if let Some(f) = &self.delay_fn {
            return f();
        }
        if self.min_delay == self.max_delay {
            return self.min_delay;
        }
        let range = self.max_delay - self.min_delay;
        let frac = if let Some(state) = &self.jitter_seed {
            seeded_xorshift_fraction(state)
        } else {
            rand_fraction()
        };
        self.min_delay + range.mul_f64(frac)
    }

    pub fn min_delay(&self) -> Duration {
        self.min_delay
    }

    pub fn max_delay(&self) -> Duration {
        self.max_delay
    }

    /// Returns the configured cap on buffered replications.
    pub fn max_buffered_replications(&self) -> usize {
        self.max_buffered_replications
    }
}

impl Default for ReplicationConfig {
    /// Default: 20-50ms random delay, buffer cap of
    /// [`DEFAULT_MAX_BUFFERED_REPLICATIONS`].
    fn default() -> Self {
        Self {
            min_delay: Duration::from_millis(20),
            max_delay: Duration::from_millis(50),
            max_buffered_replications: DEFAULT_MAX_BUFFERED_REPLICATIONS,
            delay_fn: None,
            jitter_seed: None,
        }
    }
}

/// Simple pseudo-random fraction [0, 1) using thread-local state.
///
/// Not cryptographically secure and **intentionally non-deterministic**: the
/// thread-local state is seeded from `SystemTime::now().as_nanos()` at first
/// use, so two threads spawning in the same nanosecond will share the same
/// initial seed. Tests that require reproducible replication delays must
/// either pin the seed via a separate code path (not provided today) or use
/// `ReplicationConfig::immediate()` / `ReplicationConfig::fixed`.
/// Seeded xorshift fraction in `[0, 1)` keyed by a caller-supplied state.
/// Same algorithm as [`rand_fraction`] but uses the per-config mutex as the
/// state, so sequences are deterministic and isolated per `ReplicationConfig`.
fn seeded_xorshift_fraction(state: &std::sync::Arc<std::sync::Mutex<u64>>) -> f64 {
    let mut guard = state.lock().unwrap();
    let mut x = *guard;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *guard = x;
    ((x >> 11) as f64) / ((1u64 << 53) as f64)
}

fn rand_fraction() -> f64 {
    use std::cell::Cell;
    use std::time::SystemTime;

    thread_local! {
        static STATE: Cell<u64> = Cell::new(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64
        );
    }

    STATE.with(|s| {
        // xorshift64
        let mut x = s.get();
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        s.set(x);
        ((x >> 11) as f64) / ((1u64 << 53) as f64)
    })
}

/// Per-container configuration overrides.
#[derive(Clone, Debug)]
pub struct ContainerConfig {
    partition_count: u32,
    partition_key_range_page_size: Option<u32>,
    provisioned_throughput_ru: Option<u32>,
}

/// Inclusive upper bound on the number of physical partitions a container
/// can be configured with. The cap prevents pathological inputs (e.g.
/// `u32::MAX`) from triggering 4-billion-element `Vec` allocations during
/// container creation; real Cosmos DB physical partition counts are several
/// orders of magnitude below this value.
pub const MAX_PARTITION_COUNT: u32 = 100_000;

impl ContainerConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the number of physical partitions. Validation is deferred to
    /// [`Self::build`].
    pub fn with_partition_count(mut self, count: u32) -> Self {
        self.partition_count = count;
        self
    }

    /// Sets the number of partition key ranges returned per `/pkranges` page.
    ///
    /// When unset, the emulator returns all ranges in one page.
    pub fn with_partition_key_range_page_size(mut self, page_size: u32) -> Self {
        self.partition_key_range_page_size = Some(page_size);
        self
    }

    /// Sets the provisioned throughput in RU/s. Validation is deferred to
    /// [`Self::build`].
    pub fn with_throughput(mut self, ru_per_second: u32) -> Self {
        self.provisioned_throughput_ru = Some(ru_per_second);
        self
    }

    /// Validates the configuration and returns the finalized
    /// [`ContainerConfig`].
    ///
    /// Validation rules:
    /// - `partition_count` must be in `1..=MAX_PARTITION_COUNT`.
    /// - `partition_key_range_page_size`, when set, must be greater than zero.
    /// - `provisioned_throughput_ru`, when set, must be `>= 400` RU/s.
    ///
    /// Returns a `Client` error on the first violation.
    pub fn build(self) -> crate::error::Result<Self> {
        if self.partition_count == 0 {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message("partition count must be > 0")
                .build());
        }
        if self.partition_count > MAX_PARTITION_COUNT {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message(format!("partition count must be <= {MAX_PARTITION_COUNT}"))
                .build());
        }
        if self.partition_key_range_page_size == Some(0) {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message("partition key range page size must be > 0")
                .build());
        }
        if let Some(ru) = self.provisioned_throughput_ru {
            if ru < 400 {
                return Err(crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::new(
                        azure_core::http::StatusCode::BadRequest,
                    ))
                    .with_message("provisioned throughput must be >= 400 RU/s")
                    .build());
            }
        }
        Ok(self)
    }

    pub fn partition_count(&self) -> u32 {
        self.partition_count
    }

    pub fn partition_key_range_page_size(&self) -> Option<u32> {
        self.partition_key_range_page_size
    }

    pub fn provisioned_throughput_ru(&self) -> Option<u32> {
        self.provisioned_throughput_ru
    }
}

impl Default for ContainerConfig {
    /// Defaults to 4 physical partitions, unpaged partition metadata, and no
    /// provisioned throughput.
    fn default() -> Self {
        Self {
            partition_count: 4,
            partition_key_range_page_size: None,
            provisioned_throughput_ru: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ContainerConfig;

    #[test]
    fn partition_key_range_page_size_must_be_positive() {
        let error = ContainerConfig::new()
            .with_partition_key_range_page_size(0)
            .build()
            .unwrap_err();

        assert!(error
            .to_string()
            .contains("partition key range page size must be > 0"));
    }
}
