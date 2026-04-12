// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Converts SDK throughput properties to driver request headers.

use azure_data_cosmos_driver::models::CosmosRequestHeaders;

use crate::models::ThroughputProperties;

/// Builds a [`CosmosRequestHeaders`] with throughput fields from SDK [`ThroughputProperties`].
pub(crate) fn from_throughput(
    throughput: &ThroughputProperties,
) -> azure_core::Result<CosmosRequestHeaders> {
    let mut headers = CosmosRequestHeaders::new();

    match (
        throughput.throughput(),
        throughput.autoscale_maximum().is_some(),
    ) {
        (Some(t), _) => {
            headers.offer_throughput = Some(t);
        }
        (_, true) => {
            // Serialize the autopilot settings to JSON for the header value.
            let autopilot_json = serde_json::to_string(&throughput.offer.offer_autopilot_settings)?;
            headers.offer_autopilot_settings = Some(autopilot_json);
        }
        _ => {}
    }

    Ok(headers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_throughput_produces_empty_headers() {
        let tp = ThroughputProperties::default();
        let headers = from_throughput(&tp).unwrap();
        assert!(headers.offer_throughput.is_none());
        assert!(headers.offer_autopilot_settings.is_none());
    }

    #[test]
    fn manual_throughput_sets_offer_throughput() {
        let tp = ThroughputProperties::manual(400);
        let headers = from_throughput(&tp).unwrap();
        assert_eq!(headers.offer_throughput, Some(400));
        assert!(headers.offer_autopilot_settings.is_none());
    }

    #[test]
    fn autoscale_throughput_sets_autopilot_settings() {
        let tp = ThroughputProperties::autoscale(4000, None);
        let headers = from_throughput(&tp).unwrap();
        assert!(headers.offer_throughput.is_none());
        let json = headers
            .offer_autopilot_settings
            .expect("should have autopilot settings");
        assert!(json.contains("4000"));
    }
}
