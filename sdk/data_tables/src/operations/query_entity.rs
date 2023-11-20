use crate::prelude::*;
use azure_core::{
    error::{Error, ErrorKind},
    headers::*,
    prelude::*,
    AppendToUrlQuery, CollectedResponse, Method, Pageable,
};
use azure_storage::headers::CommonStorageResponseHeaders;
use serde::de::DeserializeOwned;
use std::convert::{TryFrom, TryInto};

operation! {
    #[stream]
    QueryEntity,
    client: TableClient,
    ?filter: Filter,
    ?select: Select,
    ?top: Top,
    ?initial_partition_key: String,
    ?initial_row_key: String
}

impl QueryEntityBuilder {
    pub fn into_stream<E>(self) -> Pageable<QueryEntityResponse<E>, Error>
    where
        E: DeserializeOwned + Send + Sync,
    {
        let make_request = move |continuation: Option<(String, Option<String>)>| {
            let this = self.clone();
            let mut ctx = self.context.clone();
            async move {
                let mut url = this.client.url()?;
                url.path_segments_mut()
                    .map_err(|()| Error::message(ErrorKind::Other, "invalid table URL"))?
                    .pop()
                    .push(&format!("{}()", this.client.table_name()));

                this.filter.append_to_url_query(&mut url);
                this.select.append_to_url_query(&mut url);
                this.top.append_to_url_query(&mut url);

                if let Some((partition_key, row_key)) = continuation {
                    url.query_pairs_mut()
                        .append_pair("NextPartitionKey", &partition_key);

                    if let Some(row_key) = row_key {
                        url.query_pairs_mut().append_pair("NextRowKey", &row_key);
                    }
                } else if let Some(initial_paritition_key) = this.initial_partition_key {
                    url.query_pairs_mut()
                        .append_pair("NextPartitionKey", &initial_paritition_key);

                    if let Some(row_key) = this.initial_row_key {
                        url.query_pairs_mut().append_pair("NextRowKey", &row_key);
                    }
                }

                let mut headers = Headers::new();
                headers.insert(ACCEPT, "application/json;odata=fullmetadata");

                let mut request = TableClient::finalize_request(url, Method::Get, headers, None)?;

                let response = this.client.send(&mut ctx, &mut request).await?;

                let collected_response = CollectedResponse::from_response(response).await?;
                collected_response.try_into()
            }
        };
        Pageable::new(make_request)
    }
}

#[derive(Debug, Clone)]
pub struct QueryEntityResponse<E>
where
    E: DeserializeOwned + Send + Sync,
{
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub metadata: String,
    pub entities: Vec<E>,
    next_partition_key: Option<String>,
    next_row_key: Option<String>,
}

impl<E> Continuable for QueryEntityResponse<E>
where
    E: DeserializeOwned + Send + Sync,
{
    type Continuation = (String, Option<String>);

    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_partition_key
            .clone()
            .map(|partition_key| (partition_key, self.next_row_key.clone()))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct QueryEntityResponseInternal<E> {
    #[serde(rename = "odata.metadata")]
    pub metadata: String,
    #[serde(default = "Vec::new")]
    pub value: Vec<E>,
}

impl<E: DeserializeOwned + Send + Sync> TryFrom<CollectedResponse> for QueryEntityResponse<E> {
    type Error = Error;

    fn try_from(response: CollectedResponse) -> azure_core::Result<Self> {
        let query_entity_response_internal: QueryEntityResponseInternal<E> =
            serde_json::from_slice(response.body())?;

        let headers = response.headers();

        let next_partition_key = headers.get_optional_string(&HeaderName::from_static(
            "x-ms-continuation-nextpartitionkey",
        ));

        let next_row_key =
            headers.get_optional_string(&HeaderName::from_static("x-ms-continuation-nextrowkey"));

        Ok(QueryEntityResponse {
            common_storage_response_headers: response.headers().try_into()?,
            metadata: query_entity_response_internal.metadata,
            entities: query_entity_response_internal.value,
            next_partition_key,
            next_row_key,
        })
    }
}
