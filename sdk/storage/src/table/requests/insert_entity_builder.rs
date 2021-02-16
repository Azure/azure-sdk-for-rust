use crate::clients::TableClient;
use crate::table::requests::{PartitionKeyMissing, PartitionKeySet, RowKeyMissing, RowKeySet};
use azure_core::headers::add_optional_header;
use azure_core::prelude::*;
use bytes::Bytes;
use http::method::Method;
use http::status::StatusCode;
use serde::Serialize;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct InsertEntityBuilder<'a, E, RowKey, PartitionKey>
where
    E: Serialize,
{
    table_client: &'a TableClient,
    row_key: Option<&'a str>,
    partition_key: Option<&'a str>,
    p_partition_key: PhantomData<PartitionKey>,
    p_row_key: PhantomData<RowKey>,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId<'a>>,
    entity: &'a E,
}

impl<'a, E> InsertEntityBuilder<'a, E, RowKeyMissing, PartitionKeyMissing>
where
    E: Serialize,
{
    pub(crate) fn new(table_client: &'a TableClient, entity: &'a E) -> Self {
        Self {
            table_client,
            partition_key: None,
            row_key: None,
            p_partition_key: PhantomData,
            p_row_key: PhantomData,
            timeout: None,
            client_request_id: None,
            entity,
        }
    }
}

impl<'a, E, RowKey, PartitionKey> InsertEntityBuilder<'a, E, RowKey, PartitionKey>
where
    E: Serialize,
{
    pub fn with_partition_key(
        self,
        partition_key: &'a str,
    ) -> InsertEntityBuilder<'a, E, RowKey, PartitionKeySet> {
        InsertEntityBuilder {
            table_client: self.table_client,
            row_key: self.row_key,
            partition_key: Some(partition_key),
            p_partition_key: PhantomData,
            p_row_key: PhantomData,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
            entity: self.entity,
        }
    }

    pub fn with_row_key(
        self,
        row_key: &'a str,
    ) -> InsertEntityBuilder<'a, E, RowKeySet, PartitionKey> {
        InsertEntityBuilder {
            table_client: self.table_client,
            row_key: Some(row_key),
            partition_key: self.partition_key,
            p_partition_key: PhantomData,
            p_row_key: PhantomData,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
            entity: self.entity,
        }
    }

    setters! {
        timeout : Timeout => Some(timeout),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
    }
}

impl<'a, E> InsertEntityBuilder<'a, E, RowKeySet, PartitionKeySet>
where
    E: Serialize,
{
    pub async fn execute(&self) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        let mut url = self
            .table_client
            .table_service_client()
            .storage_account_client()
            .table_storage_url()
            .join(self.table_client.table_name())?;

        self.timeout.append_to_url_query(&mut url);

        debug!("generated url = {}", url);

        #[derive(Debug, Clone, Serialize)]
        struct _TableEntity<'b, T: Serialize> {
            #[serde(rename = "RowKey")]
            pub row_key: &'b str,
            #[serde(rename = "PartitionKey")]
            pub partition_key: &'b str,
            #[serde(flatten)]
            pub payload: &'b T,
        }
        let table_entity = _TableEntity {
            row_key: self.row_key.unwrap(),
            partition_key: self.partition_key.unwrap(),
            payload: &self.entity,
        };

        let json = serde_json::to_string(&table_entity)?;
        debug!("json payload == {}", json);

        let request = self.table_client.prepare_request(
            url.as_str(),
            &Method::POST,
            &|mut request| {
                request = request.header(http::header::CONTENT_TYPE, "application/json");
                request =
                    request.header(http::header::ACCEPT, "application/json;odata=fullmetadata");
                request = request.header("Prefer", "return-no-content");
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            Some(Bytes::from(json)),
        )?;

        println!("request == {:?}", request);

        let response = self
            .table_client
            .table_service_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(request.0, StatusCode::NO_CONTENT)
            .await?;

        println!("response == {:?}", response);

        let body = std::str::from_utf8(response.body())?;
        println!("body == {}", body);

        Ok(())
    }
}
