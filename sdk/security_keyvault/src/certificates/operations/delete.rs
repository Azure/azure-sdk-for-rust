use crate::prelude::*;
use azure_core::{headers::Headers, CollectedResponse, Method};

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
            let mut request =
                self.client
                    .keyvault_client
                    .finalize_request(uri, Method::Delete, headers, None)?;

            let response = self
                .client
                .keyvault_client
                .send(&self.context, &mut request)
                .await?;

            let response = CollectedResponse::from_response(response).await?;
            let body = response.body();
            let response = serde_json::from_slice::<KeyVaultGetCertificateResponse>(body)?;
            Ok(response)
        })
    }
}

type DeleteCertificateResponse = KeyVaultGetCertificateResponse;
