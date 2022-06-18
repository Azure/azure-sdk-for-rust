use azure_core::{
    error::{Error, ErrorKind},
    CollectedResponse, Etag,
};
use azure_storage::core::headers::CommonStorageResponseHeaders;
use http::StatusCode;
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
                    operation_response.status_code = line
                        .split_whitespace()
                        .nth(1)
                        .ok_or_else(|| {
                            Error::message(ErrorKind::Other, "missing HTTP status code")
                        })?
                        .parse()?;
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
