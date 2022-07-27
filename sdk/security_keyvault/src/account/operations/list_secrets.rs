use crate::prelude::*;
use azure_core::error::{ErrorKind, ResultExt};
use url::Url;

operation! {
    ListSecrets,
    client: SecretClient,
}

impl ListSecretsBuilder {
    pub fn into_future(mut self) -> ListSecrets {
        Box::pin(async move {
            let mut secrets = Vec::<KeyVaultSecretBaseIdentifier>::new();

            let mut uri = self.client.client.vault_url.clone();
            uri.set_path("secrets");

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

                secrets.extend(
                    response
                        .value
                        .into_iter()
                        .map(|s| KeyVaultSecretBaseIdentifier {
                            id: s.id.clone(),
                            name: s.id.split('/').last().unwrap().to_owned(),
                            enabled: s.attributes.enabled,
                            created_on: s.attributes.created,
                            updated_on: s.attributes.updated,
                        })
                        .collect::<Vec<KeyVaultSecretBaseIdentifier>>(),
                );

                match response.next_link {
                    None => break,
                    Some(u) => uri = Url::parse(&u).unwrap(),
                }
            }

            Ok(secrets)
        })
    }
}

type ListSecretsResponse = Vec<KeyVaultSecretBaseIdentifier>;
