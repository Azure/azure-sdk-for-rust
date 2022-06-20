use crate::{prelude::*, responses::*};
use azure_core::{
    error::{Error, ErrorKind, ResultExt},
    prelude::*,
};
use http::method::Method;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct SubmitTransactionBuilder<'a> {
    partition_key_client: &'a PartitionKeyClient,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId>,
}

impl<'a> SubmitTransactionBuilder<'a> {
    pub(crate) fn new(partition_key_client: &'a PartitionKeyClient) -> Self {
        Self {
            partition_key_client,
            timeout: None,
            client_request_id: None,
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
        client_request_id: ClientRequestId => Some(client_request_id),
    }

    pub async fn execute(
        &self,
        batch: &Transaction,
    ) -> azure_core::Result<SubmitTransactionResponse> {
        let mut url = self.partition_key_client.table_client().url().to_owned();
        url.path_segments_mut()
            .map_err(|()| Error::message(ErrorKind::Other, "invalid table URL"))?
            .pop()
            .push("$batch");

        self.timeout.append_to_url_query(&mut url);

        let payload = batch.to_string().map_kind(ErrorKind::DataConversion)?;

        let mut request = self.partition_key_client.prepare_request(
            url.as_str(),
            Method::POST,
            Some(bytes::Bytes::from(payload)),
        )?;
        request.add_optional_header(&self.client_request_id);
        request.insert_header(
            "Content-Type",
            &format!(
                "multipart/mixed; boundary=batch_{}",
                batch.batch_uuid().hyphenated()
            ),
        );

        let response = self
            .partition_key_client
            .http_client()
            .execute_request_check_status(&request)
            .await?;

        response.try_into()
    }
}
