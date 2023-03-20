use azure_core::auth::TokenCredential;
use futures::{executor::block_on, stream::StreamExt};
use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]
pub struct ConfigurationExplorer {
    client: azure_svc_appconfiguration::Client,
}

impl ConfigurationExplorer {
    pub fn new(endpoint: &str, token_credential: Arc<dyn TokenCredential>) -> Self {
        let scopes = &["https://azconfig.io"];
        let client = azure_svc_appconfiguration::Client::builder(token_credential)
            .endpoint(endpoint)
            .scopes(scopes)
            .build();

        Self { client }
    }

    pub async fn get_value_async(&self, key: String) -> String {
        let value = match self.client.clone().get_key_value(key).await {
            Ok(rs) => rs.value,
            Err(err) => {
                log::debug!("*ERROR :  {:?}", err);
                None
            }
        };

        match value {
            Some(value) => value,
            None => String::from(""),
        }
    }

    pub fn get_value(&self, key: String) -> String {
        block_on(self.get_value_async(key))
    }

    pub async fn get_values_async(&self, label: String) -> HashMap<String, String> {
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
                        .map(|it| match (&it.key, &it.value) {
                            (Some(key), Some(value)) => (key.to_string(), value.to_string()),
                            _ => (String::from(""), String::from("")),
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

    pub fn get_values(&self, label: String) -> HashMap<String, String> {
        block_on(self.get_values_async(label))
    }
}
