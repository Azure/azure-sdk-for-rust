use crate::authorization::authorization_policy::AuthorizationPolicy;
use crate::table::prelude::*;
use azure_core::{pipeline::Pipeline, ClientOptions, Context, Error, PipelineContext, Policy};
use http::{method::Method, request::Builder as RequestBuilder, Uri};
use std::{str::FromStr, sync::Arc};

const PORT: u16 = 10002;
const ADDRESS: &str = "127.0.0.1";
const EMULATOR_ACCOUNT: &str = "devstoreaccount1";
const EMULATOR_KEY: &str =
    "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==";

/// Options for specifying how a Table client will behave
#[derive(Debug, Clone, Default)]
pub struct TableOptions {
    options: ClientOptions<TableContext>,
}

impl TableOptions {
    #[cfg(feature = "mock_transport_framework")]
    /// Create new options with a given transaction name
    pub fn new_with_transaction_name(name: String) -> Self {
        Self {
            options: ClientOptions::new_with_transaction_name(name.into()),
        }
    }
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
        options.options,
        per_retry_policies,
        Vec::new(),
    )
}

pub struct TableClient {
    cloud_location: CloudLocation,
    pipeline: Pipeline<TableContext>,
}

impl TableClient {
    /// Create a new `TableClient`
    pub fn new(
        account: impl Into<String>,
        auth_token: AuthorizationToken,
        options: TableOptions,
    ) -> Self {
        Self {
            cloud_location: CloudLocation::Public(account.into()),
            pipeline: pipeline_from_options(options, auth_token),
        }
    }

    pub fn new_china(
        account: impl Into<String>,
        auth_token: AuthorizationToken,
        options: TableOptions,
    ) -> Self {
        Self {
            cloud_location: CloudLocation::China(account.into()),
            pipeline: pipeline_from_options(options, auth_token),
        }
    }

    pub fn new_custom(
        account: impl Into<String>,
        auth_token: AuthorizationToken,
        uri: impl Into<String>,
        options: TableOptions,
    ) -> Self {
        Self {
            pipeline: pipeline_from_options(options, auth_token),
            cloud_location: CloudLocation::Custom {
                account: account.into(),
                uri: uri.into(),
            },
        }
    }

    /// Create a new `TableClient` for Azure storage emulator
    pub fn emulator(options: TableOptions) -> Self {
        Self::new_custom(
            self::EMULATOR_ACCOUNT,
            AuthorizationToken::SharedKeyToken {
                account: self::EMULATOR_ACCOUNT.to_string(),
                key: self::EMULATOR_KEY.to_string(),
            },
            format!(
                "http://{}:{}/{}",
                self::ADDRESS,
                self::PORT,
                self::EMULATOR_ACCOUNT
            ),
            options,
        )
    }

    /// Create new options with a given transaction name
    #[cfg(feature = "mock_transport_framework")]
    pub fn new_with_transaction(
        account: impl Into<String>,
        auth_token: AuthorizationToken,
        transaction_name: impl Into<String>,
    ) -> Self {
        Self::new(
            account.into(),
            auth_token,
            TableOptions::new_with_transaction_name(transaction_name.into()),
        )
    }

    /// The Query Tables operation returns a list of tables under the specified account.
    pub async fn query_tables(
        &self,
        ctx: Context,
        options: QueryTablesOptions<'_>,
    ) -> Result<QueryTablesResponse, Error> {
        let uri_path = options.base_uri_path();
        trace!("uri path created successfully: {:#?}", uri_path);

        let mut request = self.prepare_table_request(uri_path, Method::GET);

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

        Ok(QueryTablesResponse::try_from(response).await?)
    }

    /// The Create Table operation creates a new table in a storage account.
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
            .validate(options.expected_status_code())
            .await?;

        Ok(CreateTableResponse::try_from(response).await?)
    }

    /// The Delete Table operation deletes the specified table and any data it contains.
    /// When a table is successfully deleted, it is immediately marked for deletion and is no longer accessible to clients. The table is later removed from the Table service during garbage collection.
    /// Note that deleting a table is likely to take at least 40 seconds to complete. If an operation is attempted against the table while it was being deleted, the service returns status code 409 (Conflict).
    pub async fn delete_table<N: AsRef<str>>(
        &self,
        ctx: Context,
        table_name: N,
        options: DeleteTableOptions,
    ) -> Result<DeleteTableResponse, Error> {
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

        Ok(DeleteTableResponse {})
    }

    pub(crate) fn prepare_table_request(
        &self,
        uri_path: &str,
        http_method: http::Method,
    ) -> azure_core::Request {
        let url = format!("{}/{}", self.cloud_location.url(), uri_path);
        let url = url::Url::from_str(&url).unwrap();
        let uri = Uri::from_str(url.as_str()).unwrap();
        RequestBuilder::new()
            .method(http_method)
            .uri(uri)
            .body(bytes::Bytes::new())
            .unwrap()
            .into()
    }

    pub(crate) fn pipeline(&self) -> &Pipeline<TableContext> {
        &self.pipeline
    }

    /// Crates Entity client for a given table. consuming Self in the process.
    pub fn into_entity_client(self, table_name: impl Into<String>) -> EntityClient {
        EntityClient::new(self, table_name)
    }
}

/// The cloud with which you want to interact.
#[derive(Debug, Clone)]
pub enum CloudLocation {
    /// Azure public cloud
    Public(String),
    /// Azure China cloud
    China(String),
    /// A custom base URL
    Custom { account: String, uri: String },
}

impl CloudLocation {
    /// the base URL for a given cloud location
    fn url(&self) -> String {
        match self {
            CloudLocation::China(account) => {
                format!("https://{}.table.core.chinacloudapi.cn", account)
            }
            CloudLocation::Public(account) => {
                format!("https://{}.table.core.windows.net", account)
            }
            CloudLocation::Custom { uri, .. } => uri.clone(),
        }
    }
}
