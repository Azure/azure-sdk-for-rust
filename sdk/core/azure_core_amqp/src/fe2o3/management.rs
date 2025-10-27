// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    error::{AmqpErrorKind, Result},
    management::AmqpManagementApis,
    session::AmqpSession,
    simple_value::AmqpSimpleValue,
    value::{AmqpOrderedMap, AmqpValue},
    AmqpError,
};
use azure_core::credentials::AccessToken;
use fe2o3_amqp_management::operations::ReadResponse;
use std::{
    borrow::BorrowMut,
    sync::{Arc, OnceLock},
};
use tokio::sync::Mutex;
use tracing::debug;

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
        client_node_name: String,
        access_token: AccessToken,
    ) -> Result<Self> {
        // Session::get() returns a clone of the underlying session handle.
        let session = session.implementation.get()?;

        Ok(Self {
            access_token,
            client_node_name,
            session,
            management: OnceLock::new(),
        })
    }

    fn amqp_management_already_attached() -> AmqpError {
        AmqpError::with_message("AMQP Management is already attached")
    }
    fn amqp_management_not_attached() -> AmqpError {
        AmqpError::with_message("AMQP Management is not attached")
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AmqpManagementApis for Fe2o3AmqpManagement {
    async fn attach(&self) -> Result<()> {
        let management = fe2o3_amqp_management::client::MgmtClient::builder()
            .client_node_addr(&self.client_node_name)
            .attach(self.session.lock().await.borrow_mut())
            .await
            .map_err(AmqpError::from)?;

        self.management
            .set(Mutex::new(management))
            .map_err(|_| Self::amqp_management_already_attached())?;
        Ok(())
    }

    async fn detach(mut self) -> Result<()> {
        // Detach the management client from the session.
        let management = self
            .management
            .take()
            .ok_or_else(Self::amqp_management_not_attached)?;
        let management = management.into_inner();
        management.close().await.map_err(AmqpError::from)?;

        let mut session = self.session.lock().await;
        session.end().await.map_err(AmqpError::from)?;
        Ok(())
    }

    async fn call(
        &self,
        operation_type: String,
        application_properties: AmqpOrderedMap<String, AmqpSimpleValue>,
    ) -> Result<AmqpOrderedMap<String, AmqpValue>> {
        let mut management = self
            .management
            .get()
            .ok_or_else(Self::amqp_management_not_attached)?
            .lock()
            .await;

        let request = WithApplicationPropertiesRequest::new(
            operation_type,
            &self.access_token,
            application_properties,
        );

        let response = management.call(request).await.map_err(AmqpError::from)?;
        Ok((&response.entity_attributes).into())
    }
}

impl From<fe2o3_amqp_management::error::Error> for AmqpError {
    fn from(e: fe2o3_amqp_management::error::Error) -> Self {
        match e {
            fe2o3_amqp_management::error::Error::DecodeError(_)
            | fe2o3_amqp_management::error::Error::StatusCodeNotFound
            | fe2o3_amqp_management::error::Error::NotAccepted(_)
            | fe2o3_amqp_management::error::Error::CorrelationIdAndMessageIdAreNone => {
                AmqpError::from(AmqpErrorKind::TransportImplementationError(Box::new(e)))
            }

            fe2o3_amqp_management::error::Error::Status(s) => {
                AmqpError::from(AmqpErrorKind::ManagementStatusCode(
                    azure_core::http::StatusCode::from(s.code.0.get()),
                    s.description.clone(),
                ))
            }
            fe2o3_amqp_management::error::Error::Send(s) => AmqpError::from(s),
            fe2o3_amqp_management::error::Error::Recv(r) => AmqpError::from(r),
            fe2o3_amqp_management::error::Error::Disposition(d) => AmqpError::from(d),
        }
    }
}

impl From<fe2o3_amqp_management::error::AttachError> for AmqpError {
    fn from(e: fe2o3_amqp_management::error::AttachError) -> Self {
        match e {
            fe2o3_amqp_management::error::AttachError::Sender(s) => AmqpError::from(s),
            fe2o3_amqp_management::error::AttachError::Receiver(r) => AmqpError::from(r),
        }
    }
}

struct WithApplicationPropertiesRequest<'a> {
    entity_type: String,
    access_token: &'a AccessToken,
    application_properties: AmqpOrderedMap<String, AmqpSimpleValue>,
}

impl<'a> WithApplicationPropertiesRequest<'a> {
    pub fn new(
        entity_type: String,
        access_token: &'a AccessToken,
        application_properties: AmqpOrderedMap<String, AmqpSimpleValue>,
    ) -> Self {
        Self {
            entity_type,
            access_token,
            application_properties,
        }
    }
}

impl fe2o3_amqp_management::Request for WithApplicationPropertiesRequest<'_> {
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
        let mut fe2o3_application_properties = self
            .application_properties
            .iter()
            .map(|(key, value)| {
                (
                    key.clone(),
                    Into::<fe2o3_amqp_types::primitives::SimpleValue>::into(value),
                )
            })
            .collect::<fe2o3_amqp_types::primitives::OrderedMap<_, _>>();
        fe2o3_application_properties.insert(
            "security_token".to_string(),
            fe2o3_amqp_types::primitives::SimpleValue::from(self.access_token.token.secret()),
        );

        Some(fe2o3_amqp_types::messaging::ApplicationProperties(
            fe2o3_application_properties,
        ))
    }
    fn encode_body(self) -> Self::Body {}
}
