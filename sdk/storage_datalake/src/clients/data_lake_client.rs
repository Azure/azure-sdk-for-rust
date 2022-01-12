use crate::clients::FileSystemClient;
use crate::operations::ListFileSystems;
use crate::shared_key_authorization_policy::SharedKeyAuthorizationPolicy;
use azure_core::headers::*;
use azure_core::{ClientOptions, Context, HttpClient, Pipeline};
use azure_storage::core::storage_shared_key_credential::StorageSharedKeyCredential;
use azure_storage::core::{clients::ServiceType, Result};
use http::request::Builder;
use std::sync::Arc;

pub(crate) const HEADER_VERSION: &str = "x-ms-version";
pub(crate) const AZURE_VERSION: &str = "2019-12-12";
const DEFAULT_DNS_SUFFIX: &str = "dfs.core.windows.net";

#[derive(Debug, Clone)]
pub struct DataLakeClient {
    pipeline: Pipeline,
    custom_dns_suffix: Option<String>,
    url: String, // TODO: Use CloudLocation similar to CosmosClient
    pub(crate) context: Context,
}

impl DataLakeClient {
    pub(crate) fn new_with_options(
        credential: StorageSharedKeyCredential,
        custom_dns_suffix: Option<String>,
        options: ClientOptions,
    ) -> Result<Self> {
        // we precalculate the url once in the constructor
        // so we do not have to do it at every request.
        let url = format!(
            "https://{}.{}",
            credential.account_name,
            match custom_dns_suffix.as_ref() {
                Some(custom_dns_suffix) => custom_dns_suffix,
                None => DEFAULT_DNS_SUFFIX,
            }
        );

        let per_call_policies = Vec::new();
        let auth_policy: Arc<dyn azure_core::Policy> =
            // TODO: Allow caller to choose auth policy, follow pattern of other clients
			// Arc::new(BearerTokenAuthorizationPolicy::new(bearer_token));
			Arc::new(SharedKeyAuthorizationPolicy::new(url.to_owned(), credential));

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
            custom_dns_suffix,
            url,
            context,
        })
    }

    pub fn new(
        credential: StorageSharedKeyCredential,
        custom_dns_suffix: Option<String>,
    ) -> Result<Self> {
        Self::new_with_options(credential, custom_dns_suffix, ClientOptions::default())
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

    pub(crate) fn prepare_request_pipeline(
        &self,
        uri: &str,
        http_method: http::Method,
    ) -> azure_core::Request {
        let dt = chrono::Utc::now();
        let time = format!("{}", dt.format("%a, %d %h %Y %T GMT"));

        Builder::new()
            .method(http_method)
            .uri(uri)
            .header(MS_DATE, time)
            .header(HEADER_VERSION, AZURE_VERSION)
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
