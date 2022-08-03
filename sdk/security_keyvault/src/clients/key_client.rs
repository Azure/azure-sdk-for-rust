use crate::prelude::*;
use azure_core::auth::TokenCredential;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct KeyClient {
    pub(crate) client: KeyvaultClient,
}

impl KeyClient {
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

    /// Gets the public part of a stored key.
    /// The get key operation is applicable to all key types.
    /// If the requested key is symmetric, then no key material is released in the response.
    /// This operation requires the keys/get permission.
    ///
    /// GET {vaultBaseUrl}/keys/{key-name}/{key-version}?api-version=7.1
    pub fn get<N>(&self, name: N) -> GetKeyBuilder
    where
        N: Into<String>,
    {
        GetKeyBuilder::new(self.clone(), name.into())
    }

    /// Creates a signature from a digest using the specified key.
    ///
    /// The SIGN operation is applicable to asymmetric and symmetric keys stored
    /// in Azure Key Vault since this operation uses the private portion of the
    /// key.
    ///
    /// This operation requires the keys/sign permission.
    pub fn sign<N, D>(&self, name: N, algorithm: SignatureAlgorithm, digest: D) -> SignBuilder
    where
        N: Into<String>,
        D: Into<String>,
    {
        SignBuilder::new(self.clone(), name.into(), algorithm, digest.into())
    }

