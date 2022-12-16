#[async_trait::async_trait]
pub trait RecoverableTransport {
    type RecoverError: Send;

    async fn recover(&mut self) -> Result<(), Self::RecoverError>;
}
