use crate::{
    authorization::{authorization_policy::AuthorizationPolicy, AuthorizationToken},
    operations::{
        create_table::CreateTableBuilder, delete_table::DeleteTableBuilder,
        query_tables::QueryTablesBuilder,
    },
};
use azure_core::{
    error::Result,
    headers::{MS_DATE, VERSION},
    ClientOptions, Context, Pipeline, Request, Response,
};
use chrono::Utc;
use std::sync::Arc;

const PORT: u16 = 10002;
const ADDRESS: &str = "127.0.0.1";
const EMULATOR_ACCOUNT: &str = "devstoreaccount1";
const EMULATOR_KEY: &str =
    "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==";

#[derive(Debug, Clone)]
pub struct TableServiceClient {
    cloud_location: CloudLocation,
    pipeline: Pipeline,
}

/// Create a Pipeline from TableOptions
fn pipeline_from_options(options: ClientOptions, token: AuthorizationToken) -> Pipeline {
    let policy = Arc::new(AuthorizationPolicy::new(token));
    Pipeline::new(
        option_env!("CARGO_PKG_NAME"),
        option_env!("CARGO_PKG_VERSION"),
        options,
        Vec::new(),
        vec![policy],
    )
}

impl TableServiceClient {
    pub fn new(account: impl Into<String>, auth_token: AuthorizationToken) -> Self {
        Self {
            cloud_location: CloudLocation::Public(account.into()),
            pipeline: pipeline_from_options(ClientOptions::default(), auth_token),
        }
    }

    pub fn new_china(account: impl Into<String>, auth_token: AuthorizationToken) -> Self {
        Self {
            cloud_location: CloudLocation::China(account.into()),
            pipeline: pipeline_from_options(ClientOptions::default(), auth_token),
        }
    }

    pub fn emulator() -> Self {
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
        )
    }

    pub fn new_custom(
        account: impl Into<String>,
        auth_token: AuthorizationToken,
        uri: impl Into<String>,
    ) -> Self {
        Self {
            pipeline: pipeline_from_options(ClientOptions::default(), auth_token),
            cloud_location: CloudLocation::Custom {
                account: account.into(),
                uri: uri.into(),
            },
        }
    }

    /// The name of the table account with which this client instance will interact.
    pub fn account_name(&self) -> String {
        self.cloud_location.account()
    }

    /// Gets a list of tables from the storage account.
    pub fn query_tables(&self) -> QueryTablesBuilder {
        QueryTablesBuilder::new(self.clone())
    }

    /// Creates a table on the service.
    pub fn create_table(&self, table_name: impl AsRef<str>) -> CreateTableBuilder {
        CreateTableBuilder::new(self.clone(), table_name.as_ref().to_owned())
    }

    /// Deletes a table on the service.
    pub fn delete_table(&self, table_name: impl AsRef<str>) -> DeleteTableBuilder {
        DeleteTableBuilder::new(self.clone(), table_name.as_ref().to_owned())
    }

    pub(crate) fn _pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    pub(crate) fn prepare_request_pipeline(
        &self,
        url_path: &str,
        http_method: http::Method,
    ) -> Request {
        let uri = format!("{}/{}", self.cloud_location.url(), url_path);
        let mut request = Request::new(uri.parse().unwrap(), http_method);

        let headers = request.headers_mut();

        let timestamp = Utc::now().format("%a, %d %h %Y %T GMT").to_string();
        headers.insert(MS_DATE, timestamp);
        headers.insert(VERSION, "2019-02-02");
        headers.insert(http::header::CONTENT_TYPE, "application/json");
        request
    }

    pub(crate) async fn send(
        &self,
        mut request: Request,
        mut context: Context,
    ) -> Result<Response> {
        self.pipeline.send(&mut context, &mut request).await
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

    fn account(&self) -> String {
        match self {
            CloudLocation::Public(account) => account,
            CloudLocation::China(account) => account,
            CloudLocation::Custom { account, .. } => account,
        }
        .to_owned()
    }
}
