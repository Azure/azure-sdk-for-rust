use crate::prelude::*;
use azure_core::error::{ErrorKind, ResultExt};
use serde_json::{Map, Value};

operation! {
    SetSecret,
    client: SecretClient,
    name: String,
    value: String,
}

impl SetSecretBuilder {
    pub fn into_future(mut self) -> SetSecret {
        Box::pin(async move {
            let mut uri = self.client.client.vault_url.clone();
            uri.set_path(&format!("secrets/{}", self.name));
            uri.set_query(Some(API_VERSION_PARAM));

            let mut request_body = Map::new();

            request_body.insert("value".to_string(), Value::String(self.value));

            self.client
                .client
                .request(
                    reqwest::Method::PUT,
                    uri.to_string(),
                    Some(Value::Object(request_body).to_string()),
                )
                .await
                .with_context(ErrorKind::Other, || {
                    format!("failed to set secret. secret_name: {}", self.name)
                })?;

            Ok(())
        })
    }
}

type SetSecretResponse = ();
