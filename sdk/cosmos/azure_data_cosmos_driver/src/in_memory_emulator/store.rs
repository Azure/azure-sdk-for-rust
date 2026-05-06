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

type SplitMergeLocks = HashMap<(String, String), Arc<async_lock::Mutex<()>>>;
/// Sentinel pkrange id used for control-plane (database/container/offer) session
/// tokens. Chosen as `u32::MAX` so it cannot collide with any user pkrange id,
/// since real Cosmos partition counts are bounded far below this value.
pub(crate) const MASTER_PARTITION_ID: u32 = u32::MAX;

/// Top-level emulator store holding all regions.
pub struct EmulatorStore {
    config: VirtualAccountConfig,
    rid_generator: RidGenerator,
    regions: RwLock<HashMap<String, Arc<RegionStore>>>,
    /// LSN counter for the "master partition" that tracks control plane operations
    /// (database/container CRUD, throughput changes). The real Cosmos DB service
    /// stores metadata in a special MasterPartition and replicates it similarly
    /// to user documents, producing LSN and session tokens for control plane
    /// responses.
    master_partition_lsn: AtomicU64,
    /// Per-(db, coll) async mutex serializing split/merge execution.
    split_merge_locks: std::sync::Mutex<SplitMergeLocks>,
    /// Tracks spawned replication tasks so tests can drain them.
    replication_tasks: std::sync::Mutex<tokio::task::JoinSet<()>>,
    /// Tracks spawned split/merge tasks separately from replication so a
    /// control-plane panic does not surface inside an unrelated point-write
    /// handler that happens to call `replicate()`.
    control_plane_tasks: std::sync::Mutex<tokio::task::JoinSet<()>>,
    /// Monotonic counter used to populate `x-ms-transport-request-id`. Every
    /// HTTP response gets a unique value so consumers correlating retries on
    /// the wire see stable identity instead of a partition LSN that drifts
    /// with mutations.
    transport_request_counter: AtomicU64,
}

impl EmulatorStore {
    /// Creates a new store from the given account configuration.
    pub(crate) fn new(config: VirtualAccountConfig) -> Arc<Self> {
        let mut regions = HashMap::new();
        for region in config.regions() {
            regions.insert(region.name().to_string(), Arc::new(RegionStore::new()));
        }

        Arc::new(Self {
            config,
            rid_generator: RidGenerator::new(),
            regions: RwLock::new(regions),
            master_partition_lsn: AtomicU64::new(0),
            split_merge_locks: std::sync::Mutex::new(HashMap::new()),
            replication_tasks: std::sync::Mutex::new(tokio::task::JoinSet::new()),
            control_plane_tasks: std::sync::Mutex::new(tokio::task::JoinSet::new()),
            transport_request_counter: AtomicU64::new(0),
        })
    }

    #[doc(hidden)]
    pub fn config(&self) -> &VirtualAccountConfig {
        &self.config
    }

    pub(crate) fn rid_generator(&self) -> &RidGenerator {
        &self.rid_generator
    }

    /// Advances the master partition LSN and returns a V1 session token for the
    /// emulator's synthetic "master" partition.
    ///
    /// The real Cosmos DB service tracks control plane metadata (databases,
    /// containers, throughput) in a special MasterPartition that is distinct
    /// from any user pkrange. We use the sentinel pkrange id [`MASTER_PARTITION_ID`]
    /// so that control-plane session tokens never collide with the data-plane
    /// pkrange-0 namespace if a downstream consumer ever merges them.
    pub(crate) fn advance_master_partition_lsn(&self) -> String {
        let lsn = self.master_partition_lsn.fetch_add(1, Ordering::SeqCst) + 1;
        super::session::SessionToken::format(MASTER_PARTITION_ID, lsn)
    }

    /// Allocates the next process-unique value for `x-ms-transport-request-id`.
    pub(crate) fn next_transport_request_id(&self) -> u64 {
        self.transport_request_counter
            .fetch_add(1, Ordering::Relaxed)
            + 1
    }

    /// Returns `Some((target_region, retry_after_ms))` if any non-`source`
    /// region currently has its replication queue full while paused. Used by
    /// write handlers to short-circuit with 429/3075 — matching the real
    /// service which fails new writes once the replication backlog is
    /// saturated rather than queuing them indefinitely.
    pub(crate) fn find_overflowed_replication_target(&self, source: &str) -> Option<(String, u64)> {
        let regions = self.regions.read().unwrap();
        for (target_name, region_store) in regions.iter() {
            if target_name == source {
                continue;
            }
            if !region_store.paused.load(Ordering::SeqCst) {
                continue;
            }
            let max = self
                .config
                .replication_for(source, target_name)
                .max_buffered_replications();
            let len = region_store.replication_buffer.read().unwrap().len();
            if len >= max {
                return Some((target_name.clone(), 100));
            }
        }
        None
    }

    /// Awaits all pending split/merge tasks. Test-only.
    ///
    /// Use this from tests instead of `tokio::time::sleep(...)` after a
    /// `split_partition` / `merge_partitions` call so the test deterministically
    /// observes the post-split topology without depending on wall-clock timing.
    /// Panics from the awaited tasks are re-raised on the current thread.
    #[doc(hidden)]
    pub async fn drain_pending_control_plane(&self) {
        let mut set = {
            let mut guard = self.control_plane_tasks.lock().unwrap();
            std::mem::replace(&mut *guard, tokio::task::JoinSet::new())
        };
        while let Some(res) = set.join_next().await {
            if let Err(e) = res {
                if e.is_panic() {
                    std::panic::resume_unwind(e.into_panic());
                }
            }
        }
    }

