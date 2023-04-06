use std::sync::{Arc, atomic::AtomicBool};

use tokio::sync::Mutex;

use crate::amqp::amqp_client::AmqpClient;

pub struct EventHubConnection {
    fully_qualified_namespace: String,
    event_hub_name: Arc<String>,
    is_closed: Arc<AtomicBool>,
    inner: EventHubConnectionInner,
}

pub enum EventHubConnectionInner {
    Owned(AmqpClient),
    Shared(Arc<Mutex<AmqpClient>>),
}
