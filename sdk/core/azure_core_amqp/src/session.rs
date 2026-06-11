// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::{
    connection::AmqpConnection,
    value::{AmqpOrderedMap, AmqpSymbol, AmqpValue},
};
use crate::error::Result;
use std::fmt::Debug;

#[cfg(feature = "fe2o3_amqp")]
type SessionImplementation = super::fe2o3::session::Fe2o3AmqpSession;

#[cfg(not(feature = "fe2o3_amqp"))]
type SessionImplementation = super::noop::NoopAmqpSession;

/// Options for an AMQP Session.
#[derive(Debug, Default, Clone)]
pub struct AmqpSessionOptions {
    /// The next outgoing ID for the session.
    pub next_outgoing_id: Option<u32>,

    /// The incoming window for the session.
    pub incoming_window: Option<u32>,

    /// The outgoing window for the session.
    pub outgoing_window: Option<u32>,

    /// The maximum handle for the session.
    pub handle_max: Option<u32>,

    /// The offered capabilities for the session.
    pub offered_capabilities: Option<Vec<AmqpSymbol>>,

    /// The desired capabilities for the session.
    pub desired_capabilities: Option<Vec<AmqpSymbol>>,

    /// The properties for the session.
    pub properties: Option<AmqpOrderedMap<AmqpSymbol, AmqpValue>>,

    /// The buffer size for the session.
    pub buffer_size: Option<usize>,
}

impl AmqpSessionOptions {
    /// Session options that disable session-level flow control by maxing both
    /// the incoming and outgoing windows.
    ///
    /// Messaging crates (such as Event Hubs and Service Bus) rely on per-link
    /// credit for flow control and therefore want unbounded session windows.
    /// The generic [`Default`] implementation deliberately stays `None` for
    /// both windows so non-messaging consumers are unaffected.
    pub fn with_unbounded_windows() -> Self {
        Self {
            incoming_window: Some(u32::MAX),
            outgoing_window: Some(u32::MAX),
            ..Default::default()
        }
    }
}

/// A trait for AMQP Session operations.
#[async_trait::async_trait]
pub trait AmqpSessionApis {
    /// Begin the session.
    async fn begin(
        &self,
        connection: &AmqpConnection,
        options: Option<AmqpSessionOptions>,
    ) -> Result<()>;

    /// End the session.
    async fn end(&self) -> Result<()>;
}

/// An AMQP Session.
#[derive(Clone, Default)]
pub struct AmqpSession {
    pub(crate) implementation: SessionImplementation,
}

#[async_trait::async_trait]
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
    /// Create a new AMQP Session.
    pub fn new() -> Self {
        Self {
            implementation: SessionImplementation::new(),
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

    #[test]
    fn test_with_unbounded_windows() {
        let session_options = AmqpSessionOptions::with_unbounded_windows();
        assert_eq!(session_options.incoming_window, Some(u32::MAX));
        assert_eq!(session_options.outgoing_window, Some(u32::MAX));
        // Everything else stays at the generic `Default` (`None`).
        assert_eq!(session_options.next_outgoing_id, None);
        assert_eq!(session_options.handle_max, None);
    }
}