    /// Convenience wrapper that awaits all pending split tasks. Equivalent to
    /// `drain_pending_control_plane()` today; named separately so call sites
    /// in tests document intent.
    #[doc(hidden)]
    pub async fn wait_for_split(&self, _db: &str, _coll: &str, _partition_id: u32) {
        self.drain_pending_control_plane().await;
    }

    fn split_merge_lock(&self, db: &str, coll: &str) -> Arc<async_lock::Mutex<()>> {
        let mut map = self.split_merge_locks.lock().unwrap();
        map.entry((db.to_string(), coll.to_string()))
            .or_insert_with(|| Arc::new(async_lock::Mutex::new(())))
            .clone()
    }

    /// Awaits all pending in-flight replication tasks. Test-only.
    ///
    /// If any task panicked (typically the buffer-overflow guard inside
    /// `apply_replication`), the panic is re-raised on the current thread so
    /// the test surfaces the failure. Silently dropping the panic would let
    /// stale-state bugs hide behind a paused-but-overflowed buffer
    ///.
    #[doc(hidden)]
    pub async fn drain_pending_replications(&self) {
        let mut set = {
            let mut guard = self.replication_tasks.lock().unwrap();
            std::mem::replace(&mut *guard, tokio::task::JoinSet::new())
        };
        while let Some(res) = set.join_next().await {
            if let Err(e) = res {
                if e.is_panic() {
                    std::panic::resume_unwind(e.into_panic());
                }
            }
        }
    }

    /// Returns a reference to the region store for the given region name.
    ///
    /// The returned [`RegionStoreRef`] holds an [`Arc<RegionStore>`] directly,
    /// so subsequent reads/writes against it skip the outer regions-map lock
    /// and avoid the TOCTOU window between `contains_key` and per-method
    /// re-lookup that the previous handle had.
    pub(crate) fn region(&self, name: &str) -> Option<RegionStoreRef> {
        let regions = self.regions.read().unwrap();
        regions.get(name).map(|r| RegionStoreRef {
            region: Arc::clone(r),
        })
    }

    /// Creates a database in all regions.
    #[doc(hidden)]
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
    #[doc(hidden)]
    pub fn create_container(&self, db_id: &str, coll_id: &str, pk_def: PartitionKeyDefinition) {
        self.create_container_with_config_internal(
            db_id,
            coll_id,
            pk_def,
            ContainerConfig::default(),
        );
    }

