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

/// A builder for the cosmos client.
#[derive(Debug, Clone)]
pub struct CosmosClientBuilder {
    cloud_location: CloudLocation,
    options: ClientOptions,
}

impl CosmosClientBuilder {
    /// Create a new instance of `CosmosClientBuilder`.
    #[must_use]
    pub fn new(account: impl Into<String>, auth_token: AuthorizationToken) -> Self {
        Self::with_location(CloudLocation::Public {
            account: account.into(),
            auth_token,
        })
    }

    /// Create a new instance of `CosmosClientBuilder` with a cloud location.
    #[must_use]
    pub fn with_location(cloud_location: CloudLocation) -> Self {
        Self {
            options: ClientOptions::default(),
            cloud_location,
        }
    }

    /// Convert the builder into a `CosmosClient` instance.
    #[must_use]
    pub fn build(self) -> CosmosClient {
        let auth_token = self.cloud_location.auth_token();
        CosmosClient {
            pipeline: new_pipeline_from_options(self.options, auth_token),
            cloud_location: self.cloud_location,
        }
    }

    /// Set the cloud location.
    #[must_use]
    pub fn cloud_location(mut self, cloud_location: CloudLocation) -> Self {
        self.cloud_location = cloud_location;
        self
    }

    /// Set the retry options.
    #[must_use]
    pub fn retry(mut self, retry: impl Into<azure_core::RetryOptions>) -> Self {
        self.options = self.options.retry(retry);
        self
    }

    /// Set the transport options.
    #[must_use]
    pub fn transport(mut self, transport: impl Into<azure_core::TransportOptions>) -> Self {
        self.options = self.options.transport(transport);
        self
    }
}

/// A plain Cosmos client.
#[derive(Debug, Clone)]
pub struct CosmosClient {
    pipeline: Pipeline,
    cloud_location: CloudLocation,
}

impl CosmosClient {
    /// Create a new `CosmosClient` which connects to the account's instance in the public Azure cloud.
    #[must_use]
    pub fn new(account: impl Into<String>, auth_token: AuthorizationToken) -> Self {
        CosmosClientBuilder::new(account, auth_token).build()
    }

    /// Create a new `CosmosClientBuilder`.
    #[must_use]
    pub fn builder(
        account: impl Into<String>,
        auth_token: AuthorizationToken,
    ) -> CosmosClientBuilder {
        CosmosClientBuilder::new(account, auth_token)
    }

    /// Set the auth token used
    #[must_use]
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
    Public {
        account: String,
        auth_token: AuthorizationToken,
    },
    /// Azure China cloud
    China {
        account: String,
        auth_token: AuthorizationToken,
    },
    /// Use the well-known Cosmos emulator
    Emulator { address: String, port: u16 },
    /// A custom base URL
    Custom {
        uri: String,
        auth_token: AuthorizationToken,
    },
}

impl CloudLocation {
    /// the base URL for a given cloud location
    fn url(&self) -> String {
        match self {
            CloudLocation::Public { account, .. } => {
                format!("https://{account}.documents.azure.com")
            }
            CloudLocation::China { account, .. } => format!("https://{account}.documents.azure.cn"),
            CloudLocation::Custom { uri, .. } => uri.clone(),
            CloudLocation::Emulator { address, port } => format!("https://{address}:{port}"),
        }
    }

    fn auth_token(&self) -> AuthorizationToken {
        match self {
            CloudLocation::Public { auth_token, .. } => auth_token.clone(),
            CloudLocation::China { auth_token, .. } => auth_token.clone(),
            CloudLocation::Emulator { .. } => {
                AuthorizationToken::primary_from_base64(EMULATOR_ACCOUNT_KEY).unwrap()
            }
            CloudLocation::Custom { auth_token, .. } => auth_token.clone(),
        }
    }
}
