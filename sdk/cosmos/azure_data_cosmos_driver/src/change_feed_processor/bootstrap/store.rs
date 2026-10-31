// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    change_feed_processor::{bootstrap::error, operation_options},
    models::{
        ContainerReference, CosmosOperation, CosmosResponse, ItemReference, PartitionKey,
        Precondition,
    },
    CosmosDriver, Result,
};
use azure_core::http::{Etag, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Instant;

pub(in crate::change_feed_processor) struct ChangeFeedProcessorBootstrapStoredRecord<T> {
    pub value: T,
    pub etag: Etag,
}

pub(in crate::change_feed_processor) struct ChangeFeedProcessorBootstrapStore<'a> {
    pub driver: &'a CosmosDriver,
    pub container: &'a ContainerReference,
    pub deadline: Instant,
}

impl ChangeFeedProcessorBootstrapStore<'_> {
    fn item(&self, id: &str) -> Result<ItemReference> {
        if id.len() > 1023 {
            return Err(error(
                StatusCode::BadRequest,
                "Bootstrap document identifier exceeds 1023 bytes.",
            ));
        }
        Ok(ItemReference::from_name(
            self.container,
            PartitionKey::from(id.to_owned()),
            id.to_owned(),
        ))
    }

    pub(in crate::change_feed_processor) async fn execute(
        &self,
        operation: CosmosOperation,
    ) -> Result<CosmosResponse> {
        self.driver
            .execute_singleton_operation(
                operation.with_absolute_deadline(Some(self.deadline)),
                operation_options(),
            )
            .await
    }

    pub(in crate::change_feed_processor) async fn read<T: DeserializeOwned>(
        &self,
        id: &str,
    ) -> Result<Option<ChangeFeedProcessorBootstrapStoredRecord<T>>> {
        match self
            .execute(CosmosOperation::read_item(self.item(id)?))
            .await
        {
            Ok(response) => {
                let etag = response.headers().etag.clone().ok_or_else(|| {
                    error(
                        StatusCode::InternalServerError,
                        "Bootstrap item response has no ETag.",
                    )
                })?;
                Ok(Some(ChangeFeedProcessorBootstrapStoredRecord {
                    etag,
                    value: response.into_body().into_single()?,
                }))
            }
            Err(err) if err.status().is_not_found() => Ok(None),
            Err(err) => Err(err),
        }
    }

    pub(in crate::change_feed_processor) async fn create<T: Serialize>(
        &self,
        id: &str,
        value: &T,
    ) -> Result<bool> {
        let operation =
            CosmosOperation::create_item(self.item(id)?).with_body(serde_json::to_vec(value)?);
        match self.execute(operation).await {
            Ok(_) => Ok(true),
            Err(err) if err.status().is_conflict() => Ok(false),
            Err(err) => Err(err),
        }
    }

    pub(in crate::change_feed_processor) async fn replace<T: Serialize + Clone>(
        &self,
        id: &str,
        value: &T,
        etag: Etag,
    ) -> Result<ChangeFeedProcessorBootstrapStoredRecord<T>> {
        let response = self
            .execute(
                CosmosOperation::replace_item(self.item(id)?)
                    .with_body(serde_json::to_vec(value)?)
                    .with_precondition(Precondition::if_match(etag)),
            )
            .await?;
        let etag = response.headers().etag.clone().ok_or_else(|| {
            error(
                StatusCode::InternalServerError,
                "Lease response has no ETag.",
            )
        })?;
        Ok(ChangeFeedProcessorBootstrapStoredRecord {
            etag,
            value: value.clone(),
        })
    }

    pub(super) async fn release(&self, id: &str, etag: Etag) -> Result<()> {
        let operation = CosmosOperation::delete_item(self.item(id)?)
            .with_precondition(Precondition::if_match(etag));
        match self.execute(operation).await {
            Ok(_) => Ok(()),
            Err(err) if err.status().is_not_found() || err.status().is_precondition_failed() => {
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}
