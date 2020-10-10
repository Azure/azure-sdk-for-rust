use crate::core::key_client::KeyClient;
use crate::core::prelude::*;
use crate::core::{
    client, get_default_json_mime, get_json_mime_fullmetadata, get_json_mime_nometadata,
    ConnectionString, ServiceType,
};
use azure_sdk_core::errors::{check_status_extract_body, AzureError};
use http::request::Builder;
use hyper::{
    client::ResponseFuture,
    header::{self, HeaderValue},
};
use hyper::{Method, StatusCode};
use log;
use serde_json;

const TABLE_TABLES: &str = "TABLES";

/// Requetsed meta data detail
pub enum MetadataDetail {
    Default,
    None,
    Full,
}

#[derive(Clone)]
pub struct TableClient<C>
where
    C: Client,
{
    client: C,
}

impl TableClient<KeyClient> {
    /// Create a new `TableClient` using a key.
    pub fn new(account: &str, key: &str) -> Self {
        TableClient {
            client: client::with_access_key(account, key),
        }
    }

    /// Create a new `TableClient` using a SAS token.
    pub fn azure_sas(account: &str, sas_token: &str) -> Self {
        TableClient {
            client: client::with_azure_sas(account, sas_token),
        }
    }

    pub fn from_connection_string(connection_string: &str) -> Result<Self, AzureError> {
        match ConnectionString::new(connection_string)? {
            ConnectionString {
                account_name: Some(account),
                account_key: Some(_),
                sas: Some(sas_token),
                ..
            } => {
                log::warn!("Both account key and SAS defined in connection string. Using only the provided SAS.");
                Ok(TableClient {
                    client: client::with_azure_sas(account, sas_token),
                })
            }
            ConnectionString {
                account_name: Some(account),
                sas: Some(sas_token),
                ..
            } => Ok(TableClient {
                client: client::with_azure_sas(account, sas_token),
            }),
            ConnectionString {
                account_name: Some(account),
                account_key: Some(key),
                ..
            } => Ok(TableClient {
                client: client::with_access_key(account, key),
            }),
            _ => {
                return Err(AzureError::GenericErrorWithText(
                    "Could not create an Azure Table client from the provided connection string. Please validate that you have specified the account name and means of authentication (key, SAS, etc.)."
                        .to_owned(),
                ))
            }
        }
    }
}

impl<C> TableClient<C>
where
    C: Client,
{
    pub async fn list_tables(&self) -> Result<Vec<String>, AzureError> {
        let future_response = self.request_with_default_header(
            TABLE_TABLES,
            &Method::GET,
            None,
            MetadataDetail::None,
            &|req| req,
        )?;
        let body = check_status_extract_body(future_response, StatusCode::OK).await?;
        let entities = serde_json::from_str::<TableDataCollection>(&body)?;
        // todo: shall we use the continuation or query result always fits into a single page
        let e: Vec<String> = entities.value.into_iter().map(|x| x.table_name).collect();
        Ok(e)
    }

    // Create table if not exists.
    pub async fn create_table<T: Into<String>>(&self, table_name: T) -> Result<(), AzureError> {
        let body = &serde_json::to_string(&TableData {
            table_name: table_name.into(),
        })
        .unwrap();
        log::debug!("body == {}", body);
        let future_response = self.request_with_default_header(
            TABLE_TABLES,
            &Method::POST,
            Some(body),
            MetadataDetail::None,
            &|req| req,
        )?;

        check_status_extract_body(future_response, StatusCode::CREATED).await?;
        Ok(())
    }

    pub fn get_uri_prefix(&self) -> String {
        self.client.get_uri_prefix(ServiceType::Table)
    }

    pub(crate) fn request_with_default_header(
        &self,
        segment: &str,
        method: &Method,
        request_str: Option<&str>,
        metadata: MetadataDetail,
        http_header_adder: &dyn Fn(Builder) -> Builder,
    ) -> Result<ResponseFuture, AzureError> {
        self.request(segment, method, request_str, &|mut request| {
            request = match metadata {
                MetadataDetail::Full => request.header(
                    header::ACCEPT,
                    HeaderValue::from_static(get_json_mime_fullmetadata()),
                ),
                MetadataDetail::None => request.header(
                    header::ACCEPT,
                    HeaderValue::from_static(get_json_mime_nometadata()),
                ),
                MetadataDetail::Default => request.header(
                    header::ACCEPT,
                    HeaderValue::from_static(get_default_json_mime()),
                ),
            };
            if request_str.is_some() {
                request = request.header(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(get_default_json_mime()),
                );
            }

            http_header_adder(request)
        })
    }

    pub(crate) fn request(
        &self,
        segment: &str,
        method: &Method,
        request_str: Option<&str>,
        http_header_adder: &dyn Fn(Builder) -> Builder,
    ) -> Result<ResponseFuture, AzureError> {
        log::trace!("{:?} {}", method, segment);
        if let Some(body) = request_str {
            log::trace!("Request: {}", body);
        }

        let request_vec: Option<&[u8]> = match request_str {
            Some(s) => Some(s.as_bytes()),
            None => None,
        };

        self.client
            .perform_table_request(segment, method, http_header_adder, request_vec)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct TableData {
    table_name: String,
}

#[derive(Serialize, Deserialize)]
struct TableDataCollection {
    value: Vec<TableData>,
}

#[inline]
pub(crate) fn get_batch_mime() -> &'static str {
    "multipart/mixed; boundary=batch_a1e9d677-b28b-435e-a89e-87e6a768a431"
}

pub(crate) fn entity_path(table_name: &str, partition_key: &str, row_key: &str) -> String {
    table_name.to_owned() + "(PartitionKey='" + partition_key + "',RowKey='" + row_key + "')"
}
