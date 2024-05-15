use crate::prelude::*;
use azure_core::{headers::Headers, Method};

operation! {
    GetCertificate,
    client: CertificateClient,
    name :String,
    ?version: String
}

impl GetCertificateBuilder {
    pub fn into_future(self) -> GetCertificate {
        Box::pin(async move {
            let mut uri = self.client.keyvault_client.vault_url.clone();
            let version = self.version.unwrap_or_default();
            uri.set_path(&format!("certificates/{}/{}", self.name, version));

            let headers = Headers::new();
            let mut request = KeyvaultClient::finalize_request(uri, Method::GET, headers, None);

            self.client
                .keyvault_client
                .send(&self.context, &mut request)
                .await?
                .json()
                .await
        })
    }
}

type GetCertificateResponse = KeyVaultGetCertificateResponse;
