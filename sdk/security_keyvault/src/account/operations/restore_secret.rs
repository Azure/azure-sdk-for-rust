use crate::prelude::*;

operation! {
    RestoreSecret,
    client: KeyvaultClient,
    backup_blob: String,
}

impl RestoreSecretBuilder {
    pub fn into_future(mut self) -> RestoreSecret {
        Box::pin(async move {
            let mut uri = self.client.vault_url.clone();
            uri.set_path("secrets/restore");
            uri.set_query(Some(API_VERSION_PARAM));

            let mut request_body = serde_json::Map::new();
            request_body.insert("value".to_owned(), self.backup_blob.into());

            self.client
                .request(
                    reqwest::Method::POST,
                    uri.to_string(),
                    Some(serde_json::Value::Object(request_body).to_string()),
                )
                .await?;

            Ok(())
        })
    }
}

type RestoreSecretResponse = ();
