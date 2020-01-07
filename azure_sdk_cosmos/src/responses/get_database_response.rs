use crate::database::Database;
use crate::{activity_id_from_headers, request_charge_from_headers};
use azure_sdk_core::errors::AzureError;
use hyper::header::HeaderMap;

#[derive(Debug, Clone)]
pub struct GetDatabaseResponse {
    pub database: Database,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for GetDatabaseResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;

        debug!("get database response == {}", std::str::from_utf8(body)?);

        let database: Database = serde_json::from_slice(body)?;
        let charge = request_charge_from_headers(headers)?;
        let activity_id = activity_id_from_headers(headers)?;

        Ok(Self {
            database,
            charge,
            activity_id,
        })
    }
}
