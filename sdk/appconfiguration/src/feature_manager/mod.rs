use async_lock::RwLock;
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
const FEATURE_PERCENTAGE: &str = "Microsoft.Percentage";
const FEATURE_TIME_WINDOW: &str = "Microsoft.TimeWindow";

#[derive(Debug, Clone)]
pub enum Feature {
    Percentage(FeatureContext),
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
                } else if FEATURE_PERCENTAGE.eq(it.get_name()) {
                    Feature::Percentage(FeatureContext::new(
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
            enabled,
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
pub struct AppContext {
    id: String,
    groups: Vec<String>,
}

impl AppContext {
    pub fn new(id: String, groups: Vec<String>) -> AppContext {
        AppContext { id, groups }
    }
}

trait FeatureFilter {
    fn evaluate(&self, context: Option<Arc<dyn ContextHolder>>) -> bool;
}

impl FeatureFilter for Feature {
    fn evaluate(&self, context: Option<Arc<dyn ContextHolder>>) -> bool {
        match self {
            Feature::Percentage(ctx) => {
                ctx.enabled && (ctx.value.is_some() && is_percentage(ctx.value.unwrap()))
            }
            Feature::Targeting(ctx) => {
                let context_value = match context {
                    Some(context) => context.get_context(),
                    None => AppContext::new(String::from("test"), vec![]),
                };

                ctx.enabled
                    && (ctx.users.iter().any(|it| it.eq(&context_value.id))
                        || ctx.groups.iter().any(|it| {
                            context_value.groups.contains(&it.Name)
                                && is_percentage(ctx.default_rollout_percentage)
                        })
                        || (is_percentage(ctx.default_rollout_percentage)))
            }
            Feature::TimeWindow(ctx) => {
                let now = chrono::Utc::now().timestamp_nanos();
                let start = match &ctx.start {
                    Some(start) => parce_to_nanos(start),
                    None => None,
                };
                let end = match &ctx.end {
                    Some(end) => parce_to_nanos(end),
                    None => None,
                };

                ctx.enabled
                    && ((start.is_none() || now > start.unwrap())
                        && (end.is_none() || now < end.unwrap()))
            }
            Feature::OnOff(v) => *v,
            _ => unreachable!(),
        }
    }
}

pub trait ContextHolder {
    fn get_context(&self) -> AppContext;
}

#[derive(Clone)]
pub struct FeatureManager {
    holder: Arc<dyn FeatureHolder>,
    context: Option<Arc<dyn ContextHolder>>,
    on_off: HashMap<String, Vec<Feature>>,
    client: azure_svc_appconfiguration::Client,
}

impl std::fmt::Debug for FeatureManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FeatureHolder")
            .field("features", &self.holder.get_features())
            .field("on_off", &self.on_off)
            .finish()
    }
}

impl FeatureManager {
    pub fn new(
        token_credential: Arc<dyn TokenCredential>,
        context: Option<Arc<dyn ContextHolder>>,
    ) -> FeatureManager {
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

        FeatureManager {
            holder: Arc::new(AutoRefreshingFeatures::new(client.clone())),
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

trait FeatureHolder {
    fn get_features(&self) -> HashMap<String, Vec<Feature>>;
}

struct FeatureMap {
    features: HashMap<String, Vec<Feature>>,
    expires_on: time::OffsetDateTime,
}

struct AutoRefreshingFeatures {
    current_features: Arc<RwLock<Option<FeatureMap>>>,
    client: azure_svc_appconfiguration::Client,
}

impl FeatureHolder for AutoRefreshingFeatures {
    fn get_features(&self) -> HashMap<String, Vec<Feature>> {
        block_on(async {
            // if the current cached features is good, return that.
            if let Some(feature_map) = self.current_features.read().await.as_ref() {
                if !is_expired(feature_map) {
                    return feature_map.features.clone();
                }
            }

            let mut guard = self.current_features.write().await;

            // check again in case another thread refreshed the features while we were
            // waiting on the write lock
            if let Some(feature_map) = guard.as_ref() {
                if !is_expired(feature_map) {
                    return feature_map.features.clone();
                }
            }

            let result = self.fetch_all().await;
            *guard = Some(FeatureMap {
                features: result.clone(),
                expires_on: time::OffsetDateTime::now_utc()
                    + std::time::Duration::from_secs(match std::env::var("FEATURE_EXPIRE_ON") {
                        Ok(s) => match s.parse::<u64>() {
                            Ok(i) => i,
                            Err(_) => 20,
                        },
                        Err(_) => 20,
                    }),
            });

            result
        })
    }
}

impl AutoRefreshingFeatures {
    fn new(client: azure_svc_appconfiguration::Client) -> Self {
        AutoRefreshingFeatures {
            client,
            current_features: Arc::new(RwLock::new(Option::Some(FeatureMap {
                features: HashMap::new(),
                expires_on: time::OffsetDateTime::now_utc(),
            }))),
        }
    }

    async fn fetch_all(&self) -> HashMap<String, Vec<Feature>> {
        let mut features_tmp = HashMap::new();
        let mut stream = self.client.clone().get_key_values().into_stream();
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
                        .map(|it| match (it.key.clone(), it.value.clone()) {
                            (Some(key), Some(value)) => {
                                match serde_json::from_str::<models::FeaturesFilter>(&value) {
                                    Ok(value) => Some((key, value)),
                                    Err(_) => None,
                                }
                            }
                            _ => None,
                        })
                        .filter_map(|e| e)
                        .map(|t| {
                            let key = t.0.strip_prefix(FEATURE_PREFIX).unwrap_or(&t.0).to_string();
                            (key, Feature::new(t.1))
                        })
                        .collect::<HashMap<String, Vec<Feature>>>();

                    features_tmp.extend(items);
                }
                Err(err) => {
                    println!("*ERROR :  {:?}", err)
                }
            }
        }

        features_tmp
    }
}

fn is_expired(map: &FeatureMap) -> bool {
    map.expires_on < time::OffsetDateTime::now_utc()
}

fn is_percentage(value: i64) -> bool {
    rand::thread_rng().gen_range(0..1) * 100 <= value
}

fn parce_to_nanos(s: &str) -> Option<i64> {
    let mut date = DateTime::parse_from_rfc3339(s);
    match date {
        Ok(date) => Some(date.timestamp_nanos()),
        Err(_) => {
            date = DateTime::parse_from_rfc2822(s);
            match date {
                Ok(date) => Some(date.timestamp_nanos()),
                Err(_) => None,
            }
        }
    }
}
