use crate::client::KustoClient;
use async_convert::{async_trait, TryFrom};
use azure_core::prelude::*;
use azure_core::setters;
use azure_core::{collect_pinned_stream, Response as HttpResponse};
use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};

type ExecuteQuery = BoxFuture<'static, crate::error::Result<KustoResponseDataSetV2>>;

#[derive(Debug, Serialize, Deserialize)]
struct QueryBody {
    /// Name of the database in scope that is the target of the query or control command
    db: String,
    /// Text of the query or control command to execute
    csl: String,
}

#[derive(Debug, Clone)]
pub struct ExecuteQueryBuilder {
    client: KustoClient,
    database: String,
    query: String,
    client_request_id: Option<ClientRequestId>,
    app: Option<App>,
    user: Option<User>,
    context: Context,
}

impl ExecuteQueryBuilder {
    pub(crate) fn new(
        client: KustoClient,
        database: String,
        query: String,
        context: Context,
    ) -> Self {
        Self {
            client,
            database,
            query: query.trim().into(),
            client_request_id: None,
            app: None,
            user: None,
            context,
        }
    }

    setters! {
        client_request_id: ClientRequestId => Some(client_request_id),
        app: App => Some(app),
        user: User => Some(user),
        query: String => query,
        database: String => database,
        context: Context => context,
    }

    pub fn into_future(self) -> ExecuteQuery {
        let this = self.clone();
        let ctx = self.context.clone();

        Box::pin(async move {
            let url = this.client.query_url();
            let mut request = this.client.prepare_request(url, http::Method::POST);

            if let Some(request_id) = &this.client_request_id {
                request.insert_headers(request_id);
            };
            if let Some(app) = &this.app {
                request.insert_headers(app);
            };
            if let Some(user) = &this.user {
                request.insert_headers(user);
            };

            let body = QueryBody {
                db: this.database,
                csl: this.query,
            };
            let bytes = bytes::Bytes::from(serde_json::to_string(&body)?);
            request.insert_headers(&ContentLength::new(bytes.len() as i32));
            request.set_body(bytes.into());

            let response = self
                .client
                .pipeline()
                .send(&mut ctx.clone(), &mut request)
                .await?;

            <KustoResponseDataSetV2 as TryFrom<HttpResponse>>::try_from(response).await
        })
    }
}

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for ExecuteQueryBuilder {
    type IntoFuture = ExecuteQuery;
    type Output = <ExecuteQuery as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone)]
pub struct KustoResponseDataSetV2 {
    pub tables: Vec<ResultTable>,
}

#[async_trait]
impl TryFrom<HttpResponse> for KustoResponseDataSetV2 {
    type Error = crate::error::Error;

    async fn try_from(response: HttpResponse) -> Result<Self, crate::error::Error> {
        let (_status_code, _header_map, pinned_stream) = response.deconstruct();
        let data = collect_pinned_stream(pinned_stream).await.unwrap();
        let tables: Vec<ResultTable> = serde_json::from_slice(&data.to_vec()).unwrap();
        Ok(Self { tables })
    }
}

impl KustoResponseDataSetV2 {
    pub fn table_count(&self) -> usize {
        self.tables.len()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase", tag = "FrameType")]
#[allow(clippy::enum_variant_names)]
pub enum ResultTable {
    DataSetHeader(DataSetHeader),
    DataTable(DataTable),
    DataSetCompletion(DataSetCompletion),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DataSetHeader {
    pub is_progressive: bool,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DataTable {
    pub table_id: i32,
    pub table_name: String,
    pub table_kind: TableKind,
    pub columns: Vec<Column>,
    pub rows: Vec<Vec<serde_json::Value>>,
}

/// Categorizes data tables according to the role they play in the data set that a Kusto query returns.
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum TableKind {
    PrimaryResult,
    QueryCompletionInformation,
    QueryTraceLog,
    QueryPerfLog,
    TableOfContents,
    QueryProperties,
    QueryPlan,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Column {
    pub column_name: String,
    pub column_type: ColumnType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ColumnType {
    Bool,
    Boolean,
    Datetime,
    Date,
    Dynamic,
    Guid,
    Int,
    Long,
    Real,
    String,
    Timespan,
    Time,
    Decimal,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DataSetCompletion {
    pub has_errors: bool,
    pub cancelled: bool,
}
