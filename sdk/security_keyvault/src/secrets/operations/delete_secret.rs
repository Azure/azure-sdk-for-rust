use crate::prelude::*;
use azure_core::{headers::Headers, Method};

operation! {
    DeleteSecret,
    client: SecretClient,
    name: String,
}

impl DeleteSecretBuilder {
    pub fn into_future(mut self) -> DeleteSecret {
        Box::pin(async move {
            let mut uri = self.client.client.vault_url.clone();
            uri.set_path(&format!("secrets/{}", self.name));

            let headers = Headers::new();
            let mut request =
                self.client
                    .client
                    .finalize_request(uri, Method::Delete, headers, None)?;

            self.client
                .client
                .send(&mut self.context, &mut request)
                .await?;

            Ok(())
        })
    }
}

type DeleteSecretResponse = ();
