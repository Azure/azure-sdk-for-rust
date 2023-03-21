use azure_core::{auth::TokenCredential, error::ErrorKind, FixedRetryOptions, RetryOptions};
use futures::stream::StreamExt;
use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]
pub struct ConfigurationExplorer {
    client: azure_svc_appconfiguration::Client,
}

#[derive(Clone)]
pub struct ConfigurationExplorerBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    retry: Option<RetryOptions>,
}

impl ConfigurationExplorerBuilder {
    #[doc = "Create a new instance of `FeatureExplorerBuider`."]
    fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            retry: None,
        }
    }

    #[doc = "Set the endpoint."]
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }

    #[doc = "Set the retry options."]
    pub fn retry(mut self, retry: impl Into<RetryOptions>) -> Self {
        self.retry = Some(retry.into());
        self
    }

    #[doc = "Convert the builder into a `FeatureExplorer` instance."]
    pub fn build(self) -> ConfigurationExplorer {
        let endpoint = self
            .endpoint
            .unwrap_or_else(|| azure_svc_appconfiguration::DEFAULT_ENDPOINT.to_owned());
        let retry = self
            .retry
            .unwrap_or_else(|| RetryOptions::fixed(FixedRetryOptions::default().max_retries(3u32)));

        ConfigurationExplorer::new(endpoint, self.credential, retry)
    }
}

impl ConfigurationExplorer {
    #[doc = "Create a new instance of `ConfigurationExplorerBuilder`."]
    pub fn builder(
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    ) -> ConfigurationExplorerBuilder {
        ConfigurationExplorerBuilder::new(credential)
    }

    fn new(
        endpoint: impl Into<String>,
        token_credential: Arc<dyn TokenCredential>,
        retry: impl Into<RetryOptions>,
    ) -> Self {
        let scopes = &["https://azconfig.io"];
        let client = azure_svc_appconfiguration::Client::builder(token_credential)
            .endpoint(endpoint)
            .scopes(scopes)
            .retry(retry)
            .build();

        Self { client }
    }

    #[doc = "Returns Some value by key or None."]
    pub async fn get_value(&self, key: impl Into<String>) -> azure_core::Result<Option<String>> {
        match self.client.get_key_value(key).await {
            Ok(rs) => Ok(rs.value),
            Err(err) => {
                log::debug!("*ERROR :  {:?}", err);
                Err(azure_core::Error::with_message(ErrorKind::Other, || {
                    "failed to fetch data from appconfig. Please check debug logs.".to_string()
                }))
            }
        }
    }

    #[doc = "Returns HashMap with values by label or empty map."]
    pub async fn get_values(
        &self,
        label: impl Into<String>,
    ) -> azure_core::Result<HashMap<String, String>> {
        let mut map = HashMap::new();
        let mut stream = self.client.get_key_values().label(label).into_stream();

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
                    log::debug!("*ERROR :  {:?}", err);
                    return Err(azure_core::Error::with_message(ErrorKind::Other, || {
                        "failed to fetch data from appconfig. Please check debug logs.".to_string()
                    }));
                }
            }
        }

        Ok(map)
    }
}
