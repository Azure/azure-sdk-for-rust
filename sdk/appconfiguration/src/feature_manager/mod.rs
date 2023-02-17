use azure_core::auth::TokenCredential;
use chrono::DateTime;
use futures::{executor::block_on, stream::StreamExt};
use rand::Rng;
use std::{collections::HashMap, sync::Arc};
pub mod models;

use self::models::Group;

const CONTENT_TYPE: &str = "application/vnd.microsoft.appconfig.ff+json;charset=utf-8";
const FEATURE_PREFIX: &str = ".appconfig.featureflag/";
const FEATURE_TARGETING: &str = "Microsoft.Targeting";
const FEATURE_PERCETEGE: &str = "Microsoft.Percentage";
const FEATURE_TIME_WINDOW: &str = "Microsoft.TimeWindow";

#[derive(Debug, Clone)]
pub enum Feature {
    Percentege(FeatureContext),
    Targeting(FeatureContext),
    TimeWindow(FeatureContext),
    OnOff(bool),
}

impl Feature {
    fn new(filter: models::FeaturesFilter) -> Vec<Self> {
        if filter.get_filters().is_empty() {
            return vec![Feature::OnOff(filter.is_enabled())];
        }

        filter
            .get_filters()
            .iter()
            .map(|it| {
                if FEATURE_TARGETING.eq(it.get_name()) {
                    Feature::Targeting(FeatureContext::new(
                        filter.get_id().to_string(),
                        filter.is_enabled(),
                        it,
                    ))
                } else if FEATURE_PERCETEGE.eq(it.get_name()) {
                    Feature::Percentege(FeatureContext::new(
                        filter.get_id().to_string(),
                        filter.is_enabled(),
                        it,
                    ))
                } else if FEATURE_TIME_WINDOW.eq(it.get_name()) {
                    Feature::TimeWindow(FeatureContext::new(
                        filter.get_id().to_string(),
                        filter.is_enabled(),
                        it,
                    ))
                } else {
                    Feature::OnOff(filter.is_enabled())
                }
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct FeatureContext {
    name: String,
    enabled: bool,
    users: Vec<String>,
    groups: Vec<Group>,
    default_rollout_percentage: i64,
    start: Option<String>,
    end: Option<String>,
    value: Option<i64>,
}

impl FeatureContext {
    fn new(id: String, enabled: bool, params: &models::ClientFilter) -> FeatureContext {
        FeatureContext {
            name: id,
            enabled: enabled,
            users: params.get_users(),
            groups: params.get_groups(),
            default_rollout_percentage: params.get_default_rollout_percentage(),
            start: params.get_start(),
            end: params.get_end(),
            value: params.get_value(),
        }
    }
}

#[derive(Debug, Clone)]
struct AppContext {
    // todo!
    id: String,
    groups: Vec<String>,
}

impl AppContext {
    fn new() -> AppContext {
        AppContext {
            id: String::from("test"),
            groups: vec![],
        }
    }
}

trait FeatureFilter {
    fn evaluate(&self, context: AppContext) -> bool;
}

impl FeatureFilter for Feature {
    fn evaluate(&self, context: AppContext) -> bool {
        match self {
            Feature::Percentege(ctx) => {
                ctx.enabled
                    && (ctx.value.is_some()
                        && (rand::thread_rng().gen_range(0..1) <= ctx.value.unwrap()))
            }
            Feature::Targeting(ctx) => {
                ctx.enabled
                    && (ctx.users.iter().any(|it| it.eq(&context.id))
                        || ctx.groups.iter().any(|it| {
                            context.groups.contains(&it.Name)
                                && (rand::thread_rng().gen_range(0..1)
                                    <= ctx.default_rollout_percentage)
                        })
                        || (rand::thread_rng().gen_range(0..1) <= ctx.default_rollout_percentage))
            }
            Feature::TimeWindow(ctx) => {
                let now = chrono::Utc::now().timestamp_nanos();
                let start = match &ctx.start {
                    Some(start) => Some(
                        DateTime::parse_from_rfc3339(start)
                            .unwrap_or(DateTime::parse_from_rfc2822(start).unwrap())
                            .timestamp_nanos(),
                    ),
                    None => None,
                };
                let end = match &ctx.end {
                    Some(end) => Some(
                        DateTime::parse_from_rfc3339(end)
                            .unwrap_or(DateTime::parse_from_rfc2822(end).unwrap())
                            .timestamp_nanos(),
                    ),
                    None => None,
                };

                //println!("Strat - {start:?} -- End - {end:?}");
                ctx.enabled
                    && ((start.is_none() || now > start.unwrap())
                        && (end.is_none() || now < end.unwrap()))
            }
            Feature::OnOff(v) => *v,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub struct FeatureHolder {
    // todo! do we need cashe all features ?
    features: HashMap<String, Vec<Feature>>,
    on_off: HashMap<String, Vec<Feature>>,
    client: azure_svc_appconfiguration::Client,
}

impl std::fmt::Debug for FeatureHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FeatureHolder")
            .field("features", &self.features)
            .field("on_off", &self.on_off)
            .finish()
    }
}

impl FeatureHolder {
    pub fn new(token_credential: Arc<dyn TokenCredential>) -> FeatureHolder {
        let on_off: HashMap<String, Vec<Feature>> = if std::env::var("FEATURE_ON_OFF").is_ok() {
            // todo read from file? onOff features
            HashMap::new()
        } else {
            HashMap::new()
        };

        let name =
            std::env::var("AZCONFIG_NAME").expect("Missing AZCONFIG_NAME environment variable.");
        let endpoint = format!("https://{name}.azconfig.io");
        let scopes = &["https://azconfig.io"];

        let client = azure_svc_appconfiguration::Client::builder(token_credential)
            .endpoint(endpoint)
            .scopes(scopes)
            .build();

        let mut features = HashMap::new();
        let holder = FeatureHolder {
            on_off: on_off,
            client,
            features: features,
        };

        if std::env::var("FEATURE_FETCH_ALL_OFF").is_ok() {
            holder
        } else {
            features = block_on(holder.fetch_all());
            FeatureHolder {
                features: features,
                ..holder
            }
        }
    }

    fn get_feature(&self, name: String) -> Vec<Feature> {
        if std::env::var("FEATURE_FETCH_ALL_OFF").is_ok() {
            block_on(self.fetch_by_key(name))
        } else {
            self.on_off
                .get(&name)
                .or_else(|| self.features.get(&name))
                .cloned()
                .unwrap_or(vec![])
        }
    }

    async fn fetch_all(&self) -> HashMap<String, Vec<Feature>> {
        let mut features_tmp = HashMap::new();
        let mut stream = self.client.clone().get_key_values().into_stream();
        while let Some(rs) = stream.next().await {
            let rs = rs.unwrap(); // todo! unwrap

            let rs = rs
                .items
                .iter()
                .filter(|&key| match &key.content_type {
                    Some(content_type) => content_type.eq(CONTENT_TYPE),
                    None => false,
                })
                .map(|it| {
                    (
                        it.key.clone().unwrap(),
                        serde_json::from_str::<models::FeaturesFilter>(&it.value.clone().unwrap())
                            .unwrap(),
                    )
                })
                .map(|it| {
                    let feature = Feature::new(it.1);
                    let key =
                        it.0.strip_prefix(FEATURE_PREFIX)
                            .unwrap_or(&it.0)
                            .to_string();
                    (key, feature)
                })
                .collect::<HashMap<String, Vec<Feature>>>();

            features_tmp.extend(rs.clone());
        }

        features_tmp
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
                Ok(key_value) => Feature::new(
                    serde_json::from_str::<models::FeaturesFilter>(&key_value.value.unwrap())
                        .unwrap(),
                ),
                Err(err) => {
                    println!("*ERROR :  {:?}", err);
                    vec![Feature::OnOff(false)]
                }
            },
            Err(err) => {
                println!("*ERROR :  {:?}", err);
                vec![Feature::OnOff(false)]
            }
        }
    }
}

pub trait FeatureManager {
    fn is_enabled(&self, feature: String) -> bool;
}

impl FeatureManager for FeatureHolder {
    fn is_enabled(&self, name: String) -> bool {
        let feature = self.get_feature(name);

        !feature.is_empty()
            && feature
                .iter()
                .all(|feature| feature.evaluate(AppContext::new()))
    }
}
