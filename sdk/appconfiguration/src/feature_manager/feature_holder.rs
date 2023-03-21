use async_lock::RwLock;
use futures::stream::StreamExt;
use std::{collections::HashMap, sync::Arc};

use crate::auto_refresh::{AutoRefreshing, ExpiresValue};

use super::{feature_type::Feature, models::FeaturesFilter, FEATURE_PREFIX};

const CONTENT_TYPE: &str = "application/vnd.microsoft.appconfig.ff+json;charset=utf-8";
type ArcFeature = Arc<RwLock<Option<ExpiresValue<HashMap<String, Vec<Feature>>>>>>;

#[derive(Clone)]
pub struct FeatureHolder {
    client: azure_svc_appconfiguration::Client,
    features: ArcFeature,
}

#[async_trait::async_trait]
impl AutoRefreshing<HashMap<String, Vec<Feature>>> for FeatureHolder {
    fn get_current(&self) -> ArcFeature {
        Arc::clone(&self.features)
    }
    async fn get_latest(&self) -> HashMap<String, Vec<Feature>> {
        let mut features_tmp = HashMap::new();
        let mut stream = self.client.get_key_values().into_stream();
        while let Some(rs) = stream.next().await {
            match rs {
                Ok(rs) => {
                    let items = rs
                        .items
                        .iter()
                        .filter(|&key| match &key.content_type {
                            Some(content_type) => content_type.eq(CONTENT_TYPE),
                            None => false,
                        })
                        .flat_map(|it| match (it.key.clone(), it.value.clone()) {
                            (Some(key), Some(value)) => {
                                match serde_json::from_str::<FeaturesFilter>(&value) {
                                    Ok(value) => Some((key, value)),
                                    Err(_) => None,
                                }
                            }
                            _ => None,
                        })
                        .map(|t| {
                            let key = t.0.strip_prefix(FEATURE_PREFIX).unwrap_or(&t.0).to_string();
                            (key, Feature::new(t.1))
                        })
                        .collect::<HashMap<String, Vec<Feature>>>();

                    features_tmp.extend(items);
                }
                Err(err) => {
                    log::error!("*ERROR :  {:?}", err)
                }
            }
        }
        features_tmp
    }
}

impl FeatureHolder {
    pub fn new(client: azure_svc_appconfiguration::Client) -> Self {
        Self {
            client,
            features: Arc::new(RwLock::new(Option::Some(ExpiresValue {
                value: HashMap::new(),
                expires_on: time::OffsetDateTime::now_utc(),
            }))),
        }
    }

    pub async fn get_features(&self) -> HashMap<String, Vec<Feature>> {
        self.get_value().await
    }
}
