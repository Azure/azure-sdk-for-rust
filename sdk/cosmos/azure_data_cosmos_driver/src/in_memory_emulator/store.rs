// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! In-memory document store with multi-region support.

use std::collections::{BTreeMap, HashMap, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use super::config::{ContainerConfig, VirtualAccountConfig};
use super::epk::Epk;
use super::rid::RidGenerator;
use super::session::SessionState;
use crate::models::PartitionKeyDefinition;

/// Top-level emulator store holding all regions.
pub struct EmulatorStore {
    config: VirtualAccountConfig,
    rid_generator: RidGenerator,
    regions: RwLock<HashMap<String, RegionStore>>,
}

impl EmulatorStore {
    /// Creates a new store from the given account configuration.
    pub(crate) fn new(config: VirtualAccountConfig) -> Arc<Self> {
        let mut regions = HashMap::new();
        for region in config.regions() {
            regions.insert(region.name().to_string(), RegionStore::new());
        }

        Arc::new(Self {
            config,
            rid_generator: RidGenerator::new(),
            regions: RwLock::new(regions),
        })
    }

    pub fn config(&self) -> &VirtualAccountConfig {
        &self.config
    }

    pub(crate) fn rid_generator(&self) -> &RidGenerator {
        &self.rid_generator
    }

    /// Returns a reference to the region store for the given region name.
    pub(crate) fn region(&self, name: &str) -> Option<RegionStoreRef<'_>> {
        let regions = self.regions.read().unwrap();
        if regions.contains_key(name) {
            Some(RegionStoreRef {
                store: self,
                name: name.to_string(),
            })
        } else {
            None
        }
    }

    /// Creates a database in all regions.
    pub fn create_database(&self, db_id: &str) {
        self.create_database_internal(db_id);
    }

    /// Creates a database in all regions, returning metadata for internal use.
    pub(crate) fn create_database_internal(&self, db_id: &str) -> DatabaseMetadata {
        let (numeric_id, rid) = self.rid_generator.next_database_rid();
        let ts = current_timestamp();
        let meta = DatabaseMetadata {
            id: db_id.to_string(),
            rid: rid.clone(),
            numeric_id,
            ts,
            self_link: format!("dbs/{}/", rid),
            etag: new_etag(),
        };

        let regions = self.regions.read().unwrap();
        for region in regions.values() {
            let mut dbs = region.databases.write().unwrap();
            dbs.insert(db_id.to_string(), meta.clone());
        }
        meta
    }

    /// Creates a container in all regions with default partition count.
    pub fn create_container(&self, db_id: &str, coll_id: &str, pk_def: PartitionKeyDefinition) {
        self.create_container_with_config_internal(
            db_id,
            coll_id,
            pk_def,
            ContainerConfig::default(),
        );
    }

    /// Creates a container in all regions with custom config.
    pub fn create_container_with_config(
        &self,
        db_id: &str,
        coll_id: &str,
        pk_def: PartitionKeyDefinition,
        config: ContainerConfig,
    ) {
        self.create_container_with_config_internal(db_id, coll_id, pk_def, config);
    }

    /// Creates a container in all regions, returning metadata for internal use.
    pub(crate) fn create_container_with_config_internal(
        &self,
        db_id: &str,
        coll_id: &str,
        pk_def: PartitionKeyDefinition,
        config: ContainerConfig,
    ) -> ContainerMetadata {
        let db_meta = {
            let regions = self.regions.read().unwrap();
            let region = regions.values().next().unwrap();
            let dbs = region.databases.read().unwrap();
            dbs.get(db_id).cloned()
        };

        let db_meta = db_meta.expect("database must exist before creating a container");
        let (numeric_coll_id, coll_rid) =
            self.rid_generator.next_collection_rid(db_meta.numeric_id);
        let ts = current_timestamp();
        let meta = ContainerMetadata {
            id: coll_id.to_string(),
            rid: coll_rid.clone(),
            db_rid: db_meta.rid.clone(),
            numeric_db_id: db_meta.numeric_id,
            numeric_coll_id,
            ts,
            self_link: format!("dbs/{}/colls/{}/", db_meta.rid, coll_rid),
            etag: new_etag(),
            partition_key: pk_def,
            partition_count: config.partition_count(),
            provisioned_throughput_ru: config.provisioned_throughput_ru(),
        };

        let state = ContainerState::new(&meta, &self.rid_generator);

        let regions = self.regions.read().unwrap();
        for region in regions.values() {
            let mut containers = region.containers.write().unwrap();
            containers.insert(
                (db_id.to_string(), coll_id.to_string()),
                state.clone_for_region(&self.rid_generator, &meta),
            );
        }

        // Store the "template" state in case we need to reference it
        drop(regions);
        let _ = state;

        meta
    }

    /// Forces the next read in the given region for the specified partition key
    /// to return 404/1002 (ReadSessionNotAvailable), then resets.
    pub fn force_session_not_available(&self, region: &str, partition_key_json: &str) {
        let regions = self.regions.read().unwrap();
        if let Some(region_store) = regions.get(region) {
            let containers = region_store.containers.read().unwrap();
            let pk_components = super::epk::parse_partition_key_header(partition_key_json);
            for state in containers.values() {
                let epk = super::epk::compute_epk(
                    &pk_components,
                    state.metadata.partition_key.kind(),
                    state.metadata.partition_key.version(),
                );
                if let Some(partition) = state.find_partition(&epk) {
                    partition.session_state.set_force_unavailable();
                }
            }
        }
    }

    /// Pauses replication TO the given target region.
    pub fn pause_replication(&self, target_region: &str) {
        let regions = self.regions.read().unwrap();
        if let Some(region_store) = regions.get(target_region) {
            region_store.paused.store(true, Ordering::SeqCst);
        }
    }

    /// Resumes replication TO the given target region, draining accumulated writes.
    pub fn resume_replication(&self, target_region: &str) {
        let regions = self.regions.read().unwrap();
        if let Some(region_store) = regions.get(target_region) {
            region_store.paused.store(false, Ordering::SeqCst);

            // Drain the replication buffer
            let mut buffer = region_store.replication_buffer.write().unwrap();
            while let Some(pending) = buffer.pop_front() {
                let containers = region_store.containers.read().unwrap();
                let key = (pending.db_id.clone(), pending.coll_id.clone());
                if let Some(state) = containers.get(&key) {
                    if let Some(partition) = state.find_partition(&pending.doc.epk) {
                        let mut docs = partition.documents.write().unwrap();
                        let logical = docs.entry(pending.doc.epk.clone()).or_default();
                        if pending.is_delete {
                            logical.remove(&pending.doc.id);
                        } else {
                            logical.insert(pending.doc.id.clone(), pending.doc.clone());
                        }
                        partition.lsn.fetch_max(pending.doc.lsn, Ordering::SeqCst);
                    }
                }
            }
        }
    }

    /// Triggers replication of a document write to all other regions.
    pub(crate) fn replicate(
        self: &Arc<Self>,
        source_region: &str,
        db_id: &str,
        coll_id: &str,
        doc: &StoredDocument,
        is_delete: bool,
    ) {
        let regions = self.regions.read().unwrap();
        let region_names: Vec<String> = regions
            .keys()
            .filter(|n| *n != source_region)
            .cloned()
            .collect();
        drop(regions);

        for target_name in region_names {
            let repl_config = self.config.replication_for(source_region, &target_name);
            let delay = repl_config.sample_delay();

            let store = Arc::clone(self);
            let target = target_name.clone();
            let db = db_id.to_string();
            let coll = coll_id.to_string();
            let document = doc.clone();

            if delay.is_zero() {
                // Immediate replication — no async needed
                store.apply_replication(&target, &db, &coll, &document, is_delete);
            } else {
                // Async delayed replication
                let store_clone = store;
                tokio::spawn(async move {
                    tokio::time::sleep(delay).await;
                    store_clone.apply_replication(&target, &db, &coll, &document, is_delete);
                });
            }
        }
    }

    /// Applies a replicated document to a target region.
    fn apply_replication(
        &self,
        target_region: &str,
        db_id: &str,
        coll_id: &str,
        doc: &StoredDocument,
        is_delete: bool,
    ) {
        let regions = self.regions.read().unwrap();
        if let Some(region_store) = regions.get(target_region) {
            if region_store.paused.load(Ordering::SeqCst) {
                // Enqueue to buffer
                let mut buffer = region_store.replication_buffer.write().unwrap();
                buffer.push_back(PendingReplication {
                    db_id: db_id.to_string(),
                    coll_id: coll_id.to_string(),
                    doc: doc.clone(),
                    is_delete,
                });
                return;
            }

            let containers = region_store.containers.read().unwrap();
            let key = (db_id.to_string(), coll_id.to_string());
            if let Some(state) = containers.get(&key) {
                if let Some(partition) = state.find_partition(&doc.epk) {
                    let mut docs = partition.documents.write().unwrap();
                    let logical = docs.entry(doc.epk.clone()).or_default();
                    if is_delete {
                        logical.remove(&doc.id);
                    } else {
                        logical.insert(doc.id.clone(), doc.clone());
                    }
                    partition.lsn.fetch_max(doc.lsn, Ordering::SeqCst);
                }
            }
        }
    }
}

