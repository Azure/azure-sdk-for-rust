// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore PRNG
//! Virtual account configuration for the in-memory emulator.

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;
use url::Url;

use super::ru_model::RuChargingModel;

/// Runtime-mutable account topology.
///
/// Region membership, write mode and the current write region all change over
/// the life of a real account (ARM add/remove region, enabling multi-master,
/// failover), and the driver only ever learns about it through the account-read
/// payload. Keeping them here behind a single lock lets test code mutate the
/// topology through the shared `&self` handle it already has.
#[derive(Debug)]
struct AccountTopology {
    /// Regions advertised in `readableLocations`, in order.
    active: Vec<VirtualRegion>,
    /// Regions removed from the account. Not advertised, but still resolvable
    /// by URL so requests the client sends before its next topology refresh get
    /// the same `403/1008 DatabaseAccountNotFound` the real service returns.
    retired: Vec<VirtualRegion>,
    /// Active regions whose endpoint has already begun rejecting requests with
    /// `403/1008` even though the account still advertises them. See
    /// [`RegionStatus::Draining`].
    draining: HashSet<String>,
    /// Regions whose add/buildout is still in progress.
    ///
    /// They already exist in the internal topology and have a region store, but
    /// are filtered from both client-visible location lists. This mirrors
    /// `DatabaseAccountHandler.RemoveInProgressRegionsFromConfiguration`, which
    /// checks `DocumentServiceConfiguration.IsAddRegionInProgress`.
    adding: HashMap<String, u64>,
    /// Regions taken offline by the ARM `offlineRegion` operation.
    ///
    /// Kept separate from [`Self::draining`] because the two behave differently
    /// at the endpoint: a draining region answers `403/1008`, whereas an
    /// offlined region's DNS record is withdrawn entirely. See
    /// [`RegionStatus::Offline`].
    offline: HashSet<String>,
    /// Satellite regions whose local write status is revoked.
    ///
    /// Mirrors `Topology.WriteStatusRevokedSatelliteRegions`. Revocation is a
    /// backend enforcement state, not a gateway-list filter: the region stays
    /// in both advertised lists on a multi-write account but rejects writes.
    write_revoked: HashSet<String>,
    /// Region names in failover-priority order, position 0 first.
    ///
    /// Deliberately **separate** from [`Self::active`], which carries
    /// advertisement order. Verified against a live account: changing priorities
    /// below position 0 leaves `readableLocations` byte-identical, so priority
    /// cannot be modelled by reordering the advertised list. Only position 0 is
    /// observable, and then only through which region owns writes.
    priority_order: Vec<String>,
    write_mode: WriteMode,
    /// Name of the region in `writableLocations[0]` under single-write.
    write_region: String,
    /// The outgoing write region during an in-flight failover.
    ///
    /// While set, a single-write account advertises *both* this region and
    /// [`Self::write_region`] in `writableLocations`. Verified against a live
    /// account: a manual failover widens `writableLocations` to two entries for
    /// ~16 s -- with `enableMultipleWriteLocations` still `false` -- before
    /// narrowing to the new write region alone.
    ///
    /// Mirrors the service's `Topology.PreviousWriteRegion`, which
    /// `DatabaseAccountHandler.GetLocationsFromTopology` adds to both location
    /// lists alongside the current write region.
    failing_over_from: Option<String>,
    /// The incoming write region announced *before* write ownership moves.
    ///
    /// Mirrors the service's `Topology.NextWriteRegion`, the third write-region
    /// slot `GetLocationsFromTopology` folds into `writableLocations`. Verified
    /// against a live account: `writableLocations` listed both regions for ~6 s
    /// while the outgoing region was still returning `201` for writes, and only
    /// then did it begin returning `403/3`. Write ownership does **not** move
    /// while this is set -- that is what distinguishes the announce phase from
    /// [`Self::failing_over_from`].
    next_write_region: Option<String>,
    /// Monotonic incarnation token for asynchronous region-add operations.
    ///
    /// A removed region can be re-added before an older delayed task wakes up.
    /// The token prevents that stale task from exposing the newer incarnation.
    next_add_operation_id: u64,
    /// Monotonic region-ID allocator. IDs are never reused: session-token
    /// vector clocks embed them, so renumbering would corrupt live tokens.
    next_region_id: u64,
}

/// A point-in-time view of the mutable account topology, taken under a single
/// read guard so every field is mutually consistent.
#[derive(Clone, Debug)]
pub struct TopologySnapshot {
    /// Regions currently part of the account, in advertisement order.
    pub active: Vec<VirtualRegion>,
    /// Whether all active regions accept writes.
    pub write_mode: WriteMode,
    /// Name of the region in `writableLocations[0]` under single-write.
    pub write_region: String,
    /// Regions taken offline, which must not appear in either advertised list.
    pub offline: HashSet<String>,
    /// Regions mid-add, which must not appear in either advertised list.
    pub adding: HashSet<String>,
    /// The outgoing write region while a failover is in flight, advertised as
    /// writable alongside `write_region`.
    pub failing_over_from: Option<String>,
    /// The incoming write region announced before write ownership moves, also
    /// advertised as writable.
    pub next_write_region: Option<String>,
}

impl TopologySnapshot {
    /// Returns the regions to advertise, offline regions removed and the
    /// current write region hoisted to the front.
    ///
    /// Verified against a live account: the p0 region is always first in
    /// `readableLocations`, but the *remaining* entries do not follow
    /// `failoverPriority` -- four successive priority configurations produced
    /// tail orders that no positional rule explained. Only the "write region
    /// first" rule is reproducible, so the tail is left in insertion order.
    pub fn advertised(&self) -> Vec<&VirtualRegion> {
        let mut regions: Vec<&VirtualRegion> = self
            .active
            .iter()
            .filter(|r| !self.offline.contains(r.name()) && !self.adding.contains(r.name()))
            .collect();
        if let Some(idx) = regions.iter().position(|r| r.name() == self.write_region) {
            let write_region = regions.remove(idx);
            regions.insert(0, write_region);
        }
        regions
    }

