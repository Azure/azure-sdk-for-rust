use crate::{xml::read_xml, QueueServiceProperties};
use azure_core::headers::CommonStorageResponseHeaders;
use bytes::Bytes;
use http::response::Response;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetQueueServicePropertiesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub queue_service_properties: QueueServiceProperties,
}

impl std::convert::TryFrom<&Response<Bytes>> for GetQueueServicePropertiesResponse {
    type Error = crate::Error;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        let headers = response.headers();
        let body = response.body();

        debug!("headers == {:?}", headers);
        debug!("body == {:#?}", body);
        let queue_service_properties: QueueServiceProperties = read_xml(body)?;
        debug!("deserde == {:#?}", response);

        Ok(GetQueueServicePropertiesResponse {
            common_storage_response_headers: headers.try_into()?,
            queue_service_properties,
        })
    }
}
