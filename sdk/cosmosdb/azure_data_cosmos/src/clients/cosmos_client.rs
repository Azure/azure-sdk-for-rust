use crate::authorization_policy::AuthorizationPolicy;
use crate::{CosmosClientOptions, DatabaseClient};
use azure_core::auth::TokenCredential;
use azure_core::{Pipeline, Url};
use std::sync::Arc;

#[cfg(feature = "key-auth")]
use azure_core::auth::Secret;

/// Client for Azure Cosmos DB.
#[derive(Debug, Clone)]
pub struct CosmosClient {
    endpoint: Url,
    pub(crate) pipeline: Pipeline,

    #[allow(dead_code)]
    options: CosmosClientOptions,
}

pub trait CosmosClientMethods {
    fn database(&self, name: impl Into<String>) -> DatabaseClient;
}

impl CosmosClient {
    /// Creates a new CosmosClient from the specified [TokenCredential] and [CosmosClientOptions].
    pub fn new(
        endpoint: impl AsRef<str>,
        credential: Arc<dyn TokenCredential>,
        options: Option<CosmosClientOptions>,
    ) -> azure_core::Result<Self> {
        let options = options.unwrap_or_default();
        Ok(Self {
            endpoint: endpoint.as_ref().parse()?,
            pipeline: create_pipeline(
                AuthorizationPolicy::from_token_credential(credential),
                options.client_options.clone(),
            ),
            options,
        })
    }

    #[cfg(feature = "key-auth")]
    pub fn with_shared_key(
        endpoint: impl AsRef<str>,
        key: impl Into<Secret>,
        options: Option<CosmosClientOptions>,
    ) -> azure_core::Result<Self> {
        let options = options.unwrap_or_default();
        Ok(Self {
            endpoint: endpoint.as_ref().parse()?,
            pipeline: create_pipeline(
                AuthorizationPolicy::from_shared_key(key.into()),
                options.client_options.clone(),
            ),
            options,
        })
    }

    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }
}

impl CosmosClientMethods for CosmosClient {
    fn database(&self, id: impl Into<String>) -> DatabaseClient {
        DatabaseClient::new(self.clone(), id.into())
    }
}

fn create_pipeline(
    auth_policy: AuthorizationPolicy,
    client_options: azure_core::ClientOptions,
) -> Pipeline {
    Pipeline::new(
        option_env!("CARGO_PKG_NAME"),
        option_env!("CARGO_PKG_VERSION"),
        client_options,
        Vec::new(),
        vec![Arc::new(auth_policy)],
    )
}
