use crate::database::Database;
use crate::requests::request_charge_from_headers;
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::etag_from_headers;
use http::HeaderMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDatabaseResponse {
    pub database: Database,
    pub charge: f64,
    pub etag: String,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for CreateDatabaseResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;

        let database = serde_json::from_slice::<Database>(&body)?;
        let charge = request_charge_from_headers(headers)?;
        let etag = etag_from_headers(headers)?;

        Ok(CreateDatabaseResponse {
            database,
            charge,
            etag,
        })
    }
}
