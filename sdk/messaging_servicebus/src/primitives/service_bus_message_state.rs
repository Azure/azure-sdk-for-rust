//! Defines the [`ServiceBusMessageState`] enum.

// Conditional import for docs.rs
#[cfg(docsrs)]
use crate::ServiceBusReceivedMessage;

/// Represents the message state of the [`ServiceBusReceivedMessage`]
#[derive(Debug)]
pub enum ServiceBusMessageState {
    /// Specifies an active message state.
    Active = 0,

    /// Specifies a deferred message state.
    Deferred = 1,

    /// Specifies the scheduled message state.
    Scheduled = 2,
}

// azservicebus.message.go #L399
impl Default for ServiceBusMessageState {
    fn default() -> Self {
        Self::Active
    }
}

impl From<i64> for ServiceBusMessageState {
    fn from(value: i64) -> Self {
        match value {
            1 => ServiceBusMessageState::Deferred,
            2 => ServiceBusMessageState::Scheduled,
            _ => ServiceBusMessageState::Active,
        }
    }
}

impl From<ServiceBusMessageState> for i64 {
    fn from(value: ServiceBusMessageState) -> Self {
        value as i64
    }
}
