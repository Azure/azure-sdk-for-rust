use fe2o3_amqp::link::DetachError;

use crate::{amqp::amqp_receiver::AmqpReceiver, core::TransportReceiver};

pub struct ServiceBusReceiver {
    pub(crate) inner: AmqpReceiver,
    pub(crate) entity_path: String,
    pub(crate) identifier: String,
}

impl ServiceBusReceiver {
    pub async fn dispose(mut self) -> Result<(), DetachError> {
        self.inner.close().await
    }
}
