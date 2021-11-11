use crate::core::prelude::*;
use crate::data_lake::authorization_policy::AuthorizationPolicy;
use crate::data_lake::clients::FileSystemClient;
use crate::data_lake::requests::*;
use azure_core::pipeline::Pipeline;
use azure_core::prelude::*;
use azure_core::ClientOptions;
use bytes::Bytes;
use http::method::Method;
use http::request::{Builder, Request};
use std::sync::Arc;

const DEFAULT_DNS_SUFFIX: &str = "dfs.core.windows.net";

#[derive(Debug, Clone)]
pub struct DataLakeClient {
    pipeline: Pipeline,
    storage_client: Arc<StorageClient>,
    account: String,
    custom_dns_suffix: Option<String>,
    url: String, // TODO: Use CloudLocation similar to CosmosClient
}

impl DataLakeClient {
    pub(crate) fn new_with_options(
        storage_client: Arc<StorageClient>,
        account: String,
        bearer_token: String,
        custom_dns_suffix: Option<String>,
        options: ClientOptions,
    ) -> Self {
        // we precalculate the url once in the constructor
        // so we do not have to do it at every request.
        let url = format!(
            "https://{}.{}",
            account,
            match custom_dns_suffix.as_ref() {
                Some(custom_dns_suffix) => custom_dns_suffix,
                None => DEFAULT_DNS_SUFFIX,
            }
        );

        let per_call_policies = Vec::new();
        let auth_policy: Arc<dyn azure_core::Policy> =
            Arc::new(AuthorizationPolicy::new(bearer_token));

        // take care of adding the AuthorizationPolicy as **last** retry policy.
        // Policies can change the url and/or the headers and the AuthorizationPolicy
        // must be able to inspect them or the resulting token will be invalid.
        let per_retry_policies = vec![auth_policy];

        let pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options,
            per_call_policies,
            per_retry_policies,
        );

        Self {
            pipeline,
            storage_client,
            account,
            custom_dns_suffix,
            url,
        }
    }

    pub fn new(
        storage_client: Arc<StorageClient>,
        account: String,
        bearer_token: String,
        custom_dns_suffix: Option<String>,
    ) -> DataLakeClient {
        Self::new_with_options(
            storage_client,
            account,
            bearer_token,
            custom_dns_suffix,
            ClientOptions::default(),
        )
    }

    #[cfg(feature = "mock_transport_framework")]
    pub fn new_with_transaction(
        storage_client: Arc<StorageClient>,
        account: String,
        bearer_token: String,
        transaction_name: impl Into<String>,
    ) -> DataLakeClient {
        Self::new_with_options(
            storage_client,
            account,
            bearer_token,
            None,
            ClientOptions::<DataLakeContext>::new_with_transaction_name(transaction_name.into()),
        )
    }

    pub fn custom_dns_suffix(&self) -> Option<&str> {
        self.custom_dns_suffix.as_deref()
    }

    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.storage_client.storage_account_client().http_client()
    }

    pub(crate) fn url(&self) -> &str {
        &self.url
    }

    pub fn list(&self) -> ListFileSystemsBuilder {
        ListFileSystemsBuilder::new(self)
    }

    pub fn into_file_system_client(self, file_system_name: String) -> FileSystemClient {
        FileSystemClient::new(self, file_system_name)
    }

    pub(crate) fn prepare_request(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<Bytes>,
    ) -> crate::Result<(Request<Bytes>, url::Url)> {
        self.storage_client
            .prepare_request(url, method, http_header_adder, request_body)
    }

    pub(crate) fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }
}
