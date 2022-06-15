use super::DatabaseClient;
use crate::operations::*;
use crate::resources::permission::AuthorizationToken;
use crate::resources::ResourceType;
use crate::ReadonlyString;

use azure_core::{ClientOptions, Context, Pipeline, Request, Response};

use std::fmt::Debug;
use std::sync::Arc;

/// The well-known account key used by Azure Cosmos DB Emulator.
/// https://docs.microsoft.com/azure/cosmos-db/local-emulator?tabs=ssl-netstd21#connect-with-emulator-apis
pub const EMULATOR_ACCOUNT_KEY: &str =
    "C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==";

/// A plain Cosmos client.
#[derive(Debug, Clone)]
pub struct CosmosClient {
    pipeline: Pipeline,
    cloud_location: CloudLocation,
}

impl CosmosClient {
    /// Create a new `CosmosClient` which connects to the account's instance in the public Azure cloud.
    pub fn new(account: String, auth_token: AuthorizationToken, options: CosmosOptions) -> Self {
        let cloud_location = CloudLocation::Public(account);
        let pipeline = new_pipeline_from_options(options, auth_token);
        Self {
            pipeline,
            cloud_location,
        }
    }

    #[cfg(feature = "mock_transport_framework")]
    /// Create new options with a given transaction name
    pub fn new_with_transaction(
        account: impl Into<String>,
        auth_token: AuthorizationToken,
        transaction_name: impl Into<String>,
    ) -> Self {
        Self::new(
            account.into(),
            auth_token,
            CosmosOptions::new_with_transaction_name(transaction_name.into()),
        )
    }

    /// Create a new `CosmosClient` which connects to the account's instance in the Chinese Azure cloud.
    pub fn new_china(
        account: String,
        auth_token: AuthorizationToken,
        options: CosmosOptions,
    ) -> Self {
        let cloud_location = CloudLocation::China(account);
        let pipeline = new_pipeline_from_options(options, auth_token);
        Self {
            pipeline,
            cloud_location,
        }
    }

    /// Create a new `CosmosClient` which connects to the account's instance in custom Azure cloud.
    pub fn new_custom(
        account: String,
        auth_token: AuthorizationToken,
        uri: String,
        options: CosmosOptions,
    ) -> Self {
        let cloud_location = CloudLocation::Custom { account, uri };
        let pipeline = new_pipeline_from_options(options, auth_token);
        Self {
            pipeline,
            cloud_location,
        }
    }

    /// Create a new `CosmosClient` which connects to the account's instance in Azure emulator
    pub fn new_emulator(address: &str, port: u16, options: CosmosOptions) -> Self {
        let auth_token = AuthorizationToken::primary_from_base64(EMULATOR_ACCOUNT_KEY).unwrap();
        let uri = format!("https://{}:{}", address, port);
        let cloud_location = CloudLocation::Custom {
            account: String::from("Custom"),
            uri,
        };
        let pipeline = new_pipeline_from_options(options, auth_token);
        Self {
            pipeline,
            cloud_location,
        }
    }

    /// Set the auth token used
    pub fn auth_token(&mut self, auth_token: AuthorizationToken) {
        // we replace the AuthorizationPolicy. This is
        // the last-1 policy by construction.
        let auth_policy: Arc<dyn azure_core::Policy> =
            Arc::new(crate::AuthorizationPolicy::new(auth_token));

        self.pipeline
            .replace_policy(auth_policy, self.pipeline.policies().len() - 2);
    }

    /// Create a database
    pub fn create_database<S: AsRef<str>>(&self, database_name: S) -> CreateDatabaseBuilder {
        CreateDatabaseBuilder::new(self.clone(), database_name.as_ref().to_owned())
    }

    /// List all databases
    pub fn list_databases(&self) -> ListDatabasesBuilder {
        ListDatabasesBuilder::new(self.clone())
    }

    /// Create a [`DatabaseClient`].
    pub fn database_client<S: Into<ReadonlyString>>(&self, database_name: S) -> DatabaseClient {
        DatabaseClient::new(self.clone(), database_name)
    }

    /// Prepares' an `azure_core::Request`.
    ///
    /// This function will add the cloud location to the URI suffix and generate
    /// a Request with the specified HTTP Method. It will also set the body
    /// to an empty `Bytes` instance.
    pub(crate) fn prepare_request_pipeline(
        &self,
        uri_path: &str,
        http_method: http::Method,
    ) -> Request {
        let uri = format!("{}/{}", self.cloud_location.url(), uri_path);
        Request::new(uri.parse().unwrap(), http_method)
    }

    pub(crate) async fn send(
        &self,
        mut request: Request,
        mut context: Context,
        resource_type: ResourceType,
    ) -> azure_core::error::Result<Response> {
        self.pipeline
            .send(context.insert(resource_type), &mut request)
            .await
    }

    pub(crate) fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }
}

/// Create a Pipeline from CosmosOptions
fn new_pipeline_from_options(
    options: CosmosOptions,
    authorization_token: AuthorizationToken,
) -> Pipeline {
    let auth_policy: Arc<dyn azure_core::Policy> =
        Arc::new(crate::AuthorizationPolicy::new(authorization_token));

    // The `AuthorizationPolicy` must be the **last** retry policy.
    // Policies can change the url and/or the headers, and the `AuthorizationPolicy`
    // must be able to inspect them or the resulting token will be invalid.
    let per_retry_policies = vec![auth_policy];

    Pipeline::new(
        option_env!("CARGO_PKG_NAME"),
        option_env!("CARGO_PKG_VERSION"),
        options.options,
        Vec::new(),
        per_retry_policies,
    )
}

/// Options for specifying how a Cosmos client will behave
#[derive(Debug, Clone, Default)]
pub struct CosmosOptions {
    options: ClientOptions,
}

impl CosmosOptions {
    /// Create new options
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(feature = "mock_transport_framework")]
    /// Create new options with a given transaction name
    pub fn new_with_transaction_name(name: String) -> Self {
        Self {
            options: ClientOptions::new_with_transaction_name(name.into()),
        }
    }
}

/// The cloud with which you want to interact.
///
/// All variants require the cosmos account name. `Custom` also requires a valid
/// base URL (e.g. https://custom.documents.azure.com)
#[allow(unused)]
#[derive(Debug, Clone)]
enum CloudLocation {
    /// Azure public cloud
    Public(String),
    /// Azure China cloud
    China(String),
    // TODO: Other govt clouds?
    /// A custom base URL
    Custom { account: String, uri: String },
}

impl CloudLocation {
    /// the base URL for a given cloud location
    fn url(&self) -> String {
        match self {
            CloudLocation::Public(account) => format!("https://{}.documents.azure.com", account),
            CloudLocation::China(account) => format!("https://{}.documents.azure.cn", account),
            CloudLocation::Custom { uri, .. } => uri.clone(),
        }
    }
}