    /// Decrypt a single block of encrypted data.
    ///
    /// The DECRYPT operation decrypts a well-formed block of ciphertext using
    /// the target encryption key and specified algorithm.
    //
    /// This operation is the reverse of the ENCRYPT operation; only a single
    /// block of data may be decrypted, the size of this block is dependent on
    /// the target key and the algorithm to be used.
    ///
    /// The DECRYPT operation applies to asymmetric and symmetric keys stored in
    /// Vault or HSM since it uses the private portion of the key. This
    /// operation requires the keys/decrypt permission.
    pub fn decrypt<N>(&self, name: N, decrypt_parameters: DecryptParameters) -> DecryptBuilder
    where
        N: Into<String>,
    {
        DecryptBuilder::new(self.clone(), name.into(), decrypt_parameters)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock_client;
    use crate::tests::MockCredential;
    use azure_core::date;
    use mockito::{mock, Matcher};
    use serde_json::json;
    use std::time::Duration;
    use time::OffsetDateTime;

    #[tokio::test]
    async fn can_get_key() -> azure_core::Result<()> {
        let time_created = OffsetDateTime::now_utc() - date::duration_from_days(7);
        let time_updated = OffsetDateTime::now_utc();
        let _m = mock("GET", "/keys/test-key/78deebed173b48e48f55abf87ed4cf71")
            .match_query(Matcher::UrlEncoded("api-version".into(), API_VERSION.into()))
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "key": {
                        "kid": "https://test-keyvault.vault.azure.net/keys/test-key/78deebed173b48e48f55abf87ed4cf71",
                        "kty": "RSA",
                        "key_ops": [
                            "encrypt",
                            "decrypt",
                            "sign",
                            "verify",
                            "wrapKey",
                            "unwrapKey",
                            "destroy!"
                        ],
                        "n": "2HJAE5fU3Cw2Rt9hEuq-F6XjINKGa-zskfISVqopqUy60GOs2eyhxbWbJBeUXNor_gf-tXtNeuqeBgitLeVa640UDvnEjYTKWjCniTxZRaU7ewY8BfTSk-7KxoDdLsPSpX_MX4rwlAx-_1UGk5t4sQgTbm9T6Fm2oqFd37dsz5-Gj27UP2GTAShfJPFD7MqU_zIgOI0pfqsbNL5xTQVM29K6rX4jSPtylZV3uWJtkoQIQnrIHhk1d0SC0KwlBV3V7R_LVYjiXLyIXsFzSNYgQ68ZjAwt8iL7I8Osa-ehQLM13DVvLASaf7Jnu3sC3CWl3Gyirgded6cfMmswJzY87w",
                        "e": "AQAB"
                    },
                    "attributes": {
                        "enabled": true,
                        "created": time_created.unix_timestamp(),
                        "updated": time_updated.unix_timestamp(),
                        "recoveryLevel": "Recoverable+Purgeable"
                      },
                    "tags": {
                        "purpose": "unit test",
                        "test name ": "CreateGetDeleteKeyTest"
                    }
                })
                .to_string(),
            )
            .with_status(200)
            .create();

        let client = mock_client!(&"test-keyvault", MockCredential::new());

        let key = client
            .key_client()
            .get("test-key")
            .version("78deebed173b48e48f55abf87ed4cf71")
            .into_future()
            .await?;

        let JsonWebKey { id, n, .. } = key.key;
        let KeyProperties {
            attributes,
            managed,
            tags,
        } = key.properties;
        let KeyAttributes {
            created_on,
            enabled,
            updated_on,
            ..
        } = attributes;
        let expected_n = base64::decode_config("2HJAE5fU3Cw2Rt9hEuq-F6XjINKGa-zskfISVqopqUy60GOs2eyhxbWbJBeUXNor_gf-tXtNeuqeBgitLeVa640UDvnEjYTKWjCniTxZRaU7ewY8BfTSk-7KxoDdLsPSpX_MX4rwlAx-_1UGk5t4sQgTbm9T6Fm2oqFd37dsz5-Gj27UP2GTAShfJPFD7MqU_zIgOI0pfqsbNL5xTQVM29K6rX4jSPtylZV3uWJtkoQIQnrIHhk1d0SC0KwlBV3V7R_LVYjiXLyIXsFzSNYgQ68ZjAwt8iL7I8Osa-ehQLM13DVvLASaf7Jnu3sC3CWl3Gyirgded6cfMmswJzY87w", BASE64_URL_SAFE).unwrap();
        assert_eq!(expected_n, n.unwrap());
        assert_eq!(
            "https://test-keyvault.vault.azure.net/keys/test-key/78deebed173b48e48f55abf87ed4cf71",
            id.unwrap()
        );

        assert!(managed.is_none());
        assert_eq!(tags.unwrap().get("purpose").unwrap(), "unit test");
        assert!(enabled.unwrap());
        assert!(date::diff(time_created, created_on.unwrap()) < Duration::from_secs(1));
        assert!(date::diff(time_updated, updated_on.unwrap()) < Duration::from_secs(1));
        Ok(())
    }

    #[tokio::test]
    async fn can_sign() {
        let _m = mock("POST", "/keys/test-key/78deebed173b48e48f55abf87ed4cf71/sign")
            .match_query(Matcher::UrlEncoded("api-version".into(), API_VERSION.into()))
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "kid": "https://myvault.vault.azure.net/keys/testkey/9885aa558e8d448789683188f8c194b0",
                    "value": "aKFG8NXcfTzqyR44rW42484K_zZI_T7zZuebvWuNgAoEI1gXYmxrshp42CunSmmu4oqo4-IrCikPkNIBkHXnAW2cv03Ad0UpwXhVfepK8zzDBaJPMKVGS-ZRz8CshEyGDKaLlb3J3zEkXpM3RrSEr0mdV6hndHD_mznLB5RmFui5DsKAhez4vUqajgtkgcPfCekMqeSwp6r9ItVL-gEoAohx8XMDsPedqu-7BuZcBcdayaPuBRL4wWoTDULA11P-UN_sJ5qMj3BbiRYhIlBWGR04wIGfZ3pkJjHJUpOvgH2QajdYPzUBauOCewMYbq9XkLRSzI_A7HkkDVycugSeAA"
                })
                .to_string(),
            )
            .with_status(200)
            .create();

        let creds = MockCredential::new();
        let client = mock_client!(&"test-keyvault", creds).key_client();

        let res = client
            .sign("test-key", SignatureAlgorithm::RS512, "base64msg2sign")
            .version("78deebed173b48e48f55abf87ed4cf71")
            .into_future()
            .await
            .unwrap();

        let kid = res.key_id;
        let sig = res.signature;
        let alg = res.algorithm;

        assert_eq!(
            kid,
            "https://myvault.vault.azure.net/keys/testkey/9885aa558e8d448789683188f8c194b0"
        );
        let expected_sig = base64::decode_config("aKFG8NXcfTzqyR44rW42484K_zZI_T7zZuebvWuNgAoEI1gXYmxrshp42CunSmmu4oqo4-IrCikPkNIBkHXnAW2cv03Ad0UpwXhVfepK8zzDBaJPMKVGS-ZRz8CshEyGDKaLlb3J3zEkXpM3RrSEr0mdV6hndHD_mznLB5RmFui5DsKAhez4vUqajgtkgcPfCekMqeSwp6r9ItVL-gEoAohx8XMDsPedqu-7BuZcBcdayaPuBRL4wWoTDULA11P-UN_sJ5qMj3BbiRYhIlBWGR04wIGfZ3pkJjHJUpOvgH2QajdYPzUBauOCewMYbq9XkLRSzI_A7HkkDVycugSeAA", BASE64_URL_SAFE).unwrap();
        assert_eq!(expected_sig, sig);
        assert!(matches!(alg, SignatureAlgorithm::RS512));
    }

    #[tokio::test]
    async fn can_decrypt() {
        let _m = mock("POST", "/keys/test-key/78deebed173b48e48f55abf87ed4cf71/decrypt")
            .match_query(Matcher::UrlEncoded("api-version".into(), API_VERSION.into()))
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "kid": "https://myvault.vault.azure.net/keys/test-key/78deebed173b48e48f55abf87ed4cf71",
                    "value": "dvDmrSBpjRjtYg"
                  })
                .to_string(),
            )
            .with_status(200)
            .create();

        let creds = MockCredential::new();
        let client = mock_client!(&"test-keyvault", creds);

        let decrypt_parameters = DecryptParameters {
            ciphertext: base64::decode("dvDmrSBpjRjtYg").unwrap(),
            decrypt_parameters_encryption: DecryptParametersEncryption::Rsa(
                RsaDecryptParameters::new(EncryptionAlgorithm::RsaOaep256).unwrap(),
            ),
        };

        let res: DecryptResult = client
            .key_client()
            .decrypt("test-key", decrypt_parameters)
            .version("78deebed173b48e48f55abf87ed4cf71")
            .into_future()
            .await
            .unwrap();

        let kid = res.key_id;
        let val = res.result;
        let alg = res.algorithm;

        assert_eq!(
            kid,
            "https://myvault.vault.azure.net/keys/test-key/78deebed173b48e48f55abf87ed4cf71"
        );
        let expected_val = base64::decode_config("dvDmrSBpjRjtYg", BASE64_URL_SAFE).unwrap();
        assert_eq!(expected_val, val);

        assert!(matches!(alg, EncryptionAlgorithm::RsaOaep256));
    }
}
