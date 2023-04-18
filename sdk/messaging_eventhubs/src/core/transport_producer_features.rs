#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum TransportProducerFeatures {
    None = 0u8,
    // TODO: is this needed given that AMQP protocol should already take care of duplicated message upon recovery?
    // IdempotentPublishing = 1,
}
