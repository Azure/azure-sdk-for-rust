use std::borrow::Cow;

use fe2o3_amqp_management::{error::Error as ManagementError, Request, Response};
use fe2o3_amqp_types::{messaging::ApplicationProperties, primitives::OrderedMap};
use serde_amqp::Value;
use time::OffsetDateTime;

use crate::{amqp::amqp_management::response_map, PartitionProperties};

use super::{
    PARTITION_NAME_KEY, PARTITION_RESOURCE_TYPE_VALUE, READ_OPERATION_VALUE, RESOURCE_NAME_KEY,
    RESOURCE_TYPE_KEY, SECURITY_TOKEN_KEY,
};

#[derive(Debug, Clone)]
pub(crate) struct PartitionPropertiesRequest<'a> {
    event_hub_name: Cow<'a, str>,
    partition_identifier: Cow<'a, str>,
    management_authorization_token: Cow<'a, str>,
}

impl<'a> PartitionPropertiesRequest<'a> {
    pub(crate) fn new<T, U, V>(
        event_hub_name: T,
        partition_identifier: U,
        management_authorization_token: V,
    ) -> Self
    where
        T: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        Self {
            event_hub_name: event_hub_name.into(),
            partition_identifier: partition_identifier.into(),
            management_authorization_token: management_authorization_token.into(),
        }
    }
}

fn encode_application_properties(
    event_hub_name: String,
    partition_identifier: String,
    management_authorization_token: String,
) -> ApplicationProperties {
    // Encoding the operation entry will be taken care of by the Request trait
    ApplicationProperties::builder()
        .insert(RESOURCE_NAME_KEY, event_hub_name)
        .insert(PARTITION_NAME_KEY, partition_identifier)
        .insert(RESOURCE_TYPE_KEY, PARTITION_RESOURCE_TYPE_VALUE)
        .insert(SECURITY_TOKEN_KEY, management_authorization_token)
        .build()
}

impl<'a> Request for PartitionPropertiesRequest<'a> {
    const OPERATION: &'static str = READ_OPERATION_VALUE;

    type Response = PartitionProperties;

    type Body = ();

    fn encode_application_properties(&mut self) -> Option<ApplicationProperties> {
        Some(encode_application_properties(
            self.event_hub_name.to_string(),
            self.partition_identifier.to_string(),
            self.management_authorization_token.to_string(),
        ))
    }

    fn encode_body(self) -> Self::Body {
        
    }
}

impl<'a> Request for &'a PartitionPropertiesRequest<'a> {
    const OPERATION: &'static str = READ_OPERATION_VALUE;

    type Response = PartitionProperties;

    type Body = ();

    fn encode_application_properties(&mut self) -> Option<ApplicationProperties> {
        Some(encode_application_properties(
            self.event_hub_name.to_string(),
            self.partition_identifier.to_string(),
            self.management_authorization_token.to_string(),
        ))
    }

    fn encode_body(self) -> Self::Body {
        
    }
}

impl<'a> Request for &'a mut PartitionPropertiesRequest<'a> {
    const OPERATION: &'static str = READ_OPERATION_VALUE;

    type Response = PartitionProperties;

    type Body = ();

    fn encode_application_properties(&mut self) -> Option<ApplicationProperties> {
        Some(encode_application_properties(
            self.event_hub_name.to_string(),
            self.partition_identifier.to_string(),
            self.management_authorization_token.to_string(),
        ))
    }

    fn encode_body(self) -> Self::Body {
        
    }
}

type PartitionPropertiesResponseBody = OrderedMap<String, Value>;

impl Response for PartitionProperties {
    const STATUS_CODE: u16 = 200;

    type Body = PartitionPropertiesResponseBody;

    type Error = ManagementError;

    fn decode_message(
        message: fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<Self, Self::Error> {
        let mut body = message.body;
        let event_hub_name = match body.remove(response_map::NAME) {
            Some(Value::String(name)) => name,
            _ => return Err(ManagementError::DecodeError(None)),
        };
        let partition_identifier = match body.remove(response_map::PARTITION_IDENTIFIER) {
            Some(Value::String(id)) => id,
            _ => return Err(ManagementError::DecodeError(None)),
        };
        let is_empty = match body.remove(response_map::PARTITION_RUNTIME_INFO_PARTITION_IS_EMPTY) {
            Some(Value::Bool(is_empty)) => is_empty,
            _ => return Err(ManagementError::DecodeError(None)),
        };
        let beginning_sequence_number =
            match body.remove(response_map::PARTITION_BEGIN_SEQUENCE_NUMBER) {
                Some(Value::Long(seq)) => seq,
                _ => return Err(ManagementError::DecodeError(None)),
            };
        let last_sequence_number =
            match body.remove(response_map::PARTITION_LAST_ENQUEUED_SEQUENCE_NUMBER) {
                Some(Value::Long(seq)) => seq,
                _ => return Err(ManagementError::DecodeError(None)),
            };
        let last_offset = match body.remove(response_map::PARTITION_LAST_ENQUEUED_OFFSET) {
            Some(Value::String(offset)) => {
                str::parse::<i64>(&offset).map_err(|_| ManagementError::DecodeError(None))?
            }
            Some(Value::Long(offset)) => offset,
            _ => return Err(ManagementError::DecodeError(None)),
        };
        let last_enqueued_time = match body.remove(response_map::PARTITION_LAST_ENQUEUED_TIME_UTC) {
            Some(Value::Timestamp(time)) => OffsetDateTime::from(time),
            _ => return Err(ManagementError::DecodeError(None)),
        };

        Ok(PartitionProperties {
            event_hub_name,
            id: partition_identifier,
            beginning_sequence_number,
            last_enqueued_sequence_number: last_sequence_number,
            last_enqueued_offset: last_offset,
            last_enqueued_time,
            is_empty,
        })
    }
}
