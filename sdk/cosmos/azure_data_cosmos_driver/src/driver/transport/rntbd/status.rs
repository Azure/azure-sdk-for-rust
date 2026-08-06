// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! RNTBD status mapping helpers.

use azure_core::http::StatusCode;

use crate::models::CosmosStatus;

/// Maps RNTBD frame status fields into a [`CosmosStatus`].
///
/// RNTBD carries the HTTP status in the frame header and the Cosmos DB
/// sub-status as an optional metadata token.
pub(crate) fn map_rntbd_status_to_cosmos_status(
    http_status: u32,
    sub_status: Option<u32>,
) -> CosmosStatus {
    let status = StatusCode::from(http_status as u16);
    let mut cosmos_status = CosmosStatus::new(status);
    if let Some(sub_status) = sub_status {
        cosmos_status = cosmos_status.with_sub_status(sub_status as u16);
    }
    cosmos_status
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_http_status_and_sub_status() {
        let status = map_rntbd_status_to_cosmos_status(404, Some(1002));

        assert_eq!(status.status_code(), StatusCode::NotFound);
        assert_eq!(status.sub_status().unwrap().value(), 1002);
        assert_eq!(status.name(), Some("ReadSessionNotAvailable"));
    }

    #[test]
    fn unknown_http_status_is_preserved() {
        let status = map_rntbd_status_to_cosmos_status(449, None);

        assert_eq!(status.status_code(), StatusCode::UnknownValue(449));
        assert_eq!(status.sub_status(), None);
    }
}
