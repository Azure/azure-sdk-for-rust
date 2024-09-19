use crate::prelude::*;
use azure_core::{headers::Headers, Method};

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
            let mut request = KeyvaultClient::finalize_request(uri, Method::POST, headers, None);

            self.client
                .keyvault_client
                .send(&self.context, &mut request)
                .await?
                .json()
                .await
        })
    }
}
