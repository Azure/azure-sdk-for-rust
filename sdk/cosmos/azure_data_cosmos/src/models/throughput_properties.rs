// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::borrow::Cow;

use azure_core::{
    headers::{AsHeaders, HeaderName, HeaderValue},
    Model,
};
use serde::{Deserialize, Serialize};

use crate::{constants, models::SystemProperties};

const OFFER_VERSION_2: &'static str = "V2";

#[derive(Model, Clone, Default, Debug, Deserialize, Serialize)]
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
                constants::OFFER_AUTO_SCALE,
                serde_json::to_string(&ap)?.into(),
            )],
            (None, None) => vec![],
        };

        Ok(vec.into_iter())
    }
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Offer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_throughput: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_autopilot_settings: Option<OfferAutoScaleSettings>,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OfferAutoScaleSettings {
    pub max_throughput: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_policy: Option<AutoScaleAutoUpgradePolicy>,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AutoScaleAutoUpgradePolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_policy: Option<AutoScaleThroughputPolicy>,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AutoScaleThroughputPolicy {
    pub increment_percent: usize,
}
