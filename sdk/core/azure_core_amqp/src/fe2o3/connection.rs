// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    connection::{AmqpConnectionApis, AmqpConnectionOptions, AmqpTransport},
    error::{AmqpErrorKind, Result},
    fe2o3::{
        error::{Fe2o3ConnectionError, Fe2o3ConnectionOpenError, Fe2o3TransportError},
        transport::{Closed, Transport},
    },
    value::{AmqpOrderedMap, AmqpSymbol, AmqpValue},
    AmqpError,
};
use azure_core::http::Url;
use fe2o3_amqp::connection::ConnectionHandle;
#[cfg(feature = "fe2o3_amqp_rustls")]
use rustls_platform_verifier::ConfigVerifierExt;
use std::{
    borrow::BorrowMut,
    sync::{Arc, OnceLock},
};
use tokio::{net::TcpStream, sync::Mutex};
use tracing::{debug, warn};

#[derive(Debug, Default)]
pub(crate) struct Fe2o3AmqpConnection {
    opening: Mutex<()>,
    connection: OnceLock<Mutex<ConnectionHandle<()>>>,
    transport: OnceLock<Transport<TcpStream>>,
}

impl Fe2o3AmqpConnection {
    pub fn new() -> Self {
        Self {
            opening: Mutex::new(()),
            connection: OnceLock::new(),
            transport: OnceLock::new(),
        }
    }

    pub fn get(&self) -> &OnceLock<Mutex<ConnectionHandle<()>>> {
        &self.connection
    }

    pub fn closed(&self) -> Result<Arc<Closed>> {
        self.transport
            .get()
            .map(|transport| transport.closed.clone())
            .ok_or_else(Self::connection_not_set)
    }

    pub fn transport(&self) -> Result<Transport<TcpStream>> {
        self.transport
            .get()
            .cloned()
            .ok_or_else(Self::connection_not_set)
    }

    pub fn abort(&self) {
        if let Some(transport) = self.transport.get() {
            transport.abort();
        }
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
        self.abort();
        debug!("Dropping Fe2o3AmqpConnection.");
    }
}

/// Builds the TLS connector for AMQP framed directly on TCP.
///
/// The default connector of `fe2o3-amqp` fills its root store from
/// `webpki-roots`, a compiled-in copy of the Mozilla root set, and it ignores
/// the trust store of the operating system. This connector uses the platform
/// verifier instead, so the TCP transport trusts the same roots as the HTTP
/// stack of `azure_core`, and a broker behind a private or an enterprise
/// certificate authority keeps working.
#[cfg(feature = "fe2o3_amqp_rustls")]
fn platform_verifier_connector() -> Result<tokio_rustls::TlsConnector> {
    let config = rustls::ClientConfig::with_platform_verifier().map_err(|e| {
        AmqpError::with_message(format!("Could not build the AMQP TLS configuration: {e}"))
    })?;
    Ok(tokio_rustls::TlsConnector::from(std::sync::Arc::new(
        config,
    )))
}

#[async_trait::async_trait]
impl AmqpConnectionApis for Fe2o3AmqpConnection {
    async fn open(
        &self,
        id: String,
        url: Url,
        options: Option<AmqpConnectionOptions>,
    ) -> Result<()> {
        let _opening = self
            .opening
            .try_lock()
            .map_err(|_| AmqpError::with_message("Connection opening is already in progress"))?;
        if self.connection.get().is_some() {
            return Err(Self::connection_already_set());
        }
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
                    if let Some(custom_endpoint) = options.custom_endpoint.as_ref() {
                        endpoint = custom_endpoint.clone();
                        builder = builder.hostname(url.host_str());
                    }

                    // Use the operating system trust store for rustls. Other TLS stacks
                    // selected through `fe2o3-amqp` keep their default connector.
                    #[cfg(feature = "fe2o3_amqp_rustls")]
                    let mut builder = builder.rustls_connector(platform_verifier_connector()?);

                    builder = builder
                        .scheme(endpoint.scheme())
                        .hostname(if options.custom_endpoint.is_some() {
                            url.host_str()
                        } else {
                            endpoint.host_str()
                        })
                        .sasl_hostname(endpoint.host_str())
                        .domain(endpoint.domain());
                    if let Ok(profile) = fe2o3_amqp::sasl_profile::SaslProfile::try_from(&endpoint)
                    {
                        builder = builder.sasl_profile(profile);
                    }
                    let port = endpoint
                        .port()
                        .or_else(|| match endpoint.scheme() {
                            "amqp" => Some(5672),
                            "amqps" => Some(5671),
                            _ => None,
                        })
                        .ok_or_else(|| AmqpError::with_message("AMQP endpoint has no port"))?;
                    let host = endpoint
                        .host_str()
                        .ok_or_else(|| AmqpError::with_message("AMQP endpoint has no host"))?;
                    // Tokio resolves names off the operation's executor thread.
                    let socket = TcpStream::connect((host.trim_matches(['[', ']']), port))
                        .await
                        .map_err(azure_core::Error::from)?;
                    let (transport, stream) = Transport::new(socket);
                    self.transport
                        .set(transport)
                        .map_err(|_| Self::connection_already_set())?;
                    builder
                        .open_with_stream(stream)
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
    #[cfg(feature = "fe2o3_amqp_rustls")]
    use super::platform_verifier_connector;
    use super::{AmqpConnectionApis, Fe2o3AmqpConnection};
    use azure_core::http::Url;
    use std::{sync::Arc, time::Duration};
    use tokio::{net::TcpListener, time::timeout};

    #[cfg(feature = "fe2o3_amqp_rustls")]
    #[test]
    fn platform_verifier_connector_builds() {
        // `ClientConfig::builder()` panics when the process has no default
        // crypto provider, and the platform verifier reports an error when it
        // cannot read the trust store of the operating system. Both faults
        // would otherwise appear only when a connection opens.
        assert!(platform_verifier_connector().is_ok());
    }

    #[tokio::test]
    async fn concurrent_open_is_rejected_without_connecting() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = Url::parse(&format!("amqp://{}", listener.local_addr().unwrap())).unwrap();
        let connection = Arc::new(Fe2o3AmqpConnection::new());
        let first_connection = connection.clone();
        let first_url = url.clone();
        let first =
            tokio::spawn(
                async move { first_connection.open("first".into(), first_url, None).await },
            );
        let (_peer, _) = timeout(Duration::from_secs(2), listener.accept())
            .await
            .unwrap()
            .unwrap();
        let error = timeout(
            Duration::from_millis(100),
            connection.open("second".into(), url, None),
        )
        .await
        .expect("a concurrent open must not wait for the first handshake")
        .unwrap_err();
        assert!(error.to_string().contains("already in progress"));
        assert!(timeout(Duration::from_millis(20), listener.accept())
            .await
            .is_err());
        first.abort();
        assert!(first.await.unwrap_err().is_cancelled());
        assert!(connection.opening.try_lock().is_ok());
    }
}
