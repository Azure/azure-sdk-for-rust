// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
// cspell: words amqp sasl

use crate::connection::{AmqpConnectionApis, AmqpConnectionOptions};
use crate::value::{AmqpOrderedMap, AmqpSymbol, AmqpValue};

use async_std::sync::Mutex;
use azure_core::{Result, Url};
use fe2o3_amqp::connection::ConnectionHandle;
use std::{borrow::BorrowMut, sync::OnceLock};
use tracing::debug;

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
        id: impl Into<String>,
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

            if options.is_some() {
                let options = options.ok_or_else(|| {
                    azure_core::Error::new(
                        azure_core::error::ErrorKind::Other,
                        "Connection options are not set.",
                    )
                })?;
                if options.max_frame_size.is_some() {
                    builder = builder.max_frame_size(options.max_frame_size.unwrap());
                }
                if options.channel_max.is_some() {
                    builder = builder.channel_max(options.channel_max.unwrap());
                }
                if options.idle_timeout.is_some() {
                    builder = builder
                        .idle_time_out(options.idle_timeout.unwrap().whole_milliseconds() as u32);
                }
                if options.outgoing_locales.is_some() {
                    for locale in options.outgoing_locales.as_ref().unwrap() {
                        builder = builder.add_outgoing_locales(locale.as_str());
                    }
                }
                if options.incoming_locales.is_some() {
                    for locale in options.incoming_locales.as_ref().unwrap() {
                        builder = builder.add_incoming_locales(locale.as_str());
                    }
                }
                if options.offered_capabilities.is_some() {
                    for capability in options.offered_capabilities.unwrap() {
                        let capability: fe2o3_amqp_types::primitives::Symbol = capability.into();
                        builder = builder.add_offered_capabilities(capability);
                    }
                }
                if options.desired_capabilities.is_some() {
                    for capability in options.desired_capabilities.unwrap() {
                        let capability: fe2o3_amqp_types::primitives::Symbol = capability.into();
                        builder = builder.add_desired_capabilities(capability);
                    }
                }
                if options.properties.is_some() {
                    let mut fields = fe2o3_amqp::types::definitions::Fields::new();
                    for property in options.properties.unwrap().iter() {
                        debug!("Property: {:?}, Value: {:?}", property.0, property.1);
                        let k: fe2o3_amqp_types::primitives::Symbol = property.0.into();
                        let v: fe2o3_amqp_types::primitives::Value = property.1.into();
                        debug!("Property2: {:?}, Value: {:?}", k, v);

                        fields.insert(k, v);
                    }
                    builder = builder.properties(fields);
                }
                if options.buffer_size.is_some() {
                    builder = builder.buffer_size(options.buffer_size.unwrap());
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
        condition: impl Into<AmqpSymbol>,
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
        connection
            .borrow_mut()
            .close_with_error(fe2o3_amqp::types::definitions::Error::new(
                fe2o3_amqp::types::definitions::ErrorCondition::Custom(
                    fe2o3_amqp_types::primitives::Symbol::from(condition.into()),
                ),
                description,
                info.map(|i| i.into()),
            ))
            .await
            .map_err(AmqpConnection::from)?;
        Ok(())
    }
}
