use crate::prelude::*;

operation! {
    RestoreCertificate,
    client: CertificateClient,
    backup_blob: String,
}

impl RestoreCertificateBuilder {
    pub fn into_future(mut self) -> RestoreCertificate {
        Box::pin(async move {
            let mut uri = self.client.client.vault_url.clone();
            uri.set_path("certificates/restore");
            uri.set_query(Some(API_VERSION_PARAM));

            let mut request_body = serde_json::Map::new();
            request_body.insert("value".to_owned(), self.backup_blob.into());

            self.client
                .client
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

type RestoreCertificateResponse = ();
