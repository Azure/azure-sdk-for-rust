// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore epks llsn splittable
//! In-memory document store with multi-region support.

use std::collections::{BTreeMap, HashMap, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use super::config::{ContainerConfig, VirtualAccountConfig};
use super::epk::Epk;
use super::rid::RidGenerator;
use super::session::SessionState;
use crate::models::{PartitionKeyDefinition, PartitionKeyKind, PartitionKeyVersion};

type SplitMergeLocks = HashMap<(String, String), Arc<async_lock::Mutex<()>>>;
/// Sentinel pkrange id used for control-plane (database/container/offer) session
/// tokens. Chosen as `u32::MAX` so it cannot collide with any user pkrange id,
/// since real Cosmos partition counts are bounded far below this value.
pub(crate) const MASTER_PARTITION_ID: u32 = u32::MAX;

/// Top-level emulator store holding all regions.
/// Identifies the resource a control-plane task acts on. Used by
/// `wait_for_split` / future filtered drains to wake only the tasks that
/// correspond to a given `(db, coll, partition)` triple.
#[derive(Clone, Debug)]
struct ControlPlaneTaskKey {
    db: String,
    coll: String,
    /// Set of partition ids the task references — splits have one parent,
    /// merges have two. Match if any element matches.
    partitions: Vec<u32>,
}

impl ControlPlaneTaskKey {
    fn matches(&self, db: &str, coll: &str, partition_id: u32) -> bool {
        self.db == db && self.coll == coll && self.partitions.contains(&partition_id)
    }
}

/// Applies a single document mutation to a partition under LWW
/// (Last-Writer-Wins) on `(_ts, lsn)`.
///
/// This is the only place that performs the LWW comparison and the LSN
/// bumps for replicated/replayed writes — both `apply_replication` and the
/// drain loop in `resume_replication` route through here so the rule cannot
/// drift between the two paths. The LWW comparison honors the
/// `conflictResolutionPolicy` (LastWriterWins on `/_ts`) advertised in
/// container metadata.
///
/// Always advances `partition.lsn` to `max(current, doc.lsn)` and bumps
/// `partition.local_lsn` by one — the per-region local LSN counts every
/// mutation applied at this region regardless of which write won the LWW
/// comparison.
fn apply_doc_to_partition(partition: &PhysicalPartition, doc: &StoredDocument, is_delete: bool) {
    let mut docs = partition.documents.write().unwrap();
    // Resolve the LWW outcome without mutating the outer map: avoids leaving
    // an empty inner BTreeMap behind when this call ends up being a no-op
    // (lost LWW, no-op delete on absent doc, etc.). Without that guard,
    // every unique EPK that ever saw a failed write left a permanent empty
    // entry — slow leak under fuzz / churn loads.
    let (should_apply, was_present) = {
        let existing = docs.get(&doc.epk).and_then(|m| m.get(&doc.id));
        match existing {
            None => (true, false),
            Some(e) => {
                // LWW comparison on `(ts, lsn)` only — strictly greater wins.
                // On a `(ts, lsn)` tie we keep the locally-stored doc (the
                // existing entry) and discard the incoming one. Multi-master
                // conflict resolution only happens in the burning region (the
                // local write region for the partition), and on a true tie
                // letting the local doc win is correct: there is no
                // observable distinction between "we already applied an
                // identical write" and "two regions raced to identical
                // state", and any tiebreaker over `source_region` would
                // make the converged value depend on which region a test
                // happens to query first. The resume-from-pause drain still
                // sorts by `source_region` as a final stable-ordering key
                // (so two paused entries at the same `(ts, lsn)` apply in
                // a deterministic order), but it routes through this same
                // function, so the keep-local-on-tie rule still wins
                // overall.
                let cmp = (doc.ts, doc.lsn).cmp(&(e.ts, e.lsn));
                (cmp == std::cmp::Ordering::Greater, true)
            }
        }
    };
    if !should_apply {
        // Lost the LWW; advance the high-water LSN so subsequent writes still
        // observe a non-decreasing partition LSN.
        partition.lsn.fetch_max(doc.lsn, Ordering::SeqCst);
        return;
    }
    if is_delete && !was_present {
        // No-op delete: no observable mutation, so no local_lsn bump.
        // A bumped local_lsn would advertise per-region progress for a
        // mutation that never happened, breaking session-token assertions.
        partition.lsn.fetch_max(doc.lsn, Ordering::SeqCst);
        return;
    }
    if is_delete {
        if let Some(map) = docs.get_mut(&doc.epk) {
            map.remove(&doc.id);
        }
    } else {
        docs.entry(doc.epk.clone())
            .or_default()
            .insert(doc.id.clone(), doc.clone());
    }
    // Only bump local_lsn when the mutation actually lands. The per-region
    // local LSN is a count of *applied* writes; advancing it for stale
    // replications that lose the LWW comparison would emit session tokens
    // whose local-LSN segment exceeds anything a real Cosmos gateway would
    // produce and silently break dual-backend assertions on
    // `x-ms-cosmos-llsn`.
    partition.local_lsn.fetch_add(1, Ordering::SeqCst);
    partition.lsn.fetch_max(doc.lsn, Ordering::SeqCst);
}
async fn await_control_plane_handle(handle: tokio::task::JoinHandle<()>) {
    if let Err(e) = handle.await {
        if e.is_panic() {
            std::panic::resume_unwind(e.into_panic());
        }
    }
}
/// Top-level emulator store.
///
/// # Tokio runtime requirement
///
/// Background work (delayed replication, split/merge execution, deferred
/// replication during partition locks) is scheduled via `tokio::spawn`.
/// Methods that may schedule that work — including any point write that goes
/// through a non-immediate replication config, [`Self::split_partition`], and
/// [`Self::merge_partitions`] — must therefore be called from inside a Tokio
/// runtime. Calling them outside one will panic.
pub struct EmulatorStore {
    config: VirtualAccountConfig,
    rid_generator: RidGenerator,
    regions: RwLock<HashMap<String, Arc<RegionStore>>>,

    /// Per-(db, coll) async mutex serializing split/merge execution.
    split_merge_locks: std::sync::Mutex<SplitMergeLocks>,
    /// Per-control-plane-resource async mutex serializing the
    /// `existence-check + create-internal` sequence in
    /// `handle_create_database` and `handle_create_container`.
    ///
    /// Without this, two concurrent create requests for the same id can
    /// both observe "does not exist", both proceed to
    /// `create_*_internal`, and produce duplicate metadata (last writer
    /// wins on the regional snapshot but earlier callers see a 201/Created
    /// for an entity that no longer exists). The lock makes the pair
    /// atomic per resource without serializing unrelated creates.
    ///
    /// Keys: `db_id` for database creates, `format!("{db}::{coll}")` for
    /// container creates. Map entries are never removed — control-plane
    /// resources are bounded in real workloads (and in tests), so leaking
    /// per-id locks is preferable to a remove-on-drop dance that races
    /// fresh acquisitions.
    control_plane_locks: std::sync::Mutex<HashMap<String, Arc<async_lock::Mutex<()>>>>,
    /// Serializes emulator document writes while preview distributed
    /// transactions are enabled.
    ///
    /// DTX rollback restores pre-images. Without a transaction-wide write
    /// guard, a concurrent point write can commit between preimage capture and
    /// rollback, then be overwritten by the restore path.
    #[cfg(feature = "preview_dtx")]
    document_write_lock: Arc<async_lock::Mutex<()>>,
    /// Buffers replication issued while a distributed transaction is applying so
    /// a rollback can discard replicas that were never durably committed.
    ///
    /// `Some(buffer)` means a DTX write transaction is capturing replication.
    /// The DTX write path holds `document_write_lock` for the whole
    /// transaction, which serializes every emulator write, so no unrelated
    /// write's replication can be captured here. `None` is the normal
    /// immediate-replication path. On commit the buffer is drained and
    /// replayed; on abort it is dropped so rolled-back writes never reach
    /// secondary regions.
    #[cfg(feature = "preview_dtx")]
    dtx_replication_capture: std::sync::Mutex<Option<Vec<CapturedReplication>>>,
    /// Tracks spawned replication tasks so tests can drain them.
    replication_tasks: std::sync::Mutex<tokio::task::JoinSet<()>>,
    /// Tracks spawned split/merge tasks separately from replication so a
    /// control-plane panic does not surface inside an unrelated point-write
    /// handler that happens to call `replicate()`.
    /// Tagged registry of in-flight control-plane tasks (split, merge).
    ///
    /// Each entry carries the `(db, coll, partition_ids)` it operates on so
    /// callers like `wait_for_split` can await only the tasks that match —
    /// avoiding "waiting on container A also waits on every other
    /// container's split/merge in the entire store" surprises.
    control_plane_tasks: std::sync::Mutex<Vec<(ControlPlaneTaskKey, tokio::task::JoinHandle<()>)>>,
    /// Monotonic counter used to populate `x-ms-transport-request-id`. Every
    /// HTTP response gets a unique value so consumers correlating retries on
    /// the wire see stable identity instead of a partition LSN that drifts
    /// with mutations.
    /// Per-store counter for the `x-ms-transport-request-id` header.
    /// Wraps on overflow (the SDK side parses the header as a `u32`).
    transport_request_counter: AtomicU32,
    /// Bounded permit pool for in-flight delayed replication tasks. Capped at
    /// `10 * available_parallelism` so a write storm with default 20–50ms
    /// replication delay cannot spawn an unbounded backlog of `tokio` tasks
    /// before the first batch finishes and gets reaped.
    replication_semaphore: Arc<tokio::sync::Semaphore>,
    /// Count of replication entries dropped because the per-region buffer was
    /// full. The proactive 429/3075 short-circuit in
    /// ind_overflowed_replication_target is racy across concurrent writers,
    /// so the apply path still has to drop on overflow as a safety net. Tests
    /// that want to assert the safety net never fired can read this counter
    /// in teardown via [Self::dropped_replications] and fail when non-zero.
    dropped_replications: AtomicU64,
    /// Captured panic payloads from background replication tasks. Live write
    /// handlers do **not** unwind on a captured panic — doing so threw away
    /// the in-flight HTTP response of an unrelated successful write, and the
    /// failure manifested non-causally in some other test. Instead, panics
    /// are accumulated here and surfaced when tests call
    /// [Self::drain_pending_replications] (which is the test-shutdown
    /// barrier) or [Self::take_captured_panics] for explicit inspection.
    captured_panics: std::sync::Mutex<Vec<Box<dyn std::any::Any + Send + 'static>>>,
}

/// A replication operation buffered during a distributed transaction so it can
/// be replayed on commit or dropped on rollback.
#[cfg(feature = "preview_dtx")]
struct CapturedReplication {
    source_region: String,
    db_id: String,
    coll_id: String,
    doc: StoredDocument,
    is_delete: bool,
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

            split_merge_locks: std::sync::Mutex::new(HashMap::new()),
            control_plane_locks: std::sync::Mutex::new(HashMap::new()),
            #[cfg(feature = "preview_dtx")]
            document_write_lock: Arc::new(async_lock::Mutex::new(())),
            #[cfg(feature = "preview_dtx")]
            dtx_replication_capture: std::sync::Mutex::new(None),
            replication_tasks: std::sync::Mutex::new(tokio::task::JoinSet::new()),
            control_plane_tasks: std::sync::Mutex::new(Vec::new()),
            transport_request_counter: AtomicU32::new(0),
            replication_semaphore: Arc::new(tokio::sync::Semaphore::new(
                std::thread::available_parallelism()
                    .map(|n| n.get())
                    .unwrap_or(1)
                    .saturating_mul(10)
                    .max(64),
            )),
            dropped_replications: AtomicU64::new(0),
            captured_panics: std::sync::Mutex::new(Vec::new()),
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
    /// Advances the per-region master-partition LSN and returns a V1 session
    /// token. Tracking control-plane metadata per-region matches the real
    /// service: each region's master partition advances independently, so a
    /// control-plane session token from region A is not directly comparable to
    /// one from region B.
    pub(crate) fn advance_master_partition_lsn(&self, region_name: &str) -> String {
        let regions = self.regions.read().unwrap();
        let lsn = match regions.get(region_name) {
            Some(rs) => rs.master_partition_lsn.fetch_add(1, Ordering::SeqCst) + 1,
            None => 0, // unknown region — emit a 0 token rather than panicking
        };
        super::session::SessionToken::format(MASTER_PARTITION_ID, lsn)
    }

    /// Allocates the next value for `x-ms-transport-request-id`.
    ///
    /// Wraps on overflow rather than panicking — the header is informational
    /// and the SDK side parses it as `u32`, so a `u64`-shaped emulator
    /// counter would silently round-trip to `None` past `u32::MAX` and break
    /// request correlation. Keeping the counter at `u32` width matches the
    /// real service's wire format.
    pub(crate) fn next_transport_request_id(&self) -> u32 {
        self.transport_request_counter
            .fetch_add(1, Ordering::Relaxed)
            .wrapping_add(1)
    }

    /// Number of replication entries dropped as a safety-net for buffer
    /// overflow (see `dropped_replications` field). Test-only.
    #[doc(hidden)]
    pub fn dropped_replications(&self) -> u64 {
        self.dropped_replications.load(Ordering::SeqCst)
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
        let drained: Vec<_> = {
            let mut guard = self.control_plane_tasks.lock().unwrap();
            std::mem::take(&mut *guard)
        };
        for (_, handle) in drained {
            await_control_plane_handle(handle).await;
        }
    }

    /// Awaits only the in-flight split tasks for `(db, coll, partition_id)`,
    /// leaving unrelated tasks (other containers, merges of other partitions)
    /// running. Tests use this to assert post-split topology without
    /// inadvertently serializing on every other control-plane operation in
    /// the store.
    #[doc(hidden)]
    pub async fn wait_for_split(&self, db: &str, coll: &str, partition_id: u32) {
        let matching: Vec<_> = {
            let mut guard = self.control_plane_tasks.lock().unwrap();
            // Partition the registry: keep non-matching, return matching.
            let mut keep = Vec::new();
            let mut take = Vec::new();
            for (key, handle) in std::mem::take(&mut *guard) {
                if key.matches(db, coll, partition_id) {
                    take.push(handle);
                } else {
                    keep.push((key, handle));
                }
            }
            *guard = keep;
            take
        };
        for handle in matching {
            await_control_plane_handle(handle).await;
        }
    }

    fn split_merge_lock(&self, db: &str, coll: &str) -> Arc<async_lock::Mutex<()>> {
        let mut map = self.split_merge_locks.lock().unwrap();
        map.entry((db.to_string(), coll.to_string()))
            .or_insert_with(|| Arc::new(async_lock::Mutex::new(())))
            .clone()
    }

    /// Returns the async mutex serializing the `(existence check, create)`
    /// pair for a control-plane resource keyed by `key`.
    ///
    /// Use [`Self::control_plane_lock_db`] / [`Self::control_plane_lock_coll`]
    /// for type-safe key construction; this raw form is exposed for handlers
    /// that compose their own keys.
    pub(crate) fn control_plane_lock(&self, key: &str) -> Arc<async_lock::Mutex<()>> {
        let mut map = self.control_plane_locks.lock().unwrap();
        if let Some(existing) = map.get(key) {
            return existing.clone();
        }
        let new_lock = Arc::new(async_lock::Mutex::new(()));
        map.insert(key.to_string(), new_lock.clone());
        new_lock
    }

    /// Per-database control-plane lock for `handle_create_database`.
    pub(crate) fn control_plane_lock_db(&self, db: &str) -> Arc<async_lock::Mutex<()>> {
        self.control_plane_lock(db)
    }

    /// Per-(db, coll) control-plane lock for `handle_create_container`. The
    /// key is namespaced under `db` so the same `coll` id under different
    /// databases does not contend on the same lock.
    pub(crate) fn control_plane_lock_coll(
        &self,
        db: &str,
        coll: &str,
    ) -> Arc<async_lock::Mutex<()>> {
        self.control_plane_lock(&format!("{}::{}", db, coll))
    }

    /// Returns the preview-DTX document write lock.
    #[cfg(feature = "preview_dtx")]
    pub(crate) fn document_write_lock(&self) -> Arc<async_lock::Mutex<()>> {
        self.document_write_lock.clone()
    }

    /// Returns the preview-DTX document write lock for internal emulator tests.
    #[cfg(feature = "preview_dtx")]
    #[doc(hidden)]
    pub fn document_write_lock_for_tests(&self) -> Arc<async_lock::Mutex<()>> {
        self.document_write_lock.clone()
    }

    /// Awaits all pending in-flight replication tasks and surfaces any
    /// previously-captured background panics. Test-only.
    ///
    /// Panics from background tasks are captured into `captured_panics`
    /// (rather than unwinding the live write that triggered the reap) so
    /// they don't poison unrelated in-flight requests. This drain is the
    /// barrier where tests observe them — it re-raises the **first**
    /// captured panic on the current thread, matching the prior behavior.
    /// Subsequent panics (if any) are dropped after a `tracing::error!`.
    #[doc(hidden)]
    pub async fn drain_pending_replications(&self) {
        let mut set = {
            let mut guard = self.replication_tasks.lock().unwrap();
            std::mem::replace(&mut *guard, tokio::task::JoinSet::new())
        };
        while let Some(res) = set.join_next().await {
            if let Err(e) = res {
                if e.is_panic() {
                    self.captured_panics.lock().unwrap().push(e.into_panic());
                }
            }
        }
        let panics: Vec<Box<dyn std::any::Any + Send + 'static>> = {
            let mut guard = self.captured_panics.lock().unwrap();
            std::mem::take(&mut *guard)
        };
        let extra = panics.len().saturating_sub(1);
        let mut iter = panics.into_iter();
        if let Some(first) = iter.next() {
            if extra > 0 {
                tracing::error!(
                    extra = extra,
                    "in-memory emulator: drain saw multiple background panics; only the first is re-raised",
                );
            }
            std::panic::resume_unwind(first);
        }
    }

    /// Returns and clears any panic payloads captured from background
    /// replication tasks without re-raising them. Useful for tests that want
    /// to inspect or assert on background failures explicitly.
    ///
    /// Gated behind the `__internal_in_memory_emulator` crate feature.
    #[doc(hidden)]
    pub fn take_captured_panics(&self) -> Vec<Box<dyn std::any::Any + Send + 'static>> {
        std::mem::take(&mut *self.captured_panics.lock().unwrap())
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

        let offer = meta
            .provisioned_throughput_ru
            .map(|throughput| OfferMetadata {
                id: format!("offer_{}_{}", meta.numeric_db_id, meta.numeric_coll_id),
                offer_resource_id: meta.rid.clone(),
                throughput,
                rid: format!("offer_{}_{}", meta.numeric_db_id, meta.numeric_coll_id),
                ts,
                self_link: format!(
                    "offers/offer_{}_{}/",
                    meta.numeric_db_id, meta.numeric_coll_id
                ),
                etag: new_etag(),
            });

        // Each region gets its own ContainerState (own LSNs, own document
        // store) but they all share the same `ContainerMetadata`, so pkrange
        // RIDs are allocated once via `pkrange_rid_for` and reused.
        let regions = self.regions.read().unwrap();
        for region in regions.values() {
            let mut containers = region.containers.write().unwrap();
            containers.insert(
                (db_id.to_string(), coll_id.to_string()),
                ContainerState::new(&meta, &self.rid_generator, self.config.throttling_enabled()),
            );
            if let Some(offer) = &offer {
                region
                    .offers
                    .write()
                    .unwrap()
                    .insert(offer.id.clone(), offer.clone());
            }
        }

        meta
    }

    pub(crate) fn replace_offer_internal(
        &self,
        offer_id: &str,
        throughput: u32,
    ) -> Option<OfferMetadata> {
        let regions = self.regions.read().unwrap();
        let ts = current_timestamp();
        let etag = new_etag();
        let mut updated = None;
        for region in regions.values() {
            let mut offers = region.offers.write().unwrap();
            if let Some(offer) = offers.get_mut(offer_id) {
                offer.throughput = throughput;
                offer.ts = ts;
                offer.etag = etag.clone();
                updated = Some(offer.clone());
            }
        }
        updated
    }

    /// Cascade-deletes a database from every region. Also purges any buffered
    /// replication entries for containers in this database (otherwise a paused
    /// target region would silently drop them on resume) and prunes the
    /// per-database collection-counter slot in the RID generator.
    ///
    /// Returns the numeric db id when the database existed in at least one
    /// region, otherwise `None`.
    pub(crate) fn cascade_delete_database(&self, db_id: &str) -> Option<u32> {
        // Look up the numeric id from any region before we drop the metadata.
        let numeric_db_id = {
            let regions = self.regions.read().unwrap();
            let mut id = None;
            for region in regions.values() {
                if let Some(meta) = region.databases.read().unwrap().get(db_id) {
                    id = Some(meta.numeric_id);
                    break;
                }
            }
            id
        };
        let regions = self.regions.read().unwrap();
        for region in regions.values() {
            // Purge buffered replications targeted at any container in this db.
            let mut buf = region.replication_buffer.write().unwrap();
            buf.retain(|e| e.db_id != db_id);
            drop(buf);
            // Then remove databases + cascade containers.
            let removed_db = region.databases.write().unwrap().remove(db_id).is_some();
            if removed_db {
                // Collect offer IDs for all containers in this database before
                // removing them so we can purge the associated offer entries.
                let offer_ids: Vec<String> = region
                    .containers
                    .read()
                    .unwrap()
                    .iter()
                    .filter(|((db, _), _)| db == db_id)
                    .map(|(_, state)| {
                        format!(
                            "offer_{}_{}",
                            state.metadata.numeric_db_id, state.metadata.numeric_coll_id
                        )
                    })
                    .collect();
                region
                    .containers
                    .write()
                    .unwrap()
                    .retain(|(db, _), _| db != db_id);
                let mut offers = region.offers.write().unwrap();
                for offer_id in offer_ids {
                    offers.remove(&offer_id);
                }
            }
        }
        if let Some(id) = numeric_db_id {
            self.rid_generator.forget_database(id);
        }
        numeric_db_id
    }

    /// Cascade-deletes a container from every region. Also purges any buffered
    /// replication entries for the container so a paused target region does
    /// not silently drop them on resume.
    ///
    /// Returns `true` when the container existed in at least one region.
    pub(crate) fn cascade_delete_container(&self, db_id: &str, coll_id: &str) -> bool {
        let regions = self.regions.read().unwrap();
        let mut existed = false;
        for region in regions.values() {
            let mut buf = region.replication_buffer.write().unwrap();
            buf.retain(|e| !(e.db_id == db_id && e.coll_id == coll_id));
            drop(buf);
            let removed = region
                .containers
                .write()
                .unwrap()
                .remove(&(db_id.to_string(), coll_id.to_string()));
            if let Some(state) = removed {
                let offer_id = format!(
                    "offer_{}_{}",
                    state.metadata.numeric_db_id, state.metadata.numeric_coll_id
                );
                region.offers.write().unwrap().remove(&offer_id);
                existed = true;
            }
        }
        existed
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
    ) -> crate::error::Result<()> {
        let pk_components = super::epk::parse_partition_key_header(partition_key_json)?;
        if pk_components.is_empty() {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message("force_session_not_available requires a non-empty partition key")
                .build());
        }
        let regions = self.regions.read().unwrap();
        let region_store = regions.get(region).ok_or_else(|| {
            crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message(format!("region '{region}' is not provisioned"))
                .build()
        })?;
        let containers = region_store.containers.read().unwrap();
        let key = (db_id.to_string(), coll_id.to_string());
        let state = containers.get(&key).ok_or_else(|| {
            crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message(format!(
                    "container '{db_id}/{coll_id}' is not provisioned in region '{region}'"
                ))
                .build()
        })?;
        let epk = super::epk::compute_epk(
            &pk_components,
            state.metadata.partition_key.kind(),
            state.metadata.partition_key.version(),
        );
        let partition = state.find_partition(&epk).ok_or_else(|| {
            crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message(format!(
                    "no physical partition found for EPK {} in container '{}/{}'",
                    epk.to_hex(),
                    db_id,
                    coll_id
                ))
                .build()
        })?;
        partition
            .session_state
            .set_force_unavailable_for(&epk.to_hex());
        Ok(())
    }

    /// Pauses replication TO the given target region.
    ///
    /// While paused, replicated writes destined for `target_region` accumulate in that
    /// region's replication buffer instead of being applied. Tests that compare per-region
    /// state for consistency MUST drain the buffer before asserting — call
    /// [`Self::resume_replication`] (which drains synchronously) and then
    /// [`Self::drain_pending_replications`] to await any background apply tasks. Querying
    /// a paused target's documents directly while writes are still buffered will observe
    /// stale state and is not a valid consistency assertion.
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
                        apply_doc_to_partition(partition, &entry.doc, entry.is_delete);
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
        // While a distributed transaction is applying, buffer replication so a
        // rollback can discard replicas that were never durably committed. The
        // DTX write path holds `document_write_lock` for the whole transaction,
        // which serializes all emulator writes, so this cannot capture an
        // unrelated concurrent write's replication.
        #[cfg(feature = "preview_dtx")]
        {
            let mut capture = self.dtx_replication_capture.lock().unwrap();
            if let Some(buffer) = capture.as_mut() {
                buffer.push(CapturedReplication {
                    source_region: source_region.to_string(),
                    db_id: db_id.to_string(),
                    coll_id: coll_id.to_string(),
                    doc: doc.clone(),
                    is_delete,
                });
                return;
            }
        }

        // Reap any replication tasks that have already finished so the
        // JoinSet does not grow unboundedly across long-running tests with
        // delayed replication. `try_join_next` is non-blocking and returns
        // immediately when no task is ready.
        //
        // If a reaped task panicked, **do not** resume-unwind on this
        // thread: the caller is in the middle of producing the HTTP response
        // for an unrelated successful write, and unwinding here would make
        // that write appear to fail to the client even though the store has
        // committed it. Instead capture the panic into `captured_panics`
        // and surface it on the next `drain_pending_replications` call
        // (i.e. at test teardown) along with a `tracing::error!` for live
        // visibility.
        {
            let mut set = self.replication_tasks.lock().unwrap();
            let mut local = Vec::new();
            while let Some(res) = set.try_join_next() {
                if let Err(e) = res {
                    if e.is_panic() {
                        local.push(e.into_panic());
                    }
                }
            }
            if !local.is_empty() {
                tracing::error!(
                    panics = local.len(),
                    "in-memory emulator: background replication task(s) panicked; deferred to drain_pending_replications",
                );
                self.captured_panics.lock().unwrap().extend(local);
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
                // Async delayed replication. Bound concurrency via the
                // per-store semaphore so a burst of writes against a multi-
                // region account cannot spawn an unbounded `JoinSet`. Each
                // task holds one permit for the lifetime of its sleep +
                // apply, which serializes excess work behind the cap.
                let store_clone = store;
                let semaphore = Arc::clone(&self.replication_semaphore);
                self.replication_tasks.lock().unwrap().spawn(async move {
                    let _permit = semaphore.acquire_owned().await.ok();
                    tokio::time::sleep(delay).await;
                    store_clone
                        .apply_replication(&target, &source, &db, &coll, &document, is_delete);
                });
            }
        }
    }

    /// Begins buffering replication for a distributed transaction. Must be
    /// paired with [`Self::commit_dtx_replication_capture`] (replay) or
    /// [`Self::abort_dtx_replication_capture`] (discard). Called under
    /// `document_write_lock`, which serializes all emulator writes.
    #[cfg(feature = "preview_dtx")]
    pub(crate) fn begin_dtx_replication_capture(&self) {
        *self.dtx_replication_capture.lock().unwrap() = Some(Vec::new());
    }

    /// Replays every replication buffered since
    /// [`Self::begin_dtx_replication_capture`], then returns to immediate
    /// replication. Called after a distributed transaction commits.
    #[cfg(feature = "preview_dtx")]
    pub(crate) fn commit_dtx_replication_capture(self: &Arc<Self>) {
        let captured = self.dtx_replication_capture.lock().unwrap().take();
        let Some(captured) = captured else {
            return;
        };
        for entry in captured {
            self.replicate(
                &entry.source_region,
                &entry.db_id,
                &entry.coll_id,
                &entry.doc,
                entry.is_delete,
            );
        }
    }

    /// Discards every replication buffered since
    /// [`Self::begin_dtx_replication_capture`], then returns to immediate
    /// replication. Called after a distributed transaction rolls back so
    /// rolled-back writes never reach secondary regions.
    #[cfg(feature = "preview_dtx")]
    pub(crate) fn abort_dtx_replication_capture(&self) {
        *self.dtx_replication_capture.lock().unwrap() = None;
    }

    /// Applies a replicated document to a target region.
    fn apply_replication(
        self: &Arc<Self>,
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
                //
                // The proactive check uses `len >= max` to short-circuit; the
                // apply path drops at `len > max` (one slot of headroom) so a
                // strictly serial workload that pushes exactly to `max` never
                // trips the safety net counter — the proactive 429/3075 fires
                // first on the next write attempt and `dropped_replications`
                // stays at zero (which is what tests assert in teardown).
                let max = self
                    .config
                    .replication_for(source_region, target_region)
                    .max_buffered_replications();
                let mut buffer = region_store.replication_buffer.write().unwrap();
                if buffer.len() > max {
                    self.dropped_replications.fetch_add(1, Ordering::SeqCst);
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
                    if partition.is_locked() {
                        // Drop the read guards before scheduling the retry so
                        // the spawned task can re-acquire them.
                        drop(containers);
                        drop(regions);
                        self.defer_replication_during_lock(
                            target_region,
                            source_region,
                            db_id,
                            coll_id,
                            doc,
                            is_delete,
                        );
                        return;
                    }
                    apply_doc_to_partition(partition, doc, is_delete);
                }
            }
        }
    }

    /// Schedules a bounded retry of `apply_replication` while the EPK's
    /// target partition is locked for split/merge. After the lock clears
    /// `find_partition` returns the new child partition (split) or the
    /// merged successor (merge), and the doc lands in the correct place.
    /// Without this hop, late-arriving replicated writes during a split
    /// land in the BTreeMap of the parent partition that is about to be
    /// replaced — the doc snapshot taken inside `execute_split` misses it
    /// and the document is silently lost.
    fn defer_replication_during_lock(
        self: &Arc<Self>,
        target_region: &str,
        source_region: &str,
        db_id: &str,
        coll_id: &str,
        doc: &StoredDocument,
        is_delete: bool,
    ) {
        const MAX_ATTEMPTS: u32 = 50;
        const RETRY_DELAY: Duration = Duration::from_millis(20);

        let store = Arc::clone(self);
        let target = target_region.to_string();
        let source = source_region.to_string();
        let db = db_id.to_string();
        let coll = coll_id.to_string();
        let document = doc.clone();
        let semaphore = Arc::clone(&self.replication_semaphore);
        self.replication_tasks.lock().unwrap().spawn(async move {
            let _permit = semaphore.acquire_owned().await.ok();
            for attempt in 0..MAX_ATTEMPTS {
                tokio::time::sleep(RETRY_DELAY).await;
                let still_locked = {
                    let regions = store.regions.read().unwrap();
                    let Some(region_store) = regions.get(&target) else {
                        return;
                    };
                    let containers = region_store.containers.read().unwrap();
                    let key = (db.clone(), coll.clone());
                    let Some(state) = containers.get(&key) else {
                        return;
                    };
                    state
                        .find_partition(&document.epk)
                        .map(|p| p.is_locked())
                        .unwrap_or(false)
                };
                if !still_locked {
                    store.apply_replication(&target, &source, &db, &coll, &document, is_delete);
                    return;
                }
                if attempt + 1 == MAX_ATTEMPTS {
                    store.dropped_replications.fetch_add(1, Ordering::SeqCst);
                    tracing::warn!(
                        target_region = %target,
                        source_region = %source,
                        db_id = %db,
                        coll_id = %coll,
                        epk = %document.epk,
                        "in-memory emulator: dropping replicated write after extended split/merge lock",
                    );
                }
            }
        });
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

    /// Lists all databases in deterministic id order.
    pub fn list_databases(&self) -> Vec<DatabaseMetadata> {
        let mut databases: Vec<_> = self
            .region
            .databases
            .read()
            .unwrap()
            .values()
            .cloned()
            .collect();
        databases.sort_by(|a, b| a.id.cmp(&b.id));
        databases
    }

    /// Reads a container state.
    pub fn get_container(&self, db_id: &str, coll_id: &str) -> Option<ContainerStateSnapshot> {
        let containers = self.region.containers.read().unwrap();
        let key = (db_id.to_string(), coll_id.to_string());
        containers.get(&key).map(|s| ContainerStateSnapshot {
            metadata: s.metadata.clone(),
        })
    }

    /// Lists all containers for a database in deterministic id order.
    pub fn list_containers(&self, db_id: &str) -> Vec<ContainerMetadata> {
        let mut containers: Vec<_> = self
            .region
            .containers
            .read()
            .unwrap()
            .iter()
            .filter(|((db, _), _)| db == db_id)
            .map(|(_, state)| state.metadata.clone())
            .collect();
        containers.sort_by(|a, b| a.id.cmp(&b.id));
        containers
    }

    pub fn list_offers(&self) -> Vec<OfferMetadata> {
        let mut offers: Vec<_> = self
            .region
            .offers
            .read()
            .unwrap()
            .values()
            .cloned()
            .collect();
        offers.sort_by(|a, b| a.id.cmp(&b.id));
        offers
    }

    pub fn get_offer(&self, offer_id: &str) -> Option<OfferMetadata> {
        self.region.offers.read().unwrap().get(offer_id).cloned()
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
}

/// Per-region store.
pub(crate) struct RegionStore {
    pub databases: RwLock<HashMap<String, DatabaseMetadata>>,
    pub containers: RwLock<HashMap<(String, String), ContainerState>>,
    pub offers: RwLock<HashMap<String, OfferMetadata>>,
    pub paused: AtomicBool,
    pub replication_buffer: RwLock<VecDeque<PendingReplication>>,
    /// Per-region master-partition LSN counter for control-plane operations
    /// (database / container CRUD, throughput changes). Real Cosmos DB
    /// advances each region's master partition independently.
    pub master_partition_lsn: AtomicU64,
}

impl RegionStore {
    fn new() -> Self {
        Self {
            databases: RwLock::new(HashMap::new()),
            containers: RwLock::new(HashMap::new()),
            offers: RwLock::new(HashMap::new()),
            paused: AtomicBool::new(false),
            replication_buffer: RwLock::new(VecDeque::new()),
            master_partition_lsn: AtomicU64::new(0),
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

#[derive(Clone, Debug)]
pub(crate) struct OfferMetadata {
    pub id: String,
    pub rid: String,
    pub offer_resource_id: String,
    pub throughput: u32,
    pub ts: u64,
    pub self_link: String,
    pub etag: String,
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
    pub(crate) fn new(
        meta: &ContainerMetadata,
        rid_gen: &RidGenerator,
        throttling_enabled: bool,
    ) -> Self {
        let partitions = create_partitions(meta, rid_gen, throttling_enabled);
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
    /// Replicated writes that arrived while this partition was locked for a
    /// split/merge. `execute_split` / `execute_merge` drain this buffer
    /// while still holding the containers write lock so the entries
    /// participate in child construction (split: routed by EPK to the right
    /// child; merge: included in the LSN rebase pool).
    pub deferred_replications: RwLock<Vec<(StoredDocument, bool)>>,
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

    /// Restores the partition's LSN counters to previously captured values.
    ///
    /// Used by the in-memory DTX handler to roll back a partially-applied
    /// distributed transaction: applied point operations advance the LSN, so an
    /// abort must reset the counters (in addition to the document map) to leave
    /// no trace of the rolled-back writes.
    #[cfg(feature = "preview_dtx")]
    pub fn restore_counters(&self, lsn: u64, local_lsn: u64, vector_clock_version: u64) {
        self.lsn.store(lsn, Ordering::SeqCst);
        self.local_lsn.store(local_lsn, Ordering::SeqCst);
        self.vector_clock_version
            .store(vector_clock_version, Ordering::SeqCst);
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
    /// Region that originated this version of the document.
    ///
    /// Captured at write time and carried with the doc through replication
    /// for diagnostic purposes (it shows up in tracing fields). It is **not**
    /// part of the LWW comparison: `apply_doc_to_partition` resolves
    /// `(ts, lsn)` ties by keeping the locally-stored doc, deliberately
    /// avoiding any tiebreaker that depends on which region happened to
    /// produce the colliding write. See the comment in
    /// `apply_doc_to_partition` for the rationale.
    #[allow(dead_code)]
    pub source_region: String,
}

/// Returns the cached pkrange RID for `partition_id`, allocating (and
/// caching on the shared metadata) on first sight.
///
/// The cache lives on `ContainerMetadata` and is shared across every region
/// replica of the container so the same partition has the same RID in every
/// region — matching the real Cosmos DB invariant that pkrange RIDs are a
/// property of the container, not the replica.
fn pkrange_rid_for(meta: &ContainerMetadata, rid_gen: &RidGenerator, partition_id: u32) -> String {
    // Fast path: read lock, return existing.
    {
        let map = meta.pkrange_rids.read().unwrap();
        if let Some(rid) = map.get(&partition_id) {
            return rid.clone();
        }
    }
    // Slow path: take the write lock first, then check again, then allocate.
    //
    // We deliberately do not use `or_insert_with(|| next_pkrange_rid(..))`
    // because we want to avoid even *constructing* a fresh RID when another
    // thread raced us to insert one — `next_pkrange_rid` advances the
    // global per-collection counter on `RidGenerator`, and consuming a
    // counter value only to discard it under contention causes the global
    // RID space to advance faster than necessary, producing
    // non-deterministic RID values across runs (annoying for any test that
    // asserts on RID-prefix structure).
    let mut map = meta.pkrange_rids.write().unwrap();
    if let Some(rid) = map.get(&partition_id) {
        return rid.clone();
    }
    let rid = rid_gen.next_pkrange_rid(meta.numeric_db_id, meta.numeric_coll_id, partition_id);
    map.insert(partition_id, rid.clone());
    rid
}

/// Creates physical partitions for a container by dividing the EPK space equally.
fn create_partitions(
    meta: &ContainerMetadata,
    rid_gen: &RidGenerator,
    throttling_enabled: bool,
) -> Vec<PhysicalPartition> {
    let n = meta.partition_count;
    let mut partitions = Vec::with_capacity(n as usize);

    // EPK space partitioning. The boundary scheme depends on the container's
    // partition-key version: V1 hashes are encoded over a uint32 range, while
    // V2 (Hash + MultiHash) hashes occupy the 126-bit range. See
    // `compute_partition_boundaries` for details.
    let boundaries =
        compute_partition_boundaries(n, meta.partition_key.kind(), meta.partition_key.version());

    for i in 0..n {
        // `boundaries` holds the N-1 *internal* boundaries; partition i's
        // lower bound is `boundaries[i-1]` and its upper bound is
        // `boundaries[i]`. The two open ends use the sentinels.
        let min = if i == 0 {
            Epk::MIN.clone()
        } else {
            Epk::from(boundaries[(i - 1) as usize].clone())
        };
        let max = if i == n - 1 {
            Epk::MAX.clone()
        } else {
            Epk::from(boundaries[i as usize].clone())
        };

        let rid = pkrange_rid_for(meta, rid_gen, i);

        let per_partition_ru = if throttling_enabled {
            meta.provisioned_throughput_ru.map(|total| total / n)
        } else {
            None
        };

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
            deferred_replications: RwLock::new(Vec::new()),
        });
    }

    partitions
}

/// Computes the N-1 internal EPK boundary strings that divide the reachable
/// hash space into N equal ranges, returning the boundaries in lex-comparable
/// hex form. The endpoints (partition 0's lower bound and partition N-1's
/// upper bound) are represented by the [`Epk::MIN.clone()`] / [`Epk::MAX.clone()`]
/// sentinels at the call site, so they are intentionally not emitted here.
///
/// # Boundary scheme by partition key kind/version
///
/// - **V2 `Hash` and `MultiHash`**: hashes occupy the 126-bit range
///   `[0, 2^126)` (V2 hash clears the top 2 bits). Boundaries are 32-char
///   uppercase hex of evenly-spaced 126-bit values. `MultiHash` EPKs are
///   per-component 16-byte hashes concatenated, but the **first** component
///   is also a V2 hash, so partitioning by the first 32 hex chars routes
///   correctly: longer concatenated EPKs simply tie-break above the matching
///   prefix.
/// - **V1 `Hash`**: hashes are uint32 values, encoded into the EPK string
///   as `Number(hash as f64)` via `write_number_v1_binary`. Boundaries
///   are evenly-spaced uint32 values encoded the same way, so lex-compare on
///   the encoded hex matches numeric compare on the uint32 hash. Without
///   this V1-specific path, every V1 EPK (which starts with `"05..."`)
///   sorts below the V2 boundary[0] = `"10..."` and lands in partition 0,
///   defeating partitioning entirely.
fn compute_partition_boundaries(
    n: u32,
    kind: PartitionKeyKind,
    version: PartitionKeyVersion,
) -> Vec<String> {
    if n <= 1 {
        return Vec::new();
    }
    match (kind, version) {
        (PartitionKeyKind::Hash, PartitionKeyVersion::V1) => v1_boundaries(n),
        // V2 Hash and MultiHash both partition by the first 16-byte V2 hash.
        _ => v2_boundaries(n),
    }
}

/// V2 / MultiHash boundaries: evenly-spaced 126-bit values in 32-char hex.
fn v2_boundaries(n: u32) -> Vec<String> {
    let mut boundaries = Vec::with_capacity((n - 1) as usize);
    let total: u128 = 1u128 << 126;
    let step = total / n as u128;
    for i in 1..n {
        let boundary = step * i as u128;
        boundaries.push(format!("{:032X}", boundary));
    }
    boundaries
}

/// V1 boundaries: evenly-spaced uint32 hash values, each encoded via
/// `write_number_v1_binary` (the same encoding `effective_partition_key_hash_v1`
/// uses for the leading hash component of every V1 EPK).
///
/// V1 EPKs are formed as `[Number(hash_uint32 as f64)] [components...]`, so
/// any real EPK whose leading-hash equals a boundary's encoded uint32 sorts
/// strictly *above* the boundary (the EPK has the appended-component bytes,
/// the boundary does not). That matches the half-open `[min_inclusive,
/// max_exclusive)` convention used by [`PhysicalPartition::contains_epk`].
fn v1_boundaries(n: u32) -> Vec<String> {
    let mut boundaries = Vec::with_capacity((n - 1) as usize);
    let total: u64 = 1u64 << 32;
    let step = total / n as u64;
    for i in 1..n {
        let boundary_hash = (step * i as u64) as u32;
        let mut buf = Vec::new();
        crate::models::partition_key::write_number_v1_binary(boundary_hash as f64, &mut buf);
        boundaries.push(bytes_to_hex_upper(&buf));
    }
    boundaries
}

/// Uppercase hex encoding of a byte slice. Local helper so the V1 boundary
/// code does not depend on the production crate's private hex helpers.
fn bytes_to_hex_upper(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        let _ = write!(&mut s, "{:02X}", b);
    }
    s
}
/// Returns true if `epk` represents the open lower bound of the EPK space.
///
/// Intentionally treats *any* all-`'0'` hex string as the lower-bound sentinel
/// (and the empty string as the canonical sentinel), so the bound parsing in
/// `compute_epk_midpoint` accepts every spelling we ever produce. A V2 EPK is
/// 32 hex chars with the top 2 bits cleared, so its first nibble is in
/// `0x0..=0x3` — a 32-char "00000000000000000000000000000000" hash is
/// astronomically unlikely (probability ~2^-126) and would, if it ever
/// occurred, simply be classified as the open lower bound and routed to
/// partition 0, which is exactly where a value of 0 belongs. Do not flag this
/// as a bug; tightening the check would require differentiating "sentinel
/// open bound" from "real all-zeros hash" everywhere boundaries flow, which
/// has no observable upside.
fn is_epk_min(epk: &Epk) -> bool {
    epk.as_bytes().is_empty() || epk.as_bytes().iter().all(|&b| b == 0)
}

/// Returns true if `epk` represents the open upper bound of the EPK space.
///
/// Mirrors `is_epk_min`: accepts both the canonical `"FF"` sentinel and the
/// fully-expanded 32-char "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF" form.
fn is_epk_max(epk: &Epk) -> bool {
    let s = epk.to_hex();
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
        self.split_partition_internal(db_id, coll_id, partition_id, None, min_lock_duration);
    }

    /// Splits a physical partition at an explicit EPK boundary. Test-only.
    #[doc(hidden)]
    pub fn split_partition_at_epk(
        self: &Arc<Self>,
        db_id: &str,
        coll_id: &str,
        partition_id: u32,
        split_epk: Epk,
        min_lock_duration: Duration,
    ) {
        self.split_partition_internal(
            db_id,
            coll_id,
            partition_id,
            Some(split_epk),
            min_lock_duration,
        );
    }

    fn split_partition_internal(
        self: &Arc<Self>,
        db_id: &str,
        coll_id: &str,
        partition_id: u32,
        split_epk: Option<Epk>,
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
        let key = ControlPlaneTaskKey {
            db: db_id.to_string(),
            coll: coll_id.to_string(),
            partitions: vec![partition_id],
        };
        let handle = tokio::spawn(async move {
            let _guard = lock.lock().await;
            if !min_lock_duration.is_zero() {
                tokio::time::sleep(min_lock_duration).await;
            }
            // execute_split does the actual doc redistribution under the lock,
            // then unlocks partitions when done
            store.execute_split(&db, &coll, partition_id, split_epk);
        });
        self.control_plane_tasks.lock().unwrap().push((key, handle));
    }

    /// Performs the actual split after the lock period.
    fn execute_split(&self, db_id: &str, coll_id: &str, partition_id: u32, split_epk: Option<Epk>) {
        // Local-only enum used to ferry preview state out of a regions read
        // guard so we can drop the guard before re-acquiring it on the abort
        // path. Avoids recursive same-thread RwLock::read (unspecified in std).
        enum SplitPreview {
            Found {
                parent_lsn: u64,
                parent_version: u64,
                parent_min: Epk,
                parent_max: Epk,
                midpoint: Epk,
                child_id_1: u32,
                child_id_2: u32,
                child_rid_1: String,
                child_rid_2: String,
                total_throughput: Option<u32>,
            },
            AbortUnlock,
        }
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
                    let pk_kind = state.metadata.partition_key.kind();
                    let pk_version = state.metadata.partition_key.version();
                    let midpoint = match split_epk.as_ref() {
                        Some(epk) if *epk > parent_min && *epk < parent_max => epk.clone(),
                        Some(epk) => {
                            tracing::error!(
                                db_id = db_id,
                                coll_id = coll_id,
                                partition_id = partition_id,
                                split_epk = %epk,
                                parent_min = %parent_min,
                                parent_max = %parent_max,
                                "in-memory emulator: aborting split — explicit split EPK is outside parent range",
                            );
                            found = Some(SplitPreview::AbortUnlock);
                            break;
                        }
                        None => match compute_epk_midpoint(
                            &parent_min,
                            &parent_max,
                            pk_kind,
                            pk_version,
                        ) {
                            Ok(m) => m,
                            Err(err) => {
                                tracing::error!(
                                    error = %err,
                                    db_id = db_id,
                                    coll_id = coll_id,
                                    partition_id = partition_id,
                                    "in-memory emulator: aborting split — unlocking parent",
                                );
                                // Defer unlock until after the outer read guard on
                                // `self.regions` is dropped. Re-acquiring a read
                                // lock on the same RwLock from the same thread
                                // while another guard is live has unspecified
                                // behavior in std and may deadlock.
                                found = Some(SplitPreview::AbortUnlock);
                                break;
                            }
                        },
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
                    found = Some(SplitPreview::Found {
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
                    });
                    break;
                }
            }
            found
        };
        let SplitPreview::Found {
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
        } = (match preview {
            Some(SplitPreview::Found { .. }) => preview.unwrap(),
            Some(SplitPreview::AbortUnlock) => {
                // Outer `regions` read guard has been dropped here.
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
            None => return,
        })
        else {
            unreachable!()
        };
        let child_lsn = parent_lsn + 1;
        // One ETag for the whole split, applied to every region's container
        // metadata below so the pkrange routing map's change-feed observes the
        // topology change consistently across regions.
        let new_container_etag = new_etag();

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

                // Redistribute pending forced-session-not-available markers
                // by EPK so a marker the test set on the parent before the
                // split still fires when the relevant child handles the read.
                let parent_forced = parent.session_state.snapshot_forced_epks();
                let child1_session = SessionState::new();
                let child2_session = SessionState::new();
                for epk_str in parent_forced {
                    let epk = Epk::from(epk_str.clone());
                    if epk < midpoint {
                        child1_session.set_force_unavailable_for(&epk_str);
                    } else {
                        child2_session.set_force_unavailable_for(&epk_str);
                    }
                }

                let n = state.physical_partitions.len() as f64 + 1.0;
                let per_partition_ru = if self.config.throttling_enabled() {
                    total_throughput.map(|total| total / (n as u32))
                } else {
                    None
                };

                // Per-region local LSN: child inherits *this* region's
                // applied-mutation count + 1. Capturing globally on the
                // preview pass would inflate non-write-region counts.
                let child_local_lsn = parent.current_local_lsn() + 1;

                let child1 = PhysicalPartition {
                    id: child_id_1,
                    epk_min: parent_min.clone(),
                    epk_max: midpoint.clone(),
                    lsn: AtomicU64::new(child_lsn),
                    local_lsn: AtomicU64::new(child_local_lsn),
                    vector_clock_version: AtomicU64::new(parent_version),
                    documents: RwLock::new(docs_1),
                    session_state: child1_session,
                    rid: child_rid_1.clone(),
                    rid_prefix: child_id_1,
                    throughput_fraction: 1.0 / n,
                    parents: vec![partition_id],
                    locked: AtomicBool::new(false),
                    throughput_tracker: per_partition_ru.map(ThroughputTracker::new),
                    deferred_replications: RwLock::new(Vec::new()),
                };

                let child2 = PhysicalPartition {
                    id: child_id_2,
                    epk_min: midpoint.clone(),
                    epk_max: parent_max.clone(),
                    lsn: AtomicU64::new(child_lsn),
                    local_lsn: AtomicU64::new(child_local_lsn),
                    vector_clock_version: AtomicU64::new(parent_version),
                    documents: RwLock::new(docs_2),
                    session_state: child2_session,
                    rid: child_rid_2.clone(),
                    rid_prefix: child_id_2,
                    throughput_fraction: 1.0 / n,
                    parents: vec![partition_id],
                    locked: AtomicBool::new(false),
                    throughput_tracker: per_partition_ru.map(ThroughputTracker::new),
                    deferred_replications: RwLock::new(Vec::new()),
                };

                // Drain any replicated writes that arrived while the parent
                // was locked, route by EPK to whichever child covers them,
                // and apply via the standard LWW path. The drain happens
                // under the containers write lock, so no concurrent writer
                // can push new entries to the parent buffer between the
                // drain and the parent removal.
                let deferred: Vec<(StoredDocument, bool)> = {
                    let mut buf = parent.deferred_replications.write().unwrap();
                    std::mem::take(&mut *buf)
                };
                for (doc, is_delete) in deferred {
                    let target = if doc.epk < midpoint { &child1 } else { &child2 };
                    apply_doc_to_partition(target, &doc, is_delete);
                }

                state.physical_partitions.remove(parent_idx);
                state.physical_partitions.push(child1);
                state.physical_partitions.push(child2);
                // Bump the container ETag so the pkrange routing-map change-feed
                // reflects the new topology. The driver refreshes its pkrange
                // cache incrementally via `If-None-Match`; without a new ETag the
                // emulator answers `304 Not Modified` and the driver keeps
                // resolving to the now-gone parent range, looping until it
                // exhausts split retries on a continuation issued before the split.
                state.metadata.etag = new_container_etag.clone();
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
        let key = ControlPlaneTaskKey {
            db: db_id.to_string(),
            coll: coll_id.to_string(),
            partitions: vec![partition_id_a, partition_id_b],
        };
        let handle = tokio::spawn(async move {
            let _guard = lock.lock().await;
            if !min_lock_duration.is_zero() {
                tokio::time::sleep(min_lock_duration).await;
            }
            store.execute_merge(&db, &coll, partition_id_a, partition_id_b);
        });
        self.control_plane_tasks.lock().unwrap().push((key, handle));
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

                // Drain deferred replications that arrived while the parents
                // were locked, BEFORE merging docs. The drained entries
                // participate in the rebase below so they receive fresh
                // child-partition LSNs alongside the merged-in originals.
                // Without this, a deferred apply that wakes up after the
                // lock clears would land on the child carrying its
                // source-region's pre-merge LSN — and the LWW comparison on
                // `(ts, lsn, source_region)` against rebased entries
                // (LSNs 1..=N) becomes meaningless.
                let mut deferred_lower: Vec<(StoredDocument, bool)> = {
                    let mut buf = lower.deferred_replications.write().unwrap();
                    std::mem::take(&mut *buf)
                };
                let mut deferred_upper: Vec<(StoredDocument, bool)> = {
                    let mut buf = upper.deferred_replications.write().unwrap();
                    std::mem::take(&mut *buf)
                };

                // Per-region local LSN: child inherits `max(this region's
                // parents)` plus any deferred mutations applied during the
                // merge. Captured per-region (not from the preview pass) so
                // non-write-region replicas don't inflate.
                let parents_local_lsn = lower.current_local_lsn().max(upper.current_local_lsn());

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

                // Apply each deferred entry into the merged_docs pool with
                // LWW-on-(ts, lsn) so a stale buffered write does not
                // overwrite a fresher in-store doc. On a `(ts, lsn)` tie the
                // existing entry wins, matching `apply_doc_to_partition`.
                // Deletes remove the matching entry. The result then
                // participates in the LSN rebase below.
                let mut applied_deferred = 0u64;
                for (doc, is_delete) in deferred_lower.drain(..).chain(deferred_upper.drain(..)) {
                    let bucket = merged_docs.entry(doc.epk.clone()).or_default();
                    let should_apply = match bucket.get(&doc.id) {
                        None => true,
                        Some(existing) => (doc.ts, doc.lsn) > (existing.ts, existing.lsn),
                    };
                    if !should_apply {
                        continue;
                    }
                    if is_delete {
                        bucket.remove(&doc.id);
                    } else {
                        bucket.insert(doc.id.clone(), doc);
                    }
                    applied_deferred += 1;
                }
                // Strip empty buckets that resulted from delete-only drains.
                merged_docs.retain(|_, v| !v.is_empty());

                // Rebase document LSNs onto the child partition's reset LSN
                // counter. Sort by `(ts, original_lsn, id)` to preserve the
                // original ordering, then assign 1..=N in that order.
                let mut all_entries: Vec<(Epk, String, StoredDocument)> = merged_docs
                    .iter()
                    .flat_map(|(epk, items)| {
                        items
                            .iter()
                            .map(move |(id, d)| (epk.clone(), id.clone(), d.clone()))
                    })
                    .collect();
                all_entries.sort_by(|a, b| (a.2.ts, a.2.lsn, &a.1).cmp(&(b.2.ts, b.2.lsn, &b.1)));
                let mut rebased_docs: BTreeMap<Epk, BTreeMap<String, StoredDocument>> =
                    BTreeMap::new();
                let mut next_lsn: u64 = 0;
                for (epk, id, mut doc) in all_entries {
                    next_lsn += 1;
                    doc.lsn = next_lsn;
                    rebased_docs.entry(epk).or_default().insert(id, doc);
                }
                let merged_docs = rebased_docs;
                // Child's high-water LSN matches the largest assigned doc LSN
                // (or 1 for an empty merge so reads still see a non-zero LSN).
                let child_initial_lsn = next_lsn.max(1);
                let child_local_lsn = parents_local_lsn + applied_deferred + 1;

                // Merge forced-session-not-available markers from both parents.
                let merged_session = SessionState::new();
                for epk_str in lower.session_state.snapshot_forced_epks() {
                    merged_session.set_force_unavailable_for(&epk_str);
                }
                for epk_str in upper.session_state.snapshot_forced_epks() {
                    merged_session.set_force_unavailable_for(&epk_str);
                }

                let n = state.physical_partitions.len() as f64 - 1.0;
                let per_partition_ru = if self.config.throttling_enabled() {
                    total_throughput.map(|total| total / (n.max(1.0) as u32))
                } else {
                    None
                };

                let child = PhysicalPartition {
                    id: child_id,
                    epk_min: merged_min.clone(),
                    epk_max: merged_max.clone(),
                    lsn: AtomicU64::new(child_initial_lsn),
                    local_lsn: AtomicU64::new(child_local_lsn),
                    vector_clock_version: AtomicU64::new(child_version),
                    documents: RwLock::new(merged_docs),
                    session_state: merged_session,
                    rid: child_rid.clone(),
                    rid_prefix: child_id,
                    throughput_fraction: 1.0 / n.max(1.0),
                    parents: vec![partition_id_a, partition_id_b],
                    locked: AtomicBool::new(false),
                    throughput_tracker: per_partition_ru.map(ThroughputTracker::new),
                    deferred_replications: RwLock::new(Vec::new()),
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
/// Returns `Err` if either bound is not parseable. Only V2 `Hash` /
/// `MultiHash` boundaries (32-char hex of a 126-bit value) are supported;
/// V1 `Hash` boundaries (variable-length encoded `Number` values) cannot
/// be midpoint-bisected here and will return `Err`. Splits on V1
/// containers are therefore not supported in this emulator — the split task
/// surfaces the error and unlocks the parent partition rather than wedging
/// it. V1 routing itself is fully supported via [`v1_boundaries`].
///
/// Callers in the split path
/// must surface the error and unlock the parent partition rather than panic;
/// a corrupt bound at the split site would otherwise wedge the partition
/// `locked = true` forever inside a spawned task with no caller to observe
/// the panic.
/// Computes the EPK midpoint between two EPK bounds.
///
/// Dispatches on `(kind, version)`:
/// - V2 `Hash` / `MultiHash` (32-char hex of a 126-bit value): bisects in
///   the 128-bit numeric space.
/// - V1 `Hash` (variable-length encoded `Number` value): decodes both bounds
///   to their underlying `u32` hash, bisects, and re-encodes via
///   [`write_number_v1_binary`](crate::models::partition_key::write_number_v1_binary).
///
/// Returns `Err` for any other combination — only `Hash` partitioning is
/// splittable in this emulator. Callers in the split path must surface the
/// error and unlock the parent partition rather than panic; a corrupt bound
/// at the split site would otherwise wedge the partition `locked = true`
/// forever inside a spawned task with no caller to observe the panic.
fn compute_epk_midpoint(
    min: &Epk,
    max: &Epk,
    kind: PartitionKeyKind,
    version: PartitionKeyVersion,
) -> Result<Epk, String> {
    match (kind, version) {
        (PartitionKeyKind::Hash, PartitionKeyVersion::V1) => compute_epk_midpoint_v1(min, max),
        // V2 Hash and MultiHash both bisect in the 126-bit hex space.
        (PartitionKeyKind::Hash, PartitionKeyVersion::V2) | (PartitionKeyKind::MultiHash, _) => {
            compute_epk_midpoint_v2(min, max)
        }
        (k, v) => Err(format!(
            "split unsupported for partition key (kind={:?}, version={:?})",
            k, v
        )),
    }
}

fn compute_epk_midpoint_v2(min: &Epk, max: &Epk) -> Result<Epk, String> {
    let parse = |epk: &Epk, label: &str| -> Result<u128, String> {
        if is_epk_min(epk) {
            return Ok(0u128);
        }
        if is_epk_max(epk) {
            return Ok(1u128 << 126);
        }
        u128::from_str_radix(&epk.to_hex(), 16)
            .map_err(|e| format!("corrupted EPK partition bound {}={:?}: {e}", label, epk))
    };
    let min_val = parse(min, "min")?;
    let max_val = parse(max, "max")?;
    // V2 EPKs are 126-bit values (top 2 bits cleared by the production
    // hasher). If a future change broadens that range, midpoint computation
    // would silently produce a value outside `[parent_min, parent_max]` and
    // splits would land docs in the wrong child. Trip in debug builds so the
    // regression surfaces in CI before reaching production tests.
    debug_assert!(
        min_val <= max_val && max_val <= (1u128 << 126),
        "EPK bounds out of range: min={min_val:032X} max={max_val:032X}"
    );
    // Safe midpoint: `min/2 + max/2` loses 1 bit when both operands are odd.
    // Add the missing carry explicitly.
    let mid = min_val / 2 + max_val / 2 + ((min_val & 1) & (max_val & 1));
    Ok(Epk::from(format!("{:032X}", mid)))
}

/// V1 split midpoint.
///
/// V1 boundaries from [`v1_boundaries`] are uint32 hash values fed through
/// [`write_number_v1_binary`](crate::models::partition_key::write_number_v1_binary)
/// (with `value as f64`). Decoding is the inverse of that pipeline:
///
/// 1. Hex → bytes.
/// 2. Strip the leading [`component::NUMBER`] type marker, then reassemble
///    the original 64-bit `encode_double_as_uint64` payload by concatenating
///    `byte[1]` (top 8 bits) with the top 7 bits of each subsequent byte
///    placed at descending shift positions `49, 42, …, 0`. Missing trailing
///    bytes (the encoder stops emitting once the remaining payload is zero)
///    contribute zeros.
/// 3. Reverse `encode_double_as_uint64` and recover the underlying `u32`.
///
/// Sentinels (`Epk::MIN.clone()` / `Epk::MAX.clone()`) map to `0` and `u32::MAX`. The
/// midpoint is computed in `u32` space (wrap-safe via `u64`) and re-encoded
/// the same way `v1_boundaries` produces its bounds, so the resulting EPK is
/// indistinguishable from a freshly-provisioned 2-partition layout for the
/// child range.
fn compute_epk_midpoint_v1(min: &Epk, max: &Epk) -> Result<Epk, String> {
    let parse = |epk: &Epk, label: &str| -> Result<u32, String> {
        if is_epk_min(epk) {
            return Ok(0u32);
        }
        if is_epk_max(epk) {
            return Ok(u32::MAX);
        }
        decode_v1_number_hex_to_u32(&epk.to_hex())
            .map_err(|e| format!("corrupted V1 EPK partition bound {}={:?}: {e}", label, epk))
    };
    let min_val = parse(min, "min")?;
    let max_val = parse(max, "max")?;
    if min_val >= max_val {
        return Err(format!(
            "V1 split midpoint requires min < max (min={min_val} max={max_val})"
        ));
    }
    // Bisect in u32 space; promote to u64 to avoid overflow when the parent
    // spans the full range.
    let mid = ((min_val as u64 + max_val as u64) / 2) as u32;
    let mut buf = Vec::new();
    crate::models::partition_key::write_number_v1_binary(mid as f64, &mut buf);
    Ok(Epk::from(bytes_to_hex_upper(&buf)))
}

/// V1 EPK type-marker byte for `Number` components — must match
/// `partition_key::component::NUMBER`. The latter is private so we mirror
/// the literal here; a unit test (`v1_number_marker_matches_encoder`)
/// pins the value to the encoder's output and fails fast if the encoder
/// ever changes the marker.
const V1_NUMBER_MARKER: u8 = 0x05;

/// Inverse of `write_number_v1_binary` for `value as f64` where `value: u32`.
/// Returns the original `u32` (rejects non-finite or out-of-range decodes).
fn decode_v1_number_hex_to_u32(hex: &str) -> Result<u32, String> {
    if !hex.len().is_multiple_of(2) {
        return Err(format!("V1 EPK hex has odd length: {hex}"));
    }
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    for i in (0..hex.len()).step_by(2) {
        let byte = u8::from_str_radix(&hex[i..i + 2], 16)
            .map_err(|e| format!("V1 EPK contains non-hex bytes: {hex} ({e})"))?;
        bytes.push(byte);
    }
    if bytes.len() < 2 {
        return Err(format!("V1 Number EPK too short: {hex}"));
    }
    if bytes[0] != V1_NUMBER_MARKER {
        return Err(format!(
            "not a Number-typed V1 EPK (first byte = 0x{:02X}): {hex}",
            bytes[0]
        ));
    }

    // byte[1] is the top 8 bits of the encoded u64 payload.
    let mut encoded: u64 = (bytes[1] as u64) << 56;
    // Each subsequent byte contributes 7 bits (its top 7 bits, i.e. bits 7..1)
    // placed at descending shift positions: 49, 42, 35, 28, 21, 14, 7, 0.
    // The encoder drops trailing bytes once the remaining payload is zero,
    // so missing bytes implicitly contribute zeros.
    for (k, &b) in bytes[2..].iter().enumerate() {
        let shift: i32 = 49 - 7 * k as i32;
        if shift < 0 {
            // Past the bottom of the payload — extra bytes must not corrupt
            // the decoded value. (`v1_boundaries` never produces them, but a
            // defensive bound prevents hex tampering from skewing splits.)
            break;
        }
        let payload_bits = ((b >> 1) & 0x7F) as u64;
        encoded |= payload_bits << shift;
    }

    // Reverse `encode_double_as_uint64`:
    //   non-negative input → result has high bit set, recover via XOR
    //   negative input → result was `!v + 1`, recover via `!(encoded - 1)`
    let bits = if encoded & 0x8000_0000_0000_0000 != 0 {
        encoded ^ 0x8000_0000_0000_0000
    } else {
        !(encoded.wrapping_sub(1))
    };
    let value = f64::from_le_bytes(bits.to_le_bytes());
    if !value.is_finite() {
        return Err(format!(
            "decoded V1 Number is non-finite: {value} (hex={hex})"
        ));
    }
    if value < 0.0 || value > u32::MAX as f64 {
        return Err(format!(
            "decoded V1 Number outside u32 range: {value} (hex={hex})"
        ));
    }
    Ok(value as u32)
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
                .build()
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

    /// Regression test for the V1 routing bug: every V1 EPK starts with the
    /// Number type marker byte `05`, so under the old V2-only boundary
    /// scheme (`"10..."`, `"20..."`, `"30..."` for 4 partitions) every
    /// V1 EPK lex-compared below boundary[0] and landed in partition 0.
    /// With version-aware boundaries, V1 EPKs distribute across all
    /// partitions according to the uint32 hash range.
    #[test]
    fn partitions_distribute_across_full_v1_epk_space() {
        use super::super::epk::{compute_epk, parse_partition_key_header};
        use crate::models::PartitionKeyDefinition;
        let pk_def: PartitionKeyDefinition = serde_json::from_value(serde_json::json!({
            "paths": ["/pk"], "kind": "Hash", "version": 1
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
                .build()
                .unwrap(),
        );
        let region_ref = store.region("r1").unwrap();
        let mut counts = vec![0usize; 8];
        for i in 0..2000 {
            let pk_json = format!("[\"v1key-{}\"]", i);
            let comps = parse_partition_key_header(&pk_json).unwrap();
            let epk = compute_epk(
                &comps,
                crate::models::PartitionKeyKind::Hash,
                crate::models::PartitionKeyVersion::V1,
            );
            region_ref
                .with_container("db", "c", |state| {
                    let p = state
                        .find_partition(&epk)
                        .unwrap_or_else(|| panic!("V1 EPK {} did not route to any partition", epk));
                    counts[p.id as usize] += 1;
                })
                .unwrap();
        }
        for (i, c) in counts.iter().enumerate() {
            assert!(
                *c > 0,
                "V1 partition {} had 0 docs (counts={:?}) — V1 EPK routing is broken",
                i,
                counts,
            );
        }
        // Every partition should see at least ~1% of the load. A perfectly
        // uniform distribution would put 250 in each; we accept >= 25 (10%
        // of expected) as a sanity bound that catches all-in-one-partition
        // regressions without flaking on minor hash skew.
        for (i, c) in counts.iter().enumerate() {
            assert!(
                *c >= 25,
                "V1 partition {} got only {} docs (counts={:?}); expected at least ~25",
                i,
                c,
                counts,
            );
        }
    }

    /// MultiHash EPKs concatenate per-component 16-byte V2 hashes, so the
    /// first 32 hex chars are the first component's V2 hash. The existing
    /// 126-bit boundary scheme partitions correctly by that first component.
    #[test]
    fn partitions_distribute_across_multihash_epk_space() {
        use super::super::epk::{compute_epk, parse_partition_key_header};
        use crate::models::PartitionKeyDefinition;
        let pk_def: PartitionKeyDefinition = serde_json::from_value(serde_json::json!({
            "paths": ["/tenant", "/user"], "kind": "MultiHash", "version": 2
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
                .build()
                .unwrap(),
        );
        let region_ref = store.region("r1").unwrap();
        let mut counts = vec![0usize; 8];
        for i in 0..2000 {
            // Vary tenant to drive distribution; user fixed.
            let pk_json = format!("[\"tenant-{}\", \"user1\"]", i);
            let comps = parse_partition_key_header(&pk_json).unwrap();
            let epk = compute_epk(
                &comps,
                crate::models::PartitionKeyKind::MultiHash,
                crate::models::PartitionKeyVersion::V2,
            );
            region_ref
                .with_container("db", "c", |state| {
                    let p = state.find_partition(&epk).unwrap_or_else(|| {
                        panic!("MultiHash EPK {} did not route to any partition", epk)
                    });
                    counts[p.id as usize] += 1;
                })
                .unwrap();
        }
        for (i, c) in counts.iter().enumerate() {
            assert!(
                *c > 0,
                "MultiHash partition {} had 0 docs (counts={:?})",
                i,
                counts
            );
        }
    }

    /// V1 boundaries must each start with the Number type marker `"05"`
    /// (because they encode `Number(uint32_hash as f64)` via
    /// `write_number_v1_binary`). If they don't, real V1 EPKs (which all
    /// start with `"05"`) won't lex-compare meaningfully against them.
    #[test]
    fn v1_boundaries_share_prefix_with_v1_epk_format() {
        let bs = super::v1_boundaries(4);
        assert_eq!(bs.len(), 3);
        for b in &bs {
            assert!(
                b.starts_with("05"),
                "V1 boundary {b} must start with Number type marker '05'",
            );
        }
        // Boundaries must be strictly increasing.
        for w in bs.windows(2) {
            assert!(
                w[0] < w[1],
                "V1 boundaries not strictly increasing: {:?}",
                bs
            );
        }
    }

    /// V2 boundaries must be 32-char hex with the top 2 bits of the first
    /// nibble cleared (range `[00..40)`).
    #[test]
    fn v2_boundaries_within_126_bit_range() {
        let bs = super::v2_boundaries(4);
        assert_eq!(bs.len(), 3);
        for b in &bs {
            assert_eq!(b.len(), 32, "V2 boundary must be 32 hex chars: {b}");
            let first_nibble = u8::from_str_radix(&b[..1], 16).unwrap();
            assert!(
                first_nibble < 0x4,
                "V2 boundary {b} first nibble {:X} must be <0x4 (top 2 bits cleared)",
                first_nibble,
            );
        }
    }

    /// Pin `V1_NUMBER_MARKER` to the encoder's emitted type byte. If the
    /// production encoder ever changes the Number marker, the V1 decoder
    /// will silently misclassify input — fail loudly here instead.
    #[test]
    fn v1_number_marker_matches_encoder() {
        let mut buf = Vec::new();
        crate::models::partition_key::write_number_v1_binary(0.0_f64, &mut buf);
        assert_eq!(
            buf[0],
            super::V1_NUMBER_MARKER,
            "V1_NUMBER_MARKER (0x{:02X}) must match the encoder's first byte (0x{:02X})",
            super::V1_NUMBER_MARKER,
            buf[0],
        );
    }

    /// Round-trip every value used by `v1_boundaries(n)` for a few partition
    /// counts: encode → hex → `decode_v1_number_hex_to_u32` recovers the
    /// original `u32`. Covers the only inputs `compute_epk_midpoint_v1` ever
    /// has to invert in practice.
    #[test]
    fn decode_v1_number_round_trips_v1_boundaries() {
        for n in [2u32, 4, 8, 16, 100, 1024] {
            let bs = super::v1_boundaries(n);
            for (i, hex) in bs.iter().enumerate() {
                let expected = ((1u64 << 32) / n as u64) as u32 * (i as u32 + 1);
                let recovered = super::decode_v1_number_hex_to_u32(hex)
                    .unwrap_or_else(|e| panic!("decode failed for n={n} i={i} hex={hex}: {e}"));
                assert_eq!(
                    recovered, expected,
                    "decode mismatch for n={n} i={i}: hex={hex} expected={expected} got={recovered}"
                );
            }
        }
    }

    /// `decode_v1_number_hex_to_u32` must round-trip every value
    /// `compute_epk_midpoint_v1` could produce or consume — including 0,
    /// `u32::MAX`, and a spread of arbitrary u32s — so a split's midpoint
    /// re-encoded boundary will decode identically on the next split.
    #[test]
    fn decode_v1_number_round_trips_arbitrary_u32() {
        let cases: &[u32] = &[
            0,
            1,
            127,
            128,
            255,
            256,
            65_535,
            65_536,
            16_777_215,
            16_777_216,
            (u32::MAX / 2),
            u32::MAX - 1,
            u32::MAX,
        ];
        for &v in cases {
            let mut buf = Vec::new();
            crate::models::partition_key::write_number_v1_binary(v as f64, &mut buf);
            let hex = super::bytes_to_hex_upper(&buf);
            let back = super::decode_v1_number_hex_to_u32(&hex)
                .unwrap_or_else(|e| panic!("decode failed for v={v} hex={hex}: {e}"));
            assert_eq!(back, v, "round-trip failed for v={v} hex={hex} back={back}");
        }
    }

    /// `compute_epk_midpoint_v1` must produce a boundary that lies strictly
    /// between `min` and `max` in the encoded V1 EPK lex ordering — that is
    /// what enables `PhysicalPartition::contains_epk`'s half-open interval
    /// check after a split. Verify on the full-range parent (`Epk::MIN.clone()`,
    /// `Epk::MAX.clone()`) and on a narrower parent.
    #[test]
    fn compute_epk_midpoint_v1_lies_between_bounds() {
        use crate::models::{PartitionKeyKind, PartitionKeyVersion};

        // Full range: midpoint of u32 [0, MAX] is u32::MAX / 2 = 0x7FFFFFFF.
        let mid_full = super::compute_epk_midpoint(
            &Epk::MIN,
            &Epk::MAX,
            PartitionKeyKind::Hash,
            PartitionKeyVersion::V1,
        )
        .expect("full-range V1 midpoint");
        let mid_full_u32 = super::decode_v1_number_hex_to_u32(&mid_full.to_hex()).unwrap();
        assert_eq!(mid_full_u32, u32::MAX / 2);

        // Narrow range: encode two u32 hashes as boundaries and verify the
        // midpoint decodes to the arithmetic mean.
        let lo_u32 = 1_000_000_000u32;
        let hi_u32 = 3_000_000_000u32;
        let encode = |v: u32| -> Epk {
            let mut buf = Vec::new();
            crate::models::partition_key::write_number_v1_binary(v as f64, &mut buf);
            Epk::from(super::bytes_to_hex_upper(&buf))
        };
        let mid_narrow = super::compute_epk_midpoint(
            &encode(lo_u32),
            &encode(hi_u32),
            PartitionKeyKind::Hash,
            PartitionKeyVersion::V1,
        )
        .expect("narrow V1 midpoint");
        let mid_narrow_u32 = super::decode_v1_number_hex_to_u32(&mid_narrow.to_hex()).unwrap();
        assert_eq!(mid_narrow_u32, ((lo_u32 as u64 + hi_u32 as u64) / 2) as u32);

        // Lex-order check: the encoded midpoint must sit strictly between
        // the encoded bounds. (V1 encoding is order-preserving, so this is
        // a function-level invariant — not an artifact of any single test
        // input.)
        let lo_hex = encode(lo_u32);
        let hi_hex = encode(hi_u32);
        assert!(
            lo_hex.to_hex() < mid_narrow.to_hex() && mid_narrow.to_hex() < hi_hex.to_hex(),
            "V1 midpoint not strictly between bounds: lo={} mid={} hi={}",
            lo_hex.to_hex(),
            mid_narrow.to_hex(),
            hi_hex.to_hex(),
        );
    }

    /// Non-hash partition kinds (e.g. `Range`) cannot be split — surface
    /// the error so the split task can unlock the parent and retry-or-stop
    /// instead of producing nonsense bounds.
    #[test]
    fn compute_epk_midpoint_rejects_non_hash() {
        use crate::models::{PartitionKeyKind, PartitionKeyVersion};
        let err = super::compute_epk_midpoint(
            &Epk::MIN,
            &Epk::MAX,
            PartitionKeyKind::Range,
            PartitionKeyVersion::V2,
        )
        .unwrap_err();
        assert!(err.contains("split unsupported"), "unexpected error: {err}");
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
