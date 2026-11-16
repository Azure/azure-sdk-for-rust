// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Pooled resources: one account and one cell database per service profile per
//! run, plus the shared containers inside it.
//!
//! * The account is opened once per `(backend, service profile)` cell.
//! * The cell database holds every `SharedContainer` and every
//!   `ExclusiveContainer` of the cell. It is deleted when the process exits
//!   normally and swept on a later run otherwise.
//! * With `--no-cleanup` ([`NO_CLEANUP_ARG`]), the run deletes nothing: not
//!   exclusive resources, not cell databases, and not old leftovers.
//! * Shared containers are created and seeded once per profile, then reused by
//!   every spec that asks for them.
//!
//! Pooled resources serve many trials, so they're set up outside every
//! trial's `tracing` span: their logs go to `run-logs.json`, not to whichever
//! trial needed them first.
//!
//! Every database and container the framework creates is named
//! `spec-<UUIDv7>`. The UUID's timestamp tells a sweep how old a leftover is,
//! without any other bookkeeping.

use crate::{
    backend::{Backend, BackendAccount, ClientOptions},
    profiles::ContainerProfile,
    testdata::Dataset,
    ServiceProfile, SpecResult,
};
use azure_data_cosmos::{ContainerClient, CosmosClient};
use std::{
    collections::HashMap,
    future::Future,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, OnceLock,
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::sync::OnceCell;
use uuid::Uuid;

/// Prefix of every resource the framework creates.
pub(crate) const RESOURCE_PREFIX: &str = "spec-";

/// The command-line flag that keeps every resource the run creates.
pub const NO_CLEANUP_ARG: &str = "--no-cleanup";

static CLEANUP: AtomicBool = AtomicBool::new(true);

/// Turns cleanup off for the rest of the run.
pub(crate) fn disable_cleanup() {
    CLEANUP.store(false, Ordering::Relaxed);
}

/// Whether the run deletes the resources it creates. `false` with `--no-cleanup`.
pub(crate) fn cleanup() -> bool {
    CLEANUP.load(Ordering::Relaxed)
}

/// Leftover databases older than this are deleted when a cell opens.
const SWEEP_AGE: Duration = Duration::from_secs(2 * 60 * 60);

/// A unique, time-ordered ID for this run, a UUIDv7.
pub(crate) fn run_id() -> &'static str {
    static RUN_ID: OnceLock<String> = OnceLock::new();
    RUN_ID.get_or_init(|| Uuid::now_v7().to_string())
}

/// A new resource name: `spec-<UUIDv7>`.
pub(crate) fn resource_name() -> String {
    format!("{RESOURCE_PREFIX}{}", Uuid::now_v7())
}

/// When a resource named by [`resource_name`] was created, from its UUIDv7.
///
/// Returns `None` for names the framework did not create.
pub(crate) fn created_at(name: &str) -> Option<SystemTime> {
    let uuid = Uuid::parse_str(name.strip_prefix(RESOURCE_PREFIX)?).ok()?;
    if uuid.get_version_num() != 7 {
        return None;
    }
    let (secs, nanos) = uuid.get_timestamp()?.to_unix();
    Some(UNIX_EPOCH + Duration::new(secs, nanos))
}

/// The pooled resources of one `(backend, service profile)` pair.
#[derive(Debug)]
pub(crate) struct Cell {
    pub backend: Arc<dyn Backend>,
    pub account: Arc<dyn BackendAccount>,
    pub database: String,
    pub admin: CosmosClient,
    shared: Mutex<HashMap<&'static str, Arc<OnceCell<String>>>>,
}

type CellSlot = Arc<OnceCell<Arc<Cell>>>;

type CellKey = (String, ServiceProfile);

fn cells() -> &'static Mutex<HashMap<CellKey, CellSlot>> {
    static CELLS: OnceLock<Mutex<HashMap<CellKey, CellSlot>>> = OnceLock::new();
    CELLS.get_or_init(Mutex::default)
}

/// Returns the cell for `profile`, opening it on first use.
pub(crate) async fn cell(
    backend: &Arc<dyn Backend>,
    profile: ServiceProfile,
) -> SpecResult<Arc<Cell>> {
    let slot = cells()
        .lock()
        .unwrap()
        .entry((backend.name().to_owned(), profile))
        .or_default()
        .clone();
    slot.get_or_try_init(|| detached(open_cell(backend.clone(), profile)))
        .await
        .cloned()
}

/// Runs `setup` on its own task, outside the current trial's span.
async fn detached<T: Send + 'static>(
    setup: impl Future<Output = SpecResult<T>> + Send + 'static,
) -> SpecResult<T> {
    match tokio::spawn(setup).await {
        Ok(result) => result,
        Err(e) if e.is_panic() => std::panic::resume_unwind(e.into_panic()),
        Err(e) => Err(e.into()),
    }
}

async fn open_cell(backend: Arc<dyn Backend>, profile: ServiceProfile) -> SpecResult<Arc<Cell>> {
    let account = backend.open_account(profile).await?;
    let admin = account.client(ClientOptions::default()).await?;
    if cleanup() {
        sweep(account.as_ref(), &admin).await?;
    }
    let database = resource_name();
    account
        .provisioner()
        .create_database(&admin, &database)
        .await?;
    Ok(Arc::new(Cell {
        backend,
        account,
        database,
        admin,
        shared: Mutex::default(),
    }))
}

