use crate::prelude::*;
use azure_core::auth::TokenCredential;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct CertificateClient {
    pub(crate) client: KeyvaultClient,
}

impl CertificateClient {
    pub fn new(
        vault_url: &str,
        token_credential: Arc<dyn TokenCredential>,
    ) -> azure_core::Result<Self> {
        let client = KeyvaultClient::new(vault_url, token_credential)?;
        Ok(Self { client })
    }

    pub(crate) fn new_with_client(client: KeyvaultClient) -> Self {
        Self { client }
    }

    /// Gets a certificate from the Key Vault.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_security_keyvault::KeyvaultClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use tokio::runtime::Runtime;
    /// use std::sync::Arc;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    ///     let mut client = KeyvaultClient::new(
    ///         &"KEYVAULT_URL",
    ///         Arc::new(creds),
    ///     ).unwrap().certificate_client();
    ///     let certificate = client.get("NAME").into_future().await.unwrap();
    ///     dbg!(&certificate);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn get<N>(&self, name: N) -> GetCertificateBuilder
    where
        N: Into<String>,
    {
        GetCertificateBuilder::new(self.clone(), name.into())
    }

    /// Gets all the versions for a certificate in the Key Vault.
    //
    /// # Example
    ///
    /// ```no_run
    /// use azure_security_keyvault::KeyvaultClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use tokio::runtime::Runtime;
    /// use std::sync::Arc;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    ///     let mut client = KeyvaultClient::new(
    ///         &"KEYVAULT_URL",
    ///         Arc::new(creds),
    ///     ).unwrap().certificate_client();
    ///     let certificate_versions = client.get_versions("NAME").into_future().await.unwrap();
    ///     dbg!(&certificate_versions);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn get_versions<N>(&self, name: N) -> GetCertificateVersionsBuilder
    where
        N: Into<String>,
    {
        GetCertificateVersionsBuilder::new(self.clone(), name.into())
    }

    pub fn update<N>(&self, name: N) -> UpdateCertificatePropertiesBuilder
    where
        N: Into<String>,
    {
        UpdateCertificatePropertiesBuilder::new(self.clone(), name.into())
    }

    /// Restores a backed up certificate and all its versions.
    /// This operation requires the certificates/restore permission.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_security_keyvault::KeyvaultClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use tokio::runtime::Runtime;
    /// use std::sync::Arc;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    ///     let mut client = KeyvaultClient::new(
    ///         &"KEYVAULT_URL",
    ///         Arc::new(creds),
    ///     ).unwrap().certificate_client();
    ///     client.backup("NAME").into_future().await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn backup<N>(&self, name: N) -> CertificateBackupBuilder
    where
        N: Into<String>,
    {
        CertificateBackupBuilder::new(self.clone(), name.into())
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
    /// use azure_security_keyvault::KeyvaultClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use tokio::runtime::Runtime;
    /// use std::sync::Arc;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    ///     let mut client = KeyvaultClient::new(
    ///         &"KEYVAULT_URL",
    ///         Arc::new(creds),
    ///     ).unwrap().certificate_client();
    ///     client.delete("NAME").await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub async fn delete<N>(&self, name: N) -> azure_core::Result<()>
    where
        N: Into<String>,
    {
        // let mut uri = self.vault_url.clone();
        // uri.set_path(&format!("certificates/{}", certificate_name));
        // uri.set_query(Some(API_VERSION_PARAM));

        // self.delete_authed(uri.to_string()).await?;

        // Ok(())

        let _name = name.into();

        todo!("See issue #174 at: https://github.com/Azure/azure-sdk-for-rust/issues/174.")
    }

    /// Lists all the certificates in the Key Vault.
    ///
    /// ```no_run
    /// use azure_security_keyvault::KeyvaultClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use tokio::runtime::Runtime;
    /// use std::sync::Arc;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    ///     let mut client = KeyvaultClient::new(
    ///          &"KEYVAULT_URL",
    ///          Arc::new(creds),
    ///     ).unwrap().certificate_client();
    ///     let certificates = client.list_certificates().into_future().await.unwrap();
    ///     dbg!(&certificates);
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn list_certificates(&self) -> ListCertificatesBuilder {
        ListCertificatesBuilder::new(self.clone())
    }

    /// Restores a backed up certificate and all its versions.
    /// This operation requires the certificates/restore permission.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use azure_security_keyvault::KeyvaultClient;
    /// use azure_identity::DefaultAzureCredential;
    /// use tokio::runtime::Runtime;
    /// use std::sync::Arc;
    ///
    /// async fn example() {
    ///     let creds = DefaultAzureCredential::default();
    ///     let mut client = KeyvaultClient::new(
    ///         &"KEYVAULT_URL",
    ///         Arc::new(creds),
    ///     ).unwrap().certificate_client();
    ///     client.restore_certificate("KUF6dXJlS2V5VmF1bHRTZWNyZXRCYWNrdXBWMS5taW").into_future().await.unwrap();
    /// }
    ///
    /// Runtime::new().unwrap().block_on(example());
    /// ```
    pub fn restore_certificate<S>(&self, backup_blob: S) -> RestoreCertificateBuilder
    where
        S: Into<String>,
    {
        RestoreCertificateBuilder::new(self.clone(), backup_blob.into())
    }
}

#[cfg(test)]
#[allow(unused_must_use)]
mod tests {
    use crate::mock_client;
    use crate::prelude::*;
    use crate::tests::MockCredential;
    use azure_core::date;
    use mockito::{mock, Matcher};
    use serde_json::json;
    use std::time::Duration;
    use time::OffsetDateTime;

