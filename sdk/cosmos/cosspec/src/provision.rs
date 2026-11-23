// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Creating and deleting databases and containers for specs.
//!
//! [`ControlPlaneProvisioner`] uses the SDK's `control_plane` APIs and therefore
//! needs key-based auth. Entra ID-only accounts need a provisioner backed by Azure
//! Resource Manager; that is a planned addition behind the same trait.
//!
//! Every method takes the harness client to provision through, so the requests
//! are recorded with the spec that caused them. Provisioners that don't use the
//! data plane may ignore it.

use crate::{profiles::ContainerProfile, SpecResult};
use async_trait::async_trait;
use azure_data_cosmos::{
    models::{ContainerProperties, DatabaseProperties, ThroughputProperties},
    options::CreateContainerOptions,
    CosmosClient, Query,
};
use futures::TryStreamExt;
use std::fmt::Debug;

/// The RU/s per physical partition a provisioner requests to get more than one
/// physical partition from the service.
pub const RU_PER_PHYSICAL_PARTITION: u64 = 10_000;

/// Creates and deletes the resources specs run against.
#[async_trait]
pub trait Provisioner: Send + Sync + Debug {
    /// Creates a database. Succeeds if it already exists.
    async fn create_database(&self, client: &CosmosClient, database: &str) -> SpecResult;

    /// Deletes a database and everything in it. Succeeds if it does not exist.
    async fn delete_database(&self, client: &CosmosClient, database: &str) -> SpecResult;

    /// Creates a container shaped by `profile`. Does not seed data.
    async fn create_container(
        &self,
        client: &CosmosClient,
        database: &str,
        container: &str,
        profile: &ContainerProfile,
    ) -> SpecResult;

    /// Deletes a container. Succeeds if it does not exist.
    async fn delete_container(
        &self,
        client: &CosmosClient,
        database: &str,
        container: &str,
    ) -> SpecResult;

    /// Lists the IDs of every database in the account.
    async fn list_databases(&self, client: &CosmosClient) -> SpecResult<Vec<String>>;
}

/// A [`Provisioner`] that uses the SDK's `control_plane` APIs.
#[derive(Debug, Default)]
pub struct ControlPlaneProvisioner;

fn ignore_status(result: azure_data_cosmos::Result<()>, status: u16) -> SpecResult {
    match result {
        Err(e) if u16::from(e.status().status_code()) == status => Ok(()),
        other => Ok(other?),
    }
}

#[async_trait]
impl Provisioner for ControlPlaneProvisioner {
    async fn create_database(&self, client: &CosmosClient, database: &str) -> SpecResult {
        let result = Box::pin(client.create_database(database, None))
            .await
            .map(drop);
        ignore_status(result, 409)
    }

    async fn delete_database(&self, client: &CosmosClient, database: &str) -> SpecResult {
        let result = Box::pin(client.database_client(database).delete(None))
            .await
            .map(drop);
        ignore_status(result, 404)
    }

    async fn create_container(
        &self,
        client: &CosmosClient,
        database: &str,
        container: &str,
        profile: &ContainerProfile,
    ) -> SpecResult {
        let properties = ContainerProperties::new(
            container.to_owned(),
            profile.partition_key_path.to_owned().into(),
        );
        let options = (profile.physical_partitions > 1).then(|| {
            CreateContainerOptions::default().with_throughput(ThroughputProperties::manual(
                u64::from(profile.physical_partitions) * RU_PER_PHYSICAL_PARTITION,
            ))
        });
        let result = Box::pin(
            client
                .database_client(database)
                .create_container(properties, options),
        )
        .await
        .map(drop);
        ignore_status(result, 409)
    }

    async fn delete_container(
        &self,
        client: &CosmosClient,
        database: &str,
        container: &str,
    ) -> SpecResult {
        let client = Box::pin(
            client
                .database_client(database)
                .container_client(container, None),
        )
        .await;
        let result = match client {
            Ok(client) => Box::pin(client.delete(None)).await.map(drop),
            Err(e) => Err(e),
        };
        ignore_status(result, 404)
    }

    async fn list_databases(&self, client: &CosmosClient) -> SpecResult<Vec<String>> {
        let iterator =
            Box::pin(client.query_databases(Query::from("SELECT * FROM root"), None)).await?;
        let databases: Vec<DatabaseProperties> = Box::pin(iterator.try_collect()).await?;
        Ok(databases.into_iter().filter_map(|db| db.id).collect())
    }
}
