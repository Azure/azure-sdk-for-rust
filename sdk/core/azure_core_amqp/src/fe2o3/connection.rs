// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::error::{Fe2o3ConnectionError, Fe2o3ConnectionOpenError};
use crate::connection::{AmqpConnectionApis, AmqpConnectionOptions};
use crate::error::{AmqpConnectionError, AmqpErrorKind};
use crate::value::{AmqpOrderedMap, AmqpSymbol, AmqpValue};
use crate::AmqpError;
use azure_core::{Result, Url};
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

    fn connection_not_set() -> azure_core::Error {
        azure_core::Error::message(azure_core::error::ErrorKind::Amqp, "Connection is not set")
    }
    fn connection_already_set() -> azure_core::Error {
        azure_core::Error::message(
            azure_core::error::ErrorKind::Amqp,
            "Connection is already set",
        )
    }
}

impl Drop for Fe2o3AmqpConnection {
    fn drop(&mut self) {
        debug!("Dropping Fe2o3AmqpConnection.");
    }
}

impl AmqpConnectionApis for Fe2o3AmqpConnection {
    async fn open(
        &self,
        id: String,
        url: Url,
        options: Option<AmqpConnectionOptions>,
    ) -> Result<()> {
        {
            // All AMQP clients have a similar set of options.
            let mut builder = fe2o3_amqp::Connection::builder()
                .sasl_profile(fe2o3_amqp::sasl_profile::SaslProfile::Anonymous)
                .alt_tls_establishment(true)
                .container_id(id)
                .max_frame_size(65536);

            if let Some(options) = options {
                if let Some(frame_size) = options.max_frame_size {
                    builder = builder.max_frame_size(frame_size);
                }

                if let Some(channel_max) = options.channel_max {
                    builder = builder.channel_max(channel_max);
                }
                if let Some(idle_timeout) = options.idle_timeout {
                    builder = builder.idle_time_out(idle_timeout.whole_milliseconds() as u32);
                }
                if let Some(outgoing_locales) = options.outgoing_locales.as_ref() {
                    for locale in outgoing_locales {
                        builder = builder.add_outgoing_locales(locale.as_str());
                    }
                }
                if let Some(incoming_locales) = options.incoming_locales {
                    for locale in incoming_locales {
                        builder = builder.add_incoming_locales(locale.as_str());
                    }
                }
                if let Some(offered_capabilities) = options.offered_capabilities.as_ref() {
                    for capability in offered_capabilities {
                        let capability: fe2o3_amqp_types::primitives::Symbol =
                            capability.clone().into();
                        builder = builder.add_offered_capabilities(capability);
                    }
                }
                if let Some(desired_capabilities) = options.desired_capabilities.as_ref() {
                    for capability in desired_capabilities {
                        let capability: fe2o3_amqp_types::primitives::Symbol =
                            capability.clone().into();
                        builder = builder.add_desired_capabilities(capability);
                    }
                }
                if let Some(properties) = options.properties.as_ref() {
                    let mut fields = fe2o3_amqp::types::definitions::Fields::new();
                    for property in properties.iter() {
                        let k = fe2o3_amqp_types::primitives::Symbol::from(property.0);
                        let v = fe2o3_amqp_types::primitives::Value::from(property.1);

                        fields.insert(k, v);
                    }
                    builder = builder.properties(fields);
                }
                if let Some(buffer_size) = options.buffer_size {
                    builder = builder.buffer_size(buffer_size);
                }
            }
            self.connection
                .set(Mutex::new(builder.open(url).await.map_err(|e| {
                    azure_core::Error::from(Fe2o3ConnectionOpenError(e))
                })?))
                .map_err(|_| Self::connection_already_set())?;
            Ok(())
        }
    }

    async fn close(&self) -> Result<()> {
        let mut connection = self
            .connection
            .get()
            .ok_or::<azure_core::Error>(Self::connection_not_set())?
            .lock()
            .await;
        connection
            .borrow_mut()
            .close()
            .await
            .map_err(|e| azure_core::Error::from(Fe2o3ConnectionError(e)))?;
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
            .ok_or(Self::connection_not_set())?
            .lock()
            .await;
        let res = connection
            .borrow_mut()
            .close_with_error(fe2o3_amqp::types::definitions::Error::new(
                fe2o3_amqp::types::definitions::ErrorCondition::Custom(
                    fe2o3_amqp_types::primitives::Symbol::from(condition),
                ),
                description,
                info.map(|i| i.into()),
            ))
            .await
            .map_err(|e| azure_core::Error::from(Fe2o3ConnectionError(e)));
        // If we're closing with an error, then we might get the transport error back before we get the error back.
        // that's ok.
        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                match e.kind() {
                    azure_core::error::ErrorKind::Io => {
                        warn!("I/O closing connection, ignored: {:?}", e);
                        Ok(())
                    }
                    _ => Err(e),
                }
                //                warn!("Error detaching receiver: {:?}", e);
                //                Err(e)
            }
        }
    }
}

