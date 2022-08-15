use crate::http_client;
use crate::policies::{ExponentialRetryPolicy, FixedRetryPolicy, NoRetryPolicy, Policy};
use crate::HttpClient;
use std::sync::Arc;
use std::time::Duration;

/// Client options allow customization of policies, retry options, and more.
///
/// # Examples
///
/// You can override default options and even add your own per-call or per-retry policies:
///
/// ```
/// use azure_core::{ClientOptions, RetryOptions, TelemetryOptions};
/// let options: ClientOptions = ClientOptions::default()
///     .retry(RetryOptions::default().max_retries(10u32))
///     .telemetry(TelemetryOptions::default().application_id("my-application"));
/// ```
#[derive(Clone, Debug, Default)]
pub struct ClientOptions {
    /// Policies called per call.
    pub(crate) per_call_policies: Vec<Arc<dyn Policy>>,
    /// Policies called per retry.
    pub(crate) per_retry_policies: Vec<Arc<dyn Policy>>,
    /// Retry options.
    pub(crate) retry: RetryOptions,
    /// Telemetry options.
    pub(crate) telemetry: TelemetryOptions,
    /// Transport options.
    pub(crate) transport: TransportOptions,
}

impl ClientOptions {
    pub fn new(transport: TransportOptions) -> Self {
        Self {
            per_call_policies: Vec::new(),
            per_retry_policies: Vec::new(),
            retry: RetryOptions::default(),
            telemetry: TelemetryOptions::default(),
            transport,
        }
    }

    /// A mutable reference to per-call policies.
    pub fn per_call_policies_mut(&mut self) -> &mut Vec<Arc<dyn Policy>> {
        &mut self.per_call_policies
    }

    /// A mutable reference to per-retry policies.
    pub fn per_retry_policies_mut(&mut self) -> &mut Vec<Arc<dyn Policy>> {
        &mut self.per_retry_policies
    }

    setters! {
        per_call_policies: Vec<Arc<dyn Policy>> => per_call_policies,
        per_retry_policies: Vec<Arc<dyn Policy>> => per_retry_policies,
        retry: RetryOptions => retry,
        telemetry: TelemetryOptions => telemetry,
        transport: TransportOptions => transport,
    }
}

/// The algorithm to apply when calculating the delay between retry attempts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RetryMode {
    /// Retry attempts will delay based on a back-off strategy,
    /// where each attempt will increase the duration that it waits before retrying.
    ///
    /// This is the default.
    Exponential,

    /// Retry attempts happen at fixed intervals; each delay is a consistent duration.
    Fixed,

    /// Do not retry attempts.
    None,
}

impl Default for RetryMode {
    fn default() -> Self {
        RetryMode::Exponential
    }
}

/// Specify how retries should behave.
///
/// Note that not all requests can be retried. These options will only be used
/// when a retry is attempted.
#[derive(Clone, Debug)]
pub struct RetryOptions {
    /// The algorithm to use for calculating retry delays.
    ///
    /// The default is `RetryMode::Exponential`
    pub mode: RetryMode,

    /// The delay between retry attempts for a fixed algorithm
    /// or the delay on which to base calculations for a back-off-based approach.
    ///
    /// The default is 200 milliseconds.
    pub delay: Duration,

    /// The maximum number of retry attempts before giving up.
    ///
    /// The default is 8.
    pub max_retries: u32,

    /// The maximum permissible elapsed time since starting to retry.
    ///
    /// The default is 1 minute.
    pub max_elapsed: Duration,

    /// The maximum permissible time between retries.
    ///
    /// The default is 30 seconds. For SRE reasons, this is only respected when above 1 second.
    /// This option is ignored when using retry modes that do not change their delay time.
    pub max_delay: Duration,
}

impl RetryOptions {
    setters! {
        mode: RetryMode => mode,
        delay: Duration => delay,
        max_retries: u32 => max_retries,
        max_elapsed: Duration => max_elapsed,
        max_delay: Duration => max_delay,
    }
}

impl Default for RetryOptions {
    fn default() -> Self {
        RetryOptions {
            mode: RetryMode::default(),
            delay: Duration::from_millis(200),
            max_retries: 8,
            max_elapsed: Duration::from_secs(60),
            max_delay: Duration::from_secs(30),
        }
    }
}

impl RetryOptions {
    pub(crate) fn to_policy(&self) -> Arc<dyn Policy> {
        match self.mode {
            RetryMode::Exponential => Arc::new(ExponentialRetryPolicy::new(
                self.delay,
                self.max_retries,
                self.max_elapsed,
                self.max_delay,
            )),
            RetryMode::Fixed => Arc::new(FixedRetryPolicy::new(
                self.delay,
                self.max_retries,
                self.max_elapsed,
            )),
            RetryMode::None => Arc::new(NoRetryPolicy::default()),
        }
    }
}

/// Telemetry options.
#[derive(Clone, Debug, Default)]
pub struct TelemetryOptions {
    /// Optional application ID to telemeter.
    pub(crate) application_id: Option<String>,
}

impl TelemetryOptions {
    setters! {
        application_id: String => Some(application_id),
    }
}

/// Transport options.
#[derive(Clone, Debug)]
pub struct TransportOptions {
    inner: TransportOptionsImpl,
}

#[derive(Clone, Debug)]
enum TransportOptionsImpl {
    Http {
        /// The HTTP client implementation to use for requests.
        http_client: Arc<dyn HttpClient>,
    },
    Custom(Arc<dyn Policy>),
}

impl TransportOptions {
    /// Creates a new `TransportOptions` using the given `HttpClient`.
    pub fn new(http_client: Arc<dyn HttpClient>) -> Self {
        let inner = TransportOptionsImpl::Http { http_client };
        Self { inner }
    }

    /// Creates a new `TransportOptions` using the custom policy.
    ///
    /// This policy is expected to be the last policy in the pipeline.
    pub fn new_custom_policy(policy: Arc<dyn Policy>) -> Self {
        let inner = TransportOptionsImpl::Custom(policy);
        Self { inner }
    }

    /// Use these options to send a request.
    pub async fn send(
        &self,
        ctx: &crate::Context,
        request: &mut crate::Request,
    ) -> crate::Result<crate::Response> {
        use TransportOptionsImpl as I;
        match &self.inner {
            I::Http { http_client } => http_client.execute_request(request).await,
            I::Custom(s) => s.send(ctx, request, &[]).await,
        }
    }
}

impl Default for TransportOptions {
    /// Creates an instance of the `TransportOptions` using the default `HttpClient`.
    fn default() -> Self {
        Self::new(http_client::new_http_client())
    }
}
