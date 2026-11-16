// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Per-spec state: declared capabilities, the spec's `HttpTap`, the full HTTP
//! recording, the resources used, and teardown.

use crate::{
    backend::{BackendAccount, ClientOptions},
    http_tap::{self, HttpRecord},
    pool::Cell,
    profiles::{self, ContainerProfile},
    results::{Origin, ResourceKind, ResourceRecord},
    runner::SpecDescriptor,
    Capability, HttpTap, InvalidConfiguration, RegionAlias, RegionMap, ServiceProfile, SpecResult,
};
use azure_data_cosmos::{fault_injection::FaultInjectionRule, CosmosClient};
use azure_data_cosmos_driver::models::{OperationType, ResourceType};
use std::{
    future::Future,
    sync::{Arc, Mutex},
};
use tokio::sync::OnceCell;

/// The recording label for requests from clients the spec uses.
pub(crate) const SPEC_LABEL: &str = "spec";

/// The recording label for requests the framework makes for the spec, such as
/// seeding and teardown.
pub(crate) const HARNESS_LABEL: &str = "harness";

/// The write operations a `SharedContainer` must never see.
pub(crate) const WRITE_OPERATIONS: &[&str] = &[
    "CreateItem",
    "UpsertItem",
    "ReplaceItem",
    "DeleteItem",
    "PatchItem",
    "BatchItem",
];

/// The kind of resource an injectable parameter provides.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParamKind {
    /// A pooled, read-only container.
    SharedContainer,
    /// A container created for this spec and deleted afterwards.
    ExclusiveContainer,
    /// A database created for this spec and deleted afterwards.
    ExclusiveDatabase,
    /// An account opened for this spec.
    ExclusiveAccount,
    /// The spec's HTTP tap.
    HttpTap,
}

/// A type the framework can create for a spec parameter.
pub trait Inject: Sized {
    /// The kind of resource.
    const KIND: ParamKind;

    /// Creates the resource for parameter `index` of the running spec.
    fn inject(ctx: &Arc<SpecContext>, index: usize) -> impl Future<Output = SpecResult<Self>>;
}

#[derive(Debug)]
enum Cleanup {
    Container {
        account: Arc<dyn BackendAccount>,
        client: CosmosClient,
        database: String,
        container: String,
    },
    Database {
        account: Arc<dyn BackendAccount>,
        client: CosmosClient,
        database: String,
    },
}

/// The running spec's state.
#[derive(Debug)]
pub struct SpecContext {
    pub(crate) cell: Arc<Cell>,
    pub(crate) descriptor: &'static SpecDescriptor,
    pub(crate) service_profile: ServiceProfile,
    tap: HttpTap,
    recording: HttpTap,
    harness: OnceCell<CosmosClient>,
    resources: Mutex<Vec<ResourceRecord>>,
    cleanups: Mutex<Vec<Cleanup>>,
    shared: Mutex<Vec<String>>,
}

impl SpecContext {
    pub(crate) fn new(
        cell: Arc<Cell>,
        descriptor: &'static SpecDescriptor,
        service_profile: ServiceProfile,
    ) -> Self {
        let aliases = || {
            cell.account
                .regions()
                .iter()
                .map(|(alias, region)| (region.as_str().to_owned(), alias.name().to_owned()))
                .collect::<Vec<_>>()
        };
        Self {
            tap: HttpTap::with_region_aliases(aliases()),
            recording: HttpTap::with_region_aliases(aliases()),
            cell,
            descriptor,
            service_profile,
            harness: OnceCell::new(),
            resources: Mutex::default(),
            cleanups: Mutex::default(),
            shared: Mutex::default(),
        }
    }

    /// The service profile this spec is running under.
    pub fn service_profile(&self) -> ServiceProfile {
        self.service_profile
    }

