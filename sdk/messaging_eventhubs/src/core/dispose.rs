#[async_trait::async_trait]
pub trait Dispose {
    type DisposeError: Send;

    async fn dispose(self) -> Result<(), Self::DisposeError>;
}
