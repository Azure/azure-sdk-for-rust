use async_trait::async_trait;

#[async_trait]
pub trait TransportProducer {
    type DisposeError: std::error::Error;

    async fn dispose(self) -> Result<(), Self::DisposeError>;
}
