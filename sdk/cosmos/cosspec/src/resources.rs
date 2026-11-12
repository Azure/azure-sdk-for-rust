// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Injectable resources: what a spec receives through its parameters.

use crate::{
    backend::{BackendAccount, SimpleSplitMerge},
    context::{Inject, ParamKind, SpecContext, WRITE_OPERATIONS},
    faults::Fault,
    pool::{self, resource_name},
    profiles::ContainerProfile,
    results::{Origin, ResourceKind, ResourceRecord},
    testdata::Dataset,
    Capability, InvalidConfiguration, SpecResult,
};
use azure_core::http::StatusCode;
use azure_data_cosmos::{
    fault_injection::{
        CustomResponseBuilder, FaultInjectionConditionBuilder, FaultInjectionResultBuilder,
        FaultInjectionRule, FaultInjectionRuleBuilder, FaultOperationType,
    },
    options::Region,
    ContainerClient, CosmosClient, DatabaseClient, FeedScope, Query,
};
use futures::TryStreamExt;
use serde::de::DeserializeOwned;
use std::sync::Arc;

async fn drain_query<T: DeserializeOwned + Send + 'static>(
    client: &ContainerClient,
    query: &str,
) -> SpecResult<Vec<T>> {
    let iterator = Box::pin(client.query_items::<T>(
        Query::from(query.to_owned()),
        FeedScope::full_container(),
        None,
    ))
    .await?;
    Ok(Box::pin(iterator.try_collect()).await?)
}

/// A pooled, read-only container shaped by `#[profile(...)]`.
///
/// The container is created and seeded once per run and reused by every spec
/// that asks for the same profile. Writes are rejected with `403 Forbidden` and
/// fail the spec with [`InvalidConfiguration`].
pub struct SharedContainer {
    ctx: Arc<SpecContext>,
    profile: ContainerProfile,
    name: String,
    client: ContainerClient,
}

impl SharedContainer {
    /// A client for the container. Every request is recorded by the spec's `HttpTap`.
    pub fn client(&self) -> &ContainerClient {
        &self.client
    }

    /// The container's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The data the container holds.
    pub fn dataset(&self) -> Dataset {
        (self.profile.dataset)()
    }

    /// Builds a new tapped, write-guarded client for this container with
    /// client-side faults.
    pub async fn client_with_faults(
        &self,
        faults: impl IntoIterator<Item = Fault>,
    ) -> SpecResult<ContainerClient> {
        let mut rules = write_guard(&self.name);
        rules.extend(faults.into_iter().map(Fault::build));
        let client = self
            .ctx
            .client(self.ctx.cell.account.as_ref(), rules)
            .await?;
        pool::container_client(&client, &self.ctx.cell.database, &self.name).await
    }

    /// Runs `query` across the whole container and returns every result.
    pub async fn query_all<T: DeserializeOwned + Send + 'static>(
        &self,
        query: &str,
    ) -> SpecResult<Vec<T>> {
        drain_query(&self.client, query).await
    }
}

fn write_guard(container: &str) -> Vec<Arc<FaultInjectionRule>> {
    [
        FaultOperationType::CreateItem,
        FaultOperationType::UpsertItem,
        FaultOperationType::ReplaceItem,
        FaultOperationType::DeleteItem,
        FaultOperationType::PatchItem,
        FaultOperationType::BatchItem,
    ]
    .into_iter()
    .zip(WRITE_OPERATIONS)
    .map(|(operation, name)| {
        let response = CustomResponseBuilder::new(StatusCode::Forbidden)
            .with_body(format!(
                "spec framework: SharedContainer '{container}' is read-only"
            ))
            .build();
        let result = FaultInjectionResultBuilder::new()
            .with_custom_response(response)
            .build();
        let condition = FaultInjectionConditionBuilder::new()
            .with_operation_type(operation)
            .with_container_id(format!("/colls/{container}/"))
            .build();
        Arc::new(
            FaultInjectionRuleBuilder::new(format!("spec-shared-guard-{name}"), result)
                .with_condition(condition)
                .build(),
        )
    })
    .collect()
}

impl Inject for SharedContainer {
    const KIND: ParamKind = ParamKind::SharedContainer;

    async fn inject(ctx: &Arc<SpecContext>, index: usize) -> SpecResult<Self> {
        if ctx.descriptor.params[index].profile.is_none() {
            return Err(InvalidConfiguration(
                "SharedContainer parameters need #[profile(...)]".into(),
            )
            .into());
        }
        let profile = ctx.container_profile(index);
        let name = ctx.cell.shared_container(&profile).await?;
        ctx.record_cell_database();
        ctx.record_resource(ResourceRecord {
            kind: ResourceKind::Container,
            origin: Origin::Pooled,
            name: name.clone(),
            database: Some(ctx.cell.database.clone()),
            profile: Some(profile.name),
            parameter: Some(ctx.descriptor.params[index].name),
        });
        let client = ctx
            .client(ctx.cell.account.as_ref(), write_guard(&name))
            .await?;
        let client = pool::container_client(&client, &ctx.cell.database, &name).await?;
        ctx.guard_shared(&name);
        Ok(Self {
            ctx: ctx.clone(),
            profile,
            name,
            client,
        })
    }
}

