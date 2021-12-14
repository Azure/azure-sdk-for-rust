use azure_core::{headers::CommonStorageResponseHeaders, Etag};
use bytes::Bytes;
use http::{Response, StatusCode};
use std::convert::{TryFrom, TryInto};
use url::Url;

#[derive(Debug, Clone, Default)]
pub struct OperationResponse {
    pub status_code: StatusCode,
    pub location: Option<Url>,
    pub data_service_id: Option<String>,
    pub etag: Option<Etag>,
}

#[derive(Debug, Clone)]
pub struct SubmitTransactionResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub operation_responses: Vec<OperationResponse>,
}

impl TryFrom<&Response<Bytes>> for SubmitTransactionResponse {
    type Error = crate::Error;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        let body = std::str::from_utf8(response.body())?;
        debug!("{}", body);
        debug!("headers == {:#?}", response.headers());

        let mut operation_responses = Vec::new();

        for change_set_response in body
            .split("\n--changesetresponse_")
            .into_iter()
            .filter(|change_set_response| change_set_response.contains("HTTP/1.1"))
        {
            trace!("changeset --> {}", change_set_response);

            let mut operation_response = OperationResponse::default();

            for line in change_set_response.lines().into_iter() {
                if line.starts_with("HTTP/1.1") {
                    operation_response.status_code = line
                        .split_whitespace()
                        .nth(1)
                        .ok_or_else(|| {
                            crate::Error::TransactionResponseParseError(
                                "missing HTTP status code".to_owned(),
                            )
                        })?
                        .parse()?;
                } else if line.starts_with("Location:") {
                    operation_response.location = Some(
                        line.split_whitespace()
                            .nth(1)
                            .ok_or_else(|| {
                                crate::Error::TransactionResponseParseError(
                                    "invalid Location header".to_owned(),
                                )
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
                                        crate::Error::TransactionResponseParseError(
                                            "invalid DataServiceId header".to_owned(),
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
                            .ok_or_else(|| {
                                {
                                    {
                                        crate::Error::TransactionResponseParseError(
                                            "invalid ETag header".to_owned(),
                                        )
                                    }
                                }
                            })?
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
