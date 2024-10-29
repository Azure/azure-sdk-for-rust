// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::borrow::Cow;

use azure_core::Model;
use serde::{Deserialize, Serialize};

use crate::models::SystemProperties;

const OFFER_VERSION_2: &'static str = "V2";

#[derive(Model, Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThroughputProperties {
    resource: String,
    #[serde(rename = "content")]
    offer: Offer,
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

    pub fn auto_scale(
        starting_maximum_throughput: usize,
        increment_percent: Option<usize>,
    ) -> ThroughputProperties {
        ThroughputProperties {
            offer_version: OFFER_VERSION_2.into(),
            offer: Offer {
                offer_autopilot_settings: Some(OfferAutoScaleSettings {
                    max_throughput: starting_maximum_throughput,
                    auto_upgrade_policy: increment_percent.map(|p| AutoScaleAutoUpgradePolicy {
                        throughput_policy: Some(AutoScaleThroughputPolicy {
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

    pub fn auto_scale_maximum(&self) -> Option<usize> {
        Some(self.offer.offer_autopilot_settings.as_ref()?.max_throughput)
    }

    pub fn auto_scale_increment(&self) -> Option<usize> {
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

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Offer {
    pub offer_throughput: Option<usize>,
    pub offer_autopilot_settings: Option<OfferAutoScaleSettings>,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct OfferAutoScaleSettings {
    pub max_throughput: usize,
    pub auto_upgrade_policy: Option<AutoScaleAutoUpgradePolicy>,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AutoScaleAutoUpgradePolicy {
    pub throughput_policy: Option<AutoScaleThroughputPolicy>,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AutoScaleThroughputPolicy {
    pub increment_percent: usize,
}
