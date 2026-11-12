// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Pooled resources: one account and one cell database per service profile per
//! run, plus the shared containers inside it.
//!
//! * The account is opened once per `(backend, service profile)` cell.
//! * The cell database holds every `SharedContainer` and every
//!   `ExclusiveContainer` of the cell. It is deleted when the process exits
//!   normally and swept on a later run otherwise.
//! * Shared containers are created and seeded once per profile, then reused by
//!   every spec that asks for them.
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
    sync::{Arc, Mutex, OnceLock},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::sync::OnceCell;
use uuid::Uuid;

/// Prefix of every resource the framework creates.
pub(crate) const RESOURCE_PREFIX: &str = "spec-";

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
    slot.get_or_try_init(|| open_cell(backend.clone(), profile))
        .await
        .cloned()
}

async fn open_cell(backend: Arc<dyn Backend>, profile: ServiceProfile) -> SpecResult<Arc<Cell>> {
    let account = backend.open_account(profile).await?;
    let admin = account.client(ClientOptions::default()).await?;
    sweep(account.as_ref(), &admin).await?;
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
            .delete_database(&cell.admin, &cell.database)
            .await?;
    }
    Ok(())
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
    pub async fn shared_container(&self, profile: &ContainerProfile) -> SpecResult<String> {
        let slot = self
            .shared
            .lock()
            .unwrap()
            .entry(profile.name)
            .or_default()
            .clone();
        slot.get_or_try_init(|| async {
            let name = resource_name();
            self.account
                .provisioner()
                .create_container(&self.admin, &self.database, &name, profile)
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

#[cfg(test)]
mod tests {
    use super::{created_at, resource_name, Duration, SystemTime};

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