/// A container created for one spec, seeded from `#[profile(...)]`, and deleted
/// when the spec ends.
///
/// It lives in the pooled cell database unless created through
/// [`ExclusiveDatabase::create_container`] or [`ExclusiveAccount::create_container`].
pub struct ExclusiveContainer {
    ctx: Arc<SpecContext>,
    account: Arc<dyn BackendAccount>,
    profile: ContainerProfile,
    database: String,
    name: String,
    client: ContainerClient,
}

impl ExclusiveContainer {
    async fn create(
        ctx: &Arc<SpecContext>,
        account: Arc<dyn BackendAccount>,
        harness: &CosmosClient,
        database: &str,
        profile: ContainerProfile,
        parameter: Option<&'static str>,
        delete_on_teardown: bool,
    ) -> SpecResult<Self> {
        let name = resource_name();
        account
            .provisioner()
            .create_container(harness, database, &name, &profile)
            .await?;
        ctx.record_resource(ResourceRecord {
            kind: ResourceKind::Container,
            origin: Origin::Created,
            name: name.clone(),
            database: Some(database.to_owned()),
            profile: Some(profile.name),
            parameter,
        });
        if delete_on_teardown {
            ctx.on_teardown_delete_container(account.clone(), harness.clone(), database, &name);
        }
        pool::seed(harness, database, &name, &(profile.dataset)()).await?;
        let client = ctx.client(account.as_ref(), Vec::new()).await?;
        let client = pool::container_client(&client, database, &name).await?;
        Ok(Self {
            ctx: ctx.clone(),
            account,
            profile,
            database: database.to_owned(),
            name,
            client,
        })
    }

    /// A client for the container. Every request is recorded by the spec's `HttpTap`.
    pub fn client(&self) -> &ContainerClient {
        &self.client
    }

    /// The container's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The data the container was seeded with.
    pub fn dataset(&self) -> Dataset {
        (self.profile.dataset)()
    }

    /// Builds a new tapped client for this container with client-side faults.
    pub async fn client_with_faults(
        &self,
        faults: impl IntoIterator<Item = Fault>,
    ) -> SpecResult<ContainerClient> {
        let faults = faults.into_iter().map(Fault::build).collect();
        let client = self.ctx.client(self.account.as_ref(), faults).await?;
        pool::container_client(&client, &self.database, &self.name).await
    }

    /// Runs `query` across the whole container and returns every result.
    pub async fn query_all<T: DeserializeOwned + Send + 'static>(
        &self,
        query: &str,
    ) -> SpecResult<Vec<T>> {
        drain_query(&self.client, query).await
    }

    /// Splits every physical partition in two and waits until the split is done.
    ///
    /// Requires `#[requires(TopologyControl)]`.
    pub async fn split(&self) -> SpecResult {
        self.ctx.demand(Capability::TopologyControl)?;
        let topology = self
            .account
            .topology()
            .ok_or_else(|| InvalidConfiguration("the backend has no topology control".into()))?;
        topology.split_all(&self.database, &self.name).await
    }

    /// Splits the container's one physical partition into two and waits until
    /// the split is done.
    ///
    /// Requires `#[requires(SimpleSplitMerge)]` and a container with one physical
    /// partition.
    pub async fn simple_partition_split(&self) -> SpecResult {
        self.ctx.demand(Capability::SimpleSplitMerge)?;
        self.simple_split_merge()?
            .simple_partition_split(&self.database, &self.name)
            .await
    }

    /// Merges the container's two physical partitions into one and waits until
    /// the merge is done.
    ///
    /// Requires `#[requires(SimpleSplitMerge)]` and a container with two physical
    /// partitions.
    pub async fn simple_partition_merge(&self) -> SpecResult {
        self.ctx.demand(Capability::SimpleSplitMerge)?;
        self.simple_split_merge()?
            .simple_partition_merge(&self.database, &self.name)
            .await
    }

    fn simple_split_merge(&self) -> SpecResult<Box<dyn SimpleSplitMerge + '_>> {
        self.account.simple_split_merge().ok_or_else(|| {
            InvalidConfiguration("the backend has no simple split or merge".into()).into()
        })
    }

    /// The number of feed ranges the service reports, read with a fresh client so
    /// no cached routing state is involved.
    pub async fn feed_range_count(&self) -> SpecResult<usize> {
        let client = self.ctx.harness_client(self.account.as_ref()).await?;
        let client = pool::container_client(&client, &self.database, &self.name).await?;
        Ok(Box::pin(client.read_feed_ranges(None)).await?.len())
    }
}

impl Inject for ExclusiveContainer {
    const KIND: ParamKind = ParamKind::ExclusiveContainer;

