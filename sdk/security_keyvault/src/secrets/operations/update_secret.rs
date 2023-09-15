use crate::prelude::*;
use azure_core::{headers::Headers, Method};
use serde::Serialize;
use std::collections::HashMap;
use time::OffsetDateTime;

operation! {
    UpdateSecret,
    client: SecretClient,
    name: String,
    ?version: String,
    ?enabled: bool,
    ?content_type: String,
    ?expiration: OffsetDateTime,
    ?not_before: OffsetDateTime,
    ?recoverable_days: usize,
    ?tags: HashMap<String, String>,
    ?recovery_level: String
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Attributes {
    enabled: Option<bool>,
    #[serde(default, with = "azure_core::date::timestamp::option", rename = "exp")]
    expiration: Option<OffsetDateTime>,
    #[serde(default, with = "azure_core::date::timestamp::option", rename = "nbf")]
    not_before: Option<OffsetDateTime>,
    recovery_level: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct UpdateRequest {
    content_type: Option<String>,
    attributes: Attributes,
    tags: Option<HashMap<String, String>>,
}

impl UpdateSecretBuilder {
    pub fn into_future(self) -> UpdateSecret {
        Box::pin(async move {
            let mut uri = self.client.keyvault_client.vault_url.clone();
            let version = self.version.unwrap_or_default();
            uri.set_path(&format!("secrets/{}/{}", self.name, version));

            let request = UpdateRequest {
                content_type: self.content_type,
                attributes: Attributes {
                    enabled: self.enabled,
                    expiration: self.expiration,
                    not_before: self.not_before,
                    recovery_level: self.recovery_level,
                },
                tags: self.tags,
            };

            let body = serde_json::to_string(&request)?;

            let headers = Headers::new();
            let mut request = self.client.keyvault_client.finalize_request(
                uri,
                Method::Patch,
                headers,
                Some(body.into()),
            )?;

            self.client
                .keyvault_client
                .send(&self.context, &mut request)
                .await?;

            Ok(())
        })
    }
}

type UpdateSecretResponse = ();
