// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    connection::{AmqpConnectionApis, AmqpConnectionOptions, AmqpTransport},
    error::{AmqpErrorKind, Result},
    fe2o3::error::{
        Fe2o3ConnectionError, Fe2o3ConnectionOpenError, Fe2o3TransportError, Fe2o3WebSocketError,
    },
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

// cspell:ignore servicebus

/// The well-known path that Service Bus and Event Hubs expose for the AMQP
/// WebSocket binding. Matches the suffix used by the other Azure SDKs.
const WEBSOCKET_PATH: &str = "/$servicebus/websocket/";

/// Builds the secure WebSocket (`wss://`) address used to tunnel AMQP for the
/// given connection target. The target is the AMQP service URL (or a custom
/// endpoint proxy). Its scheme and path are discarded: only the host and an
/// explicit port (if any) are carried over, since AMQP-over-WebSockets always
/// uses TLS and a fixed binding path. When no port is present the default
/// `wss` port (443) is used.
fn websocket_address(target: &Url) -> Result<String> {
    let host = target
        .host_str()
        .ok_or_else(|| AmqpError::with_message("AMQP connection URL is missing a host."))?;
    let authority = match target.port() {
        Some(port) => format!("{host}:{port}"),
        None => host.to_string(),
    };
    Ok(format!("wss://{authority}{WEBSOCKET_PATH}"))
}

#[async_trait::async_trait]
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

            let handle = match options.transport.unwrap_or_default() {
                AmqpTransport::Tcp => {
                    // `custom_endpoint` redirects the socket to a proxy while the
                    // AMQP `hostname` stays the real service host.
                    if let Some(custom_endpoint) = options.custom_endpoint {
                        endpoint = custom_endpoint;
                        builder = builder.hostname(url.host_str());
                    }
                    builder
                        .open(endpoint)
                        .await
                        .map_err(|e| AmqpError::from(Fe2o3ConnectionOpenError(e)))?
                }
                AmqpTransport::WebSocket => {
                    // Tunnel AMQP over a secure WebSocket (port 443) for networks
                    // that block the native AMQP ports. The socket connects to the
                    // websocket address (or the custom endpoint proxy, if set),
                    // while the AMQP `hostname` remains the real service host.
                    // `open_with_stream` does not derive the hostname from a URL,
                    // so it must be set explicitly.
                    let ws_target = options.custom_endpoint.as_ref().unwrap_or(&url);
                    let ws_address = websocket_address(ws_target)?;
                    debug!("Opening AMQP-over-WebSockets connection to {ws_address}.");
                    let ws_stream = fe2o3_amqp_ws::WebSocketStream::connect_tls_with_config(
                        &ws_address,
                        None,
                        false,
                        None,
                    )
                    .await
                    .map_err(|e| AmqpError::from(Fe2o3WebSocketError(e)))?;
                    builder
                        .hostname(url.host_str())
                        .open_with_stream(ws_stream)
                        .await
                        .map_err(|e| AmqpError::from(Fe2o3ConnectionOpenError(e)))?
                }
            };

            self.connection
                .set(Mutex::new(handle))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn websocket_address_uses_default_port_for_service_url() {
        // The Event Hubs connection URL has no explicit port; the wss default
        // (443) is implied and the binding path is appended.
        let url = Url::parse("amqps://my-namespace.servicebus.windows.net/my-eventhub").unwrap();
        assert_eq!(
            websocket_address(&url).unwrap(),
            "wss://my-namespace.servicebus.windows.net/$servicebus/websocket/"
        );
    }

    #[test]
    fn websocket_address_preserves_explicit_port() {
        // A custom endpoint (e.g. a local proxy) may carry an explicit port,
        // which must be preserved in the websocket address.
        let proxy = Url::parse("amqps://localhost:8081/").unwrap();
        assert_eq!(
            websocket_address(&proxy).unwrap(),
            "wss://localhost:8081/$servicebus/websocket/"
        );
    }
}
