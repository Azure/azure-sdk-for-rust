use crate::prelude::*;
use serde_json::{Map, Value};

operation! {
    Decrypt,
    client: KeyClient,
    name: String,
    decrypt_parameters: DecryptParameters,
    ?version: String
}

impl DecryptBuilder {
    pub fn into_future(mut self) -> Decrypt {
        Box::pin(async move {
            // POST {vaultBaseUrl}/keys/{key-name}/{key-version}/decrypt?api-version=7.2
            let version = self.version.unwrap_or_default();
            let mut uri = self.client.client.vault_url.clone();
            let path = format!("keys/{}/{}/decrypt", self.name, version);

            uri.set_path(&path);
            uri.set_query(Some(API_VERSION_PARAM));

            let mut request_body = Map::new();
            request_body.insert(
                "value".to_owned(),
                Value::String(base64::encode(self.decrypt_parameters.ciphertext)),
            );

            let algorithm = match self.decrypt_parameters.decrypt_parameters_encryption {
                DecryptParametersEncryption::Rsa(RsaDecryptParameters { algorithm }) => {
                    request_body
                        .insert("alg".to_owned(), serde_json::to_value(&algorithm).unwrap());
                    algorithm
                }
                DecryptParametersEncryption::AesGcm(AesGcmDecryptParameters {
                    algorithm,
                    iv,
                    authentication_tag,
                    additional_authenticated_data,
                }) => {
                    request_body
                        .insert("alg".to_owned(), serde_json::to_value(&algorithm).unwrap());
                    request_body.insert("iv".to_owned(), serde_json::to_value(iv).unwrap());
                    request_body.insert(
                        "tag".to_owned(),
                        serde_json::to_value(authentication_tag).unwrap(),
                    );
                    if let Some(aad) = additional_authenticated_data {
                        request_body.insert("aad".to_owned(), serde_json::to_value(aad).unwrap());
                    };
                    algorithm
                }
                DecryptParametersEncryption::AesCbc(AesCbcDecryptParameters { algorithm, iv }) => {
                    request_body
                        .insert("alg".to_owned(), serde_json::to_value(&algorithm).unwrap());
                    request_body.insert("iv".to_owned(), serde_json::to_value(iv).unwrap());
                    algorithm
                }
            };

            let response = self
                .client
                .client
                .request(
                    reqwest::Method::POST,
                    uri.to_string(),
                    Some(Value::Object(request_body).to_string()),
                )
                .await?;

            let mut result = serde_json::from_str::<DecryptResult>(&response)?;
            result.algorithm = algorithm;
            Ok(result)
        })
    }
}

type DecryptResponse = DecryptResult;
