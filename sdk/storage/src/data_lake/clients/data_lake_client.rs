use crate::core::prelude::*;
use crate::data_lake::authorization_policy::AuthorizationPolicy;
use crate::data_lake::authorization_policy::DataLakeContext;
use crate::data_lake::requests::*;
use azure_core::pipeline::Pipeline;
use azure_core::prelude::*;
use azure_core::ClientOptions;
use bytes::Bytes;
use http::method::Method;
use http::request::{Builder, Request};
use std::sync::Arc;
use url::{ParseError, Url};

const DEFAULT_DNS_SUFFIX: &str = "dfs.core.windows.net";

pub trait AsDataLakeClient<A: Into<String>> {
    fn as_data_lake_client(
        &self,
        account: A,
        bearer_token: String,
    ) -> Result<Arc<DataLakeClient>, url::ParseError>;

    #[cfg(feature = "mock_transport_framework")]
    fn as_data_lake_client_with_transaction(
        &self,
        account: A,
        bearer_token: String,
        transaction_name: impl Into<String>,
    ) -> Result<Arc<DataLakeClient>, url::ParseError>;
}

pub trait AsCustomDataLakeClient<DS: Into<String>, A: Into<String>> {
    fn as_data_lake_client_with_custom_dns_suffix(
        &self,
        account: A,
        bearer_token: String,
        dns_suffix: DS,
    ) -> Result<Arc<DataLakeClient>, url::ParseError>;
}

impl<A: Into<String>> AsDataLakeClient<A> for StorageClient {
    fn as_data_lake_client(
        &self,
        account: A,
        bearer_token: String,
    ) -> Result<Arc<DataLakeClient>, url::ParseError> {
        DataLakeClient::new(self.clone(), account.into(), bearer_token, None)
    }

    #[cfg(feature = "mock_transport_framework")]
    fn as_data_lake_client_with_transaction(
        &self,
        account: A,
        bearer_token: String,
        transaction_name: impl Into<String>,
    ) -> Result<Arc<DataLakeClient>, url::ParseError> {
        DataLakeClient::new_with_transaction(
            self.clone(),
            account.into(),
            bearer_token,
            transaction_name,
        )
    }
}

impl<DS: Into<String>, A: Into<String>> AsCustomDataLakeClient<DS, A> for StorageClient {
    fn as_data_lake_client_with_custom_dns_suffix(
        &self,
        account: A,
        bearer_token: String,
        dns_suffix: DS,
    ) -> Result<Arc<DataLakeClient>, url::ParseError> {
        DataLakeClient::new(
            self.clone(),
            account.into(),
            bearer_token,
            Some(dns_suffix.into()),
        )
    }
}

#[derive(Debug, Clone)]
pub struct DataLakeClient {
    pipeline: Pipeline<DataLakeContext>,
    storage_client: StorageClient,
    account: String,
    custom_dns_suffix: Option<String>,
    url: Url, // TODO: Use CloudLocation similar to CosmosClient
}

impl DataLakeClient {
    pub(crate) fn new_with_options(
        storage_client: StorageClient,
        account: String,
        bearer_token: String,
        custom_dns_suffix: Option<String>,
        options: ClientOptions<DataLakeContext>,
    ) -> Result<Arc<Self>, url::ParseError> {
        // we precalculate the url once in the constructor
        // so we do not have to do it at every request.
        // This means we have to account for possible
        // malfolmed urls in the constructor, hence
        // the Result<_, url::ParseError>.
        let url = url::Url::parse(&format!(
            "https://{}.{}",
            account,
            match custom_dns_suffix.as_ref() {
                Some(custom_dns_suffix) => custom_dns_suffix,
                None => DEFAULT_DNS_SUFFIX,
            }
        ))?;

        let per_call_policies = Vec::new();
        let auth_policy: Arc<dyn azure_core::Policy<DataLakeContext>> =
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

        Ok(Arc::new(Self {
            pipeline,
            storage_client,
            account,
            custom_dns_suffix,
            url,
        }))
    }

    pub fn new(
        storage_client: StorageClient,
        account: String,
        bearer_token: String,
        custom_dns_suffix: Option<String>,
    ) -> Result<Arc<DataLakeClient>, ParseError> {
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
    ) -> Result<Arc<DataLakeClient>, ParseError> {
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

    pub(crate) fn url(&self) -> &Url {
        &self.url
    }

    pub fn list(&self) -> ListFileSystemsBuilder {
        ListFileSystemsBuilder::new(self)
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

    pub(crate) fn pipeline(&self) -> &Pipeline<DataLakeContext> {
        &self.pipeline
    }
}
