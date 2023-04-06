use std::sync::{Arc, atomic::AtomicBool};

use tokio::sync::Mutex;

use crate::{amqp::amqp_client::AmqpClient, authorization::event_hub_token_credential::EventHubTokenCredential, event_hubs_connection_option::EventHubConnectionOptions};

pub struct EventHubConnection {
    fully_qualified_namespace: String,
    event_hub_name: Arc<String>,
    is_closed: Arc<AtomicBool>,
    inner: EventHubConnectionInner,
}

pub(crate) enum EventHubConnectionInner {
    Owned(AmqpClient),
    Shared(Arc<Mutex<AmqpClient>>),
}

impl EventHubConnection {
    pub fn fully_qualified_namespace(&self) -> &str {
        &self.fully_qualified_namespace
    }

    pub fn event_hub_name(&self) -> &str {
        &self.event_hub_name
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub async fn new(
        fully_qualified_namespace: String,
        event_hub_name: String,
        options: EventHubConnectionOptions,
    ) -> Result<Self, azure_core::Error> {
        todo!()
    }

    pub async fn new_with_credential(
        fully_qualifed_namespace: String,
        event_hub_name: String,
        credential: impl Into<EventHubTokenCredential>,
        options: EventHubConnectionOptions,
    ) -> Result<Self, azure_core::Error> {
        todo!()
    }
}
