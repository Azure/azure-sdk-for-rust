# azure_core_test

- **Description**: Utilities for testing client libraries built on azure_core.
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`
- `tracing`
- `tracing-subscriber`

```rust
pub fn load_dotenv_file<impl AsRef<Path>: AsRef<std::path::Path>>(cargo_dir: impl AsRef<std::path::Path>) -> azure_core::Result<()>;
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ApplyCondition {
    #[serde(rename = "UriRegex")]
    pub uri_regex: String,
}
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyKeySanitizer {
    pub json_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_for_replace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ApplyCondition>,
}
impl AsHeaders for BodyKeySanitizer {
    type Error = Infallible;
    type Iter = Once<(HeaderName, HeaderValue)>;
    fn as_headers(&self) -> Result<<Self as >::Iter, <Self as >::Error>;
}
impl Sanitizer for BodyKeySanitizer {
}
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyRegexSanitizer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_for_replace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ApplyCondition>,
}
impl AsHeaders for BodyRegexSanitizer {
    type Error = Infallible;
    type Iter = Once<(HeaderName, HeaderValue)>;
    fn as_headers(&self) -> Result<<Self as >::Iter, <Self as >::Error>;
}
impl Sanitizer for BodyRegexSanitizer {
}
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyStringSanitizer {
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ApplyCondition>,
}
impl AsHeaders for BodyStringSanitizer {
    type Error = Infallible;
    type Iter = Once<(HeaderName, HeaderValue)>;
    fn as_headers(&self) -> Result<<Self as >::Iter, <Self as >::Error>;
}
impl Sanitizer for BodyStringSanitizer {
}
#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomDefaultMatcher {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compare_bodies: Option<bool>,
    #[serde(serialize_with = "join", skip_serializing_if = "Vec::is_empty")]
    pub excluded_headers: Vec<&'static str>,
    #[serde(serialize_with = "join", skip_serializing_if = "Vec::is_empty")]
    pub ignored_headers: Vec<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_query_ordering: Option<bool>,
    #[serde(serialize_with = "join", skip_serializing_if = "Vec::is_empty")]
    pub ignored_query_parameters: Vec<&'static str>,
}
impl Default for CustomDefaultMatcher {
    fn default() -> Self;
}
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralRegexSanitizer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_for_replace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ApplyCondition>,
}
impl AsHeaders for GeneralRegexSanitizer {
    type Error = Infallible;
    type Iter = Once<(HeaderName, HeaderValue)>;
    fn as_headers(&self) -> Result<<Self as >::Iter, <Self as >::Error>;
}
impl Sanitizer for GeneralRegexSanitizer {
}
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralStringSanitizer {
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ApplyCondition>,
}
impl AsHeaders for GeneralStringSanitizer {
    type Error = Infallible;
    type Iter = Once<(HeaderName, HeaderValue)>;
    fn as_headers(&self) -> Result<<Self as >::Iter, <Self as >::Error>;
}
impl Sanitizer for GeneralStringSanitizer {
}
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeaderRegexSanitizer {
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_for_replace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ApplyCondition>,
}
impl AsHeaders for HeaderRegexSanitizer {
    type Error = Infallible;
    type Iter = Once<(HeaderName, HeaderValue)>;
    fn as_headers(&self) -> Result<<Self as >::Iter, <Self as >::Error>;
}
impl Sanitizer for HeaderRegexSanitizer {
}
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeaderStringSanitizer {
    pub key: String,
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ApplyCondition>,
}
impl AsHeaders for HeaderStringSanitizer {
    type Error = Infallible;
    type Iter = Once<(HeaderName, HeaderValue)>;
    fn as_headers(&self) -> Result<<Self as >::Iter, <Self as >::Error>;
}
impl Sanitizer for HeaderStringSanitizer {
}
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuthResponseSanitizer;
impl AsHeaders for OAuthResponseSanitizer {
    type Error = Infallible;
    type Iter = Once<(HeaderName, HeaderValue)>;
    fn as_headers(&self) -> Result<<Self as >::Iter, <Self as >::Error>;
}
impl Sanitizer for OAuthResponseSanitizer {
}
#[derive(Debug)]
pub struct Recording {
}
impl Recording {
    async fn add_sanitizer<S>(&self, sanitizer: S) -> azure_core::Result<()> where S: Sanitizer, azure_core::Error: From<<S as AsHeaders>::Error>;
    fn credential(&self) -> Arc<dyn TokenCredential>;
    fn instrument(&self, options: &mut ClientOptions);
    fn instrument_perf(&self, options: &mut ClientOptions) -> azure_core::Result<()>;
    fn random<T>(&self) -> T where StandardUniform: Distribution<T>;
    fn random_string<const LEN: usize>(&self, prefix: Option<&str>) -> String;
    async fn remove_sanitizers(&self, sanitizers: &[&str]) -> azure_core::Result<()>;
    async fn set_matcher(&self, matcher: Matcher) -> azure_core::Result<()>;
    fn skip(&self, skip: Skip) -> azure_core::Result<SkipGuard<'_>>;
    fn test_mode(&self) -> TestMode;
    fn var<K>(&self, key: K, options: Option<VarOptions>) -> String where K: AsRef<str>;
    fn var_opt<K>(&self, key: K, options: Option<VarOptions>) -> Option<String> where K: AsRef<str>;
}
impl Drop for Recording {
    fn drop(&mut self);
}
impl Send for Recording {
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct RegexEntrySanitizer {
    pub target: RegexEntryValues,
    pub regex: String,
}
impl AsHeaders for RegexEntrySanitizer {
    type Error = Infallible;
    type Iter = Once<(HeaderName, HeaderValue)>;
    fn as_headers(&self) -> Result<<Self as >::Iter, <Self as >::Error>;
}
impl Sanitizer for RegexEntrySanitizer {
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct RemoveHeaderSanitizer {
    #[serde(rename = "headersForRemoval")]
    #[serde(serialize_with = "join")]
    pub headers_for_removal: Vec<&'static str>,
}
impl AsHeaders for RemoveHeaderSanitizer {
    type Error = Infallible;
    type Iter = Once<(HeaderName, HeaderValue)>;
    fn as_headers(&self) -> Result<<Self as >::Iter, <Self as >::Error>;
}
impl Sanitizer for RemoveHeaderSanitizer {
}
#[derive(Debug)]
pub struct RemoveRecording(pub bool);
impl Header for RemoveRecording {
    fn name(&self) -> HeaderName;
    fn value(&self) -> HeaderValue;
}
pub struct SkipGuard<'a>(/* private fields */);
impl Drop for SkipGuard<'_> {
    fn drop(&mut self);
}
#[derive(Debug)]
pub struct TestContext {
}
impl TestContext {
    fn crate_dir(&self) -> &'static Path;
    fn module_name(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn recording(&self) -> &Recording;
    fn repo_dir(&self) -> &'static Path;
    fn service_dir(&self) -> &'static str;
    fn test_data_dir(&self) -> PathBuf;
}
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UriRegexSanitizer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_for_replace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ApplyCondition>,
}
impl AsHeaders for UriRegexSanitizer {
    type Error = Infallible;
    type Iter = Once<(HeaderName, HeaderValue)>;
    fn as_headers(&self) -> Result<<Self as >::Iter, <Self as >::Error>;
}
impl Sanitizer for UriRegexSanitizer {
}
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UriStringSanitizer {
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ApplyCondition>,
}
impl AsHeaders for UriStringSanitizer {
    type Error = Infallible;
    type Iter = Once<(HeaderName, HeaderValue)>;
    fn as_headers(&self) -> Result<<Self as >::Iter, <Self as >::Error>;
}
impl Sanitizer for UriStringSanitizer {
}
#[derive(Clone, Debug, Default, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UriSubscriptionIdSanitizer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ApplyCondition>,
}
impl AsHeaders for UriSubscriptionIdSanitizer {
    type Error = Infallible;
    type Iter = Once<(HeaderName, HeaderValue)>;
    fn as_headers(&self) -> Result<<Self as >::Iter, <Self as >::Error>;
}
impl Sanitizer for UriSubscriptionIdSanitizer {
}
#[derive(Clone, Debug)]
pub struct VarOptions {
    pub default_value: Option<std::borrow::Cow<'static, str>>,
    pub sanitize: bool,
    pub sanitize_value: std::borrow::Cow<'static, str>,
}
impl Default for VarOptions {
    fn default() -> Self;
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
#[derive(Debug, serde::Serialize)]
pub enum Matcher {
    BodilessMatcher,
    HeaderlessMatcher,
    #[serde(untagged)]
    CustomDefaultMatcher(CustomDefaultMatcher),
}
impl AsHeaders for Matcher {
    type Error = Infallible;
    type Iter = Once<(HeaderName, HeaderValue)>;
    fn as_headers(&self) -> Result<<Self as >::Iter, <Self as >::Error>;
}
impl From<CustomDefaultMatcher> for Matcher {
    fn from(matcher: CustomDefaultMatcher) -> Self;
}
impl TryFrom<Matcher> for azure_core::Bytes {
    type Error = Error;
    fn try_from(matcher: Matcher) -> std::result::Result<Self, <Self as >::Error>;
}
#[derive(Clone, Copy, Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RegexEntryValues {
    Body,
    Header,
    Uri,
}
#[derive(Debug)]
pub enum Skip {
    RequestBody,
    RequestResponse,
}
impl Header for Skip {
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
pub trait Sanitizer: AsHeaders + fmt::Debug + Serialize {
}
pub const DEFAULT_IGNORED_HEADERS: &[&str; 6] = _;
pub const DEFAULT_SANITIZED_VALUE: &str = "Sanitized";
pub const DEFAULT_SANITIZERS_TO_REMOVE: &[&str; 2] = _;
pub const SANITIZE_BODY_ETAG: &str = "AZSDK3490";
pub const SANITIZE_BODY_ID: &str = "AZSDK3430";
pub const SANITIZE_BODY_NAME: &str = "AZSDK3493";
pub mod credentials {
    pub fn from_env(options: Option<azure_core::http::ClientOptions>) -> azure_core::Result<std::sync::Arc<dyn TokenCredential>>;
    #[derive(Clone, Debug, Default)]
    pub struct MockCredential;
    impl MockCredential {
        fn new() -> azure_core::Result<Arc<Self>>;
    }
    impl TokenCredential for MockCredential {
        #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
        fn get_token(&self, scopes: &[&str], __arg2: Option<TokenRequestOptions<'_>>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = azure_core::Result<AccessToken>> + ::core::marker::Send>>;
    }
}
pub mod http {
    pub struct MockHttpClient<C>(/* private fields */);
    impl<C> MockHttpClient<C> where C: FnMut(&azure_core::http::request::Request) -> futures::future::BoxFuture<'_, azure_core::Result<azure_core::http::AsyncRawResponse>> + Send + Sync {
        fn new(client: C) -> Self;
    }
    impl<C> Debug for MockHttpClient<C> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl<C> HttpClient for MockHttpClient<C> where C: FnMut(&azure_core::http::request::Request) -> futures::future::BoxFuture<'_, azure_core::Result<azure_core::http::AsyncRawResponse>> + Send + Sync {
        #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
        fn execute_request(&self, req: &Request) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<AsyncRawResponse>> + ::core::marker::Send>>;
    }
}
pub mod perf {
    #[derive(Clone, Debug, Default)]
    pub struct PeekSubcommandInfo {
        pub size: Option<i64>,
    }
    #[derive(Clone, Debug)]
    pub struct PerfRunner<T: PerfTestFactory> {
    }
    impl<T: PerfTestFactory> PerfRunner<T> {
        fn new(package_dir: &'static str, module_name: &'static str) -> std::result::Result<Self, clap::Error>;
        async fn run(&self) -> azure_core::Result<()>;
        async fn run_test_for(&self, test_instances: &[Arc<dyn PerfTest>], test_contexts: &[Arc<TestContext>], duration: Duration, track_latency: bool) -> azure_core::Result<(f64, Vec<tokio::time::Duration>)>;
        fn with_command_line(package_dir: &'static str, module_name: &'static str, args: Vec<&str>) -> std::result::Result<Self, clap::Error>;
    }
    #[async_trait]
    pub trait PerfTest: Send + Sync {
        #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn cleanup(&self, context: Arc<TestContext>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = azure_core::Result<()>> + ::core::marker::Send>>;
        #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn run(&self, context: Arc<TestContext>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = azure_core::Result<()>> + ::core::marker::Send>>;
        #[allow(elided_named_lifetimes, clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn setup(&self, context: Arc<TestContext>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = azure_core::Result<()>> + ::core::marker::Send>>;
    }
    pub trait PerfTestFactory: Subcommand + Clone + Debug {
        fn create_test(&self) -> CreatePerfTestReturn;
        fn name(&self) -> &'static str;
        fn peek(&self) -> PeekSubcommandInfo;
    }
    pub type CreatePerfTestReturn = std::pin::Pin<Box<dyn Future<Output = azure_core::Result<Box<dyn PerfTest>>>>>;
}
pub mod proxy {
    pub async fn start<impl AsRef<Path>: AsRef<Path>>(test_mode: Option<TestMode>, crate_dir: impl AsRef<Path>, options: Option<ProxyOptions>) -> Result<Proxy>;
    #[derive(Debug, Default)]
    pub struct Proxy {
    }
    impl Proxy {
        async fn stop(&mut self) -> Result<()>;
        async fn wait(&mut self) -> Result<ExitStatus>;
    }
    impl Proxy {
        fn endpoint(&self) -> &Url;
        fn existing() -> Result<Self>;
    }
    impl Drop for Proxy {
        fn drop(&mut self);
    }
    #[derive(Clone, Debug)]
    pub struct ProxyOptions {
        pub auto: bool,
        pub insecure: bool,
        pub auto_shutdown_in_seconds: u32,
    }
    impl Default for ProxyOptions {
        fn default() -> Self;
    }
    #[derive(Clone, Debug)]
    pub struct RecordingId(/* private fields */);
    impl AsRef<str> for RecordingId {
        fn as_ref(&self) -> &str;
    }
    impl FromStr for RecordingId {
        type Err = Infallible;
        fn from_str(value: &str) -> std::result::Result<Self, <Self as >::Err>;
    }
    impl Header for &RecordingId {
        fn name(&self) -> HeaderName;
        fn value(&self) -> HeaderValue;
    }
    impl Header for RecordingId {
        fn name(&self) -> HeaderName;
        fn value(&self) -> HeaderValue;
    }
}
pub mod recorded {
    #[proc_macro_attribute]
    #[test]
    pub async fn start(mode: azure_core::test::TestMode, crate_dir: &'static str, module_dir: &'static str, name: &'static str, options: Option<crate::proxy::ProxyOptions>) -> azure_core::Result<crate::TestContext>;
}
pub mod stream {
    #[derive(Clone)]
    pub struct GeneratedStream<I, const LENGTH: usize, const CHUNK: usize = 1024> {
    }
    impl<I, const LENGTH: usize, const CHUNK: usize> GeneratedStream<I, LENGTH, CHUNK> where I: Iterator<Item = u8> + Clone {
        #[allow(clippy::should_implement_trait)]
        fn from_iter(iter: I) -> Self;
    }
    impl<const LENGTH: usize, const CHUNK: usize> GeneratedStream<std::ops::Range<u8>, LENGTH, CHUNK> {
        fn new() -> GeneratedStream<Range<u8>, LENGTH, CHUNK>;
    }
    impl<I, const LENGTH: usize, const CHUNK: usize> AsyncRead for GeneratedStream<I, LENGTH, CHUNK> where I: Clone, std::iter::Cycle<I>: Iterator<Item = u8> + Unpin {
        fn poll_read(self: Pin<&mut Self>, _cx: &mut std::task::Context<'_>, buf: &mut [u8]) -> Poll<std::io::Result<usize>>;
    }
    impl<I, const LENGTH: usize, const CHUNK: usize> Debug for GeneratedStream<I, LENGTH, CHUNK> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl<I, const LENGTH: usize, const CHUNK: usize> From<&GeneratedStream<I, LENGTH, CHUNK>> for azure_core::http::Body where for<'a> I: Clone + Send + Sync + 'a, std::iter::Cycle<I>: Iterator<Item = u8> + Unpin {
        fn from(stream: &GeneratedStream<I, LENGTH, CHUNK>) -> Self;
    }
    impl<I, const LENGTH: usize, const CHUNK: usize> From<&GeneratedStream<I, LENGTH, CHUNK>> for azure_core::http::RequestContent<azure_core::Bytes, azure_core::http::NoFormat> where for<'a> I: Clone + Send + Sync + 'a, std::iter::Cycle<I>: Iterator<Item = u8> + Unpin {
        fn from(stream: &GeneratedStream<I, LENGTH, CHUNK>) -> Self;
    }
    impl<I, const LENGTH: usize, const CHUNK: usize> From<GeneratedStream<I, LENGTH, CHUNK>> for azure_core::http::Body where for<'a> I: Clone + Send + Sync + 'a, std::iter::Cycle<I>: Iterator<Item = u8> + Unpin {
        fn from(stream: GeneratedStream<I, LENGTH, CHUNK>) -> Self;
    }
    impl<I, const LENGTH: usize, const CHUNK: usize> From<GeneratedStream<I, LENGTH, CHUNK>> for azure_core::http::RequestContent<azure_core::Bytes, azure_core::http::NoFormat> where for<'a> I: Clone + Send + Sync + 'a, std::iter::Cycle<I>: Iterator<Item = u8> + Unpin {
        fn from(stream: GeneratedStream<I, LENGTH, CHUNK>) -> Self;
    }
    impl<I, const LENGTH: usize, const CHUNK: usize> SeekableStream for GeneratedStream<I, LENGTH, CHUNK> where I: Clone + Send + Sync, std::iter::Cycle<I>: Iterator<Item = u8> + Unpin {
        fn len(&self) -> Option<u64>;
        #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
        fn reset(&mut self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = azure_core::Result<()>> + ::core::marker::Send>>;
    }
    impl<I, const LENGTH: usize, const CHUNK: usize> Stream for GeneratedStream<I, LENGTH, CHUNK> where I: Clone, std::iter::Cycle<I>: Iterator<Item = u8> + Unpin {
        type Item = Result<Vec<u8>, Error>;
        fn poll_next(self: Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> Poll<Option<<Self as >::Item>>;
    }
    impl<const LENGTH: usize, const CHUNK: usize> Default for GeneratedStream<std::ops::Range<u8>, LENGTH, CHUNK> {
        fn default() -> Self;
    }
}
pub mod tracing {
    pub async fn assert_instrumentation_information<C, FnInit, FnTest, T>(create_client: FnInit, test_api: FnTest, api_information: ExpectedInstrumentation) -> azure_core::Result<()> where FnInit: FnOnce(std::sync::Arc<dyn TracerProvider>) -> azure_core::Result<C>, FnTest: AsyncFnOnce(C) -> azure_core::Result<T>;
    pub fn check_instrumentation_result(mock_tracer: std::sync::Arc<MockTracingProvider>, expected_tracers: Vec<ExpectedTracerInformation<'_>>);
    #[derive(Clone, Debug)]
    pub struct ExpectedApiInformation {
        pub api_name: Option<&'static str>,
        pub api_children: Vec<ExpectedRestApiSpan>,
        pub additional_api_attributes: Vec<(&'static str, azure_core::tracing::AttributeValue)>,
    }
    impl Default for ExpectedApiInformation {
        fn default() -> Self;
    }
    #[derive(Clone, Debug, Default)]
    pub struct ExpectedInstrumentation {
        pub package_name: String,
        pub package_version: String,
        pub package_namespace: Option<&'static str>,
        pub api_calls: Vec<ExpectedApiInformation>,
    }
    #[derive(Clone, Debug)]
    pub struct ExpectedRestApiSpan {
        pub api_verb: azure_core::http::Method,
        pub expected_status_code: azure_core::http::StatusCode,
        pub is_wildcard: bool,
    }
    impl Default for ExpectedRestApiSpan {
        fn default() -> Self;
    }
    #[derive(Debug)]
    pub struct ExpectedSpanInformation<'a> {
        pub span_name: &'a str,
        pub status: azure_core::tracing::SpanStatus,
        pub span_id: azure_core::Uuid,
        pub parent_id: Option<azure_core::Uuid>,
        pub kind: azure_core::tracing::SpanKind,
        pub attributes: Vec<(&'a str, azure_core::tracing::AttributeValue)>,
        pub is_wildcard: bool,
    }
    impl Default for ExpectedSpanInformation<'_> {
        fn default() -> Self;
    }
    #[derive(Debug)]
    pub struct ExpectedTracerInformation<'a> {
        pub name: &'a str,
        pub version: Option<&'a str>,
        pub namespace: Option<&'a str>,
        pub spans: Vec<ExpectedSpanInformation<'a>>,
    }
    pub struct MockSpan {
    }
    impl AsAny for MockSpan {
        fn as_any(&self) -> &dyn std::any::Any;
    }
    impl Drop for MockSpan {
        fn drop(&mut self);
    }
    impl Span for MockSpan {
        fn end(&self);
        fn is_recording(&self) -> bool;
        fn propagate_headers(&self, request: &mut Request);
        fn record_error(&self, error: &dyn std::error::Error);
        fn set_attribute(&self, key: &'static str, value: AttributeValue);
        fn set_current(&self, context: &Context<'_>) -> Box<dyn SpanGuard>;
        fn set_status(&self, status: azure_core::tracing::SpanStatus);
        fn span_id(&self) -> [u8; 8];
    }
    #[derive(Debug)]
    pub struct MockTracer {
    }
    impl Tracer for MockTracer {
        fn namespace(&self) -> Option<&'static str>;
        fn start_span(&self, name: Cow<'static, str>, kind: SpanKind, attributes: Vec<Attribute>) -> Arc<dyn Span>;
        fn start_span_with_parent(&self, name: Cow<'static, str>, kind: SpanKind, attributes: Vec<Attribute>, parent: Arc<dyn crate::tracing::Span>) -> Arc<dyn crate::tracing::Span>;
    }
    #[derive(Debug)]
    pub struct MockTracingProvider {
    }
    impl MockTracingProvider {
        fn new() -> Self;
    }
    impl Default for MockTracingProvider {
        fn default() -> Self;
    }
    impl TracerProvider for MockTracingProvider {
        fn get_tracer(&self, azure_namespace: Option<&'static str>, crate_name: &'static str, crate_version: Option<&'static str>) -> Arc<dyn crate::tracing::Tracer>;
    }
}
```
