// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

// cspell: words eventhub eventhubs amqp
use crate::{
    error::ErrorKind,
    models::{EventHubPartitionProperties, EventHubProperties},
};
use azure_core::error::Result;
use azure_core_amqp::{
    management::{AmqpManagement, AmqpManagementTrait},
    value::{AmqpOrderedMap, AmqpTimestamp, AmqpValue},
};
use std::time::SystemTime;

pub(crate) mod user_agent;

#[derive(Debug)]
pub(crate) struct ManagementInstance {
    pub(crate) management: AmqpManagement,
}

impl ManagementInstance {
    pub(crate) fn new(management: AmqpManagement) -> Self {
        Self { management }
    }

    pub(crate) async fn get_eventhub_properties(
        &self,
        eventhub: impl Into<String>,
    ) -> Result<EventHubProperties> {
        let mut application_properties: AmqpOrderedMap<String, AmqpValue> = AmqpOrderedMap::new();
        application_properties.insert("name", eventhub.into());

        let response = self
            .management
            .call("com.microsoft:eventhub", application_properties)
            .await?;

        if !response.contains_key("name")
            || !response.contains_key("type")
            || !response.contains_key("created_at")
            || !response.contains_key("partition_count")
            || !response.contains_key("partition_ids")
        {
            return Err(ErrorKind::InvalidManagementResponse.into());
        }
        let name: String = response.get("name").unwrap().clone().into();
        let created_at: SystemTime =
            Into::<AmqpTimestamp>::into(response.get("created_at").unwrap().clone()).0;
        //        let partition_count: i32 =
        //            Into::<i32>::into(response.get("partition_count".to_string()).unwrap().clone());

        let partition_ids = response.get("partition_ids").unwrap();
        let partition_ids = match partition_ids {
            AmqpValue::Array(partition_ids) => partition_ids
                .iter()
                .map(|id| match id {
                    AmqpValue::String(id) => Ok(id.clone()),
                    _ => Err(ErrorKind::InvalidManagementResponse.into()),
                })
                .collect::<Result<Vec<String>>>()?,
            _ => return Err(ErrorKind::InvalidManagementResponse.into()),
        };
        Ok(EventHubProperties {
            name,
            created_on: created_at,
            partition_ids,
        })
    }

    pub(crate) async fn get_eventhub_partition_properties(
        &self,
        eventhub: impl Into<String>,
        partition_id: impl Into<String>,
    ) -> Result<EventHubPartitionProperties> {
        let partition_id: String = partition_id.into();

        let mut application_properties: AmqpOrderedMap<String, AmqpValue> = AmqpOrderedMap::new();
        application_properties.insert("name", eventhub.into());
        application_properties.insert("partition", partition_id);

        let response = self
            .management
            .call("com.microsoft:partition", application_properties)
            .await?;

        // Look for the required response properties
        if !response.contains_key("name")
            || !response.contains_key("type")
            || !response.contains_key("partition")
            || !response.contains_key("begin_sequence_number_epoch")
            || !response.contains_key("begin_sequence_number")
            || !response.contains_key("last_enqueued_sequence_number_epoch")
            || !response.contains_key("last_enqueued_sequence_number")
            || !response.contains_key("last_enqueued_offset")
            || !response.contains_key("last_enqueued_time_utc")
            || !response.contains_key("is_partition_empty")
        {
            return Err(ErrorKind::InvalidManagementResponse.into());
        }

        Ok(EventHubPartitionProperties {
            beginning_sequence_number: response
                .get("begin_sequence_number")
                .unwrap()
                .clone()
                .into(),
            id: response.get("partition").unwrap().clone().into(),
            eventhub: response.get("name").unwrap().clone().into(),

            last_enqueued_sequence_number: response
                .get("last_enqueued_sequence_number")
                .unwrap()
                .clone()
                .into(),
            last_enqueued_offset: response.get("last_enqueued_offset").unwrap().clone().into(),
            last_enqueued_time_utc: Into::<AmqpTimestamp>::into(
                response
                    .get("last_enqueued_time_utc".to_string())
                    .unwrap()
                    .clone(),
            )
            .0,
            is_empty: response.get("is_partition_empty").unwrap().clone().into(),
        })
    }
}