    /// The account's region aliases, in preference order.
    pub fn regions(&self) -> &'static [RegionAlias] {
        self.service_profile.regions()
    }

    /// The real region behind each region alias. Every account of a backend
    /// and service profile has the same map.
    pub(crate) fn region_map(&self) -> &RegionMap {
        self.cell.account.regions()
    }

    /// Fails unless the spec declared `capability` with `#[requires(...)]`.
    pub fn demand(&self, capability: Capability) -> SpecResult {
        if self.descriptor.requires.contains(&capability) {
            Ok(())
        } else {
            Err(InvalidConfiguration(format!(
                "this spec uses {capability} but does not declare #[requires({capability})]"
            ))
            .into())
        }
    }

    /// The tap attached to every client the spec receives.
    pub fn tap(&self) -> &HttpTap {
        &self.tap
    }

    /// The container profile of parameter `index`, or [`profiles::empty`].
    pub(crate) fn container_profile(&self, index: usize) -> ContainerProfile {
        self.descriptor.params[index]
            .profile
            .map(|profile| profile())
            .unwrap_or_else(profiles::empty)
    }

    /// Builds a spec-facing client: tapped, recorded, with optional faults.
    pub(crate) async fn client(
        &self,
        account: &dyn BackendAccount,
        faults: Vec<Arc<FaultInjectionRule>>,
    ) -> SpecResult<CosmosClient> {
        let observer = http_tap::fan_out(vec![
            self.tap.observer(),
            self.recording.labeled_observer(SPEC_LABEL),
        ]);
        account
            .client(ClientOptions {
                observer: Some(observer),
                faults,
            })
            .await
    }

    /// Builds a new framework client for `account`. Its requests are recorded
    /// but hidden from the spec's `HttpTap`.
    pub(crate) async fn harness_client(
        &self,
        account: &dyn BackendAccount,
    ) -> SpecResult<CosmosClient> {
        account
            .client(ClientOptions {
                observer: Some(self.recording.labeled_observer(HARNESS_LABEL)),
                faults: Vec::new(),
            })
            .await
    }

    /// The framework client for the pooled account, built on first use.
    pub(crate) async fn cell_harness_client(&self) -> SpecResult<CosmosClient> {
        self.harness
            .get_or_try_init(|| self.harness_client(self.cell.account.as_ref()))
            .await
            .cloned()
    }

    /// Records a resource the spec used, once.
    pub(crate) fn record_resource(&self, record: ResourceRecord) {
        let mut resources = self.resources.lock().unwrap();
        if !resources.contains(&record) {
            resources.push(record);
        }
    }

    /// Records an account the spec used.
    pub(crate) fn record_account(&self, account: &dyn BackendAccount, origin: Origin) {
        self.record_resource(ResourceRecord {
            kind: ResourceKind::Account,
            origin,
            name: account.endpoint(),
            database: None,
            profile: None,
            parameter: None,
        });
    }

    /// Records that the spec uses the pooled account and cell database.
    pub(crate) fn record_cell_database(&self) {
        self.record_account(self.cell.account.as_ref(), Origin::Pooled);
        self.record_resource(ResourceRecord {
            kind: ResourceKind::Database,
            origin: Origin::Pooled,
            name: self.cell.database.clone(),
            database: None,
            profile: None,
            parameter: None,
        });
    }

    /// The resources the spec created or borrowed, in the order it first used them.
    pub(crate) fn resources(&self) -> Vec<ResourceRecord> {
        self.resources.lock().unwrap().clone()
    }

    /// Every HTTP exchange of the spec and of the framework on its behalf, in
    /// start order. Unlike the spec's `HttpTap`, it is never cleared.
    pub(crate) fn recording(&self) -> Vec<HttpRecord> {
        self.recording.records()
    }

    pub(crate) fn on_teardown_delete_container(
        &self,
        account: Arc<dyn BackendAccount>,
        client: CosmosClient,
        database: &str,
        container: &str,
    ) {
        self.cleanups.lock().unwrap().push(Cleanup::Container {
            account,
            client,
            database: database.to_owned(),
            container: container.to_owned(),
        });
    }

    pub(crate) fn on_teardown_delete_database(
        &self,
        account: Arc<dyn BackendAccount>,
        client: CosmosClient,
        database: &str,
    ) {
        self.cleanups.lock().unwrap().push(Cleanup::Database {
            account,
            client,
            database: database.to_owned(),
        });
    }

    pub(crate) fn guard_shared(&self, container: &str) {
        self.shared.lock().unwrap().push(container.to_owned());
    }

    /// Deletes exclusive resources (unless `--no-cleanup`) and checks that no
    /// shared container, or its items, was modified.
    ///
    /// Every deletion is attempted even if an earlier one fails. A shared
    /// container violation is reported ahead of a deletion error.
    pub(crate) async fn finish(&self) -> SpecResult {
        let mut cleanups = std::mem::take(&mut *self.cleanups.lock().unwrap());
        if !crate::pool::cleanup() {
            cleanups.clear();
        }
        let deleted =
            crate::pool::attempt_all(cleanups.into_iter().rev().map(|cleanup| async move {
                match cleanup {
                    Cleanup::Container {
                        account,
                        client,
                        database,
                        container,
                    } => {
                        account
                            .provisioner()
                            .delete_container(&client, &database, &container)
                            .await
                    }
                    Cleanup::Database {
                        account,
                        client,
                        database,
                    } => {
                        account
                            .provisioner()
                            .delete_database(&client, &database)
                            .await
                    }
                }
            }))
            .await;

        let shared = self.shared.lock().unwrap().clone();
        for record in self.tap.records() {
            if let Some(name) = shared.iter().find(|name| modifies_shared(&record, name)) {
                return Err(InvalidConfiguration(format!(
                    "the spec sent {} to SharedContainer '{name}'; shared containers are read-only, use ExclusiveContainer",
                    record.operation.as_deref().unwrap_or("a write"),
                ))
                .into());
            }
        }
        deleted
    }
}

