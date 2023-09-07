use async_trait::async_trait;

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub(crate) trait RecoverableTransport {
    type RecoverError: Send;

    async fn recover(&mut self) -> Result<(), Self::RecoverError>;
}
