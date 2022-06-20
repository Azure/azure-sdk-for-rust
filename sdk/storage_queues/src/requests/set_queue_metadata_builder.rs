use crate::clients::QueueClient;
use crate::responses::*;
use azure_core::prelude::*;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct SetQueueMetadataBuilder<'a> {
    queue_client: &'a QueueClient,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId>,
}

impl<'a> SetQueueMetadataBuilder<'a> {
    pub(crate) fn new(queue_client: &'a QueueClient) -> Self {
        SetQueueMetadataBuilder {
            queue_client,
            timeout: None,
            client_request_id: None,
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    /// This call sets the metadata.
    /// Keep in mind that keys present on Azure but not included in the passed
    /// metadata parameter will be deleted. If you want to keep the preexisting
    /// key-value pairs, retrieve them with GetMetadata first and
    /// then update/add to the received Metadata struct. Then pass the Metadata
    /// back to SetQueueMetadata.
    /// If you just want to clear the metadata, just pass an empty Metadata
    /// struct.
    pub async fn execute(
        &self,
        metadata: &Metadata,
    ) -> azure_core::Result<SetQueueMetadataResponse> {
        let mut url = self.queue_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("comp", "metadata");
        self.timeout.append_to_url_query(&mut url);

        let mut request = self.queue_client.storage_client().prepare_request(
            url.as_str(),
            http::method::Method::PUT,
            None,
        )?;
        for m in metadata.iter() {
            request.add_mandatory_header(&m);
        }
        request.add_optional_header(self.client_request_id.as_ref());

        let response = self
            .queue_client
            .storage_client()
            .execute_request_check_status(&request)
            .await?;

        response.try_into()
    }
}
