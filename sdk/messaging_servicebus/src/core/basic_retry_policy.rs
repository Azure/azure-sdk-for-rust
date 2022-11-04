use std::fmt::Display;

use crate::primitives::service_bus_retry_policy::{
    ServiceBusRetryPolicy, ServiceBusRetryPolicyState,
};

/// <summary>
///   The default retry policy for the Service Bus client library, respecting the
///   configuration specified as a set of <see cref="ServiceBusRetryOptions" />.
/// </summary>
///
/// <seealso cref="ServiceBusRetryOptions"/>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct BasicRetryPolicy {}

#[derive(Debug, thiserror::Error)]
pub enum BasicRetryPolicyError {}

pub struct BasicRetryPolicyState {}

impl ServiceBusRetryPolicyState for BasicRetryPolicyState {
    fn is_server_busy(&self) -> bool {
        todo!()
    }

    fn is_server_busy_mut(&mut self) -> &mut bool {
        todo!()
    }

    fn server_busy_error_message(&self) -> &String {
        todo!()
    }

    fn server_busy_error_message_mut(&mut self) -> &mut String {
        todo!()
    }
}

impl Display for BasicRetryPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BasicRetryPolicy")
    }
}

impl ServiceBusRetryPolicy for BasicRetryPolicy {
    type Ok = ();

    type Error = BasicRetryPolicyError;

    type State = BasicRetryPolicyState;

    fn state(&self) -> &Self::State {
        todo!()
    }

    fn state_mut(&mut self) -> &mut Self::State {
        todo!()
    }

    fn calculate_try_timeout(&self, attempt_count: i32) -> std::time::Duration {
        todo!()
    }

    fn calculate_retry_delay(
        &self,
        last_error: &Self::Error,
        attempt_count: i32,
    ) -> Option<std::time::Duration> {
        todo!()
    }
}
