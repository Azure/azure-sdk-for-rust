#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum TransportProducerFeatures {
    None = 0u8,
    IdempotentPublishing = 1,
}
