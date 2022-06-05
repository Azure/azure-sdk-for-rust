use crate::client::API_VERSION_PARAM;
use crate::CertificateClient;
use azure_core::error::{Error, ErrorKind, ResultExt};

use azure_core::auth::TokenCredential;
use chrono::serde::{ts_seconds, ts_seconds_option};
use chrono::{DateTime, Utc};
use getset::Getters;
use reqwest::Url;
use serde::Deserialize;
use serde_json::{Map, Value};

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultCertificateBaseIdentifierAttributedRaw {
    enabled: bool,
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    exp: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    nbf: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds")]
    created: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    updated: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultCertificateBaseIdentifierRaw {
    id: String,
    #[allow(unused)]
    x5t: String,
    attributes: KeyVaultCertificateBaseIdentifierAttributedRaw,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetCertificatesResponse {
    value: Vec<KeyVaultCertificateBaseIdentifierRaw>,
    #[serde(rename = "nextLink")]
    next_link: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetCertificateResponse {
    kid: String,
    sid: String,
    x5t: String,
    cer: String,
    id: String,
    attributes: KeyVaultGetCertificateResponseAttributes,
    policy: KeyVaultGetCertificateResponsePolicy,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetCertificateResponseAttributes {
    enabled: bool,
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    exp: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    nbf: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds")]
    created: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    updated: DateTime<Utc>,
    #[serde(rename = "recoveryLevel")]
    #[allow(unused)]
    recovery_level: String,
}
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub(crate) struct KeyVaultGetCertificateResponsePolicy {
    id: String,
    key_props: KeyVaultGetCertificateResponsePolicyKeyProperties,
    secret_props: KeyVaultGetCertificateResponsePolicySecretProperties,
    x509_props: KeyVaultGetCertificateResponsePolicyX509Properties,
    issuer: KeyVaultGetCertificateResponsePolicyIssuer,
    attributes: KeyVaultGetCertificateResponsePolicyAttributes,
}
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub(crate) struct KeyVaultGetCertificateResponsePolicyKeyProperties {
    exportable: bool,
    kty: String,
    key_size: u64,
    reuse_key: bool,
}
#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetCertificateResponsePolicySecretProperties {
    #[serde(rename = "contentType")]
    content_type: String,
}
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub(crate) struct KeyVaultGetCertificateResponsePolicyX509Properties {
    subject: String,
    validity_months: u64,
}
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub(crate) struct KeyVaultGetCertificateResponsePolicyIssuer {
    name: String,
}
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub(crate) struct KeyVaultGetCertificateResponsePolicyAttributes {
    enabled: bool,
    #[serde(with = "ts_seconds")]
    created: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    updated: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultCertificateBackupResponseRaw {
    value: String,
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct KeyVaultCertificate {
    key_id: String,
    secret_id: String,
    x5t: String,
    cer: String,
    content_type: String,
    properties: CertificateProperties,
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct CertificateProperties {
    id: String,
    name: String,
    version: String,
    not_before: Option<DateTime<Utc>>,
    expires_on: Option<DateTime<Utc>>,
    created_on: DateTime<Utc>,
    updated_on: DateTime<Utc>,
    enabled: bool,
}

pub struct CertificateBackupResult {
    pub backup: Vec<u8>,
}

impl<'a, T: TokenCredential> CertificateClient<'a, T> {
    /// Gets a certificate from the Key Vault.
    /// Note that the latest version is fetched. For a specific version, use `get_certificate_with_version`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_security_keyvault::CertificateClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    ///     let mut client = CertificateClient::new(
    ///         &"KEYVAULT_URL",
    ///         &creds,
    ///     ).unwrap();
    ///     let certificate = client.get_certificate(&"CERTIFICATE_NAME").await.unwrap();
    ///     dbg!(&certificate);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn get_certificate(&mut self, name: &'a str) -> Result<KeyVaultCertificate, Error> {
        self.get_certificate_with_version(name, "").await
    }

    /// Gets a certificate from the Key Vault with a specific version.
    /// If you need the latest version, use `get_certificate`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_security_keyvault::CertificateClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    /// let mut client = CertificateClient::new(
    ///     &"KEYVAULT_URL",
    ///     &creds,
    ///     ).unwrap();
    ///     let certificate = client.get_certificate_with_version(&"CERTIFICATE_NAME", &"CERTIFICATE_VERSION").await.unwrap();
    ///     dbg!(&certificate);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn get_certificate_with_version(
        &mut self,
        name: &'a str,
        version: &'a str,
    ) -> Result<KeyVaultCertificate, Error> {
        let mut uri = self.vault_url.clone();
        uri.set_path(&format!("certificates/{}/{}", name, version));
        uri.set_query(Some(API_VERSION_PARAM));

        let response_body = self.get_authed(uri.to_string()).await?;
        let response = serde_json::from_str::<KeyVaultGetCertificateResponse>(&response_body)
            .with_context(ErrorKind::DataConversion, || {
                format!("failed to parse get certificate response. uri: {} certificate_name: {} response_body: {}", uri, name, response_body)
            })?;
        Ok(KeyVaultCertificate {
            key_id: response.kid,
            secret_id: response.sid,
            x5t: response.x5t,
            cer: response.cer,
            content_type: response.policy.secret_props.content_type,
            properties: CertificateProperties {
                id: response.id,
                name: name.to_string(),
                version: version.to_string(),
                enabled: response.attributes.enabled,
                not_before: response.attributes.nbf,
                expires_on: response.attributes.exp,
                created_on: response.attributes.created,
                updated_on: response.attributes.updated,
            },
        })
    }

    /// Lists all the certificates in the Key Vault.
    ///
    /// ```no_run
    /// use azure_security_keyvault::CertificateClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    ///     let mut client = CertificateClient::new(
    ///          &"KEYVAULT_URL",
    ///          &creds,
    ///     ).unwrap();
    ///     let certificates = client.list_properties_of_certificates().await.unwrap();
    ///     dbg!(&certificates);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn list_properties_of_certificates(
        &mut self,
    ) -> Result<Vec<CertificateProperties>, Error> {
        let mut certificates = Vec::<CertificateProperties>::new();

        let mut uri = self.vault_url.clone();
        uri.set_path("certificates");
        uri.set_query(Some(API_VERSION_PARAM));

        loop {
            let resp_body = self.get_authed(uri.to_string()).await?;
            let response =
                serde_json::from_str::<KeyVaultGetCertificatesResponse>(&resp_body).unwrap();

            certificates.extend(
                response
                    .value
                    .into_iter()
                    .map(|s| CertificateProperties {
                        id: s.id.to_owned(),
                        name: s.id.split('/').collect::<Vec<_>>()[4].to_string(),
                        version: s.id.split('/').collect::<Vec<_>>()[5].to_string(),
                        enabled: s.attributes.enabled,
                        created_on: s.attributes.created,
                        updated_on: s.attributes.updated,
                        not_before: s.attributes.nbf,
                        expires_on: s.attributes.exp,
                    })
                    .collect::<Vec<CertificateProperties>>(),
            );

            match response.next_link {
                None => break,
                Some(u) => uri = Url::parse(&u).unwrap(),
            }
        }

        Ok(certificates)
    }

    /// Gets all the versions for a certificate in the Key Vault.
    //
    /// # Example
    ///
    /// ```no_run
    /// use azure_security_keyvault::CertificateClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    ///     let mut client = CertificateClient::new(
    ///         &"KEYVAULT_URL",
    ///         &creds,
    ///     ).unwrap();
    ///     let certificate_versions = client.list_properties_of_certificate_versions(&"CERTIFICATE_NAME").await.unwrap();
    ///     dbg!(&certificate_versions);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn list_properties_of_certificate_versions(
        &mut self,
        name: &'a str,
    ) -> Result<Vec<CertificateProperties>, Error> {
        let mut versions = Vec::<CertificateProperties>::new();

        let mut uri = self.vault_url.clone();
        uri.set_path(&format!("certificates/{}/versions", name));
        uri.set_query(Some(API_VERSION_PARAM));

        loop {
            let resp_body = self.get_authed(uri.to_string()).await?;

            let response =
                serde_json::from_str::<KeyVaultGetCertificatesResponse>(&resp_body).unwrap();

            versions.extend(
                response
                    .value
                    .into_iter()
                    .map(|s| CertificateProperties {
                        id: s.id.to_owned(),
                        name: name.to_string(),
                        version: s.id.split('/').collect::<Vec<_>>()[5].to_string(),
                        enabled: s.attributes.enabled,
                        created_on: s.attributes.created,
                        updated_on: s.attributes.updated,
                        not_before: s.attributes.nbf,
                        expires_on: s.attributes.exp,
                    })
                    .collect::<Vec<CertificateProperties>>(),
            );
            match response.next_link {
                None => break,
                Some(u) => uri = Url::parse(&u).unwrap(),
            }
        }

        // Return the certificate versions sorted by the time modified in descending order.
        versions.sort_by(|a, b| {
            if a.updated_on > b.updated_on {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        Ok(versions)
    }

    pub async fn update_certificate_attributes(
        &mut self,
        properties: CertificateProperties,
    ) -> Result<(), Error> {
        let mut uri = self.vault_url.clone();
        uri.set_path(&format!(
            "certificates/{}/{}",
            properties.name, properties.version
        ));
        uri.set_query(Some(API_VERSION_PARAM));

        let mut request_body = Map::new();
        request_body.insert(
            "attributes".to_string(),
            serde_json::json!({
                "enabled": properties.enabled,
                "nbf": properties.not_before,
                "exp": properties.expires_on,
            }),
        );

        self.patch_authed(uri.to_string(), Value::Object(request_body).to_string())
            .await?;

        Ok(())
    }

    async fn _update_certificate_policy(
        &mut self,
        name: &'a str,
        policy: Map<String, Value>,
    ) -> Result<(), Error> {
        let mut uri = self.vault_url.clone();
        uri.set_path(&format!("certificates/{}/policy", name));
        uri.set_query(Some(API_VERSION_PARAM));

        self.patch_authed(uri.to_string(), Value::Object(policy).to_string())
            .await?;

        Ok(())
    }

    /// Restores a backed up certificate and all its versions.
    /// This operation requires the certificates/restore permission.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_security_keyvault::CertificateClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    ///     let mut client = CertificateClient::new(
    ///         &"KEYVAULT_URL",
    ///         &creds,
    ///     ).unwrap();
    ///     client.restore_certificate(b"KUF6dXJlS2V5VmF1bHRTZWNyZXRCYWNrdXBWMS5taW").await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn restore_certificate(&mut self, backup: &[u8]) -> Result<(), Error> {
        let mut uri = self.vault_url.clone();
        uri.set_path("certificates/restore");
        uri.set_query(Some(API_VERSION_PARAM));

        let mut request_body = Map::new();
        request_body.insert("value".to_owned(), Value::String(base64::encode(backup)));

        self.post_authed(
            uri.to_string(),
            Some(Value::Object(request_body).to_string()),
        )
        .await?;

        Ok(())
    }

    /// Restores a backed up certificate and all its versions.
    /// This operation requires the certificates/restore permission.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_security_keyvault::CertificateClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    ///     let mut client = CertificateClient::new(
    ///         &"KEYVAULT_URL",
    ///         &creds,
    ///     ).unwrap();
    ///     client.backup_certificate(&"CERTIFICATE_NAME").await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn backup_certificate(
        &mut self,
        name: &'a str,
    ) -> Result<CertificateBackupResult, Error> {
        let mut uri = self.vault_url.clone();
        uri.set_path(&format!("certificates/{}/backup", name));
        uri.set_query(Some(API_VERSION_PARAM));

        let response_body = self.post_authed(uri.to_string(), None).await?;
        let backup_blob =
            serde_json::from_str::<KeyVaultCertificateBackupResponseRaw>(&response_body)
                .with_context(ErrorKind::DataConversion, || {
                    format!("failed to parse certificate backup response. uri: {}", uri)
                })?;

        Ok(CertificateBackupResult {
            backup: base64::decode(backup_blob.value).context(
                ErrorKind::DataConversion,
                "failed base64 decode of backup blob",
            )?,
        })
    }

    /// Deletes a certificate in the Key Vault.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the certificate
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_security_keyvault::CertificateClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use tokio::runtime::Runtime;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    ///     let mut client = CertificateClient::new(
    ///     &"KEYVAULT_URL",
    ///     &creds,
    ///     ).unwrap();
    ///     client.delete_certificate(&"CERTIFICATE_NAME").await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn delete_certificate(&mut self, _name: &'a str) -> Result<(), Error> {
        // let mut uri = self.vault_url.clone();
        // uri.set_path(&format!("certificates/{}", certificate_name));
        // uri.set_query(Some(API_VERSION_PARAM));

        // self.delete_authed(uri.to_string()).await?;

        // Ok(())

        todo!("See issue #174 at: https://github.com/Azure/azure-sdk-for-rust/issues/174.")
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use super::*;

    use chrono::{Duration, Utc};
    use mockito::{mock, Matcher};
    use serde_json::json;

    use crate::client::*;
    use crate::mock_cert_client;
    use crate::tests::MockCredential;

    fn diff(first: DateTime<Utc>, second: DateTime<Utc>) -> Duration {
        if first > second {
            first - second
        } else {
            second - first
        }
    }

    #[tokio::test]
    async fn get_certificate() {
        let time_created = Utc::now() - Duration::days(7);
        let time_updated = Utc::now();
        let _m = mock("GET", "/certificates/test-certificate/")
            .match_query(Matcher::UrlEncoded("api-version".into(), API_VERSION.into()))
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "id": "https://test-keyvault.vault.azure.net/certificates/test-certificate/002ade539442463aba45c0efb42e3e84",
                    "x5t": "fLi3U52HunIVNXubkEnf8tP6Wbo",
                    "kid": "https://test-keyvault.vault.azure.net/keys/test-certificate/002ade539442463aba45c0efb42e3e84",
                    "sid": "https://test-keyvault.vault.azure.net/secrets/test-certificate/002ade539442463aba45c0efb42e3e84",
                    "cer": "MIICODCCAeagAwIBAgIQqHmpBAv+CY9IJFoUhlbziTAJBgUrDgMCHQUAMBYxFDASBgNVBAMTC1Jvb3QgQWdlbmN5MB4XDTE1MDQyOTIxNTM0MVoXDTM5MTIzMTIzNTk1OVowFzEVMBMGA1UEAxMMS2V5VmF1bHRUZXN0MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA5bVAT73zr4+N4WVv2+SvTunAw08ksS4BrJW/nNliz3S9XuzMBMXvmYzU5HJ8TtEgluBiZZYd5qsMJD+OXHSNbsLdmMhni0jYX09h3XlC2VJw2sGKeYF+xEaavXm337aZZaZyjrFBrrUl51UePaN+kVFXNlBb3N3TYpqa7KokXenJQuR+i9Gv9a77c0UsSsDSryxppYhKK7HvTZCpKrhVtulF5iPMswWe9np3uggfMamyIsK/0L7X9w9B2qN7993RR0A00nOk4H6CnkuwO77dSsD0KJsk6FyAoZBzRXDZh9+d9R76zCL506NcQy/jl0lCiQYwsUX73PG5pxOh02OwKwIDAQABo0swSTBHBgNVHQEEQDA+gBAS5AktBh0dTwCNYSHcFmRjoRgwFjEUMBIGA1UEAxMLUm9vdCBBZ2VuY3mCEAY3bACqAGSKEc+41KpcNfQwCQYFKw4DAh0FAANBAGqIjo2geVagzuzaZOe1ClGKhZeiCKfWAxklaGN+qlGUbVS4IN4V1lot3VKnzabasmkEHeNxPwLn1qvSD0cX9CE=",
                    "attributes": {
                        "enabled": true,
                        "created": time_created.timestamp(),
                        "updated": time_updated.timestamp(),
                        "recoveryLevel": "Recoverable+Purgeable"
                    },
                    "policy": {
                        "id": "https://test-keyvault.vault.azure.net/certificates/selfSignedCert01/policy",
                        "key_props": {
                            "exportable": true,
                            "kty": "RSA",
                            "key_size": 2048,
                            "reuse_key": false
                        },
                        "secret_props": {
                            "contentType": "application/x-pkcs12"
                        },
                        "x509_props": {
                            "subject": "CN=KeyVaultTest",
                            "ekus": [],
                            "key_usage": [],
                            "validity_months": 297
                        },
                        "issuer": {
                            "name": "Unknown"
                        },
                        "attributes": {
                            "enabled": true,
                            "created": 1493938289,
                            "updated": 1493938291
                        }
                    }
                })
                .to_string(),
            )
            .with_status(200)
            .create();

        let creds = MockCredential;
        dbg!(mockito::server_url());
        let mut client = mock_cert_client!(&"test-keyvault", &creds,);

        let certificate: KeyVaultCertificate =
            client.get_certificate("test-certificate").await.unwrap();

        assert_eq!(
            "https://test-keyvault.vault.azure.net/keys/test-certificate/002ade539442463aba45c0efb42e3e84",
            certificate.key_id()
        );
        assert!(*certificate.properties.enabled());
        assert!(diff(time_created, *certificate.properties.created_on()) < Duration::seconds(1));
        assert!(diff(time_updated, *certificate.properties.updated_on()) < Duration::seconds(1));
    }

    #[tokio::test]
    async fn get_certificate_versions() {
        let time_created_1 = Utc::now() - Duration::days(7);
        let time_updated_1 = Utc::now();
        let time_created_2 = Utc::now() - Duration::days(9);
        let time_updated_2 = Utc::now() - Duration::days(2);

        let _m1 = mock("GET", "/certificates/test-certificate/versions")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api-version".into(), API_VERSION.into()),
            ]))
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "value": [{
                        "id": "https://test-keyvault.vault.azure.net/certificates/test-certificate/VERSION_1",
                        "x5t": "fLi3U52HunIVNXubkEnf8tP6Wbo",
                        "attributes": {
                            "enabled": true,
                            "created": time_created_1.timestamp(),
                            "updated": time_updated_1.timestamp(),
                        }
                    }],
                    "nextLink": format!("{}/certificates/text-certificate/versions?api-version={}&maxresults=1&$skiptoken=SKIP_TOKEN_MOCK", mockito::server_url(), API_VERSION)
                })
                .to_string(),
            )
            .with_status(200)
            .create();

        let _m2 = mock("GET", "/certificates/text-certificate/versions")
            .match_query(Matcher::AllOf(vec![
                Matcher::UrlEncoded("api-version".into(), API_VERSION.into()),
                Matcher::UrlEncoded("maxresults".into(), "1".into()),
                Matcher::UrlEncoded("$skiptoken".into(), "SKIP_TOKEN_MOCK".into()),
            ]))
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "value": [{
                        "id": "https://test-keyvault.vault.azure.net/certificates/test-certificate/VERSION_2",
                        "x5t": "fLi3U52HunIVNXubkEnf8tP6Wbo",
                        "attributes": {
                            "enabled": true,
                            "created": time_created_2.timestamp(),
                            "updated": time_updated_2.timestamp(),
                        }
                    }],
                    "nextLink": null
                })
                .to_string(),
            )
            .with_status(200)
            .create();

        let creds = MockCredential;
        let mut client = mock_cert_client!(&"test-keyvault", &creds,);

        let certificate_versions = client
            .list_properties_of_certificate_versions("test-certificate")
            .await
            .unwrap();

        let certificate_1 = &certificate_versions[0];
        assert_eq!(
            "https://test-keyvault.vault.azure.net/certificates/test-certificate/VERSION_1",
            certificate_1.id()
        );
        assert!(diff(time_created_1, *certificate_1.created_on()) < Duration::seconds(1));
        assert!(diff(time_updated_1, *certificate_1.updated_on()) < Duration::seconds(1));

        let certificate_2 = &certificate_versions[1];
        assert_eq!(
            "https://test-keyvault.vault.azure.net/certificates/test-certificate/VERSION_2",
            certificate_2.id()
        );
        assert!(diff(time_created_2, *certificate_2.created_on()) < Duration::seconds(1));
        assert!(diff(time_updated_2, *certificate_2.updated_on()) < Duration::seconds(1));
    }
}
