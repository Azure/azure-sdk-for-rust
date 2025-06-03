// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::{
    connection::AmqpConnection,
    value::{AmqpOrderedMap, AmqpSymbol, AmqpValue},
};
use azure_core::error::Result;
use std::fmt::Debug;

#[cfg(all(feature = "fe2o3_amqp", not(target_arch = "wasm32")))]
type SessionImplementation = super::fe2o3::session::Fe2o3AmqpSession;

#[cfg(any(not(feature = "fe2o3_amqp"), target_arch = "wasm32"))]
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

    pub fn offered_capabilities(&self) -> Option<&[AmqpSymbol]> {
        self.offered_capabilities.as_deref()
    }

    pub fn desired_capabilities(&self) -> Option<&[AmqpSymbol]> {
        self.desired_capabilities.as_deref()
    }

    pub fn properties(&self) -> Option<&AmqpOrderedMap<AmqpSymbol, AmqpValue>> {
        self.properties.as_ref()
    }

    pub fn buffer_size(&self) -> Option<usize> {
        self.buffer_size
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
pub trait AmqpSessionApis {
    async fn begin(
        &self,
        connection: &AmqpConnection,
        options: Option<AmqpSessionOptions>,
    ) -> Result<()>;
    async fn end(&self) -> Result<()>;
}

#[derive(Clone, Default)]
pub struct AmqpSession {
    pub(crate) implementation: SessionImplementation,
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
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
