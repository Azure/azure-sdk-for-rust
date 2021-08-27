use crate::operations::create_table::{CreateTableOptions, CreateTableResponse};
use crate::operations::delete_table::DeleteTableOptions;
use crate::operations::list_tables::{ListTablesOptions, ListTablesResponse};
use crate::{
    authorization::{authorization_policy::AuthorizationPolicy, AuthorizationToken},
    core::clients::StorageAccountClient,
};
use crate::{
    table::{clients::TableServiceClient, requests::*},
    table_context::TableContext,
};
use azure_core::{pipeline::Pipeline, ClientOptions, Context, Error, PipelineContext, Policy};
use bytes::Bytes;
use http::request::{Builder, Request};
use http::{method::Method, Uri};
use std::str::FromStr;
use std::sync::Arc;

pub trait AsTableClient<S: Into<String>> {
    fn as_table_client(&self, s: S) -> Arc<TableClient>;
}

impl<S: Into<String>> AsTableClient<S> for Arc<TableServiceClient> {
    fn as_table_client(&self, s: S) -> Arc<TableClient> {
        TableClient::new(self.clone(), s)
    }
}

#[derive(Debug, Clone)]
pub struct TableClient {
    table_service_client: Arc<TableServiceClient>,
    table_name: String,
}

impl TableClient {
    pub(crate) fn new<S: Into<String>>(
        table_service_client: Arc<TableServiceClient>,
        s: S,
    ) -> Arc<Self> {
        Arc::new(Self {
            table_service_client,
            table_name: s.into(),
        })
    }

    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    pub fn create(&self) -> CreateTableBuilder {
        CreateTableBuilder::new(self)
    }

    pub fn query(&self) -> QueryEntityBuilder {
        QueryEntityBuilder::new(self)
    }

    pub fn delete(&self) -> DeleteTableBuilder {
        DeleteTableBuilder::new(self)
    }

    pub fn insert(&self) -> InsertEntityBuilder {
        InsertEntityBuilder::new(self)
    }

    pub(crate) fn url(&self) -> &url::Url {
        self.table_service_client.url()
    }

    pub(crate) fn storage_account_client(&self) -> &StorageAccountClient {
        self.table_service_client.storage_account_client()
    }

    pub(crate) fn http_client(&self) -> &dyn azure_core::HttpClient {
        self.table_service_client.http_client()
    }

    pub(crate) fn prepare_request(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<Bytes>,
    ) -> Result<(Request<Bytes>, url::Url), crate::Error> {
        self.table_service_client
            .prepare_request(url, method, http_header_adder, request_body)
    }
}

#[cfg(test)]
#[cfg(feature = "test_integration")]
mod integration_tests {
    use super::*;
    use crate::{
        core::prelude::*,
        table::clients::{AsTableClient, AsTableServiceClient},
    };
    use futures::StreamExt;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestEntity {
        #[serde(rename = "PartitionKey")]
        pub city: String,
        pub name: String,
        #[serde(rename = "RowKey")]
        pub surname: String,
    }

    fn get_emulator_client() -> Arc<TableServiceClient> {
        let storage_account = StorageAccountClient::new_emulator_default().as_storage_client();
        storage_account
            .as_table_service_client()
            .expect("a table service client")
    }

    #[tokio::test]
    async fn test_create_delete() {
        let table_client = get_emulator_client();
        let table = table_client.as_table_client("TableClientCreateDelete");

        assert_eq!(
            table.table_name(),
            "TableClientCreateDelete",
            "the table name should match what was provided"
        );

        println!("Create the table");
        match table.create().execute().await {
            _ => {}
        }

        println!("Validate that the table was created");
        let mut stream = Box::pin(table_client.list().stream());
        while let Some(result) = stream.next().await {
            let result = result.expect("the request should succeed");

            let has_table = result
                .tables
                .iter()
                .any(|t| t.name == "TableClientCreateDelete");
            assert!(has_table, "the table should be present in the tables list");
        }

        println!("Delete the table");
        table
            .delete()
            .execute()
            .await
            .expect("we should be able to delete the table");

        println!("Validate that the table was deleted");
        let mut stream = Box::pin(table_client.list().stream());
        while let Some(result) = stream.next().await {
            let result = result.expect("the request should succeed");
            let has_table = result
                .tables
                .iter()
                .any(|t| t.name == "TableClientCreateDelete");
            assert!(
                !has_table,
                "the table should not be present in the tables list"
            );
        }
    }

