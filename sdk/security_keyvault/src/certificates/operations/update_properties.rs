use crate::prelude::*;
use azure_core::{headers::Headers, Method};
use serde::Serialize;
use time::OffsetDateTime;

operation! {
    UpdateCertificateProperties,
    client: CertificateClient,
    name: String,
    ?version: String,
    ?enabled: bool,
    ?not_before: OffsetDateTime,
    ?expiration: OffsetDateTime
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Attributes {
    enabled: Option<bool>,
    #[serde(default, with = "azure_core::date::timestamp::option", rename = "exp")]
    expiration: Option<OffsetDateTime>,
    #[serde(default, with = "azure_core::date::timestamp::option", rename = "nbf")]
    not_before: Option<OffsetDateTime>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct UpdateRequest {
    attributes: Attributes,
}

impl UpdateCertificatePropertiesBuilder {
    pub fn into_future(self) -> UpdateCertificateProperties {
        Box::pin(async move {
            let mut uri = self.client.keyvault_client.vault_url.clone();
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

type UpdateCertificatePropertiesResponse = ();
