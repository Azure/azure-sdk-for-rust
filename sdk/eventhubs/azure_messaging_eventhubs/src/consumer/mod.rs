// Copyright (c) Microsoft Corp. All Rights Reserved.

//cspell: words amqp eventhub
use azure_core::RetryOptions;
use azure_core_amqp::connection::AmqpConnection;

pub struct ConsumerClientOptions {
    application_id: Option<String>,
    retry_options: Option<RetryOptions>,
}

impl ConsumerClientOptions {
    pub fn builder() -> ConsumerClientOptionsBuilder {
        ConsumerClientOptionsBuilder::new()
    }

    pub fn with_application_id(mut self, application_id: &str) -> Self {
        self.application_id = Some(application_id.to_owned());
        self
    }

    pub fn with_retry_options(mut self, retry_options: RetryOptions) -> Self {
        self.retry_options = Some(retry_options);
        self
    }
}

pub struct ConsumerClientOptionsBuilder {
    application_id: Option<String>,
    retry_options: Option<RetryOptions>,
}

impl ConsumerClientOptionsBuilder {
    pub fn new() -> Self {
        Self {
            application_id: None,
            retry_options: None,
        }
    }

    pub fn with_application_id(mut self, application_id: &str) -> Self {
        self.application_id = Some(application_id.to_owned());
        self
    }

    pub fn with_retry_options(mut self, retry_options: RetryOptions) -> Self {
        self.retry_options = Some(retry_options);
        self
    }

    pub fn build(self) -> ConsumerClientOptions {
        ConsumerClientOptions {
            application_id: self.application_id,
            retry_options: self.retry_options,
        }
    }
}

pub struct EventHubsConsumerClient {
    _connection: AmqpConnection,
    _consumer_group: String,
    _eventhub_name: String,
    _consumer_name: String,
    _partition_id: String,
    _options: ConsumerClientOptions,
}
