use azure_core::auth::TokenCredential;
use futures::stream::StreamExt;
use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]
pub struct ConfigurationExplorer {
    client: azure_svc_appconfiguration::Client,
}

impl ConfigurationExplorer {
    pub fn new(endpoint: impl Into<String>, token_credential: Arc<dyn TokenCredential>) -> Self {
        let scopes = &["https://azconfig.io"];
        let client = azure_svc_appconfiguration::Client::builder(token_credential)
            .endpoint(endpoint)
            .scopes(scopes)
            .build();

        Self { client }
    }

    pub async fn get_value(&self, key: impl Into<String>) -> Option<String> {
        match self.client.clone().get_key_value(key).await {
            Ok(rs) => rs.value,
            Err(err) => {
                log::debug!("*ERROR :  {:?}", err);
                None
            }
        }
    }

    pub async fn get_values(&self, label: impl Into<String>) -> HashMap<String, String> {
        let mut map = HashMap::new();
        let mut stream = self
            .client
            .clone()
            .get_key_values()
            .label(label)
            .into_stream();
        while let Some(rs) = stream.next().await {
            match rs {
                Ok(rs) => {
                    let items = rs
                        .items
                        .iter()
                        .flat_map(|it| match (&it.key, &it.value) {
                            (Some(key), Some(value)) => Some((key.to_string(), value.to_string())),
                            _ => None,
                        })
                        .collect::<HashMap<String, String>>();

                    map.extend(items);
                }
                Err(err) => {
                    log::debug!("*ERROR :  {:?}", err)
                }
            }
        }
        map
    }
}
