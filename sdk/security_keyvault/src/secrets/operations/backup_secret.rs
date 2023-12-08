use crate::prelude::*;
use azure_core::{headers::Headers, Method};
use serde::Deserialize;

operation! {
    BackupSecret,
    client: SecretClient,
    name: String,
}

impl BackupSecretBuilder {
    pub fn into_future(self) -> BackupSecret {
        Box::pin(async move {
            let mut uri = self.client.keyvault_client.vault_url.clone();
            uri.set_path(&format!("secrets/{}/backup", self.name));

            let headers = Headers::new();
            let mut request = KeyvaultClient::finalize_request(uri, Method::Post, headers, None);

            self.client
                .keyvault_client
                .send(&self.context, &mut request)
                .await?
                .json()
                .await
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct BackupSecretResponse {
    pub value: String,
}