    #[tokio::test]
    async fn get_certificate() {
        let time_created = OffsetDateTime::now_utc() - date::duration_from_days(7);
        let time_updated = OffsetDateTime::now_utc();
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
                        "created": time_created.unix_timestamp(),
                        "updated": time_updated.unix_timestamp(),
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

        let creds = MockCredential::new();
        dbg!(mockito::server_url());
        let client = mock_client!(&"test-keyvault", creds);

        let certificate = client
            .certificate_client()
            .get("test-certificate")
            .into_future()
            .await
            .unwrap();

        assert_eq!(
            "https://test-keyvault.vault.azure.net/keys/test-certificate/002ade539442463aba45c0efb42e3e84",
            certificate.key_id
        );
        assert!(
            date::diff(time_created, certificate.properties.created_on) < Duration::from_secs(1)
        );
        assert!(
            date::diff(time_updated, certificate.properties.updated_on) < Duration::from_secs(1)
        );
    }

    #[tokio::test]
    async fn get_certificate_versions() {
        let time_created_1 = OffsetDateTime::now_utc() - date::duration_from_days(7);
        let time_updated_1 = OffsetDateTime::now_utc();
        let time_created_2 = OffsetDateTime::now_utc() - date::duration_from_days(9);
        let time_updated_2 = OffsetDateTime::now_utc() - date::duration_from_days(2);

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
                            "created": time_created_1.unix_timestamp(),
                            "updated": time_updated_1.unix_timestamp(),
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
                            "created": time_created_2.unix_timestamp(),
                            "updated": time_updated_2.unix_timestamp(),
                        }
                    }],
                    "nextLink": null
                })
                .to_string(),
            )
            .with_status(200)
            .create();

        let creds = MockCredential::new();
        let client = mock_client!(&"test-keyvault", creds);

        let certificate_versions = client
            .certificate_client()
            .get_versions("test-certificate")
            .into_future()
            .await
            .unwrap();

        let certificate_1 = &certificate_versions[0];
        assert_eq!(
            "https://test-keyvault.vault.azure.net/certificates/test-certificate/VERSION_1",
            certificate_1.id
        );
        assert!(date::diff(time_created_1, certificate_1.created_on) < Duration::from_secs(1));
        assert!(date::diff(time_updated_1, certificate_1.updated_on) < Duration::from_secs(1));

        let certificate_2 = &certificate_versions[1];
        assert_eq!(
            "https://test-keyvault.vault.azure.net/certificates/test-certificate/VERSION_2",
            certificate_2.id
        );
        assert!(date::diff(time_created_2, certificate_2.created_on) < Duration::from_secs(1));
        assert!(date::diff(time_updated_2, certificate_2.updated_on) < Duration::from_secs(1));
    }
}