    #[tokio::test]
    async fn test_insert() {
        let table_client = get_emulator_client();

        let table = table_client.as_table_client("TableClientInsert");
        assert_eq!(
            table.table_name(),
            "TableClientInsert",
            "the table name should match what was provided"
        );

        println!("Delete the table (if it exists)");
        match table.delete().execute().await {
            _ => {}
        }

        println!("Create the table");
        table
            .create()
            .execute()
            .await
            .expect("the table should be created");

        let entity = TestEntity {
            city: "Milan".to_owned(),
            name: "Francesco".to_owned(),
            surname: "Cogno".to_owned(),
        };

        println!("Insert an entity into the table");
        table
            .insert()
            .return_entity(true)
            .execute(&entity)
            .await
            .expect("the insert operation should succeed");

        // TODO: Validate that the entity was inserted
    }
}

///////////////////////////////////////////////////////////////////////
//////////////////////// pipeline table client ////////////////////////
//////////////////////////////////////////////////////////////////////

const PORT: u16 = 10002;
const ADDRESS: &'static str = "127.0.0.1";
const EMULATOR_ACCOUNT: &'static str = "devstoreaccount1";
const EMULATOR_KEY: &'static str =
    "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==";

/// The cloud with which you want to interact.
#[derive(Debug, Clone)]
enum CloudTableLocation {
    /// Azure public cloud
    Public(String),
    /// Azure China cloud
    China(String),
    // TODO: Other govt clouds?
    /// A custom base URL
    Custom { account: String, url: String },
}

impl CloudTableLocation {
    /// the base URL for a given cloud location
    fn url(&self) -> String {
        match self {
            CloudTableLocation::China(account) => {
                format!("https://{}.table.core.chinacloudapi.cn", account)
            }
            CloudTableLocation::Public(account) => {
                format!("https://{}.table.core.windows.net", account)
            }
            CloudTableLocation::Custom { url, account } => url.clone(),
        }
    }
}

/// Options for specifying how a Table client will behave
#[derive(Debug, Clone, Default)]
pub struct TableOptions {
    options: ClientOptions<TableContext>,
}

/// Create a Pipeline from TableOptions
fn pipeline_from_options(
    options: TableOptions,
    token: AuthorizationToken,
) -> Pipeline<TableContext> {
    let policy = AuthorizationPolicy::new(token);
    let policy: Arc<dyn Policy<TableContext>> = Arc::new(policy);
    let per_retry_policies: Vec<Arc<dyn Policy<TableContext>>> = vec![policy];
    Pipeline::new(
        option_env!("CARGO_PKG_NAME"),
        option_env!("CARGO_PKG_VERSION"),
        &options.options,
        per_retry_policies,
        Vec::new(),
    )
}

pub struct PipelineTableClient {
    cloud_location: CloudTableLocation,
    pipeline: Pipeline<TableContext>,
}

impl PipelineTableClient {
    /// Create a new `TableClient`
    pub fn new(account: String, auth_token: AuthorizationToken, options: TableOptions) -> Self {
        Self {
            cloud_location: CloudTableLocation::Public(account),
            pipeline: pipeline_from_options(options, auth_token),
        }
    }

    /// Create a new `TableClient` for Azure storage emulator
    pub fn emulator(options: TableOptions) -> Self {
        let auth_token = AuthorizationToken::SharedKeyToken {
            account: self::EMULATOR_ACCOUNT.to_string(),
            key: self::EMULATOR_KEY.to_string(),
        };

        Self {
            cloud_location: CloudTableLocation::Custom {
                account: self::EMULATOR_ACCOUNT.to_string(),
                url: format!(
                    "http://{}:{}/{}",
                    self::ADDRESS,
                    self::PORT,
                    self::EMULATOR_ACCOUNT
                ),
            },
            pipeline: pipeline_from_options(options, auth_token),
        }
    }