impl std::fmt::Debug for EmulatorStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EmulatorStore")
            .field("regions", &"...")
            .finish()
    }
}

/// A reference-counted handle to a specific region's store.
pub(crate) struct RegionStoreRef<'a> {
    store: &'a EmulatorStore,
    name: String,
}

impl<'a> RegionStoreRef<'a> {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Reads a database metadata.
    pub fn get_database(&self, db_id: &str) -> Option<DatabaseMetadata> {
        let regions = self.store.regions.read().unwrap();
        let region = regions.get(&self.name)?;
        let dbs = region.databases.read().unwrap();
        dbs.get(db_id).cloned()
    }

    /// Reads a container state.
    pub fn get_container(&self, db_id: &str, coll_id: &str) -> Option<ContainerStateSnapshot> {
        let regions = self.store.regions.read().unwrap();
        let region = regions.get(&self.name)?;
        let containers = region.containers.read().unwrap();
        let key = (db_id.to_string(), coll_id.to_string());
        containers.get(&key).map(|s| ContainerStateSnapshot {
            metadata: s.metadata.clone(),
            partitions_ref: s as *const ContainerState,
        })
    }

    /// Executes a closure against the container's physical partitions.
    pub fn with_container<R>(
        &self,
        db_id: &str,
        coll_id: &str,
        f: impl FnOnce(&ContainerState) -> R,
    ) -> Option<R> {
        let regions = self.store.regions.read().unwrap();
        let region = regions.get(&self.name)?;
        let containers = region.containers.read().unwrap();
        let key = (db_id.to_string(), coll_id.to_string());
        containers.get(&key).map(f)
    }

