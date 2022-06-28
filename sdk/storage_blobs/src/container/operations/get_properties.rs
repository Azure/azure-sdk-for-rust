use crate::{container::Container, prelude::*};
use azure_core::Method;
use azure_core::{
    error::{ErrorKind, ResultExt},
    headers::{self, Headers},
    prelude::*,
    RequestId,
};
use chrono::{DateTime, FixedOffset};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct GetPropertiesBuilder {
    container_client: ContainerClient,
    timeout: Option<Timeout>,
    lease_id: Option<LeaseId>,
    context: Context,
}

impl GetPropertiesBuilder {
    pub(crate) fn new(container_client: ContainerClient) -> Self {
        Self {
            container_client,
            timeout: None,
            lease_id: None,
            context: Context::new(),
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
        lease_id: LeaseId => Some(lease_id),
        context: Context => context,
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.container_client.url_with_segments(None)?;
            self.timeout.append_to_url_query(&mut url);

            let mut request =
                self.container_client
                    .prepare_request(url.as_str(), Method::Head, None)?;
            request.add_optional_header(&self.lease_id);

            let response = self
                .container_client
                .send(&mut self.context, &mut request)
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
        let request_id = headers.get_as(&headers::REQUEST_ID)?;

        let date = DateTime::parse_from_rfc2822(headers.get_str(&headers::DATE)?)
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
