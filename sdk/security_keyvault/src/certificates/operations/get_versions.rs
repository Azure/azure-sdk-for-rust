use crate::prelude::*;
use azure_core::{error::Error, headers::Headers, Method, Pageable, Url};

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

                this.client
                    .keyvault_client
                    .send(&ctx, &mut request)
                    .await?
                    .json()
                    .await
            }
        };
        Pageable::new(make_request)
    }
}
