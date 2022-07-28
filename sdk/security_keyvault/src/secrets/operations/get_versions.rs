use crate::{prelude::*, secrets::models::*};
use azure_core::{headers::Headers, CollectedResponse, Method};
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

            println!("uri: {}", uri);
            loop {
                let headers = Headers::new();
                let mut request =
                    self.client
                        .client
                        .finalize_request(uri, Method::Get, headers, None)?;

                let response = self
                    .client
                    .client
                    .send(&mut self.context, &mut request)
                    .await?;

                let response = CollectedResponse::from_response(response).await?;
                let body = response.body();

                let response = serde_json::from_slice::<KeyVaultGetSecretsResponse>(body)?;

                secret_versions.extend(
                    response
                        .value
                        .into_iter()
                        .map(|s| KeyVaultSecretBaseIdentifier {
                            id: s.id.to_owned(),
                            name: s.id.split('/').last().unwrap().to_owned(),
                            enabled: s.attributes.enabled,
                            created_on: s.attributes.created,
                            updated_on: s.attributes.updated,
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
                if a.updated_on > b.updated_on {
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
