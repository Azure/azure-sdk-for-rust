use crate::prelude::*;
use azure_core::{headers::Headers, Method};

operation! {
    RestoreCertificate,
    client: CertificateClient,
    backup_blob: String,
}

impl RestoreCertificateBuilder {
    pub fn into_future(self) -> RestoreCertificate {
        Box::pin(async move {
            let mut uri = self.client.keyvault_client.vault_url.clone();
            uri.set_path("certificates/restore");

            let mut request_body = serde_json::Map::new();
            request_body.insert("value".to_owned(), self.backup_blob.into());

            let headers = Headers::new();
            let mut request = self.client.keyvault_client.finalize_request(
                uri,
                Method::Post,
                headers,
                Some(serde_json::Value::Object(request_body).to_string().into()),
            )?;

            self.client
                .keyvault_client
                .send(&self.context, &mut request)
                .await?;

            Ok(())
        })
    }
}

type RestoreCertificateResponse = ();