    pub async fn list_tables(
        &self,
        ctx: Context,
        options: ListTablesOptions<'_>,
    ) -> Result<ListTablesResponse, Error> {
        let mut request = self.prepare_pipeline_request("Tables", Method::GET);

        // add basic request properties
        options.decorate_request(&mut request)?;
        trace!("request after decoration: {:#?}", request);

        // start passing the request in the pipeline
        let mut pipeline_context = PipelineContext::new(ctx, TableContext::default());
        let response = self
            .pipeline
            .send(&mut pipeline_context, &mut request)
            .await?
            .validate(http::StatusCode::OK)
            .await?;

        //
        Ok(ListTablesResponse::try_from(response).await?)
    }

    pub async fn create_table<N: AsRef<str>>(
        &self,
        ctx: Context,
        table_name: N,
        options: CreateTableOptions,
    ) -> Result<CreateTableResponse, Error> {
        let mut request = self.prepare_pipeline_request("Tables", Method::POST);

        let mut pipeline_context = PipelineContext::new(ctx, TableContext::default());
        options.decorate_request(&mut request, table_name.as_ref())?;

        let response = self
            .pipeline
            .send(&mut pipeline_context, &mut request)
            .await?
            .validate(http::StatusCode::CREATED)
            .await?;

        Ok(CreateTableResponse::try_from(response).await?)
    }

    pub async fn delete_table<N: AsRef<str>>(
        &self,
        ctx: Context,
        table_name: N,
        options: DeleteTableOptions,
    ) -> Result<(), Error> {
        let mut request = self.prepare_pipeline_request(
            format!("Tables('{}')", table_name.as_ref()).as_str(),
            Method::DELETE,
        );

        options.decorate_request(&mut request)?;
        let table_context = TableContext::default();
        let mut pipeline_context = PipelineContext::new(ctx, table_context);

        let _ = self
            .pipeline
            .send(&mut pipeline_context, &mut request)
            .await?
            .validate(http::StatusCode::NO_CONTENT)
            .await?;

        Ok(())
    }

    fn prepare_pipeline_request(
        &self,
        uri_path: &str,
        http_method: http::Method,
    ) -> azure_core::Request {
        let uri = format!("{}/{}", self.cloud_location.url(), uri_path);
        let uri = Uri::from_str(uri.as_str()).unwrap();
        azure_core::Request::new(uri, http_method)
    }
}

#[cfg(test)]
pub mod test_pipeline_table_client {
    use super::{PipelineTableClient, TableOptions};
    use crate::{
        operations::{
            create_table::CreateTableOptions, delete_table::DeleteTableOptions,
            list_tables::ListTablesOptions, OdataMetadataLevel,
        },
        Filter, Top,
    };
    use azure_core::Context;

    #[tokio::test]
    async fn test_list_tables() {
        let response = emulator_table_client()
            .list_tables(
                Context::new(),
                ListTablesOptions::default()
                    .odata_metadata_level(OdataMetadataLevel::FullMetadata)
                    .filter(Filter::new("TableName gt 'emails'"))
                    .top(Top::new(2)),
            )
            .await
            .and_then(|ok_response| {
                ok_response
                    .tables
                    .iter()
                    .for_each(|table| println!("{:?}", table.odata_link));
                Ok(())
            });
        println!("{:?}", response);
    }

    #[tokio::test]
    async fn test_delete_table() {
        let response = emulator_table_client()
            .delete_table(
                Context::new(),
                "TableForTest",
                DeleteTableOptions::default(),
            )
            .await;
        println!("{:#?}", response);
    }

    #[tokio::test]
    async fn test_create_table() {
        let response = emulator_table_client()
            .create_table(
                Context::new(),
                "TableForTest",
                CreateTableOptions::default(),
            )
            .await;
        println!("{:#?}", response);
    }

    fn emulator_table_client() -> PipelineTableClient {
        PipelineTableClient::emulator(TableOptions::default())
    }
}
