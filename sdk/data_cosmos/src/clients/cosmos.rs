use crate::clients::DatabaseClient;
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
    pub fn new(account: impl Into<String>, auth_token: AuthorizationToken) -> Self {
        Self {
            pipeline: new_pipeline_from_options(ClientOptions::default(), auth_token),
            cloud_location: CloudLocation::Public {
                account: account.into(),
            },
        }
    }

    /// Create a new instance of `CosmosClient` connecting to a specific cloud.
    pub fn with_cloud(auth_token: AuthorizationToken, cloud_location: CloudLocation) -> Self {
        Self {
            pipeline: new_pipeline_from_options(ClientOptions::default(), auth_token),
            cloud_location,
        }
    }

    /// Create a new `CosmosClient` which connects to the account's instance in Azure emulator
    pub fn with_emulator(address: impl AsRef<str>, port: u16) -> Self {
        let auth_token = AuthorizationToken::primary_from_base64(EMULATOR_ACCOUNT_KEY).unwrap();
        Self {
            pipeline: new_pipeline_from_options(ClientOptions::default(), auth_token),
            cloud_location: CloudLocation::Custom {
                uri: format!("https://{}:{}", address.as_ref(), port),
            },
        }
    }

    #[cfg(feature = "mock_transport_framework")]
    /// Create a new instance of `CosmosClient` using a mock backend. The
    /// transaction name is used to look up which files to read to validate the
    /// request and mock the response.
    // TODO(yosh): consider adding a general way to replace transports, and remove this method.
    pub fn with_mock(
        account: impl Into<String>,
        auth_token: AuthorizationToken,
        transaction_name: impl Into<String>,
    ) -> Self {
        let options = ClientOptions::new_with_transaction_name(transaction_name.into());
        Self {
            pipeline: new_pipeline_from_options(options, auth_token),
            cloud_location: CloudLocation::Public {
                account: account.into(),
            },
        }
    }

    /// Set the auth token used
    pub fn auth_token(mut self, auth_token: AuthorizationToken) -> Self {
        // we replace the AuthorizationPolicy. This is
        // the last-1 policy by construction.
        let auth_policy: Arc<dyn azure_core::Policy> =
            Arc::new(crate::AuthorizationPolicy::new(auth_token));

        self.pipeline
            .replace_policy(auth_policy, self.pipeline.policies().len() - 2);
        self
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
    pub(crate) fn request(&self, uri_path: &str, http_method: azure_core::Method) -> Request {
        let uri = format!("{}/{}", self.cloud_location.url(), uri_path);
        Request::new(uri.parse().unwrap(), http_method)
    }

    /// Sends a request through the pipeline
    pub(crate) async fn send(
        &self,
        mut request: Request,
        mut context: Context,
        resource_type: ResourceType,
    ) -> azure_core::Result<Response> {
        self.pipeline
            .send(context.insert(resource_type), &mut request)
            .await
    }

    /// Access this client's pipeline
    pub(crate) fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }
}

/// Create a Pipeline from CosmosOptions
fn new_pipeline_from_options(
    options: ClientOptions,
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
        options,
        Vec::new(),
        per_retry_policies,
    )
}

/// The cloud with which you want to interact.
// TODO: Other govt clouds?
#[derive(Debug, Clone)]
pub enum CloudLocation {
    /// Azure public cloud
    Public { account: String },
    /// Azure China cloud
    China { account: String },
    /// A custom base URL
    Custom { uri: String },
}

impl CloudLocation {
    /// the base URL for a given cloud location
    fn url(&self) -> String {
        match self {
            CloudLocation::Public { account } => format!("https://{}.documents.azure.com", account),
            CloudLocation::China { account } => format!("https://{}.documents.azure.cn", account),
            CloudLocation::Custom { uri } => uri.clone(),
        }
    }
}
