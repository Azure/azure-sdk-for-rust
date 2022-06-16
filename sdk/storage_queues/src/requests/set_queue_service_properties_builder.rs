use crate::responses::*;
use crate::QueueServiceProperties;
use azure_core::error::{ErrorKind, Result, ResultExt};
use azure_core::headers::add_optional_header;
use azure_core::prelude::*;
use azure_storage::core::clients::StorageClient;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct SetQueueServicePropertiesBuilder<'a> {
    storage_client: &'a StorageClient,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId>,
}

impl<'a> SetQueueServicePropertiesBuilder<'a> {
    pub(crate) fn new(storage_client: &'a StorageClient) -> Self {
        SetQueueServicePropertiesBuilder {
            storage_client,
            timeout: None,
            client_request_id: None,
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    /// Pass the properties here.
    /// More info here
    /// [https://docs.microsoft.com/rest/api/storageservices/set-queue-service-properties](https://docs.microsoft.com/rest/api/storageservices/set-queue-service-properties).
    pub async fn execute(
        &self,
        queue_service_properties: &QueueServiceProperties,
    ) -> Result<SetQueueServicePropertiesResponse> {
        let mut url = self
            .storage_client
            .storage_account_client()
            .queue_storage_url()
            .to_owned();

        url.query_pairs_mut().append_pair("restype", "service");
        url.query_pairs_mut().append_pair("comp", "properties");
        self.timeout.append_to_url_query(&mut url);

        let xml_body = serde_xml_rs::to_string(&queue_service_properties)
            .map_kind(ErrorKind::DataConversion)?;
        debug!("xml about to be sent == {}", xml_body);

        let request = self.storage_client.prepare_request(
            url.as_str(),
            &http::method::Method::PUT,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            Some(xml_body.into()),
        )?;

        let response = self
            .storage_client
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0, http::status::StatusCode::ACCEPTED)
            .await?;

        (&response).try_into()
    }
}
