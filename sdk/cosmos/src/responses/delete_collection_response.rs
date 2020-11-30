use crate::headers::from_headers::*;
use crate::CosmosError;
use http::response::Response;

#[derive(Debug, Clone)]
pub struct DeleteCollectionResponse {
    pub charge: f64,
    pub activity_id: uuid::Uuid,
}

impl std::convert::TryFrom<Response<Vec<u8>>> for DeleteCollectionResponse {
    type Error = CosmosError;
    fn try_from(response: Response<Vec<u8>>) -> Result<Self, Self::Error> {
        let headers = response.headers();

        let charge = request_charge_from_headers(headers)?;
        let activity_id = activity_id_from_headers(headers)?;

        Ok(Self {
            charge,
            activity_id,
        })
    }
}
