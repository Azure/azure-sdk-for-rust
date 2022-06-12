use super::{TableItem, MINIMAL_METADATA};
use crate::clients::table_service_client::TableServiceClient;
use azure_core::error::Result;
use azure_core::headers;
use azure_core::{
    collect_pinned_stream, headers::HeaderName, prelude::Continuation, setters, Context,
    Continuable, Pageable, Response,
};
use http::HeaderMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct QueryTablesBuilder {
    context: Context,
    filter: Option<String>,
    max_per_page: Option<i32>,
    client: TableServiceClient,
}

pub type QueryTables = Pageable<QueryTablesResponse, azure_core::error::Error>;

impl QueryTablesBuilder {
    pub(crate) fn new(client: TableServiceClient) -> Self {
        Self {
            client,
            filter: None,
            max_per_page: None,
            context: Context::new(),
        }
    }

    setters! {
        context: Context => context,
        filter: String => Some(filter),
        max_per_page: i32 => Some(max_per_page),
    }

    pub fn into_stream(self) -> QueryTables {
        let make_request = move |continuation: Option<Continuation>| {
            let this = self.clone();
            let context = self.context.clone();

            let mut path_and_query = match (this.filter.as_ref(), this.max_per_page) {
                (None, None) => format!("Tables"),
                (None, Some(max_per_page)) => format!("Tables?$top={}", max_per_page),
                (Some(filter), None) => format!("Tables?$filter={}", filter),
                (Some(filter), Some(max_per_page)) => {
                    format!("Tables?$filter={}&$top={}", filter, max_per_page)
                }
            };

            async move {
                if let Some(continuation) = continuation {
                    path_and_query = match (this.filter.as_ref(), this.max_per_page) {
                        (None, None) => format!(
                            "{}?NextTableName={}",
                            path_and_query,
                            continuation.into_raw()
                        ),
                        _ => format!(
                            "{}&NextTableName={}",
                            path_and_query,
                            continuation.into_raw()
                        ),
                    };
                }

                let mut request = this
                    .client
                    .prepare_request_pipeline(&path_and_query, http::Method::GET);

                let headers = request.headers_mut();
                headers.insert(HeaderName::from("accept"), MINIMAL_METADATA);

                let response = this.client.send(request, context).await?;
                QueryTablesResponse::try_from(response).await
            }
        };
        Pageable::new(make_request)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryTablesResponse {
    pub next_table_name: Option<String>,
    pub tables: Vec<TableItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct QueryTablesResponseBody {
    #[serde(rename = "value")]
    pub tables: Vec<TableItem>,
}

impl QueryTablesResponse {
    pub async fn try_from(response: Response) -> Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body: bytes::Bytes = collect_pinned_stream(pinned_stream).await?;
        let body: QueryTablesResponseBody = serde_json::from_slice(&body)?;
        Ok(Self {
            tables: body.tables,
            next_table_name: Self::next_table_name_from_headers(&headers)?,
        })
    }

    fn next_table_name_from_headers(headers: &HeaderMap) -> Result<Option<String>> {
        headers::get_option_from_headers(headers, "x-ms-continuation-NextTableName")
    }
}

impl Continuable for QueryTablesResponse {
    fn continuation(&self) -> Option<String> {
        self.next_table_name.clone()
    }
}

impl IntoIterator for QueryTablesResponse {
    type Item = TableItem;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.tables.into_iter()
    }
}
