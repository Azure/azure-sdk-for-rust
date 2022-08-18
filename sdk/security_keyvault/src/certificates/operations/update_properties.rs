use crate::prelude::*;
use azure_core::error::{ErrorKind, ResultExt};
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
    pub fn into_future(mut self) -> UpdateCertificateProperties {
        Box::pin(async move {
            let mut uri = self.client.client.vault_url.clone();
            let version = self.version.unwrap_or_default();
            uri.set_path(&format!("certificates/{}/{}", self.name, version));
            uri.set_query(Some(API_VERSION_PARAM));

            let request = UpdateRequest {
                attributes: Attributes {
                    enabled: self.enabled,
                    expiration: self.expiration,
                    not_before: self.not_before,
                },
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
                .await?;

            Ok(())
        })
    }
}

type UpdateCertificatePropertiesResponse = ();
