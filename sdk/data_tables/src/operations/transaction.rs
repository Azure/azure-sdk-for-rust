use crate::{
    prelude::*, transaction::TransactionOperations, transaction_operation::TransactionOperation,
    IfMatchCondition,
};
use azure_core::{
    error::{Error, ErrorKind},
    headers::*,
    prelude::*,
    CollectedResponse, Etag, Method, Request, StatusCode,
};
use azure_storage::headers::CommonStorageResponseHeaders;
use serde::Serialize;
use std::convert::{TryFrom, TryInto};
use url::Url;

operation! {
    Transaction,
    client: PartitionKeyClient,
    transaction: TransactionOperations,
    ?timeout: Timeout
}

impl TransactionBuilder {
    /// Insert a new entity into a table
    ///
    /// ref: <https://docs.microsoft.com/en-us/rest/api/storageservices/insert-entity>
    pub fn insert<E: Serialize>(mut self, entity: E) -> azure_core::Result<Self> {
        let body = serde_json::to_string(&entity)?;

        let mut url = self.client.table_client().url()?;
        url.path_segments_mut()
            .map_err(|()| Error::message(ErrorKind::Other, "invalid table URL"))?
            .pop()
            .push(self.client.table_client().table_name());

        let mut request = Request::new(url, Method::Post);
        request.insert_header(ACCEPT, "application/json;odata=fullmetadata");
        request.insert_headers(&ContentType::APPLICATION_JSON);
        request.set_body(body);

        self.transaction.add(TransactionOperation::new(request));

        Ok(self)
    }

    /// Update an existing entity in a table. The Update Entity operation
    /// replaces the entire entity and can be used to remove properties.
    ///
    /// ref: <https://docs.microsoft.com/en-us/rest/api/storageservices/update-entity2>
    pub fn update<RK: Into<String>, E: Serialize>(
        self,
        row_key: RK,
        entity: E,
        match_condition: Option<IfMatchCondition>,
    ) -> azure_core::Result<Self> {
        self.entity_operation(row_key, entity, Method::Put, match_condition)
    }

    /// Replaces an existing entity or inserts a new entity if it does not exist
    /// in the table. Because this operation can insert or update an entity, it
    /// is also known as an upsert operation.
    ///
    /// ref: <https://docs.microsoft.com/en-us/rest/api/storageservices/insert-or-replace-entity>
    pub fn insert_or_replace<RK: Into<String>, E: Serialize>(
        self,
        row_key: RK,
        entity: E,
    ) -> azure_core::Result<Self> {
        self.entity_operation(row_key, entity, Method::Put, None)
    }

    /// Update an existing entity by updating the entity's properties. This
    /// operation does not replace the existing entity, as the Update Entity
    /// operation does.
    ///
    /// ref: <https://docs.microsoft.com/en-us/rest/api/storageservices/merge-entity>
    pub fn merge<RK: Into<String>, E: Serialize>(
        self,
        row_key: RK,
        entity: E,
        match_condition: Option<IfMatchCondition>,
    ) -> azure_core::Result<Self> {
        self.entity_operation(row_key, entity, Method::Merge, match_condition)
    }

    /// Update an existing entity or inserts a new entity if it does not exist
    /// in the table. Because this operation can insert or update an entity, it
    /// is also known as an upsert operation.
    ///
    /// ref: <https://docs.microsoft.com/en-us/rest/api/storageservices/insert-or-merge-entity>
    pub fn insert_or_merge<RK: Into<String>, E: Serialize>(
        self,
        row_key: RK,
        entity: E,
    ) -> azure_core::Result<Self> {
        self.entity_operation(row_key, entity, Method::Merge, None)
    }

    /// Delete an existing entity in a table.
    ///
    /// ref: <https://docs.microsoft.com/en-us/rest/api/storageservices/delete-entity1>
    pub fn delete<RK: Into<String>>(
        mut self,
        row_key: RK,
        match_condition: Option<IfMatchCondition>,
    ) -> azure_core::Result<Self> {
        let entity_client = self.client.entity_client(row_key);
        let url = entity_client.url()?;

        let mut request = Request::new(url, Method::Delete);
        request.insert_header(ACCEPT, "application/json;odata=minimalmetadata");

        let match_condition = match_condition.unwrap_or(IfMatchCondition::Any);
        request.add_mandatory_header(&match_condition);
        request.set_body("");

        self.transaction.add(TransactionOperation::new(request));
        Ok(self)
    }

    pub fn into_future(mut self) -> Transaction {
        Box::pin(async move {
            let mut url = self.client.table_client().url()?;
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

            let mut request =
                PartitionKeyClient::finalize_request(url, Method::Post, headers, request_body)?;

            let response = self.client.send(&mut self.context, &mut request).await?;

            let collected_response = CollectedResponse::from_response(response).await?;
            collected_response.try_into()
        })
    }

    fn entity_operation<RK: Into<String>, E: Serialize>(
        mut self,
        row_key: RK,
        entity: E,
        method: Method,
        match_condition: Option<IfMatchCondition>,
    ) -> azure_core::Result<Self> {
        let body = serde_json::to_string(&entity)?;
        let entity_client = self.client.entity_client(row_key);
        let url = entity_client.url()?;

        let mut request = Request::new(url, method);
        request.insert_header(ACCEPT, "application/json;odata=fullmetadata");
        request.insert_headers(&ContentType::APPLICATION_JSON);
        request.set_body(body);
        request.add_optional_header(&match_condition);

        self.transaction.add(TransactionOperation::new(request));
        Ok(self)
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
pub struct TransactionResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub operation_responses: Vec<OperationResponse>,
}

impl TryFrom<CollectedResponse> for TransactionResponse {
    type Error = Error;

    fn try_from(response: CollectedResponse) -> azure_core::Result<Self> {
        let body = std::str::from_utf8(response.body())?;

        let mut operation_responses = Vec::new();

        for change_set_response in body
            .split("\n--changesetresponse_")
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

        Ok(TransactionResponse {
            common_storage_response_headers: response.headers().try_into()?,
            operation_responses,
        })
    }
}
