// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Pooled resources: one account and one cell database per service profile per
//! run, plus the shared containers inside it.
//!
//! * The account is opened once per `(backend, service profile)` cell.
//! * The cell database `spec-<run>` holds every `SharedContainer` and every
//!   `ExclusiveContainer` of the cell. It is deleted when the process exits
//!   normally and swept on a later run otherwise.
//! * Shared containers are created and seeded once per profile, then reused by
//!   every spec that asks for them.

use crate::{
    backend::{Backend, BackendAccount, ClientOptions},
    profiles::ContainerProfile,
    testdata::Dataset,
    ServiceProfile, SpecResult,
};
use azure_data_cosmos::{ContainerClient, CosmosClient};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::sync::OnceCell;

/// Prefix of every database the framework creates.
pub(crate) const DATABASE_PREFIX: &str = "spec-";

/// Leftover databases older than this are deleted when a cell opens.
const SWEEP_AGE: Duration = Duration::from_secs(2 * 60 * 60);

/// A unique, time-stamped name for this run: `<unix-seconds>-<random>`.
pub(crate) fn run_id() -> &'static str {
    static RUN_ID: OnceLock<String> = OnceLock::new();
    RUN_ID.get_or_init(|| {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        format!("{now}-{}", short_id())
    })
}

/// A short random suffix for resource names.
pub(crate) fn short_id() -> String {
    uuid::Uuid::new_v4().simple().to_string()[..8].to_owned()
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

fn cells() -> &'static Mutex<HashMap<ServiceProfile, CellSlot>> {
    static CELLS: OnceLock<Mutex<HashMap<ServiceProfile, CellSlot>>> = OnceLock::new();
    CELLS.get_or_init(Mutex::default)
}

/// Returns the cell for `profile`, opening it on first use.
pub(crate) async fn cell(
    backend: &Arc<dyn Backend>,
    profile: ServiceProfile,
) -> SpecResult<Arc<Cell>> {
    let slot = cells().lock().unwrap().entry(profile).or_default().clone();
    slot.get_or_try_init(|| open_cell(backend.clone(), profile))
        .await
        .cloned()
}

async fn open_cell(backend: Arc<dyn Backend>, profile: ServiceProfile) -> SpecResult<Arc<Cell>> {
    let account = backend.open_account(profile).await?;
    sweep(account.as_ref()).await?;
    let database = format!("{DATABASE_PREFIX}{}", run_id());
    account.provisioner().create_database(&database).await?;
    let admin = account.client(ClientOptions::default()).await?;
    Ok(Arc::new(Cell {
        backend,
        account,
        database,
        admin,
        shared: Mutex::default(),
    }))
}

/// Deletes the cell database of every opened cell.
pub(crate) async fn close_all() -> SpecResult {
    let opened: Vec<Arc<Cell>> = cells()
        .lock()
        .unwrap()
        .values()
        .filter_map(|slot| slot.get().cloned())
        .collect();
    for cell in opened {
        cell.account
            .provisioner()
            .delete_database(&cell.database)
            .await?;
    }
    Ok(())
}

/// Deletes framework databases left behind by runs that did not clean up.
async fn sweep(account: &dyn BackendAccount) -> SpecResult {
    let cutoff = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .saturating_sub(SWEEP_AGE)
        .as_secs();
    for database in account.provisioner().list_databases().await? {
        let created = database
            .strip_prefix(DATABASE_PREFIX)
            .and_then(|rest| rest.split('-').next())
            .and_then(|secs| secs.parse::<u64>().ok());
        if matches!(created, Some(secs) if secs < cutoff) {
            account.provisioner().delete_database(&database).await?;
        }
    }
    Ok(())
}

impl Cell {
    /// Returns the shared container for `profile`, creating and seeding it once.
    pub async fn shared_container(&self, profile: &ContainerProfile) -> SpecResult<String> {
        let slot = self
            .shared
            .lock()
            .unwrap()
            .entry(profile.name)
            .or_default()
            .clone();
        slot.get_or_try_init(|| async {
            let name = format!("shared-{}", profile.name.replace('_', "-"));
            self.account
                .provisioner()
                .create_container(&self.database, &name, profile)
                .await?;
            seed(&self.admin, &self.database, &name, &(profile.dataset)()).await?;
            Ok::<_, crate::SpecError>(name)
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
