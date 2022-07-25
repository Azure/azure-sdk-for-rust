use crate::prelude::*;

operation! {
    GetKey,
    client: KeyClient,
    ?version: String
}

impl GetKeyBuilder {
    pub fn into_future(mut self) -> GetKey {
        Box::pin(async move {
            let mut uri = self.client.client.vault_url.clone();
            let path = if let Some(ver) = self.version {
                format!("keys/{}/{}", self.client.name, ver)
            } else {
                format!("keys/{}", self.client.name)
            };
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