    /// Creates a database in this region only (for HTTP-based creation).
    #[allow(dead_code)]
    pub fn create_database_local(&self, meta: &DatabaseMetadata) {
        let regions = self.store.regions.read().unwrap();
        if let Some(region) = regions.get(&self.name) {
            let mut dbs = region.databases.write().unwrap();
            dbs.insert(meta.id.clone(), meta.clone());
        }
    }

    /// Creates a container in this region only.
    #[allow(dead_code)]
    pub fn create_container_local(&self, db_id: &str, state: ContainerState) {
        let regions = self.store.regions.read().unwrap();
        if let Some(region) = regions.get(&self.name) {
            let mut containers = region.containers.write().unwrap();
            containers.insert((db_id.to_string(), state.metadata.id.clone()), state);
        }
    }

    /// Checks if a database exists.
    pub fn database_exists(&self, db_id: &str) -> bool {
        let regions = self.store.regions.read().unwrap();
        let region = match regions.get(&self.name) {
            Some(r) => r,
            None => return false,
        };
        let dbs = region.databases.read().unwrap();
        dbs.contains_key(db_id)
    }

    /// Checks if a container exists.
    pub fn container_exists(&self, db_id: &str, coll_id: &str) -> bool {
        let regions = self.store.regions.read().unwrap();
        let region = match regions.get(&self.name) {
            Some(r) => r,
            None => return false,
        };
        let containers = region.containers.read().unwrap();
        containers.contains_key(&(db_id.to_string(), coll_id.to_string()))
    }

    /// Deletes a database and all its containers in this region.
    pub fn delete_database(&self, db_id: &str) -> bool {
        let regions = self.store.regions.read().unwrap();
        let region = match regions.get(&self.name) {
            Some(r) => r,
            None => return false,
        };

        let removed = {
            let mut dbs = region.databases.write().unwrap();
            dbs.remove(db_id).is_some()
        };

        if removed {
            // Cascade: remove all containers in this database
            let mut containers = region.containers.write().unwrap();
            containers.retain(|(db, _), _| db != db_id);
        }

        removed
    }