    /// Creates a container in all regions with custom config.
    #[doc(hidden)]
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
            // Shared counter — first id allocated by split/merge will be
            // `partition_count` (one past the last initial partition id).
            next_partition_id: Arc::new(AtomicU32::new(config.partition_count())),
            pkrange_rids: Arc::new(RwLock::new(HashMap::new())),
        };

        // Each region gets its own ContainerState (own LSNs, own document
        // store) but they all share the same `ContainerMetadata`, so pkrange
        // RIDs are allocated once via `pkrange_rid_for` and reused.
        let regions = self.regions.read().unwrap();
        for region in regions.values() {
            let mut containers = region.containers.write().unwrap();
            containers.insert(
                (db_id.to_string(), coll_id.to_string()),
                ContainerState::new(&meta, &self.rid_generator),
            );
        }

        meta
    }

    /// Forces the next read in the given region against the specified
    /// `(db_id, coll_id, partition_key)` to return 404/1002
    /// (ReadSessionNotAvailable), then resets.
    ///
    /// `partition_key_json` is the JSON-encoded partition key (for example
    /// `r#"["pk1"]"#`).
    ///
    /// Returns a descriptive error when the region, database, or container is
    /// not provisioned, when the JSON partition-key header is malformed, or
    /// when no physical partition matches the EPK. Test code should treat any
    /// `Err` as a programmer mistake ( — the previous `bool`
    /// return type silently no-op'd on misuse and applied cross-container
    /// side effects).
    #[doc(hidden)]
    pub fn force_session_not_available(
        &self,
        region: &str,
        db_id: &str,
        coll_id: &str,
        partition_key_json: &str,
    ) -> azure_core::Result<()> {
        let pk_components = super::epk::parse_partition_key_header(partition_key_json)?;
        if pk_components.is_empty() {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "force_session_not_available requires a non-empty partition key",
            ));
        }
        let regions = self.regions.read().unwrap();
        let region_store = regions.get(region).ok_or_else(|| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("region '{}' is not provisioned", region),
            )
        })?;
        let containers = region_store.containers.read().unwrap();
        let key = (db_id.to_string(), coll_id.to_string());
        let state = containers.get(&key).ok_or_else(|| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!(
                    "container '{}/{}' is not provisioned in region '{}'",
                    db_id, coll_id, region
                ),
            )
        })?;
        let epk = super::epk::compute_epk(
            &pk_components,
            state.metadata.partition_key.kind(),
            state.metadata.partition_key.version(),
        );
        let partition = state.find_partition(&epk).ok_or_else(|| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!(
                    "no physical partition found for EPK {} in container '{}/{}'",
                    epk.as_str(),
                    db_id,
                    coll_id
                ),
            )
        })?;
        partition.session_state.set_force_unavailable();
        Ok(())
    }

    /// Pauses replication TO the given target region.
    #[doc(hidden)]
    pub fn pause_replication(&self, target_region: &str) {
        let regions = self.regions.read().unwrap();
        if let Some(region_store) = regions.get(target_region) {
            region_store.paused.store(true, Ordering::SeqCst);
        }
    }

    /// Resumes replication TO the given target region, draining accumulated writes.
    #[doc(hidden)]
    pub fn resume_replication(&self, target_region: &str) {
        let regions = self.regions.read().unwrap();
        if let Some(region_store) = regions.get(target_region) {
            region_store.paused.store(false, Ordering::SeqCst);

            // Drain the replication buffer.
            //
            // The buffer is FIFO across **all** source regions, so for
            // multi-master accounts the arrival order does not necessarily
            // match per-(epk, id) causal order. Sort by (db, coll, epk, id, ts,
            // lsn) before applying so the LWW comparison in `apply_replication`
            // sees mutations in a stable order — without this, a paused
            // multi-master target could apply a stale write after a fresh one
            // and the LWW guard would still be observed as correct only by
            // accident.
            let mut buffer = region_store.replication_buffer.write().unwrap();
            let mut pending: Vec<PendingReplication> = buffer.drain(..).collect();
            // Sort key includes `source_region` as a final tiebreaker so that
            // two entries from different source regions colliding on
            // `(db, coll, epk, id, ts, lsn)` get a deterministic, stable
            // order — input/FIFO order across mixed sources is exactly the
            // case the sort is meant to neutralize.
            pending.sort_by(|a, b| {
                (
                    &a.db_id,
                    &a.coll_id,
                    &a.doc.epk,
                    &a.doc.id,
                    a.doc.ts,
                    a.doc.lsn,
                    &a.source_region,
                )
                    .cmp(&(
                        &b.db_id,
                        &b.coll_id,
                        &b.doc.epk,
                        &b.doc.id,
                        b.doc.ts,
                        b.doc.lsn,
                        &b.source_region,
                    ))
            });
            for entry in pending {
                let containers = region_store.containers.read().unwrap();
                let key = (entry.db_id.clone(), entry.coll_id.clone());
                if let Some(state) = containers.get(&key) {
                    if let Some(partition) = state.find_partition(&entry.doc.epk) {
                        let mut docs = partition.documents.write().unwrap();
                        let logical = docs.entry(entry.doc.epk.clone()).or_default();
                        let should_apply = match logical.get(&entry.doc.id) {
                            None => true,
                            Some(existing) => {
                                (entry.doc.ts, entry.doc.lsn) > (existing.ts, existing.lsn)
                            }
                        };
                        if should_apply {
                            if entry.is_delete {
                                logical.remove(&entry.doc.id);
                            } else {
                                logical.insert(entry.doc.id.clone(), entry.doc.clone());
                            }
                        }
                        partition.lsn.fetch_max(entry.doc.lsn, Ordering::SeqCst);
                        partition.local_lsn.fetch_add(1, Ordering::SeqCst);
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
        // Reap any replication tasks that have already finished so the JoinSet
        // does not grow unboundedly across long-running tests with delayed
        // replication. `try_join_next` is non-blocking and returns immediately
        // when no task is ready.
        //
        // If a reaped task panicked (e.g. the buffer-overflow `assert!` in
        // `apply_replication`), resume-unwind on the current thread so the
        // failure surfaces in the test rather than being silently swallowed.
        // Without this, a panic in a spawned replication task would only be
        // observable if a test happened to call `drain_pending_replications`
        // *and* iterate the join results — which `drain` itself does not
        //.
        {
            let mut set = self.replication_tasks.lock().unwrap();
            while let Some(res) = set.try_join_next() {
                if let Err(e) = res {
                    if e.is_panic() {
                        std::panic::resume_unwind(e.into_panic());
                    }
                }
            }
        }

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

            let source = source_region.to_string();
            if delay.is_zero() {
                // Immediate replication — no async needed
                store.apply_replication(&target, &source, &db, &coll, &document, is_delete);
            } else {
                // Async delayed replication
                let store_clone = store;
                self.replication_tasks.lock().unwrap().spawn(async move {
                    tokio::time::sleep(delay).await;
                    store_clone
                        .apply_replication(&target, &source, &db, &coll, &document, is_delete);
                });
            }
        }
    }

    /// Applies a replicated document to a target region.
    fn apply_replication(
        &self,
        target_region: &str,
        source_region: &str,
        db_id: &str,
        coll_id: &str,
        doc: &StoredDocument,
        is_delete: bool,
    ) {
        let regions = self.regions.read().unwrap();
        if let Some(region_store) = regions.get(target_region) {
            if region_store.paused.load(Ordering::SeqCst) {
                // Enqueue to the paused region's buffer. If the queue is at
                // capacity the entry is dropped with a warning rather than
                // panicking — write handlers proactively short-circuit
                // *future* writes with 429/3075 once any target's queue
                // saturates (see `find_overflowed_replication_target`),
                // mirroring the real service's
                // `RetryWith` / `ReplicaTooMuchTimeBehind` behavior. The
                // drop-on-overflow here is the safety net for the racy case
                // where multiple writers pass the pre-check before any of
                // them lands in the buffer.
                let max = self
                    .config
                    .replication_for(source_region, target_region)
                    .max_buffered_replications();
                let mut buffer = region_store.replication_buffer.write().unwrap();
                if buffer.len() >= max {
                    tracing::warn!(
                        target_region = target_region,
                        source_region = source_region,
                        db_id = db_id,
                        coll_id = coll_id,
                        max = max,
                        "in-memory emulator: replication buffer full; dropping entry (write handlers will return 429/3075 until the buffer drains)",
                    );
                    return;
                }
                buffer.push_back(PendingReplication {
                    db_id: db_id.to_string(),
                    coll_id: coll_id.to_string(),
                    source_region: source_region.to_string(),
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
                    // Last-Writer-Wins on (_ts, lsn): if there is already a
                    // record for this id with a strictly newer timestamp, or
                    // the same timestamp and a strictly higher lsn, the
                    // incoming replicated mutation is stale and must be
                    // dropped. This honors the conflictResolutionPolicy
                    // advertised in container metadata
                    // (LastWriterWins on /_ts) and prevents an out-of-order
                    // arrival (paused-then-resumed buffer, multi-master
                    // concurrent writes) from clobbering newer state.
                    let should_apply = match logical.get(&doc.id) {
                        None => true,
                        Some(existing) => (doc.ts, doc.lsn) > (existing.ts, existing.lsn),
                    };
                    if should_apply {
                        if is_delete {
                            logical.remove(&doc.id);
                        } else {
                            logical.insert(doc.id.clone(), doc.clone());
                        }
                    }
                    // Always advance the partition's global LSN watermark; the
                    // LSN reflects the replication stream, not which write
                    // ultimately won the LWW comparison.
                    partition.lsn.fetch_max(doc.lsn, Ordering::SeqCst);
                    // Bump local LSN at this region (target replica): this is
                    // the count of mutations applied here, regardless of
                    // origin.
                    partition.local_lsn.fetch_add(1, Ordering::SeqCst);
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
///
/// Holds an [`Arc<RegionStore>`] directly so each access only takes the
/// per-resource (databases / containers) lock — no outer regions-map lock
/// per call.
pub(crate) struct RegionStoreRef {
    region: Arc<RegionStore>,
}

impl RegionStoreRef {
    /// Reads a database metadata.
    pub fn get_database(&self, db_id: &str) -> Option<DatabaseMetadata> {
        self.region.databases.read().unwrap().get(db_id).cloned()
    }

    /// Reads a container state.
    pub fn get_container(&self, db_id: &str, coll_id: &str) -> Option<ContainerStateSnapshot> {
        let containers = self.region.containers.read().unwrap();
        let key = (db_id.to_string(), coll_id.to_string());
        containers.get(&key).map(|s| ContainerStateSnapshot {
            metadata: s.metadata.clone(),
        })
    }

    /// Executes a closure against the container's physical partitions.
    pub fn with_container<R>(
        &self,
        db_id: &str,
        coll_id: &str,
        f: impl FnOnce(&ContainerState) -> R,
    ) -> Option<R> {
        let containers = self.region.containers.read().unwrap();
        let key = (db_id.to_string(), coll_id.to_string());
        containers.get(&key).map(f)
    }

    /// Checks if a database exists.
    pub fn database_exists(&self, db_id: &str) -> bool {
        self.region.databases.read().unwrap().contains_key(db_id)
    }

    /// Checks if a container exists.
    pub fn container_exists(&self, db_id: &str, coll_id: &str) -> bool {
        self.region
            .containers
            .read()
            .unwrap()
            .contains_key(&(db_id.to_string(), coll_id.to_string()))
    }

    /// Deletes a database and all its containers in this region.
    pub fn delete_database(&self, db_id: &str) -> bool {
        let removed = self
            .region
            .databases
            .write()
            .unwrap()
            .remove(db_id)
            .is_some();
        if removed {
            // Cascade: remove all containers in this database
            self.region
                .containers
                .write()
                .unwrap()
                .retain(|(db, _), _| db != db_id);
        }
        removed
    }

    /// Deletes a container in this region.
    pub fn delete_container(&self, db_id: &str, coll_id: &str) -> bool {
        self.region
            .containers
            .write()
            .unwrap()
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
    /// Region that originated this write. Used as a stable tiebreaker when
    /// draining a paused replication buffer where two entries from different
    /// source regions can collide on `(epk, id, ts, lsn)`.
    pub source_region: String,
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
    /// Shared atomic counter for allocating new partition IDs (split/merge).
    /// Authoritative across *all* regions so partition IDs cannot diverge —
    /// real Cosmos DB pkrange IDs are properties of the container, not the
    /// replica.
    pub next_partition_id: Arc<AtomicU32>,
    /// Stable per-partition pkrange RIDs (`partition_id` → RID).
    ///
    /// Real Cosmos DB guarantees that a given pkrange has the same RID in
    /// every region. We cache the RID at first allocation so subsequent
    /// region replicas (and split-child seeding paths) reuse it instead of
    /// drawing a fresh value from the per-account `RidGenerator`.
    pub pkrange_rids: Arc<RwLock<HashMap<u32, String>>>,
}

/// A container's state including metadata and physical partitions.
pub(crate) struct ContainerState {
    pub metadata: ContainerMetadata,
    pub physical_partitions: Vec<PhysicalPartition>,
}

/// Snapshot of container metadata (without borrowing the lock).
#[allow(dead_code)]
pub(crate) struct ContainerStateSnapshot {
    pub metadata: ContainerMetadata,
}

impl ContainerState {
    pub(crate) fn new(meta: &ContainerMetadata, rid_gen: &RidGenerator) -> Self {
        let partitions = create_partitions(meta, rid_gen);
        Self {
            metadata: meta.clone(),
            physical_partitions: partitions,
        }
    }

    /// Finds the physical partition responsible for the given EPK.
    pub fn find_partition(&self, epk: &Epk) -> Option<&PhysicalPartition> {
        self.physical_partitions
            .iter()
            .find(|p| p.contains_epk(epk))
    }

    /// Allocates the next partition ID from the container-wide shared counter.
    pub fn next_partition_id(&self) -> u32 {
        self.metadata
            .next_partition_id
            .fetch_add(1, Ordering::SeqCst)
    }
}

/// A physical partition covering a range of EPK hash values.
pub(crate) struct PhysicalPartition {
    pub id: u32,
    pub epk_min: Epk,
    pub epk_max: Epk,
    pub lsn: AtomicU64,
    /// Per-region local LSN: count of mutations applied at *this* region for
    /// this partition (locally produced + replicated in). Real Cosmos DB
    /// session tokens carry both a global LSN and per-region local LSNs;
    /// using one value for both produces tokens that look correct only on
    /// single-region accounts.
    pub local_lsn: AtomicU64,
    pub vector_clock_version: AtomicU64,
    pub documents: RwLock<BTreeMap<Epk, BTreeMap<String, StoredDocument>>>,
    pub session_state: SessionState,
    pub rid: String,
    pub rid_prefix: u32,
    pub throughput_fraction: f64,
    pub parents: Vec<u32>,
    pub locked: AtomicBool,
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

    /// Returns the current local LSN for this region.
    pub fn current_local_lsn(&self) -> u64 {
        self.local_lsn.load(Ordering::SeqCst)
    }

    /// Advances the local LSN by 1 and returns the new value.
    pub fn advance_local_lsn(&self) -> u64 {
        self.local_lsn.fetch_add(1, Ordering::SeqCst) + 1
    }

    /// Returns whether this partition is currently locked (split/merge in progress).
    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::SeqCst)
    }
}

/// A stored document with system properties.
#[derive(Clone, Debug)]
pub(crate) struct StoredDocument {
    pub body: serde_json::Value,
    pub id: String,
    pub rid: String,
    pub etag: String,
    #[allow(dead_code)]
    pub ts: u64,
    pub self_link: String,
    #[allow(dead_code)]
    pub lsn: u64,
    pub epk: Epk,
    /// Size of `body` when serialized to JSON, captured at insertion. Cached
    /// so the read-RU computation does not have to re-serialize on every
    /// point read.
    pub body_size_bytes: usize,
}

/// Returns the cached pkrange RID for `partition_id`, allocating (and
/// caching on the shared metadata) on first sight.
///
/// The cache lives on `ContainerMetadata` and is shared across every region
/// replica of the container so the same partition has the same RID in every
/// region — matching the real Cosmos DB invariant that pkrange RIDs are a
/// property of the container, not the replica.
fn pkrange_rid_for(meta: &ContainerMetadata, rid_gen: &RidGenerator, partition_id: u32) -> String {
    {
        let map = meta.pkrange_rids.read().unwrap();
        if let Some(rid) = map.get(&partition_id) {
            return rid.clone();
        }
    }
    let mut map = meta.pkrange_rids.write().unwrap();
    map.entry(partition_id)
        .or_insert_with(|| {
            rid_gen.next_pkrange_rid(meta.numeric_db_id, meta.numeric_coll_id, partition_id)
        })
        .clone()
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
        // `boundaries` holds the N-1 *internal* boundaries; partition i's
        // lower bound is `boundaries[i-1]` and its upper bound is
        // `boundaries[i]`. The two open ends use the sentinels.
        let min = if i == 0 {
            Epk::min()
        } else {
            Epk::from(boundaries[(i - 1) as usize].clone())
        };
        let max = if i == n - 1 {
            Epk::max()
        } else {
            Epk::from(boundaries[i as usize].clone())
        };

        let rid = pkrange_rid_for(meta, rid_gen, i);

        let per_partition_ru = meta.provisioned_throughput_ru.map(|total| total / n);

        partitions.push(PhysicalPartition {
            id: i,
            epk_min: min,
            epk_max: max,
            lsn: AtomicU64::new(0),
            local_lsn: AtomicU64::new(0),
            vector_clock_version: AtomicU64::new(0),
            documents: RwLock::new(BTreeMap::new()),
            session_state: SessionState::new(),
            rid,
            rid_prefix: i,
            throughput_fraction: 1.0 / n as f64,
            parents: Vec::new(),
            locked: AtomicBool::new(false),
            throughput_tracker: per_partition_ru.map(ThroughputTracker::new),
        });
    }

    partitions
}

/// Computes the N-1 internal EPK boundary strings that divide the reachable
/// 128-bit hex space into N equal ranges.
///
/// The endpoints (partition 0's lower bound and partition N-1's upper bound)
/// are represented by the [`Epk::min()`] / [`Epk::max()`] sentinels at the
/// call site, so they are intentionally not emitted here. The returned vec
/// has length N-1 and is indexed `[1..N)` by the caller.
fn compute_partition_boundaries(n: u32) -> Vec<String> {
    if n <= 1 {
        return Vec::new();
    }
    // V2 EPK hash clears the top 2 bits, so EPKs lie in [0, 2^126).
    // Divide that reachable space evenly across N partitions.
    //
    // 32-character uppercase hex (16 bytes) is used for every boundary so
    // that lex-compare on equal-length hex strings matches numeric compare,
    // which is what `compute_epk_midpoint` relies on.
    let mut boundaries = Vec::with_capacity((n - 1) as usize);
    let total: u128 = 1u128 << 126;
    let step = total / n as u128;
    for i in 1..n {
        let boundary = step * i as u128;
        boundaries.push(format!("{:032X}", boundary));
    }
    boundaries
}

/// Returns true if `epk` represents the open lower bound of the EPK space.
fn is_epk_min(epk: &Epk) -> bool {
    epk.as_str().is_empty() || epk.as_str().chars().all(|c| c == '0')
}

/// Returns true if `epk` represents the open upper bound of the EPK space.
fn is_epk_max(epk: &Epk) -> bool {
    let s = epk.as_str();
    s == "FF" || s.eq_ignore_ascii_case("ffffffffffffffffffffffffffffffff")
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

/// Tracks RU consumption per 1-second tumbling window for throughput throttling.
///
/// Uses a monotonic [`std::time::Instant`] reference rather than wall-clock so:
/// - tests are immune to NTP / DST steps,
/// - bursts cannot accidentally span a second boundary and double the available
///   budget when the wall clock happens to tick during the burst.
pub(crate) struct ThroughputTracker {
    provisioned_ru: u32,
    inner: std::sync::Mutex<ThroughputWindow>,
}

struct ThroughputWindow {
    window_start: std::time::Instant,
    consumed_centiru: u64,
}

const THROUGHPUT_WINDOW: std::time::Duration = std::time::Duration::from_secs(1);

impl ThroughputTracker {
    pub fn new(provisioned_ru: u32) -> Self {
        Self {
            provisioned_ru,
            inner: std::sync::Mutex::new(ThroughputWindow {
                window_start: std::time::Instant::now(),
                consumed_centiru: 0,
            }),
        }
    }

    /// Attempts to consume `charge` RU. Returns `Ok(())` if within budget,
    /// or `Err(retry_after_ms)` if throttled.
    pub fn try_consume(&self, charge: f64) -> Result<(), u64> {
        let now = std::time::Instant::now();
        let charge_centiru = (charge * 100.0) as u64;
        let budget_centiru = (self.provisioned_ru as u64) * 100;
        let mut w = self.inner.lock().unwrap();
        if now.duration_since(w.window_start) >= THROUGHPUT_WINDOW {
            w.window_start = now;
            w.consumed_centiru = 0;
        }
        if w.consumed_centiru.saturating_add(charge_centiru) > budget_centiru {
            // Suggest waiting until the start of the next window.
            let remaining = THROUGHPUT_WINDOW.saturating_sub(now.duration_since(w.window_start));
            return Err(remaining.as_millis().max(1) as u64);
        }
        w.consumed_centiru += charge_centiru;
        Ok(())
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
    #[doc(hidden)]
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
                        p.locked.store(true, Ordering::SeqCst);
                    }
                }
            }
        }

        let store = Arc::clone(self);
        let db = db_id.to_string();
        let coll = coll_id.to_string();

        let lock = self.split_merge_lock(db_id, coll_id);
        self.control_plane_tasks.lock().unwrap().spawn(async move {
            let _guard = lock.lock().await;
            if !min_lock_duration.is_zero() {
                tokio::time::sleep(min_lock_duration).await;
            }
            // execute_split does the actual doc redistribution under the lock,
            // then unlocks partitions when done
            store.execute_split(&db, &coll, partition_id);
        });
    }

    /// Performs the actual split after the lock period.
    fn execute_split(&self, db_id: &str, coll_id: &str, partition_id: u32) {
        // Compute child IDs/RIDs ONCE based on the first region's view; all regions
        // share the same partition layout so this is consistent.
        let key = (db_id.to_string(), coll_id.to_string());
        let preview = {
            let regions = self.regions.read().unwrap();
            let mut found = None;
            for region in regions.values() {
                let containers = region.containers.read().unwrap();
                if let Some(state) = containers.get(&key) {
                    let parent = match state
                        .physical_partitions
                        .iter()
                        .find(|p| p.id == partition_id)
                    {
                        Some(p) => p,
                        None => continue,
                    };
                    let parent_lsn = parent.current_lsn();
                    let parent_version = parent.current_version();
                    let parent_min = parent.epk_min.clone();
                    let parent_max = parent.epk_max.clone();
                    let midpoint = match compute_epk_midpoint(&parent_min, &parent_max) {
                        Ok(m) => m,
                        Err(err) => {
                            tracing::error!(
                                error = %err,
                                db_id = db_id,
                                coll_id = coll_id,
                                partition_id = partition_id,
                                "in-memory emulator: aborting split — unlocking parent",
                            );
                            // Unlock the parent in every region so the partition
                            // does not stay wedged at locked=true.
                            let regions = self.regions.read().unwrap();
                            for region in regions.values() {
                                let containers = region.containers.read().unwrap();
                                if let Some(state) = containers.get(&key) {
                                    if let Some(p) = state
                                        .physical_partitions
                                        .iter()
                                        .find(|p| p.id == partition_id)
                                    {
                                        p.locked.store(false, Ordering::SeqCst);
                                    }
                                }
                            }
                            return;
                        }
                    };
                    // Both child IDs come from the shared per-container
                    // counter on `ContainerMetadata`, so they are identical
                    // across regions. Likewise RIDs go through the shared
                    // pkrange_rids cache so a given child id maps to the same
                    // RID in every region.
                    let child_id_1 = state.next_partition_id();
                    let child_id_2 = state.next_partition_id();
                    let child_rid_1 =
                        pkrange_rid_for(&state.metadata, &self.rid_generator, child_id_1);
                    let child_rid_2 =
                        pkrange_rid_for(&state.metadata, &self.rid_generator, child_id_2);
                    let total_throughput = state.metadata.provisioned_throughput_ru;
                    found = Some((
                        parent_lsn,
                        parent_version,
                        parent_min,
                        parent_max,
                        midpoint,
                        child_id_1,
                        child_id_2,
                        child_rid_1,
                        child_rid_2,
                        total_throughput,
                    ));
                    break;
                }
            }
            found
        };
        let (
            parent_lsn,
            parent_version,
            parent_min,
            parent_max,
            midpoint,
            child_id_1,
            child_id_2,
            child_rid_1,
            child_rid_2,
            total_throughput,
        ) = match preview {
            Some(t) => t,
            None => return,
        };
        let child_lsn = parent_lsn + 1;

        let regions = self.regions.read().unwrap();
        for region in regions.values() {
            let mut containers = region.containers.write().unwrap();
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

                let n = state.physical_partitions.len() as f64 + 1.0;
                let per_partition_ru = total_throughput.map(|total| total / (n as u32));

                let child1 = PhysicalPartition {
                    id: child_id_1,
                    epk_min: parent_min.clone(),
                    epk_max: midpoint.clone(),
                    lsn: AtomicU64::new(child_lsn),
                    local_lsn: AtomicU64::new(child_lsn),
                    vector_clock_version: AtomicU64::new(parent_version),
                    documents: RwLock::new(docs_1),
                    session_state: SessionState::new(),
                    rid: child_rid_1.clone(),
                    rid_prefix: child_id_1,
                    throughput_fraction: 1.0 / n,
                    parents: vec![partition_id],
                    locked: AtomicBool::new(false),
                    throughput_tracker: per_partition_ru.map(ThroughputTracker::new),
                };

                let child2 = PhysicalPartition {
                    id: child_id_2,
                    epk_min: midpoint.clone(),
                    epk_max: parent_max.clone(),
                    lsn: AtomicU64::new(child_lsn),
                    local_lsn: AtomicU64::new(child_lsn),
                    vector_clock_version: AtomicU64::new(parent_version),
                    documents: RwLock::new(docs_2),
                    session_state: SessionState::new(),
                    rid: child_rid_2.clone(),
                    rid_prefix: child_id_2,
                    throughput_fraction: 1.0 / n,
                    parents: vec![partition_id],
                    locked: AtomicBool::new(false),
                    throughput_tracker: per_partition_ru.map(ThroughputTracker::new),
                };

                state.physical_partitions.remove(parent_idx);
                state.physical_partitions.push(child1);
                state.physical_partitions.push(child2);
                // No per-region counter to reconcile any more — the shared
                // counter on `ContainerMetadata` was already advanced when the
                // child IDs were allocated.
            }
        }
    }

    /// Merges two adjacent physical partitions into one child partition.
    ///
    /// During `min_lock_duration` (plus doc merge time), operations on both partitions
    /// return 410/1007. After: both parents are replaced by a single child.
    /// Child vector_clock_version = max(parent_versions) + 1 (merge DOES increment version).
    /// Child LSN = 1 (restarts).
    #[doc(hidden)]
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
                            p.locked.store(true, Ordering::SeqCst);
                        }
                    }
                }
            }
        }

        let store = Arc::clone(self);
        let db = db_id.to_string();
        let coll = coll_id.to_string();

        let lock = self.split_merge_lock(db_id, coll_id);
        self.control_plane_tasks.lock().unwrap().spawn(async move {
            let _guard = lock.lock().await;
            if !min_lock_duration.is_zero() {
                tokio::time::sleep(min_lock_duration).await;
            }
            store.execute_merge(&db, &coll, partition_id_a, partition_id_b);
        });
    }

    fn unlock_partitions(&self, key: &(String, String), partition_ids: &[u32]) {
        let regions = self.regions.read().unwrap();
        for region in regions.values() {
            let containers = region.containers.read().unwrap();
            if let Some(state) = containers.get(key) {
                for partition in &state.physical_partitions {
                    if partition_ids.contains(&partition.id) {
                        partition.locked.store(false, Ordering::SeqCst);
                    }
                }
            }
        }
    }

    /// Performs the actual merge after the lock period.
    fn execute_merge(&self, db_id: &str, coll_id: &str, partition_id_a: u32, partition_id_b: u32) {
        enum MergePreview {
            Ready((Epk, Epk, u64, u32, String, Option<u32>)),
            NonAdjacent(Epk, Epk),
        }

        // Compute child id/rid + merged bounds ONCE, based on the first region's view.
        let key = (db_id.to_string(), coll_id.to_string());
        let preview = {
            let regions = self.regions.read().unwrap();
            let mut found = None;
            for region in regions.values() {
                let containers = region.containers.read().unwrap();
                if let Some(state) = containers.get(&key) {
                    let pa = state
                        .physical_partitions
                        .iter()
                        .find(|p| p.id == partition_id_a);
                    let pb = state
                        .physical_partitions
                        .iter()
                        .find(|p| p.id == partition_id_b);
                    let (pa, pb) = match (pa, pb) {
                        (Some(a), Some(b)) => (a, b),
                        _ => continue,
                    };
                    let (lower, upper) = if pa.epk_min < pb.epk_min {
                        (pa, pb)
                    } else {
                        (pb, pa)
                    };
                    if lower.epk_max != upper.epk_min {
                        found = Some(MergePreview::NonAdjacent(
                            lower.epk_max.clone(),
                            upper.epk_min.clone(),
                        ));
                        break;
                    }
                    let merged_min = lower.epk_min.clone();
                    let merged_max = upper.epk_max.clone();
                    let max_version =
                        std::cmp::max(lower.current_version(), upper.current_version());
                    let child_version = max_version + 1;
                    let child_id = state.next_partition_id();
                    let child_rid = pkrange_rid_for(&state.metadata, &self.rid_generator, child_id);
                    let total_throughput = state.metadata.provisioned_throughput_ru;
                    found = Some(MergePreview::Ready((
                        merged_min,
                        merged_max,
                        child_version,
                        child_id,
                        child_rid,
                        total_throughput,
                    )));
                    break;
                }
            }
            found
        };
        let (merged_min, merged_max, child_version, child_id, child_rid, total_throughput) =
            match preview {
                Some(MergePreview::Ready(preview)) => preview,
                Some(MergePreview::NonAdjacent(left_max, right_min)) => {
                    tracing::warn!(
                        db_id = db_id,
                        coll_id = coll_id,
                        partition_id_a = partition_id_a,
                        partition_id_b = partition_id_b,
                        left_max = %left_max,
                        right_min = %right_min,
                        "in-memory emulator: rejecting merge for non-adjacent partitions",
                    );
                    self.unlock_partitions(&key, &[partition_id_a, partition_id_b]);
                    return;
                }
                None => {
                    self.unlock_partitions(&key, &[partition_id_a, partition_id_b]);
                    return;
                }
            };

        let regions = self.regions.read().unwrap();
        for region in regions.values() {
            let mut containers = region.containers.write().unwrap();
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
                let (lower_idx, upper_idx) = if state.physical_partitions[idx_a].epk_min
                    < state.physical_partitions[idx_b].epk_min
                {
                    (idx_a, idx_b)
                } else {
                    (idx_b, idx_a)
                };
                let lower = &state.physical_partitions[lower_idx];
                let upper = &state.physical_partitions[upper_idx];
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

                let n = state.physical_partitions.len() as f64 - 1.0;
                let per_partition_ru = total_throughput.map(|total| total / (n.max(1.0) as u32));

                let child = PhysicalPartition {
                    id: child_id,
                    epk_min: merged_min.clone(),
                    epk_max: merged_max.clone(),
                    lsn: AtomicU64::new(1),
                    local_lsn: AtomicU64::new(1),
                    vector_clock_version: AtomicU64::new(child_version),
                    documents: RwLock::new(merged_docs),
                    session_state: SessionState::new(),
                    rid: child_rid.clone(),
                    rid_prefix: child_id,
                    throughput_fraction: 1.0 / n.max(1.0),
                    parents: vec![partition_id_a, partition_id_b],
                    locked: AtomicBool::new(false),
                    throughput_tracker: per_partition_ru.map(ThroughputTracker::new),
                };

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
///
/// Returns `Err` if either bound is not parseable. Callers in the split path
/// must surface the error and unlock the parent partition rather than panic;
/// a corrupt bound at the split site would otherwise wedge the partition
/// `locked = true` forever inside a spawned task with no caller to observe
/// the panic.
fn compute_epk_midpoint(min: &Epk, max: &Epk) -> Result<Epk, String> {
    let parse = |epk: &Epk, label: &str| -> Result<u128, String> {
        if is_epk_min(epk) {
            return Ok(0u128);
        }
        if is_epk_max(epk) {
            return Ok(1u128 << 126);
        }
        u128::from_str_radix(epk.as_str(), 16)
            .map_err(|e| format!("corrupted EPK partition bound {}={:?}: {e}", label, epk))
    };
    let min_val = parse(min, "min")?;
    let max_val = parse(max, "max")?;
    // Safe midpoint: `min/2 + max/2` loses 1 bit when both operands are odd.
    // Add the missing carry explicitly.
    let mid = min_val / 2 + max_val / 2 + ((min_val & 1) & (max_val & 1));
    Ok(Epk::from(format!("{:032X}", mid)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partitions_distribute_across_full_v2_epk_space() {
        use super::super::epk::{compute_epk, parse_partition_key_header};
        use crate::models::PartitionKeyDefinition;
        let pk_def: PartitionKeyDefinition = serde_json::from_value(serde_json::json!({
            "paths": ["/pk"], "kind": "Hash", "version": 2
        }))
        .unwrap();
        let config = super::super::config::VirtualAccountConfig::new(vec![
            super::super::config::VirtualRegion::new(
                "r1",
                url::Url::parse("https://r1.local").unwrap(),
            ),
        ])
        .unwrap();
        let store = EmulatorStore::new(config);
        store.create_database("db");
        store.create_container_with_config(
            "db",
            "c",
            pk_def,
            super::super::config::ContainerConfig::new()
                .with_partition_count(8)
                .unwrap(),
        );
        let region_ref = store.region("r1").unwrap();
        let mut counts = vec![0usize; 8];
        for i in 0..2000 {
            let pk_json = format!("[\"key-{}\"]", i);
            let comps = parse_partition_key_header(&pk_json).unwrap();
            let epk = compute_epk(
                &comps,
                crate::models::PartitionKeyKind::Hash,
                crate::models::PartitionKeyVersion::V2,
            );
            region_ref
                .with_container("db", "c", |state| {
                    let p = state.find_partition(&epk).unwrap();
                    counts[p.id as usize] += 1;
                })
                .unwrap();
        }
        for (i, c) in counts.iter().enumerate() {
            assert!(*c > 0, "partition {} had 0 docs (counts={:?})", i, counts);
        }
    }

    #[test]
    fn database_and_container_etags_consistent_across_regions() {
        let config = super::super::config::VirtualAccountConfig::new(vec![
            super::super::config::VirtualRegion::new(
                "r1",
                url::Url::parse("https://r1.local").unwrap(),
            ),
            super::super::config::VirtualRegion::new(
                "r2",
                url::Url::parse("https://r2.local").unwrap(),
            ),
        ])
        .unwrap();
        let store = EmulatorStore::new(config);
        store.create_database("db");
        let pk_def: crate::models::PartitionKeyDefinition =
            serde_json::from_value(serde_json::json!({
                "paths": ["/pk"], "kind": "Hash", "version": 2
            }))
            .unwrap();
        store.create_container("db", "c", pk_def);
        let r1 = store.region("r1").unwrap();
        let r2 = store.region("r2").unwrap();
        assert_eq!(
            r1.get_database("db").unwrap().etag,
            r2.get_database("db").unwrap().etag
        );
        assert_eq!(
            r1.get_container("db", "c").unwrap().metadata.etag,
            r2.get_container("db", "c").unwrap().metadata.etag,
        );
    }
}
