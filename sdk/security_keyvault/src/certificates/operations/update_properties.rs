use crate::prelude::*;
use azure_core::{headers::Headers, Method};
use chrono::{serde::ts_seconds_option, DateTime, Utc};
use serde::Serialize;

operation! {
    UpdateCertificateProperties,
    client: CertificateClient,
    name: String,
    ?version: String,
    ?enabled: bool,
    ?not_before: DateTime<Utc>,
    ?expiration: DateTime<Utc>
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Attributes {
    enabled: Option<bool>,
    #[serde(with = "ts_seconds_option", rename = "exp")]
    expiration: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option", rename = "nbf")]
    not_before: Option<DateTime<Utc>>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct UpdateRequest {
    attributes: Attributes,
}

impl UpdateCertificatePropertiesBuilder {
    pub fn into_future(mut self) -> UpdateCertificateProperties {
        Box::pin(async move {
            let mut uri = self.client.client.vault_url.clone();
            let version = self.version.unwrap_or_default();
            uri.set_path(&format!("certificates/{}/{}", self.name, version));

            let request = UpdateRequest {
                attributes: Attributes {
                    enabled: self.enabled,
                    expiration: self.expiration,
                    not_before: self.not_before,
                },
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

type UpdateCertificatePropertiesResponse = ();
