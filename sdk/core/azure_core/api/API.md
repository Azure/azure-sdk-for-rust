# azure_core

- **Description**: Rust wrappers around Microsoft Azure REST APIs - Core crate
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`
  - `reqwest`
  - `reqwest_deflate`
  - `reqwest_gzip`
  - `reqwest_rustls`
  - `tokio`
- `debug`
- `decimal`
- `hmac_openssl`
- `hmac_rust`
- `reqwest`
- `reqwest_deflate`
- `reqwest_gzip`
- `reqwest_rustls`
- `tokio`
- `xml`

```rust
#![deny(unsafe_code)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
pub use bytes::Bytes;
pub use uuid::Uuid;
pub use serde_json::value::Value;
#[macro_export]
macro_rules! request_header {
    ($(#[$outer:meta])* $name:ident, $header:ident) => { ... };
    ($(#[$outer:meta])* $name:ident, $header:ident, $(($(#[$inner:meta])*$variant:ident, $value:expr)), *) => { ... };
}
#[macro_export]
macro_rules! request_option {
    ($(#[$outer:meta])* $name:ident) => { ... };
}
#[macro_export]
macro_rules! request_query {
    ($(#[$outer:meta])* $name:ident, $option:expr) => { ... };
}
#[macro_export]
macro_rules! static_url {
    ( $(#[$outer:meta])* $name:ident, $value:expr) => { ... };
}
pub async fn sleep(duration: crate::time::Duration);
pub struct Error {
}
impl Error {
    fn downcast_mut<T: std::error::Error + 'static>(&mut self) -> Option<&mut T>;
    fn downcast_ref<T: std::error::Error + 'static>(&self) -> Option<&T>;
    fn get_mut(&mut self) -> Option<&mut dyn std::error::Error + Send + Sync + 'static>;
    fn get_ref(&self) -> Option<&dyn std::error::Error + Send + Sync + 'static>;
    #[cfg(feature = "http")]
    fn http_status(&self) -> Option<StatusCode>;
    fn into_downcast<T: std::error::Error + 'static>(self) -> std::result::Result<T, Self>;
    fn into_inner(self) -> std::result::Result<Box<dyn std::error::Error + Send + Sync>, Self>;
    fn kind(&self) -> &ErrorKind;
    fn new<E>(kind: ErrorKind, error: E) -> Self where E: Into<Box<dyn std::error::Error + Send + Sync>>;
    #[must_use]
    fn with_context<C>(self, message: C) -> Self where C: Into<Cow<'static, str>>;
    #[must_use]
    fn with_context_fn<F, C>(self, f: F) -> Self where F: FnOnce() -> C, C: Into<Cow<'static, str>>;
    #[must_use]
    fn with_error<E, C>(kind: ErrorKind, error: E, message: C) -> Self where E: Into<Box<dyn std::error::Error + Send + Sync>>, C: Into<Cow<'static, str>>;
    #[must_use]
    fn with_error_fn<E, F, C>(kind: ErrorKind, error: E, f: F) -> Self where E: Into<Box<dyn std::error::Error + Send + Sync>>, F: FnOnce() -> C, C: Into<Cow<'static, str>>;
    #[must_use]
    fn with_message<C>(kind: ErrorKind, message: C) -> Self where C: Into<Cow<'static, str>>;
    #[must_use]
    fn with_message_fn<F, C>(kind: ErrorKind, f: F) -> Self where F: FnOnce() -> C, C: Into<Cow<'static, str>>;
}
impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
impl Error for Error {
    fn source(&self) -> Option<&dyn std::error::Error + 'static>;
}
impl From<DecodeError> for Error {
    fn from(error: base64::DecodeError) -> Self;
}
impl From<Error> for Error {
    fn from(error: std::io::Error) -> Self;
}
impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self;
}
impl From<FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self;
}
impl From<Infallible> for Error {
    fn from(_: core::convert::Infallible) -> Self;
}
impl From<ParseBoolError> for Error {
    fn from(error: std::str::ParseBoolError) -> Self;
}
impl From<ParseError> for Error {
    fn from(error: url::ParseError) -> Self;
}
impl From<ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self;
}
impl From<Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self;
}
pub type Result<T> = std::result::Result<T, Error>;
pub mod async_runtime {
    pub fn get_async_runtime() -> std::sync::Arc<dyn AsyncRuntime>;
    pub fn set_async_runtime(runtime: std::sync::Arc<dyn AsyncRuntime>) -> crate::Result<()>;
    pub trait AbortableTask: Future<Output = std::result::Result<(), Box<dyn std::error::Error + Send>>> + Send {
        fn abort(&self);
    }
    pub trait AsyncRuntime: Send + Sync {
        #[must_use = "futures do nothing unless you `.await` or poll them"]
        fn sleep(&self, duration: Duration) -> TaskFuture;
        #[must_use = "futures do nothing unless you `.await` or poll them"]
        fn spawn(&self, f: TaskFuture) -> SpawnedTask;
        #[must_use = "futures do nothing unless you `.await` or poll them"]
        fn yield_now(&self) -> TaskFuture;
    }
    pub type SpawnedTask = std::pin::Pin<Box<dyn AbortableTask>>;
    pub type TaskFuture = std::pin::Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
}
pub mod base64 {
    pub fn decode<T>(input: T) -> crate::Result<Vec<u8>> where T: AsRef<[u8]>;
    pub fn decode_url_safe<T>(input: T) -> crate::Result<Vec<u8>> where T: AsRef<[u8]>;
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, <D as >::Error> where D: Deserializer<'de>;
    pub fn deserialize_url_safe<'de, D>(deserializer: D) -> Result<Vec<u8>, <D as >::Error> where D: Deserializer<'de>;
    pub fn encode<T>(input: T) -> String where T: AsRef<[u8]>;
    pub fn encode_url_safe<T>(input: T) -> String where T: AsRef<[u8]>;
    pub fn serialize<S, T>(to_serialize: &T, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: Serializer, T: AsRef<[u8]>;
    pub fn serialize_url_safe<S, T>(to_serialize: &T, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: Serializer, T: AsRef<[u8]>;
    pub mod option {
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, <D as >::Error> where D: Deserializer<'de>;
        pub fn deserialize_url_safe<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, <D as >::Error> where D: Deserializer<'de>;
        pub fn serialize<S, T>(to_serialize: &Option<T>, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: Serializer, T: AsRef<[u8]>;
        pub fn serialize_url_safe<S, T>(to_serialize: &Option<T>, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: Serializer, T: AsRef<[u8]>;
    }
}
pub mod cloud {
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    pub struct Audiences(/* private fields */);
    impl Audiences {
        fn get<T: 'static>(&self) -> Option<&str>;
        fn insert<T: 'static>(&mut self, audience: String);
        fn new() -> Self;
        fn with<T: 'static>(self, audience: String) -> Self;
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    #[non_exhaustive]
    pub struct CustomConfiguration {
        pub authority_host: String,
        pub audiences: Audiences,
    }
    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum CloudConfiguration {
        #[default]
        AzurePublic,
        AzureGovernment,
        AzureChina,
        Custom(CustomConfiguration),
    }
    impl From<CustomConfiguration> for CloudConfiguration {
        fn from(config: CustomConfiguration) -> Self;
    }
}
pub mod credentials {
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct AccessToken {
        pub token: Secret,
        pub expires_on: typespec_client_core::time::OffsetDateTime,
    }
    impl AccessToken {
        fn new<T>(token: T, expires_on: OffsetDateTime) -> Self where T: Into<Secret>;
    }
    #[derive(Clone, Eq, serde::Deserialize, serde::Serialize)]
    pub struct Secret(/* private fields */);
    impl Secret {
        fn new<T>(access_token: T) -> Self where T: Into<Cow<'static, str>>;
        fn secret(&self) -> &str;
    }
    impl Debug for Secret {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }
    impl From<&'static str> for Secret {
        fn from(access_token: &'static str) -> Self;
    }
    impl From<String> for Secret {
        fn from(access_token: String) -> Self;
    }
    impl PartialEq for Secret {
        fn eq(&self, other: &Self) -> bool;
    }
    #[derive(Clone, Eq)]
    pub struct SecretBytes(/* private fields */);
    impl SecretBytes {
        fn bytes(&self) -> &[u8];
        fn new<impl Into<Vec<u8>>: Into<Vec<u8>>>(bytes: impl Into<Vec<u8>>) -> Self;
    }
    impl Debug for SecretBytes {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl Display for SecretBytes {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl From<&[u8]> for SecretBytes {
        fn from(bytes: &[u8]) -> Self;
    }
    impl From<Bytes> for SecretBytes {
        fn from(bytes: Bytes) -> Self;
    }
    impl From<Vec<u8>> for SecretBytes {
        fn from(bytes: Vec<u8>) -> Self;
    }
    impl PartialEq for SecretBytes {
        fn eq(&self, other: &Self) -> bool;
    }
    #[derive(Clone, Debug, Default)]
    pub struct TokenRequestOptions<'a> {
        pub method_options: typespec_client_core::http::ClientMethodOptions<'a>,
    }
    #[async_trait]
    pub trait TokenCredential: Send + Sync + fmt::Debug {
        #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn get_token(&self, scopes: &[&str], options: Option<TokenRequestOptions<'_>>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = crate::Result<AccessToken>> + ::core::marker::Send>>;
    }
}
pub mod error {
    pub async fn check_success<T: Response>(response: T, options: Option<CheckSuccessOptions>) -> crate::Result<T>;
    #[derive(Debug, Default)]
    pub struct CheckSuccessOptions {
        pub success_codes: &'static [u16],
    }
    pub struct Error {
    }
    impl Error {
        fn downcast_mut<T: std::error::Error + 'static>(&mut self) -> Option<&mut T>;
        fn downcast_ref<T: std::error::Error + 'static>(&self) -> Option<&T>;
        fn get_mut(&mut self) -> Option<&mut dyn std::error::Error + Send + Sync + 'static>;
        fn get_ref(&self) -> Option<&dyn std::error::Error + Send + Sync + 'static>;
        #[cfg(feature = "http")]
        fn http_status(&self) -> Option<StatusCode>;
        fn into_downcast<T: std::error::Error + 'static>(self) -> std::result::Result<T, Self>;
        fn into_inner(self) -> std::result::Result<Box<dyn std::error::Error + Send + Sync>, Self>;
        fn kind(&self) -> &ErrorKind;
        fn new<E>(kind: ErrorKind, error: E) -> Self where E: Into<Box<dyn std::error::Error + Send + Sync>>;
        #[must_use]
        fn with_context<C>(self, message: C) -> Self where C: Into<Cow<'static, str>>;
        #[must_use]
        fn with_context_fn<F, C>(self, f: F) -> Self where F: FnOnce() -> C, C: Into<Cow<'static, str>>;
        #[must_use]
        fn with_error<E, C>(kind: ErrorKind, error: E, message: C) -> Self where E: Into<Box<dyn std::error::Error + Send + Sync>>, C: Into<Cow<'static, str>>;
        #[must_use]
        fn with_error_fn<E, F, C>(kind: ErrorKind, error: E, f: F) -> Self where E: Into<Box<dyn std::error::Error + Send + Sync>>, F: FnOnce() -> C, C: Into<Cow<'static, str>>;
        #[must_use]
        fn with_message<C>(kind: ErrorKind, message: C) -> Self where C: Into<Cow<'static, str>>;
        #[must_use]
        fn with_message_fn<F, C>(kind: ErrorKind, f: F) -> Self where F: FnOnce() -> C, C: Into<Cow<'static, str>>;
    }
    impl Debug for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl Error for Error {
        fn source(&self) -> Option<&dyn std::error::Error + 'static>;
    }
    impl From<DecodeError> for Error {
        fn from(error: base64::DecodeError) -> Self;
    }
    impl From<Error> for Error {
        fn from(error: std::io::Error) -> Self;
    }
    impl From<FromUtf8Error> for Error {
        fn from(error: std::string::FromUtf8Error) -> Self;
    }
    impl From<Infallible> for Error {
        fn from(_: core::convert::Infallible) -> Self;
    }
    impl From<ParseBoolError> for Error {
        fn from(error: std::str::ParseBoolError) -> Self;
    }
    impl From<ParseError> for Error {
        fn from(error: url::ParseError) -> Self;
    }
    impl From<ParseIntError> for Error {
        fn from(error: std::num::ParseIntError) -> Self;
    }
    impl From<Utf8Error> for Error {
        fn from(error: std::str::Utf8Error) -> Self;
    }
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ErrorDetail {
        pub code: Option<String>,
        pub message: Option<String>,
        pub target: Option<String>,
        #[serde(default)]
        pub details: Vec<ErrorDetail>,
        #[serde(rename = "innererror")]
        pub inner_error: Option<InnerError>,
        #[serde(flatten)]
        pub additional_properties: std::collections::HashMap<String, crate::Value>,
    }
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ErrorResponse {
        pub error: Option<ErrorDetail>,
    }
    impl TryFrom<Error> for ErrorResponse {
        type Error = Error;
        fn try_from(value: Error) -> Result<Self, <Self as >::Error>;
    }
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct InnerError {
        pub code: Option<String>,
        #[serde(rename = "innererror")]
        pub inner_error: Option<Box<InnerError>>,
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum ErrorKind {
        #[cfg(feature = "http")]
        HttpResponse { status: crate::http::StatusCode, error_code: Option<String>, raw_response: Option<Box<crate::http::RawResponse>> },
        Connection,
        Io,
        DataConversion,
        Credential,
        Other,
    }
    impl ErrorKind {
        fn into_error(self) -> Error;
    }
    impl Display for ErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl From<ErrorKind> for Error {
        fn from(kind: ErrorKind) -> Self;
    }
    pub trait Response: crate::private::Sealed {
        fn status(&self) -> StatusCode;
        fn try_into_raw_response(self) -> impl Future<Output = crate::Result<RawResponse>>;
    }
    pub trait ResultExt<T>: private::Sealed {
        fn with_context<C>(self, kind: ErrorKind, message: C) -> Result<T> where Self: Sized, C: Into<Cow<'static, str>>;
        fn with_context_fn<F, C>(self, kind: ErrorKind, f: F) -> Result<T> where Self: Sized, F: FnOnce() -> C, C: Into<Cow<'static, str>>;
        fn with_kind(self, kind: ErrorKind) -> Result<T> where Self: Sized;
    }
    pub type Result<T> = std::result::Result<T, Error>;
}
pub mod fmt {
    #[cfg(feature = "derive")]
    #[proc_macro_derive(SafeDebug, attributes(safe))]
    #[derive(SafeDebug)] {
        // Attributes available to this derive:
        #[safe]
    }
    pub fn to_ascii_lowercase(value: &str) -> std::borrow::Cow<'_, str>;
    pub mod as_string {
        pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, <D as >::Error> where D: Deserializer<'de>, T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Display;
        pub fn serialize<S, T>(to_serialize: &Option<T>, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: Serializer, T: std::string::ToString;
    }
    pub mod empty_as_null {
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, <D as >::Error> where D: Deserializer<'de>;
    }
}
pub mod hmac {
    #[cfg(feature = "hmac_openssl")]
    pub fn hmac_sha256(data: &str, key: &crate::credentials::Secret) -> crate::Result<String>;
}
pub mod http {
    pub use azure_core::http::pager::ItemIterator;
    pub use azure_core::http::pager::PageIterator;
    pub use azure_core::http::pager::Pager;
    pub use azure_core::http::poller::Poller;
    pub use url::Url;
    pub use azure_core::error::error_response::check_success;
    #[cfg_attr(not(feature = "reqwest"), allow(unused_variables))]
    pub fn new_http_client(options: Option<HttpClientOptions>) -> std::sync::Arc<dyn HttpClient>;
    #[derive(Debug)]
    pub struct AsyncRawResponse {
    }
    impl AsyncRawResponse {
        fn deconstruct(self) -> (StatusCode, Headers, AsyncResponseBody);
        fn from_bytes<impl Into<Bytes>: Into<Bytes>>(status: StatusCode, headers: Headers, bytes: impl Into<Bytes>) -> Self;
        fn headers(&self) -> &Headers;
        fn into_body(self) -> AsyncResponseBody;
        fn new(status: StatusCode, headers: Headers, stream: PinnedStream) -> Self;
        fn status(&self) -> StatusCode;
        async fn try_into_raw_response(self) -> crate::Result<RawResponse>;
    }
    impl<T> From<AsyncRawResponse> for AsyncResponse<T> {
        fn from(raw: AsyncRawResponse) -> Self;
    }
    impl<T> From<AsyncResponse<T>> for AsyncRawResponse {
        fn from(response: AsyncResponse<T>) -> Self;
    }
    pub struct AsyncResponse<T = ()> {
    }
    impl<T> AsyncResponse<T> {
        fn deconstruct(self) -> (StatusCode, Headers, AsyncResponseBody);
        fn headers(&self) -> &Headers;
        fn into_body(self) -> AsyncResponseBody;
        fn status(&self) -> StatusCode;
    }
    impl<T> Debug for AsyncResponse<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    #[pin_project]
    pub struct AsyncResponseBody(/* private fields */);
    impl AsyncResponseBody {
        async fn collect(self) -> crate::Result<Bytes>;
        async fn collect_into(self, buffer: &mut [u8]) -> crate::Result<usize>;
        async fn collect_string(self) -> crate::Result<String>;
    }
    impl Debug for AsyncResponseBody {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl Stream for AsyncResponseBody {
        type Item = Result<Bytes, Error>;
        fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<<Self as >::Item>>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct ClientMethodOptions<'a> {
        pub context: crate::http::Context<'a>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct ClientOptions {
        pub per_call_policies: Vec<std::sync::Arc<dyn Policy>>,
        pub per_try_policies: Vec<std::sync::Arc<dyn Policy>>,
        pub retry: RetryOptions,
        pub transport: Option<Transport>,
        pub user_agent: UserAgentOptions,
        pub instrumentation: InstrumentationOptions,
        pub logging: LoggingOptions,
        pub cloud: Option<std::sync::Arc<crate::cloud::CloudConfiguration>>,
    }
    #[derive(Clone, Debug)]
    pub struct Context<'a> {
    }
    impl<'a> Context<'a> {
        fn insert<E>(&mut self, entity: E) -> Option<Arc<E>> where E: Send + Sync + 'static;
        fn into_owned(self) -> Context<'static>;
        fn is_empty(&self) -> bool;
        fn new() -> Self;
        #[must_use]
        fn to_borrowed<'b>(&self) -> Context<'b> where 'a: 'b;
        #[must_use]
        fn to_owned(&self) -> Context<'static>;
        fn value<E>(&self) -> Option<&E> where E: Send + Sync + 'static;
        #[must_use]
        fn with_context<'b>(context: &'a Context<'_>) -> Context<'b> where 'a: 'b;
        #[must_use]
        fn with_value<E>(self, entity: E) -> Self where E: Send + Sync + 'static;
    }
    impl Default for Context<'_> {
        fn default() -> Self;
    }
    #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Etag(/* private fields */);
    impl AsRef<str> for Etag {
        fn as_ref(&self) -> &str;
    }
    impl Display for Etag {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl From<&str> for Etag {
        fn from(s: &str) -> Self;
    }
    impl From<Etag> for String {
        fn from(etag: Etag) -> Self;
    }
    impl From<Etag> for crate::http::headers::HeaderValue {
        fn from(etag: Etag) -> Self;
    }
    impl From<String> for Etag {
        fn from(s: String) -> Self;
    }
    impl FromStr for Etag {
        type Err = Error;
        fn from_str(s: &str) -> crate::Result<Self>;
    }
    #[derive(Clone, Debug)]
    pub struct ExponentialRetryOptions {
        pub initial_delay: crate::time::Duration,
        pub max_retries: u32,
        pub max_total_elapsed: crate::time::Duration,
        pub max_delay: crate::time::Duration,
    }
    impl Default for ExponentialRetryOptions {
        fn default() -> Self;
    }
    #[derive(Clone, Debug)]
    pub struct FixedRetryOptions {
        pub delay: crate::time::Duration,
        pub max_retries: u32,
        pub max_total_elapsed: crate::time::Duration,
    }
    impl Default for FixedRetryOptions {
        fn default() -> Self;
    }
    #[derive(Clone, Debug)]
    pub struct HttpClientOptions {
        pub automatic_decompression: bool,
    }
    impl Default for HttpClientOptions {
        fn default() -> Self;
    }
    #[derive(Clone, Debug, Default)]
    pub struct InstrumentationOptions {
        pub tracer_provider: Option<std::sync::Arc<dyn crate::tracing::TracerProvider>>,
    }
    #[derive(Clone, Debug)]
    #[cfg(feature = "json")]
    pub struct JsonFormat;
    #[cfg(feature = "json")]
    impl Format for JsonFormat {
        fn deserialize<T: DeserializeOwned, S: AsRef<[u8]>>(body: S) -> crate::Result<T>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct LoggingOptions {
        pub additional_allowed_header_names: Vec<std::borrow::Cow<'static, str>>,
        pub additional_allowed_query_params: Vec<std::borrow::Cow<'static, str>>,
    }
    #[derive(Clone, Debug)]
    pub struct NoFormat;
    impl Format for NoFormat {
        fn deserialize<T: DeserializeOwned, S: AsRef<[u8]>>(_body: S) -> crate::Result<T>;
    }
    #[derive(Clone, Debug)]
    pub struct Pipeline(/* private fields */);
    impl Pipeline {
        fn new(crate_name: Option<&'static str>, crate_version: Option<&'static str>, options: ClientOptions, per_call_policies: Vec<Arc<dyn Policy>>, per_try_policies: Vec<Arc<dyn Policy>>, pipeline_options: Option<PipelineOptions>) -> Self;
        async fn send(&self, ctx: &http::Context<'_>, request: &mut http::Request, options: Option<PipelineSendOptions>) -> crate::Result<http::RawResponse>;
        async fn stream(&self, ctx: &http::Context<'_>, request: &mut http::Request, options: Option<PipelineStreamOptions>) -> crate::Result<http::AsyncRawResponse>;
    }
    #[derive(Clone, Debug)]
    pub struct PipelineOptions {
        pub retry_headers: crate::http::policies::RetryHeaders,
        pub retry_status_codes: Vec<crate::http::StatusCode>,
    }
    impl Default for PipelineOptions {
        fn default() -> Self;
    }
    #[derive(Debug, Default)]
    pub struct PipelineSendOptions {
        pub skip_checks: bool,
        pub check_success: crate::error::CheckSuccessOptions,
    }
    #[derive(Debug, Default)]
    pub struct PipelineStreamOptions {
        pub skip_checks: bool,
        pub check_success: crate::error::CheckSuccessOptions,
    }
    pub struct QueryBuilder<'a> {
    }
    impl<'a> QueryBuilder<'a> {
        fn append_key_only<impl Into<Cow<'a, str>>: Into<Cow<'a, str>>>(&mut self, key: impl Into<Cow<'a, str>>) -> &mut Self;
        fn append_pair<impl Into<Cow<'a, str>>: Into<Cow<'a, str>>, impl Into<Cow<'a, str>>: Into<Cow<'a, str>>>(&mut self, key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> &mut Self;
        fn build(self);
        fn set_pair<impl Into<Cow<'a, str>>: Into<Cow<'a, str>>, impl Into<Cow<'a, str>>: Into<Cow<'a, str>>>(&mut self, key: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> &mut Self;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct RawResponse {
    }
    impl RawResponse {
        fn body(&self) -> &ResponseBody;
        fn deconstruct(self) -> (StatusCode, Headers, ResponseBody);
        fn from_bytes<impl Into<Bytes>: Into<Bytes>>(status: StatusCode, headers: Headers, body: impl Into<Bytes>) -> Self;
        fn headers(&self) -> &Headers;
        fn into_body(self) -> ResponseBody;
        fn status(&self) -> StatusCode;
    }
    #[derive(Clone)]
    pub struct Request {
    }
    impl Request {
        fn add_mandatory_header<T: Header>(&mut self, item: &T);
        fn add_optional_header<T: Header>(&mut self, item: &Option<T>);
        fn body(&self) -> &Body;
        fn body_mut(&mut self) -> &mut Body;
        fn headers(&self) -> &Headers;
        fn headers_mut(&mut self) -> &mut Headers;
        fn insert_header<K, V>(&mut self, key: K, value: V) where K: Into<HeaderName>, V: Into<HeaderValue>;
        fn insert_headers<T: AsHeaders>(&mut self, headers: &T) -> Result<(), <T as >::Error>;
        fn method(&self) -> Method;
        fn new(url: Url, method: Method) -> Self;
        fn path_and_query(&self) -> String;
        fn set_body<impl Into<Body>: Into<Body>>(&mut self, body: impl Into<Body>);
        #[cfg(feature = "json")]
        fn set_json<T>(&mut self, data: &T) -> crate::Result<()> where T: ?Sized + Serialize;
        fn set_method(&mut self, method: Method);
        fn url(&self) -> &Url;
        fn url_mut(&mut self) -> &mut Url;
    }
    impl Debug for Request {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    #[derive(Clone, Debug)]
    #[cfg(feature = "json")]
    pub struct RequestContent<T, F = crate::http::JsonFormat> {
    }
    impl<T, F> RequestContent<T, F> {
        fn body(&self) -> &Body;
        fn from(body: Vec<u8>) -> Self;
        fn from_reader<R>(reader: R, len: Option<u64>) -> Self where R: AsyncRead + Unpin + Send + Sync + 'static;
        fn from_seekable_reader<R>(reader: R, len: Option<u64>) -> Self where R: AsyncRead + AsyncSeek + Unpin + Send + Sync + 'static;
        fn from_slice(body: &[u8]) -> Self;
        fn from_static(body: &'static [u8]) -> Self;
        #[allow(clippy::should_implement_trait)]
        fn from_str(body: &str) -> Self;
    }
    impl<T, F> From<Box<dyn SeekableStream>> for RequestContent<T, F> {
        fn from(stream: Box<dyn SeekableStream>) -> Self;
    }
    impl<T, F> From<Bytes> for RequestContent<T, F> {
        fn from(body: Bytes) -> Self;
    }
    #[allow(unknown_lints, clippy::infallible_try_from, reason = "maintain a consistent pattern of `try_into()`")]
    impl<T, F> TryFrom<Decimal> for RequestContent<T, F> {
        type Error = Infallible;
        fn try_from(value: Decimal) -> Result<Self, Infallible>;
    }
    impl<T, F> TryFrom<HashMap<String, OffsetDateTime>> for RequestContent<T, F> {
        type Error = Error;
        fn try_from(body: HashMap<String, OffsetDateTime>) -> Result<Self, <Self as >::Error>;
    }
    #[allow(unknown_lints, clippy::infallible_try_from, reason = "maintain a consistent pattern of `try_into()`")]
    impl<T, F> TryFrom<Option<Decimal>> for RequestContent<T, F> {
        type Error = Infallible;
        fn try_from(value: Option<Decimal>) -> Result<Self, Infallible>;
    }
    impl<T, F> TryFrom<Vec<OffsetDateTime>> for RequestContent<T, F> {
        type Error = Error;
        fn try_from(body: Vec<OffsetDateTime>) -> Result<Self, <Self as >::Error>;
    }
    impl<T> TryFrom<&str> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: &str) -> $crate::Result<Self>;
    }
    impl<T> TryFrom<HashMap<String, &str>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: ::std::collections::HashMap<String, &str>) -> $crate::Result<Self>;
    }
    impl<T> TryFrom<HashMap<String, String>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: ::std::collections::HashMap<String, String>) -> $crate::Result<Self>;
    }
    impl<T> TryFrom<HashMap<String, Value>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: ::std::collections::HashMap<String, Value>) -> $crate::Result<Self>;
    }
    impl<T> TryFrom<HashMap<String, bool>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: ::std::collections::HashMap<String, bool>) -> $crate::Result<Self>;
    }
    impl<T> TryFrom<HashMap<String, f32>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: ::std::collections::HashMap<String, f32>) -> $crate::Result<Self>;
    }
    impl<T> TryFrom<HashMap<String, f64>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: ::std::collections::HashMap<String, f64>) -> $crate::Result<Self>;
    }
    impl<T> TryFrom<HashMap<String, i32>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: ::std::collections::HashMap<String, i32>) -> $crate::Result<Self>;
    }
    impl<T> TryFrom<HashMap<String, i64>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: ::std::collections::HashMap<String, i64>) -> $crate::Result<Self>;
    }
    impl<T> TryFrom<String> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: String) -> $crate::Result<Self>;
    }
    impl<T> TryFrom<Value> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: Value) -> $crate::Result<Self>;
    }
    impl<T> TryFrom<Vec<&str>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: ::std::vec::Vec<&str>) -> $crate::Result<Self>;
    }
    impl<T> TryFrom<Vec<String>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: ::std::vec::Vec<String>) -> $crate::Result<Self>;
    }
    impl<T> TryFrom<Vec<Value>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: ::std::vec::Vec<Value>) -> $crate::Result<Self>;
    }
    impl<T> TryFrom<Vec<bool>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: ::std::vec::Vec<bool>) -> $crate::Result<Self>;
    }
    impl<T> TryFrom<Vec<f32>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: ::std::vec::Vec<f32>) -> $crate::Result<Self>;
    }
    impl<T> TryFrom<Vec<f64>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: ::std::vec::Vec<f64>) -> $crate::Result<Self>;
    }
    impl<T> TryFrom<Vec<i32>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: ::std::vec::Vec<i32>) -> $crate::Result<Self>;
    }
    impl<T> TryFrom<Vec<i64>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: ::std::vec::Vec<i64>) -> $crate::Result<Self>;
    }
    impl<T> TryFrom<bool> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: bool) -> $crate::Result<Self>;
    }
    impl<T> TryFrom<f32> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: f32) -> $crate::Result<Self>;
    }
    impl<T> TryFrom<f64> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: f64) -> $crate::Result<Self>;
    }
    impl<T> TryFrom<i32> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: i32) -> $crate::Result<Self>;
    }
    impl<T> TryFrom<i64> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
        type Error = Error;
        fn try_from(value: i64) -> $crate::Result<Self>;
    }
    #[cfg(feature = "json")]
    pub struct Response<T, F = crate::http::JsonFormat> {
    }
    impl<T, F> Response<T, F> {
        fn body(&self) -> &ResponseBody;
        fn deconstruct(self) -> (StatusCode, Headers, ResponseBody);
        fn headers(&self) -> &Headers;
        fn into_body(self) -> ResponseBody;
        fn status(&self) -> StatusCode;
        fn to_raw_response(&self) -> RawResponse;
    }
    impl<T: DeserializeWith<F>, F: Format> Response<T, F> {
        fn into_model(self) -> crate::Result<T>;
    }
    impl<T, F> Debug for Response<T, F> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl<T, F> From<RawResponse> for Response<T, F> {
        fn from(raw: RawResponse) -> Self;
    }
    impl<T, F> From<Response<T, F>> for RawResponse {
        fn from(response: Response<T, F>) -> Self;
    }
    #[derive(Clone, Debug, Default)]
    pub struct RetryOptions {
    }
    impl RetryOptions {
        fn custom<T: RetryPolicy + 'static>(policy: Arc<T>) -> Self;
        fn exponential(options: ExponentialRetryOptions) -> Self;
        fn fixed(options: FixedRetryOptions) -> Self;
        fn none() -> Self;
    }
    #[derive(Clone, Debug)]
    pub struct Transport {
    }
    impl Transport {
        fn new(http_client: Arc<dyn HttpClient>) -> Self;
        async fn send(&self, ctx: &Context<'_>, request: &mut Request) -> Result<AsyncRawResponse>;
        fn with_policy(policy: Arc<dyn Policy>) -> Self;
    }
    impl Default for Transport {
        fn default() -> Self;
    }
    #[derive(Clone, Debug, Default)]
    pub struct UserAgentOptions {
        pub application_id: Option<String>,
    }
    #[cfg(feature = "xml")]
    #[derive(Clone, Debug)]
    pub struct XmlFormat;
    #[cfg(feature = "xml")]
    impl Format for XmlFormat {
        fn deserialize<T: DeserializeOwned, S: AsRef<[u8]>>(body: S) -> crate::Result<T>;
    }
    #[derive(Clone)]
    pub enum Body {
        Bytes(crate::Bytes),
        SeekableStream(Box<dyn SeekableStream>),
    }
    impl Body {
        fn is_empty(&self) -> Option<bool>;
        fn len(&self) -> Option<u64>;
        async fn reset(&mut self) -> crate::Result<()>;
        fn take(&mut self) -> Body;
    }
    impl Debug for Body {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl From<&Body> for crate::Bytes {
        fn from(value: &Body) -> Self;
    }
    impl From<Box<dyn SeekableStream>> for Body {
        fn from(seekable_stream: Box<dyn SeekableStream>) -> Self;
    }
    impl From<BytesStream> for crate::http::Body {
        fn from(stream: BytesStream) -> Self;
    }
    impl<B> From<B> for Body where B: Into<crate::Bytes> {
        fn from(bytes: B) -> Self;
    }
    impl<T, F> From<Body> for RequestContent<T, F> {
        fn from(body: Body) -> Self;
    }
    impl<T, F> From<RequestContent<T, F>> for Body {
        fn from(content: RequestContent<T, F>) -> Self;
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    pub enum Method {
        Delete,
        Get,
        Head,
        Patch,
        Post,
        Put,
    }
    impl Method {
        const fn as_str(&self) -> &'static str;
        fn is_safe(&self) -> bool;
    }
    impl AsRef<str> for Method {
        fn as_ref(&self) -> &'static str;
    }
    impl Display for Method {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl FromStr for Method {
        type Err = Error;
        fn from_str(s: &str) -> Result<Self, <Self as >::Err>;
    }
    impl Serialize for super::Method {
        fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> TryFrom<&'a str> for Method {
        type Error = Error;
        fn try_from(value: &'a str) -> Result<Self, <Self as >::Error>;
    }
    impl<'de> Deserialize<'de> for super::Method {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    #[non_exhaustive]
    #[repr(u16)]
    pub enum StatusCode {
        Continue = 100,
        SwitchingProtocols = 101,
        EarlyHints = 103,
        Ok = 200,
        Created = 201,
        Accepted = 202,
        NonAuthoritativeInformation = 203,
        NoContent = 204,
        ResetContent = 205,
        PartialContent = 206,
        MultiStatus = 207,
        ImUsed = 226,
        MultipleChoice = 300,
        MovedPermanently = 301,
        Found = 302,
        SeeOther = 303,
        NotModified = 304,
        TemporaryRedirect = 307,
        PermanentRedirect = 308,
        BadRequest = 400,
        Unauthorized = 401,
        PaymentRequired = 402,
        Forbidden = 403,
        NotFound = 404,
        MethodNotAllowed = 405,
        NotAcceptable = 406,
        ProxyAuthenticationRequired = 407,
        RequestTimeout = 408,
        Conflict = 409,
        Gone = 410,
        LengthRequired = 411,
        PreconditionFailed = 412,
        PayloadTooLarge = 413,
        UriTooLong = 414,
        UnsupportedMediaType = 415,
        RequestedRangeNotSatisfiable = 416,
        ExpectationFailed = 417,
        ImATeapot = 418,
        MisdirectedRequest = 421,
        UnprocessableEntity = 422,
        Locked = 423,
        FailedDependency = 424,
        TooEarly = 425,
        UpgradeRequired = 426,
        PreconditionRequired = 428,
        TooManyRequests = 429,
        RequestHeaderFieldsTooLarge = 431,
        UnavailableForLegalReasons = 451,
        InternalServerError = 500,
        NotImplemented = 501,
        BadGateway = 502,
        ServiceUnavailable = 503,
        GatewayTimeout = 504,
        HttpVersionNotSupported = 505,
        VariantAlsoNegotiates = 506,
        InsufficientStorage = 507,
        LoopDetected = 508,
        NotExtended = 510,
        NetworkAuthenticationRequired = 511,
        UnknownValue(u16),
    }
    impl StatusCode {
        fn canonical_reason(&self) -> Cow<'static, str>;
        fn is_client_error(&self) -> bool;
        fn is_informational(&self) -> bool;
        fn is_redirection(&self) -> bool;
        fn is_server_error(&self) -> bool;
        fn is_success(&self) -> bool;
    }
    impl Deref for StatusCode {
        type Target = u16;
        fn deref(&self) -> &<Self as >::Target;
    }
    impl Display for StatusCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl From<StatusCode> for u16 {
        fn from(code: StatusCode) -> u16;
    }
    impl From<u16> for StatusCode {
        fn from(num: u16) -> Self;
    }
    impl PartialEq<StatusCode> for u16 {
        fn eq(&self, other: &StatusCode) -> bool;
    }
    impl PartialEq<u16> for StatusCode {
        fn eq(&self, other: &u16) -> bool;
    }
    impl Serialize for super::StatusCode {
        fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::StatusCode {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    pub trait AppendToUrlQuery {
        fn append_to_url_query(&self, url: &mut Url);
    }
    pub trait DeserializeWith<F: Format>: Sized {
        fn deserialize_from(response: RawResponse) -> crate::Result<Self>;
        fn deserialize_with(body: ResponseBody) -> typespec::Result<Self>;
    }
    pub trait Format: std::fmt::Debug {
        fn deserialize<T: DeserializeOwned, S: AsRef<[u8]>>(body: S) -> crate::Result<T>;
    }
    #[async_trait]
    pub trait HttpClient: Send + Sync + std::fmt::Debug {
        #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn execute_request(&self, request: &Request) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<AsyncRawResponse>> + ::core::marker::Send>>;
    }
    pub trait Sanitizer {
        fn sanitize(&self, allowed_patterns: &HashSet<Cow<'static, str>>) -> String;
    }
    pub trait UrlExt: crate::private::Sealed {
        fn append_path<impl AsRef<str>: AsRef<str>>(&mut self, path: impl AsRef<str>);
        fn query_builder(&mut self) -> QueryBuilder<'_>;
    }
    pub const REDACTED_PATTERN: &str = "REDACTED";
    pub static DEFAULT_ALLOWED_HEADER_NAMES: std::sync::LazyLock<std::collections::HashSet<std::borrow::Cow<'static, str>>> = _;
    pub static DEFAULT_ALLOWED_QUERY_PARAMETERS: std::sync::LazyLock<std::collections::HashSet<std::borrow::Cow<'static, str>>> = _;
    pub mod headers {
        #[derive(Clone, Debug, Eq, Ord, PartialOrd)]
        pub struct HeaderName {
        }
        impl HeaderName {
            fn as_str(&self) -> &str;
            const fn from_static(s: &'static str) -> Self;
            const fn from_static_standard(s: &'static str) -> Self;
            fn is_standard(&self) -> bool;
        }
        impl From<&'static str> for HeaderName {
            fn from(s: &'static str) -> Self;
        }
        impl From<String> for HeaderName {
            fn from(s: String) -> Self;
        }
        impl Hash for HeaderName {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H);
        }
        impl PartialEq<&str> for HeaderName {
            fn eq(&self, other: &&str) -> bool;
        }
        impl PartialEq for HeaderName {
            fn eq(&self, other: &Self) -> bool;
        }
        #[derive(Clone, Eq, PartialEq)]
        pub struct HeaderValue(/* private fields */);
        impl HeaderValue {
            fn as_str(&self) -> &str;
            fn from_cow<C>(c: C) -> Self where C: Into<Cow<'static, str>>;
            const fn from_static(s: &'static str) -> Self;
        }
        impl Debug for HeaderValue {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
        }
        impl From<&'static str> for HeaderValue {
            fn from(s: &'static str) -> Self;
        }
        impl From<&String> for HeaderValue {
            fn from(s: &String) -> Self;
        }
        impl From<String> for HeaderValue {
            fn from(s: String) -> Self;
        }
        #[derive(Clone, Default, Eq, PartialEq)]
        pub struct Headers(/* private fields */);
        impl Headers {
            fn add<H>(&mut self, header: H) -> Result<(), <H as >::Error> where H: AsHeaders;
            fn get<H: FromHeaders>(&self) -> crate::Result<H>;
            fn get_as<V, E>(&self, key: &HeaderName) -> crate::Result<V> where V: FromStr<Err = E>, E: std::error::Error + Send + Sync + 'static;
            fn get_optional<H: FromHeaders>(&self) -> Result<Option<H>, <H as >::Error>;
            fn get_optional_as<V, E>(&self, key: &HeaderName) -> crate::Result<Option<V>> where V: FromStr<Err = E>, E: std::error::Error + Send + Sync + 'static;
            fn get_optional_str(&self, key: &HeaderName) -> Option<&str>;
            fn get_optional_string(&self, key: &HeaderName) -> Option<String>;
            fn get_optional_with<'a, V, F, E>(&self, key: &HeaderName, parser: F) -> crate::Result<Option<V>> where F: FnOnce(&'a HeaderValue) -> Result<V, E>, E: std::error::Error + Send + Sync + 'static;
            fn get_str(&self, key: &HeaderName) -> crate::Result<&str>;
            fn get_with<'a, V, F, E>(&self, key: &HeaderName, parser: F) -> crate::Result<V> where F: FnOnce(&'a HeaderValue) -> Result<V, E>, E: std::error::Error + Send + Sync + 'static;
            fn insert<K, V>(&mut self, key: K, value: V) where K: Into<HeaderName>, V: Into<HeaderValue>;
            fn iter(&self) -> impl Iterator<Item = (&HeaderName, &HeaderValue)>;
            fn new() -> Self;
            fn remove<K>(&mut self, key: K) -> Option<HeaderValue> where K: Into<HeaderName>;
        }
        impl Debug for Headers {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
        }
        impl From<HashMap<HeaderName, HeaderValue>> for Headers {
            fn from(c: std::collections::HashMap<HeaderName, HeaderValue>) -> Self;
        }
        impl IntoIterator for Headers {
            type IntoIter = IntoIter<HeaderName, HeaderValue>;
            type Item = (HeaderName, HeaderValue);
            fn into_iter(self) -> <Self as >::IntoIter;
        }
        pub trait AsHeaders {
            type Error: std::error::Error + Send + Sync + 'static;
            type Iter: Iterator<Item = (HeaderName, HeaderValue)>;
            fn as_headers(&self) -> Result<<Self as >::Iter, <Self as >::Error>;
        }
        pub trait FromHeaders: Sized {
            type Error: std::error::Error + Send + Sync + 'static;
            fn from_headers(headers: &Headers) -> Result<Option<Self>, <Self as >::Error>;
            fn header_names() -> &'static [&'static str];
        }
        pub trait Header {
            fn name(&self) -> HeaderName;
            fn value(&self) -> HeaderValue;
        }
        pub const ACCEPT: HeaderName = _;
        pub const ACCEPT_CHARSET: HeaderName = _;
        pub const AUTHORIZATION: HeaderName = _;
        pub const CLIENT_REQUEST_ID: HeaderName = _;
        pub const CONTENT_LENGTH: HeaderName = _;
        pub const CONTENT_TYPE: HeaderName = _;
        pub const ERROR_CODE: HeaderName = _;
        pub const ETAG: HeaderName = _;
        pub const IF_MATCH: HeaderName = _;
        pub const LAST_MODIFIED: HeaderName = _;
        pub const MS_DATE: HeaderName = _;
        pub const PREFER: HeaderName = _;
        pub const RETRY_AFTER: HeaderName = _;
        pub const RETRY_AFTER_MS: HeaderName = _;
        pub const USER_AGENT: HeaderName = _;
        pub const VERSION: HeaderName = _;
        pub const WWW_AUTHENTICATE: HeaderName = _;
        pub const X_MS_RETRY_AFTER_MS: HeaderName = _;
        pub static DEFAULT_ALLOWED_HEADER_NAMES: std::sync::LazyLock<std::collections::HashSet<std::borrow::Cow<'static, str>>> = _;
        pub mod content_type {
            pub const APPLICATION_X_WWW_FORM_URLENCODED: crate::http::headers::HeaderValue = _;
        }
    }
    pub mod pager {
        #[must_use = "streams do nothing unless you poll them"]
        #[pin_project(project = ItemIteratorProjection, project_replace = ItemIteratorProjectionOwned)]
        pub struct ItemIterator<P> where P: Page + Send {
        }
        impl<P> ItemIterator<P> where P: Page + Send {
            fn continuation(&self) -> Option<&PagerContinuation>;
            fn into_continuation(self) -> Option<PagerContinuation>;
            fn into_pages(self) -> PageIterator<P>;
            fn new<F: Fn(PagerState, PagerOptions<'static>) -> PagerResultFuture<P> + Send + 'static>(make_request: F, options: Option<PagerOptions<'static>>) -> Self;
        }
        impl<P> Debug for ItemIterator<P> where P: Page + Send {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
        }
        impl<P> FusedStream for ItemIterator<P> where P: Page + Send {
            fn is_terminated(&self) -> bool;
        }
        impl<P> Stream for ItemIterator<P> where P: Page + Send {
            type Item = Result<<P as Page>::Item, Error>;
            fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<<Self as >::Item>>;
        }
        #[must_use = "streams do nothing unless you poll them"]
        #[pin_project(project = PageIteratorProjection, project_replace = PageIteratorProjectionOwned)]
        pub struct PageIterator<P> where P: Send {
        }
        impl<P> PageIterator<P> where P: Send {
            fn continuation(&self) -> Option<&PagerContinuation>;
            fn into_continuation(self) -> Option<PagerContinuation>;
            fn new<F: Fn(PagerState, PagerOptions<'static>) -> PagerResultFuture<P> + Send + 'static>(make_request: F, options: Option<PagerOptions<'static>>) -> Self;
        }
        impl<P> Debug for PageIterator<P> where P: Send {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
        }
        impl<P> FusedStream for PageIterator<P> where P: Send {
            fn is_terminated(&self) -> bool;
        }
        impl<P> Stream for PageIterator<P> where P: Send {
            type Item = Result<P, Error>;
            fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<<Self as >::Item>>;
        }
        #[derive(Clone)]
        pub struct PagerOptions<'a> {
            pub context: crate::http::Context<'a>,
            pub continuation: Option<PagerContinuation>,
        }
        impl<'a> Debug for PagerOptions<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
        }
        impl<'a> Default for PagerOptions<'a> {
            fn default() -> Self;
        }
        #[derive(Clone, Debug, Eq, PartialEq)]
        #[non_exhaustive]
        pub enum PagerContinuation {
            Link(crate::http::Url),
            Token(String),
        }
        impl AsRef<str> for PagerContinuation {
            fn as_ref(&self) -> &str;
        }
        impl Display for PagerContinuation {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
        }
        impl From<PagerContinuation> for String {
            fn from(value: PagerContinuation) -> Self;
        }
        impl TryFrom<PagerContinuation> for crate::http::Url {
            type Error = Error;
            fn try_from(value: PagerContinuation) -> Result<Self, <Self as >::Error>;
        }
        pub enum PagerResult<P> {
            More { response: P, continuation: PagerContinuation },
            Done { response: P },
        }
        impl<P, F> PagerResult<crate::http::response::Response<P, F>> {
            fn from_response_header(response: Response<P, F>, header_name: &HeaderName) -> Self;
        }
        impl<P> Debug for PagerResult<P> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
        }
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub enum PagerState {
            #[default]
            Initial,
            More(PagerContinuation),
        }
        #[async_trait]
        pub trait Page {
            type IntoIter: Iterator<Item = <Self as >::Item>;
            type Item;
            #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
            fn into_items(self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = crate::Result<<Self as >::IntoIter>> + ::core::marker::Send>>;
        }
        pub type Pager<P, F = crate::http::JsonFormat> = ItemIterator<crate::http::response::Response<P, F>>;
        pub type PagerResultFuture<P> = std::pin::Pin<Box<dyn Future<Output = crate::Result<PagerResult<P>>> + Send + 'static>>;
    }
    pub mod policies {
        pub fn create_public_api_span(ctx: &crate::http::Context<'_>, tracer: Option<std::sync::Arc<dyn Tracer>>, public_api_instrumentation: Option<PublicApiInstrumentationInformation>) -> Option<std::sync::Arc<dyn Span>>;
        pub fn get_retry_after(headers: &crate::http::headers::Headers, now: fn() -> crate::time::OffsetDateTime, retry_headers: &[crate::http::headers::HeaderName]) -> Option<crate::time::Duration>;
        #[derive(Debug)]
        pub struct ClientRequestIdPolicy(/* private fields */);
        impl ClientRequestIdPolicy {
            const fn new() -> Self;
            const fn with_header_name(header: &'static str) -> Self;
        }
        impl Default for ClientRequestIdPolicy {
            fn default() -> Self;
        }
        impl From<HeaderName> for ClientRequestIdPolicy {
            fn from(header_name: headers::HeaderName) -> Self;
        }
        impl Policy for ClientRequestIdPolicy {
            #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
            fn send(&self, ctx: &Context<'_>, request: &mut Request, next: &[Arc<dyn Policy>]) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = PolicyResult> + ::core::marker::Send>>;
        }
        #[derive(Clone, Debug)]
        pub struct PublicApiInstrumentationInformation {
        }
        impl PublicApiInstrumentationInformation {
            fn new<impl Into<Cow<'static, str>>: Into<Cow<'static, str>>>(api_name: impl Into<Cow<'static, str>>, attributes: Vec<Attribute>) -> Self;
        }
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub struct RetryHeaders {
            pub retry_headers: Vec<crate::http::headers::HeaderName>,
        }
        pub struct RetryPolicyCount(/* private fields */);
        impl Deref for RetryPolicyCount {
            type Target = u32;
            fn deref(&self) -> &<Self as >::Target;
        }
        #[derive(Clone, Debug)]
        pub struct TransportPolicy {
        }
        impl TransportPolicy {
            fn new(transport: Transport) -> Self;
        }
        impl Policy for TransportPolicy {
            #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
            fn send(&self, ctx: &Context<'_>, request: &mut Request, next: &[Arc<dyn Policy>]) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = PolicyResult> + ::core::marker::Send>>;
        }
        #[derive(Clone, Debug)]
        pub struct UserAgentPolicy {
        }
        impl<'a> UserAgentPolicy {
            fn new(crate_name: Option<&'a str>, crate_version: Option<&'a str>, options: &UserAgentOptions) -> Self;
        }
        impl Policy for UserAgentPolicy {
            #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
            fn send(&self, ctx: &Context<'_>, request: &mut Request, next: &[Arc<dyn Policy>]) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = PolicyResult> + ::core::marker::Send>>;
        }
        #[async_trait]
        pub trait Policy: Send + Sync + std::fmt::Debug {
            #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
            fn send(&self, ctx: &Context<'_>, request: &mut Request, next: &[Arc<dyn Policy>]) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = PolicyResult> + ::core::marker::Send>>;
        }
        #[async_trait]
        pub trait RetryPolicy: std::fmt::Debug + Send + Sync {
            fn is_expired(&self, duration_since_start: Duration, retry_count: u32) -> bool;
            fn retry_headers(&self) -> Option<&RetryHeaders>;
            fn retry_status_codes(&self) -> &[StatusCode];
            fn sleep_duration(&self, retry_count: u32) -> Duration;
            #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
            fn wait(&self, retry_count: u32, retry_after: Option<Duration>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send>>;
        }
        pub type PolicyResult = typespec::error::Result<crate::http::AsyncRawResponse>;
        pub mod auth {
            #[derive(Clone, Debug)]
            pub struct BearerTokenAuthorizationPolicy {
            }
            impl BearerTokenAuthorizationPolicy {
                fn new<A, B>(credential: Arc<dyn TokenCredential>, scopes: A) -> Self where A: IntoIterator<Item = B>, B: Into<String>;
                fn with_on_challenge(self, on_challenge: Arc<dyn OnChallenge>) -> Self;
                fn with_on_request(self, on_request: Arc<dyn OnRequest>) -> Self;
            }
            impl Policy for BearerTokenAuthorizationPolicy {
                #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
                fn send(&self, ctx: &Context<'_>, request: &mut Request, next: &[Arc<dyn Policy>]) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = PolicyResult> + ::core::marker::Send>>;
            }
            #[async_trait]
            pub trait Authorizer: crate::private::Sealed + std::fmt::Debug + Send + Sync {
                #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
                fn authorize(&self, request: &mut Request, scopes: &[&str], options: TokenRequestOptions<'_>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
            }
            #[async_trait]
            pub trait OnChallenge: std::fmt::Debug + Send + Sync {
                #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
                fn on_challenge(&self, context: &Context<'_>, request: &mut Request, authorizer: &dyn Authorizer, headers: &Headers) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
            }
            #[async_trait]
            pub trait OnRequest: std::fmt::Debug + Send + Sync {
                #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
                fn on_request(&self, context: &mut Context<'_>, request: &mut Request, authorizer: &dyn Authorizer) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
            }
        }
    }
    pub mod poller {
        pub fn get_retry_after(headers: &crate::http::headers::Headers, retry_headers: &[crate::http::headers::HeaderName], options: &PollerOptions<'_>) -> crate::time::Duration;
        #[must_use = "streams do nothing unless you `.await` or poll them"]
        #[pin_project]
        pub struct Poller<M, F = crate::http::JsonFormat> where M: StatusMonitor, F: Format {
        }
        impl<M, F> Poller<M, F> where M: StatusMonitor, F: Format + Send {
            fn new<Fun>(make_request: Fun, options: Option<PollerOptions<'static>>) -> Self where M: Send + 'static, <M as >::Output: Send + 'static, <M as >::Format: Send + 'static, Fun: Fn(PollerState, PollerOptions<'static>) -> PollerResultFuture<M, F> + Send + 'static;
        }
        impl<M, F> Debug for Poller<M, F> where M: StatusMonitor, F: Format {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
        }
        impl<M, F> IntoFuture for Poller<M, F> where M: StatusMonitor + 'static, <M as >::Output: Send + 'static, <M as >::Format: Send + 'static, F: Format + 'static {
            type IntoFuture = Pin<Box<dyn Future<Output = <Poller<M, F> as IntoFuture>::Output> + Send>>;
            type Output = Result<Response<<M as StatusMonitor>::Output, <M as StatusMonitor>::Format>, Error>;
            fn into_future(self) -> <Self as >::IntoFuture;
        }
        impl<M, F> Stream for Poller<M, F> where M: StatusMonitor, F: Format {
            type Item = Result<Response<M, F>, Error>;
            fn poll_next(self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Option<<Self as >::Item>>;
        }
        #[derive(Clone, Debug)]
        pub struct PollerOptions<'a> {
            pub context: crate::http::Context<'a>,
            pub frequency: crate::time::Duration,
        }
        impl<'a> PollerOptions<'a> {
            #[must_use]
            fn into_owned(self) -> PollerOptions<'static>;
        }
        impl Default for PollerOptions<'_> {
            fn default() -> Self;
        }
        #[derive(Clone, Debug, Eq, PartialEq)]
        #[non_exhaustive]
        pub enum PollerContinuation {
            Links { next_link: crate::http::Url, final_link: Option<crate::http::Url> },
        }
        impl Display for PollerContinuation {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
        }
        pub enum PollerResult<M, F = crate::http::JsonFormat> where M: StatusMonitor, F: Format {
            InProgress { response: crate::http::Response<M, F>, retry_after: crate::time::Duration, continuation: PollerContinuation },
            Done { response: crate::http::Response<M, F> },
            Succeeded { response: crate::http::Response<M, F>, target: Box<dyn FnOnce() -> std::pin::Pin<Box<dyn Future<Output = crate::Result<crate::http::Response<<M as StatusMonitor>::Output, <M as StatusMonitor>::Format>>> + Send>> + Send> },
        }
        impl<M, F> Debug for PollerResult<M, F> where M: StatusMonitor, F: Format {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
        }
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub enum PollerState {
            #[default]
            Initial,
            More(PollerContinuation),
        }
        #[derive(Clone, Debug, Default, Eq, PartialEq)]
        pub enum PollerStatus {
            #[default]
            InProgress,
            Succeeded,
            Failed,
            Canceled,
            UnknownValue(String),
        }
        impl From<&str> for PollerStatus {
            fn from(value: &str) -> Self;
        }
        impl FromStr for PollerStatus {
            type Err = Infallible;
            fn from_str(value: &str) -> Result<Self, <Self as >::Err>;
        }
        impl<'de> Deserialize<'de> for PollerStatus {
            fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error> where D: serde::Deserializer<'de>;
        }
        pub trait StatusMonitor {
            type Format: Format + Send;
            type Output;
            fn status(&self) -> PollerStatus;
        }
        pub type PollerResultFuture<M, F> = std::pin::Pin<Box<dyn Future<Output = crate::Result<super::PollerResult<M, F>>> + Send + 'static>>;
    }
    pub mod request {
        #[derive(Clone)]
        pub struct Request {
        }
        impl Request {
            fn add_mandatory_header<T: Header>(&mut self, item: &T);
            fn add_optional_header<T: Header>(&mut self, item: &Option<T>);
            fn body(&self) -> &Body;
            fn body_mut(&mut self) -> &mut Body;
            fn headers(&self) -> &Headers;
            fn headers_mut(&mut self) -> &mut Headers;
            fn insert_header<K, V>(&mut self, key: K, value: V) where K: Into<HeaderName>, V: Into<HeaderValue>;
            fn insert_headers<T: AsHeaders>(&mut self, headers: &T) -> Result<(), <T as >::Error>;
            fn method(&self) -> Method;
            fn new(url: Url, method: Method) -> Self;
            fn path_and_query(&self) -> String;
            fn set_body<impl Into<Body>: Into<Body>>(&mut self, body: impl Into<Body>);
            #[cfg(feature = "json")]
            fn set_json<T>(&mut self, data: &T) -> crate::Result<()> where T: ?Sized + Serialize;
            fn set_method(&mut self, method: Method);
            fn url(&self) -> &Url;
            fn url_mut(&mut self) -> &mut Url;
        }
        impl Debug for Request {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
        }
        #[derive(Clone, Debug)]
        #[cfg(feature = "json")]
        pub struct RequestContent<T, F = crate::http::JsonFormat> {
        }
        impl<T, F> RequestContent<T, F> {
            fn body(&self) -> &Body;
            fn from(body: Vec<u8>) -> Self;
            fn from_reader<R>(reader: R, len: Option<u64>) -> Self where R: AsyncRead + Unpin + Send + Sync + 'static;
            fn from_seekable_reader<R>(reader: R, len: Option<u64>) -> Self where R: AsyncRead + AsyncSeek + Unpin + Send + Sync + 'static;
            fn from_slice(body: &[u8]) -> Self;
            fn from_static(body: &'static [u8]) -> Self;
            #[allow(clippy::should_implement_trait)]
            fn from_str(body: &str) -> Self;
        }
        impl<T, F> From<Box<dyn SeekableStream>> for RequestContent<T, F> {
            fn from(stream: Box<dyn SeekableStream>) -> Self;
        }
        impl<T, F> From<Bytes> for RequestContent<T, F> {
            fn from(body: Bytes) -> Self;
        }
        #[allow(unknown_lints, clippy::infallible_try_from, reason = "maintain a consistent pattern of `try_into()`")]
        impl<T, F> TryFrom<Decimal> for RequestContent<T, F> {
            type Error = Infallible;
            fn try_from(value: Decimal) -> Result<Self, Infallible>;
        }
        impl<T, F> TryFrom<HashMap<String, OffsetDateTime>> for RequestContent<T, F> {
            type Error = Error;
            fn try_from(body: HashMap<String, OffsetDateTime>) -> Result<Self, <Self as >::Error>;
        }
        #[allow(unknown_lints, clippy::infallible_try_from, reason = "maintain a consistent pattern of `try_into()`")]
        impl<T, F> TryFrom<Option<Decimal>> for RequestContent<T, F> {
            type Error = Infallible;
            fn try_from(value: Option<Decimal>) -> Result<Self, Infallible>;
        }
        impl<T, F> TryFrom<Vec<OffsetDateTime>> for RequestContent<T, F> {
            type Error = Error;
            fn try_from(body: Vec<OffsetDateTime>) -> Result<Self, <Self as >::Error>;
        }
        impl<T> TryFrom<&str> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: &str) -> $crate::Result<Self>;
        }
        impl<T> TryFrom<HashMap<String, &str>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: ::std::collections::HashMap<String, &str>) -> $crate::Result<Self>;
        }
        impl<T> TryFrom<HashMap<String, String>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: ::std::collections::HashMap<String, String>) -> $crate::Result<Self>;
        }
        impl<T> TryFrom<HashMap<String, Value>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: ::std::collections::HashMap<String, Value>) -> $crate::Result<Self>;
        }
        impl<T> TryFrom<HashMap<String, bool>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: ::std::collections::HashMap<String, bool>) -> $crate::Result<Self>;
        }
        impl<T> TryFrom<HashMap<String, f32>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: ::std::collections::HashMap<String, f32>) -> $crate::Result<Self>;
        }
        impl<T> TryFrom<HashMap<String, f64>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: ::std::collections::HashMap<String, f64>) -> $crate::Result<Self>;
        }
        impl<T> TryFrom<HashMap<String, i32>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: ::std::collections::HashMap<String, i32>) -> $crate::Result<Self>;
        }
        impl<T> TryFrom<HashMap<String, i64>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: ::std::collections::HashMap<String, i64>) -> $crate::Result<Self>;
        }
        impl<T> TryFrom<String> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: String) -> $crate::Result<Self>;
        }
        impl<T> TryFrom<Value> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: Value) -> $crate::Result<Self>;
        }
        impl<T> TryFrom<Vec<&str>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: ::std::vec::Vec<&str>) -> $crate::Result<Self>;
        }
        impl<T> TryFrom<Vec<String>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: ::std::vec::Vec<String>) -> $crate::Result<Self>;
        }
        impl<T> TryFrom<Vec<Value>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: ::std::vec::Vec<Value>) -> $crate::Result<Self>;
        }
        impl<T> TryFrom<Vec<bool>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: ::std::vec::Vec<bool>) -> $crate::Result<Self>;
        }
        impl<T> TryFrom<Vec<f32>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: ::std::vec::Vec<f32>) -> $crate::Result<Self>;
        }
        impl<T> TryFrom<Vec<f64>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: ::std::vec::Vec<f64>) -> $crate::Result<Self>;
        }
        impl<T> TryFrom<Vec<i32>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: ::std::vec::Vec<i32>) -> $crate::Result<Self>;
        }
        impl<T> TryFrom<Vec<i64>> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: ::std::vec::Vec<i64>) -> $crate::Result<Self>;
        }
        impl<T> TryFrom<bool> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: bool) -> $crate::Result<Self>;
        }
        impl<T> TryFrom<f32> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: f32) -> $crate::Result<Self>;
        }
        impl<T> TryFrom<f64> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: f64) -> $crate::Result<Self>;
        }
        impl<T> TryFrom<i32> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: i32) -> $crate::Result<Self>;
        }
        impl<T> TryFrom<i64> for $crate::http::RequestContent<T, $crate::http::JsonFormat> {
            type Error = Error;
            fn try_from(value: i64) -> $crate::Result<Self>;
        }
        #[derive(Clone)]
        pub enum Body {
            Bytes(crate::Bytes),
            SeekableStream(Box<dyn SeekableStream>),
        }
        impl Body {
            fn is_empty(&self) -> Option<bool>;
            fn len(&self) -> Option<u64>;
            async fn reset(&mut self) -> crate::Result<()>;
            fn take(&mut self) -> Body;
        }
        impl Debug for Body {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
        }
        impl From<&Body> for crate::Bytes {
            fn from(value: &Body) -> Self;
        }
        impl From<Box<dyn SeekableStream>> for Body {
            fn from(seekable_stream: Box<dyn SeekableStream>) -> Self;
        }
        impl From<BytesStream> for crate::http::Body {
            fn from(stream: BytesStream) -> Self;
        }
        impl<B> From<B> for Body where B: Into<crate::Bytes> {
            fn from(bytes: B) -> Self;
        }
        impl<T, F> From<Body> for RequestContent<T, F> {
            fn from(body: Body) -> Self;
        }
        impl<T, F> From<RequestContent<T, F>> for Body {
            fn from(content: RequestContent<T, F>) -> Self;
        }
        pub mod options {
            #[derive(Clone, Debug)]
            pub struct ClientRequestId(/* private fields */);
            impl ClientRequestId {
                const fn from_static(s: &'static str) -> Self;
                fn new<S>(s: S) -> Self where S: Into<std::borrow::Cow<'static, str>>;
            }
            impl Header for ClientRequestId {
                fn name(&self) -> $crate::http::headers::HeaderName;
                fn value(&self) -> $crate::http::headers::HeaderValue;
            }
            impl<S> From<S> for ClientRequestId where S: Into<std::borrow::Cow<'static, str>> {
                fn from(s: S) -> Self;
            }
            #[derive(Clone, Debug)]
            pub struct ContentType(/* private fields */);
            impl ContentType {
                const fn from_static(s: &'static str) -> Self;
                fn new<S>(s: S) -> Self where S: Into<std::borrow::Cow<'static, str>>;
            }
            impl ContentType {
                const APPLICATION_JSON: ContentType = _;
            }
            impl Header for ContentType {
                fn name(&self) -> $crate::http::headers::HeaderName;
                fn value(&self) -> $crate::http::headers::HeaderValue;
            }
            impl<S> From<S> for ContentType where S: Into<std::borrow::Cow<'static, str>> {
                fn from(s: S) -> Self;
            }
        }
        pub mod options {
            #[derive(Clone, Debug)]
            pub struct ContentType(/* private fields */);
            impl ContentType {
                const fn from_static(s: &'static str) -> Self;
                fn new<S>(s: S) -> Self where S: Into<std::borrow::Cow<'static, str>>;
            }
            impl ContentType {
                const APPLICATION_JSON: ContentType = _;
            }
            impl Header for ContentType {
                fn name(&self) -> $crate::http::headers::HeaderName;
                fn value(&self) -> $crate::http::headers::HeaderValue;
            }
            impl<S> From<S> for ContentType where S: Into<std::borrow::Cow<'static, str>> {
                fn from(s: S) -> Self;
            }
        }
    }
    pub mod response {
        #[derive(Debug)]
        pub struct AsyncRawResponse {
        }
        impl AsyncRawResponse {
            fn deconstruct(self) -> (StatusCode, Headers, AsyncResponseBody);
            fn from_bytes<impl Into<Bytes>: Into<Bytes>>(status: StatusCode, headers: Headers, bytes: impl Into<Bytes>) -> Self;
            fn headers(&self) -> &Headers;
            fn into_body(self) -> AsyncResponseBody;
            fn new(status: StatusCode, headers: Headers, stream: PinnedStream) -> Self;
            fn status(&self) -> StatusCode;
            async fn try_into_raw_response(self) -> crate::Result<RawResponse>;
        }
        impl<T> From<AsyncRawResponse> for AsyncResponse<T> {
            fn from(raw: AsyncRawResponse) -> Self;
        }
        impl<T> From<AsyncResponse<T>> for AsyncRawResponse {
            fn from(response: AsyncResponse<T>) -> Self;
        }
        pub struct AsyncResponse<T = ()> {
        }
        impl<T> AsyncResponse<T> {
            fn deconstruct(self) -> (StatusCode, Headers, AsyncResponseBody);
            fn headers(&self) -> &Headers;
            fn into_body(self) -> AsyncResponseBody;
            fn status(&self) -> StatusCode;
        }
        impl<T> Debug for AsyncResponse<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
        }
        #[pin_project]
        pub struct AsyncResponseBody(/* private fields */);
        impl AsyncResponseBody {
            async fn collect(self) -> crate::Result<Bytes>;
            async fn collect_into(self, buffer: &mut [u8]) -> crate::Result<usize>;
            async fn collect_string(self) -> crate::Result<String>;
        }
        impl Debug for AsyncResponseBody {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
        }
        impl Stream for AsyncResponseBody {
            type Item = Result<Bytes, Error>;
            fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<<Self as >::Item>>;
        }
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct RawResponse {
        }
        impl RawResponse {
            fn body(&self) -> &ResponseBody;
            fn deconstruct(self) -> (StatusCode, Headers, ResponseBody);
            fn from_bytes<impl Into<Bytes>: Into<Bytes>>(status: StatusCode, headers: Headers, body: impl Into<Bytes>) -> Self;
            fn headers(&self) -> &Headers;
            fn into_body(self) -> ResponseBody;
            fn status(&self) -> StatusCode;
        }
        #[cfg(feature = "json")]
        pub struct Response<T, F = crate::http::JsonFormat> {
        }
        impl<T, F> Response<T, F> {
            fn body(&self) -> &ResponseBody;
            fn deconstruct(self) -> (StatusCode, Headers, ResponseBody);
            fn headers(&self) -> &Headers;
            fn into_body(self) -> ResponseBody;
            fn status(&self) -> StatusCode;
            fn to_raw_response(&self) -> RawResponse;
        }
        impl<T: DeserializeWith<F>, F: Format> Response<T, F> {
            fn into_model(self) -> crate::Result<T>;
        }
        impl<T, F> Debug for Response<T, F> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
        }
        impl<T, F> From<RawResponse> for Response<T, F> {
            fn from(raw: RawResponse) -> Self;
        }
        impl<T, F> From<Response<T, F>> for RawResponse {
            fn from(response: Response<T, F>) -> Self;
        }
        #[derive(Clone, Eq, PartialEq)]
        pub struct ResponseBody(/* private fields */);
        impl ResponseBody {
            fn from_bytes<impl Into<Bytes>: Into<Bytes>>(bytes: impl Into<Bytes>) -> Self;
            fn into_string(self) -> crate::Result<String>;
            #[cfg(feature = "json")]
            fn json<T>(&self) -> crate::Result<T> where T: DeserializeOwned;
            #[cfg(feature = "xml")]
            fn xml<T>(&self) -> crate::Result<T> where T: DeserializeOwned;
        }
        impl AsRef<[u8]> for ResponseBody {
            #[inline]
            fn as_ref(&self) -> &[u8];
        }
        impl Debug for ResponseBody {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
        }
        impl Deref for ResponseBody {
            type Target = [u8];
            #[inline]
            fn deref(&self) -> &<Self as >::Target;
        }
        impl From<ResponseBody> for crate::Bytes {
            fn from(body: ResponseBody) -> Self;
        }
        pub type PinnedStream = std::pin::Pin<Box<dyn Stream<Item = crate::Result<crate::Bytes>> + Send>>;
    }
}
#[cfg(feature = "json")]
pub mod json {
    pub fn from_json<S, T>(body: S) -> crate::error::Result<T> where S: AsRef<[u8]>, T: DeserializeOwned;
    pub fn to_json<T>(value: &T) -> crate::error::Result<bytes::Bytes> where T: ?Sized + Serialize;
}
pub mod sleep {
    pub async fn sleep(duration: crate::time::Duration);
}
pub mod stream {
    #[derive(Clone)]
    pub struct BytesStream {
    }
    impl BytesStream {
        fn new<impl Into<Bytes>: Into<Bytes>>(bytes: impl Into<Bytes>) -> Self;
        fn new_empty() -> Self;
    }
    impl AsyncRead for BytesStream {
        fn poll_read(self: Pin<&mut Self>, _cx: &mut std::task::Context<'_>, buf: &mut [u8]) -> Poll<std::io::Result<usize>>;
    }
    impl Debug for BytesStream {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl From<Bytes> for BytesStream {
        fn from(bytes: Bytes) -> Self;
    }
    impl From<BytesStream> for crate::http::Body {
        fn from(stream: BytesStream) -> Self;
    }
    impl SeekableStream for BytesStream {
        fn len(&self) -> Option<u64>;
        #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
        fn reset(&mut self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = crate::Result<()>> + ::core::marker::Send>>;
    }
    impl Stream for BytesStream {
        type Item = Result<Bytes, Error>;
        fn poll_next(self: Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> Poll<Option<<Self as >::Item>>;
    }
    pub struct ReadStream<R> {
    }
    impl<R> ReadStream<R> {
        fn new(reader: R, len: Option<u64>) -> Self;
    }
    impl<R> AsyncRead for ReadStream<R> where R: AsyncRead + Unpin {
        fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<std::io::Result<usize>>;
    }
    impl<R> Clone for ReadStream<R> {
        fn clone(&self) -> Self;
    }
    impl<R> Debug for ReadStream<R> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl<R> SeekableStream for ReadStream<R> where R: AsyncRead + Unpin + Send + Sync {
        fn len(&self) -> Option<u64>;
        #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
        fn reset(&mut self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = crate::Result<()>> + ::core::marker::Send>>;
    }
    pub struct SeekableReadStream<R> {
    }
    impl<R> SeekableReadStream<R> {
        fn new(reader: R, len: Option<u64>) -> Self;
    }
    impl<R> AsyncRead for SeekableReadStream<R> where R: AsyncRead + AsyncSeek + Unpin {
        fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<std::io::Result<usize>>;
    }
    impl<R> Clone for SeekableReadStream<R> {
        fn clone(&self) -> Self;
    }
    impl<R> Debug for SeekableReadStream<R> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl<R> SeekableStream for SeekableReadStream<R> where R: AsyncRead + AsyncSeek + Unpin + Send + Sync {
        fn len(&self) -> Option<u64>;
        #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
        fn reset(&mut self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = crate::Result<()>> + ::core::marker::Send>>;
    }
    #[async_trait]
    pub trait SeekableStream: AsyncRead + Unpin + std::fmt::Debug + Send + Sync + DynClone {
        fn buffer_size(&self) -> usize;
        fn is_empty(&self) -> Option<bool>;
        fn len(&self) -> Option<u64>;
        #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn reset(&mut self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send>>;
    }
    pub const DEFAULT_BUFFER_SIZE: usize = _;
}
#[cfg(feature = "test")]
pub mod test {
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub enum RecordingMode {
        #[default]
        Playback,
        Record,
    }
    impl From<&RecordingMode> for &'static str {
        fn from(mode: &RecordingMode) -> Self;
    }
    impl From<RecordingMode> for &'static str {
        fn from(mode: RecordingMode) -> Self;
    }
    impl FromHeaders for RecordingMode {
        type Error = Error;
        fn from_headers(headers: &Headers) -> Result<Option<Self>, <Self as >::Error>;
        fn header_names() -> &'static [&'static str];
    }
    impl FromStr for RecordingMode {
        type Err = Error;
        fn from_str(s: &str) -> Result<Self, <Self as >::Err>;
    }
    impl Header for RecordingMode {
        fn name(&self) -> HeaderName;
        fn value(&self) -> HeaderValue;
    }
    #[derive(Clone, Copy, Default, Eq, Ord, PartialEq, PartialOrd)]
    pub enum TestMode {
        #[default]
        Playback,
        Record,
        Live,
    }
    impl TestMode {
        fn current() -> typespec::Result<Self>;
        fn current_opt() -> typespec::Result<Option<Self>>;
    }
    impl Debug for TestMode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl From<&TestMode> for &'static str {
        fn from(mode: &TestMode) -> Self;
    }
    impl From<TestMode> for &'static str {
        fn from(mode: TestMode) -> Self;
    }
    impl FromStr for TestMode {
        type Err = Error;
        fn from_str(s: &str) -> Result<Self, <Self as >::Err>;
    }
    impl TryFrom<TestMode> for RecordingMode {
        type Error = Error;
        fn try_from(value: TestMode) -> crate::Result<Self>;
    }
    pub const RECORDING_MODE: crate::http::headers::HeaderName = _;
}
pub mod time {
    pub use time::error::component_range::ComponentRange;
    pub use time::Duration;
    pub use time::offset_date_time::OffsetDateTime;
    pub use typespec_client_core::time::unix_time::parse_unix_time;
    pub use time::serde::rfc3339;
    pub use time::serde::timestamp;
    pub fn diff(first: OffsetDateTime, second: OffsetDateTime) -> Duration;
    pub fn duration_from_days(days: u64) -> Duration;
    pub fn duration_from_hours(hours: u64) -> Duration;
    pub fn duration_from_minutes(minutes: u64) -> Duration;
    pub fn parse_last_state_change(s: &str) -> crate::Result<OffsetDateTime>;
    pub fn parse_rfc3339(s: &str) -> crate::Result<OffsetDateTime>;
    pub fn parse_rfc7231(s: &str) -> crate::Result<OffsetDateTime>;
    pub fn to_last_state_change(date: &OffsetDateTime) -> String;
    pub fn to_rfc3339(date: &OffsetDateTime) -> String;
    pub fn to_rfc7231(date: &OffsetDateTime) -> String;
    pub mod iso8601 {
        pub fn deserialize<'de, D>(deserializer: D) -> Result<time::OffsetDateTime, <D as >::Error> where D: Deserializer<'de>;
        pub fn parse_iso8601(s: &str) -> crate::Result<time::OffsetDateTime>;
        pub fn serialize<S>(date: &time::OffsetDateTime, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: Serializer;
        pub fn to_iso8601(date: &time::OffsetDateTime) -> crate::Result<String>;
        pub mod option {
            pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<time::OffsetDateTime>, <D as >::Error> where D: Deserializer<'de>;
            pub fn serialize<S>(date: &Option<time::OffsetDateTime>, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: Serializer;
        }
    }
    pub mod rfc7231 {
        pub fn deserialize<'de, D>(deserializer: D) -> Result<time::OffsetDateTime, <D as >::Error> where D: Deserializer<'de>;
        pub fn serialize<S>(date: &time::OffsetDateTime, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: Serializer;
        pub mod option {
            pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<time::OffsetDateTime>, <D as >::Error> where D: Deserializer<'de>;
            pub fn serialize<S>(date: &Option<time::OffsetDateTime>, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: Serializer;
        }
    }
    pub mod unix_time {
        pub fn deserialize<'de, D>(deserializer: D) -> Result<crate::time::OffsetDateTime, <D as >::Error> where D: Deserializer<'de>;
        pub fn parse_unix_time(s: &str) -> crate::Result<crate::time::OffsetDateTime>;
        pub fn serialize<S>(date: &crate::time::OffsetDateTime, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: Serializer;
        pub mod option {
            pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<time::OffsetDateTime>, <D as >::Error> where D: Deserializer<'de>;
            pub fn serialize<S>(date: &Option<time::OffsetDateTime>, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: Serializer;
        }
    }
}
pub mod tracing {
    pub use azure_core::http::policies::instrumentation::public_api_instrumentation::PublicApiInstrumentationInformation;
    #[proc_macro_attribute]
    #[client]
    #[proc_macro_attribute]
    #[function]
    #[proc_macro_attribute]
    #[new]
    #[proc_macro_attribute]
    #[subclient]
    #[derive(Clone, Debug, PartialEq)]
    pub struct Attribute {
        pub key: std::borrow::Cow<'static, str>,
        pub value: AttributeValue,
    }
    #[derive(Clone, Debug, Default)]
    pub struct SpanOptions {
        pub start_time: Option<crate::time::OffsetDateTime>,
    }
    #[derive(Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum AttributeArray {
        Bool(Vec<bool>),
        I64(Vec<i64>),
        F64(Vec<f64>),
        String(Vec<String>),
    }
    #[derive(Clone, Debug, PartialEq)]
    #[non_exhaustive]
    pub enum AttributeValue {
        Bool(bool),
        I64(i64),
        F64(f64),
        String(String),
        Array(AttributeArray),
    }
    impl From<&Url> for AttributeValue {
        fn from(value: &Url) -> Self;
    }
    impl From<&str> for AttributeValue {
        fn from(value: &str) -> Self;
    }
    impl From<String> for AttributeValue {
        fn from(value: String) -> Self;
    }
    impl From<Url> for AttributeValue {
        fn from(value: Url) -> Self;
    }
    impl From<Vec<String>> for AttributeValue {
        fn from(value: Vec<String>) -> Self;
    }
    impl From<Vec<bool>> for AttributeValue {
        fn from(value: Vec<bool>) -> Self;
    }
    impl From<Vec<f64>> for AttributeValue {
        fn from(value: Vec<f64>) -> Self;
    }
    impl From<Vec<i64>> for AttributeValue {
        fn from(value: Vec<i64>) -> Self;
    }
    impl From<bool> for AttributeValue {
        fn from(value: bool) -> Self;
    }
    impl From<f64> for AttributeValue {
        fn from(value: f64) -> Self;
    }
    impl From<i32> for AttributeValue {
        fn from(value: i32) -> Self;
    }
    impl From<i64> for AttributeValue {
        fn from(value: i64) -> Self;
    }
    impl From<u16> for AttributeValue {
        fn from(value: u16) -> Self;
    }
    impl From<u32> for AttributeValue {
        fn from(value: u32) -> Self;
    }
    impl PartialEq<&str> for AttributeValue {
        fn eq(&self, other: &&str) -> bool;
    }
    impl PartialEq<i64> for AttributeValue {
        fn eq(&self, other: &i64) -> bool;
    }
    #[derive(Debug, Default, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum SpanKind {
        #[default]
        Internal,
        Client,
        Server,
        Producer,
        Consumer,
    }
    #[derive(Debug, PartialEq)]
    #[non_exhaustive]
    pub enum SpanStatus {
        Unset,
        Error { description: String },
    }
    pub trait AsAny {
        fn as_any(&self) -> &dyn std::any::Any;
    }
    pub trait Span: AsAny + Send + Sync {
        fn end(&self);
        fn end_at(&self, end_time: OffsetDateTime);
        fn is_recording(&self) -> bool;
        fn propagate_headers(&self, request: &mut Request);
        fn record_error(&self, error: &dyn std::error::Error);
        fn set_attribute(&self, key: &'static str, value: AttributeValue);
        fn set_current(&self, context: &Context<'_>) -> Box<dyn SpanGuard>;
        fn set_status(&self, status: SpanStatus);
        fn span_id(&self) -> [u8; 8];
    }
    pub trait SpanGuard {
        fn end(self);
    }
    pub trait Tracer: Send + Sync + Debug {
        fn namespace(&self) -> Option<&'static str>;
        fn start_span(&self, name: Cow<'static, str>, kind: SpanKind, attributes: Vec<Attribute>) -> Arc<dyn Span>;
        fn start_span_with_options(&self, name: Cow<'static, str>, kind: SpanKind, attributes: Vec<Attribute>, options: SpanOptions) -> Arc<dyn Span>;
        fn start_span_with_parent(&self, name: Cow<'static, str>, kind: SpanKind, attributes: Vec<Attribute>, parent: Arc<dyn Span>) -> Arc<dyn Span>;
        fn start_span_with_parent_and_options(&self, name: Cow<'static, str>, kind: SpanKind, attributes: Vec<Attribute>, parent: Arc<dyn Span>, options: SpanOptions) -> Arc<dyn Span>;
    }
    pub trait TracerProvider: Send + Sync + Debug {
        fn get_tracer(&self, namespace_name: Option<&'static str>, crate_name: &'static str, crate_version: Option<&'static str>) -> Arc<dyn Tracer>;
    }
}
#[cfg(feature = "xml")]
pub mod xml {
    pub use quick_xml::serde_helpers::text_content as content;
    pub fn from_xml<S, T>(body: S) -> crate::error::Result<T> where S: AsRef<[u8]>, T: DeserializeOwned;
    pub fn to_xml<T>(value: &T) -> crate::error::Result<bytes::Bytes> where T: serde::Serialize;
    pub fn to_xml_with_root<T>(root_tag: &str, value: &T) -> crate::error::Result<bytes::Bytes> where T: serde::Serialize;
}
```
