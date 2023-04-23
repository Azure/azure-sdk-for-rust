use crate::{EventHubConnectionOptions, EventHubsRetryOptions};

/// The set of options that can be specified when creating an
/// [`crate::consumer::EventHubConsumerClient`] to configure its behavior.
#[derive(Debug, PartialEq, Eq, Clone, Default, Hash)]
pub struct EventHubConsumerClientOptions {
    /// The set of options that can be specified when creating an Event Hub connection.
    pub connection_options: EventHubConnectionOptions,

    /// The set of options that can be specified when retrying operations.
    pub retry_options: EventHubsRetryOptions,

    /// The identifier of the consumer. If not specified, a UUID will be generated.
    pub identifier: Option<String>,
}
