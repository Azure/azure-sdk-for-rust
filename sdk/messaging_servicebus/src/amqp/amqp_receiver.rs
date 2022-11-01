use async_trait::async_trait;

use crate::core::TransportReceiver;

use super::LINK_IDENTIFIER;

pub struct AmqpReceiver {
    pub identifier: u32,
    pub receiver: fe2o3_amqp::Receiver,
}

// impl AmqpReceiver {
//     pub fn new(receiver: fe2o3_amqp::Receiver) -> Self {
//         Self {
//             identifier: LINK_IDENTIFIER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
//             receiver,
//         }
//     }
// }

#[async_trait]
impl TransportReceiver for AmqpReceiver {}
