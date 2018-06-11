mod batch;

pub use self::batch::BatchItem;

use self::batch::generate_batch_payload;
use azure::core::errors::{
    check_status_extract_body, extract_status_and_body, AzureError, UnexpectedHTTPResult,
};
use azure::storage::client::Client;
use azure::storage::rest_client::ServiceType;
use hyper::{header::{self, HeaderValue}, Method, StatusCode, client::ResponseFuture};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;

use futures::future::*;

const TABLE_TABLES: &str = "TABLES";

pub struct TableService {
    client: Client,
}

impl TableService {
    pub fn new(client: Client) -> Self {
        TableService { client }
    }

    pub fn list_tables(&self) -> impl Future<Item = Vec<String>, Error = AzureError> {
        self.query_entities(TABLE_TABLES, None)
            .and_then(|entities| {
                let e: Vec<String> = entities
                    .into_iter()
                    .map(|x: TableEntity| x.TableName)
                    .collect();
                ok(e)
            })
    }

    // Create table if not exists.
    pub fn create_table<T: Into<String>>(
        &self,
        table_name: T,
    ) -> impl Future<Item = (), Error = AzureError> {
        let body = &serde_json::to_string(&TableEntity {
            TableName: table_name.into(),
        }).unwrap();
        debug!("body == {}", body);
        let req = self.request_with_default_header(TABLE_TABLES, Method::POST, Some(body));

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::CREATED)
                .and_then(move |_| ok(()))
        })
    }

    pub fn get_entity<T: DeserializeOwned>(
        &self,
        table_name: &str,
        partition_key: &str,
        row_key: &str,
    ) -> impl Future<Item = Option<T>, Error = AzureError> {
        let path = &entity_path(table_name, partition_key, row_key);
        let req = self.request_with_default_header(path, Method::GET, None);
        done(req).from_err().and_then(move |future_response| {
            extract_status_and_body(future_response).and_then(move |(status, body)| {
                if status == StatusCode::NOT_FOUND {
                    ok(None)
                } else if status != StatusCode::OK {
                    err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(
                        StatusCode::OK,
                        status,
                        &body,
                    )))
                } else {
                    match serde_json::from_str(&body) {
                        Ok(item) => ok(Some(item)),
                        Err(error) => err(error.into()),
                    }
                }
            })
        })
    }

    pub fn query_entities<T: DeserializeOwned>(
        &self,
        table_name: &str,
        query: Option<&str>,
    ) -> impl Future<Item = Vec<T>, Error = AzureError> {
        let mut path = table_name.to_owned();
        if let Some(clause) = query {
            path.push_str("?");
            path.push_str(clause);
        }

        let req = self.request_with_default_header(path.as_str(), Method::GET, None);

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::OK).and_then(move |body| {
                done(serde_json::from_str::<EntityCollection<T>>(&body))
                    .from_err()
                    .and_then(|ec| ok(ec.value))
            })
        })
    }

    fn _prepare_insert_entity<T>(
        &self,
        table_name: &str,
        entity: &T,
    ) -> Result<ResponseFuture, AzureError>
    where
        T: Serialize,
    {
        let obj_ser = serde_json::to_string(entity)?;
        self.request_with_default_header(table_name, Method::POST, Some(&obj_ser))
    }

    pub fn insert_entity<T: Serialize>(
        &self,
        table_name: &str,
        entity: &T,
    ) -> impl Future<Item = (), Error = AzureError> {
        let req = self._prepare_insert_entity(table_name, entity);

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::CREATED)
                .and_then(move |_| ok(()))
        })
    }

    fn _prepare_update_entity<T>(
        &self,
        table_name: &str,
        partition_key: &str,
        row_key: &str,
        entity: &T,
    ) -> Result<ResponseFuture, AzureError>
    where
        T: Serialize,
    {
        let body = &serde_json::to_string(entity)?;
        let path = &entity_path(table_name, partition_key, row_key);
        self.request_with_default_header(path, Method::PUT, Some(body))
    }

    pub fn update_entity<T: Serialize>(
        &self,
        table_name: &str,
        partition_key: &str,
        row_key: &str,
        entity: &T,
    ) -> impl Future<Item = (), Error = AzureError> {
        let req = self._prepare_update_entity(table_name, partition_key, row_key, entity);
        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::NO_CONTENT)
                .and_then(move |_| ok(()))
        })
    }

    pub fn delete_entity(
        &self,
        table_name: &str,
        partition_key: &str,
        row_key: &str,
    ) -> impl Future<Item = (), Error = AzureError> {
        let path = &entity_path(table_name, partition_key, row_key);

        let req = self.request(path, Method::DELETE, None, |ref mut request| {
            request.header(header::ACCEPT, HeaderValue::from_static(get_json_mime_nometadata()));
            request.header(header::IF_MATCH, header::HeaderValue::from_static("*"));
        });
        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::NO_CONTENT)
                .and_then(move |_| ok(()))
        })
    }

    pub fn batch<T: Serialize>(
        &self,
        table_name: &str,
        partition_key: &str,
        batch_items: &[BatchItem<T>],
    ) -> impl Future<Item = (), Error = AzureError> {
        let payload = &generate_batch_payload(
            self.client.get_uri_prefix(&ServiceType::Table).as_str(),
            table_name,
            partition_key,
            batch_items,
        );

        let req = self.request("$batch", Method::POST, Some(payload), |ref mut request| {
            request.header(header::CONTENT_TYPE, header::HeaderValue::from_static(get_batch_mime()));
        });
        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::ACCEPTED).and_then(move |_| {
                // TODO deal with body response, handle batch failure.
                // let ref body = get_response_body(&mut response)?;
                // info!("{}", body);
                ok(())
            })
        })
    }

    fn request_with_default_header(
        &self,
        segment: &str,
        method: Method,
        request_str: Option<&str>,
    ) -> Result<ResponseFuture, AzureError> {
        self.request(segment, method, request_str, |ref mut request| {
            request.header(header::ACCEPT, HeaderValue::from_static(get_json_mime_nometadata()));
            if request_str.is_some() {
                request.header(header::CONTENT_TYPE, HeaderValue::from_static(get_default_json_mime()));
            }
        })
    }

    fn request<F>(
        &self,
        segment: &str,
        method: Method,
        request_str: Option<&str>,
        headers_func: F,
    ) -> Result<ResponseFuture, AzureError>
    where
        F: FnOnce(&mut ::http::request::Builder),
    {
        trace!("{:?} {}", method, segment);
        if let Some(body) = request_str {
            trace!("Request: {}", body);
        }

        let request_vec: Option<&[u8]> = match request_str {
            Some(s) => Some(s.as_bytes()),
            None => None,
        };

        self.client
            .perform_table_request(segment, method, headers_func, request_vec)
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct TableEntity {
    TableName: String,
}

#[derive(Deserialize)]
struct EntityCollection<T> {
    value: Vec<T>,
}

#[inline]
fn entity_path(table_name: &str, partition_key: &str, row_key: &str) -> String {
    table_name.to_owned() + "(PartitionKey='" + partition_key + "',RowKey='" + row_key + "')"
}

#[inline]
pub fn get_default_json_mime() -> &'static str {
    "application/json; charset=utf-8"
}

#[inline]
pub fn get_json_mime_nometadata() -> &'static str {
    "application/json; odata=nometadata"
}

#[inline]
pub fn get_batch_mime() -> &'static str {
    "multipart/mixed; boundary=batch_a1e9d677-b28b-435e-a89e-87e6a768a431"
}
