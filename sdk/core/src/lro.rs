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

pub mod body_content {
    use crate::{lro::LroStatus, StatusCode};
    use serde::Serialize;

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

    fn get_provisioning_state_from_body<S>(body: &S) -> Option<LroStatus>
    where
        S: Serialize,
    {
        let body: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(body).ok()?).ok()?;
        let provisioning_state = body["properties"]["provisioningState"].as_str()?;
        match provisioning_state {
            "Succeeded" => Some(LroStatus::Succeeded),
            "Failed" => Some(LroStatus::Failed),
            "Canceled" => Some(LroStatus::Canceled),
            _ => Some(LroStatus::Other(provisioning_state.to_owned())),
        }
    }
}
