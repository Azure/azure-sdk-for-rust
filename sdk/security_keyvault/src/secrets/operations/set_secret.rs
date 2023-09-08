use crate::prelude::*;
use azure_core::{headers::Headers, Method};
use serde_json::{Map, Value};

operation! {
    SetSecret,
    client: SecretClient,
    name: String,
    value: String,
}

impl SetSecretBuilder {
    pub fn into_future(self) -> SetSecret {
        Box::pin(async move {
            let mut uri = self.client.keyvault_client.vault_url.clone();
            uri.set_path(&format!("secrets/{}", self.name));

            let mut request_body = Map::new();

            request_body.insert("value".to_string(), Value::String(self.value));

            let body = Some(Value::Object(request_body).to_string().into());

            let headers = Headers::new();
            let mut request =
                self.client
                    .keyvault_client
                    .finalize_request(uri, Method::Put, headers, body)?;

            self.client
                .keyvault_client
                .send(&self.context, &mut request)
                .await?;

            Ok(())
        })
    }
}

type SetSecretResponse = ();
