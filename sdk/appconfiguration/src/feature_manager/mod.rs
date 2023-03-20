use azure_core::auth::TokenCredential;
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

impl FeatureExplorer {
    pub fn new(
        endpoint: &str,
        token_credential: Arc<dyn TokenCredential>,
        context: Option<Arc<dyn ContextHolder>>,
    ) -> Self {
        let on_off: HashMap<String, Vec<Feature>> =
            if let Ok(env_path) = std::env::var("FEATURE_ON_OFF") {
                let file = File::open(env_path).expect("Cant open the file. Check the path");
                match serde_json::from_reader(file) {
                    Ok(it) => it,
                    Err(err) => {
                        log::debug!("*ERROR :  {:?}", err);
                        HashMap::new()
                    }
                }
            } else {
                HashMap::new()
            };

        let scopes = &["https://azconfig.io"];
        let client = azure_svc_appconfiguration::Client::builder(token_credential)
            .endpoint(endpoint)
            .scopes(scopes)
            .build();

        Self {
            holder: Arc::new(FeatureHolder::new(client.clone())),
            context,
            on_off,
            client,
        }
    }

    pub fn is_enabled(&self, name: String) -> bool {
        let feature = self.get_features(name);

        !feature.is_empty()
            && feature
                .iter()
                .all(|feature| feature.evaluate(self.context.clone()))
    }

    fn get_features(&self, name: String) -> Vec<Feature> {
        if std::env::var("FEATURE_FETCH_ALL_OFF").is_ok() {
            self.on_off
                .get(&name)
                .map_or_else(|| block_on(self.fetch_by_key(name)), |it| it.clone())
        } else {
            self.on_off.get(&name).map_or_else(
                || {
                    self.holder
                        .get_features()
                        .get(&name)
                        .map_or(vec![], |it| it.clone())
                },
                |it| it.clone(),
            )
        }
    }

    async fn fetch_by_key(&self, key: String) -> Vec<Feature> {
        let result = self
            .client
            .clone()
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
                    log::debug!("*ERROR :  {:?}", err);
                    vec![Feature::OnOff(false)]
                }
            },
            Err(err) => {
                log::debug!("*ERROR :  {:?}", err);
                vec![Feature::OnOff(false)]
            }
        }
    }
}
