use crate::prelude::*;
use azure_core::{headers::Headers, CollectedResponse, Method};
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
    pub fn into_future(self) -> Sign {
        Box::pin(async move {
            // POST {vaultBaseUrl}/keys/{key-name}/{key-version}/sign?api-version=7.1
            let version = self.version.unwrap_or_default();
            let mut uri = self.client.keyvault_client.vault_url.clone();
            uri.set_path(&format!("keys/{}/{}/sign", self.name, version));

            let mut request_body = Map::new();
            request_body.insert("alg".to_owned(), Value::String(self.algorithm.to_string()));
            request_body.insert("value".to_owned(), Value::String(self.digest.to_owned()));

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

            let mut result = serde_json::from_slice::<SignResult>(body)?;
            result.algorithm = self.algorithm;
            Ok(result)
        })
    }
}

type SignResponse = SignResult;
