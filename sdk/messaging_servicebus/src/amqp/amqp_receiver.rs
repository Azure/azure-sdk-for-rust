use async_trait::async_trait;

use crate::core::TransportReceiver;

pub struct AmqpReceiver {}

#[async_trait]
impl TransportReceiver for AmqpReceiver {}
