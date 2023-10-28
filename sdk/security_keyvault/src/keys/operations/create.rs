use crate::{models::JsonWebKeyType, prelude::*};
use azure_core::{headers::Headers, CollectedResponse, Method};
use serde::Serialize;

operation! {
    CreateKey,
    client: KeyClient,
    name: String,
    kty: JsonWebKeyType,
    ?crv: CurveName,
    ?attributes: KeyAttributes,
    ?key_size: i32
}

#[derive(Serialize, Debug)]
struct CreateKeyRequest {
    kty: JsonWebKeyType,
    #[serde(skip_serializing_if = "Option::is_none")]
    crv: Option<CurveName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<KeyAttributes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_size: Option<i32>,
}

impl CreateKeyBuilder {
    pub fn into_future(self) -> CreateKey {
        Box::pin(async move {
            // POST {vaultBaseUrl}/keys/{key-name}/create?api-version=7.2
            let mut uri = self.client.keyvault_client.vault_url.clone();
            uri.set_path(&format!("keys/{}/create", self.name));

            let request = CreateKeyRequest {
                kty: self.kty,
                key_size: self.key_size,
                crv: self.crv,
                attributes: self.attributes,
            };

            let body = serde_json::to_string(&request)?;

            let headers = Headers::new();
            let mut request =
                KeyvaultClient::finalize_request(uri, Method::Post, headers, Some(body.into()));

            let response = self
                .client
                .keyvault_client
                .send(&self.context, &mut request)
                .await?;

            let response = CollectedResponse::from_response(response).await?;
            let body = response.body();
            let result = serde_json::from_slice::<CreateKeyResponse>(body)?;
            Ok(result)
        })
    }
}

type CreateKeyResponse = KeyVaultKey;
