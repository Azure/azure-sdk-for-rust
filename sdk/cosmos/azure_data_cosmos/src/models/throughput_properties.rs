// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::borrow::Cow;

use azure_core::{
    fmt::SafeDebug,
    http::headers::{AsHeaders, HeaderName, HeaderValue},
};
use serde::{Deserialize, Serialize};

use crate::{constants, models::SystemProperties};

const OFFER_VERSION_2: &str = "V2";

#[derive(Clone, Default, SafeDebug, Deserialize, Serialize)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub struct ThroughputProperties {
    resource: String,
    #[serde(rename = "content")]
    pub(crate) offer: Offer,
    #[serde(rename = "id")]
    pub(crate) offer_id: String,
    offer_resource_id: String,
    offer_type: String,
    offer_version: Cow<'static, str>, // When we serialize, this is always going to be a constant.
    #[serde(flatten)]
    pub(crate) system_properties: SystemProperties,
}

impl ThroughputProperties {
    pub fn manual(throughput: usize) -> ThroughputProperties {
        ThroughputProperties {
            offer_version: OFFER_VERSION_2.into(),
            offer: Offer {
                offer_throughput: Some(throughput),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn autoscale(
        starting_maximum_throughput: usize,
        increment_percent: Option<usize>,
    ) -> ThroughputProperties {
        ThroughputProperties {
            offer_version: OFFER_VERSION_2.into(),
            offer: Offer {
                offer_autopilot_settings: Some(OfferAutoscaleSettings {
                    max_throughput: starting_maximum_throughput,
                    auto_upgrade_policy: increment_percent.map(|p| AutoscaleAutoUpgradePolicy {
                        throughput_policy: Some(AutoscaleThroughputPolicy {
                            increment_percent: p,
                        }),
                    }),
                }),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn throughput(&self) -> Option<usize> {
        self.offer.offer_throughput
    }

    pub fn autoscale_maximum(&self) -> Option<usize> {
        Some(self.offer.offer_autopilot_settings.as_ref()?.max_throughput)
    }

    pub fn autoscale_increment(&self) -> Option<usize> {
        Some(
            self.offer
                .offer_autopilot_settings
                .as_ref()?
                .auto_upgrade_policy
                .as_ref()?
                .throughput_policy
                .as_ref()?
                .increment_percent,
        )
    }
}

impl AsHeaders for ThroughputProperties {
    type Error = azure_core::Error;
    type Iter = std::vec::IntoIter<(HeaderName, HeaderValue)>;

    fn as_headers(&self) -> Result<Self::Iter, Self::Error> {
        let vec = match (
            self.offer.offer_throughput,
            self.offer.offer_autopilot_settings.as_ref(),
        ) {
            (Some(t), _) => vec![(constants::OFFER_THROUGHPUT, t.to_string().into())],
            (_, Some(ap)) => vec![(
                constants::OFFER_AUTOPILOT_SETTINGS,
                serde_json::to_string(&ap)?.into(),
            )],
            (None, None) => vec![],
        };

        Ok(vec.into_iter())
    }
}

#[derive(Clone, Default, SafeDebug, Deserialize, Serialize)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Offer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_throughput: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_autopilot_settings: Option<OfferAutoscaleSettings>,
}

#[derive(Clone, Default, SafeDebug, Deserialize, Serialize)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OfferAutoscaleSettings {
    pub max_throughput: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_policy: Option<AutoscaleAutoUpgradePolicy>,
}

#[derive(Clone, Default, SafeDebug, Deserialize, Serialize)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AutoscaleAutoUpgradePolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_policy: Option<AutoscaleThroughputPolicy>,
}

#[derive(Clone, Default, SafeDebug, Deserialize, Serialize)]
#[safe(true)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AutoscaleThroughputPolicy {
    pub increment_percent: usize,
}

impl ThroughputProperties {
    /// Applies throughput settings to the given request headers.
    ///
    /// Sets either the manual throughput or autoscale settings header,
    /// depending on how this `ThroughputProperties` was constructed.
    pub(crate) fn apply_headers(
        &self,
        headers: &mut azure_data_cosmos_driver::models::CosmosRequestHeaders,
    ) {
        match (
            self.offer.offer_throughput,
            self.offer.offer_autopilot_settings.as_ref(),
        ) {
            (Some(t), _) => {
                headers.offer_throughput = Some(t);
            }
            (_, Some(ap)) => {
                let mut settings = azure_data_cosmos_driver::models::OfferAutoscaleSettings::new(
                    ap.max_throughput,
                );
                if let Some(policy) = ap.auto_upgrade_policy.as_ref() {
                    if let Some(tp) = policy.throughput_policy.as_ref() {
                        settings = settings.with_increment_percent(tp.increment_percent);
                    }
                }
                headers.offer_autopilot_settings = Some(settings);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_data_cosmos_driver::models::CosmosRequestHeaders;

    #[test]
    fn default_throughput_produces_empty_headers() {
        let tp = ThroughputProperties::default();
        let mut headers = CosmosRequestHeaders::new();
        tp.apply_headers(&mut headers);
        assert!(headers.offer_throughput.is_none());
        assert!(headers.offer_autopilot_settings.is_none());
    }

    #[test]
    fn manual_throughput_sets_offer_throughput() {
        let tp = ThroughputProperties::manual(400);
        let mut headers = CosmosRequestHeaders::new();
        tp.apply_headers(&mut headers);
        assert_eq!(headers.offer_throughput, Some(400));
        assert!(headers.offer_autopilot_settings.is_none());
    }

    #[test]
    fn autoscale_throughput_sets_autopilot_settings() {
        let tp = ThroughputProperties::autoscale(4000, None);
        let mut headers = CosmosRequestHeaders::new();
        tp.apply_headers(&mut headers);
        assert!(headers.offer_throughput.is_none());
        let settings = headers
            .offer_autopilot_settings
            .expect("should have autopilot settings");
        assert_eq!(settings.max_throughput, 4000);
    }
}
