// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
// cspell: words amqp sasl

use crate::connection::{AmqpConnectionApis, AmqpConnectionOptions};
use crate::value::{AmqpOrderedMap, AmqpSymbol, AmqpValue};

use async_std::sync::Mutex;
use azure_core::{Result, Url};
use fe2o3_amqp::connection::ConnectionHandle;
use std::{borrow::BorrowMut, sync::OnceLock};
use tracing::{debug, warn};

use super::error::{AmqpConnection, AmqpOpen};

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
                .set(Mutex::new(builder.open(url).await.map_err(AmqpOpen::from)?))
                .map_err(|_| {
                    azure_core::Error::new(
                        azure_core::error::ErrorKind::Other,
                        "Connection already set.",
                    )
                })?;
            Ok(())
        }
    }

    async fn close(&self) -> Result<()> {
        let mut connection = self
            .connection
            .get()
            .ok_or_else(|| {
                azure_core::Error::message(
                    azure_core::error::ErrorKind::Other,
                    "Connection is not set",
                )
            })?
            .lock()
            .await;
        connection
            .borrow_mut()
            .close()
            .await
            .map_err(AmqpConnection::from)?;
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
            .ok_or_else(|| {
                azure_core::Error::message(
                    azure_core::error::ErrorKind::Other,
                    "Connection is not set",
                )
            })?
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
            .map_err(AmqpConnection::from);
        // If we're closing with an error, then we might get the transport error back before we get the error back.
        // that's ok.
        match res {
            Ok(_) => Ok(()),
            Err(e) => match e.0 {
                fe2o3_amqp::connection::Error::TransportError(e) => {
                    debug!(
                        "Transport error closing connection with error: {:?} - ignored",
                        e
                    );
                    Ok(())
                }
                _ => {
                    warn!("Error detaching receiver: {:?}", e);
                    Err(e.into())
                }
            },
        }
    }
}
