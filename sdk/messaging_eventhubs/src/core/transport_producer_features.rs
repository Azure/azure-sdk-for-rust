#[repr(u8)]
pub(crate) enum TransportProducerFeatures {
    None = 0u8,
    IdempotentPublishing = 1,
}
