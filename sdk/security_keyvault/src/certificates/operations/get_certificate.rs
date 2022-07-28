use crate::prelude::*;
use azure_core::{headers::Headers, CollectedResponse, Method};

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

            let headers = Headers::new();
            let mut request =
                self.client
                    .client
                    .finalize_request(uri, Method::Get, headers, None)?;

            let response = self
                .client
                .client
                .send(&mut self.context, &mut request)
                .await?;

            let response = CollectedResponse::from_response(response).await?;
            let body = response.body();

            let response: KeyVaultGetCertificateResponse = serde_json::from_slice(body)?;

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
