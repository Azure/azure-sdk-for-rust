use crate::prelude::*;
use azure_core::{headers::Headers, CollectedResponse, Method};
use serde::Serialize;
use std::collections::HashMap;
use time::OffsetDateTime;

operation! {
    CreateCertificate,
    client: CertificateClient,
    name: String,
    subject: String,
    issuer_name: String,
    ?kty: JsonWebKeyType,
    ?key_size: i32,
    ?dns_names: Vec<String>,
    ?exportable: bool,
    ?reuse_key: bool,
    ?enabled: bool,
    ?not_before: OffsetDateTime,
    ?expiration: OffsetDateTime,
    ?tags: HashMap<String, String>,
    ?cert_transparency: bool
}

#[derive(Serialize, Debug)]
struct CreateRequest {
    policy: Policy,
    attributes: Attributes,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<HashMap<String, String>>,
}

#[derive(Serialize, Debug)]
struct Attributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(with = "azure_core::date::timestamp::option", rename = "exp")]
    expiration: Option<OffsetDateTime>,
    #[serde(with = "azure_core::date::timestamp::option", rename = "nbf")]
    not_before: Option<OffsetDateTime>,
}

#[derive(Serialize, Debug)]
struct Policy {
    key_props: KeyProperties,
    x509_props: X509Properties,
    issuer: Issuer,
}

#[derive(Serialize, Debug)]
struct KeyProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    exportable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kty: Option<JsonWebKeyType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reuse_key: Option<bool>,
}

#[derive(Serialize, Debug)]
struct X509Properties {
    subject: String,
    sans: SubjectAlternativeNames,
}

#[derive(Serialize, Debug)]
struct SubjectAlternativeNames {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_names: Option<Vec<String>>,
}

#[derive(Serialize, Debug)]
struct Issuer {
    #[serde(skip_serializing_if = "Option::is_none")]
    cert_transparency: Option<bool>,
    name: String,
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum JsonWebKeyType {
    #[serde(rename = "EC")]
    Ec,
    #[serde(rename = "EC-HSM")]
    EcHsm,
    #[serde(rename = "RSA")]
    Rsa,
    #[serde(rename = "RSA-HSM")]
    RsaHsm,
    #[serde(rename = "oct")]
    Oct,
    #[serde(rename = "oct-HSM")]
    OctHsm,
}

impl CreateCertificateBuilder {
    pub fn into_future(self) -> CreateCertificate {
        Box::pin(async move {
            let mut uri = self.client.keyvault_client.vault_url.clone();
            uri.set_path(&format!("certificates/{}/create", self.name));

            let request = CreateRequest {
                policy: Policy {
                    key_props: KeyProperties {
                        exportable: self.exportable,
                        kty: self.kty,
                        key_size: self.key_size,
                        reuse_key: self.reuse_key,
                    },
                    x509_props: X509Properties {
                        subject: self.subject,
                        sans: SubjectAlternativeNames {
                            dns_names: self.dns_names,
                        },
                    },
                    issuer: Issuer {
                        name: self.issuer_name,
                        cert_transparency: self.cert_transparency,
                    },
                },
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
            let response = serde_json::from_slice::<CertificateOperationResponse>(body)?;
            Ok(response)
        })
    }
}

type CreateCertificateResponse = CertificateOperationResponse;
