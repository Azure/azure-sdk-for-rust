use crate::policies::{ExponentialRetryPolicy, FixedRetryPolicy, NoRetryPolicy, Policy};
use crate::{new_http_client, HttpClient};
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
/// let options: ClientOptions<()> = ClientOptions::default()
///     .retry(RetryOptions::default().max_retries(10u32))
///     .telemetry(TelemetryOptions::default().application_id("my-application"));
/// ```
#[derive(Clone, Debug)]
pub struct ClientOptions<C>
where
    C: Send + Sync,
{
    // TODO: Expose transport override.
    /// Policies called per call.
    pub(crate) per_call_policies: Vec<Arc<dyn Policy<C>>>,

    /// Policies called per retry.
    pub(crate) per_retry_policies: Vec<Arc<dyn Policy<C>>>,

    /// Retry options.
    pub(crate) retry: RetryOptions,

    /// Telemetry options.
    pub(crate) telemetry: TelemetryOptions,

    /// Transport options.
    pub(crate) transport: TransportOptions,
}

#[cfg(not(feature = "mock_transport_framework"))]
impl<C> Default for ClientOptions<C>
where
    C: Send + Sync,
{
    fn default() -> Self {
        Self {
            per_call_policies: Vec::new(),
            per_retry_policies: Vec::new(),
            retry: RetryOptions::default(),
            telemetry: TelemetryOptions::default(),
            transport: TransportOptions::default(),
        }
    }
}

impl<C> ClientOptions<C>
where
    C: Send + Sync,
{
    pub fn new(#[cfg(feature = "mock_transport_framework")] transaction_name: String) -> Self {
        Self {
            per_call_policies: Vec::new(),
            per_retry_policies: Vec::new(),
            retry: RetryOptions::default(),
            telemetry: TelemetryOptions::default(),
            #[cfg(not(feature = "mock_transport_framework"))]
            transport: TransportOptions::default(),
            #[cfg(feature = "mock_transport_framework")]
            transport: TransportOptions::new(transaction_name),
        }
    }

    /// A mutable reference to per-call policies.
    pub fn per_call_policies_mut(&mut self) -> &mut Vec<Arc<dyn Policy<C>>> {
        &mut self.per_call_policies
    }

    /// A mutable reference to per-retry policies.
    pub fn per_retry_policies_mut(&mut self) -> &mut Vec<Arc<dyn Policy<C>>> {
        &mut self.per_retry_policies
    }

    setters! {
        per_call_policies: Vec<Arc<dyn Policy<C>>> => per_call_policies,
        per_retry_policies: Vec<Arc<dyn Policy<C>>> => per_retry_policies,
        retry: RetryOptions => retry,
        telemetry: TelemetryOptions => telemetry,
        transport: TransportOptions => transport,
    }
}

/// The algorithm to apply when calculating the delay between retry attempts.
#[derive(Clone, Debug, PartialEq)]
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

/// The set of options that can be specified to influence how retry attempts are made,
/// and a failure is eligible to be retried.
#[derive(Clone, Debug)]
pub struct RetryOptions {
    /// The algorithm to use for calculating retry delays.
    mode: RetryMode,

    /// The delay between retry attempts for a fixed algorithm
    /// or the delay on which to base calculations for a back-off-based approach.
    ///
    /// The default is 800 milliseconds.
    delay: Duration,

    /// The maximum number of retry attempts before giving up.
    ///
    /// The default is 3.
    max_retries: u32,

    /// The maximum permissible delay between retry attempts.
    ///
    /// The default is 1 minute.
    max_delay: Duration,
}

impl RetryOptions {
    setters! {
        mode: RetryMode => mode,
        delay: Duration => delay,
        max_retries: u32 => max_retries,
        max_delay: Duration => max_delay,
    }
}

impl Default for RetryOptions {
    fn default() -> Self {
        RetryOptions {
            mode: RetryMode::default(),
            delay: Duration::from_millis(800),
            max_retries: 3,
            max_delay: Duration::from_secs(60),
        }
    }
}

impl RetryOptions {
    pub(crate) fn to_policy<C: Send + Sync>(&self) -> Arc<dyn Policy<C>> {
        match self.mode {
            RetryMode::Exponential => Arc::new(ExponentialRetryPolicy::new(
                self.delay,
                self.max_retries,
                self.max_delay,
            )),
            RetryMode::Fixed => Arc::new(FixedRetryPolicy::new(
                self.delay,
                self.max_retries,
                self.max_delay,
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

#[cfg(not(feature = "mock_transport_framework"))]
mod transport_options {
    use super::*;

    /// Transport options.
    #[derive(Clone, Debug)]
    pub struct TransportOptions {
        /// The HTTP client implementation to use for requests.
        pub(crate) http_client: Arc<dyn HttpClient>,
    }

    impl TransportOptions {
        /// Creates a new `TransportOptions` using the given `HttpClient`.
        pub fn new(http_client: Arc<dyn HttpClient>) -> Self {
            Self { http_client }
        }
    }

    impl Default for TransportOptions {
        /// Creates an instance of the `TransportOptions` using the default `HttpClient`.
        fn default() -> Self {
            Self::new(new_http_client())
        }
    }
}

#[cfg(feature = "mock_transport_framework")]
mod transport_options {
    use super::*;

    /// Transport options.
    #[derive(Clone, Debug)]
    pub struct TransportOptions {
        /// The HTTP client implementation to use for requests.
        pub(crate) http_client: Arc<dyn HttpClient>,
        /// The name of the transaction used when reading or writing mock requests and responses.
        pub(crate) transaction_name: String,
    }

    impl TransportOptions {
        pub fn new(transaction_name: String) -> Self {
            Self {
                transaction_name,
                http_client: new_http_client(),
            }
        }
    }
}

pub use transport_options::*;
