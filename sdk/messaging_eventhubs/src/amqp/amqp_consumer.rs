use fe2o3_amqp::{session::SessionHandle, Receiver};

use crate::{core::TransportConsumer, event_hubs_retry_policy::EventHubsRetryPolicy, util::sharable::Sharable};

use super::amqp_client::AmqpClient;

pub struct AmqpConsumer<RP> {
    pub(crate) session_handle: SessionHandle<()>,
    pub(crate) session_identifier: u32,
    pub(crate) receiver: Receiver,
    pub(crate) link_identifier: u32,
    pub(crate) invalidate_consumer_when_partition_stolen: bool,
    pub(crate) retry_policy: RP,
}

pub struct RecoverableAmqpConsumer<'a, RP> {
    consumer: &'a mut AmqpConsumer<RP>,
    client: &'a Sharable<AmqpClient>,
}
