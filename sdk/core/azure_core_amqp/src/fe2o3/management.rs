// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use std::borrow::BorrowMut;

use crate::{
    error::AmqpErrorKind,
    fe2o3::error::Fe2o3ManagementError,
    management::AmqpManagementApis,
    session::AmqpSession,
    value::{AmqpOrderedMap, AmqpValue},
    AmqpError,
};
use async_trait::async_trait;
use azure_core::{credentials::AccessToken, Result};
use fe2o3_amqp_management::operations::ReadResponse;
use fe2o3_amqp_types::{messaging::ApplicationProperties, primitives::SimpleValue};
use std::sync::{Arc, OnceLock};
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

    fn amqp_management_already_attached() -> azure_core::Error {
        azure_core::Error::message(
            azure_core::error::ErrorKind::Amqp,
            "AMQP Management is already attached",
        )
    }
    fn amqp_management_not_attached() -> azure_core::Error {
        azure_core::Error::message(
            azure_core::error::ErrorKind::Amqp,
            "AMQP Management is not attached",
        )
    }
}

#[async_trait]
impl AmqpManagementApis for Fe2o3AmqpManagement {
    async fn attach(&self) -> Result<()> {
        let management = fe2o3_amqp_management::client::MgmtClient::builder()
            .client_node_addr(&self.client_node_name)
            .attach(self.session.lock().await.borrow_mut())
            .await
            .map_err(|e| azure_core::Error::from(AmqpError::from(e)))?;

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
        Ok(())
    }

    async fn call(
        &self,
        operation_type: String,
        application_properties: AmqpOrderedMap<String, AmqpValue>,
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

        let response = management.call(request).await;
        if let Err(e) = response {
            let e = azure_core::Error::try_from(Fe2o3ManagementError(e))?;
            Err(e)
        } else {
            Ok(response.unwrap().entity_attributes.into())
        }
    }
}

impl TryFrom<Fe2o3ManagementError> for azure_core::Error {
    type Error = azure_core::Error;
    fn try_from(e: Fe2o3ManagementError) -> std::result::Result<Self, Self::Error> {
        let e = AmqpError::try_from(e.0)?;
        Ok(e.into())
    }
}

impl TryFrom<fe2o3_amqp_management::error::Error> for AmqpError {
    type Error = azure_core::Error;
    fn try_from(e: fe2o3_amqp_management::error::Error) -> std::result::Result<Self, Self::Error> {
        match e {
            fe2o3_amqp_management::error::Error::DecodeError(_)
            | fe2o3_amqp_management::error::Error::StatusCodeNotFound
            | fe2o3_amqp_management::error::Error::NotAccepted(_)
            | fe2o3_amqp_management::error::Error::CorrelationIdAndMessageIdAreNone => Ok(
                AmqpError::from(AmqpErrorKind::TransportImplementationError(Box::new(e))),
            ),

            fe2o3_amqp_management::error::Error::Status(s) => {
                Ok(AmqpError::from(AmqpErrorKind::ManagementStatusCode(
                    azure_core::http::StatusCode::from(s.code.0.get()),
                    s.description.clone(),
                )))
            }
            fe2o3_amqp_management::error::Error::Send(s) => Ok(AmqpError::from(s)),
            fe2o3_amqp_management::error::Error::Recv(r) => Ok(AmqpError::from(r)),
            fe2o3_amqp_management::error::Error::Disposition(d) => Ok(AmqpError::from(d)),
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
    application_properties: AmqpOrderedMap<String, AmqpValue>,
}

impl<'a> WithApplicationPropertiesRequest<'a> {
    pub fn new(
        entity_type: String,
        access_token: &'a AccessToken,
        application_properties: AmqpOrderedMap<String, AmqpValue>,
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
