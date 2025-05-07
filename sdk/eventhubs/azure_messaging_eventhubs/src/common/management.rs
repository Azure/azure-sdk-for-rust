// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::{retry_azure_operation, RetryOptions};
use crate::{
    error::{ErrorKind, EventHubsError},
    models::{EventHubPartitionProperties, EventHubProperties},
};
use azure_core::error::{ErrorKind as AzureErrorKind, Result};
use azure_core_amqp::{
    error::{AmqpErrorCondition, AmqpErrorKind},
    AmqpError, AmqpManagement, AmqpManagementApis, AmqpOrderedMap, AmqpSimpleValue, AmqpTimestamp,
    AmqpValue,
};
use std::{error::Error, time::SystemTime};
use tracing::{debug, warn};

pub(crate) struct ManagementInstance {
    pub management: AmqpManagement,
    retry_options: RetryOptions,
}

const EVENTHUB_ENTITY_TYPE: &str = "com.microsoft:eventhub";
const PARTITION_ENTITY_TYPE: &str = "com.microsoft:partition";

const EVENTHUB_PROPERTY_PARTITION_COUNT: &str = "partition_count";
const EVENTHUB_PROPERTY_PARTITION_IDS: &str = "partition_ids";
const EVENTHUB_PROPERTY_NAME: &str = "name";
const EVENTHUB_PROPERTY_PARTITION: &str = "partition";
const EVENTHUB_PROPERTY_CREATED_AT: &str = "created_at";

const EVENTHUB_PARTITION_PROPERTIES_TYPE: &str = "type";
const EVENTHUB_PARTITION_PROPERTIES_LAST_ENQUEUED_SEQUENCE_NUMBER_EPOCH: &str =
    "last_enqueued_sequence_number_epoch";
const EVENTHUB_PARTITION_PROPERTIES_BEGIN_SEQUENCE_NUMBER: &str = "begin_sequence_number";
const EVENTHUB_PARTITION_PROPERTIES_LAST_ENQUEUED_SEQUENCE_NUMBER: &str =
    "last_enqueued_sequence_number";
const EVENTHUB_PARTITION_PROPERTIES_LAST_ENQUEUED_OFFSET: &str = "last_enqueued_offset";
const EVENTHUB_PARTITION_PROPERTIES_LAST_ENQUEUED_TIME_UTC: &str = "last_enqueued_time_utc";
const EVENTHUB_PARTITION_PROPERTIES_IS_EMPTY: &str = "is_partition_empty";

impl ManagementInstance {
    pub fn new(management: AmqpManagement, retry_options: RetryOptions) -> Self {
        Self {
            management,
            retry_options,
        }
    }

    fn should_retry_management_response(e: &azure_core::Error) -> bool {
        match e.kind() {
            AzureErrorKind::Amqp => {
                warn!("Amqp operation failed: {}", e.source().unwrap());
                if let Some(e) = e.source() {
                    debug!("Error: {}", e);

                    if let Some(amqp_error) = e.downcast_ref::<Box<AmqpError>>() {
                        Self::should_retry_amqp_error(amqp_error)
                    } else if let Some(amqp_error) = e.downcast_ref::<AmqpError>() {
                        Self::should_retry_amqp_error(amqp_error)
                    } else {
                        debug!("Non AMQP error: {}", e);
                        false
                    }
                } else {
                    debug!("No source error found");
                    false
                }
            }
            _ => {
                debug!("Non AMQP error: {}", e);
                false
            }
        }
    }

    fn should_retry_amqp_error(amqp_error: &AmqpError) -> bool {
        match amqp_error.kind() {
            AmqpErrorKind::ManagementStatusCode(code, _) => {
                debug!("Management operation error: {}", code);
                match code {
                    // Retry on 408 (Request Timeout) and 429 (Too Many Requests)
                    azure_core::http::StatusCode::RequestTimeout
                    | azure_core::http::StatusCode::TooManyRequests
                    | azure_core::http::StatusCode::InternalServerError
                    | azure_core::http::StatusCode::BadGateway
                    | azure_core::http::StatusCode::ServiceUnavailable
                    | azure_core::http::StatusCode::GatewayTimeout => true,
                    _ => false,
                }
            }
            AmqpErrorKind::AmqpDescribedError(described_error) => {
                debug!("AMQP described error: {:?}", described_error);
                matches!(
                    described_error.condition(),
                    AmqpErrorCondition::ResourceLimitExceeded
                        | AmqpErrorCondition::ConnectionFramingError
                        | AmqpErrorCondition::LinkStolen
                )
            }
            _ => {
                debug!("Other AMQP error: {}", amqp_error);
                false
            }
        }
    }

