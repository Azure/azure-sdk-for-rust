// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    connection::AmqpConnection,
    error::{AmqpErrorKind, Result},
    session::{AmqpSessionApis, AmqpSessionOptions},
    AmqpError,
};
use std::{
    borrow::BorrowMut,
    sync::{Arc, OnceLock},
};
use tokio::sync::Mutex;
use tracing::{debug, trace};

#[derive(Debug, Clone, Default)]
pub(crate) struct Fe2o3AmqpSession {
    session: OnceLock<Arc<Mutex<fe2o3_amqp::session::SessionHandle<()>>>>,
}

impl Drop for Fe2o3AmqpSession {
    fn drop(&mut self) {
        debug!("Dropping Fe2o3AmqpSession: {:?}.", self.session);
    }
}

impl Fe2o3AmqpSession {
    pub fn new() -> Self {
        Self {
            session: OnceLock::new(),
        }
    }

    /// Returns a reference to the session handle
    pub fn get(&self) -> Result<Arc<Mutex<fe2o3_amqp::session::SessionHandle<()>>>> {
        Ok(self
            .session
            .get()
            .ok_or_else(Self::session_not_set)?
            .clone())
    }

    fn session_already_attached() -> AmqpError {
        AmqpError::with_message("AMQP Session is already attached")
    }
    fn session_not_set() -> AmqpError {
        AmqpError::with_message("AMQP Session is not set")
    }
    fn could_not_set_session() -> AmqpError {
        AmqpError::with_message("Could not set AMQP Session")
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AmqpSessionApis for Fe2o3AmqpSession {
    async fn begin(
        &self,
        connection: &AmqpConnection,
        options: Option<AmqpSessionOptions>,
    ) -> Result<()> {
        let mut connection = connection
            .implementation
            .get()
            .get()
            .ok_or_else(Self::session_already_attached)?
            .lock()
            .await;

        let mut session_builder = fe2o3_amqp::session::Session::builder();

        if let Some(options) = options {
            if let Some(incoming_window) = options.incoming_window {
                session_builder = session_builder.incoming_window(incoming_window);
            }
            if let Some(outgoing_window) = options.outgoing_window {
                session_builder = session_builder.outgoing_window(outgoing_window);
            }
            if let Some(handle_max) = options.handle_max {
                session_builder = session_builder.handle_max(handle_max);
            }
            if let Some(offered_capabilities) = options.offered_capabilities {
                session_builder = session_builder.set_offered_capabilities(
                    offered_capabilities.iter().map(Into::into).collect(),
                );
            }
            if let Some(desired_capabilities) = options.desired_capabilities {
                session_builder = session_builder.set_desired_capabilities(
                    desired_capabilities.iter().map(Into::into).collect(),
                );
            }
            if let Some(properties) = options.properties {
                session_builder = session_builder.properties(
                    properties
                        .iter()
                        .map(|(k, v)| (k.into(), v.into()))
                        .collect(),
                );
            }
            if let Some(buffer_size) = options.buffer_size {
                session_builder = session_builder.buffer_size(buffer_size);
            }
        }
        let session = session_builder
            .begin(connection.borrow_mut())
            .await
            .map_err(AmqpError::from)?;
        self.session
            .set(Arc::new(Mutex::new(session)))
            .map_err(|_| Self::could_not_set_session())?;
        Ok(())
    }

    async fn end(&self) -> Result<()> {
        let mut session = self
            .session
            .get()
            .ok_or_else(Self::session_not_set)?
            .lock()
            .await;
        if session.is_ended() {
            trace!("Session already ended, returning.");
            return Ok(());
        }
        session.end().await.map_err(AmqpError::from)?;
        Ok(())
    }
}

impl From<fe2o3_amqp::session::BeginError> for AmqpError {
    fn from(e: fe2o3_amqp::session::BeginError) -> Self {
        match e {
            fe2o3_amqp::session::BeginError::IllegalState
            | fe2o3_amqp::session::BeginError::IllegalConnectionState => {
                AmqpErrorKind::ConnectionDropped(Box::new(e)).into()
            }
            fe2o3_amqp::session::BeginError::RemoteEnded => {
                AmqpErrorKind::SessionClosedByRemote(Box::new(e)).into()
            }
            fe2o3_amqp::session::BeginError::RemoteEndedWithError(error) => {
                AmqpErrorKind::AmqpDescribedError(error.into()).into()
            }
            fe2o3_amqp::session::BeginError::LocalChannelMaxReached => {
                AmqpErrorKind::TransportImplementationError(Box::new(e)).into()
            }
        }
    }
}

impl From<fe2o3_amqp::session::Error> for AmqpError {
    fn from(e: fe2o3_amqp::session::Error) -> Self {
        match e {
            fe2o3_amqp::session::Error::UnattachedHandle
            | fe2o3_amqp::session::Error::RemoteAttachingLinkNameNotFound
            | fe2o3_amqp::session::Error::HandleInUse
            | fe2o3_amqp::session::Error::IllegalState
            | fe2o3_amqp::session::Error::IllegalConnectionState
            | fe2o3_amqp::session::Error::TransferFrameToSender => {
                AmqpErrorKind::TransportImplementationError(Box::new(e)).into()
            }
            fe2o3_amqp::session::Error::RemoteEnded => {
                AmqpErrorKind::SessionClosedByRemote(Box::new(e)).into()
            }
            fe2o3_amqp::session::Error::RemoteEndedWithError(error) => {
                AmqpErrorKind::AmqpDescribedError(error.into()).into()
            }
        }
    }
}
