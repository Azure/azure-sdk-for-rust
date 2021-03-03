use crate::queue::clients::PopReceiptClient;
use crate::queue::responses::*;
use azure_core::headers::add_optional_header;
use azure_core::prelude::*;
use std::convert::TryInto;

#[derive(Debug)]
pub struct DeleteMessageBuilder<'a> {
    pop_receipt_client: &'a PopReceiptClient,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId<'a>>,
}

impl<'a> DeleteMessageBuilder<'a> {
    pub(crate) fn new(pop_receipt_client: &'a PopReceiptClient) -> Self {
        DeleteMessageBuilder {
            pop_receipt_client,
            timeout: None,
            client_request_id: None,
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
    }

    pub async fn execute(
        &self,
    ) -> Result<DeleteMessageResponse, Box<dyn std::error::Error + Sync + Send>> {
        let mut url = self.pop_receipt_client.pop_receipt_url()?;

        self.timeout.append_to_url_query(&mut url);

        debug!("url == {}", url.as_str());

        let request = self.pop_receipt_client.storage_client().prepare_request(
            url.as_str(),
            &http::method::Method::DELETE,
            &|mut request| {
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            None,
        )?;

        let response = self
            .pop_receipt_client
            .http_client()
            .execute_request_check_status(request.0, http::status::StatusCode::NO_CONTENT)
            .await?;

        Ok((&response).try_into()?)
    }
}
