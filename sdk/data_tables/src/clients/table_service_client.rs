use crate::{
    authorization::{authorization_policy::AuthorizationPolicy, AuthorizationToken},
    operations::{
        create_table::CreateTableBuilder, delete_table::DeleteTableBuilder,
        query_tables::QueryTablesBuilder,
    },
};
use azure_core::{
    headers::{MS_DATE, VERSION},
    ClientOptions, Pipeline,
};
use chrono::Utc;
use http::{request::Builder, HeaderValue, Method};
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
    /// Create a new `TableClient`
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

    /// Create a new `TableClient` for Azure storage emulator
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

    /// The Query Tables operation returns a list of tables under the specified account.
    pub fn delete_table(&self, table_name: impl Into<String>) -> DeleteTableBuilder {
        DeleteTableBuilder::new(self, table_name.into())
    }

    /// The Create Table operation creates a new table in a storage account.
    pub fn create_table(&self, table_name: impl Into<String>) -> CreateTableBuilder {
        CreateTableBuilder::new(self, table_name.into())
    }

    /// The Query Tables operation returns a list of tables under the specified account.
    pub fn query_tables(&self) -> QueryTablesBuilder {
        QueryTablesBuilder::new(self)
    }

    pub(crate) fn pipeline_request(&self, method: Method, uri_path: &str) -> Builder {
        let timestamp = Utc::now().format("%a, %d %h %Y %T GMT").to_string();
        Builder::new()
            .method(method)
            .uri(format!("{}/{}", self.cloud_location.uri(), uri_path))
            .header(VERSION, HeaderValue::from_static("2019-12-12"))
            .header(MS_DATE, HeaderValue::from_str(&timestamp).unwrap())
            .header("content-type", HeaderValue::from_static("application/json"))
    }

    /// Get a reference to the table client's pipeline.
    pub fn pipeline(&self) -> &Pipeline {
        &self.pipeline
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
    fn uri(&self) -> String {
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
