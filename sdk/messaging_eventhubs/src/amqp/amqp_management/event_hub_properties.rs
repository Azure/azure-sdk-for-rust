use std::borrow::Cow;

use fe2o3_amqp_management::{error::Error as ManagementError, Request, Response};
use fe2o3_amqp_types::{
    messaging::ApplicationProperties,
    primitives::{Array, OrderedMap},
};
use serde_amqp::Value;
use time::OffsetDateTime;

use crate::{amqp::amqp_management::response_map, event_hubs_properties::EventHubProperties};

use super::{
    EVENT_HUB_RESOURCE_TYPE_VALUE, READ_OPERATION_VALUE, RESOURCE_NAME_KEY, RESOURCE_TYPE_KEY,
    SECURITY_TOKEN_KEY,
};

#[derive(Debug, Clone)]
pub(crate) struct EventHubPropertiesRequest<'a> {
    event_hub_name: Cow<'a, str>,
    management_authorization_token: Cow<'a, str>,
}

impl<'a> EventHubPropertiesRequest<'a> {
    pub(crate) fn new<T, U>(event_hub_name: T, management_authorization_token: U) -> Self
    where
        T: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
    {
        Self {
            event_hub_name: event_hub_name.into(),
            management_authorization_token: management_authorization_token.into(),
        }
    }
}

fn encode_application_properties<'a>(
    event_hub_name: String,
    management_authorization_token: String,
) -> ApplicationProperties {
    // Encoding the operation entry will be taken care of by the Request trait
    ApplicationProperties::builder()
        .insert(RESOURCE_NAME_KEY, event_hub_name.as_ref())
        .insert(RESOURCE_TYPE_KEY, EVENT_HUB_RESOURCE_TYPE_VALUE)
        .insert(SECURITY_TOKEN_KEY, management_authorization_token.as_ref())
        .build()
}

impl<'a> Request for EventHubPropertiesRequest<'a> {
    const OPERATION: &'static str = READ_OPERATION_VALUE;

    type Response = EventHubProperties;

    type Body = ();

    fn encode_application_properties(&mut self) -> Option<ApplicationProperties> {
        Some(encode_application_properties(
            self.event_hub_name.to_string(),
            self.management_authorization_token.to_string(),
        ))
    }

    fn encode_body(self) -> Self::Body {
        
    }
}

impl<'a> Request for &'a EventHubPropertiesRequest<'a> {
    const OPERATION: &'static str = READ_OPERATION_VALUE;

    type Response = EventHubProperties;

    type Body = ();

    fn encode_application_properties(&mut self) -> Option<ApplicationProperties> {
        Some(encode_application_properties(
            self.event_hub_name.to_string(),
            self.management_authorization_token.to_string(),
        ))
    }

    fn encode_body(self) -> Self::Body {
        
    }
}

impl<'a> Request for &'a mut EventHubPropertiesRequest<'a> {
    const OPERATION: &'static str = READ_OPERATION_VALUE;

    type Response = EventHubProperties;

    type Body = ();

    fn encode_application_properties(&mut self) -> Option<ApplicationProperties> {
        Some(encode_application_properties(
            self.event_hub_name.to_string(),
            self.management_authorization_token.to_string(),
        ))
    }

    fn encode_body(self) -> Self::Body {
        
    }
}

type EventHubPropertiesResponseBody = OrderedMap<String, Value>;

impl Response for EventHubProperties {
    type Body = EventHubPropertiesResponseBody;

    const STATUS_CODE: u16 = 200;

    type Error = ManagementError;

    fn decode_message(
        message: fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<Self, Self::Error> {
        let mut body = message.body;
        let name = match body.remove(response_map::NAME) {
            Some(Value::String(name)) => name,
            _ => return Err(ManagementError::DecodeError(None)),
        };
        let created_on = match body.remove(response_map::CREATED_AT) {
            Some(Value::Timestamp(created_on)) => OffsetDateTime::from(created_on),
            _ => return Err(ManagementError::DecodeError(None)),
        };
        let partition_ids = match body.remove(response_map::PARTITION_IDENTIFIERS) {
            Some(Value::Array(Array(partition_ids))) | Some(Value::List(partition_ids)) => {
                partition_ids
                    .into_iter()
                    .map(|id| match id {
                        Value::String(id) => Ok(id),
                        _ => Err(ManagementError::DecodeError(None)),
                    })
                    .collect::<Result<Vec<String>, ManagementError>>()?
            }
            _ => return Err(ManagementError::DecodeError(None)),
        };

        Ok(EventHubProperties {
            name,
            created_on,
            partition_ids,
        })
    }
}
