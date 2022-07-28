use crate::prelude::*;
use azure_core::error::{ErrorKind, ResultExt};

operation! {
    CertificateBackup,
    client: CertificateClient,
    name: String,
}

impl CertificateBackupBuilder {
    pub fn into_future(mut self) -> CertificateBackup {
        Box::pin(async move {
            let mut uri = self.client.client.vault_url.clone();
            uri.set_path(&format!("certificates/{}/backup", self.name));
            uri.set_query(Some(API_VERSION_PARAM));

            let response_body = self
                .client
                .client
                .request(reqwest::Method::POST, uri.to_string(), None)
                .await?;

            let backup_blob = serde_json::from_str::<CertificateBackupResponse>(&response_body)
                .with_context(ErrorKind::DataConversion, || {
                    format!("failed to parse certificate backup response. uri: {uri}")
                })?;

            Ok(backup_blob)
        })
    }
}
