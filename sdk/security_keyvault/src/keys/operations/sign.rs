use crate::prelude::*;
use serde_json::{Map, Value};

operation! {
    Sign,
    client: KeyClient,
    name: String,
    algorithm: SignatureAlgorithm,
    digest: String,
    ?version: String
}

impl SignBuilder {
    pub fn into_future(mut self) -> Sign {
        Box::pin(async move {
            // POST {vaultBaseUrl}/keys/{key-name}/{key-version}/sign?api-version=7.1
            let version = self.version.unwrap_or_default();
            let mut uri = self.client.client.vault_url.clone();
            uri.set_path(&format!("keys/{}/{}/sign", self.name, version));
            uri.set_query(Some(API_VERSION_PARAM));

            let mut request_body = Map::new();
            request_body.insert("alg".to_owned(), Value::String(self.algorithm.to_string()));
            request_body.insert("value".to_owned(), Value::String(self.digest.to_owned()));

            let response = self
                .client
                .client
                .request(
                    reqwest::Method::POST,
                    uri.to_string(),
                    Some(Value::Object(request_body).to_string()),
                )
                .await?;

            let mut result = serde_json::from_str::<SignResult>(&response)?;
            result.algorithm = self.algorithm;
            Ok(result)
        })
    }
}

type SignResponse = SignResult;
