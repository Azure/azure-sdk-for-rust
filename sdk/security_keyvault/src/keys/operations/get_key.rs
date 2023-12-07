use crate::prelude::*;
use azure_core::{headers::Headers, Method};

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
            let mut request = KeyvaultClient::finalize_request(uri, Method::Get, headers, None);

            self.client
                .keyvault_client
                .send(&self.context, &mut request)
                .await?
                .json()
                .await
        })
    }
}

type GetKeyResponse = KeyVaultKey;