/// Whether `record` writes an item in, or deletes or replaces, the shared
/// container `name`.
///
/// Item writes are also rejected before dispatch by the shared write guard.
/// Container deletes and replaces can't be, so they're only caught here.
fn modifies_shared(record: &HttpRecord, name: &str) -> bool {
    let path = record.url.split(['?', '#']).next().unwrap_or_default();
    if WRITE_OPERATIONS.iter().any(|op| record.is(op)) {
        return path.contains(&format!("/colls/{name}/"));
    }
    record.resource_type == Some(ResourceType::DocumentCollection)
        && matches!(
            record.operation_type,
            Some(OperationType::Delete | OperationType::Replace)
        )
        && path
            .trim_end_matches('/')
            .ends_with(&format!("/colls/{name}"))
}

impl Inject for HttpTap {
    const KIND: ParamKind = ParamKind::HttpTap;

    async fn inject(ctx: &Arc<SpecContext>, _index: usize) -> SpecResult<Self> {
        Ok(ctx.tap.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::{modifies_shared, HttpRecord, OperationType, ResourceType};
    use std::time::Instant;

    fn record(
        url: &str,
        operation: Option<&str>,
        resource_type: ResourceType,
        operation_type: OperationType,
    ) -> HttpRecord {
        HttpRecord {
            sequence: 0,
            method: "DELETE".into(),
            url: url.into(),
            operation: operation.map(str::to_owned),
            operation_type: Some(operation_type),
            resource_type: Some(resource_type),
            transport_mode: None,
            execution_context: None,
            effective_partition_key: None,
            consistency_level: None,
            read_consistency_strategy: None,
            headers: None,
            region: None,
            region_alias: None,
            activity_id: None,
            label: None,
            started_at: Instant::now(),
            completed_at: None,
            outcome: None,
            request_body: None,
            response_body: None,
        }
    }

    const BASE: &str = "https://localhost:8081/dbs/db";

    #[test]
    fn container_delete_and_replace_modify_the_shared_container() {
        for operation_type in [OperationType::Delete, OperationType::Replace] {
            for url in [
                format!("{BASE}/colls/shared"),
                format!("{BASE}/colls/shared/"),
                format!("{BASE}/colls/shared?x=1"),
            ] {
                let record = record(&url, None, ResourceType::DocumentCollection, operation_type);
                assert!(modifies_shared(&record, "shared"), "{url}");
            }
        }
    }

    #[test]
    fn item_writes_modify_the_shared_container() {
        let record = record(
            &format!("{BASE}/colls/shared/docs/a"),
            Some("DeleteItem"),
            ResourceType::Document,
            OperationType::Delete,
        );
        assert!(modifies_shared(&record, "shared"));
    }

    #[test]
    fn other_requests_do_not_modify_the_shared_container() {
        let cases = [
            record(
                &format!("{BASE}/colls/shared"),
                None,
                ResourceType::DocumentCollection,
                OperationType::Read,
            ),
            record(
                &format!("{BASE}/colls/shared-2"),
                None,
                ResourceType::DocumentCollection,
                OperationType::Delete,
            ),
            record(
                &format!("{BASE}/colls/other/docs/a"),
                Some("DeleteItem"),
                ResourceType::Document,
                OperationType::Delete,
            ),
            record(
                &format!("{BASE}/colls/shared/docs/a"),
                Some("ReadItem"),
                ResourceType::Document,
                OperationType::Read,
            ),
        ];
        for record in cases {
            assert!(!modifies_shared(&record, "shared"), "{}", record.url);
        }
    }
}
