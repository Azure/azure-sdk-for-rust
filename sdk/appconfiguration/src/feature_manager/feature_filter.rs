use azure_core::date;
use futures::executor::block_on;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use time::OffsetDateTime;

use super::{
    app_context::{AppContext, ContextHolder},
    feature_type::Feature,
    models::{ClientFilter, Group},
};

pub trait FeatureFilter {
    fn evaluate(&self, context: Option<Arc<dyn ContextHolder>>) -> bool;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureContext {
    enabled: bool,
    users: Vec<String>,
    groups: Vec<Group>,
    default_rollout_percentage: f32,
    start: Option<String>,
    end: Option<String>,
    value: Option<f32>,
}

impl FeatureContext {
    pub fn new(enabled: bool, params: &ClientFilter) -> FeatureContext {
        FeatureContext {
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

impl FeatureFilter for Feature {
    fn evaluate(&self, context: Option<Arc<dyn ContextHolder>>) -> bool {
        match self {
            Feature::Percentage(ctx) => {
                ctx.enabled && (ctx.value.is_some() && is_percentage(ctx.value.unwrap()))
            }
            Feature::Targeting(ctx) => {
                let context_value = match context {
                    Some(context) => block_on(context.get_context()),
                    None => AppContext::new(String::from(""), vec![]),
                };

                ctx.enabled
                    && (ctx.users.iter().any(|it| it.eq(&context_value.get_id()))
                        || ctx.groups.iter().any(|it| {
                            context_value.get_groups().contains(&it.name)
                                && is_percentage(ctx.default_rollout_percentage)
                        })
                        || (is_percentage(ctx.default_rollout_percentage)))
            }
            Feature::TimeWindow(ctx) => {
                let now = OffsetDateTime::now_utc().unix_timestamp_nanos();
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
        }
    }
}

fn is_percentage(value: f32) -> bool {
    rand::thread_rng().gen_range(0.0..1.0) * 100.0 < value
}

fn parce_to_nanos(s: &str) -> Option<i128> {
    match date::parse_rfc3339(s) {
        Ok(date) => Some(date.unix_timestamp_nanos()),
        Err(_) => match date::parse_rfc1123(s) {
            Ok(date) => Some(date.unix_timestamp_nanos()),
            Err(_) => None,
        },
    }
}
