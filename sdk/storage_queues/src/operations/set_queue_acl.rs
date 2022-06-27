use crate::{clients::QueueClient, QueueStoredAccessPolicy};
use azure_core::{error::Error, prelude::*, Method, Response as AzureResponse};
use azure_storage::{core::headers::CommonStorageResponseHeaders, StoredAccessPolicyList};
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct SetQueueACLBuilder {
    queue_client: QueueClient,
    policies: Vec<QueueStoredAccessPolicy>,
    timeout: Option<Timeout>,
    context: Context,
}

impl SetQueueACLBuilder {
    pub(crate) fn new(queue_client: QueueClient, policies: Vec<QueueStoredAccessPolicy>) -> Self {
        SetQueueACLBuilder {
            queue_client,
            policies,
            timeout: None,
            context: Context::new(),
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
    }

    pub fn into_future(mut self) -> Response {
        Box::pin(async move {
            let mut url = self.queue_client.url_with_segments(None)?;

            url.query_pairs_mut().append_pair("comp", "acl");
            self.timeout.append_to_url_query(&mut url);

            // convert the queue_stored_access_policies slice
            // in a StoredAccessPolicyList to get its XML
            // representation.
            let xml_body = {
                let mut qapl = StoredAccessPolicyList::new();
                self.policies
                    .iter()
                    .for_each(|queue_policy| qapl.stored_access.push(queue_policy.into()));

                qapl.to_xml()
            };

            let mut request = self.queue_client.storage_client().prepare_request(
                url.as_str(),
                Method::PUT,
                Some(xml_body.into()),
            )?;

            let response = self
                .queue_client
                .send(&mut self.context, &mut request)
                .await?;

            response.try_into()
        })
    }
}

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<SetQueueACLResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for SetQueueACLBuilder {
    type IntoFuture = Response;
    type Output = <Response as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone)]
pub struct SetQueueACLResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl std::convert::TryFrom<AzureResponse> for SetQueueACLResponse {
    type Error = Error;

    fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        Ok(SetQueueACLResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
