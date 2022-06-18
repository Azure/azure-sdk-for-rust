use crate::QueueServiceProperties;
use azure_core::error::{Error, ErrorKind, ResultExt};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use azure_storage::core::xml::read_xml;
use bytes::Bytes;
use http::response::Response;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetQueueServicePropertiesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub queue_service_properties: QueueServiceProperties,
}

impl std::convert::TryFrom<CollectedResponse> for GetQueueServicePropertiesResponse {
    type Error = Error;

    fn try_from(response: CollectedResponse) -> azure_core::Result<Self> {
        let headers = response.headers();
        let body = response.body();

        debug!("headers == {:?}", headers);
        debug!("body == {:#?}", body);
        let queue_service_properties: QueueServiceProperties =
            read_xml(body).map_kind(ErrorKind::DataConversion)?;
        debug!("deserde == {:#?}", response);

        Ok(GetQueueServicePropertiesResponse {
            common_storage_response_headers: headers.try_into()?,
            queue_service_properties,
        })
    }
}
