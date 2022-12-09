//! The type of approach to apply when calculating the delay

/// The type of approach to apply when calculating the delay
/// between retry attempts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ServiceBusRetryMode {
    /// Retry attempts happen at fixed intervals; each delay is a consistent duration.
    Fixed,

    /// Retry attempts will delay based on a backoff strategy, where each attempt will increase the duration that it waits before retrying.
    Exponential,
}

impl Default for ServiceBusRetryMode {
    fn default() -> Self {
        Self::Exponential
    }
}
