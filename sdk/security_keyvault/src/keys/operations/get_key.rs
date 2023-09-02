use crate::prelude::*;
use azure_core::{headers::Headers, CollectedResponse, Method};

operation! {
    GetKey,
    client: KeyClient,
    name: String,
    ?version: String
}

impl GetKeyBuilder {
    pub fn into_future(self) -> GetKey {
        Box::pin(async move {
            let mut uri = self.client.keyvault_client.vault_url.clone();
            let version = self.version.unwrap_or_default();
            let path = format!("keys/{}/{}", self.name, version);
            uri.set_path(&path);

            let headers = Headers::new();
            let mut request =
                self.client
                    .keyvault_client
                    .finalize_request(uri, Method::Get, headers, None)?;

            let response = self
                .client
                .keyvault_client
                .send(&self.context, &mut request)
                .await?;

            let response = CollectedResponse::from_response(response).await?;
            let body = response.body();
            let response = serde_json::from_slice::<KeyVaultKey>(body)?;

            Ok(response)
        })
    }
}

type GetKeyResponse = KeyVaultKey;
