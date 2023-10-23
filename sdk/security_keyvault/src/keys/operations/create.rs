use crate::{models::JsonWebKeyType, prelude::*};
use azure_core::{headers::Headers, CollectedResponse, Method};
use serde::Serialize;

operation! {
    Create,
    client: KeyClient,
    name: String,
    kty: JsonWebKeyType,
    ?key_size: i32
}

#[derive(Serialize, Debug)]
struct CreateRequest {
    kty: JsonWebKeyType,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_size: Option<i32>,
}

impl CreateBuilder {
    pub fn into_future(mut self) -> Create {
        Box::pin(async move {
            // POST {vaultBaseUrl}/keys/{key-name}/create?api-version=7.2
            let mut uri = self.client.keyvault_client.vault_url.clone();
            uri.set_path(&format!("keys/{}/create", self.name));

            let request = CreateRequest {
                kty: self.kty,
                key_size: self.key_size,
            };

            let body = serde_json::to_string(&request)?;

            let headers = Headers::new();
            let mut request =
                KeyvaultClient::finalize_request(uri, Method::Post, headers, Some(body.into()));

            let response = self
                .client
                .keyvault_client
                .send(&mut self.context, &mut request)
                .await?;

            let response = CollectedResponse::from_response(response).await?;
            let body = response.body();
            let result = serde_json::from_slice::<CreateResponse>(body)?;
            Ok(result)
        })
    }
}

type CreateResponse = KeyVaultKey;
