use crate::prelude::*;
use azure_core::{headers::Headers, CollectedResponse, Method};
use serde_json::{Map, Value};

operation! {
    UnwrapKey,
    client: KeyClient,
    name: String,
    unwrap_key_parameters: UnwrapKeyParameters,
    ?version: String
}

impl UnwrapKeyBuilder {
    pub fn into_future(self) -> UnwrapKey {
        Box::pin(async move {
            // POST {vaultBaseUrl}/keys/{key-name}/{key-version}/decrypt?api-version=7.2
            let version = self.version.unwrap_or_default();
            let mut uri = self.client.keyvault_client.vault_url.clone();
            let path = format!("keys/{}/{}/unwrapkey", self.name, version);

            uri.set_path(&path);

            let mut request_body = Map::new();
            request_body.insert(
                "value".to_owned(),
                Value::String(String::from_utf8(self.unwrap_key_parameters.ciphertext)?),
            );

            let algorithm = match self.unwrap_key_parameters.decrypt_parameters_encryption {
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

            let mut result = serde_json::from_slice::<UnwrapKeyResult>(body)?;
            result.algorithm = algorithm;
            Ok(result)
        })
    }
}

type UnwrapKeyResponse = UnwrapKeyResult;