impl From<Fe2o3ConnectionOpenError> for azure_core::Error {
    fn from(e: Fe2o3ConnectionOpenError) -> Self {
        match e.0 {
            fe2o3_amqp::connection::OpenError::Io(e) => azure_core::Error::from(e),
            fe2o3_amqp::connection::OpenError::UrlError(parse_error) => {
                azure_core::Error::from(parse_error)
            }
            fe2o3_amqp::connection::OpenError::RemoteClosed => {
                AmqpError::from(AmqpErrorKind::ClosedByRemote(None)).into()
            }
            fe2o3_amqp::connection::OpenError::RemoteClosedWithError(error) => {
                AmqpError::from(AmqpErrorKind::ClosedByRemote(Some(error.into()))).into()
            }
            fe2o3_amqp::connection::OpenError::TransportError(error) => match error {
                fe2o3_amqp::transport::Error::Io(e) => azure_core::Error::from(e),
                fe2o3_amqp::transport::Error::IdleTimeoutElapsed => {
                    AmqpConnectionError::IdleTimeoutElapsed.into()
                }
                fe2o3_amqp::transport::Error::DecodeError(s) => {
                    AmqpConnectionError::DecodeError(s).into()
                }
                fe2o3_amqp::transport::Error::NotImplemented(o) => {
                    AmqpConnectionError::NotImplemented(o).into()
                }
                fe2o3_amqp::transport::Error::FramingError => {
                    AmqpConnectionError::FramingError.into()
                }
            },
            _ => AmqpError::from(AmqpErrorKind::from(AmqpConnectionError::from(e.0))).into(),
        }
    }
}

impl From<fe2o3_amqp::connection::OpenError> for AmqpConnectionError {
    fn from(e: fe2o3_amqp::connection::OpenError) -> Self {
        match e {
            fe2o3_amqp::connection::OpenError::Io(e) => panic!(
                "Io error: {:?} cannot be directly mapped to AmqpConnectionError.",
                e
            ),
            fe2o3_amqp::connection::OpenError::UrlError(parse_error) => panic!(
                "Url error: {:?} cannot be directly mapped to AmqpConnectionError.",
                parse_error
            ),
            fe2o3_amqp::connection::OpenError::TransportError(error) => panic!(
                "Transport error: {:?} cannot be directly mapped to AmqpConnectionError.",
                error
            ),
            fe2o3_amqp::connection::OpenError::RemoteClosed => {
                panic!("RemoteClosed cannot be directly mapped to AmqpConnectionError.")
            }
            fe2o3_amqp::connection::OpenError::RemoteClosedWithError(_) => {
                panic!("RemoteClosedWithError cannot be directly mapped to AmqpConnectionError.")
            }
            fe2o3_amqp::connection::OpenError::InvalidDomain => Self::InvalidDomain,
            fe2o3_amqp::connection::OpenError::TlsConnectorNotFound => Self::TlsConnectorNotFound,
            fe2o3_amqp::connection::OpenError::InvalidScheme => Self::InvalidScheme,
            fe2o3_amqp::connection::OpenError::ProtocolHeaderMismatch(_) => {
                Self::ProtocolHeaderMismatch(Box::new(e))
            }
            fe2o3_amqp::connection::OpenError::SaslError {
                code: _,
                additional_data: _,
            } => Self::SaslError(Box::new(e)),
            fe2o3_amqp::connection::OpenError::IllegalState => Self::IllegalState,
            fe2o3_amqp::connection::OpenError::NotImplemented(s) => Self::NotImplemented(s),
            fe2o3_amqp::connection::OpenError::DecodeError(d) => Self::DecodeError(d),
        }
    }
}

impl From<Fe2o3ConnectionError> for azure_core::Error {
    fn from(e: Fe2o3ConnectionError) -> Self {
        match e.0 {
            fe2o3_amqp::connection::Error::TransportError(error) => match error {
                fe2o3_amqp::transport::Error::Io(e) => azure_core::Error::from(e),
                fe2o3_amqp::transport::Error::DecodeError(s) => {
                    AmqpConnectionError::DecodeError(s).into()
                }
                fe2o3_amqp::transport::Error::NotImplemented(o) => {
                    AmqpConnectionError::NotImplemented(o).into()
                }
                fe2o3_amqp::transport::Error::IdleTimeoutElapsed => {
                    AmqpConnectionError::IdleTimeoutElapsed.into()
                }
                fe2o3_amqp::transport::Error::FramingError => {
                    AmqpConnectionError::FramingError.into()
                }
            },
            fe2o3_amqp::connection::Error::RemoteClosed => {
                AmqpError::from(AmqpErrorKind::ClosedByRemote(None)).into()
            }
            fe2o3_amqp::connection::Error::RemoteClosedWithError(error) => {
                AmqpError::from(AmqpErrorKind::ClosedByRemote(Some(error.into()))).into()
            }

            _ => AmqpError::from(AmqpErrorKind::from(AmqpConnectionError::from(e.0))).into(),
        }
    }
}

impl From<fe2o3_amqp::connection::Error> for AmqpConnectionError {
    fn from(e: fe2o3_amqp::connection::Error) -> Self {
        match e {
            // Transport errors and RemoteClosed errors are expressed directly in the AmqpError.
            fe2o3_amqp::connection::Error::TransportError(e) => panic!(
                "Transport error: {:?} cannot be directly mapped to AmqpConnectionError.",
                e
            ),
            fe2o3_amqp::connection::Error::RemoteClosed => {
                panic!("RemoteClosed error cannot be directly mapped to AmqpConnectionError.")
            }
            fe2o3_amqp::connection::Error::RemoteClosedWithError(_) => panic!(
                "RemoteClosedWithError error cannot be directly mapped to AmqpConnectionError."
            ),
            fe2o3_amqp::connection::Error::IllegalState => Self::IllegalState,
            fe2o3_amqp::connection::Error::NotImplemented(s) => Self::NotImplemented(s),
            fe2o3_amqp::connection::Error::NotFound(s) => Self::NotFound(s),
            fe2o3_amqp::connection::Error::NotAllowed(s) => Self::NotAllowed(s),
            fe2o3_amqp::connection::Error::JoinError(join_error) => {
                Self::JoinError(Box::new(join_error))
            }
        }
    }
}
