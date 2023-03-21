use azure_core::{auth::TokenCredential, FixedRetryOptions, RetryOptions};
use futures::executor::block_on;
use std::{collections::HashMap, fs::File, sync::Arc};

use self::{
    app_context::ContextHolder, feature_filter::FeatureFilter, feature_holder::FeatureHolder,
    feature_type::Feature,
};

pub mod app_context;
mod feature_filter;
mod feature_holder;
mod feature_type;
mod models;

pub const FEATURE_PREFIX: &str = ".appconfig.featureflag/";

#[derive(Clone)]
pub struct FeatureExplorer {
    holder: Arc<FeatureHolder>,
    context: Option<Arc<dyn ContextHolder>>,
    // use for dev perspective, to override the feature value from the remote
    on_off: HashMap<String, Vec<Feature>>,
    client: azure_svc_appconfiguration::Client,
}

#[derive(Clone)]
pub struct FeatureExplorerBuider {
    endpoint: Option<String>,
    credential: Arc<dyn TokenCredential>,
    context: Option<Arc<dyn ContextHolder>>,
    on_off: HashMap<String, Vec<Feature>>,
    retry: Option<RetryOptions>,
}

impl std::fmt::Debug for FeatureExplorer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FeatureExplorer")
            .field(
                "features",
                &serde_json::to_string(&self.holder.get_features()).unwrap(),
            )
            .field("on_off", &self.on_off)
            .finish()
    }
}

impl FeatureExplorerBuider {
    #[doc = "Create a new instance of `FeatureExplorerBuider`."]
    fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            on_off: HashMap::new(),
            context: None,
            retry: None,
        }
    }

    #[doc = "Set the endpoint."]
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }

    #[doc = "Set the context."]
    pub fn context(mut self, context: Arc<dyn ContextHolder>) -> Self {
        self.context = Some(context);
        self
    }

    #[doc = "Set the retry options."]
    pub fn retry(mut self, retry: impl Into<RetryOptions>) -> Self {
        self.retry = Some(retry.into());
        self
    }

    #[doc = "Use for reading features form file, for dev perspective, to override the feature value from the remote."]
    pub fn on_off(mut self, env_path: impl Into<String>) -> Self {
        let file = File::open(env_path.into()).expect("Cant open the file. Check the path");
        match serde_json::from_reader::<File, HashMap<String, Vec<Feature>>>(file) {
            Ok(it) => self.on_off.extend(it),
            Err(err) => {
                log::debug!("*ERROR :  {:?}", err);
            }
        }
        self
    }

    #[doc = "Convert the builder into a `FeatureExplorer` instance."]
    pub fn build(self) -> FeatureExplorer {
        let endpoint = self
            .endpoint
            .unwrap_or_else(|| azure_svc_appconfiguration::DEFAULT_ENDPOINT.to_owned());
        let retry = self
            .retry
            .unwrap_or_else(|| RetryOptions::fixed(FixedRetryOptions::default().max_retries(3u32)));

        FeatureExplorer::new(endpoint, self.credential, self.context, self.on_off, retry)
    }
}

impl FeatureExplorer {
    #[doc = "Create a new `FeatureExplorerBuider`."]
    pub fn builder(
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    ) -> FeatureExplorerBuider {
        FeatureExplorerBuider::new(credential)
    }

    fn new(
        endpoint: impl Into<String>,
        token_credential: Arc<dyn TokenCredential>,
        context: Option<Arc<dyn ContextHolder>>,
        on_off: HashMap<String, Vec<Feature>>,
        retry: impl Into<azure_core::RetryOptions>,
    ) -> Self {
        let scopes = &["https://azconfig.io"];
        let client = azure_svc_appconfiguration::Client::builder(token_credential)
            .endpoint(endpoint)
            .scopes(scopes)
            .retry(retry)
            .build();

        Self {
            holder: Arc::new(FeatureHolder::new(client.clone())),
            context,
            on_off,
            client,
        }
    }

    #[doc = "Checks to see if the feature is enabled. If enabled it check each filter, once a single filter returns true it returns true. If no filter returns true, it returns false. If there are no filters, it returns true. If feature isn't found it returns false"]
    pub fn is_enabled(&self, name: String) -> bool {
        let feature = self.get_features(name);

        !feature.is_empty()
            && feature
                .iter()
                .any(|feature| feature.evaluate(self.context.clone()))
    }

    fn get_features(&self, name: String) -> Vec<Feature> {
        self.on_off.get(&name).map_or_else(
            || {
                if std::env::var("FEATURE_FETCH_ALL_OFF").is_ok() {
                    block_on(self.fetch_by_key(name))
                } else {
                    self.holder
                        .get_features()
                        .get(&name)
                        .map_or(vec![], |it| it.clone())
                }
            },
            |it| it.clone(),
        )
    }

    async fn fetch_by_key(&self, key: String) -> Vec<Feature> {
        let result = self
            .client
            .get_key_value(format!("{}{}", FEATURE_PREFIX, key))
            .send()
            .await;
        match result {
            Ok(rs) => match rs.into_body().await {
                Ok(key_value) => {
                    match serde_json::from_str::<models::FeaturesFilter>(&key_value.value.unwrap())
                    {
                        Ok(key_value) => Feature::new(key_value),
                        Err(_) => vec![Feature::OnOff(false)],
                    }
                }
                Err(err) => {
                    log::error!("*ERROR :  {:?}", err);
                    vec![Feature::OnOff(false)]
                }
            },
            Err(err) => {
                log::error!("*ERROR :  {:?}", err);
                vec![Feature::OnOff(false)]
            }
        }
    }
}