    /// Deletes a container in this region.
    pub fn delete_container(&self, db_id: &str, coll_id: &str) -> bool {
        let regions = self.store.regions.read().unwrap();
        let region = match regions.get(&self.name) {
            Some(r) => r,
            None => return false,
        };
        let mut containers = region.containers.write().unwrap();
        containers
            .remove(&(db_id.to_string(), coll_id.to_string()))
            .is_some()
    }
}

/// Per-region store.
pub(crate) struct RegionStore {
    pub databases: RwLock<HashMap<String, DatabaseMetadata>>,
    pub containers: RwLock<HashMap<(String, String), ContainerState>>,
    pub paused: AtomicBool,
    pub replication_buffer: RwLock<VecDeque<PendingReplication>>,
}

impl RegionStore {
    fn new() -> Self {
        Self {
            databases: RwLock::new(HashMap::new()),
            containers: RwLock::new(HashMap::new()),
            paused: AtomicBool::new(false),
            replication_buffer: RwLock::new(VecDeque::new()),
        }
    }
}

/// Pending replication entry.
#[derive(Clone)]
pub(crate) struct PendingReplication {
    pub db_id: String,
    pub coll_id: String,
    pub doc: StoredDocument,
    pub is_delete: bool,
}

/// Database metadata.
#[derive(Clone, Debug)]
pub(crate) struct DatabaseMetadata {
    pub id: String,
    pub rid: String,
    pub numeric_id: u32,
    pub ts: u64,
    pub self_link: String,
    pub etag: String,
}

/// Container metadata.
#[derive(Clone, Debug)]
pub(crate) struct ContainerMetadata {
    pub id: String,
    pub rid: String,
    #[allow(dead_code)]
    pub db_rid: String,
    pub numeric_db_id: u32,
    pub numeric_coll_id: u32,
    pub ts: u64,
    pub self_link: String,
    pub etag: String,
    pub partition_key: PartitionKeyDefinition,
    pub partition_count: u32,
    pub provisioned_throughput_ru: Option<u32>,
}

/// A container's state including metadata and physical partitions.
pub(crate) struct ContainerState {
    pub metadata: ContainerMetadata,
    pub physical_partitions: Vec<PhysicalPartition>,
    pub next_partition_id: AtomicU32,
}

/// Snapshot of container metadata (without borrowing the lock).
#[allow(dead_code)]
pub(crate) struct ContainerStateSnapshot {
    pub metadata: ContainerMetadata,
    partitions_ref: *const ContainerState,
}

// Safety: ContainerStateSnapshot only stores metadata (Clone) and a pointer
// that is only used while the parent lock is held.
unsafe impl Send for ContainerStateSnapshot {}
unsafe impl Sync for ContainerStateSnapshot {}

impl ContainerState {
    pub(crate) fn new(meta: &ContainerMetadata, rid_gen: &RidGenerator) -> Self {
        let partitions = create_partitions(meta, rid_gen);
        let next_id = partitions.iter().map(|p| p.id).max().unwrap_or(0) + 1;
        Self {
            metadata: meta.clone(),
            physical_partitions: partitions,
            next_partition_id: AtomicU32::new(next_id),
        }
    }

    /// Creates a clone of this state for another region (new partitions, same layout).
    fn clone_for_region(&self, rid_gen: &RidGenerator, meta: &ContainerMetadata) -> Self {
        let partitions = create_partitions(meta, rid_gen);
        let next_id = partitions.iter().map(|p| p.id).max().unwrap_or(0) + 1;
        Self {
            metadata: meta.clone(),
            physical_partitions: partitions,
            next_partition_id: AtomicU32::new(next_id),
        }
    }

    /// Finds the physical partition responsible for the given EPK.
    pub fn find_partition(&self, epk: &Epk) -> Option<&PhysicalPartition> {
        self.physical_partitions
            .iter()
            .find(|p| p.contains_epk(epk))
    }

    /// Allocates the next partition ID.
    pub fn next_partition_id(&self) -> u32 {
        self.next_partition_id.fetch_add(1, Ordering::SeqCst)
    }
}

