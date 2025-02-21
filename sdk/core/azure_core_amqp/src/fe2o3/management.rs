// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use std::borrow::BorrowMut;

use crate::{
    error::{AmqpError, AmqpErrorKind},
    management::{error::AmqpManagementError, AmqpManagementApis},
    session::AmqpSession,
    value::{AmqpOrderedMap, AmqpValue},
};
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
}

impl AmqpManagementApis for Fe2o3AmqpManagement {
    async fn attach(&self) -> Result<()> {
        let management = fe2o3_amqp_management::client::MgmtClient::builder()
            .client_node_addr(&self.client_node_name)
            .attach(self.session.lock().await.borrow_mut())
            .await
            .map_err(|e| {
                AmqpError::new(AmqpErrorKind::ManagementError(AmqpManagementError::from(e)))
            })?;

        self.management.set(Mutex::new(management)).map_err(|_| {
            azure_core::Error::from(AmqpError::new(AmqpErrorKind::ManagementError(
                AmqpManagementError::AmqpManagementAlreadyAttached,
            )))
        })?;
        Ok(())
    }

    async fn detach(mut self) -> Result<()> {
        // Detach the management client from the session.
        let management = self.management.take().ok_or_else(|| {
            azure_core::Error::from(AmqpError::new(AmqpErrorKind::ManagementError(
                AmqpManagementError::AmqpManagementNotAttached,
            )))
        })?;
        let management = management.into_inner();
        management
            .close()
            .await
            .map_err(AmqpManagementError::from)?;
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
            .ok_or_else(|| {
                azure_core::Error::from(AmqpError::new(AmqpErrorKind::ManagementError(
                    AmqpManagementError::AmqpManagementNotAttached,
                )))
            })?
            .lock()
            .await;

        let request = WithApplicationPropertiesRequest::new(
            operation_type,
            &self.access_token,
            application_properties,
        );

        let response = management.call(request).await;
        if let Err(e) = response {
            let e = AmqpManagementError::try_from(e)?;
            Err(azure_core::Error::from(AmqpError::new(
                AmqpErrorKind::ManagementError(e),
            )))
        } else {
            Ok(response.unwrap().entity_attributes.into())
        }
    }
}

impl TryFrom<fe2o3_amqp_management::error::Error> for AmqpManagementError {
    type Error = azure_core::Error;
    fn try_from(e: fe2o3_amqp_management::error::Error) -> std::result::Result<Self, Self::Error> {
        match e {
            fe2o3_amqp_management::error::Error::CorrelationIdAndMessageIdAreNone => {
                Ok(AmqpManagementError::InvalidManagementResponse(
                    "CorrelationId and MessageId are not present.".to_string(),
                ))
            }

            fe2o3_amqp_management::error::Error::StatusCodeNotFound => {
                Ok(AmqpManagementError::InvalidManagementResponse(
                    "Status code not found.".to_string(),
                ))
            }

            fe2o3_amqp_management::error::Error::DecodeError(_t) => {
                Ok(AmqpManagementError::DecodingError)
            }

            fe2o3_amqp_management::error::Error::Status(s) => {
                Ok(AmqpManagementError::HttpStatusCode(
                    azure_core::StatusCode::try_from(s.code.0.get()).map_err(|_| {
                        azure_core::Error::message(
                            azure_core::error::ErrorKind::DataConversion,
                            format!("invalid status code {s}"),
                        )
                    })?,
                    s.description.clone(),
                ))
            }

            fe2o3_amqp_management::error::Error::Send(s) => {
                Ok(AmqpManagementError::SendError(s.into()))
            }

            fe2o3_amqp_management::error::Error::NotAccepted(_o) => {
                Ok(AmqpManagementError::NotAccepted)
            }

            fe2o3_amqp_management::error::Error::Recv(r) => {
                Ok(AmqpManagementError::ReceiveError(r.into()))
            }

            fe2o3_amqp_management::error::Error::Disposition(_d) => {
                Ok(AmqpManagementError::Disposition)
            }
        }
    }
}

impl From<fe2o3_amqp_management::error::AttachError> for AmqpManagementError {
    fn from(e: fe2o3_amqp_management::error::AttachError) -> Self {
        match e {
            fe2o3_amqp_management::error::AttachError::Sender(s) => {
                AmqpManagementError::SendError(s.into())
            }
            fe2o3_amqp_management::error::AttachError::Receiver(r) => {
                AmqpManagementError::ReceiveError(r.into())
            }
        }
    }
}

impl From<fe2o3_amqp::link::DetachError> for AmqpManagementError {
    fn from(e: fe2o3_amqp::link::DetachError) -> Self {
        AmqpManagementError::DetachError(e.into())
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
