use azure_core::auth::TokenCredential;
use futures::{executor::block_on, stream::StreamExt};
use std::{collections::HashMap, sync::Arc};

const CONTENT_TYPE: &str = "application/vnd.microsoft.appconfig.ff+json;charset=utf-8";

#[derive(Clone)]
pub struct ConfigurationExplorer {
    values: HashMap<String, HashMap<String, String>>,
    client: azure_svc_appconfiguration::Client,
}

impl std::fmt::Debug for ConfigurationExplorer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConfigurationExplorer")
            .field("vaues", &self.values)
            .finish()
    }
}

impl ConfigurationExplorer {
    pub fn new(token_credential: Arc<dyn TokenCredential>) -> Self {
        let name =
            std::env::var("AZCONFIG_NAME").expect("Missing AZCONFIG_NAME environment variable.");
        let endpoint = format!("https://{name}.azconfig.io");
        let scopes = &["https://azconfig.io"];

        let client = azure_svc_appconfiguration::Client::builder(token_credential)
            .endpoint(endpoint)
            .scopes(scopes)
            .build();

        let mut stream = client.get_key_values().into_stream();
        let map = block_on(async move {
            let mut map: HashMap<String, HashMap<String, String>> = HashMap::new();

            while let Some(rs) = stream.next().await {
                match rs {
                    Ok(rs) => {
                        rs.items
                            .iter()
                            .filter(|&key| match &key.content_type {
                                Some(content_type) => !content_type.eq(CONTENT_TYPE),
                                None => false,
                            })
                            .map(|it| match &it.label {
                                Some(label) => (label.to_string(), it),
                                None => (String::from("no_label"), it),
                            })
                            .for_each(|it| {
                                let insert = match (&it.1.key, &it.1.value) {
                                    (Some(key), Some(value)) => {
                                        (key.to_string(), value.to_string())
                                    }
                                    _ => (String::from(""), String::from("")),
                                };

                                match map.get_mut(&it.0) {
                                    Some(section) => {
                                        section.insert(insert.0, insert.1);
                                    }
                                    None => {
                                        map.insert(it.0, HashMap::from([insert]));
                                    }
                                }
                            });
                    }
                    Err(err) => {
                        log::debug!("*ERROR :  {:?}", err)
                    }
                }
            }

            map
        });

        Self {
            values: map,
            client,
        }
    }

    pub fn get_value(&self, key: String) -> String {
        let value = block_on(async move {
            match self.client.clone().get_key_value(key).await {
                Ok(rs) => rs.value,
                Err(err) => {
                    log::debug!("*ERROR :  {:?}", err);
                    None
                }
            }
        });

        match value {
            Some(value) => value.clone(),
            None => String::from(""),
        }
    }

    pub fn get_values(&self, label: String) -> HashMap<String, String> {
        let map = block_on(async move {
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
        });

        map
    }
}
