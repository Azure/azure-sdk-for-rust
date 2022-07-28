use crate::prelude::*;

operation! {
    GetKey,
    client: KeyClient,
    name: String,
    ?version: String
}

impl GetKeyBuilder {
    pub fn into_future(mut self) -> GetKey {
        Box::pin(async move {
            let mut uri = self.client.client.vault_url.clone();
            let version = self.version.unwrap_or_default();
            let path = format!("keys/{}/{}", self.name, version);
            uri.set_path(&path);
            uri.set_query(Some(API_VERSION_PARAM));

            let resp_body = self
                .client
                .client
                .request(reqwest::Method::GET, uri.to_string(), None)
                .await?;
            let response = serde_json::from_str::<KeyVaultKey>(&resp_body)?;
            Ok(response)
        })
    }
}

type GetKeyResponse = KeyVaultKey;