/// A physical partition covering a range of EPK hash values.
pub(crate) struct PhysicalPartition {
    pub id: u32,
    pub epk_min: Epk,
    pub epk_max: Epk,
    pub lsn: AtomicU64,
    pub vector_clock_version: AtomicU64,
    pub documents: RwLock<BTreeMap<Epk, BTreeMap<String, StoredDocument>>>,
    pub session_state: SessionState,
    pub rid: String,
    pub rid_prefix: u32,
    pub throughput_fraction: f64,
    pub parents: Vec<u32>,
    pub locked_until: RwLock<Option<tokio::time::Instant>>,
    pub throughput_tracker: Option<ThroughputTracker>,
}

impl PhysicalPartition {
    /// Checks if an EPK falls within this partition's range [min_inclusive, max_exclusive).
    pub fn contains_epk(&self, epk: &Epk) -> bool {
        *epk >= self.epk_min && *epk < self.epk_max
    }

    /// Returns the current LSN.
    pub fn current_lsn(&self) -> u64 {
        self.lsn.load(Ordering::SeqCst)
    }

    /// Advances LSN by 1 and returns the new value.
    pub fn advance_lsn(&self) -> u64 {
        self.lsn.fetch_add(1, Ordering::SeqCst) + 1
    }

    /// Returns the current vector clock version.
    pub fn current_version(&self) -> u64 {
        self.vector_clock_version.load(Ordering::SeqCst)
    }

    /// Returns whether this partition is currently locked (split/merge in progress).
    pub fn is_locked(&self) -> bool {
        let guard = self.locked_until.read().unwrap();
        match *guard {
            Some(until) => tokio::time::Instant::now() < until,
            None => false,
        }
    }
}

/// A stored document with system properties.
#[derive(Clone, Debug)]
pub(crate) struct StoredDocument {
    pub body: serde_json::Value,
    pub id: String,
    pub rid: String,
    pub etag: String,
    pub ts: u64,
    pub self_link: String,
    pub lsn: u64,
    pub epk: Epk,
}

/// Partition key range metadata (exposed via PKRanges feed).
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub(crate) struct PartitionKeyRangeMetadata {
    pub id: String,
    pub rid: String,
    pub self_link: String,
    pub etag: String,
    pub ts: u64,
    pub lsn: u64,
    pub min_inclusive: String,
    pub max_exclusive: String,
    pub status: String,
    pub parents: Vec<String>,
    pub rid_prefix: u32,
    pub throughput_fraction: f64,
}

/// Creates physical partitions for a container by dividing the EPK space equally.
fn create_partitions(meta: &ContainerMetadata, rid_gen: &RidGenerator) -> Vec<PhysicalPartition> {
    let n = meta.partition_count;
    let mut partitions = Vec::with_capacity(n as usize);

    // For EPK space division, we use a simple scheme:
    // Divide the hex space [00..00, FF..FF) into N equal ranges.
    // We use 32-char hex strings (16 bytes) as boundary markers.
    let boundaries = compute_partition_boundaries(n);

    for i in 0..n {
        let min = if i == 0 {
            Epk::MIN
        } else {
            Epk::new(boundaries[i as usize].clone())
        };
        let max = if i == n - 1 {
            Epk::max()
        } else {
            Epk::new(boundaries[(i + 1) as usize].clone())
        };

        let rid = rid_gen.next_pkrange_rid(meta.numeric_db_id, meta.numeric_coll_id, i);

        let per_partition_ru = meta.provisioned_throughput_ru.map(|total| total / n);

        partitions.push(PhysicalPartition {
            id: i,
            epk_min: min,
            epk_max: max,
            lsn: AtomicU64::new(0),
            vector_clock_version: AtomicU64::new(0),
            documents: RwLock::new(BTreeMap::new()),
            session_state: SessionState::new(),
            rid,
            rid_prefix: i,
            throughput_fraction: 1.0 / n as f64,
            parents: Vec::new(),
            locked_until: RwLock::new(None),
            throughput_tracker: per_partition_ru.map(ThroughputTracker::new),
        });
    }

    partitions
}

/// Computes N+1 EPK boundary strings that divide the 128-bit hex space into N equal ranges.
fn compute_partition_boundaries(n: u32) -> Vec<String> {
    let mut boundaries = Vec::with_capacity((n + 1) as usize);
    // We work with u128 for the full hash space
    // Min = 0, Max = 0x3FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF (after bit clearing)
    // But EPK space is really [0, FF) where FF is the exclusive upper bound
    // We use 128-bit space: [0, 2^128)
    let total: u128 = u128::MAX;
    let step = total / n as u128;

    for i in 0..=n {
        if i == 0 {
            boundaries.push(String::new()); // Epk::MIN
        } else if i == n {
            boundaries.push("FF".to_string()); // Epk::max()
        } else {
            let boundary = step * i as u128;
            boundaries.push(format!("{:032X}", boundary));
        }
    }

    boundaries
}

