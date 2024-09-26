// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
// cspell: words amqp servicebus eventhub mgmt

use std::borrow::BorrowMut;

use crate::{
    management::AmqpManagementApis,
    session::AmqpSession,
    value::{AmqpOrderedMap, AmqpValue},
};

use async_std::sync::Mutex;
use azure_core::{credentials::AccessToken, error::Result};
use fe2o3_amqp_management::operations::ReadResponse;
use fe2o3_amqp_types::{messaging::ApplicationProperties, primitives::SimpleValue};
use std::sync::{Arc, OnceLock};
use tracing::debug;

use super::error::{AmqpManagement, AmqpManagementAttach};

#[derive(Debug)]
pub(crate) struct Fe2o3AmqpManagement {
    client_node_name: String,
    access_token: AccessToken,
    session: Arc<Mutex<fe2o3_amqp::session::SessionHandle<()>>>,
    management: OnceLock<Mutex<fe2o3_amqp_management::MgmtClient>>,
}

impl Drop for Fe2o3AmqpManagement {
    fn drop(&mut self) {
        debug!("Dropping Fe2o3AmqpManagement.");
    }
}

impl Fe2o3AmqpManagement {
    pub fn new(
        session: AmqpSession,
        client_node_name: impl Into<String>,
        access_token: AccessToken,
    ) -> Result<Self> {
        // Session::get() returns a clone of the underlying session handle.
        let session = session.implementation.get()?;

        Ok(Self {
            access_token,
            client_node_name: client_node_name.into(),
            session,
            management: OnceLock::new(),
        })
    }
}

impl AmqpManagementApis for Fe2o3AmqpManagement {
    async fn attach(&self) -> Result<()> {
        let management = fe2o3_amqp_management::client::MgmtClient::builder()
            .client_node_addr(&self.client_node_name)
            .attach(self.session.lock().await.borrow_mut())
            .await
            .map_err(AmqpManagementAttach::from)?;

        self.management.set(Mutex::new(management)).map_err(|_| {
            azure_core::Error::message(
                azure_core::error::ErrorKind::Other,
                "Management is already set.",
            )
        })?;
        Ok(())
    }
    async fn call(
        &self,
        operation_type: impl Into<String>,
        application_properties: AmqpOrderedMap<String, AmqpValue>,
    ) -> Result<AmqpOrderedMap<String, AmqpValue>> {
        let mut management = self
            .management
            .get()
            .ok_or_else(|| {
                azure_core::Error::message(
                    azure_core::error::ErrorKind::Other,
                    "management is not set.",
                )
            })?
            .lock()
            .await;

        let request = WithApplicationPropertiesRequest::new(
            operation_type,
            &self.access_token,
            application_properties,
        );

        let response = management
            .call(request)
            .await
            .map_err(AmqpManagement::from)?;
        Ok(response.entity_attributes.into())
    }
}

struct WithApplicationPropertiesRequest<'a> {
    entity_type: String,
    access_token: &'a AccessToken,
    application_properties: AmqpOrderedMap<String, AmqpValue>,
}

impl<'a> WithApplicationPropertiesRequest<'a> {
    pub fn new(
        entity_type: impl Into<String>,
        access_token: &'a AccessToken,
        application_properties: AmqpOrderedMap<String, AmqpValue>,
    ) -> Self {
        Self {
            entity_type: entity_type.into(),
            access_token,
            application_properties,
        }
    }
}

impl<'a> fe2o3_amqp_management::Request for WithApplicationPropertiesRequest<'a> {
    const OPERATION: &'static str = "READ";
    type Response = ReadResponse;
    type Body = ();

    fn manageable_entity_type(&mut self) -> Option<String> {
        Some(self.entity_type.clone())
    }
    fn locales(&mut self) -> Option<String> {
        None
    }
    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        let builder = ApplicationProperties::builder();
        let builder = self
            .application_properties
            .iter()
            .fold(builder, |builder, (key, value)| {
                builder.insert(
                    key.clone(),
                    Into::<fe2o3_amqp_types::primitives::SimpleValue>::into(value),
                )
            })
            .insert(
                "security_token",
                Into::<SimpleValue>::into(self.access_token.token.secret()),
            );
        Some(builder.build())
    }
    fn encode_body(self) -> Self::Body {}
}
