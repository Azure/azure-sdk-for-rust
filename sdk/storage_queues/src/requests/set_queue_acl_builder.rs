use crate::clients::QueueClient;
use crate::responses::*;
use crate::QueueStoredAccessPolicy;
use azure_core::prelude::*;

use azure_storage::StoredAccessPolicyList;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct SetQueueACLBuilder<'a> {
    queue_client: &'a QueueClient,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId>,
}

impl<'a> SetQueueACLBuilder<'a> {
    pub(crate) fn new(queue_client: &'a QueueClient) -> Self {
        SetQueueACLBuilder {
            queue_client,
            timeout: None,
            client_request_id: None,
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    /// Pass the requested polices here.
    /// While this SDK does not enforce any limit,
    /// keep in mind Azure supports a limited number of
    /// stored access policies for each queue.
    /// More info here
    /// [https://docs.microsoft.com/rest/api/storageservices/set-queue-acl#remarks](https://docs.microsoft.com/rest/api/storageservices/set-queue-acl#remarks).
    pub async fn execute(
        &self,
        queue_stored_access_policies: &[QueueStoredAccessPolicy],
    ) -> azure_core::Result<SetQueueACLResponse> {
        let mut url = self.queue_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("comp", "acl");
        self.timeout.append_to_url_query(&mut url);

        // convert the queue_stored_access_policies slice
        // in a StoredAccessPolicyList to get its XML
        // representation.
        let xml_body = {
            let mut qapl = StoredAccessPolicyList::new();
            queue_stored_access_policies
                .iter()
                .for_each(|queue_policy| qapl.stored_access.push(queue_policy.into()));

            qapl.to_xml()
        };

        let mut request = self.queue_client.storage_client().prepare_request(
            url.as_str(),
            azure_core::Method::PUT,
            Some(xml_body.into()),
        )?;
        request.add_optional_header(&self.client_request_id);

        let response = self
            .queue_client
            .storage_client()
            .http_client()
            .execute_request_check_status(&request)
            .await?;

        response.try_into()
    }
}
