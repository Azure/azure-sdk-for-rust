// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Types and methods for Long-Running Operations (LROs).

use crate::http::headers::Headers;
use typespec_client_core::time::{Duration, OffsetDateTime};

/// Default retry time for long running operations if no retry-after header is present
///
/// This value is the same as the default used in the Azure SDK for Python.
/// Ref: <https://github.com/Azure/azure-sdk-for-python/blob/main/sdk/core/azure-mgmt-core/azure/mgmt/core/polling/arm_polling.py#L191>
const DEFAULT_RETRY_TIME: Duration = Duration::seconds(30);

/// Long-Running Operation (LRO) status.
#[derive(Debug)]
pub enum PollerStatus {
    InProgress,
    Succeeded,
    Failed,
    Canceled,
    Other(String),
}

impl From<&str> for PollerStatus {
    fn from(s: &str) -> Self {
        match s {
            "InProgress" => PollerStatus::InProgress,
            "Succeeded" => PollerStatus::Succeeded,
            "Failed" => PollerStatus::Failed,
            // While the specification indicates we should use `Canceled`, in
            // practice numerous services use `Cancelled`.  As such, we support
            // both.
            //
            // Ref: <https://github.com/Azure/azure-resource-manager-rpc/issues/144>
            "Canceled" | "Cancelled" => PollerStatus::Canceled,
            _ => PollerStatus::Other(s.to_owned()),
        }
    }
}

/// Get the retry duration from the operation response.
pub fn get_retry_after(headers: &Headers) -> Duration {
    crate::http::policies::get_retry_after(headers, OffsetDateTime::now_utc)
        .unwrap_or(DEFAULT_RETRY_TIME)
}

/// Types and methods for getting Long-Running Operation (LRO) resource locations.
pub mod location {
    use crate::{
        http::{
            headers::{Headers, AZURE_ASYNCOPERATION, LOCATION, OPERATION_LOCATION},
            poller::PollerStatus,
            Url,
        },
        json::from_json,
    };

    /// How to find the final resource URL.
    #[derive(Debug, Clone, Copy)]
    pub enum FinalState {
        /// The final resource URL is found in the `azure-asyncoperation` header.
        AzureAsyncOperation,

        /// The final resource URL is found in the `location` header.
        Location,

        /// The final resource URL is found in the `operation-location` header.
        OperationLocation,
    }

    /// Get the location from the `headers` based on the `final_state` location.
    pub fn get_location(headers: &Headers, final_state: FinalState) -> crate::Result<Option<Url>> {
        match final_state {
            FinalState::AzureAsyncOperation => headers.get_optional_as(&AZURE_ASYNCOPERATION),
            FinalState::Location => headers.get_optional_as(&LOCATION),
            FinalState::OperationLocation => headers.get_optional_as(&OPERATION_LOCATION),
        }
    }

    /// Get the [`PollerStatus`] from the response body.
    pub fn get_operation_state(body: &[u8]) -> Option<PollerStatus> {
        #[derive(serde::Deserialize)]
        struct Body {
            status: String,
        }
        let body: Body = from_json(body).ok()?;
        Some(PollerStatus::from(body.status.as_str()))
    }
}

/// Types and methods for getting operation status from the body.
pub mod body_content {
    use crate::http::{poller::PollerStatus, StatusCode};
    use crate::json::{from_json, to_json};
    use serde::{Deserialize, Serialize};

    /// Extract the Long-Running Operation (LRO) state based on the status code and response body.
    pub fn get_operation_state<S>(status_code: StatusCode, body: &S) -> crate::Result<PollerStatus>
    where
        S: Serialize,
    {
        match status_code {
            StatusCode::Accepted => Ok(PollerStatus::InProgress),
            StatusCode::Created => {
                Ok(get_provisioning_state_from_body(body).unwrap_or(PollerStatus::InProgress))
            }
            StatusCode::Ok => {
                Ok(get_provisioning_state_from_body(body).unwrap_or(PollerStatus::Succeeded))
            }
            StatusCode::NoContent => Ok(PollerStatus::Succeeded),
            _ => Err(crate::error::Error::from(
                crate::error::ErrorKind::HttpResponse {
                    status: status_code,
                    error_code: Some("invalid status found in LRO response".to_owned()),
                },
            )),
        }
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "snake_case")]
    struct Properties {
        provisioning_state: String,
    }

    #[derive(Deserialize)]
    struct Body {
        properties: Properties,
    }

    fn get_provisioning_state_from_body<S>(body: &S) -> Option<PollerStatus>
    where
        S: Serialize,
    {
        let body: Body = from_json(to_json(&body).ok()?).ok()?;
        Some(PollerStatus::from(
            body.properties.provisioning_state.as_str(),
        ))
    }
}
