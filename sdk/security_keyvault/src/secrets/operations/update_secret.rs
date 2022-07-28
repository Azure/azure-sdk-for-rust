use crate::prelude::*;
use azure_core::{headers::Headers, Method};
use chrono::{serde::ts_seconds_option, DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;

operation! {
    UpdateSecret,
    client: SecretClient,
    name: String,
    ?version: String,
    ?enabled: bool,
    ?content_type: String,
    ?expiration: DateTime<Utc>,
    ?not_before: DateTime<Utc>,
    ?recoverable_days: usize,
    ?tags: HashMap<String, String>,
    ?recovery_level: String
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Attributes {
    enabled: Option<bool>,
    #[serde(with = "ts_seconds_option", rename = "exp")]
    expiration: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option", rename = "nbf")]
    not_before: Option<DateTime<Utc>>,
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
            let mut request = self.client.client.finalize_request(
                uri,
                Method::Patch,
                headers,
                Some(body.into()),
            )?;

            self.client
                .client
                .send(&mut self.context, &mut request)
                .await?;

            Ok(())
        })
    }
}

type UpdateSecretResponse = ();
