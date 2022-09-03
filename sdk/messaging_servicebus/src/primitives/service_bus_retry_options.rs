use std::time::Duration;

use crate::receiver::service_bus_receive_mode::ServiceBusReceiveMode;

use super::service_bus_retry_policy::ServiceBusRetryPolicy;

const MAX_RETRIES: u8 = 100;
const DEFAULT_MAX_RETRIES: u8 = 3;
const DEFAULT_DELAY: Duration = Duration::from_millis(800);
const DEFAULT_MAX_DELAY: Duration = Duration::from_secs(1 * 60);
const DEFAULT_TRY_TIMEOUT: Duration = Duration::from_secs(1 * 60);

pub struct ServiceBusRetryOptions<P>
where
    P: ServiceBusRetryPolicy,
{
    /// <summary>
    ///   The approach to use for calculating retry delays.
    /// </summary>
    /// <value>The default retry mode is <see cref="ServiceBusRetryMode.Exponential"/>.</value>
    mode: ServiceBusReceiveMode,

    /// The maximum number of retry attempts before considering the associated operation to have failed.
    max_retries: u8,

    /// The delay or backoff factor to apply between retry attempts.
    delay: Duration,

    /// The maximum delay to allow between retry attempts.
    max_delay: Duration,

    /// The maximum duration to wait for an operation, per attempt.
    try_timeout: Duration,

    /// <summary>
    ///   A custom retry policy to be used in place of the individual option values.
    /// </summary>
    ///
    /// <remarks>
    ///   When populated, this custom policy will take precedence over the individual retry
    ///   options provided.
    /// </remarks>
    policy: P,
}

impl<P> ServiceBusRetryOptions<P>
where
    P: ServiceBusRetryPolicy,
{
    pub fn mode(&self) -> &ServiceBusReceiveMode {
        todo!()
    }

    pub fn set_mode(&mut self, mode: ServiceBusReceiveMode) {
        todo!()
    }

    pub fn max_retries(&self) -> u8 {
        self.max_retries
    }

    pub fn set_max_retries(&mut self, value: u8) {
        let value = u8::min(MAX_RETRIES, value);
        self.max_retries = value;
    }

    pub fn delay(&self) -> &Duration {
        todo!()
    }

    pub fn set_delay(&mut self, value: Duration) {
        todo!()
    }

    pub fn max_delay(&self) -> &Duration {
        todo!()
    }

    pub fn set_max_delay(&mut self, value: Duration) {
        todo!()
    }

    pub fn try_timeout(&self) -> &Duration {
        todo!()
    }

    pub fn set_try_timeout(&mut self, value: Duration) {
        todo!()
    }

    pub fn policy(&self) -> &P {
        todo!()
    }

    pub fn set_policy(&mut self, policy: P) {
        todo!()
    }

    pub fn set_custom_policy<Q>(self, policy: Q) -> ServiceBusRetryOptions<Q>
    where
        Q: ServiceBusRetryPolicy,
    {
        todo!()
    }
}
