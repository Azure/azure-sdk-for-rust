use crate::prelude::*;
use azure_core::auth::TokenCredential;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ManagedHsmClient {
    pub(crate) keyvault_client: KeyvaultClient,
}

impl ManagedHsmClient {
    pub fn new(
        vault_url: &str,
        token_credential: Arc<dyn TokenCredential>,
    ) -> azure_core::Result<Self> {
        let keyvault_client = KeyvaultClient::new(vault_url, token_credential)?;
        Ok(Self::new_with_client(keyvault_client))
    }

    pub(crate) fn new_with_client(keyvault_client: KeyvaultClient) -> Self {
        Self { keyvault_client }
    }

    /// Get the requested number of bytes containing random values from a managed HSM.
    ///
    /// The length of the random values is limited to 128 bytes. And this operation is
    /// valid only for clients built using HSM urls.
    /// This operation requires the rng permission to HSM.
    ///
    /// POST {managedHsmBaseUrl}/rng?api-version=7.4
    pub fn get_random_bytes<N>(&self, hsm_name: N, count: u32) -> GetRandomBytesBuilder
    where
        N: Into<String>,
    {
        GetRandomBytesBuilder::new(self.clone(), hsm_name.into(), count)
    }
}
