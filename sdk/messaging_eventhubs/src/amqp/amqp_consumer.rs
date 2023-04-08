use fe2o3_amqp::{session::SessionHandle, Receiver};

use crate::{core::transport_consumer::TransportConsumer, event_hubs_retry_policy::EventHubsRetryPolicy};

pub struct AmqpConsumer<RP> {
    pub(crate) session_handle: SessionHandle<()>,
    pub(crate) session_identifier: u32,
    pub(crate) receiver: Receiver,
    pub(crate) link_identifier: u32,
    pub(crate) invalidate_consumer_when_partition_stolen: bool,
    pub(crate) retry_policy: RP,
}

impl<RP> TransportConsumer for AmqpConsumer<RP>
where
    RP: EventHubsRetryPolicy + Send,
{

}
