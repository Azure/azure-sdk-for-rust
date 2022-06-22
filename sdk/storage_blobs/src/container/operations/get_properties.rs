use crate::{container::Container, prelude::*};
use azure_core::{
    error::{ErrorKind, ResultExt},
    headers::{self, Headers},
    prelude::*,
    RequestId,
};
use chrono::{DateTime, FixedOffset};
use http::method::Method;
use std::convert::{TryFrom, TryInto};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GetPropertiesBuilder {
    container_client: ContainerClient,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
    lease_id: Option<LeaseId>,
}

impl GetPropertiesBuilder {
    pub(crate) fn new(container_client: ContainerClient) -> Self {
        Self {
            container_client,
            client_request_id: None,
            timeout: None,
            lease_id: None,
        }
    }

    setters! {
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        lease_id: LeaseId => Some(lease_id),
    }

    pub fn into_future(self) -> Response {
        Box::pin(async move {
            let mut url = self.container_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("restype", "container");

            self.timeout.append_to_url_query(&mut url);

            let mut request =
                self.container_client
                    .prepare_request(url.as_str(), Method::HEAD, None)?;
            request.add_optional_header(&self.client_request_id);
            request.add_optional_header(&self.lease_id);

            let response = self
                .container_client
                .storage_client()
                .storage_account_client()
                .http_client()
                .execute_request_check_status(&request)
                .await?;

            (self.container_client.container_name(), response.headers()).try_into()
        })
    }
}

#[derive(Debug, Clone)]
pub struct GetPropertiesResponse {
    pub container: Container,
    pub request_id: RequestId,
    pub date: DateTime<FixedOffset>,
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
        let request_id = Uuid::parse_str(headers.get_as_str_or_err(&headers::REQUEST_ID)?)
            .map_kind(ErrorKind::DataConversion)?;

        let date = DateTime::parse_from_rfc2822(headers.get_as_str_or_err(&headers::DATE)?)
            .map_kind(ErrorKind::DataConversion)?;

        let container = Container::from_response(container_name, headers)?;

        Ok(GetPropertiesResponse {
            container,
            request_id,
            date,
        })
    }
}

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<GetPropertiesResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for GetPropertiesBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}
