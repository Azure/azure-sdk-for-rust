use crate::headers::Headers;
use std::time::Duration;
use time::OffsetDateTime;

/// Default retry time for long running operations if no retry-after header is present
///
/// This value is the same as the default used in the Azure SDK for Python.
/// Ref: <https://github.com/Azure/azure-sdk-for-python/blob/main/sdk/core/azure-mgmt-core/azure/mgmt/core/polling/arm_polling.py#L191>
const DEFAULT_RETRY_TIME: Duration = Duration::from_secs(30);

/// Long Running Operation (LRO) status
///
/// Ref: <https://learn.microsoft.com/en-us/azure/azure-resource-manager/management/async-operations#provisioningstate-values>
#[derive(Debug)]
pub enum LroStatus {
    InProgress,
    Succeeded,
    Failed,
    Canceled,
    Other(String),
}

impl From<&str> for LroStatus {
    fn from(s: &str) -> Self {
        match s {
            "InProgress" => LroStatus::InProgress,
            "Succeeded" => LroStatus::Succeeded,
            "Failed" => LroStatus::Failed,
            // While the specification indicates we should use `Canceled`, in
            // practice numerous services use `Cancelled`.  As such, we support
            // both.
            //
            // Ref: <https://github.com/Azure/azure-resource-manager-rpc/issues/144>
            "Canceled" | "Cancelled" => LroStatus::Canceled,
            _ => LroStatus::Other(s.to_owned()),
        }
    }
}

pub fn get_retry_after(headers: &Headers) -> Duration {
    crate::get_retry_after(headers, OffsetDateTime::now_utc).unwrap_or(DEFAULT_RETRY_TIME)
}

pub mod location {
    use crate::{
        from_json,
        headers::{Headers, AZURE_ASYNCOPERATION, LOCATION, OPERATION_LOCATION},
        lro::LroStatus,
        Url,
    };

    #[derive(Debug, Clone, Copy)]
    pub enum FinalState {
        AzureAsyncOperation,
        Location,
        OperationLocation,
    }

    pub fn get_location(headers: &Headers, final_state: FinalState) -> crate::Result<Option<Url>> {
        match final_state {
            FinalState::AzureAsyncOperation => headers.get_optional_as(&AZURE_ASYNCOPERATION),
            FinalState::Location => headers.get_optional_as(&LOCATION),
            FinalState::OperationLocation => headers.get_optional_as(&OPERATION_LOCATION),
        }
    }

    pub fn get_provisioning_state(body: &[u8]) -> Option<LroStatus> {
        #[derive(serde::Deserialize)]
        struct Body {
            status: String,
        }
        let body: Body = from_json(body).ok()?;
        Some(LroStatus::from(body.status.as_str()))
    }
}

pub mod body_content {
    use crate::{from_json, lro::LroStatus, to_json, StatusCode};
    use serde::{Deserialize, Serialize};

    /// Extract the provisioning state based on the status code and response body
    ///
    /// Ref: <https://github.com/Azure/azure-sdk-for-python/blob/main/sdk/core/azure-core/azure/core/polling/base_polling.py>
    pub fn get_provisioning_state<S>(status_code: StatusCode, body: &S) -> crate::Result<LroStatus>
    where
        S: Serialize,
    {
        match status_code {
            StatusCode::Accepted => Ok(LroStatus::InProgress),
            StatusCode::Created => {
                Ok(get_provisioning_state_from_body(body).unwrap_or(LroStatus::InProgress))
            }
            StatusCode::Ok => {
                Ok(get_provisioning_state_from_body(body).unwrap_or(LroStatus::Succeeded))
            }
            StatusCode::NoContent => Ok(LroStatus::Succeeded),
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

    fn get_provisioning_state_from_body<S>(body: &S) -> Option<LroStatus>
    where
        S: Serialize,
    {
        let body: Body = from_json(to_json(&body).ok()?).ok()?;
        Some(LroStatus::from(body.properties.provisioning_state.as_str()))
    }
}
