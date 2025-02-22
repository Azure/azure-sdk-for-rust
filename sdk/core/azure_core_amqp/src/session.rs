// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::{
    connection::AmqpConnection,
    value::{AmqpOrderedMap, AmqpSymbol, AmqpValue},
};
use azure_core::error::Result;
use std::fmt::Debug;

#[cfg(all(feature = "fe2o3-amqp", not(target_arch = "wasm32")))]
type SessionImplementation = super::fe2o3::session::Fe2o3AmqpSession;

#[cfg(any(not(feature = "fe2o3-amqp"), target_arch = "wasm32"))]
type SessionImplementation = super::noop::NoopAmqpSession;

#[derive(Debug, Default, Clone)]
pub struct AmqpSessionOptions {
    pub next_outgoing_id: Option<u32>,
    pub incoming_window: Option<u32>,
    pub outgoing_window: Option<u32>,
    pub handle_max: Option<u32>,
    pub offered_capabilities: Option<Vec<AmqpSymbol>>,
    pub desired_capabilities: Option<Vec<AmqpSymbol>>,
    pub properties: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,
    pub buffer_size: Option<usize>,
}

impl AmqpSessionOptions {
    pub fn next_outgoing_id(&self) -> Option<u32> {
        self.next_outgoing_id
    }

    pub fn incoming_window(&self) -> Option<u32> {
        self.incoming_window
    }

    pub fn outgoing_window(&self) -> Option<u32> {
        self.outgoing_window
    }

    pub fn handle_max(&self) -> Option<u32> {
        self.handle_max
    }

    pub fn offered_capabilities(&self) -> Option<&Vec<AmqpSymbol>> {
        self.offered_capabilities.as_ref()
    }

    pub fn desired_capabilities(&self) -> Option<&Vec<AmqpSymbol>> {
        self.desired_capabilities.as_ref()
    }

    pub fn properties(&self) -> Option<&AmqpOrderedMap<AmqpSymbol, AmqpValue>> {
        self.properties.as_ref()
    }

    pub fn buffer_size(&self) -> Option<usize> {
        self.buffer_size
    }
}

#[allow(unused_variables)]
pub trait AmqpSessionApis {
    fn begin(
        &self,
        connection: &AmqpConnection,
        options: Option<AmqpSessionOptions>,
    ) -> impl std::future::Future<Output = Result<()>>;
    fn end(&self) -> impl std::future::Future<Output = Result<()>>;
}

#[derive(Clone, Default)]
pub struct AmqpSession {
    pub(crate) implementation: SessionImplementation,
}

impl AmqpSessionApis for AmqpSession {
    async fn begin(
        &self,
        connection: &AmqpConnection,
        options: Option<AmqpSessionOptions>,
    ) -> Result<()> {
        self.implementation.begin(connection, options).await
    }

    async fn end(&self) -> Result<()> {
        self.implementation.end().await
    }
}

impl AmqpSession {
    pub fn new() -> Self {
        Self {
            implementation: SessionImplementation::new(),
        }
    }
}

pub(crate) mod error {
    use crate::{
        error::{AmqpDescribedError, AmqpErrorKind},
        AmqpError,
    };

    pub enum AmqpSessionError {
        SessionNotAttached,
        SessionNotSet,
        SessionAlreadyAttached,
        CouldNotSetSession,
        SessionImplementationError(Box<dyn std::error::Error + Send + Sync>),

        /// Remote session ended
        RemoteEnded,

        /// Remote session ended with error
        RemoteEndedWithError(AmqpDescribedError),

        /// Channel max reached
        LocalChannelMaxReached,
    }

    impl std::fmt::Display for AmqpSessionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                AmqpSessionError::SessionImplementationError(e) => {
                    write!(f, "Session Implementation Error: {}", e.as_ref())
                }
                AmqpSessionError::CouldNotSetSession => write!(f, "Could not set session"),
                AmqpSessionError::SessionNotAttached => write!(f, "Session not attached"),
                AmqpSessionError::SessionNotSet => write!(f, "Session not set"),
                AmqpSessionError::SessionAlreadyAttached => write!(f, "Session already attached"),
                AmqpSessionError::RemoteEnded => write!(f, "Remote session ended"),
                AmqpSessionError::RemoteEndedWithError(e) => {
                    write!(f, "Remote ended with error: {:?}", e)
                }
                AmqpSessionError::LocalChannelMaxReached => {
                    write!(f, "Local channel-max reached")
                }
            }
        }
    }

    impl std::error::Error for AmqpSessionError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                AmqpSessionError::SessionImplementationError(e) => Some(e.as_ref()),
                _ => None,
            }
        }
    }
    impl std::fmt::Debug for AmqpSessionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AmqpSessionError: {}", self)
        }
    }
    impl From<AmqpSessionError> for azure_core::error::Error {
        fn from(e: AmqpSessionError) -> Self {
            AmqpError::new(e.into()).into()
        }
    }

    impl From<AmqpSessionError> for AmqpErrorKind {
        fn from(e: AmqpSessionError) -> Self {
            AmqpErrorKind::SessionError(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amqp_session_options_builder() {
        let session_options = AmqpSessionOptions {
            next_outgoing_id: Some(1),
            incoming_window: Some(1),
            outgoing_window: Some(1),
            handle_max: Some(1),
            offered_capabilities: Some(vec!["capability".into()]),
            desired_capabilities: Some(vec!["capability".into()]),
            properties: Some(
                vec![("key", "value")]
                    .into_iter()
                    .map(|(k, v)| (AmqpSymbol::from(k), AmqpValue::from(v)))
                    .collect(),
            ),
            buffer_size: Some(1024),
        };
        assert_eq!(session_options.next_outgoing_id, Some(1));
        assert_eq!(session_options.incoming_window, Some(1));
        assert_eq!(session_options.outgoing_window, Some(1));
        assert_eq!(session_options.handle_max, Some(1));
        assert_eq!(
            session_options.offered_capabilities,
            Some(vec!["capability".into()])
        );
        assert_eq!(
            session_options.desired_capabilities,
            Some(vec!["capability".into()])
        );
        assert!(session_options.properties.is_some());
        let properties = session_options.properties.clone().unwrap();
        assert!(properties.contains_key("key"));
        assert_eq!(
            *properties.get(&AmqpSymbol::from("key")).unwrap(),
            AmqpValue::String("value".to_string())
        );

        assert_eq!(session_options.buffer_size, Some(1024));
    }
}
