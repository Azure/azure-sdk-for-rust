use crate::{QueueServiceClient, QueueServiceProperties};
use azure_core::{collect_pinned_stream, prelude::*, Context, Method, Response as AzureResponse};
use azure_storage::core::{headers::CommonStorageResponseHeaders, xml::read_xml};
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetQueueServicePropertiesBuilder {
    service_client: QueueServiceClient,
    timeout: Option<Timeout>,
    context: Context,
}

impl GetQueueServicePropertiesBuilder {
    pub(crate) fn new(service_client: QueueServiceClient) -> Self {
        Self {
            service_client,
            timeout: None,
            context: Context::new(),
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
        context: Context => context,
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

            let mut request =
                self.service_client
                    .storage_client
                    .prepare_request(url, Method::Get, None)?;

            let response = self
                .service_client
                .send(&mut self.context, &mut request)
                .await?;

            GetQueueServicePropertiesResponse::try_from(response).await
        })
    }
}

pub type Response =
    futures::future::BoxFuture<'static, azure_core::Result<GetQueueServicePropertiesResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for GetQueueServicePropertiesBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone)]
pub struct GetQueueServicePropertiesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub queue_service_properties: QueueServiceProperties,
}

impl GetQueueServicePropertiesResponse {
    async fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        let (_, headers, body) = response.deconstruct();
        let body = collect_pinned_stream(body).await?;

        let queue_service_properties: QueueServiceProperties = read_xml(&body)?;

        Ok(GetQueueServicePropertiesResponse {
            common_storage_response_headers: (&headers).try_into()?,
            queue_service_properties,
        })
    }
}
