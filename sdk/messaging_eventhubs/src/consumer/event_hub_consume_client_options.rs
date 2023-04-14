use crate::{EventHubConnectionOptions, EventHubsRetryOptions};

#[derive(Debug, PartialEq, Eq, Clone, Default, Hash)]
pub struct EventHubConsumeClientOptions {
    pub connection_options: EventHubConnectionOptions,
    pub retry_options: EventHubsRetryOptions,
    pub identifier: Option<String>,
}
