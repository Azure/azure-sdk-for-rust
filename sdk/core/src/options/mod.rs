use crate::policies::{ExponentialRetryPolicy, FixedRetryPolicy, NoRetryPolicy, Policy};
use crate::{http_client, TimeoutPolicy};
use crate::{HttpClient, RetryPolicy};
use std::fmt::Debug;
use std::sync::Arc;
use std::time::Duration;

/// Client options allow customization of policies, retry options, and more.
///
/// # Examples
///
/// You can override default options and even add your own per-call or per-retry policies:
///
/// ```
/// use azure_core::{ClientOptions, ExponentialRetryOptions, RetryOptions, TelemetryOptions};
/// let options: ClientOptions = ClientOptions::default()
///     .retry(RetryOptions::exponential(ExponentialRetryOptions::default().max_retries(10u32)))
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
    /// Transport options.
    pub timeout: TimeoutPolicy,
}

impl ClientOptions {
    pub fn new(transport: TransportOptions) -> Self {
        Self {
            per_call_policies: Vec::new(),
            per_retry_policies: Vec::new(),
            retry: RetryOptions::default(),
            telemetry: TelemetryOptions::default(),
            transport,
            timeout: TimeoutPolicy::default(),
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
        timeout: TimeoutPolicy => timeout,
    }
}

/// The algorithm to apply when calculating the delay between retry attempts.
#[derive(Clone)]
enum RetryMode {
    /// Retry attempts will delay based on a back-off strategy,
    /// where each attempt will increase the duration that it waits before retrying.
    ///
    /// This is the default.
    Exponential(ExponentialRetryOptions),

    /// Retry attempts happen at fixed intervals; each delay is a consistent duration.
    Fixed(FixedRetryOptions),

    /// A custom retry policy
    Custom(Arc<dyn Policy>),

    /// Do not retry attempts.
    None,
}

impl Debug for RetryMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RetryMode::Exponential(o) => write!(f, "Exponetial({o:?})"),
            RetryMode::Fixed(o) => write!(f, "Fixed({o:?})"),
            RetryMode::Custom(_) => write!(f, "Custom"),
            RetryMode::None => write!(f, "None"),
        }
    }
}

impl Default for RetryMode {
    fn default() -> Self {
        RetryMode::Exponential(ExponentialRetryOptions::default())
    }
}

/// Specify how retries should behave.
///
/// Note that not all requests can be retried. These options will only be used
/// when a retry is attempted.
///
/// The default is an exponential retry policy using the default `ExponentialRetryOptions`.
#[derive(Clone, Debug, Default)]
pub struct RetryOptions {
    /// The algorithm to use for calculating retry delays.
    mode: RetryMode,
}

impl RetryOptions {
    /// A retry strategy where attempts happen at intervals that get exponentially longer with each retry.
    pub fn exponential(options: ExponentialRetryOptions) -> Self {
        Self {
            mode: RetryMode::Exponential(options),
        }
    }

    /// A retry strategy where attempts happen at fixed intervals; each delay is a consistent duration.
    pub fn fixed(options: FixedRetryOptions) -> Self {
        Self {
            mode: RetryMode::Fixed(options),
        }
    }

    /// A custom retry using the supplied retry policy.
    pub fn custom<T: RetryPolicy + 'static>(policy: Arc<T>) -> Self {
        Self {
            mode: RetryMode::Custom(policy),
        }
    }

    /// No retries will be attempted.
    pub fn none() -> Self {
        Self {
            mode: RetryMode::None,
        }
    }

    pub(crate) fn to_policy(&self) -> Arc<dyn Policy> {
        match &self.mode {
            RetryMode::Exponential(options) => Arc::new(ExponentialRetryPolicy::new(
                options.initial_delay,
                options.max_retries,
                options.max_total_elapsed,
                options.max_delay,
            )),
            RetryMode::Fixed(options) => Arc::new(FixedRetryPolicy::new(
                options.delay,
                options.max_retries,
                options.max_total_elapsed,
            )),
            RetryMode::Custom(c) => c.clone(),
            RetryMode::None => Arc::new(NoRetryPolicy::default()),
        }
    }
}

/// Options for how an exponential retry strategy should behave.
///
/// # Example
///
/// Configuring retry to be exponential with 10 retries max and an initial delay of 1 second.
/// ```
/// # use core::time::Duration; use azure_core::RetryOptions; use azure_core::ExponentialRetryOptions;
/// RetryOptions::exponential(
///    ExponentialRetryOptions::default()
///        .max_retries(10u32)
///        .initial_delay(Duration::from_secs(1)),
/// );
/// ```
#[derive(Clone, Debug)]
pub struct ExponentialRetryOptions {
    /// The initial delay between retry attempts. The delay will increase with each retry.
    ///
    /// The default is 200 milliseconds.
    pub initial_delay: Duration,

    /// The maximum number of retry attempts before giving up.
    ///
    /// The default is 8.
    pub max_retries: u32,

    /// The maximum permissible elapsed time since starting to retry before giving up.
    ///
    /// The default is 1 minute.
    pub max_total_elapsed: Duration,

    /// The maximum permissible time between retries.
    ///
    /// The default is 30 seconds. For SRE reasons, this is only respected when above 1 second.
    pub max_delay: Duration,
}

impl ExponentialRetryOptions {
    setters! {
        initial_delay: Duration => initial_delay,
        max_retries: u32 => max_retries,
        max_total_elapsed: Duration => max_total_elapsed,
        max_delay: Duration => max_delay,
    }
}

impl Default for ExponentialRetryOptions {
    fn default() -> Self {
        Self {
            initial_delay: Duration::from_millis(200),
            max_retries: 8,
            max_total_elapsed: Duration::from_secs(60),
            max_delay: Duration::from_secs(30),
        }
    }
}

/// Options for how a fixed retry strategy should behave.
///
/// # Example
///
/// Configuring retry to be fixed with 10 retries max.
/// ```
/// # use azure_core::RetryOptions; use azure_core::FixedRetryOptions;
/// RetryOptions::fixed(
///    FixedRetryOptions::default()
///        .max_retries(10u32)
/// );
/// ```
#[derive(Clone, Debug)]
pub struct FixedRetryOptions {
    /// The delay between retry attempts.
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
    pub max_total_elapsed: Duration,
}

impl FixedRetryOptions {
    setters! {
        #[doc = "Set the delay between retry attempts."]
        delay: Duration => delay,
        #[doc = "Set the maximum number of retry attempts before giving up."]
        max_retries: u32 => max_retries,
        #[doc = "Set the maximum permissible elapsed time since starting to retry."]
        max_total_elapsed: Duration => max_total_elapsed,
    }
}

impl Default for FixedRetryOptions {
    fn default() -> Self {
        Self {
            delay: Duration::from_millis(200),
            max_retries: 8,
            max_total_elapsed: Duration::from_secs(60),
        }
    }
}

/// Telemetry options.
#[derive(Clone, Debug, Default)]
pub struct TelemetryOptions {
    /// Optional application ID to telemetry.
    pub(crate) application_id: Option<String>,
}

impl TelemetryOptions {
    setters! {
        #[doc = "Set the application ID to telemetry."]
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