    /// Returns the regions to advertise in `writableLocations`.
    ///
    /// Under multi-write every advertised region is writable. Under
    /// single-write it is the write region, plus the incoming region of an
    /// announced failover and the outgoing region of one already switched --
    /// the three write-region slots the service folds together in
    /// `DatabaseAccountHandler.GetLocationsFromTopology` (`WriteRegion`,
    /// `NextWriteRegion`, `PreviousWriteRegion`).
    pub fn writable(&self, allow_multiple_write_locations: bool) -> Vec<&VirtualRegion> {
        let advertised = self.advertised();
        if self.write_mode == WriteMode::Multi {
            if allow_multiple_write_locations {
                return advertised;
            }
            // Strong consistency is the gateway's second gate on multi-write:
            // failover transition slots do not bypass the hub-only result.
            return advertised
                .into_iter()
                .filter(|r| r.name() == self.write_region)
                .collect();
        }
        advertised
            .into_iter()
            .filter(|r| {
                r.name() == self.write_region
                    || self.failing_over_from.as_deref() == Some(r.name())
                    || self.next_write_region.as_deref() == Some(r.name())
            })
            .collect()
    }
}

/// Outcome of resolving a request URL against the account topology.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedRegion {
    /// Region name.
    pub name: String,
    /// Whether the region is still part of the account.
    pub status: RegionStatus,
}

/// Whether a resolved region is still part of the account.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionStatus {
    /// Advertised in the account topology and serving requests.
    Active,
    /// Still advertised in the account topology, but its endpoint already
    /// rejects requests with `403/1008`.
    ///
    /// This is a real window, not a synthetic one: removing a region makes its
    /// regional endpoint start returning `403/1008` within seconds, while the
    /// account read keeps listing it for several more minutes. A client in that
    /// window refreshes topology in response to the 1008 and gets back a payload
    /// that still contains the dead region.
    Draining,
    /// Taken out of service by the ARM `offlineRegion` operation.
    ///
    /// Distinct from [`Self::Draining`] in both cause and symptom. Verified
    /// against a live account: offlining a region withdraws its regional DNS
    /// record, so a client opening a new connection fails to resolve the host
    /// rather than receiving `403/1008`. The region also disappears from both
    /// `readableLocations` and `writableLocations`, which is what makes it
    /// invisible to a client that only reads the account payload.
    Offline,
    /// Removed from the account. Requests must fail with 403/1008.
    Retired,
}

impl RegionStatus {
    /// Whether a request routed here must be rejected with `403/1008`.
    ///
    /// [`Self::Offline`] is deliberately excluded: an offlined region fails at
    /// the transport layer (DNS withdrawal), not with a Cosmos error, so it is
    /// handled separately by the dispatcher.
    pub fn is_unavailable(&self) -> bool {
        matches!(self, RegionStatus::Draining | RegionStatus::Retired)
    }

    /// Whether a request routed here must fail before reaching the service,
    /// as it would against a host whose DNS record has been withdrawn.
    pub fn is_unreachable(&self) -> bool {
        matches!(self, RegionStatus::Offline)
    }
}

/// Controls how quickly a newly added region starts serving reads.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SeedingPolicy {
    /// The region is fully seeded and advertised before `add_region` returns.
    #[default]
    Immediate,
    /// The region is advertised immediately but stays empty for the given
    /// duration, emulating the window where the service has accepted the region
    /// but replication has not caught up.
    Delayed(Duration),
    /// The region is internally seeded and participates in replication, but is
    /// hidden from both account location lists until buildout completes.
    ///
    /// This models the production gateway's default
    /// `EnableSkipInProgressRegionInGetDatabaseAccount=true`: regions whose
    /// `IsAddRegionInProgress` flag is set are removed from both read and write
    /// locations. Unlike [`Self::Delayed`], the region is not intentionally
    /// emptied: internal replication keeps it current while external clients
    /// cannot route to it.
    HiddenUntilReady(Duration),
}

