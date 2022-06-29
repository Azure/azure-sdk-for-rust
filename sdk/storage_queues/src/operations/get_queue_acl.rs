use crate::{clients::QueueClient, QueueStoredAccessPolicy};
use azure_core::{
    collect_pinned_stream, headers::Headers, prelude::*, Context, Method, Response as AzureResponse,
};
use azure_storage::{core::headers::CommonStorageResponseHeaders, StoredAccessPolicyList};
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetQueueACLBuilder {
    queue_client: QueueClient,
    timeout: Option<Timeout>,
    context: Context,
}

impl GetQueueACLBuilder {
    pub(crate) fn new(queue_client: QueueClient) -> Self {
        Self {
            queue_client,
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
            let mut url = self.queue_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("comp", "acl");

            self.timeout.append_to_url_query(&mut url);

            let mut request = self.queue_client.storage_client().finalize_request(
                url,
                Method::Get,
                Headers::new(),
                None,
            )?;

            let response = self
                .queue_client
                .send(&mut self.context, &mut request)
                .await?;

            GetQueueACLResponse::try_from(response).await
        })
    }
}

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<GetQueueACLResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for GetQueueACLBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone)]
pub struct GetQueueACLResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub stored_access_policies: Vec<QueueStoredAccessPolicy>,
}

impl GetQueueACLResponse {
    async fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        let (_, headers, body) = response.deconstruct();
        let body = collect_pinned_stream(body).await?;

        let a: azure_core::Result<Vec<QueueStoredAccessPolicy>> =
            StoredAccessPolicyList::from_xml(&body)?
                .stored_access
                .into_iter()
                .map(|sap| sap.try_into())
                .collect();

        Ok(GetQueueACLResponse {
            common_storage_response_headers: (&headers).try_into()?,
            stored_access_policies: a?,
        })
    }
}
