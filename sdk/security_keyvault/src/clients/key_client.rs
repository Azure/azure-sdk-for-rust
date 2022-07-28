use crate::prelude::*;
use azure_core::auth::TokenCredential;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct KeyClient {
    pub(crate) client: KeyvaultClient,
}

impl KeyClient {
    pub fn new(
        vault_url: &str,
        token_credential: Arc<dyn TokenCredential>,
    ) -> azure_core::Result<Self> {
        let client = KeyvaultClient::new(vault_url, token_credential)?;
        Ok(Self { client })
    }

    pub(crate) fn new_with_client(client: KeyvaultClient) -> Self {
        Self { client }
    }

    /// Gets the public part of a stored key.
    /// The get key operation is applicable to all key types.
    /// If the requested key is symmetric, then no key material is released in the response.
    /// This operation requires the keys/get permission.
    ///
    /// GET {vaultBaseUrl}/keys/{key-name}/{key-version}?api-version=7.1
    pub fn get<N>(&self, name: N) -> GetKeyBuilder
    where
        N: Into<String>,
    {
        GetKeyBuilder::new(self.clone(), name.into())
    }

    /// Creates a signature from a digest using the specified key.
    ///
    /// The SIGN operation is applicable to asymmetric and symmetric keys stored
    /// in Azure Key Vault since this operation uses the private portion of the
    /// key.
    ///
    /// This operation requires the keys/sign permission.
    pub fn sign<N, D>(&self, name: N, algorithm: SignatureAlgorithm, digest: D) -> SignBuilder
    where
        N: Into<String>,
        D: Into<String>,
    {
        SignBuilder::new(self.clone(), name.into(), algorithm, digest.into())
    }

    /// Decrypt a single block of encrypted data.
    ///
    /// The DECRYPT operation decrypts a well-formed block of ciphertext using
    /// the target encryption key and specified algorithm.
    //
    /// This operation is the reverse of the ENCRYPT operation; only a single
    /// block of data may be decrypted, the size of this block is dependent on
    /// the target key and the algorithm to be used.
    ///
    /// The DECRYPT operation applies to asymmetric and symmetric keys stored in
    /// Vault or HSM since it uses the private portion of the key. This
    /// operation requires the keys/decrypt permission.
    pub fn decrypt<N>(&self, name: N, decrypt_parameters: DecryptParameters) -> DecryptBuilder
    where
        N: Into<String>,
    {
        DecryptBuilder::new(self.clone(), name.into(), decrypt_parameters)
    }
}
