use crate::prelude::*;
use azure_core::{headers::Headers, Method};

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

type GetCertificateOperationResponse = CertificateOperationResponse;
