use crate::prelude::*;
use azure_core::{headers::Headers, Method};

operation! {
    DeleteCertificate,
    client: CertificateClient,
    name: String,
}

impl DeleteCertificateBuilder {
    pub fn into_future(self) -> DeleteCertificate {
        Box::pin(async move {
            let mut uri = self.client.keyvault_client.vault_url.clone();
            uri.set_path(&format!("certificates/{}", self.name));

            let headers = Headers::new();
            let mut request = KeyvaultClient::finalize_request(uri, Method::Delete, headers, None);

            self.client
                .keyvault_client
                .send(&self.context, &mut request)
                .await?
                .json()
                .await
        })
    }
}

type DeleteCertificateResponse = KeyVaultGetCertificateResponse;
