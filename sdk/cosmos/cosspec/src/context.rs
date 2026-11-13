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
    Capability, HttpTap, InvalidConfiguration, ServiceProfile, SpecResult,
};
use azure_data_cosmos::{fault_injection::FaultInjectionRule, CosmosClient};
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
        Self {
            cell,
            descriptor,
            service_profile,
            tap: HttpTap::new(),
            recording: HttpTap::new(),
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
    /// shared container was written.
    pub(crate) async fn finish(&self) -> SpecResult {
        let mut cleanups = std::mem::take(&mut *self.cleanups.lock().unwrap());
        if !crate::pool::cleanup() {
            cleanups.clear();
        }
        for cleanup in cleanups.into_iter().rev() {
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
                        .await?
                }
                Cleanup::Database {
                    account,
                    client,
                    database,
                } => {
                    account
                        .provisioner()
                        .delete_database(&client, &database)
                        .await?
                }
            }
        }

        let shared = self.shared.lock().unwrap().clone();
        for record in self.tap.records() {
            let is_write = WRITE_OPERATIONS.iter().any(|op| record.is(op));
            if let Some(name) = shared
                .iter()
                .find(|name| is_write && record.url.contains(&format!("/colls/{name}/")))
            {
                return Err(InvalidConfiguration(format!(
                    "the spec sent {} to SharedContainer '{name}'; shared containers are read-only, use ExclusiveContainer",
                    record.operation.as_deref().unwrap_or("a write"),
                ))
                .into());
            }
        }
        Ok(())
    }
}

impl Inject for HttpTap {
    const KIND: ParamKind = ParamKind::HttpTap;

    async fn inject(ctx: &Arc<SpecContext>, _index: usize) -> SpecResult<Self> {
        Ok(ctx.tap.clone())
    }
}
