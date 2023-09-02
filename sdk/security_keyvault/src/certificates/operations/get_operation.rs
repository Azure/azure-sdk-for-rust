use crate::prelude::*;
use azure_core::{headers::Headers, CollectedResponse, Method};

operation! {
    GetCertificateOperation,
    client: CertificateClient,
    name :String,
}

impl GetCertificateOperationBuilder {
    pub fn into_future(self) -> GetCertificateOperation {
        Box::pin(async move {
            let mut uri = self.client.keyvault_client.vault_url.clone();
            uri.set_path(&format!("certificates/{}/pending", self.name));

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

            let response: CertificateOperationResponse = serde_json::from_slice(body)?;

            Ok(response)
        })
    }
}

type GetCertificateOperationResponse = CertificateOperationResponse;
