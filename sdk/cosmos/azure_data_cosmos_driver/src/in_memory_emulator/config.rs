// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Virtual account configuration for the in-memory emulator.

use std::collections::HashMap;
use std::time::Duration;
use url::Url;

use super::ru_model::RuChargingModel;

/// Configures the emulated Cosmos DB account.
#[derive(Clone, Debug)]
pub struct VirtualAccountConfig {
    regions: Vec<VirtualRegion>,
    write_mode: WriteMode,
    consistency: ConsistencyLevel,
    replication: ReplicationConfig,
    replication_overrides: HashMap<(String, String), ReplicationConfig>,
    ru_model: RuChargingModel,
    throttling_enabled: bool,
}

impl VirtualAccountConfig {
    /// Creates a new configuration with the given regions.
    /// The first region is the hub/primary write region in single-write mode.
    pub fn new(regions: Vec<VirtualRegion>) -> Self {
        assert!(!regions.is_empty(), "at least one region is required");
        Self {
            regions,
            write_mode: WriteMode::Single,
            consistency: ConsistencyLevel::Session,
            replication: ReplicationConfig::default(),
            replication_overrides: HashMap::new(),
            ru_model: RuChargingModel::default(),
            throttling_enabled: false,
        }
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
    pub fn with_replication_override(
        mut self,
        source: &str,
        target: &str,
        config: ReplicationConfig,
    ) -> Self {
        self.replication_overrides
            .insert((source.to_string(), target.to_string()), config);
        self
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
    pub fn region_for_url(&self, url: &Url) -> Option<&str> {
        let host = url.host_str()?;
        for r in &self.regions {
            if let Some(rhost) = r.gateway_url.host_str() {
                if rhost.eq_ignore_ascii_case(host) {
                    return Some(&r.name);
                }
            }
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
    /// Creates a new region. The `region_id` is auto-assigned based on position
    /// in the regions list when constructing `VirtualAccountConfig`.
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

/// Replication delay configuration.
#[derive(Clone, Debug)]
pub struct ReplicationConfig {
    min_delay: Duration,
    max_delay: Duration,
}

impl ReplicationConfig {
    /// Zero-delay replication (synchronous).
    pub fn immediate() -> Self {
        Self {
            min_delay: Duration::ZERO,
            max_delay: Duration::ZERO,
        }
    }

    /// Fixed delay for deterministic testing.
    pub fn fixed(delay: Duration) -> Self {
        Self {
            min_delay: delay,
            max_delay: delay,
        }
    }

    /// Random delay within a range.
    pub fn range(min: Duration, max: Duration) -> Self {
        assert!(min <= max, "min delay must be <= max delay");
        Self {
            min_delay: min,
            max_delay: max,
        }
    }

    /// Returns whether this is immediate (zero-delay) replication.
    pub fn is_immediate(&self) -> bool {
        self.max_delay == Duration::ZERO
    }

    /// Samples a delay duration from the configured range.
    pub fn sample_delay(&self) -> Duration {
        if self.min_delay == self.max_delay {
            return self.min_delay;
        }
        let range = self.max_delay - self.min_delay;
        let frac = rand_fraction();
        self.min_delay + range.mul_f64(frac)
    }

    pub fn min_delay(&self) -> Duration {
        self.min_delay
    }

    pub fn max_delay(&self) -> Duration {
        self.max_delay
    }
}

impl Default for ReplicationConfig {
    /// Default: 20-50ms random delay.
    fn default() -> Self {
        Self {
            min_delay: Duration::from_millis(20),
            max_delay: Duration::from_millis(50),
        }
    }
}

/// Simple pseudo-random fraction [0, 1) using thread-local state.
/// Not cryptographically secure — fine for test infrastructure.
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
        (x as f64) / (u64::MAX as f64)
    })
}

/// Per-container configuration overrides.
#[derive(Clone, Debug)]
pub struct ContainerConfig {
    partition_count: u32,
    provisioned_throughput_ru: Option<u32>,
}

impl ContainerConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the number of physical partitions.
    pub fn with_partition_count(mut self, count: u32) -> Self {
        assert!(count > 0, "partition count must be > 0");
        self.partition_count = count;
        self
    }

    /// Sets the provisioned throughput in RU/s.
    /// Minimum is 400 RU/s. When set and throttling is enabled, the emulator
    /// returns 429/3200 when consumed RU/s exceeds this limit.
    pub fn with_throughput(mut self, ru_per_second: u32) -> Self {
        assert!(
            ru_per_second >= 400,
            "provisioned throughput must be >= 400 RU/s"
        );
        self.provisioned_throughput_ru = Some(ru_per_second);
        self
    }

    pub fn partition_count(&self) -> u32 {
        self.partition_count
    }

    pub fn provisioned_throughput_ru(&self) -> Option<u32> {
        self.provisioned_throughput_ru
    }
}

impl Default for ContainerConfig {
    fn default() -> Self {
        Self {
            partition_count: 4,
            provisioned_throughput_ru: None,
        }
    }
}
