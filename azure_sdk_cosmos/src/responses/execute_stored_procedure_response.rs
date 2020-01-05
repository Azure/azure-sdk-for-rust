use crate::{activity_id_from_headers, request_charge_from_headers};
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::session_token_from_headers;
use http::HeaderMap;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone)]
pub struct ExecuteStoredProcedureResponse<T>
where
    T: DeserializeOwned,
{
    pub payload: T,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
}

impl<T> std::convert::TryFrom<(&HeaderMap, &[u8])> for ExecuteStoredProcedureResponse<T>
where
    T: DeserializeOwned,
{
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;

        debug!("headers == {:?}", headers);
        debug!("body == {:#?}", body);

        Ok(Self {
            payload: serde_json::from_slice(body)?,
            charge: request_charge_from_headers(headers)?,
            activity_id: activity_id_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
        })
    }
}
