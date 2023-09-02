use crate::prelude::*;
use azure_core::{headers::Headers, CollectedResponse, Method};

operation! {
    CertificateBackup,
    client: CertificateClient,
    name: String,
}

impl CertificateBackupBuilder {
    pub fn into_future(self) -> CertificateBackup {
        Box::pin(async move {
            let mut uri = self.client.keyvault_client.vault_url.clone();
            uri.set_path(&format!("certificates/{}/backup", self.name));

            let headers = Headers::new();
            let mut request =
                self.client
                    .keyvault_client
                    .finalize_request(uri, Method::Post, headers, None)?;

            let response = self
                .client
                .keyvault_client
                .send(&self.context, &mut request)
                .await?;

            let response = CollectedResponse::from_response(response).await?;
            let body = response.body();

            let backup_blob = serde_json::from_slice(body)?;
            Ok(backup_blob)
        })
    }
}
