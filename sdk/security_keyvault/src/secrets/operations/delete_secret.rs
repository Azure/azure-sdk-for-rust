use crate::prelude::*;

operation! {
    DeleteSecret,
    client: SecretClient,
}

impl DeleteSecretBuilder {
    pub fn into_future(mut self) -> DeleteSecret {
        Box::pin(async move {
            let mut uri = self.client.client.vault_url.clone();
            uri.set_path(&format!("secrets/{}", self.client.name));
            uri.set_query(Some(API_VERSION_PARAM));

            self.client
                .client
                .request(reqwest::Method::DELETE, uri.to_string(), None)
                .await?;

            Ok(())
        })
    }
}

type DeleteSecretResponse = ();
