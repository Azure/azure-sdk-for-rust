use crate::{primitives::sub_queue::SubQueue, ServiceBusReceiveMode};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServiceBusReceiverOptions {
    pub prefetch_count: u32,
    pub receive_mode: ServiceBusReceiveMode,
    pub identifier: Option<String>,
    pub sub_queue: SubQueue,
}
