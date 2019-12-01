mod batch;
use self::batch::generate_batch_payload;
pub use self::batch::BatchItem;
use azure_sdk_core::errors::{
    check_status_extract_body, check_status_extract_headers_and_body, extract_status_and_body,
    AzureError, UnexpectedHTTPResult,
};
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_core::{
    get_default_json_mime, get_json_mime_fullmetadata, get_json_mime_nometadata, ServiceType,
};
use futures::stream::Stream;
use http::HeaderMap;
use hyper::{
    client::ResponseFuture,
    header::{self, HeaderValue},
    Method, StatusCode,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;

const TABLE_TABLES: &str = "TABLES";

#[derive(Clone)]
pub struct TableService {
    client: Client,
}

impl TableService {
    pub fn new(client: Client) -> Self {
        TableService { client }
    }

    pub async fn list_tables(&self) -> Result<Vec<String>, AzureError> {
        let entities = self.query_entities(TABLE_TABLES, None).await?;
        let e: Vec<String> = entities
            .into_iter()
            .map(|x: TableEntity| x.TableName)
            .collect();
        Ok(e)
    }

    // Create table if not exists.
    pub async fn create_table<T: Into<String>>(&self, table_name: T) -> Result<(), AzureError> {
        let body = &serde_json::to_string(&TableEntity {
            TableName: table_name.into(),
        })
        .unwrap();
        debug!("body == {}", body);
        let future_response =
            self.request_with_default_header(TABLE_TABLES, &Method::POST, Some(body), false)?;

        check_status_extract_body(future_response, StatusCode::CREATED).await?;
        Ok(())
    }

    pub async fn get_entity<T: DeserializeOwned>(
        &self,
        table_name: &str,
        partition_key: &str,
        row_key: &str,
    ) -> Result<Option<T>, AzureError> {
        let path = &entity_path(table_name, partition_key, row_key);
        let future_response = self.request_with_default_header(path, &Method::GET, None, false)?;
        let (status, body) = extract_status_and_body(future_response).await?;

        if status == StatusCode::NOT_FOUND {
            Ok(None)
        } else if status != StatusCode::OK {
            Err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(
                StatusCode::OK,
                status,
                &body,
            )))
        } else {
            Ok(serde_json::from_str(&body)?)
        }
    }

    pub async fn query_entities<T: DeserializeOwned>(
        &self,
        table_name: &str,
        query: Option<&str>,
    ) -> Result<Vec<T>, AzureError> {
        let mut path = table_name.to_owned();
        if let Some(clause) = query {
            path.push_str("?");
            path.push_str(clause);
        }

        let future_response =
            self.request_with_default_header(path.as_str(), &Method::GET, None, false)?;
        let body = check_status_extract_body(future_response, StatusCode::OK).await?;
        let ec = serde_json::from_str::<EntityCollection<T>>(&body)?;
        Ok(ec.value)
    }

    async fn query_entity_collection<T: DeserializeOwned>(
        &self,
        table_name: &str,
        query: Option<&str>,
        continuation: Option<&Continuation>,
        fullmetadata: bool,
    ) -> Result<EntityCollection<T>, AzureError> {
        let mut path = table_name.to_owned();
        path.push_str("?");
        if let Some(clause) = query {
            path.push_str(clause);
        }
        if let Some(cont) = continuation {
            path.push_str("&NextPartitionKey=");
            path.push_str(&cont.next_partition_key);
            path.push_str("&NextRowKey=");
            path.push_str(&cont.next_row_key);
        }

        let future_response =
            self.request_with_default_header(path.as_str(), &Method::GET, None, fullmetadata)?;

        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::OK).await?;
        Ok(
            serde_json::from_slice::<EntityCollection<T>>(&body).map(|mut ec| {
                ec.continuation = continuation_from_headers(&headers);
                ec
            })?,
        )
    }

    fn stream_query_entities_metadata<'a, T: DeserializeOwned + 'a>(
        &'a self,
        table_name: &'a str,
        query: Option<&'a str>,
        fullmetadata: bool,
    ) -> impl Stream<Item = Result<Vec<T>, AzureError>> + 'a {
        futures::stream::unfold(ContinuationState::Start, move |cont_state| {
            async move {
                let cont = match cont_state {
                    ContinuationState::Start => None,
                    ContinuationState::Next(Some(cont)) => Some(cont),
                    ContinuationState::Next(None) => return None,
                };

                let mut path = table_name.to_owned();
                if let Some(clause) = query {
                    path.push_str("?");
                    path.push_str(clause);
                }

                let ec = self
                    .query_entity_collection(table_name, query, cont.as_ref(), fullmetadata)
                    .await;

                let ec = match ec {
                    Ok(ec) => ec,
                    Err(err) => return Some((Err(err), ContinuationState::Next(None))),
                };

                Some((Ok(ec.value), ContinuationState::Next(ec.continuation)))
            }
        })
    }

    pub fn stream_query_entities<'a, T: DeserializeOwned + 'a>(
        &'a self,
        table_name: &'a str,
        query: Option<&'a str>,
    ) -> impl Stream<Item = Result<Vec<T>, AzureError>> + 'a {
        self.stream_query_entities_metadata(table_name, query, false)
    }

    pub fn stream_query_entities_fullmetadata<'a, T: DeserializeOwned + 'a>(
        &'a self,
        table_name: &'a str,
        query: Option<&'a str>,
    ) -> impl Stream<Item = Result<Vec<T>, AzureError>> + 'a {
        self.stream_query_entities_metadata(table_name, query, true)
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
        self.request_with_default_header(table_name, &Method::POST, Some(&obj_ser), false)
    }

    pub async fn insert_entity<T: Serialize>(
        &self,
        table_name: &str,
        entity: &T,
    ) -> Result<(), AzureError> {
        let future_response = self._prepare_insert_entity(table_name, entity)?;

        check_status_extract_body(future_response, StatusCode::CREATED).await?;
        Ok(())
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
        self.request_with_default_header(path, &Method::PUT, Some(body), false)
    }

    pub async fn update_entity<T: Serialize>(
        &self,
        table_name: &str,
        partition_key: &str,
        row_key: &str,
        entity: &T,
    ) -> Result<(), AzureError> {
        let future_response =
            self._prepare_update_entity(table_name, partition_key, row_key, entity)?;
        check_status_extract_body(future_response, StatusCode::NO_CONTENT).await?;
        Ok(())
    }

    pub async fn delete_entity(
        &self,
        table_name: &str,
        partition_key: &str,
        row_key: &str,
    ) -> Result<(), AzureError> {
        let path = &entity_path(table_name, partition_key, row_key);

        let future_response = self.request(path, &Method::DELETE, None, |ref mut request| {
            request.header(
                header::ACCEPT,
                HeaderValue::from_static(get_json_mime_nometadata()),
            );
            request.header(header::IF_MATCH, header::HeaderValue::from_static("*"));
        })?;
        check_status_extract_body(future_response, StatusCode::NO_CONTENT).await?;
        Ok(())
    }

    pub async fn batch<T: Serialize>(
        &self,
        table_name: &str,
        partition_key: &str,
        batch_items: &[BatchItem<T>],
    ) -> Result<(), AzureError> {
        let payload = &generate_batch_payload(
            self.client.get_uri_prefix(ServiceType::Table).as_str(),
            table_name,
            partition_key,
            batch_items,
        );

        let future_response =
            self.request("$batch", &Method::POST, Some(payload), |ref mut request| {
                request.header(
                    header::CONTENT_TYPE,
                    header::HeaderValue::from_static(get_batch_mime()),
                );
            })?;
        check_status_extract_body(future_response, StatusCode::ACCEPTED).await?;
        // TODO deal with body response, handle batch failure.
        // let ref body = get_response_body(&mut response)?;
        // info!("{}", body);
        Ok(())
    }

    fn request_with_default_header(
        &self,
        segment: &str,
        method: &Method,
        request_str: Option<&str>,
        fullmetadata: bool,
    ) -> Result<ResponseFuture, AzureError> {
        self.request(segment, method, request_str, |ref mut request| {
            if fullmetadata {
                request.header(
                    header::ACCEPT,
                    HeaderValue::from_static(get_json_mime_fullmetadata()),
                );
            } else {
                request.header(
                    header::ACCEPT,
                    HeaderValue::from_static(get_json_mime_nometadata()),
                );
            }
            request.header(
                header::ACCEPT,
                HeaderValue::from_static(get_json_mime_nometadata()),
            );
            if request_str.is_some() {
                request.header(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static(get_default_json_mime()),
                );
            }
        })
    }

    fn request<F>(
        &self,
        segment: &str,
        method: &Method,
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

#[derive(Clone)]
pub struct TableStorage {
    service: TableService,
    table_name: String,
}

impl TableStorage {
    pub fn new<S:Into<String>>(service: TableService, table_name: S) -> Self {
        TableStorage { 
            service, 
            table_name: table_name.into()
        }
    }

    pub fn create_table(&self) -> impl Future<Item = (), Error = AzureError> {
        self.service.create_table(self.table_name.clone())
    }

    pub fn get_entity<T: DeserializeOwned>(
        &self,
        partition_key: &str,
        row_key: &str,
    ) -> impl Future<Item = Option<T>, Error = AzureError> {
        self.service.get_entity(&self.table_name, partition_key, row_key)
    }

    pub fn query_entities<T: DeserializeOwned>(
        &self,
        query: Option<&str>,
    ) -> impl Future<Item = Vec<T>, Error = AzureError> {
        self.service.query_entities(&self.table_name, query)
    }

    pub fn stream_query_entities<'a, T: DeserializeOwned + 'a>(
        &'a self,
        query: Option<&'a str>,
    ) ->  impl Stream<Item = T, Error = AzureError> + 'a {
        self.service.stream_query_entities(&self.table_name, query)
    }

    pub fn stream_query_entities_fullmetadata<'a, T: DeserializeOwned + 'a>(
        &'a self,
        query: Option<&'a str>,
    ) ->  impl Stream<Item = T, Error = AzureError> + 'a {
        self.service.stream_query_entities_fullmetadata(&self.table_name, query)
    }

    pub fn insert_entity<T: Serialize>(&self, entity: &T) -> impl Future<Item = (), Error = AzureError> {
        self.service.insert_entity::<T>(&self.table_name, entity)
    }

    pub fn update_entity<T: Serialize>(
        &self,
        partition_key: &str,
        row_key: &str,
        entity: &T,
    ) -> impl Future<Item = (), Error = AzureError> {
        self.service.update_entity(&self.table_name, partition_key, row_key, entity)
    }

    pub fn delete_entity(&self, partition_key: &str, row_key: &str) -> impl Future<Item = (), Error = AzureError> {
        self.service.delete_entity(&self.table_name, partition_key, row_key)
    }

    pub fn batch<T: Serialize>(
        &self,
        partition_key: &str,
        batch_items: &[BatchItem<T>],
    ) -> impl Future<Item = (), Error = AzureError> {
        self.service.batch(&self.table_name, partition_key, batch_items)
    }
}

