/// The type of approach to apply when calculating the delay
/// between retry attempts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventHubsRetryMode {
    /// Retry attempts happen at fixed intervals; each delay is a consistent duration
    Fixed,

    /// Retry attempts will delay based on a back-off strategy, where each attempt will increase the duration that it waits before retrying
    Exponential,
}

impl Default for EventHubsRetryMode {
    fn default() -> Self {
        Self::Exponential
    }
}
