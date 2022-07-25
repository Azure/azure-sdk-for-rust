use crate::prelude::*;
use azure_core::error::{ErrorKind, ResultExt};
use serde::Deserialize;

operation! {
    BackupSecret,
    client: SecretClient,
}

impl BackupSecretBuilder {
    pub fn into_future(mut self) -> BackupSecret {
        Box::pin(async move {
            let mut uri = self.client.client.vault_url.clone();
            uri.set_path(&format!("secrets/{}/backup", self.client.name));
            uri.set_query(Some(API_VERSION_PARAM));

            let response_body = self
                .client
                .client
                .request(reqwest::Method::POST, uri.to_string(), None)
                .await?;

            serde_json::from_str::<BackupSecretResponse>(&response_body).with_context(
                ErrorKind::DataConversion,
                || {
                    format!(
                        "failed to parse BackupSecretResponse. secret_name: {}",
                        self.client.name
                    )
                },
            )
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct BackupSecretResponse {
    pub value: String,
}