/// Returns the current Unix timestamp in seconds.
pub(crate) fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Generates a new quoted UUID ETag.
pub(crate) fn new_etag() -> String {
    format!("\"{}\"", uuid::Uuid::new_v4())
}

/// Tracks RU consumption per second for throughput throttling.
pub(crate) struct ThroughputTracker {
    provisioned_ru: u32,
    /// Consumed RU in the current window, stored as RU * 100 to avoid floats in atomics.
    consumed_centiru: AtomicU64,
    /// Window start timestamp in seconds.
    window_start: AtomicU64,
}

impl ThroughputTracker {
    pub fn new(provisioned_ru: u32) -> Self {
        Self {
            provisioned_ru,
            consumed_centiru: AtomicU64::new(0),
            window_start: AtomicU64::new(current_timestamp()),
        }
    }

    /// Attempts to consume `charge` RU. Returns `Ok(())` if within budget,
    /// or `Err(retry_after_ms)` if throttled.
    pub fn try_consume(&self, charge: f64) -> Result<(), u64> {
        let now = current_timestamp();
        let window = self.window_start.load(Ordering::SeqCst);

        // Reset window if we've moved to a new second
        if now > window {
            self.consumed_centiru.store(0, Ordering::SeqCst);
            self.window_start.store(now, Ordering::SeqCst);
        }

        let charge_centiru = (charge * 100.0) as u64;
        let prev = self
            .consumed_centiru
            .fetch_add(charge_centiru, Ordering::SeqCst);
        let budget_centiru = (self.provisioned_ru as u64) * 100;

        if prev + charge_centiru > budget_centiru {
            // Over budget — undo the add and return retry-after
            self.consumed_centiru
                .fetch_sub(charge_centiru, Ordering::SeqCst);
            Err(1000) // Retry after ~1 second (next window)
        } else {
            Ok(())
        }
    }
}

// --- Split / Merge ---

impl EmulatorStore {
    /// Splits a physical partition into two child partitions.
    ///
    /// During `min_lock_duration` (plus doc redistribution time), operations on the
    /// partition return 410/1007. After: parent is replaced by two children with the
    /// EPK range split in half.
    /// Child LSN = parent_lsn + 1, vector_clock_version is preserved (split does NOT change it).
    pub fn split_partition(
        self: &Arc<Self>,
        db_id: &str,
        coll_id: &str,
        partition_id: u32,
        min_lock_duration: Duration,
    ) {
        // Lock the partition in all regions
        {
            let regions = self.regions.read().unwrap();
            for region in regions.values() {
                let containers = region.containers.read().unwrap();
                let key = (db_id.to_string(), coll_id.to_string());
                if let Some(state) = containers.get(&key) {
                    if let Some(p) = state
                        .physical_partitions
                        .iter()
                        .find(|p| p.id == partition_id)
                    {
                        let mut lock = p.locked_until.write().unwrap();
                        *lock = Some(
                            tokio::time::Instant::now()
                                + min_lock_duration
                                + Duration::from_secs(3600),
                        );
                    }
                }
            }
        }

        let store = Arc::clone(self);
        let db = db_id.to_string();
        let coll = coll_id.to_string();

        tokio::spawn(async move {
            if !min_lock_duration.is_zero() {
                tokio::time::sleep(min_lock_duration).await;
            }
            // execute_split does the actual doc redistribution under the lock,
            // then clears locked_until when done
            store.execute_split(&db, &coll, partition_id);
        });
    }

