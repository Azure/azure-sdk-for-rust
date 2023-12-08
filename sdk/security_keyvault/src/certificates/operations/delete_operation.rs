use crate::prelude::*;
use azure_core::{headers::Headers, Method};

operation! {
    DeleteCertificateOperation,
    client: CertificateClient,
    name: String,
}

impl DeleteCertificateOperationBuilder {
    pub fn into_future(self) -> DeleteCertificateOperation {
        Box::pin(async move {
            let mut uri = self.client.keyvault_client.vault_url.clone();
            uri.set_path(&format!("certificates/{}/pending", self.name));

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

type DeleteCertificateOperationResponse = CertificateOperationResponse;
