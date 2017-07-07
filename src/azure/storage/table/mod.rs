mod batch;

pub use self::batch::BatchItem;

use self::batch::generate_batch_payload;
use mime::Mime;
use azure::core::errors::{AzureError, check_status_extract_body, extract_status_and_body,
                          UnexpectedHTTPResult};
use azure::storage::client::Client;
use azure::storage::rest_client::ServiceType;
use hyper::Method;
use hyper::client::FutureResponse;
use hyper::header::{Accept, ContentType, Headers, IfMatch, qitem};
use hyper::StatusCode;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json;

use futures::future::*;

const TABLE_TABLES: &'static str = "TABLES";

pub struct TableService {
    client: Client,
}

impl TableService {
    pub fn new(client: Client) -> Self {
        TableService { client: client }
    }

    pub fn list_tables(&self) -> impl Future<Item = Vec<String>, Error = AzureError> {
        self.query_entities(TABLE_TABLES, None).and_then(
            |entities| {
                let e: Vec<String> = entities
                    .into_iter()
                    .map(|x: TableEntity| x.TableName)
                    .collect();
                ok(e)
            },
        )
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
        let req = self.request_with_default_header(TABLE_TABLES, Method::Post, Some(body));

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::Created)
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
        let req = self.request_with_default_header(path, Method::Get, None);
        done(req).from_err().and_then(move |future_response| {
            extract_status_and_body(future_response).and_then(
                move |(status, body)| if status == StatusCode::NotFound {
                    ok(None)
                } else if status != StatusCode::Ok {
                    err(AzureError::UnexpectedHTTPResult(
                        UnexpectedHTTPResult::new(StatusCode::Ok, status, &body),
                    ))
                } else {
                    match serde_json::from_str(&body) {
                        Ok(item) => ok(Some(item)),
                        Err(error) => err(error.into()),
                    }
                },
            )
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

        let req = self.request_with_default_header(path.as_str(), Method::Get, None);

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::Ok).and_then(move |body| {
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
    ) -> Result<FutureResponse, AzureError>
    where
        T: Serialize,
    {
        let obj_ser = serde_json::to_string(entity)?;
        self.request_with_default_header(table_name, Method::Post, Some(&obj_ser))
    }

    pub fn insert_entity<T: Serialize>(
        &self,
        table_name: &str,
        entity: &T,
    ) -> impl Future<Item = (), Error = AzureError> {
        let req = self._prepare_insert_entity(table_name, entity);

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::Created)
                .and_then(move |_| ok(()))
        })
    }


    fn _prepare_update_entity<T>(
        &self,
        table_name: &str,
        partition_key: &str,
        row_key: &str,
        entity: &T,
    ) -> Result<FutureResponse, AzureError>
    where
        T: Serialize,
    {
        let body = &serde_json::to_string(entity)?;
        let path = &entity_path(table_name, partition_key, row_key);
        self.request_with_default_header(path, Method::Put, Some(body))
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
            check_status_extract_body(future_response, StatusCode::NoContent)
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

        let req = self.request(path, Method::Delete, None, |ref mut headers| {
            headers.set(Accept(vec![qitem(get_json_mime_nometadata())]));
            headers.set(IfMatch::Any);
        });
        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::NoContent)
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
            self.client.get_uri_prefix(ServiceType::Table).as_str(),
            table_name,
            partition_key,
            batch_items,
        );

        let req = self.request("$batch", Method::Post, Some(payload), |ref mut headers| {
            headers.set(ContentType(get_batch_mime()));
        });
        done(req).from_err().and_then(move |future_response| {
            check_status_extract_body(future_response, StatusCode::Accepted).and_then(move |_| {
                // TODO deal with body response, handle batch failure.
                // let ref body = try!(get_response_body(&mut response));
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
    ) -> Result<FutureResponse, AzureError> {
        self.request(segment, method, request_str, |ref mut headers| {
            headers.set(Accept(vec![qitem(get_json_mime_nometadata())]));
            if request_str.is_some() {
                headers.set(ContentType(get_default_json_mime()));
            }
        })
    }

    fn request<F>(
        &self,
        segment: &str,
        method: Method,
        request_str: Option<&str>,
        headers_func: F,
    ) -> Result<FutureResponse, AzureError>
    where
        F: FnOnce(&mut Headers),
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
pub fn get_default_json_mime() -> Mime {
    "application/json; charset=utf-8".parse().unwrap()
}

#[inline]
pub fn get_json_mime_nometadata() -> Mime {
    "application/json; odata=nometadata".parse().unwrap()
}

#[inline]
pub fn get_batch_mime() -> Mime {
    "multipart/mixed; boundary=batch_a1e9d677-b28b-435e-a89e-87e6a768a431"
        .parse()
        .unwrap()
}