    pub async fn get_eventhub_properties(&self, eventhub: &str) -> Result<EventHubProperties> {
        let response = retry_azure_operation(
            || async move {
                let mut application_properties: AmqpOrderedMap<String, AmqpSimpleValue> =
                    AmqpOrderedMap::new();
                application_properties.insert(EVENTHUB_PROPERTY_NAME.to_string(), eventhub.into());
                let response = self
                    .management
                    .call(EVENTHUB_ENTITY_TYPE.to_string(), application_properties)
                    .await?;
                Ok(response)
            },
            &self.retry_options,
            Some(Self::should_retry_management_response),
        )
        .await?;

        if !response.contains_key(EVENTHUB_PROPERTY_PARTITION_COUNT) {
            return Err(EventHubsError::from(ErrorKind::InvalidManagementResponse).into());
        }
        let name: String = response
            .get(EVENTHUB_PROPERTY_NAME)
            .ok_or_else(|| EventHubsError::from(ErrorKind::InvalidManagementResponse))?
            .into();
        let created_at: Option<SystemTime> = Into::<AmqpTimestamp>::into(
            response
                .get(EVENTHUB_PROPERTY_CREATED_AT)
                .ok_or_else(|| EventHubsError::from(ErrorKind::InvalidManagementResponse))?
                .clone(),
        )
        .0;

        let partition_ids = response
            .get(EVENTHUB_PROPERTY_PARTITION_IDS)
            .ok_or_else(|| EventHubsError::from(ErrorKind::InvalidManagementResponse))?;

        let partition_ids = match partition_ids {
            AmqpValue::Array(partition_ids) => partition_ids
                .iter()
                .map(|id| match id {
                    AmqpValue::String(id) => Ok(id.clone()),
                    _ => Err(EventHubsError::from(ErrorKind::InvalidManagementResponse).into()),
                })
                .collect::<Result<Vec<String>>>()?,
            _ => return Err(EventHubsError::from(ErrorKind::InvalidManagementResponse).into()),
        };
        Ok(EventHubProperties {
            name,
            created_on: created_at,
            partition_ids,
        })
    }

