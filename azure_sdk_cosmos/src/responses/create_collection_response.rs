use crate::collection::Collection;
use crate::{activity_id_from_headers, request_charge_from_headers};
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::etag_from_headers;
use http::HeaderMap;

#[derive(Debug, Clone, PartialEq)]
pub struct CreateCollectionResponse {
    pub collection: Collection,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub etag: String,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for CreateCollectionResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;

        Ok(Self {
            collection: serde_json::from_slice(body)?,
            charge: request_charge_from_headers(headers)?,
            activity_id: activity_id_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
        })
    }
}
