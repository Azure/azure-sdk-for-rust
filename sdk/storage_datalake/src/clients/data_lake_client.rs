// use crate::bearer_token_authorization_policy::BearerTokenAuthorizationPolicy;
use crate::clients::FileSystemClient;
use crate::requests::*;
use crate::shared_key_authorization_policy::SharedKeyAuthorizationPolicy;
use azure_core::{ClientOptions, HttpClient, Pipeline};
use azure_storage::core::prelude::*;
use azure_storage::storage_shared_key_credential::StorageSharedKeyCredential;
use bytes::Bytes;
use http::method::Method;
use http::request::{Builder, Request};
use std::sync::Arc;

const DEFAULT_DNS_SUFFIX: &str = "dfs.core.windows.net";

#[derive(Debug, Clone)]
pub struct DataLakeClient {
    pipeline: Pipeline,
    storage_client: Arc<StorageClient>,
    custom_dns_suffix: Option<String>,
    url: String, // TODO: Use CloudLocation similar to CosmosClient
}

impl DataLakeClient {
    pub(crate) fn new_with_options(
        storage_client: Arc<StorageClient>,
        credential: StorageSharedKeyCredential,
        custom_dns_suffix: Option<String>,
        options: ClientOptions,
    ) -> Self {
        // we precalculate the url once in the constructor
        // so we do not have to do it at every request.
        let url = format!(
            "https://{}.{}",
            credential.account_name.to_owned(),
            match custom_dns_suffix.as_ref() {
                Some(custom_dns_suffix) => custom_dns_suffix,
                None => DEFAULT_DNS_SUFFIX,
            }
        );

        let per_call_policies = Vec::new();
        let auth_policy: Arc<dyn azure_core::Policy> =
            // TODO: Allow caller to choose auth policy, follow pattern of other clients
			// Arc::new(BearerTokenAuthorizationPolicy::new(bearer_token));
			Arc::new(SharedKeyAuthorizationPolicy::new(url.to_owned(), credential.to_owned()));

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
            custom_dns_suffix,
            url,
        }
    }

    pub fn new(
        storage_client: Arc<StorageClient>,
        credential: StorageSharedKeyCredential,
        custom_dns_suffix: Option<String>,
    ) -> DataLakeClient {
        Self::new_with_options(
            storage_client,
            credential,
            custom_dns_suffix,
            ClientOptions::default(),
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
