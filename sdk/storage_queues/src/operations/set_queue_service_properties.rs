use crate::{QueueServiceClient, QueueServiceProperties};
use azure_core::{
    error::{Error, ErrorKind, ResultExt},
    prelude::*,
    Context, Method, Response as AzureResponse,
};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct SetQueueServicePropertiesBuilder {
    service_client: QueueServiceClient,
    properties: QueueServiceProperties,
    timeout: Option<Timeout>,
    context: Context,
}

impl SetQueueServicePropertiesBuilder {
    pub(crate) fn new(
        service_client: QueueServiceClient,
        properties: QueueServiceProperties,
    ) -> Self {
        SetQueueServicePropertiesBuilder {
            service_client,
            properties,
            timeout: None,
            context: Context::new(),
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self
                .service_client
                .storage_client
                .storage_account_client()
                .queue_storage_url()
                .to_owned();

            url.query_pairs_mut().append_pair("restype", "service");
            url.query_pairs_mut().append_pair("comp", "properties");
            self.timeout.append_to_url_query(&mut url);

            let xml_body =
                serde_xml_rs::to_string(&self.properties).map_kind(ErrorKind::DataConversion)?;

            let mut request = self.service_client.storage_client.prepare_request(
                url,
                Method::PUT,
                Some(xml_body.into()),
            )?;

            let response = self
                .service_client
                .send(&mut self.context, &mut request)
                .await?;

            response.try_into()
        })
    }
}

pub type Response =
    futures::future::BoxFuture<'static, azure_core::Result<SetQueueServicePropertiesResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for SetQueueServicePropertiesBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone)]
pub struct SetQueueServicePropertiesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl std::convert::TryFrom<AzureResponse> for SetQueueServicePropertiesResponse {
    type Error = Error;

    fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        Ok(SetQueueServicePropertiesResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