/// Deletes the cell database of every opened cell, unless cleanup is off.
///
/// Returns the kept cell databases as `(backend, profile, database)`, sorted.
pub(crate) async fn close_all() -> SpecResult<Vec<(String, ServiceProfile, String)>> {
    let mut opened: Vec<(String, ServiceProfile, Arc<Cell>)> = cells()
        .lock()
        .unwrap()
        .iter()
        .filter_map(|((backend, profile), slot)| {
            slot.get()
                .map(|cell| (backend.clone(), *profile, cell.clone()))
        })
        .collect();
    if !cleanup() {
        opened.sort_by(|a, b| (&a.0, a.1).cmp(&(&b.0, b.1)));
        return Ok(opened
            .into_iter()
            .map(|(backend, profile, cell)| (backend, profile, cell.database.clone()))
            .collect());
    }
    attempt_all(opened.iter().map(|(_, _, cell)| async move {
        cell.account
            .provisioner()
            .delete_database(&cell.admin, &cell.database)
            .await
    }))
    .await?;
    Ok(Vec::new())
}

/// Runs every cleanup step in order, even after one fails.
///
/// Returns the first error; later errors are logged so one unavailable
/// resource doesn't leave the others behind.
pub(crate) async fn attempt_all<F>(steps: impl IntoIterator<Item = F>) -> SpecResult
where
    F: Future<Output = SpecResult>,
{
    let mut first = None;
    for step in steps {
        if let Err(error) = step.await {
            if first.is_some() {
                tracing::warn!(%error, "cleanup step failed");
            } else {
                first = Some(error);
            }
        }
    }
    first.map_or(Ok(()), Err)
}

/// Deletes framework databases left behind by runs that did not clean up.
async fn sweep(account: &dyn BackendAccount, client: &CosmosClient) -> SpecResult {
    let cutoff = SystemTime::now()
        .checked_sub(SWEEP_AGE)
        .unwrap_or(UNIX_EPOCH);
    for database in account.provisioner().list_databases(client).await? {
        if created_at(&database).is_some_and(|created| created < cutoff) {
            account
                .provisioner()
                .delete_database(client, &database)
                .await?;
        }
    }
    Ok(())
}

impl Cell {
    /// Returns the shared container for `profile`, creating and seeding it once.
    pub async fn shared_container(
        self: &Arc<Self>,
        profile: &ContainerProfile,
    ) -> SpecResult<String> {
        let slot = self
            .shared
            .lock()
            .unwrap()
            .entry(profile.name)
            .or_default()
            .clone();
        let (cell, profile) = (self.clone(), *profile);
        slot.get_or_try_init(|| {
            detached(async move {
                let name = resource_name();
                cell.account
                    .provisioner()
                    .create_container(&cell.admin, &cell.database, &name, &profile)
                    .await?;
                seed(&cell.admin, &cell.database, &name, &(profile.dataset)()).await?;
                Ok(name)
            })
        })
        .await
        .cloned()
    }
}

/// Opens a container client.
pub(crate) async fn container_client(
    client: &CosmosClient,
    database: &str,
    container: &str,
) -> SpecResult<ContainerClient> {
    Ok(Box::pin(
        client
            .database_client(database)
            .container_client(container, None),
    )
    .await?)
}

/// Writes every item of `dataset` into a container.
pub(crate) async fn seed(
    client: &CosmosClient,
    database: &str,
    container: &str,
    dataset: &Dataset,
) -> SpecResult {
    if dataset.is_empty() {
        return Ok(());
    }
    let container = container_client(client, database, container).await?;
    for item in dataset.all() {
        Box::pin(container.upsert_item(item.partition_key(), &item.id, &item, None)).await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{attempt_all, created_at, resource_name, Duration, SystemTime};
    use crate::SpecResult;
    use std::sync::Mutex;

    #[tokio::test]
    async fn attempt_all_runs_every_step_and_returns_the_first_error() {
        let ran = Mutex::new(Vec::new());
        let result = attempt_all((0..4).map(|i| {
            let ran = &ran;
            async move {
                ran.lock().unwrap().push(i);
                if i == 1 || i == 2 {
                    SpecResult::Err(format!("step {i} failed").into())
                } else {
                    Ok(())
                }
            }
        }))
        .await;
        assert_eq!(result.unwrap_err().to_string(), "step 1 failed");
        assert_eq!(*ran.lock().unwrap(), [0, 1, 2, 3]);
    }

    #[test]
    fn resource_names_carry_their_creation_time() {
        let before = SystemTime::now() - Duration::from_millis(1);
        let name = resource_name();
        let created = created_at(&name).expect("framework names parse");
        assert!(name.starts_with("spec-"));
        assert!(created >= before && created <= SystemTime::now());
    }

    #[test]
    fn foreign_names_have_no_creation_time() {
        assert_eq!(created_at("customers"), None);
        assert_eq!(created_at("spec-not-a-uuid"), None);
        assert_eq!(
            created_at("spec-67e55044-10b1-426f-9247-bb680e5fe0c8"),
            None,
            "UUIDv4 has no timestamp"
        );
    }
}
