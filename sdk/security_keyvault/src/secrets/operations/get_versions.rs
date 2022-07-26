use crate::prelude::*;
use crate::secrets::models::*;
use azure_core::error::{ErrorKind, ResultExt};
use url::Url;

operation! {
    GetSecretVersions,
    client: SecretClient,
    name: String,
}

impl GetSecretVersionsBuilder {
    pub fn into_future(mut self) -> GetSecretVersions {
        Box::pin(async move {
            let mut secret_versions = Vec::<KeyVaultSecretBaseIdentifier>::new();

            let mut uri = self.client.client.vault_url.clone();
            uri.set_path(&format!("secrets/{}/versions", self.name));
            uri.set_query(Some(API_VERSION_PARAM));

            loop {
                let resp_body = self
                    .client
                    .client
                    .request(reqwest::Method::GET, uri.to_string(), None)
                    .await?;
                let response = serde_json::from_str::<KeyVaultGetSecretsResponse>(&resp_body)
                    .with_context(ErrorKind::DataConversion, || {
                        format!(
                            "failed to parse KeyVaultGetSecretsResponse. resp_body: {resp_body}"
                        )
                    })?;

                secret_versions.extend(
                    response
                        .value
                        .into_iter()
                        .map(|s| KeyVaultSecretBaseIdentifier {
                            id: s.id.to_owned(),
                            name: s.id.split('/').last().unwrap().to_owned(),
                            enabled: s.attributes.enabled,
                            time_created: s.attributes.created,
                            time_updated: s.attributes.updated,
                        })
                        .collect::<Vec<KeyVaultSecretBaseIdentifier>>(),
                );
                match response.next_link {
                    None => break,
                    Some(u) => uri = Url::parse(&u)?,
                }
            }

            // Return the secret versions sorted by the time modified in descending order.
            secret_versions.sort_by(|a, b| {
                if a.time_updated > b.time_updated {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });
            Ok(secret_versions)
        })
    }
}

type GetSecretVersionsResponse = Vec<KeyVaultSecretBaseIdentifier>;
