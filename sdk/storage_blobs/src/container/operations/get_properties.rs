use crate::{container::Container, prelude::*};
use azure_core::{date, Method};
use azure_core::{
    headers::{self, Headers},
    prelude::*,
    RequestId,
};
use std::convert::{TryFrom, TryInto};
use time::OffsetDateTime;

operation! {
    GetProperties,
    client: ContainerClient,
    ?lease_id: LeaseId
}

impl GetPropertiesBuilder {
    pub fn into_future(mut self) -> GetProperties {
        Box::pin(async move {
            let mut url = self.client.url()?;

            url.query_pairs_mut().append_pair("restype", "container");

            let mut headers = Headers::new();
            headers.add(self.lease_id);

            let mut request = self
                .client
                .finalize_request(url, Method::Head, headers, None)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            (self.client.container_name(), response.headers()).try_into()
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetPropertiesResponse {
    pub container: Container,
    pub request_id: RequestId,
    pub date: OffsetDateTime,
}

impl TryFrom<(&str, &Headers)> for GetPropertiesResponse {
    type Error = crate::Error;

    fn try_from((body, header_map): (&str, &Headers)) -> azure_core::Result<Self> {
        GetPropertiesResponse::from_response(body, header_map)
    }
}

impl GetPropertiesResponse {
    pub(crate) fn from_response(
        container_name: &str,
        headers: &Headers,
    ) -> azure_core::Result<GetPropertiesResponse> {
        let request_id = headers.get_as(&headers::REQUEST_ID)?;

        let date = date::parse_rfc1123(headers.get_str(&headers::DATE)?)?;

        let container = Container::from_response(container_name, headers)?;

        Ok(GetPropertiesResponse {
            container,
            request_id,
            date,
        })
    }
}
