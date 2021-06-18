use crate::QueueServiceProperties;
use azure_core::headers::CommonStorageResponseHeaders;
use azure_core::util::to_str_without_bom;
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
        let body = to_str_without_bom(response.body())?;

        debug!("headers == {:?}", headers);
        debug!("body == {:#?}", body);
        let queue_service_properties: QueueServiceProperties = serde_xml_rs::from_str(body)?;
        debug!("deserde == {:#?}", response);

        Ok(GetQueueServicePropertiesResponse {
            common_storage_response_headers: headers.try_into()?,
            queue_service_properties,
        })
    }
}
