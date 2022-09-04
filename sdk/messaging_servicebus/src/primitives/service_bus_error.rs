use std::fmt::Display;

use crate::processor::processor_error_source::ServiceBusErrorSource;

use super::service_bus_failure_reason::ServiceBusFailureReason;

/// Serves as a basis for exceptions produced within the Service Bus
/// context.
#[derive(Debug)]
pub struct ServiceBusError {
    is_transient: bool,
    message: String,
    reason: ServiceBusFailureReason,
    entity_path: Option<String>,
    pub(crate) processor_error_source: Option<ServiceBusErrorSource>,
}

impl ServiceBusError {
    /// Indicates whether an exception should be considered transient or final.
    ///
    /// # Value
    ///
    /// `true` if the exception is likely transient; otherwise, `false`.
    pub fn is_transient(&self) -> bool {
        self.is_transient
    }

    /// The reason for the failure of an Service Bus operation that resulted in the exception.
    pub fn reason(&self) -> &ServiceBusFailureReason {
        &self.reason
    }

    /// The name of the Service Bus to which the exception is associated.
    ///
    /// # Value
    ///
    /// The name of the Service Bus entity, if available; otherwise, `None`.
    pub fn entity_path(&self) -> Option<&str> {
        self.entity_path.as_ref().map(|s| s.as_str())
    }

    pub(crate) fn new(message: impl Into<String>, reason: ServiceBusFailureReason) -> Self {
        let is_transient = match &reason {
            ServiceBusFailureReason::ServiceCommunicationProblem
            | ServiceBusFailureReason::ServiceTimeout
            | ServiceBusFailureReason::ServiceBusy => true,
            _ => false,
        };

        Self {
            is_transient,
            message: message.into(),
            reason,
            entity_path: None,
            processor_error_source: None,
        }
    }

    pub(crate) fn with_entity_path(
        message: impl Into<String>,
        reason: ServiceBusFailureReason,
        entity_path: Option<impl Into<String>>,
    ) -> Self {
        Self {
            entity_path: entity_path.map(Into::into),
            ..Self::new(message, reason)
        }
    }
}

impl Display for ServiceBusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.entity_path {
            Some(entity_path) => write!(
                f,
                "{:#?} ({:#?} - {:#?})",
                self.message, entity_path, self.reason
            ),
            None => {
                write!(f, "{:#?} ({:#?})", self.message, self.reason)
            }
        }
    }
}

impl std::error::Error for ServiceBusError {}
