use crate::prelude::*;
use azure_core::{
    error::Error, headers::*, prelude::*, AppendToUrlQuery, CollectedResponse, Method, Pageable,
};
use azure_storage::headers::CommonStorageResponseHeaders;
use std::convert::{TryFrom, TryInto};

operation! {
    #[stream]
    ListTables,
    client: TableServiceClient,
    ?filter: Filter,
    ?select: Select,
    ?top: Top
}

impl ListTablesBuilder {
    pub fn into_stream(self) -> Pageable<ListTablesResponse, Error> {
        let make_request = move |continuation: Option<String>| {
            let this = self.clone();
            let mut ctx = self.context.clone();
            async move {
                let mut url = this.client.url()?;

                this.filter.append_to_url_query(&mut url);
                this.select.append_to_url_query(&mut url);
                this.top.append_to_url_query(&mut url);

                if let Some(continuation) = continuation {
                    url.query_pairs_mut()
                        .append_pair("NextTableName", &continuation);
                }

                let mut headers = Headers::new();
                headers.insert(ACCEPT, "application/json;odata=fullmetadata");

                let mut request =
                    TableServiceClient::finalize_request(url, Method::Get, headers, None)?;

                let response = this.client.send(&mut ctx, &mut request).await?;

                let collected_response = CollectedResponse::from_response(response).await?;
                collected_response.try_into()
            }
        };
        Pageable::new(make_request)
    }
}

#[derive(Debug, Clone)]
pub struct ListTablesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub metadata: String,
    pub tables: Vec<Table>,
    pub continuation_next_table_name: Option<String>,
}

impl Continuable for ListTablesResponse {
    type Continuation = String;

    fn continuation(&self) -> Option<Self::Continuation> {
        self.continuation_next_table_name.clone()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ListTablesResponseInternal {
    #[serde(rename = "odata.metadata")]
    pub metadata: String,
    #[serde(default = "Vec::new")]
    pub value: Vec<Table>,
}

impl TryFrom<CollectedResponse> for ListTablesResponse {
    type Error = Error;

    fn try_from(response: CollectedResponse) -> azure_core::Result<Self> {
        let list_tables_response_internal: ListTablesResponseInternal =
            serde_json::from_slice(response.body())?;

        let continuation_next_table_name = response
            .headers()
            .get_optional_string(&HeaderName::from_static("x-ms-continuation-nexttablename"));

        Ok(ListTablesResponse {
            common_storage_response_headers: response.headers().try_into()?,
            metadata: list_tables_response_internal.metadata,
            tables: list_tables_response_internal.value,
            continuation_next_table_name,
        })
    }
}
