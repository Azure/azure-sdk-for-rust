use crate::prelude::*;
use azure_core::{
    error::{Error, ErrorKind},
    headers::*,
    prelude::*,
    CollectedResponse, Context, Etag, Method, StatusCode,
};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use std::convert::{TryFrom, TryInto};
use url::Url;

#[derive(Debug, Clone)]
pub struct SubmitTransactionBuilder {
    partition_key_client: PartitionKeyClient,
    transaction: Transaction,
    timeout: Option<Timeout>,
    context: Context,
}

impl SubmitTransactionBuilder {
    pub(crate) fn new(partition_key_client: PartitionKeyClient, transaction: Transaction) -> Self {
        Self {
            partition_key_client,
            transaction,
            timeout: None,
            context: Context::new(),
        }
    }

    setters! {
        timeout: Timeout => Some(timeout),
        context: Context => context,
    }

    pub fn into_future(mut self) -> FutureResponse {
        Box::pin(async move {
            let mut url = self.partition_key_client.table_client().url().to_owned();
            url.path_segments_mut()
                .map_err(|()| Error::message(ErrorKind::Other, "invalid table URL"))?
                .pop()
                .push("$batch");

            self.timeout.append_to_url_query(&mut url);

            let request_body = Some(self.transaction.to_string()?.into());

            let mut headers = Headers::new();
            headers.insert(
                CONTENT_TYPE,
                &format!(
                    "multipart/mixed; boundary=batch_{}",
                    self.transaction.batch_uuid().hyphenated()
                ),
            );

            let mut request = self.partition_key_client.finalize_request(
                url,
                Method::Get,
                headers,
                request_body,
            )?;

            let response = self
                .partition_key_client
                .send(&mut self.context, &mut request)
                .await?;

            let collected_response = CollectedResponse::from_response(response).await?;
            collected_response.try_into()
        })
    }
}

pub type FutureResponse =
    futures::future::BoxFuture<'static, azure_core::Result<SubmitTransactionResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for SubmitTransactionBuilder {
    type IntoFuture = FutureResponse;
    type Output = <FutureResponse as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone)]
pub struct OperationResponse {
    pub status_code: StatusCode,
    pub location: Option<Url>,
    pub data_service_id: Option<String>,
    pub etag: Option<Etag>,
}

impl Default for OperationResponse {
    fn default() -> Self {
        Self {
            status_code: StatusCode::Ok,
            location: None,
            data_service_id: None,
            etag: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SubmitTransactionResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub operation_responses: Vec<OperationResponse>,
}

impl TryFrom<CollectedResponse> for SubmitTransactionResponse {
    type Error = Error;

    fn try_from(response: CollectedResponse) -> azure_core::Result<Self> {
        let body = std::str::from_utf8(response.body())?;

        let mut operation_responses = Vec::new();

        for change_set_response in body
            .split("\n--changesetresponse_")
            .into_iter()
            .filter(|change_set_response| change_set_response.contains("HTTP/1.1"))
        {
            trace!("changeset --> {}", change_set_response);

            let mut operation_response = OperationResponse::default();

            for line in change_set_response.lines() {
                if line.starts_with("HTTP/1.1") {
                    let status_code = line.split_whitespace().nth(1).ok_or_else(|| {
                        Error::message(ErrorKind::Other, "missing HTTP status code")
                    })?;
                    let status_code = status_code.parse::<u16>().map_err(|_| {
                        Error::with_message(ErrorKind::DataConversion, || {
                            format!("invalid HTTP status code `{status_code}`")
                        })
                    })?;
                    operation_response.status_code =
                        StatusCode::try_from(status_code).map_err(|_| {
                            Error::with_message(ErrorKind::DataConversion, || {
                                format!("invalid status code {status_code}")
                            })
                        })?;
                } else if line.starts_with("Location:") {
                    operation_response.location = Some(
                        line.split_whitespace()
                            .nth(1)
                            .ok_or_else(|| {
                                Error::message(ErrorKind::Other, "invalid Location header")
                            })?
                            .parse()?,
                    );
                } else if line.starts_with("DataServiceId:") {
                    operation_response.data_service_id = Some(
                        line.split_whitespace()
                            .nth(1)
                            .ok_or_else(|| {
                                {
                                    {
                                        Error::message(
                                            ErrorKind::Other,
                                            "invalid DataServiceId header",
                                        )
                                    }
                                }
                            })?
                            .to_owned(),
                    );
                } else if line.starts_with("ETag:") {
                    operation_response.etag = Some(
                        line.split_whitespace()
                            .nth(1)
                            .ok_or_else(|| Error::message(ErrorKind::Other, "invalid ETag header"))?
                            .into(),
                    );
                }
            }

            operation_responses.push(operation_response);
        }

        Ok(SubmitTransactionResponse {
            common_storage_response_headers: response.headers().try_into()?,
            operation_responses,
        })
    }
}
