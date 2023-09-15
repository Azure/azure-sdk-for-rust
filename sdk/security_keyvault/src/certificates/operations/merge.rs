use crate::prelude::*;
use azure_core::{headers::Headers, CollectedResponse, Method};
use serde::Serialize;
use std::collections::HashMap;
use time::OffsetDateTime;

operation! {
    MergeCertificate,
    client: CertificateClient,
    name: String,
    x5c: Vec<String>,
    ?enabled: bool,
    ?not_before: OffsetDateTime,
    ?expiration: OffsetDateTime,
    ?tags: HashMap<String, String>
}

#[derive(Serialize, Debug)]
struct Attributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(default, with = "azure_core::date::timestamp::option", rename = "exp")]
    expiration: Option<OffsetDateTime>,
    #[serde(default, with = "azure_core::date::timestamp::option", rename = "nbf")]
    not_before: Option<OffsetDateTime>,
}

#[derive(Serialize, Debug)]
struct MergeRequest {
    x5c: Vec<String>,
    attributes: Attributes,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<HashMap<String, String>>,
}

impl MergeCertificateBuilder {
    pub fn into_future(self) -> MergeCertificate {
        Box::pin(async move {
            let mut uri = self.client.keyvault_client.vault_url.clone();
            uri.set_path(&format!("certificates/{}/pending/merge", self.name));

            let request = MergeRequest {
                x5c: self.x5c,
                attributes: Attributes {
                    enabled: self.enabled,
                    expiration: self.expiration,
                    not_before: self.not_before,
                },
                tags: self.tags,
            };

            let body = serde_json::to_string(&request)?;

            let headers = Headers::new();
            let mut request = self.client.keyvault_client.finalize_request(
                uri,
                Method::Post,
                headers,
                Some(body.into()),
            )?;

            let response = self
                .client
                .keyvault_client
                .send(&self.context, &mut request)
                .await?;

            let response = CollectedResponse::from_response(response).await?;
            let body = response.body();
            let response = serde_json::from_slice::<KeyVaultGetCertificateResponse>(body)?;
            Ok(response)
        })
    }
}

type MergeCertificateResponse = KeyVaultGetCertificateResponse;
