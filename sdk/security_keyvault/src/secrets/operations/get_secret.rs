use crate::prelude::*;
use azure_core::error::{ErrorKind, ResultExt};

operation! {
    GetSecret,
    client: SecretClient,
    name: String,
    ?version: String
}

impl GetSecretBuilder {
    pub fn into_future(mut self) -> GetSecret {
        Box::pin(async move {
            let mut uri = self.client.client.vault_url.clone();
            let version = self.version.unwrap_or_default();
            uri.set_path(&format!("secrets/{}/{}", self.name, version));
            uri.set_query(Some(API_VERSION_PARAM));

            let response_body = self
                .client
                .client
                .request(reqwest::Method::GET, uri.to_string(), None)
                .await?;
            let response = serde_json::from_str::<KeyVaultGetSecretResponse>(&response_body)
            .with_context(ErrorKind::DataConversion, || {
                format!(
                    "failed to parse KeyVaultGetSecretResponse. secret_name: {} secret_version_name: {version} response_body: {response_body}", self.name
                )
            })?;
            Ok(KeyVaultSecret {
                expires_on: response.attributes.exp,
                enabled: response.attributes.enabled,
                value: response.value,
                created_on: response.attributes.created,
                updated_on: response.attributes.updated,
                id: response.id,
            })
        })
    }
}

type GetSecretResponse = KeyVaultSecret;
