use super::entity_client::PipelineEntityClient;
use crate::{
    authorization::{authorization_policy::AuthorizationPolicy, AuthorizationToken},
    operations::{
        create_table::{CreateTableOptions, CreateTableResponse},
        delete_table::DeleteTableOptions,
        list_tables::{ListTablesOptions, ListTablesResponse},
    },
    table_context::TableContext,
};
use azure_core::{pipeline::Pipeline, ClientOptions, Context, Error, PipelineContext, Policy};
use http::{method::Method, Uri};
use std::borrow::Cow;
use std::str::FromStr;
use std::sync::Arc;

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

pub struct TableClient {
    cloud_location: CloudTableLocation,
    pipeline: Pipeline<TableContext>,
}

impl TableClient {
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
        let mut request = self.prepare_table_request("Tables", Method::GET);

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

    pub async fn create_table(
        &self,
        ctx: Context,
        table_name: impl AsRef<str>,
        options: CreateTableOptions,
    ) -> Result<CreateTableResponse, Error> {
        let mut request = self.prepare_table_request("Tables", Method::POST);

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
        let mut request = self.prepare_table_request(
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

    pub fn into_entity_client<S: Into<Cow<'static, str>>>(
        self,
        table_name: S,
    ) -> PipelineEntityClient {
        PipelineEntityClient::new(self, table_name)
    }

    // Create a new table bas request
    pub(crate) fn prepare_table_request(
        &self,
        uri_path: &str,
        http_method: http::Method,
    ) -> azure_core::Request {
        let uri = format!("{}/{}", self.cloud_location.url(), uri_path);
        let uri = Uri::from_str(uri.as_str()).unwrap();
        azure_core::Request::new(uri, http_method)
    }

    // Get a reference to the pipeline table client's pipeline.
    pub(crate) fn pipeline(&self) -> &Pipeline<TableContext> {
        &self.pipeline
    }
}

#[cfg(test)]
pub mod test_pipeline_table_client {
    use super::{TableClient, TableOptions};
    use crate::{
        operations::{
            create_table::CreateTableOptions, delete_table::DeleteTableOptions,
            list_tables::ListTablesOptions, OdataMetadataLevel,
        },
        Filter, Top,
    };
    use azure_core::Context;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestEntity {
        #[serde(rename = "PartitionKey")]
        pub city: String,
        pub name: String,
        #[serde(rename = "RowKey")]
        pub surname: String,
    }

    fn emulator_table_client() -> TableClient {
        TableClient::emulator(TableOptions::default())
    }

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
        let table_name = "TableForTest";
        assert_eq!(
            emulator_table_client()
                .list_tables(Context::new(), ListTablesOptions::default())
                .await
                .unwrap()
                .tables
                .iter()
                .filter(|&t| t.table_name == table_name)
                .next(),
            None
        );

        assert!(
            emulator_table_client()
                .create_table(Context::new(), table_name, CreateTableOptions::default())
                .await
                .unwrap()
                .table_name
                .as_str()
                == table_name
        );

        let tables = emulator_table_client()
            .list_tables(Context::new(), ListTablesOptions::default())
            .await
            .unwrap()
            .tables
            .iter()
            .filter(|&t| t.table_name == table_name);
        assert_eq!(tables.next(), Some(table_name));
        assert_eq!(tables.next(), None);
    }
}