    async fn inject(ctx: &Arc<SpecContext>, index: usize) -> SpecResult<Self> {
        ctx.record_cell_database();
        let harness = ctx.cell_harness_client().await?;
        Self::create(
            ctx,
            ctx.cell.account.clone(),
            &harness,
            &ctx.cell.database,
            ctx.container_profile(index),
            Some(ctx.descriptor.params[index].name),
            true,
        )
        .await
    }
}

/// A database created for one spec and deleted, with everything in it, when the
/// spec ends.
pub struct ExclusiveDatabase {
    ctx: Arc<SpecContext>,
    name: String,
    client: DatabaseClient,
}

impl ExclusiveDatabase {
    /// A client for the database. Every request is recorded by the spec's `HttpTap`.
    pub fn client(&self) -> &DatabaseClient {
        &self.client
    }

    /// The database's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Creates a container in this database shaped and seeded by `profile`.
    pub async fn create_container(
        &self,
        profile: ContainerProfile,
    ) -> SpecResult<ExclusiveContainer> {
        let harness = self.ctx.cell_harness_client().await?;
        ExclusiveContainer::create(
            &self.ctx,
            self.ctx.cell.account.clone(),
            &harness,
            &self.name,
            profile,
            None,
            false,
        )
        .await
    }
}

impl Inject for ExclusiveDatabase {
    const KIND: ParamKind = ParamKind::ExclusiveDatabase;

    async fn inject(ctx: &Arc<SpecContext>, index: usize) -> SpecResult<Self> {
        let account = ctx.cell.account.clone();
        ctx.record_account(account.as_ref(), Origin::Pooled);
        let harness = ctx.cell_harness_client().await?;
        let name = resource_name();
        account
            .provisioner()
            .create_database(&harness, &name)
            .await?;
        ctx.record_resource(ResourceRecord {
            kind: ResourceKind::Database,
            origin: Origin::Created,
            name: name.clone(),
            database: None,
            profile: None,
            parameter: Some(ctx.descriptor.params[index].name),
        });
        ctx.on_teardown_delete_database(account.clone(), harness, &name);
        let client = ctx.client(account.as_ref(), Vec::new()).await?;
        Ok(Self {
            ctx: ctx.clone(),
            client: client.database_client(&name),
            name,
        })
    }
}

/// An account opened for one spec, so it can change account-wide state such as
/// which regions are online.
///
/// Requires `#[requires(AccountIsolation)]`.
#[derive(Debug)]
pub struct ExclusiveAccount {
    ctx: Arc<SpecContext>,
    account: Arc<dyn BackendAccount>,
    admin: CosmosClient,
    database: String,
    client: CosmosClient,
}

impl ExclusiveAccount {
    /// A client for the account. Every request is recorded by the spec's `HttpTap`.
    pub fn client(&self) -> &CosmosClient {
        &self.client
    }

    /// The account's regions, in preference order.
    pub fn regions(&self) -> Vec<Region> {
        self.account.profile().regions()
    }

    /// Creates a container shaped and seeded by `profile`.
    pub async fn create_container(
        &self,
        profile: ContainerProfile,
    ) -> SpecResult<ExclusiveContainer> {
        ExclusiveContainer::create(
            &self.ctx,
            self.account.clone(),
            &self.admin,
            &self.database,
            profile,
            None,
            false,
        )
        .await
    }

    /// Takes `region` offline. Requires `#[requires(RegionControl)]`.
    pub async fn take_region_offline(&self, region: &Region) -> SpecResult {
        self.region_control()?.set_offline(region).await
    }

    /// Brings `region` back online. Requires `#[requires(RegionControl)]`.
    pub async fn bring_region_online(&self, region: &Region) -> SpecResult {
        self.region_control()?.set_online(region).await
    }

    fn region_control(&self) -> SpecResult<&dyn crate::backend::RegionControl> {
        self.ctx.demand(Capability::RegionControl)?;
        self.account
            .regions()
            .ok_or_else(|| InvalidConfiguration("the backend has no region control".into()).into())
    }
}

impl Inject for ExclusiveAccount {
    const KIND: ParamKind = ParamKind::ExclusiveAccount;

    async fn inject(ctx: &Arc<SpecContext>, index: usize) -> SpecResult<Self> {
        ctx.demand(Capability::AccountIsolation)?;
        let account = ctx.cell.backend.open_account(ctx.service_profile()).await?;
        ctx.record_account(account.as_ref(), Origin::Created);
        let admin = ctx.harness_client(account.as_ref()).await?;
        let database = resource_name();
        account
            .provisioner()
            .create_database(&admin, &database)
            .await?;
        ctx.record_resource(ResourceRecord {
            kind: ResourceKind::Database,
            origin: Origin::Created,
            name: database.clone(),
            database: None,
            profile: None,
            parameter: Some(ctx.descriptor.params[index].name),
        });
        ctx.on_teardown_delete_database(account.clone(), admin.clone(), &database);
        let client = ctx.client(account.as_ref(), Vec::new()).await?;
        Ok(Self {
            ctx: ctx.clone(),
            account,
            admin,
            database,
            client,
        })
    }
}
