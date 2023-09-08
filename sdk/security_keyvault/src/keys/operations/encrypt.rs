use crate::prelude::*;
use azure_core::{base64, headers::Headers, CollectedResponse, Method};
use serde_json::{Map, Value};

operation! {
    Encrypt,
    client: KeyClient,
    name: String,
    encrypt_parameters: EncryptParameters,
    ?version: String
}

impl EncryptBuilder {
    pub fn into_future(self) -> Encrypt {
        Box::pin(async move {
            // POST {vaultBaseUrl}/keys/{key-name}/{key-version}/encrypt?api-version=7.2
            let version = self.version.unwrap_or_default();
            let mut uri = self.client.keyvault_client.vault_url.clone();
            let path = format!("keys/{}/{}/encrypt", self.name, version);

            uri.set_path(&path);

            let mut request_body = Map::new();
            request_body.insert(
                "value".to_owned(),
                Value::String(base64::encode(self.encrypt_parameters.plaintext)),
            );

            let algorithm = match self.encrypt_parameters.encrypt_parameters_encryption {
                CryptographParamtersEncryption::Rsa(RsaEncryptionParameters { algorithm }) => {
                    request_body.insert("alg".to_owned(), serde_json::to_value(&algorithm)?);
                    algorithm
                }
                CryptographParamtersEncryption::AesGcm(AesGcmEncryptionParameters {
                    algorithm,
                    iv,
                    authentication_tag,
                    additional_authenticated_data,
                }) => {
                    request_body.insert("alg".to_owned(), serde_json::to_value(&algorithm)?);
                    request_body.insert("iv".to_owned(), serde_json::to_value(iv)?);
                    request_body
                        .insert("tag".to_owned(), serde_json::to_value(authentication_tag)?);
                    if let Some(aad) = additional_authenticated_data {
                        request_body.insert("aad".to_owned(), serde_json::to_value(aad)?);
                    };
                    algorithm
                }
                CryptographParamtersEncryption::AesCbc(AesCbcEncryptionParameters {
                    algorithm,
                    iv,
                }) => {
                    request_body.insert("alg".to_owned(), serde_json::to_value(&algorithm)?);
                    request_body.insert("iv".to_owned(), serde_json::to_value(iv)?);
                    algorithm
                }
            };

            let headers = Headers::new();
            let mut request = self.client.keyvault_client.finalize_request(
                uri,
                Method::Post,
                headers,
                Some(Value::Object(request_body).to_string().into()),
            )?;

            let response = self
                .client
                .keyvault_client
                .send(&self.context, &mut request)
                .await?;

            let response = CollectedResponse::from_response(response).await?;
            let body = response.body();

            let mut result = serde_json::from_slice::<EncryptResult>(body)?;
            result.algorithm = algorithm;
            Ok(result)
        })
    }
}

type EncryptResponse = EncryptResult;
