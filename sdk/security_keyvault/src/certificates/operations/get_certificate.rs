use crate::prelude::*;
use azure_core::{headers::Headers, CollectedResponse, Method};

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

            let response: KeyVaultGetCertificateResponse = serde_json::from_slice(body)?;

            Ok(response)
        })
    }
}

type GetCertificateResponse = KeyVaultGetCertificateResponse;
