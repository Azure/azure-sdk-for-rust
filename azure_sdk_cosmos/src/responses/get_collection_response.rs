use crate::collection::Collection;
use crate::{activity_id_from_headers, request_charge_from_headers};
use azure_sdk_core::errors::AzureError;
use http::HeaderMap;

#[derive(Debug, Clone, PartialEq)]
pub struct GetCollectionResponse {
    pub collection: Collection,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for GetCollectionResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;

        debug!("body == {}", std::str::from_utf8(body)?);

        Ok(Self {
            collection: serde_json::from_slice(body)?,
            charge: request_charge_from_headers(headers)?,
            activity_id: activity_id_from_headers(headers)?,
        })
    }
}
