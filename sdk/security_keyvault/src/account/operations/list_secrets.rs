use crate::prelude::*;
use azure_core::{error::Error, headers::Headers, Continuable, Method, Pageable, Url};

operation! {
    #[stream]
    ListSecrets,
    client: SecretClient,
}

impl ListSecretsBuilder {
    pub fn into_stream(self) -> Pageable<KeyVaultGetSecretsResponse, Error> {
        let make_request = move |continuation: Option<String>| {
            let this = self.clone();
            let ctx = self.context.clone();
            async move {
                let url = if let Some(url) = continuation {
                    Url::parse(&url)?
                } else {
                    let mut url = this.client.keyvault_client.vault_url.clone();
                    url.set_path("secrets");
                    url
                };

                let headers = Headers::new();
                let mut request = KeyvaultClient::finalize_request(url, Method::Get, headers, None);

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

type ListSecretsResponse = KeyVaultGetSecretsResponse;

impl Continuable for ListSecretsResponse {
    type Continuation = String;

    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
