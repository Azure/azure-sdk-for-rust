use crate::prelude::*;
use azure_core::error::{ErrorKind, ResultExt};

operation! {
    GetCertificate,
    client: CertificateClient,
    name :String,
    ?version: String
}

impl GetCertificateBuilder {
    pub fn into_future(mut self) -> GetCertificate {
        Box::pin(async move {
            let mut uri = self.client.client.vault_url.clone();
            let version = self.version.unwrap_or_default();
            uri.set_path(&format!("certificates/{}/{}", self.name, version));
            uri.set_query(Some(API_VERSION_PARAM));

            let response_body = self
                .client
                .client
                .request(reqwest::Method::GET, uri.to_string(), None)
                .await?;
            let response = serde_json::from_str::<KeyVaultGetCertificateResponse>(&response_body)
            .with_context(ErrorKind::DataConversion, || {
                format!("failed to parse get certificate response. uri: {uri} certificate_name: {} response_body: {response_body}", self.name)
            })?;
            Ok(KeyVaultCertificate {
                key_id: response.kid,
                secret_id: response.sid,
                x5t: response.x5t,
                cer: response.cer,
                content_type: response.policy.secret_props.content_type,
                properties: CertificateProperties {
                    id: response.id,
                    name: self.name.to_string(),
                    version: version.to_string(),
                    enabled: response.attributes.enabled,
                    not_before: response.attributes.nbf,
                    expires_on: response.attributes.exp,
                    created_on: response.attributes.created,
                    updated_on: response.attributes.updated,
                },
            })
        })
    }
}

type GetCertificateResponse = KeyVaultCertificate;