const HEADER_NEXTPARTITIONKEY: &'static str = "x-ms-continuation-NextPartitionKey";
const HEADER_NEXTROWKEY: &'static str = "x-ms-continuation-NextRowKey";

fn continuation_from_headers(headers: &HeaderMap) -> Option<Continuation> {
    if headers.contains_key(HEADER_NEXTPARTITIONKEY) && headers.contains_key(HEADER_NEXTROWKEY) {
        Some(Continuation {
            next_partition_key: headers[HEADER_NEXTPARTITIONKEY]
                .to_str()
                .unwrap()
                .to_string(),
            next_row_key: headers[HEADER_NEXTROWKEY].to_str().unwrap().to_string(),
        })
    } else {
        None
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
    #[serde(skip)]
    continuation: Option<Continuation>,
}

#[derive(Debug, Clone)]
struct Continuation {
    next_partition_key: String,
    next_row_key: String,
}

#[derive(Debug, Clone)]
enum ContinuationState {
    Start,
    Next(Option<Continuation>),
}

#[inline]
fn entity_path(table_name: &str, partition_key: &str, row_key: &str) -> String {
    table_name.to_owned() + "(PartitionKey='" + partition_key + "',RowKey='" + row_key + "')"
}

#[inline]
pub fn get_batch_mime() -> &'static str {
    "multipart/mixed; boundary=batch_a1e9d677-b28b-435e-a89e-87e6a768a431"
}
