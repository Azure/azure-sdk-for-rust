use crate::authorization_policy::AuthorizationPolicy;
use crate::connection_string::ConnectionString;
use crate::error::Result;
use crate::operations::query::ExecuteQueryBuilder;
use azure_core::auth::TokenCredential;
use azure_core::{ClientOptions, Context, Pipeline, Request};
use azure_identity::token_credentials::{
    AzureCliCredential, ClientSecretCredential, DefaultAzureCredential,
    ImdsManagedIdentityCredential, TokenCredentialOptions,
};
use std::convert::TryFrom;
use std::fmt::Debug;
use std::sync::Arc;

/// Options for specifying how a OpenMetadata client will behave
#[derive(Clone, Default)]
pub struct KustoClientOptions {
    options: ClientOptions,
}

impl KustoClientOptions {
    /// Create new options
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(feature = "mock_transport_framework")]
    /// Create new options with a given transaction name
    pub fn new_with_transaction_name<T: Into<String>>(name: T) -> Self {
        Self {
            options: ClientOptions::new_with_transaction_name(name.into()),
        }
    }
}

fn new_pipeline_from_options(
    credential: Arc<dyn TokenCredential>,
    resource: &str,
    options: KustoClientOptions,
) -> Pipeline {
    let auth_policy = Arc::new(AuthorizationPolicy::new(credential, resource));
    // take care of adding the AuthorizationPolicy as **last** retry policy.
    let per_retry_policies: Vec<Arc<(dyn azure_core::Policy + 'static)>> = vec![auth_policy];

    Pipeline::new(
        option_env!("CARGO_PKG_NAME"),
        option_env!("CARGO_PKG_VERSION"),
        options.options,
        Vec::new(),
        per_retry_policies,
    )
}

/// Kusto client for Rust.
/// The client is a wrapper around the Kusto REST API.
/// To read more about it, go to https://docs.microsoft.com/en-us/azure/kusto/api/rest/
///
/// The primary methods are:
/// `execute_query`:  executes a KQL query against the Kusto service.
#[derive(Clone, Debug)]
pub struct KustoClient {
    pipeline: Pipeline,
    query_url: String,
    management_url: String,
}

impl KustoClient {
    pub fn new_with_options<T>(
        url: T,
        credential: Arc<dyn TokenCredential>,
        options: KustoClientOptions,
    ) -> Result<Self>
    where
        T: Into<String>,
    {
        let service_url: String = url.into();
        let service_url = service_url.trim_end_matches("/");
        let query_url = format!("{}/v2/rest/query", service_url);
        let management_url = format!("{}/v1/rest/mgmt", service_url);
        let pipeline = new_pipeline_from_options(credential, service_url, options);

        Ok(Self {
            pipeline,
            query_url,
            management_url,
        })
    }

    pub fn query_url(&self) -> &str {
        &self.query_url.as_str()
    }

    pub fn management_url(&self) -> &str {
        &self.management_url.as_str()
    }

    /// Execute a KQL query.
    /// To learn more about KQL go to https://docs.microsoft.com/en-us/azure/kusto/query/
    ///
    /// # Arguments
    ///
    /// * `database` - Database against query will be executed.
    /// * `query` - Query to be executed.
    pub fn execute_query(&self, database: &str, query: &str) -> ExecuteQueryBuilder {
        ExecuteQueryBuilder::new(self.clone(), database.into(), query.into(), Context::new())
    }

    pub(crate) fn prepare_request(&self, uri: &str, http_method: http::Method) -> Request {
        Request::new(uri.parse().unwrap(), http_method)
    }

    pub(crate) fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }
}

impl<'a> TryFrom<ConnectionString<'a>> for KustoClient {
    type Error = crate::error::Error;

    fn try_from(value: ConnectionString) -> Result<Self> {
        let service_url = value
            .data_source
            .expect("A data source / service url must always be specified");

        let credential: Arc<dyn TokenCredential> = match value {
            ConnectionString {
                application_client_id: Some(client_id),
                application_key: Some(client_secret),
                authority_id: Some(tenant_id),
                ..
            } => Arc::new(ClientSecretCredential::new(
                tenant_id.to_string(),
                client_id.to_string(),
                client_secret.to_string(),
                TokenCredentialOptions::default(),
            )),
            ConnectionString {
                msi_auth: Some(true),
                ..
            } => Arc::new(ImdsManagedIdentityCredential {}),
            ConnectionString {
                az_cli: Some(true), ..
            } => Arc::new(AzureCliCredential {}),
            _ => Arc::new(DefaultAzureCredential::default()),
        };
        Self::new_with_options(service_url, credential, KustoClientOptions::new())
    }
}

impl TryFrom<String> for KustoClient {
    type Error = crate::error::Error;

    fn try_from(value: String) -> Result<Self> {
        let connection_string = ConnectionString::new(value.as_str())?;
        Self::try_from(connection_string)
    }
}
