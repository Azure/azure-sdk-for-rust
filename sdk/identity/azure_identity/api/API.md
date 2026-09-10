# azure_identity

- **Description**: Rust wrappers around Microsoft Azure REST APIs - Azure identity helper crate
- **Edition**: 2021

## Features

- `default`
  - `azure_core/default`
  - `reqwest_rustls`
- `client_certificate`
- `openssl`
- `reqwest`
- `reqwest_rustls`
- `tokio`

```rust
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
pub fn new_executor() -> std::sync::Arc<dyn Executor>;
pub struct AzureCliCredential {
}
impl AzureCliCredential {
    fn new(options: Option<AzureCliCredentialOptions>) -> azure_core::Result<Arc<Self>>;
}
impl Debug for AzureCliCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
impl TokenCredential for AzureCliCredential {
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn get_token(&self, scopes: &[&str], __arg2: Option<TokenRequestOptions<'_>>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = azure_core::Result<AccessToken>> + ::core::marker::Send>>;
}
#[derive(Clone, Default)]
pub struct AzureCliCredentialOptions {
    pub subscription: Option<String>,
    pub tenant_id: Option<String>,
    pub executor: Option<std::sync::Arc<dyn Executor>>,
}
impl Debug for AzureCliCredentialOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
pub struct AzureDeveloperCliCredential {
}
impl AzureDeveloperCliCredential {
    fn new(options: Option<AzureDeveloperCliCredentialOptions>) -> azure_core::Result<Arc<Self>>;
}
impl Debug for AzureDeveloperCliCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
impl TokenCredential for AzureDeveloperCliCredential {
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn get_token(&self, scopes: &[&str], __arg2: Option<TokenRequestOptions<'_>>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = azure_core::Result<AccessToken>> + ::core::marker::Send>>;
}
#[derive(Clone, Default)]
pub struct AzureDeveloperCliCredentialOptions {
    pub executor: Option<std::sync::Arc<dyn Executor>>,
    pub tenant_id: Option<String>,
}
impl Debug for AzureDeveloperCliCredentialOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
pub struct AzurePipelinesCredential(/* private fields */);
impl AzurePipelinesCredential {
    fn new<T>(tenant_id: String, client_id: String, service_connection_id: &str, system_access_token: T, options: Option<AzurePipelinesCredentialOptions>) -> azure_core::Result<Arc<Self>> where T: Into<Secret>;
}
impl Debug for AzurePipelinesCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
impl TokenCredential for AzurePipelinesCredential {
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn get_token(&self, scopes: &[&str], options: Option<TokenRequestOptions<'_>>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = azure_core::Result<AccessToken>> + ::core::marker::Send>>;
}
#[derive(Default)]
pub struct AzurePipelinesCredentialOptions {
    pub credential_options: crate::ClientAssertionCredentialOptions,
}
impl Debug for AzurePipelinesCredentialOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
pub struct ClientAssertionCredential<C> {
}
impl<C: ClientAssertion> ClientAssertionCredential<C> {
    fn new(tenant_id: String, client_id: String, assertion: C, options: Option<ClientAssertionCredentialOptions>) -> azure_core::Result<Arc<Self>>;
}
impl<C: ClientAssertion> TokenCredential for ClientAssertionCredential<C> {
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn get_token(&self, scopes: &[&str], options: Option<TokenRequestOptions<'_>>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = azure_core::Result<AccessToken>> + ::core::marker::Send>>;
}
impl<C> Debug for ClientAssertionCredential<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
#[derive(Default)]
pub struct ClientAssertionCredentialOptions {
    pub client_options: azure_core::http::ClientOptions,
}
impl Debug for ClientAssertionCredentialOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
#[cfg(feature = "client_certificate")]
pub struct ClientCertificateCredential {
}
#[cfg(feature = "client_certificate")]
impl ClientCertificateCredential {
    fn new(tenant_id: String, client_id: String, certificate: SecretBytes, options: Option<ClientCertificateCredentialOptions>) -> azure_core::Result<Arc<ClientCertificateCredential>>;
}
#[cfg(feature = "client_certificate")]
impl Debug for ClientCertificateCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
#[cfg(feature = "client_certificate")]
impl TokenCredential for ClientCertificateCredential {
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn get_token(&self, scopes: &[&str], options: Option<TokenRequestOptions<'_>>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = azure_core::Result<AccessToken>> + ::core::marker::Send>>;
}
#[cfg(feature = "client_certificate")]
#[derive(Clone, Default)]
pub struct ClientCertificateCredentialOptions {
    pub client_options: azure_core::http::ClientOptions,
    pub password: Option<azure_core::credentials::Secret>,
}
#[cfg(feature = "client_certificate")]
impl Debug for ClientCertificateCredentialOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
pub struct ClientSecretCredential {
}
impl ClientSecretCredential {
    fn new(tenant_id: &str, client_id: String, secret: Secret, options: Option<ClientSecretCredentialOptions>) -> Result<Arc<Self>>;
}
impl Debug for ClientSecretCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
impl TokenCredential for ClientSecretCredential {
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn get_token(&self, scopes: &[&str], options: Option<TokenRequestOptions<'_>>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<AccessToken>> + ::core::marker::Send>>;
}
#[derive(Default)]
pub struct ClientSecretCredentialOptions {
    pub client_options: azure_core::http::ClientOptions,
}
impl Debug for ClientSecretCredentialOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
pub struct DeveloperToolsCredential {
}
impl DeveloperToolsCredential {
    fn new(options: Option<DeveloperToolsCredentialOptions>) -> azure_core::Result<Arc<DeveloperToolsCredential>>;
}
impl Debug for DeveloperToolsCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
impl TokenCredential for DeveloperToolsCredential {
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn get_token(&self, scopes: &[&str], options: Option<TokenRequestOptions<'_>>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = azure_core::Result<AccessToken>> + ::core::marker::Send>>;
}
#[derive(Clone, Default)]
pub struct DeveloperToolsCredentialOptions {
    pub executor: Option<std::sync::Arc<dyn Executor>>,
}
impl Debug for DeveloperToolsCredentialOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
pub struct ManagedIdentityCredential {
}
impl ManagedIdentityCredential {
    fn new(options: Option<ManagedIdentityCredentialOptions>) -> azure_core::Result<Arc<Self>>;
}
impl Debug for ManagedIdentityCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
impl TokenCredential for ManagedIdentityCredential {
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn get_token(&self, scopes: &[&str], options: Option<TokenRequestOptions<'_>>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = azure_core::Result<AccessToken>> + ::core::marker::Send>>;
}
#[derive(Clone, Default)]
pub struct ManagedIdentityCredentialOptions {
    pub user_assigned_id: Option<UserAssignedId>,
    pub client_options: azure_core::http::ClientOptions,
}
impl Debug for ManagedIdentityCredentialOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
#[derive(Debug)]
pub struct TokenProxyClientOptions<'a> {
    pub certificate_authority: Option<&'a [u8]>,
    pub server_name: &'a str,
    pub resolved_addresses: &'a [std::net::SocketAddr],
}
pub struct WorkloadIdentityCredential(/* private fields */);
impl WorkloadIdentityCredential {
    fn new(options: Option<WorkloadIdentityCredentialOptions>) -> azure_core::Result<Arc<Self>>;
}
impl Debug for WorkloadIdentityCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
impl TokenCredential for WorkloadIdentityCredential {
    #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
    fn get_token(&self, scopes: &[&str], options: Option<TokenRequestOptions<'_>>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = azure_core::Result<AccessToken>> + ::core::marker::Send>>;
}
#[derive(Default)]
pub struct WorkloadIdentityCredentialOptions {
    pub credential_options: super::ClientAssertionCredentialOptions,
    pub client_id: Option<String>,
    pub tenant_id: Option<String>,
    pub token_file_path: Option<std::path::PathBuf>,
    pub enable_proxy: bool,
    pub proxy_client: Option<std::sync::Arc<dyn TokenProxyClient>>,
}
impl Debug for WorkloadIdentityCredentialOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum UserAssignedId {
    ClientId(String),
    ObjectId(String),
    ResourceId(String),
}
#[async_trait]
pub trait ClientAssertion: Send + Sync + fmt::Debug {
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn secret(&self, options: Option<ClientMethodOptions<'_>>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = azure_core::Result<String>> + ::core::marker::Send>>;
}
#[async_trait]
pub trait Executor: Send + Sync + fmt::Debug {
    #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn run(&self, program: &OsStr, args: &[&OsStr]) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = io::Result<Output>> + ::core::marker::Send>>;
}
pub trait TokenProxyClient: Debug + Send + Sync {
    fn create(&self, options: TokenProxyClientOptions<'_>) -> azure_core::Result<Arc<dyn HttpClient>>;
}
```
