// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    connection::{AmqpConnectionApis, AmqpConnectionOptions},
    error::{AmqpErrorKind, Result},
    fe2o3::error::{Fe2o3ConnectionError, Fe2o3ConnectionOpenError, Fe2o3TransportError},
    value::{AmqpOrderedMap, AmqpSymbol, AmqpValue},
    AmqpError,
};
use azure_core::http::Url;
use fe2o3_amqp::connection::ConnectionHandle;
use std::{borrow::BorrowMut, sync::OnceLock};
use tokio::sync::Mutex;
use tracing::{debug, warn};

#[derive(Debug, Default)]
pub(crate) struct Fe2o3AmqpConnection {
    connection: OnceLock<Mutex<ConnectionHandle<()>>>,
}

impl Fe2o3AmqpConnection {
    pub fn new() -> Self {
        Self {
            connection: OnceLock::new(),
        }
    }

    pub fn get(&self) -> &OnceLock<Mutex<ConnectionHandle<()>>> {
        &self.connection
    }

    fn connection_not_set() -> AmqpError {
        AmqpError::with_message("Connection is not set")
    }
    fn connection_already_set() -> AmqpError {
        AmqpError::with_message("Connection is already set")
    }
}

impl Drop for Fe2o3AmqpConnection {
    fn drop(&mut self) {
        debug!("Dropping Fe2o3AmqpConnection.");
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AmqpConnectionApis for Fe2o3AmqpConnection {
    async fn open(
        &self,
        id: String,
        url: Url,
        options: Option<AmqpConnectionOptions>,
    ) -> Result<()> {
        {
            let options = options.unwrap_or_default();
            let mut endpoint = url.clone();

            // All AMQP clients have a similar set of options.
            let mut builder = fe2o3_amqp::Connection::builder()
                .sasl_profile(fe2o3_amqp::sasl_profile::SaslProfile::Anonymous)
                .alt_tls_establishment(true)
                .container_id(id)
                .max_frame_size(65536);

            if let Some(frame_size) = options.max_frame_size {
                builder = builder.max_frame_size(frame_size);
            }

            if let Some(channel_max) = options.channel_max {
                builder = builder.channel_max(channel_max);
            }
            if let Some(idle_timeout) = options.idle_timeout {
                builder = builder.idle_time_out(idle_timeout.whole_milliseconds() as u32);
            }
            if let Some(outgoing_locales) = options.outgoing_locales {
                builder = builder.set_outgoing_locales(
                    outgoing_locales
                        .into_iter()
                        .map(fe2o3_amqp_types::primitives::Symbol::from)
                        .collect(),
                );
            }
            if let Some(incoming_locales) = options.incoming_locales {
                builder = builder.set_incoming_locales(
                    incoming_locales
                        .into_iter()
                        .map(fe2o3_amqp_types::primitives::Symbol::from)
                        .collect(),
                );
            }
            if let Some(offered_capabilities) = options.offered_capabilities {
                builder = builder.set_offered_capabilities(
                    offered_capabilities.into_iter().map(Into::into).collect(),
                );
            }
            if let Some(desired_capabilities) = options.desired_capabilities {
                builder = builder.set_desired_capabilities(
                    desired_capabilities.into_iter().map(Into::into).collect(),
                );
            }
            if let Some(properties) = options.properties {
                builder = builder.properties(
                    properties
                        .iter()
                        .map(|(k, v)| (k.into(), v.into()))
                        .collect(),
                );
            }
            if let Some(buffer_size) = options.buffer_size {
                builder = builder.buffer_size(buffer_size);
            }

            if let Some(custom_endpoint) = options.custom_endpoint {
                endpoint = custom_endpoint;
                builder = builder.hostname(url.host_str());
            }

            self.connection
                .set(Mutex::new(
                    builder
                        .open(endpoint)
                        .await
                        .map_err(|e| AmqpError::from(Fe2o3ConnectionOpenError(e)))?,
                ))
                .map_err(|_| Self::connection_already_set())?;
            Ok(())
        }
    }

    async fn close(&self) -> Result<()> {
        let mut connection = self
            .connection
            .get()
            .ok_or_else(Self::connection_not_set)?
            .lock()
            .await;
        connection
            .borrow_mut()
            .close()
            .await
            .map_err(|e| AmqpError::from(Fe2o3ConnectionError(e)))?;
        Ok(())
    }

    async fn close_with_error(
        &self,
        condition: AmqpSymbol,
        description: Option<String>,
        info: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
    ) -> Result<()> {
        let mut connection = self
            .connection
            .get()
            .ok_or_else(Self::connection_not_set)?
            .lock()
            .await;
        let res = connection
            .borrow_mut()
            .close_with_error(fe2o3_amqp::types::definitions::Error::new(
                fe2o3_amqp::types::definitions::ErrorCondition::Custom(
                    fe2o3_amqp_types::primitives::Symbol::from(condition),
                ),
                description,
                info.map(Into::into),
            ))
            .await
            .map_err(|e| AmqpError::from(Fe2o3ConnectionError(e)));
        // If we're closing with an error, then we might get the transport error back before we get the error back.
        // that's ok.
        match res {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                AmqpErrorKind::AzureCore(err)
                    if matches!(err.kind(), azure_core::error::ErrorKind::Io) =>
                {
                    warn!("I/O closing connection, ignored: {:?}", e);
                    Ok(())
                }
                _ => Err(e),
            },
        }
    }
}

impl From<Fe2o3ConnectionOpenError> for AmqpError {
    fn from(e: Fe2o3ConnectionOpenError) -> Self {
        match e.0 {
            fe2o3_amqp::connection::OpenError::Io(e) => azure_core::Error::from(e).into(),
            fe2o3_amqp::connection::OpenError::UrlError(parse_error) => {
                azure_core::Error::from(parse_error).into()
            }
            fe2o3_amqp::connection::OpenError::RemoteClosed => {
                AmqpErrorKind::ConnectionClosedByRemote(Box::new(e.0)).into()
            }
            fe2o3_amqp::connection::OpenError::RemoteClosedWithError(error) => {
                AmqpErrorKind::AmqpDescribedError(error.into()).into()
            }
            fe2o3_amqp::connection::OpenError::TransportError(error) => {
                AmqpError::from(Fe2o3TransportError(error))
            }
            _ => AmqpErrorKind::TransportImplementationError(Box::new(e.0)).into(),
        }
    }
}

impl From<Fe2o3ConnectionError> for AmqpError {
    fn from(e: Fe2o3ConnectionError) -> Self {
        match e.0 {
            fe2o3_amqp::connection::Error::TransportError(error) => {
                AmqpError::from(Fe2o3TransportError(error))
            }
            fe2o3_amqp::connection::Error::RemoteClosed => {
                AmqpErrorKind::ConnectionClosedByRemote(Box::new(e.0)).into()
            }
            fe2o3_amqp::connection::Error::RemoteClosedWithError(error) => {
                AmqpErrorKind::AmqpDescribedError(error.into()).into()
            }

            _ => AmqpErrorKind::TransportImplementationError(Box::new(e.0)).into(),
        }
    }
}
