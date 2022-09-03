/// <summary>
///   The type of approach to apply when calculating the delay
///   between retry attempts.
/// </summary>
pub enum ServiceBusRetryMode {
    /// <summary>Retry attempts happen at fixed intervals; each delay is a consistent duration.</summary>
    Fixed,

    /// <summary>Retry attempts will delay based on a backoff strategy, where each attempt will increase the duration that it waits before retrying.</summary>
    Exponential,
}

impl Default for ServiceBusRetryMode {
    fn default() -> Self {
        Self::Exponential
    }
}
