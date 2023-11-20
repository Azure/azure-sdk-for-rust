use crate::prelude::*;
use azure_core::{error::Error, headers::Headers, CollectedResponse, Method, Pageable};
use url::Url;

operation! {
    #[stream]
    GetCertificateVersions,
    client: CertificateClient,
    name: String,
}

impl GetCertificateVersionsBuilder {
    pub fn into_stream(self) -> Pageable<KeyVaultGetCertificatesResponse, Error> {
        let make_request = move |continuation: Option<String>| {
            let this = self.clone();
            let ctx = self.context.clone();
            async move {
                let mut uri = this.client.keyvault_client.vault_url.clone();
                uri.set_path(&format!("certificates/{}/versions", this.name));

                if let Some(continuation) = continuation {
                    uri = Url::parse(&continuation)?;
                }

                let headers = Headers::new();
                let mut request = KeyvaultClient::finalize_request(uri, Method::Get, headers, None);

                let response = this.client.keyvault_client.send(&ctx, &mut request).await?;

                let response = CollectedResponse::from_response(response).await?;
                let body = response.body();

                let response = serde_json::from_slice::<KeyVaultGetCertificatesResponse>(body)?;
                Ok(response)
            }
        };
        Pageable::new(make_request)
    }
}
