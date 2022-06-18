use crate::authorization_policies::{
    SharedKeyAuthorizationPolicy, TokenCredentialAuthorizationPolicy,
};
use crate::clients::FileSystemClient;
use crate::operations::ListFileSystemsBuilder;
use azure_core::auth::TokenCredential;
use azure_core::{ClientOptions, Context, HttpClient, Pipeline};
use azure_storage::core::clients::ServiceType;
use azure_storage::core::storage_shared_key_credential::StorageSharedKeyCredential;
use std::sync::Arc;

const DEFAULT_DNS_SUFFIX: &str = "dfs.core.windows.net";
const DEFAULT_RESOURCE: &str = "https://storage.azure.com/";

#[derive(Debug, Clone)]
pub struct DataLakeClient {
    pipeline: Pipeline,
    custom_dns_suffix: Option<String>,
    url: String, // TODO: Use CloudLocation similar to CosmosClient
    pub(crate) context: Context,
}

impl DataLakeClient {
    pub fn new_with_shared_key(
        credential: StorageSharedKeyCredential,
        custom_dns_suffix: Option<String>,
        options: ClientOptions,
    ) -> Self {
        let account_name = credential.account_name.clone();
        let auth_policy: Arc<dyn azure_core::Policy> =
            Arc::new(SharedKeyAuthorizationPolicy::new(credential));
        Self::new_with_auth_policy(auth_policy, account_name, custom_dns_suffix, options)
    }

    pub fn new_with_token_credential<A: Into<String>>(
        credential: Arc<dyn TokenCredential>,
        account_name: A,
        custom_dns_suffix: Option<String>,
        options: ClientOptions,
    ) -> Self {
        let auth_policy: Arc<dyn azure_core::Policy> = Arc::new(
            TokenCredentialAuthorizationPolicy::new(credential, DEFAULT_RESOURCE),
        );
        Self::new_with_auth_policy(auth_policy, account_name, custom_dns_suffix, options)
    }

    pub fn new_with_auth_policy<A: Into<String>>(
        auth_policy: Arc<dyn azure_core::Policy>,
        account_name: A,
        custom_dns_suffix: Option<String>,
        options: ClientOptions,
    ) -> Self {
        // we precalculate the url once in the constructor
        // so we do not have to do it at every request.
        let url = format!(
            "https://{}.{}",
            account_name.into(),
            match custom_dns_suffix.as_ref() {
                Some(custom_dns_suffix) => custom_dns_suffix,
                None => DEFAULT_DNS_SUFFIX,
            }
        );

        let per_call_policies = Vec::new();

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

        Self {
            pipeline,
            custom_dns_suffix,
            url,
            context,
        }
    }

    pub fn new(credential: StorageSharedKeyCredential, custom_dns_suffix: Option<String>) -> Self {
        Self::new_with_shared_key(credential, custom_dns_suffix, ClientOptions::default())
    }

    pub fn custom_dns_suffix(&self) -> Option<&str> {
        self.custom_dns_suffix.as_deref()
    }

    pub(crate) fn url(&self) -> &str {
        &self.url
    }

    pub fn list_file_systems(&self) -> ListFileSystemsBuilder {
        ListFileSystemsBuilder::new(self.clone(), Some(self.context.clone()))
    }

    pub fn into_file_system_client<FS>(self, file_system_name: FS) -> FileSystemClient
    where
        FS: Into<String>,
    {
        FileSystemClient::new(self, file_system_name.into())
    }

    pub(crate) fn prepare_request(
        &self,
        uri: &str,
        http_method: http::Method,
    ) -> azure_core::Request {
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
