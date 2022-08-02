use crate::prelude::*;
use azure_core::error::{ErrorKind, ResultExt};
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
    #[serde(with = "azure_core::date::timestamp::option", rename = "exp")]
    expiration: Option<OffsetDateTime>,
    #[serde(with = "azure_core::date::timestamp::option", rename = "nbf")]
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
    pub fn into_future(mut self) -> UpdateSecret {
        Box::pin(async move {
            let mut uri = self.client.client.vault_url.clone();
            let version = self.version.unwrap_or_default();
            uri.set_path(&format!("secrets/{}/{}", self.name, version));
            uri.set_query(Some(API_VERSION_PARAM));

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

            let body = serde_json::to_string(&request)
                .with_context(ErrorKind::Other, || {
                    format!(
                        "failed to serialize UpdateRequest. secret_name: {} secret_version_name: {version}",
                        self.name
                    )
                })?;

            self.client
                .client
                .request(reqwest::Method::PATCH, uri.to_string(), Some(body))
                .await
                .with_context(ErrorKind::Other, || {
                    format!("failed to set secret. secret_name: {}", self.name)
                })?;

            Ok(())
        })
    }
}

type UpdateSecretResponse = ();
