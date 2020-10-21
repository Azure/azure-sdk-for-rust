use crate::from_headers::*;
use crate::ResourceQuota;
use azure_core::errors::AzureError;
use azure_core::headers::session_token_from_headers;
use hyper::header::HeaderMap;

#[derive(Debug, Clone)]
pub struct DeleteDatabaseResponse {
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for DeleteDatabaseResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let _body = value.1;

        let charge = request_charge_from_headers(headers)?;
        let activity_id = activity_id_from_headers(headers)?;

        Ok(Self {
            charge,
            activity_id,
            session_token: session_token_from_headers(headers)?,
            resource_quota: resource_quota_from_headers(headers)?,
            resource_usage: resource_usage_from_headers(headers)?,
        })
    }
}
