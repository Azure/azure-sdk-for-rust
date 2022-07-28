use crate::prelude::*;
use azure_core::{
    error::Error, headers::Headers, CollectedResponse, Continuable, Method, Pageable,
};
use url::Url;

operation! {
    #[stream]
    ListSecrets,
    client: SecretClient,
}

impl ListSecretsBuilder {
    pub fn into_stream(self) -> Pageable<KeyVaultGetSecretsResponse, Error> {
        let make_request = move |continuation: Option<String>| {
            let this = self.clone();
            let mut ctx = self.context.clone();
            async move {
                let mut uri = this.client.client.vault_url.clone();
                uri.set_path("secrets");

                println!("url: {}", uri);
                if let Some(continuation) = continuation {
                    uri = Url::parse(&continuation)?;
                }

                let headers = Headers::new();
                let mut request =
                    this.client
                        .client
                        .finalize_request(uri, Method::Get, headers, None)?;

                let response = this.client.client.send(&mut ctx, &mut request).await?;

                let response = CollectedResponse::from_response(response).await?;
                let body = response.body();

                let response = serde_json::from_slice::<KeyVaultGetSecretsResponse>(body)?;
                Ok(response)
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
