use crate::prelude::*;
use azure_core::{headers::Headers, to_json, Method};
use serde::Serialize;
use std::collections::HashMap;
use time::OffsetDateTime;

operation! {
    ImportCertificate,
    client: CertificateClient,
    name: String,
    value: String,
    ?pwd: String,
    ?enabled: bool,
    ?not_before: OffsetDateTime,
    ?expiration: OffsetDateTime,
    ?tags: HashMap<String, String>
}

#[derive(Serialize, Debug)]
struct Attributes {
    enabled: Option<bool>,
    #[serde(default, with = "azure_core::date::timestamp::option", rename = "exp")]
    expiration: Option<OffsetDateTime>,
    #[serde(default, with = "azure_core::date::timestamp::option", rename = "nbf")]
    not_before: Option<OffsetDateTime>,
}

#[derive(Serialize, Debug)]
struct ImportRequest {
    value: String,
    attributes: Attributes,
    #[serde(skip_serializing_if = "Option::is_none")]
    pwd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<HashMap<String, String>>,
}

impl ImportCertificateBuilder {
    pub fn into_future(self) -> ImportCertificate {
        Box::pin(async move {
            let mut uri = self.client.keyvault_client.vault_url.clone();
            uri.set_path(&format!("certificates/{}/import", self.name));

            let request = ImportRequest {
                value: self.value,
                attributes: Attributes {
                    enabled: self.enabled,
                    expiration: self.expiration,
                    not_before: self.not_before,
                },
                pwd: self.pwd,
                tags: self.tags,
            };

            let body = to_json(&request)?;

            let headers = Headers::new();
            let mut request =
                KeyvaultClient::finalize_request(uri, Method::POST, headers, Some(body.into()));

            self.client
                .keyvault_client
                .send(&self.context, &mut request)
                .await?
                .json()
                .await
        })
    }
}

type ImportCertificateResponse = KeyVaultGetCertificateResponse;