/// Configures the emulated Cosmos DB account.
///
/// # Cloning shares mutable topology
///
/// `Clone` is shallow for the runtime-mutable state: region membership, write
/// mode, write region and the PPAF flag all live behind `Arc`s that clones
/// share. That is deliberate — [`super::EmulatorStore`] holds one config and
/// tests mutate it through `&self` — but it means a clone is **not** an
/// independent account. Build separate accounts with [`Self::new`].
///
/// # Runtime-mutable fields
///
/// Static fields (consistency, replication, RU model) are set at construction
/// time and never change. Two groups are deliberately mutable through a shared
/// `&self` handle, because the real service changes them under a running client
/// and the driver is expected to notice via its background account refresh:
///
/// - the per-partition-failover flag (PPAF, `enablePerPartitionFailoverBehavior`),
///   flipped with [`Self::set_per_partition_failover`];
/// - the account topology -- region membership, write mode and current write
///   region -- mutated through [`super::EmulatorStore`], which owns the
///   corresponding per-region data stores.
///
/// Cloning a config shares both, so a clone observes the same topology.
#[derive(Clone, Debug)]
pub struct VirtualAccountConfig {
    topology: Arc<RwLock<AccountTopology>>,
    account_id: String,
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
        // Auto-assign monotonically increasing region IDs by position only
        // when the caller did not set one explicitly.
        for (idx, r) in regions.iter_mut().enumerate() {
            r.region_id.get_or_insert(idx as u64);
        }
        let mut region_ids = HashSet::with_capacity(regions.len());
        for region in &regions {
            let region_id = region.region_id.expect("region IDs were assigned above");
            if !region_ids.insert(region_id) {
                return Err(crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::new(
                        azure_core::http::StatusCode::BadRequest,
                    ))
                    .with_message(format!(
                        "region ID {region_id} is configured more than once"
                    ))
                    .build());
            }
        }
        let write_region = regions[0].name.clone();
        let priority_order: Vec<String> = regions.iter().map(|r| r.name.clone()).collect();
        let next_region_id = regions
            .iter()
            .filter_map(|r| r.region_id)
            .max()
            .map_or(0, |max| max.saturating_add(1));
        Ok(Self {
            topology: Arc::new(RwLock::new(AccountTopology {
                active: regions,
                retired: Vec::new(),
                draining: HashSet::new(),
                adding: HashMap::new(),
                offline: HashSet::new(),
                write_revoked: HashSet::new(),
                priority_order,
                write_mode: WriteMode::Single,
                write_region,
                failing_over_from: None,
                next_write_region: None,
                next_add_operation_id: 0,
                next_region_id,
            })),
            account_id: "emulator-account".to_owned(),
            consistency: ConsistencyLevel::Session,
            replication: ReplicationConfig::default(),
            replication_overrides: HashMap::new(),
            ru_model: RuChargingModel::default(),
            throttling_enabled: false,
            enable_per_partition_failover: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Sets the account ID emitted by the hosted account-discovery response.
    #[cfg(feature = "__internal_in_memory_emulator")]
    #[doc(hidden)]
    pub fn with_account_id(mut self, account_id: impl Into<String>) -> Self {
        self.account_id = account_id.into();
        self
    }

    /// Returns the account ID emitted by account discovery.
    pub(crate) fn account_id(&self) -> &str {
        &self.account_id
    }

    /// Sets the write mode.
    ///
    /// # Aliasing
    ///
    /// Unlike the other `with_*` builders, this mutates state shared with every
    /// clone of this config: the topology lives behind an `Arc<RwLock<_>>` so
    /// that [`super::EmulatorStore`] can change it at runtime. Building two
    /// configs from one clone therefore does **not** give them independent
    /// write modes — both observe the last value set:
    ///
    /// ```ignore
    /// let base = VirtualAccountConfig::new(regions)?;
    /// let single = base.clone().with_write_mode(WriteMode::Single);
    /// let multi = base.clone().with_write_mode(WriteMode::Multi);
    /// // `single` is Multi too — same underlying topology.
    /// ```
    ///
    /// Construct each config with [`Self::new`] instead of cloning a base.
    pub fn with_write_mode(self, mode: WriteMode) -> Self {
        self.set_write_mode(mode);
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
        let known: Vec<String> = self.active_region_names();
        if !known.iter().any(|r| r == source) {
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
        if !known.iter().any(|r| r == target) {
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

    /// Returns a single consistent snapshot of the topology.
    ///
    /// Callers that need more than one topology field **must** use this rather
    /// than several accessors: reading `write_mode` twice under two different
    /// guards can straddle a concurrent `set_write_mode` and produce a payload
    /// the real service never emits (for instance `enableMultipleWriteLocations:
    /// true` alongside a single writable location).
    pub fn topology_snapshot(&self) -> TopologySnapshot {
        let topology = self.topology.read().unwrap();
        TopologySnapshot {
            active: topology.active.clone(),
            write_mode: topology.write_mode,
            write_region: topology.write_region.clone(),
            offline: topology.offline.clone(),
            adding: topology.adding.keys().cloned().collect(),
            failing_over_from: topology.failing_over_from.clone(),
            next_write_region: topology.next_write_region.clone(),
        }
    }

    /// Returns the regions currently part of the account, in advertisement
    /// order. Retired regions are excluded.
    pub fn active_regions(&self) -> Vec<VirtualRegion> {
        self.topology.read().unwrap().active.clone()
    }

    /// Returns the names of the regions currently part of the account.
    pub fn active_region_names(&self) -> Vec<String> {
        self.topology
            .read()
            .unwrap()
            .active
            .iter()
            .map(|r| r.name.clone())
            .collect()
    }

    pub fn write_mode(&self) -> WriteMode {
        self.topology.read().unwrap().write_mode
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

    /// Returns the current write region name (`writableLocations[0]` under
    /// single-write mode).
    pub fn write_region_name(&self) -> String {
        self.topology.read().unwrap().write_region.clone()
    }

    /// Returns whether a region is allowed to accept writes.
    pub fn is_write_region(&self, region_name: &str) -> bool {
        let topology = self.topology.read().unwrap();
        // An offlined or still-building region accepts no writes, in either
        // mode. The latter exists internally but is intentionally not yet
        // available to external customers.
        if topology.offline.contains(region_name) || topology.adding.contains_key(region_name) {
            return false;
        }
        match topology.write_mode {
            // Only regions still in the account count as writable, so a retired
            // region never silently accepts writes on a multi-write account.
            // Strong consistency is the gateway's second gate on multi-write:
            // `enableMultipleWriteLocations` may still be true in the account
            // response, but only the hub is actually writable.
            WriteMode::Multi if self.consistency != ConsistencyLevel::Strong => {
                topology.active.iter().any(|r| r.name == region_name)
                    && !topology.write_revoked.contains(region_name)
            }
            WriteMode::Multi => topology.write_region == region_name,
            // The outgoing region of an in-flight failover is deliberately
            // excluded even though it is still advertised as writable. Verified
            // against a live account: the demoted region begins returning
            // `403/3` ~10 s *before* it disappears from `writableLocations`, so
            // a payload that still lists it is knowingly stale.
            WriteMode::Single => topology.write_region == region_name,
        }
    }

    /// Resolves a request URL to a region, including regions that have been
    /// removed from the account.
    ///
    /// Retired regions stay resolvable because a client keeps sending to a
    /// removed endpoint until its next topology refresh, and the real service
    /// answers those with `403/1008 DatabaseAccountNotFound` rather than
    /// failing to route at all.
    ///
    /// Matches on (scheme, host, port) — not host alone — so two regions
    /// that share a hostname but differ in port (or scheme) route correctly.
    /// Useful when adding e.g. `https://localhost:8081` and
    /// `https://localhost:8082` regions for parity tests.
    pub fn region_for_url(&self, url: &Url) -> Option<ResolvedRegion> {
        let host = url.host_str()?;
        let scheme = url.scheme();
        let port = url.port_or_known_default();
        // Matches either the standard gateway URL or the Gateway 2.0 URL, so a
        // request sent to a region's thin-client endpoint resolves to the same
        // region.
        let matches = |r: &VirtualRegion| {
            let matches_url = |candidate: &Url| {
                candidate
                    .host_str()
                    .is_some_and(|candidate_host| candidate_host.eq_ignore_ascii_case(host))
                    && candidate.scheme() == scheme
                    && candidate.port_or_known_default() == port
            };
            matches_url(&r.gateway_url) || r.gateway_v2_url.as_ref().is_some_and(matches_url)
        };

        let topology = self.topology.read().unwrap();
        if let Some(r) = topology.active.iter().find(|r| matches(r)) {
            // Offline outranks draining: a region can be drained and then
            // offlined, and the withdrawn DNS record is what a client hits
            // first.
            let status = if topology.offline.contains(&r.name) {
                RegionStatus::Offline
            } else if topology.draining.contains(&r.name) {
                RegionStatus::Draining
            } else {
                RegionStatus::Active
            };
            return Some(ResolvedRegion {
                name: r.name.clone(),
                status,
            });
        }
        topology
            .retired
            .iter()
            .find(|r| matches(r))
            .map(|r| ResolvedRegion {
                name: r.name.clone(),
                status: RegionStatus::Retired,
            })
    }

    /// Finds the region ID for a given region name.
    ///
    /// Retired regions are included: their IDs are still referenced by session
    /// tokens issued while they were active.
    pub fn region_id_for(&self, region_name: &str) -> u64 {
        let topology = self.topology.read().unwrap();
        topology
            .active
            .iter()
            .chain(topology.retired.iter())
            .find(|r| r.name == region_name)
            .and_then(|r| r.region_id)
            .unwrap_or(0)
    }

    /// Adds a region to the account, allocating a fresh region ID unless the
    /// region is being re-added (in which case it keeps its original ID so
    /// session tokens issued before its removal stay meaningful).
    ///
    /// Returns the region as it was added. Errors with `400` if the region is
    /// already part of the account, matching the service's rejection of an
    /// add-read-region request for an existing region.
    pub(crate) fn add_region(
        &self,
        region: VirtualRegion,
        hidden_until_ready: bool,
    ) -> crate::error::Result<(VirtualRegion, Option<u64>)> {
        let mut topology = self.topology.write().unwrap();
        if topology.active.iter().any(|r| r.name == region.name) {
            return Err(already_present(&region.name));
        }
        let add_operation_id = if hidden_until_ready {
            let operation_id = topology.next_add_operation_id;
            if operation_id == u64::MAX {
                return Err(bad_request(
                    "region add operation ID space is exhausted".to_string(),
                ));
            }
            Some(operation_id)
        } else {
            None
        };

        let mut region = region;
        if let Some(idx) = topology.retired.iter().position(|r| r.name == region.name) {
            // A re-added region keeps its original ID: session-token vector
            // clocks issued while it was active still reference it.
            let previous = topology.retired.remove(idx);
            region.region_id = previous.region_id;
        } else if let Some(explicit) = region.region_id {
            if topology
                .active
                .iter()
                .chain(topology.retired.iter())
                .any(|r| r.region_id == Some(explicit))
            {
                return Err(bad_request(format!(
                    "region ID {explicit} is already in use"
                )));
            }
            topology.next_region_id = topology.next_region_id.max(explicit.saturating_add(1));
        } else {
            let id = topology.next_region_id;
            if id == u64::MAX {
                // Refuse rather than wrap: reusing an ID would break the
                // never-reuse invariant that makes stale session tokens safe.
                return Err(bad_request("region ID space is exhausted".to_string()));
            }
            region.region_id = Some(id);
            topology.next_region_id = id + 1;
        }

        topology.active.push(region.clone());
        if let Some(operation_id) = add_operation_id {
            topology.next_add_operation_id = operation_id + 1;
            topology.adding.insert(region.name.clone(), operation_id);
        }
        if !topology.priority_order.contains(&region.name) {
            // A newly added or re-added region has the lowest priority.
            topology.priority_order.push(region.name.clone());
        }
        Ok((region, add_operation_id))
    }

    /// Removes a region from the account, retaining it as retired so its
    /// endpoint keeps resolving.
    ///
    /// Errors with `400` if the region is unknown, if it is the last region, or
    /// if it currently owns writes. The write-region guard applies in **both**
    /// write modes: under multi-write every active region accepts writes, but
    /// `write_region` still designates the hub that a later `set_write_mode`
    /// back to single-write would restore, and it is the region new regions are
    /// seeded from. Letting it dangle would silently produce an account with an
    /// empty `writableLocations` and unseeded new regions.
    pub(crate) fn remove_region(&self, region_name: &str) -> crate::error::Result<()> {
        let mut topology = self.topology.write().unwrap();
        let Some(idx) = topology.active.iter().position(|r| r.name == region_name) else {
            return Err(bad_request(format!(
                "region '{region_name}' is not part of the account"
            )));
        };
        if topology.active.len() == 1 {
            return Err(bad_request(
                "cannot remove the last region from the account".to_string(),
            ));
        }
        if topology.write_region == region_name {
            return Err(bad_request(format!(
                "cannot remove '{region_name}': it is the account's write region; \
                 promote another region with set_write_region first"
            )));
        }

        let removed = topology.active.remove(idx);
        topology.draining.remove(region_name);
        topology.adding.remove(region_name);
        topology.offline.remove(region_name);
        topology.write_revoked.remove(region_name);
        topology.priority_order.retain(|name| name != region_name);
        if topology.failing_over_from.as_deref() == Some(region_name) {
            topology.failing_over_from = None;
        }
        if topology.next_write_region.as_deref() == Some(region_name) {
            topology.next_write_region = None;
        }
        topology.retired.push(removed);
        Ok(())
    }

    /// Marks a hidden, in-progress region ready for external clients.
    pub(crate) fn complete_region_add(&self, region_name: &str, operation_id: u64) {
        let mut topology = self.topology.write().unwrap();
        if topology.adding.get(region_name) == Some(&operation_id) {
            topology.adding.remove(region_name);
        }
    }

    /// Starts draining a region: its endpoint begins rejecting requests with
    /// `403/1008` while the account still advertises it.
    ///
    /// Reproduces the ordering the real service exhibits during a region
    /// removal -- the regional endpoint starts failing within seconds, but the
    /// account read keeps listing the region for minutes afterwards, so a client
    /// that refreshes topology in response to the 1008 gets a payload that still
    /// contains the dead region.
    ///
    /// Carries the same guards as [`Self::remove_region`], of which this is the
    /// first phase: draining the write region or the last region would leave an
    /// account that is advertised but cannot serve anything, and that no public
    /// call could repair.
    pub(crate) fn begin_region_removal(&self, region_name: &str) -> crate::error::Result<()> {
        let mut topology = self.topology.write().unwrap();
        if !topology.active.iter().any(|r| r.name == region_name) {
            return Err(bad_request(format!(
                "region '{region_name}' is not part of the account"
            )));
        }
        if topology.active.len() == 1 {
            return Err(bad_request(
                "cannot drain the last region in the account".to_string(),
            ));
        }
        if topology.write_region == region_name {
            return Err(bad_request(format!(
                "cannot drain '{region_name}': it is the account's write region; \
                 promote another region with set_write_region first"
            )));
        }
        topology.draining.insert(region_name.to_string());
        Ok(())
    }

    /// Switches the account between single- and multi-write.
    ///
    /// Both directions are permitted: multi → single is a supported transition
    /// on a normal account, not a one-way door.
    pub(crate) fn set_write_mode(&self, mode: WriteMode) {
        let mut topology = self.topology.write().unwrap();
        if topology.write_mode != mode {
            topology.failing_over_from = None;
            topology.next_write_region = None;
        }
        topology.write_mode = mode;
    }

    /// Revokes local write status from a satellite region.
    ///
    /// The region remains advertised in both `readableLocations` and
    /// `writableLocations`; writes fail with the normal `403/3 WriteForbidden`
    /// path because `is_write_region` returns false. The service stores this in
    /// `Topology.WriteStatusRevokedSatelliteRegions`, and the routing gateway
    /// does not remove those regions when it constructs the account payload.
    ///
    /// Only a non-hub region of a multi-write account may be revoked. The
    /// service topology invariant explicitly forbids putting `WriteRegion` in
    /// the revoked set.
    pub(crate) fn revoke_region_write(&self, region_name: &str) -> crate::error::Result<()> {
        let mut topology = self.topology.write().unwrap();
        if topology.write_mode != WriteMode::Multi {
            return Err(bad_request(
                "satellite write revocation requires multi-write mode".to_string(),
            ));
        }
        if !topology.active.iter().any(|r| r.name == region_name) {
            return Err(bad_request(format!(
                "region '{region_name}' is not part of the account"
            )));
        }
        if topology.write_region == region_name {
            return Err(bad_request(format!(
                "cannot revoke local write status from '{region_name}': it is the hub write region"
            )));
        }
        topology.write_revoked.insert(region_name.to_string());
        Ok(())
    }

    /// Restores local write status to a previously revoked satellite region.
    pub(crate) fn restore_region_write(&self, region_name: &str) -> crate::error::Result<()> {
        let mut topology = self.topology.write().unwrap();
        if !topology.active.iter().any(|r| r.name == region_name) {
            return Err(bad_request(format!(
                "region '{region_name}' is not part of the account"
            )));
        }
        topology.write_revoked.remove(region_name);
        Ok(())
    }

    /// Moves write ownership to another region, as a failover would.
    ///
    /// This is not an exotic scenario: for single-master accounts the gateway
    /// itself can report an arbitrary read location as the write location
    /// between successive account reads, so clients must tolerate the advertised
    /// write region moving at any time.
    pub(crate) fn set_write_region(&self, region_name: &str) -> crate::error::Result<()> {
        let mut topology = self.topology.write().unwrap();
        if !topology.active.iter().any(|r| r.name == region_name) {
            return Err(bad_request(format!(
                "region '{region_name}' is not part of the account"
            )));
        }
        // Mirror of the guards on `begin_region_removal` / `remove_region`.
        // Promoting a draining region would make the account advertise a write
        // region whose endpoint already rejects every request with 403/1008,
        // and once it is the write region neither removal path can retire it --
        // an unrecoverable state reachable purely through public calls.
        if topology.draining.contains(region_name) {
            return Err(bad_request(format!(
                "cannot promote '{region_name}': it is draining and its endpoint \
                 already rejects requests with 403/1008"
            )));
        }
        if topology.adding.contains_key(region_name) {
            return Err(bad_request(format!(
                "cannot promote '{region_name}': its region add is still in progress"
            )));
        }
        if topology.offline.contains(region_name) {
            return Err(bad_request(format!(
                "cannot promote '{region_name}': it is offline and is not \
                 advertised in the account topology"
            )));
        }
        if topology.write_revoked.contains(region_name) {
            return Err(bad_request(format!(
                "cannot promote '{region_name}': its local write status is revoked"
            )));
        }
        topology.write_region = region_name.to_string();
        // An explicit write-region move is atomic: it models the settled
        // outcome, not the widened intermediate payload that
        // `announce_failover` / `begin_failover` reproduce.
        topology.failing_over_from = None;
        topology.next_write_region = None;
        Ok(())
    }

    /// Aborts an in-flight region removal, returning a draining region to
    /// normal service.
    ///
    /// The real service can abort a region removal, and without this `draining`
    /// would be a one-way door: `remove_region` is the only other exit, and it
    /// is refused for the write region and the last region.
    pub(crate) fn cancel_region_removal(&self, region_name: &str) -> crate::error::Result<()> {
        let mut topology = self.topology.write().unwrap();
        if !topology.active.iter().any(|r| r.name == region_name) {
            return Err(bad_request(format!(
                "region '{region_name}' is not part of the account"
            )));
        }
        topology.draining.remove(region_name);
        Ok(())
    }

    /// Takes a region out of service, as the ARM `offlineRegion` operation does.
    ///
    /// The region stays a member of the account but is dropped from both
    /// advertised location lists, and its endpoint becomes unreachable rather
    /// than returning `403/1008` -- the service withdraws the regional DNS
    /// record.
    ///
    /// Offlining the **write region** is permitted and performs a failover, as
    /// the service does: the next region in priority order is promoted. That is
    /// deliberately unlike [`Self::begin_region_removal`], which refuses the
    /// write region -- removal has no such promotion step, so allowing it would
    /// strand the account without a writable region.
    ///
    /// Errors with `400` if the region is unknown or if it is the only region
    /// left online, which would leave an account that can serve nothing.
    pub(crate) fn set_region_offline(&self, region_name: &str) -> crate::error::Result<()> {
        let mut topology = self.topology.write().unwrap();
        if !topology.active.iter().any(|r| r.name == region_name) {
            return Err(bad_request(format!(
                "region '{region_name}' is not part of the account"
            )));
        }
        if topology.offline.contains(region_name) {
            return Ok(());
        }

        // Promotion follows failover priority, not advertisement order: the
        // service promotes the next region in priority order. Falls back to
        // advertisement order for any region missing from the priority list.
        // Draining regions are excluded alongside offline ones: promoting one
        // would advertise a write region whose endpoint already answers
        // `403/1008`, and once it owns writes neither removal path can retire
        // it.
        let next_online = topology
            .priority_order
            .iter()
            .chain(topology.active.iter().map(|r| &r.name))
            .find(|name| {
                name.as_str() != region_name
                    && !topology.offline.contains(name.as_str())
                    && !topology.draining.contains(name.as_str())
                    && !topology.adding.contains_key(name.as_str())
                    && !topology.write_revoked.contains(name.as_str())
                    && topology.active.iter().any(|r| &r.name == *name)
            })
            .cloned();
        let Some(next_online) = next_online else {
            return Err(bad_request(format!(
                "cannot offline '{region_name}': no other online, non-draining \
                 region remains in the account"
            )));
        };

        if topology.write_region == region_name {
            topology.write_region = next_online.clone();
            // The service renumbers priorities during an offline-triggered
            // failover: surviving regions move up and the offlined former hub
            // moves to the lowest-priority position.
            topology
                .priority_order
                .retain(|name| name != region_name && name != &next_online);
            topology.priority_order.insert(0, next_online);
            topology.priority_order.push(region_name.to_string());
            // No `failing_over_from`: the outgoing region is offline, and an
            // offline region is filtered out of the advertised lists entirely,
            // so advertising it as writable is not representable.
            topology.failing_over_from = None;
            topology.next_write_region = None;
        } else {
            // A non-hub region may still occupy a failover transition slot.
            // Hiding it via `offline` is not enough: onlining it later would
            // resurrect the stale transition unless the slot is cancelled now.
            if topology.failing_over_from.as_deref() == Some(region_name) {
                topology.failing_over_from = None;
            }
            if topology.next_write_region.as_deref() == Some(region_name) {
                topology.next_write_region = None;
            }
        }
        topology.offline.insert(region_name.to_string());
        Ok(())
    }

    /// Returns an offlined region to service, as the ARM `onlineRegion`
    /// operation does.
    ///
    /// Note that on a live account `onlineRegion` is gated behind an account
    /// capability that is **off by default** -- an attempt against a normal
    /// account is rejected with `400 "OnlineRegion capability not enabled"`, and
    /// re-listing the region with an ordinary topology update does not restore
    /// it either. The emulator models the capability as always present so tests
    /// can exercise recovery without a second account shape; tests that care
    /// about the default should assert the offline path only.
    ///
    /// Does not restore write ownership: a failover triggered by offlining the
    /// write region is not undone by bringing the region back.
    pub(crate) fn set_region_online(&self, region_name: &str) -> crate::error::Result<()> {
        let mut topology = self.topology.write().unwrap();
        if !topology.active.iter().any(|r| r.name == region_name) {
            return Err(bad_request(format!(
                "region '{region_name}' is not part of the account"
            )));
        }
        topology.offline.remove(region_name);
        Ok(())
    }

    /// Reorders the account's failover priorities, as the ARM
    /// `failoverPriorityChange` operation does.
    ///
    /// `order` must name every active region exactly once, matching the
    /// service, which requires a complete assignment and rejects partial ones.
    ///
    /// Position 0 designates the hub in both modes, so changing it hoists the
    /// promoted region in the account payload. Under [`WriteMode::Single`] this
    /// **is** a manual failover and moves exclusive write ownership; under
    /// multi-write the other eligible regions remain writable. A reorder that
    /// leaves position 0 alone has no observable effect on the account payload
    /// at all -- verified against a live account, where swapping p1 and p2
    /// changed the ARM view while leaving `readableLocations` byte-identical
    /// and interrupting no writes.
    pub(crate) fn set_failover_priorities(&self, order: &[&str]) -> crate::error::Result<()> {
        let mut topology = self.topology.write().unwrap();
        if order.len() != topology.active.len() {
            return Err(bad_request(format!(
                "failover priorities must cover all {} active regions, got {}",
                topology.active.len(),
                order.len()
            )));
        }
        let mut seen = HashSet::with_capacity(order.len());
        for name in order {
            if !topology.active.iter().any(|r| &r.name == name) {
                return Err(bad_request(format!(
                    "region '{name}' is not part of the account"
                )));
            }
            if !seen.insert(*name) {
                return Err(bad_request(format!(
                    "region '{name}' is listed more than once"
                )));
            }
        }

        // Every validation runs before any mutation: a rejected
        // `failoverPriorityChange` must leave the account exactly as it was,
        // and a half-applied `priority_order` would silently steer the next
        // `set_region_offline` promotion.
        let promoted = order[0].to_string();
        // Position 0 designates the hub in both modes. Under single-write this
        // moves exclusive write ownership; under multi-write every eligible
        // region can still write, but the promoted hub is advertised first and
        // becomes the source for later seeding or a switch back to single-write.
        let moves_hub = topology.write_region != promoted;
        if moves_hub {
            if topology.offline.contains(&promoted) {
                return Err(bad_request(format!(
                    "cannot promote '{promoted}' to write region: it is offline"
                )));
            }
            if topology.draining.contains(&promoted) {
                return Err(bad_request(format!(
                    "cannot promote '{promoted}' to write region: it is draining \
                     and its endpoint already rejects requests with 403/1008"
                )));
            }
            if topology.adding.contains_key(&promoted) {
                return Err(bad_request(format!(
                    "cannot promote '{promoted}' to write region: its region add is still in progress"
                )));
            }
            if topology.write_revoked.contains(&promoted) {
                return Err(bad_request(format!(
                    "cannot promote '{promoted}' to write region: its local write status is revoked"
                )));
            }
        }

        // Recorded as priority order only. `active` -- the advertisement order --
        // is deliberately left untouched: on a live account a reorder below
        // position 0 leaves `readableLocations` byte-identical, so reordering
        // the advertised list here would invent a change the service never
        // makes. It also keeps every `region_id` stable, which matters because
        // session-token vector clocks embed them.
        topology.priority_order = order.iter().map(|name| (*name).to_string()).collect();

        if moves_hub {
            topology.write_region = promoted;
            // Like `set_write_region`, a priority-driven move models the settled
            // outcome. Leaving a prior failover slot set would advertise the
            // outgoing (or announced incoming) region of an *unrelated* failover
            // alongside the new write region -- a payload the service never
            // produces.
            topology.failing_over_from = None;
            topology.next_write_region = None;
        }
        Ok(())
    }

    /// Announces an upcoming failover without moving write ownership.
    ///
    /// Models the service's `Topology.NextWriteRegion`: `writableLocations`
    /// widens to include the incoming region while the **outgoing** region is
    /// still the one that actually accepts writes. Verified against a live
    /// single-write account -- both regions were advertised for ~6 s during
    /// which writes to the outgoing region still returned `201`, and only then
    /// did it begin returning `403/3`.
    ///
    /// This is the leading edge of a failover; [`Self::begin_failover`] is the
    /// trailing edge, where ownership has moved and the outgoing region rejects
    /// writes while still advertised. A full transition is
    /// `announce_failover` -> `begin_failover` -> [`Self::complete_failover`].
    ///
    /// The announced region does **not** accept writes yet: ownership has not
    /// moved. Whether the live service accepts writes there during this window
    /// was not observed, so the emulator takes the conservative reading that
    /// only the current write region does.
    pub(crate) fn announce_failover(&self, region_name: &str) -> crate::error::Result<()> {
        let mut topology = self.topology.write().unwrap();
        if !topology.active.iter().any(|r| r.name == region_name) {
            return Err(bad_request(format!(
                "region '{region_name}' is not part of the account"
            )));
        }
        if topology.offline.contains(region_name) {
            return Err(bad_request(format!(
                "cannot announce a failover to '{region_name}': it is offline"
            )));
        }
        if topology.draining.contains(region_name) {
            return Err(bad_request(format!(
                "cannot announce a failover to '{region_name}': it is draining \
                 and its endpoint already rejects requests with 403/1008"
            )));
        }
        if topology.adding.contains_key(region_name) {
            return Err(bad_request(format!(
                "cannot announce a failover to '{region_name}': its region add is still in progress"
            )));
        }
        if topology.write_revoked.contains(region_name) {
            return Err(bad_request(format!(
                "cannot announce a failover to '{region_name}': its local write status is revoked"
            )));
        }
        if topology.write_region == region_name {
            return Ok(());
        }
        topology.next_write_region = Some(region_name.to_string());
        Ok(())
    }

    /// Begins a manual failover to `region_name`, entering the intermediate
    /// state in which **both** the outgoing and incoming write regions are
    /// advertised as writable.
    ///
    /// Verified against a live single-write account: a failover widens
    /// `writableLocations` to two entries -- while `enableMultipleWriteLocations`
    /// stays `false` -- for ~16 s before narrowing to the new write region.
    /// Writes to the outgoing region begin failing with `403/3` partway through
    /// that window, so for ~10 s the payload advertises a region that is already
    /// refusing writes. [`Self::complete_failover`] ends the window.
    ///
    /// Write ownership moves immediately, so the outgoing region starts
    /// rejecting writes as soon as this returns -- reproducing the stale-payload
    /// race rather than a clean handover.
    pub(crate) fn begin_failover(&self, region_name: &str) -> crate::error::Result<()> {
        let mut topology = self.topology.write().unwrap();
        if !topology.active.iter().any(|r| r.name == region_name) {
            return Err(bad_request(format!(
                "region '{region_name}' is not part of the account"
            )));
        }
        if topology.offline.contains(region_name) {
            return Err(bad_request(format!(
                "cannot fail over to '{region_name}': it is offline"
            )));
        }
        if topology.draining.contains(region_name) {
            return Err(bad_request(format!(
                "cannot fail over to '{region_name}': it is draining and its \
                 endpoint already rejects requests with 403/1008"
            )));
        }
        if topology.adding.contains_key(region_name) {
            return Err(bad_request(format!(
                "cannot fail over to '{region_name}': its region add is still in progress"
            )));
        }
        if topology.write_revoked.contains(region_name) {
            return Err(bad_request(format!(
                "cannot fail over to '{region_name}': its local write status is revoked"
            )));
        }
        if topology.write_region == region_name {
            return Ok(());
        }
        let outgoing = std::mem::replace(&mut topology.write_region, region_name.to_string());
        topology.failing_over_from = Some(outgoing);
        // The announce slot has served its purpose: the incoming region is now
        // the write region, so keeping it in `next_write_region` would list it
        // twice in the same payload.
        topology.next_write_region = None;
        Ok(())
    }

    /// Ends an in-flight failover, narrowing `writableLocations` back to the
    /// single current write region. A no-op when no failover is in flight.
    pub(crate) fn complete_failover(&self) {
        let mut topology = self.topology.write().unwrap();
        topology.failing_over_from = None;
        topology.next_write_region = None;
    }
}

fn bad_request(message: String) -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(crate::error::CosmosStatus::new(
            azure_core::http::StatusCode::BadRequest,
        ))
        .with_message(message)
        .build()
}

/// Rejection for adding a region the account already has, matching the
/// service's rejection of a redundant add-read-region request.
pub(crate) fn already_present(region_name: &str) -> crate::error::CosmosError {
    bad_request(format!(
        "region '{region_name}' is already part of the account"
    ))
}

/// A virtual region with a name and gateway URL.
#[derive(Clone, Debug)]
pub struct VirtualRegion {
    name: String,
    gateway_url: Url,
    #[cfg(feature = "__internal_in_memory_emulator")]
    gateway_v2_url: Option<Url>,
    region_id: Option<u64>,
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
            #[cfg(feature = "__internal_in_memory_emulator")]
            gateway_v2_url: None,
            region_id: None,
        }
    }

    /// Configures the Gateway V2 thin-client endpoint for this region.
    #[cfg(feature = "__internal_in_memory_emulator")]
    #[doc(hidden)]
    pub fn with_gateway_v2_url(mut self, url: Url) -> Self {
        self.gateway_v2_url = Some(url);
        self
    }

    /// Creates a new region with an explicit region ID.
    pub fn with_region_id(mut self, id: u64) -> Self {
        self.region_id = Some(id);
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn gateway_url(&self) -> &Url {
        &self.gateway_url
    }

    /// Returns the Gateway V2 thin-client endpoint when hosted externally.
    #[cfg(feature = "__internal_in_memory_emulator")]
    #[doc(hidden)]
    pub(crate) fn gateway_v2_url(&self) -> Option<&Url> {
        self.gateway_v2_url.as_ref()
    }

    pub fn region_id(&self) -> u64 {
        self.region_id
            .expect("VirtualAccountConfig assigns every region an ID")
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
    use super::*;

    fn region(name: &str) -> VirtualRegion {
        VirtualRegion::new(
            name,
            Url::parse(&format!(
                "https://{}.emulator.local",
                name.to_ascii_lowercase()
            ))
            .unwrap(),
        )
    }

    #[test]
    fn assigns_region_ids_by_position_when_omitted() {
        let config = VirtualAccountConfig::new(vec![region("East"), region("West")]).unwrap();
        assert_eq!(config.active_regions()[0].region_id(), 0);
        assert_eq!(config.active_regions()[1].region_id(), 1);
    }

    #[test]
    fn preserves_explicit_zero_for_non_first_region() {
        let config = VirtualAccountConfig::new(vec![
            region("East").with_region_id(1),
            region("West").with_region_id(0),
        ])
        .unwrap();
        assert_eq!(config.active_regions()[0].region_id(), 1);
        assert_eq!(config.active_regions()[1].region_id(), 0);
    }

    #[test]
    fn rejects_duplicate_effective_region_ids() {
        let error =
            VirtualAccountConfig::new(vec![region("East").with_region_id(1), region("West")])
                .unwrap_err();
        assert_eq!(
            error.status().status_code(),
            azure_core::http::StatusCode::BadRequest
        );
    }

    #[test]
    fn rejected_hidden_readd_preserves_retired_region_id() {
        let config = VirtualAccountConfig::new(vec![region("East"), region("West")]).unwrap();
        let original_id = config.region_id_for("West");
        config.remove_region("West").unwrap();
        config.topology.write().unwrap().next_add_operation_id = u64::MAX;

        let error = config.add_region(region("West"), true).unwrap_err();
        assert_eq!(
            error.status().status_code(),
            azure_core::http::StatusCode::BadRequest
        );

        let topology = config.topology.read().unwrap();
        let retired = topology
            .retired
            .iter()
            .find(|region| region.name() == "West")
            .expect("rejected re-add must preserve the retired region");
        assert_eq!(retired.region_id(), original_id);
    }

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

    #[test]
    fn write_mode_change_clears_failover_transition() {
        let announced = VirtualAccountConfig::new(vec![region("East"), region("West")]).unwrap();
        announced.announce_failover("West").unwrap();
        announced.set_write_mode(WriteMode::Multi);
        announced.set_write_mode(WriteMode::Single);

        let snapshot = announced.topology_snapshot();
        assert_eq!(
            snapshot
                .writable(true)
                .iter()
                .map(|region| region.name())
                .collect::<Vec<_>>(),
            vec!["East"]
        );
        assert_eq!(snapshot.next_write_region, None);
        assert_eq!(snapshot.failing_over_from, None);

        let begun = VirtualAccountConfig::new(vec![region("East"), region("West")]).unwrap();
        begun.begin_failover("West").unwrap();
        begun.set_write_mode(WriteMode::Multi);
        begun.set_write_mode(WriteMode::Single);

        let snapshot = begun.topology_snapshot();
        assert_eq!(
            snapshot
                .writable(true)
                .iter()
                .map(|region| region.name())
                .collect::<Vec<_>>(),
            vec!["West"]
        );
        assert_eq!(snapshot.next_write_region, None);
        assert_eq!(snapshot.failing_over_from, None);
    }

    #[test]
    fn offline_failover_hoists_first_online_priority() {
        let config =
            VirtualAccountConfig::new(vec![region("East"), region("West"), region("Central")])
                .unwrap();
        config.set_region_offline("West").unwrap();
        config.set_region_offline("East").unwrap();

        let topology = config.topology.read().unwrap();
        assert_eq!(topology.write_region, "Central");
        assert_eq!(topology.priority_order, vec!["Central", "West", "East"]);
    }
}