    /// Performs the actual split after the lock period.
    fn execute_split(&self, db_id: &str, coll_id: &str, partition_id: u32) {
        let regions = self.regions.read().unwrap();
        for region in regions.values() {
            let mut containers = region.containers.write().unwrap();
            let key = (db_id.to_string(), coll_id.to_string());
            if let Some(state) = containers.get_mut(&key) {
                let parent_idx = match state
                    .physical_partitions
                    .iter()
                    .position(|p| p.id == partition_id)
                {
                    Some(idx) => idx,
                    None => continue,
                };

                let parent = &state.physical_partitions[parent_idx];
                let parent_lsn = parent.current_lsn();
                let parent_version = parent.current_version();
                let parent_min = parent.epk_min.clone();
                let parent_max = parent.epk_max.clone();

                // Compute midpoint
                let midpoint = compute_epk_midpoint(&parent_min, &parent_max);

                // Get new child IDs
                let child_id_1 = state.next_partition_id();
                let child_id_2 = state.next_partition_id();
                let child_lsn = parent_lsn + 1;

                // Generate RIDs for children
                let child_rid_1 = self.rid_generator.next_pkrange_rid(
                    state.metadata.numeric_db_id,
                    state.metadata.numeric_coll_id,
                    child_id_1,
                );
                let child_rid_2 = self.rid_generator.next_pkrange_rid(
                    state.metadata.numeric_db_id,
                    state.metadata.numeric_coll_id,
                    child_id_2,
                );

                // Redistribute documents
                let parent_docs = parent.documents.read().unwrap();
                let mut docs_1: BTreeMap<Epk, BTreeMap<String, StoredDocument>> = BTreeMap::new();
                let mut docs_2: BTreeMap<Epk, BTreeMap<String, StoredDocument>> = BTreeMap::new();

                for (epk, items) in parent_docs.iter() {
                    if *epk < midpoint {
                        docs_1.insert(epk.clone(), items.clone());
                    } else {
                        docs_2.insert(epk.clone(), items.clone());
                    }
                }
                drop(parent_docs);

                let n = state.physical_partitions.len() as f64 + 1.0; // +1 since we add 2 and remove 1
                let per_partition_ru = state
                    .metadata
                    .provisioned_throughput_ru
                    .map(|total| total / (n as u32));

                let child1 = PhysicalPartition {
                    id: child_id_1,
                    epk_min: parent_min,
                    epk_max: midpoint.clone(),
                    lsn: AtomicU64::new(child_lsn),
                    vector_clock_version: AtomicU64::new(parent_version),
                    documents: RwLock::new(docs_1),
                    session_state: SessionState::new(),
                    rid: child_rid_1,
                    rid_prefix: child_id_1,
                    throughput_fraction: 1.0 / n,
                    parents: vec![partition_id],
                    locked_until: RwLock::new(None),
                    throughput_tracker: per_partition_ru.map(ThroughputTracker::new),
                };

                let child2 = PhysicalPartition {
                    id: child_id_2,
                    epk_min: midpoint,
                    epk_max: parent_max,
                    lsn: AtomicU64::new(child_lsn),
                    vector_clock_version: AtomicU64::new(parent_version),
                    documents: RwLock::new(docs_2),
                    session_state: SessionState::new(),
                    rid: child_rid_2,
                    rid_prefix: child_id_2,
                    throughput_fraction: 1.0 / n,
                    parents: vec![partition_id],
                    locked_until: RwLock::new(None),
                    throughput_tracker: per_partition_ru.map(ThroughputTracker::new),
                };

                // Remove parent, add children
                state.physical_partitions.remove(parent_idx);
                state.physical_partitions.push(child1);
                state.physical_partitions.push(child2);
            }
        }
    }

    /// Merges two adjacent physical partitions into one child partition.
    ///
    /// During `min_lock_duration` (plus doc merge time), operations on both partitions
    /// return 410/1007. After: both parents are replaced by a single child.
    /// Child vector_clock_version = max(parent_versions) + 1 (merge DOES increment version).
    /// Child LSN = 1 (restarts).
    pub fn merge_partitions(
        self: &Arc<Self>,
        db_id: &str,
        coll_id: &str,
        partition_id_a: u32,
        partition_id_b: u32,
        min_lock_duration: Duration,
    ) {
        // Lock both partitions in all regions
        {
            let regions = self.regions.read().unwrap();
            for region in regions.values() {
                let containers = region.containers.read().unwrap();
                let key = (db_id.to_string(), coll_id.to_string());
                if let Some(state) = containers.get(&key) {
                    for p in &state.physical_partitions {
                        if p.id == partition_id_a || p.id == partition_id_b {
                            let mut lock = p.locked_until.write().unwrap();
                            *lock = Some(
                                tokio::time::Instant::now()
                                    + min_lock_duration
                                    + Duration::from_secs(3600),
                            );
                        }
                    }
                }
            }
        }

        let store = Arc::clone(self);
        let db = db_id.to_string();
        let coll = coll_id.to_string();

        tokio::spawn(async move {
            if !min_lock_duration.is_zero() {
                tokio::time::sleep(min_lock_duration).await;
            }
            store.execute_merge(&db, &coll, partition_id_a, partition_id_b);
        });
    }

