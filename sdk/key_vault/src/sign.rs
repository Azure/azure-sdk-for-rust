use anyhow::Context;
use azure_core::TokenCredential;
use getset::Getters;
use serde::Deserialize;
use serde_json::{Map, Value};
use url::Url;

use crate::client::API_VERSION;
use crate::key::JsonWebKeySignatureAlgorithm;
use crate::{KeyVaultClient, KeyVaultError};

#[derive(Debug, Deserialize, Getters)]
#[getset(get = "pub")]
pub struct KeyOperationResult {
    kid: String,
    value: String,
}

impl<'a, T: TokenCredential> KeyVaultClient<'a, T> {
    /// Creates a signature from a digest using the specified key.
    /// The SIGN operation is applicable to asymmetric and symmetric keys stored in Azure Key Vault since this operation uses the private portion of the key. This operation requires the keys/sign permission.
    pub async fn sign(
        &mut self,
        key_name: &str,
        key_version: &str,
        value: &str,
        alg: JsonWebKeySignatureAlgorithm,
    ) -> Result<KeyOperationResult, KeyVaultError> {
        // POST {vaultBaseUrl}/keys/{key-name}/{key-version}/sign?api-version=7.1
        let uri = Url::parse_with_params(
            &format!(
                "{}/keys/{}/{}/sign",
                self.keyvault_endpoint, key_name, key_version
            ),
            &[("api-version", API_VERSION)],
        )
        .unwrap();

        let mut request_body = Map::new();
        request_body.insert("alg".to_owned(), Value::String(alg.to_string()));
        request_body.insert("value".to_owned(), Value::String(value.to_owned()));

        let response = self
            .post_authed(
                uri.to_string(),
                Some(Value::Object(request_body).to_string()),
            )
            .await?;

        let result = serde_json::from_str::<KeyOperationResult>(&response).with_context(|| {
            format!(
                "Failed to parse response from Key Vault when signing digest with {}: {}",
                key_name,
                response.to_string()
            )
        })?;

        Ok(result)
    }
}
