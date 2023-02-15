use azure_core::auth::TokenCredential;
use chrono::DateTime;
use futures::{executor::block_on, stream::StreamExt};
use rand::Rng;
use std::{collections::HashMap, sync::Arc};
pub mod models;

const FEATURE_PREFIX: &str = ".appconfig.featureflag/";
const FEATURE_TARGETING: &str = "Microsoft.Targeting";
const FEATURE_PERCETEGE: &str = "Microsoft.Percentege";
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
    // TODO!
    users: Vec<String>,
    groups: Vec<String>,
    start: String,
    end: String,
    value: String,
}

impl FeatureContext {
    fn new(id: String, enabled: bool, params: &models::ClientFilter) -> FeatureContext {
        FeatureContext {
            name: id,
            enabled: enabled,
            users: params.get_users().to_vec(),
            groups: params.get_groups().to_vec(),
            start: String::from(""), // todo from param!
            end: String::from(""),   // todo from param!
            value: String::from(""), // todo from param!
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
                    && (rand::thread_rng().gen_range(0..100) < ctx.value.parse::<i32>().unwrap())
            }
            Feature::Targeting(ctx) => {
                ctx.enabled
                    && (ctx.users.iter().any(|it| it.eq(&context.id))
                        || context.groups.iter().any(|it| ctx.groups.contains(it)))
            }
            Feature::TimeWindow(ctx) => {
                let now = chrono::Utc::now().time();
                let start = DateTime::parse_from_rfc3339(&ctx.start)
                    .unwrap_or(DateTime::parse_from_rfc2822(&ctx.start).unwrap())
                    .time();
                let end = DateTime::parse_from_rfc3339(&ctx.end)
                    .unwrap_or(DateTime::parse_from_rfc2822(&ctx.end).unwrap())
                    .time();

                ctx.enabled && (now > start && now < end)
            }
            Feature::OnOff(v) => *v,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FeatureHolder {
    // todo! do we need cashe all features ?
    features: HashMap<String, Vec<Feature>>,
    // for local/dev work
    on_off: HashMap<String, Vec<Feature>>,
}

impl FeatureHolder {
    pub fn new(token_credential: Arc<dyn TokenCredential>) -> FeatureHolder {
        let on_off: HashMap<String, Vec<Feature>> = if std::env::var("FEATURE_ON_OFF").is_ok() {
            // todo read from file? onOff features
            HashMap::new()
        } else {
            HashMap::new()
        };

        // let name = std::env::var("AZCONFIG_NAME").expect("Missing AZCONFIG_NAME environment variable.");
        // let endpoint = format!("https://{name}.azconfig.io");
        let endpoint = String::from("http://127.0.0.1:8080");
        let scopes = &["https://azconfig.io/"];

        let client = azure_svc_appconfiguration::Client::builder(token_credential)
            .endpoint(endpoint)
            .scopes(scopes)
            .build();

        let future = async {
            let mut features_tmp = HashMap::new();
            let mut stream = client.get_key_values().into_stream();
            while let Some(rs) = stream.next().await {
                let rs = rs.unwrap(); // todo! unwrap

                let rs = rs
                    .items
                    .iter()
                    .filter(|&key| match &key.content_type {
                        Some(content_type) => content_type
                            .eq("application/vnd.microsoft.appconfig.ff+json;charset=utf-8"),
                        None => false,
                    })
                    .map(|it| {
                        (
                            it.key.clone().unwrap(),
                            serde_json::from_str::<models::FeaturesFilter>(
                                &it.value.clone().unwrap(),
                            )
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
        };

        let features = block_on(future);

        FeatureHolder {
            features: features,
            on_off: on_off,
        }
    }

    fn get_feature(&self, name: String) -> Vec<Feature> {
        self.on_off
            .get(&name)
            .or_else(|| self.features.get(&name))
            .cloned()
            .unwrap_or(vec![])
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
