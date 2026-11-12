// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Per-spec state: declared capabilities, the spec's `HttpTap`, and teardown.

use crate::{
    backend::{BackendAccount, ClientOptions},
    pool::Cell,
    profiles::{self, ContainerProfile},
    runner::SpecDescriptor,
    Capability, HttpTap, InvalidConfiguration, ServiceProfile, SpecResult,
};
use azure_data_cosmos::{fault_injection::FaultInjectionRule, CosmosClient};
use std::{
    future::Future,
    sync::{Arc, Mutex},
};

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

impl ParamKind {
    /// The type name used in the catalog.
    pub fn name(self) -> &'static str {
        match self {
            ParamKind::SharedContainer => "SharedContainer",
            ParamKind::ExclusiveContainer => "ExclusiveContainer",
            ParamKind::ExclusiveDatabase => "ExclusiveDatabase",
            ParamKind::ExclusiveAccount => "ExclusiveAccount",
            ParamKind::HttpTap => "HttpTap",
        }
    }
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
        database: String,
        container: String,
    },
    Database {
        account: Arc<dyn BackendAccount>,
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

    /// Builds a spec-facing client: tapped, with optional faults.
    pub(crate) async fn client(
        &self,
        account: &dyn BackendAccount,
        faults: Vec<Arc<FaultInjectionRule>>,
    ) -> SpecResult<CosmosClient> {
        account
            .client(ClientOptions {
                observer: Some(self.tap.observer()),
                faults,
            })
            .await
    }

    pub(crate) fn on_teardown_delete_container(
        &self,
        account: Arc<dyn BackendAccount>,
        database: &str,
        container: &str,
    ) {
        self.cleanups.lock().unwrap().push(Cleanup::Container {
            account,
            database: database.to_owned(),
            container: container.to_owned(),
        });
    }

    pub(crate) fn on_teardown_delete_database(
        &self,
        account: Arc<dyn BackendAccount>,
        database: &str,
    ) {
        self.cleanups.lock().unwrap().push(Cleanup::Database {
            account,
            database: database.to_owned(),
        });
    }

    pub(crate) fn guard_shared(&self, container: &str) {
        self.shared.lock().unwrap().push(container.to_owned());
    }

    /// Deletes exclusive resources and checks that no shared container was written.
    pub(crate) async fn finish(&self) -> SpecResult {
        let cleanups = std::mem::take(&mut *self.cleanups.lock().unwrap());
        for cleanup in cleanups.into_iter().rev() {
            match cleanup {
                Cleanup::Container {
                    account,
                    database,
                    container,
                } => {
                    account
                        .provisioner()
                        .delete_container(&database, &container)
                        .await?
                }
                Cleanup::Database { account, database } => {
                    account.provisioner().delete_database(&database).await?
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
