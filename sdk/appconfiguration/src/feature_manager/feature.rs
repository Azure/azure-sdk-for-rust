use super::{feature_filter::FeatureContext, models::FeaturesFilter};

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
    pub fn new(filter: FeaturesFilter) -> Vec<Self> {
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