    /// Performs the actual merge after the lock period.
    fn execute_merge(&self, db_id: &str, coll_id: &str, partition_id_a: u32, partition_id_b: u32) {
        let regions = self.regions.read().unwrap();
        for region in regions.values() {
            let mut containers = region.containers.write().unwrap();
            let key = (db_id.to_string(), coll_id.to_string());
            if let Some(state) = containers.get_mut(&key) {
                let idx_a = state
                    .physical_partitions
                    .iter()
                    .position(|p| p.id == partition_id_a);
                let idx_b = state
                    .physical_partitions
                    .iter()
                    .position(|p| p.id == partition_id_b);

                let (idx_a, idx_b) = match (idx_a, idx_b) {
                    (Some(a), Some(b)) => (a, b),
                    _ => continue,
                };

                // Determine which is "lower" (smaller EPK) and "upper"
                let (lower_idx, upper_idx) = if state.physical_partitions[idx_a].epk_min
                    < state.physical_partitions[idx_b].epk_min
                {
                    (idx_a, idx_b)
                } else {
                    (idx_b, idx_a)
                };

                let lower = &state.physical_partitions[lower_idx];
                let upper = &state.physical_partitions[upper_idx];

                let merged_min = lower.epk_min.clone();
                let merged_max = upper.epk_max.clone();
                let max_version = std::cmp::max(lower.current_version(), upper.current_version());
                let child_version = max_version + 1;

                // Merge documents
                let mut merged_docs: BTreeMap<Epk, BTreeMap<String, StoredDocument>> =
                    BTreeMap::new();
                {
                    let docs_a = lower.documents.read().unwrap();
                    let docs_b = upper.documents.read().unwrap();
                    for (epk, items) in docs_a.iter() {
                        merged_docs.insert(epk.clone(), items.clone());
                    }
                    for (epk, items) in docs_b.iter() {
                        merged_docs
                            .entry(epk.clone())
                            .or_default()
                            .extend(items.clone());
                    }
                }

                let child_id = state.next_partition_id();
                let child_rid = self.rid_generator.next_pkrange_rid(
                    state.metadata.numeric_db_id,
                    state.metadata.numeric_coll_id,
                    child_id,
                );

                let n = state.physical_partitions.len() as f64 - 1.0; // -1 since we remove 2 and add 1
                let per_partition_ru = state
                    .metadata
                    .provisioned_throughput_ru
                    .map(|total| total / (n.max(1.0) as u32));

                let child = PhysicalPartition {
                    id: child_id,
                    epk_min: merged_min,
                    epk_max: merged_max,
                    lsn: AtomicU64::new(1),
                    vector_clock_version: AtomicU64::new(child_version),
                    documents: RwLock::new(merged_docs),
                    session_state: SessionState::new(),
                    rid: child_rid,
                    rid_prefix: child_id,
                    throughput_fraction: 1.0 / n.max(1.0),
                    parents: vec![partition_id_a, partition_id_b],
                    locked_until: RwLock::new(None),
                    throughput_tracker: per_partition_ru.map(ThroughputTracker::new),
                };

                // Remove both parents (remove higher index first to avoid shifting)
                let (first_remove, second_remove) = if lower_idx > upper_idx {
                    (lower_idx, upper_idx)
                } else {
                    (upper_idx, lower_idx)
                };
                state.physical_partitions.remove(first_remove);
                state.physical_partitions.remove(second_remove);
                state.physical_partitions.push(child);
            }
        }
    }
}

/// Computes the EPK midpoint between two EPK bounds (hex strings).
fn compute_epk_midpoint(min: &Epk, max: &Epk) -> Epk {
    // Parse hex strings to u128, compute midpoint, format back
    let min_val = if min.as_str().is_empty() {
        0u128
    } else {
        u128::from_str_radix(min.as_str(), 16).unwrap_or(0)
    };

    let max_val = if max.as_str() == "FF" {
        u128::MAX
    } else {
        u128::from_str_radix(max.as_str(), 16).unwrap_or(u128::MAX)
    };

    let mid = min_val / 2 + max_val / 2;
    Epk::new(format!("{:032X}", mid))
}
