use crate::authorization_policy::AuthorizationPolicy;
use crate::clients::FileSystemClient;
use crate::operations::*;
use azure_core::{ClientOptions, Context, HttpClient, Pipeline, Result};
use azure_storage::clients::ServiceType;
use azure_storage::core::prelude::*;
use bytes::Bytes;
use http::method::Method;
use http::request::{Builder, Request};
use std::sync::Arc;

const DEFAULT_DNS_SUFFIX: &str = "dfs.core.windows.net";

#[derive(Debug, Clone)]
pub struct DataLakeClient {
    pipeline: Pipeline,
    #[allow(unused)]
    account: String,
    custom_dns_suffix: Option<String>,
    url: String, // TODO: Use CloudLocation similar to CosmosClient
    pub(crate) context: Context,
}

impl DataLakeClient {
    pub(crate) fn new_with_options(
        account: String,
        bearer_token: String,
        custom_dns_suffix: Option<String>,
        options: ClientOptions,
    ) -> Result<Self> {
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

        let mut context = Context::new();
        context.insert(ServiceType::Blob);

        Ok(Self {
            pipeline,
            account,
            custom_dns_suffix,
            url,
            context,
        })
    }

    pub fn new(
        account: String,
        bearer_token: String,
        custom_dns_suffix: Option<String>,
    ) -> Result<Self> {
        Self::new_with_options(
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
            ClientOptions::new_with_transaction_name(transaction_name.into()),
        )
    }

    pub fn custom_dns_suffix(&self) -> Option<&str> {
        self.custom_dns_suffix.as_deref()
    }

    pub(crate) fn url(&self) -> &str {
        &self.url
    }

    pub fn list_file_systems(&self) -> ListFileSystems {
        ListFileSystems::new(self.clone(), Some(self.context.clone()))
    }

    pub fn into_file_system_client(self, file_system_name: String) -> FileSystemClient {
        FileSystemClient::new(self, file_system_name)
    }

    // pub(crate) fn prepare_request(
    //     &self,
    //     url: &str,
    //     method: &Method,
    //     http_header_adder: &dyn Fn(Builder) -> Builder,
    //     request_body: Option<Bytes>,
    // ) -> crate::Result<(Request<Bytes>, url::Url)> {
    //     self.storage_client
    //         .prepare_request(url, method, http_header_adder, request_body)
    // }

    /// Prepares' an `azure_core::Request`. This function will
    /// add the cloud location to the URI suffix and generate
    /// a Request with the specified HTTP Method.
    /// It will also set the body to an empty Bytes instance.
    /// *Note*: This call does not handle authorization as
    /// it will be done by the `AuthorizationPolicy`.
    ///
    /// Note: Eventually this method will replace `prepare_request` fully.
    pub(crate) fn prepare_request_pipeline(
        &self,
        uri: &str,
        http_method: http::Method,
    ) -> azure_core::Request {
        // let uri = format!("{}/{}", self.cloud_location.url(), uri_path);
        Builder::new()
            .method(http_method)
            .uri(uri)
            .body(bytes::Bytes::new())
            .unwrap()
            .into()
    }

    pub fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    pub fn http_client(&self) -> &dyn HttpClient {
        self.pipeline.http_client()
    }
}
