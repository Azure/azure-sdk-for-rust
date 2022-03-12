use crate::client::KustoClient;
use crate::request_options::*;
use azure_core::prelude::*;
use azure_core::setters;
use azure_core::{
    collect_pinned_stream,
    headers::{add_mandatory_header2, add_optional_header2},
    Response as HttpResponse,
};
use futures::future::BoxFuture;

/// A future of a delete file response
type ExecuteQuery = BoxFuture<'static, crate::error::Result<KustoResponseDataSetV2>>;

#[derive(Debug, Serialize, Deserialize)]
struct QueryBody {
    db: String,
    csl: String,
}

#[derive(Debug, Clone)]
pub struct ExecuteQueryBuilder {
    client: KustoClient,
    database: String,
    query: String,
    client_request_id: Option<ClientRequestId>,
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
            query,
            client_request_id: None,
            context,
        }
    }

    setters! {
        client_request_id: ClientRequestId => Some(client_request_id),
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

            add_mandatory_header2(
                &ContentType::new("application/json; charset=utf-8"),
                &mut request,
            )
            .unwrap();
            add_mandatory_header2(&Accept::new("application/json"), &mut request).unwrap();
            add_mandatory_header2(&AcceptEncoding::new("gzip,deflate"), &mut request).unwrap();
            add_optional_header2(&this.client_request_id, &mut request).unwrap();

            let body = QueryBody {
                db: this.database.clone(),
                csl: this.query.clone(),
            };
            request.set_body(bytes::Bytes::from(serde_json::to_string(&body)?).into());

            let response = self
                .client
                .pipeline()
                .send(&mut ctx.clone(), &mut request)
                .await?;

            KustoResponseDataSetV2::try_from(response).await
        })
    }
}

#[derive(Debug, Clone)]
pub struct KustoResponseDataSetV2 {
    pub tables: Vec<ResultTable>,
}

impl KustoResponseDataSetV2 {
    pub async fn try_from(response: HttpResponse) -> Result<Self, crate::error::Error> {
        let (_status_code, _header_map, pinned_stream) = response.deconstruct();
        let data = collect_pinned_stream(pinned_stream).await.unwrap();
        let tables: Vec<ResultTable> = serde_json::from_slice(&data.to_vec()).unwrap();
        Ok(Self { tables })
    }

    pub fn table_count(&self) -> usize {
        self.tables.len()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "PascalCase", tag = "FrameType")]
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