    pub async fn get_eventhub_partition_properties(
        &self,
        eventhub: &str,
        partition_id: &str,
    ) -> Result<EventHubPartitionProperties> {
        let response = retry_azure_operation(
            || async move {
                let mut application_properties: AmqpOrderedMap<String, AmqpSimpleValue> =
                    AmqpOrderedMap::new();
                application_properties.insert(EVENTHUB_PROPERTY_NAME.to_string(), eventhub.into());
                application_properties
                    .insert(EVENTHUB_PROPERTY_PARTITION.to_string(), partition_id.into());

                self.management
                    .call(PARTITION_ENTITY_TYPE.to_string(), application_properties)
                    .await
            },
            &self.retry_options,
            Some(Self::should_retry_management_response),
        )
        .await?;

        // Look for the required response properties
        if !response.contains_key(EVENTHUB_PARTITION_PROPERTIES_TYPE)
            || !response
                .contains_key(EVENTHUB_PARTITION_PROPERTIES_LAST_ENQUEUED_SEQUENCE_NUMBER_EPOCH)
        {
            return Err(EventHubsError::from(ErrorKind::InvalidManagementResponse).into());
        }

        Ok(EventHubPartitionProperties {
            beginning_sequence_number: response
                .get(EVENTHUB_PARTITION_PROPERTIES_BEGIN_SEQUENCE_NUMBER)
                .ok_or_else(|| EventHubsError::from(ErrorKind::InvalidManagementResponse))?
                .into(),
            id: response
                .get(EVENTHUB_PROPERTY_PARTITION)
                .ok_or_else(|| EventHubsError::from(ErrorKind::InvalidManagementResponse))?
                .into(),
            eventhub: response
                .get(EVENTHUB_PROPERTY_NAME)
                .ok_or_else(|| EventHubsError::from(ErrorKind::InvalidManagementResponse))?
                .into(),

            last_enqueued_sequence_number: response
                .get(EVENTHUB_PARTITION_PROPERTIES_LAST_ENQUEUED_SEQUENCE_NUMBER)
                .ok_or_else(|| EventHubsError::from(ErrorKind::InvalidManagementResponse))?
                .into(),
            last_enqueued_offset: response
                .get(EVENTHUB_PARTITION_PROPERTIES_LAST_ENQUEUED_OFFSET)
                .ok_or_else(|| EventHubsError::from(ErrorKind::InvalidManagementResponse))?
                .into(),
            last_enqueued_time_utc: Into::<AmqpTimestamp>::into(
                response
                    .get(EVENTHUB_PARTITION_PROPERTIES_LAST_ENQUEUED_TIME_UTC)
                    .ok_or_else(|| EventHubsError::from(ErrorKind::InvalidManagementResponse))?,
            )
            .0,
            is_empty: response
                .get(EVENTHUB_PARTITION_PROPERTIES_IS_EMPTY)
                .ok_or_else(|| EventHubsError::from(ErrorKind::InvalidManagementResponse))?
                .into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use azure_core_amqp::AmqpError;

    use crate::consumer;

    use super::*;

    #[test]
    fn should_retry_management_response() {
        consumer::tests::setup();

        {
            let error: azure_core::Error = AmqpError::new_management_error(
                azure_core::http::StatusCode::TooManyRequests,
                Some("Too many requests!".into()),
            )
            .into();

            assert!(ManagementInstance::should_retry_management_response(&error));
        }
        {
            let error: azure_core::Error = AmqpError::new_management_error(
                azure_core::http::StatusCode::SwitchingProtocols,
                Some("Switcheroo".into()),
            )
            .into();
            assert!(!ManagementInstance::should_retry_management_response(
                &error
            ));
        }
        // Verify that an explicitly boxed error is handled correctly
        {
            let error = azure_core::Error::new(
                AzureErrorKind::Amqp,
                Box::new(AmqpError::new_management_error(
                    azure_core::http::StatusCode::TooManyRequests,
                    Some("Too many requests!".into()),
                )),
            );
            assert!(ManagementInstance::should_retry_management_response(&error));
        }

        {
            let error: azure_core::Error = AmqpError::new_management_error(
                azure_core::http::StatusCode::BadGateway,
                Some("Bad Gateway".into()),
            )
            .into();
            assert!(ManagementInstance::should_retry_management_response(&error));
        }
        {
            let error: azure_core::Error = AmqpError::new_management_error(
                azure_core::http::StatusCode::RequestTimeout,
                Some("Request Timeout".into()),
            )
            .into();
            assert!(ManagementInstance::should_retry_management_response(&error));
        }
        {
            let error: azure_core::Error = AmqpError::new_management_error(
                azure_core::http::StatusCode::InternalServerError,
                Some("Internal Server Error".into()),
            )
            .into();
            assert!(ManagementInstance::should_retry_management_response(&error));
            {
                let error: azure_core::Error =
                    EventHubsError::from(ErrorKind::InvalidManagementResponse).into();
                assert!(!ManagementInstance::should_retry_management_response(
                    &error
                ));
            }

            {
                let error: azure_core::Error = AmqpError::new_described_error(
                    AmqpErrorCondition::ResourceLimitExceeded,
                    Some("Resource Limit Exceeded".into()),
                    Default::default(),
                )
                .into();

                assert!(ManagementInstance::should_retry_management_response(&error));
            }
            {
                let error: azure_core::Error = AmqpError::new_described_error(
                    AmqpErrorCondition::IllegalState,
                    Some("Illegal State".into()),
                    Default::default(),
                )
                .into();

                assert!(!ManagementInstance::should_retry_management_response(
                    &error
                ));
            }
        }
    }
}
