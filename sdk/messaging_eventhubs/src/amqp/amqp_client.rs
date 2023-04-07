use std::{sync::{atomic::Ordering, Arc}, time::Duration};

use async_trait::async_trait;
use url::Url;

use crate::{
    amqp::amqp_management::event_hub_properties::EventHubPropertiesRequest,
    authorization::event_hub_token_credential::EventHubTokenCredential,
    consumer::EventPosition,
    core::{
        transport_client::TransportClient, transport_producer_features::TransportProducerFeatures,
    },
    event_hubs_properties::EventHubProperties,
    event_hubs_retry_policy::EventHubsRetryPolicy,
    producer::PartitionPublishingOptions,
    util::{self, IntoAzureCoreError},
    PartitionProperties, event_hubs_transport_type::EventHubsTransportType, event_hubs_connection_option::EventHubConnectionOptions,
};

use super::{
    amqp_connection_scope::AmqpConnectionScope,
    amqp_consumer::AmqpConsumer,
    amqp_management::partition_properties::PartitionPropertiesRequest,
    amqp_management_link::AmqpManagementLink,
    amqp_producer::AmqpProducer,
    error::{OpenConsumerError, OpenProducerError, AmqpConnectionScopeError},
};

const DEFAULT_PREFETCH_COUNT: u32 = 300;

pub struct AmqpClient {
    pub(crate) connection_scope: AmqpConnectionScope,
    pub(crate) management_link: AmqpManagementLink,
}

impl AmqpClient {
    pub(crate) async fn new(
        host: &str,
        event_hub_name: Arc<String>,
        credential: EventHubTokenCredential,
        options: EventHubConnectionOptions,
    ) -> Result<Self, azure_core::Error> {
        // Scheme of service endpoint must always be either "amqp" or "amqps"
        let service_endpoint = format!("{}://{}", options.transport_type.url_scheme(), host);
        let service_endpoint = Url::parse(&service_endpoint)?;

        let connection_endpoint = match options.custom_endpoint_address {
            Some(url) => {
                let url = format!("{}://{}", options.transport_type.url_scheme(), url.host_str().unwrap_or(""));
                Url::parse(&url)?
            },
            None => service_endpoint.clone()
        };

        // Create AmqpConnectionScope
        let mut connection_scope = AmqpConnectionScope::new(
            service_endpoint,
            connection_endpoint,
            event_hub_name,
            credential,
            options.transport_type,
            options.connection_idle_timeout,
            None
        ).await
        .map_err(IntoAzureCoreError::into_azure_core_error)?;

        // Create AmqpManagementLink
        let management_link = connection_scope.open_management_link().await
            .map_err(IntoAzureCoreError::into_azure_core_error)?;

        Ok(Self {
            connection_scope,
            management_link,
        })
    }
}

#[async_trait]
impl TransportClient for AmqpClient {
    type Producer = AmqpProducer;
    type Consumer = AmqpConsumer;
    type OpenProducerError = OpenProducerError;
    type OpenConsumerError = OpenConsumerError;

    fn is_closed(&self) -> bool {
        self.connection_scope.is_disposed.load(Ordering::Relaxed)
    }

    fn service_endpoint(&self) -> &Url {
        &self.connection_scope.service_endpoint
    }

    async fn get_properties<RP>(
        &mut self,
        retry_policy: RP,
    ) -> Result<EventHubProperties, azure_core::Error>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        // TODO: use cancellation token?
        let mut try_timeout = retry_policy.calculate_try_timeout(0);
        let mut failed_attempt = 0;
        let access_token = self
            .connection_scope
            .credential
            .get_token(EventHubTokenCredential::DEFAULT_SCOPE)
            .await?;
        let token_value = access_token.token.secret();
        let request =
            EventHubPropertiesRequest::new(&*self.connection_scope.event_hub_name, token_value);
        loop {
            // The request internally uses Cow, so cloning is cheap.
            let fut = self.management_link.client.call(request.clone());
            let (delay, error) = match util::time::timeout(try_timeout, fut).await {
                Ok(Ok(response)) => return Ok(response),
                Ok(Err(mgmt_err)) => {
                    failed_attempt += 1;
                    let delay = retry_policy.calculate_retry_delay(&mgmt_err, failed_attempt);
                    let error = mgmt_err.into_azure_core_error();
                    (delay, error)
                }
                Err(elapsed) => {
                    failed_attempt += 1;
                    let delay = retry_policy.calculate_retry_delay(&elapsed, failed_attempt);
                    let error = elapsed.into_azure_core_error();
                    (delay, error)
                }
            };

            match delay {
                Some(delay) => {
                    util::time::sleep(delay).await;
                    try_timeout = retry_policy.calculate_try_timeout(failed_attempt);
                }
                None => return Err(error),
            }
        }
    }

    async fn get_partition_properties<RP>(
        &mut self,
        partition_id: &str,
        retry_policy: RP,
    ) -> Result<PartitionProperties, azure_core::Error>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        let mut try_timeout = retry_policy.calculate_try_timeout(0);
        let mut failed_attempt = 0;
        let access_token = self
            .connection_scope
            .credential
            .get_token(EventHubTokenCredential::DEFAULT_SCOPE)
            .await?;
        let token_value = access_token.token.secret();
        let request = PartitionPropertiesRequest::new(
            &*self.connection_scope.event_hub_name,
            partition_id,
            token_value,
        );
        loop {
            // The request internally uses Cow, so cloning is cheap.
            let fut = self.management_link.client.call(request.clone());
            let (delay, error) = match util::time::timeout(try_timeout, fut).await {
                Ok(Ok(response)) => return Ok(response),
                Ok(Err(mgmt_err)) => {
                    failed_attempt += 1;
                    let delay = retry_policy.calculate_retry_delay(&mgmt_err, failed_attempt);
                    let error = mgmt_err.into_azure_core_error();
                    (delay, error)
                }
                Err(elapsed) => {
                    failed_attempt += 1;
                    let delay = retry_policy.calculate_retry_delay(&elapsed, failed_attempt);
                    let error = elapsed.into_azure_core_error();
                    (delay, error)
                }
            };

            match delay {
                Some(delay) => {
                    util::time::sleep(delay).await;
                    try_timeout = retry_policy.calculate_try_timeout(failed_attempt);
                }
                None => return Err(error),
            }
        }
    }

    async fn create_producer<RP>(
        &mut self,
        partition_id: Option<String>,
        producer_identifier: Option<String>,
        requested_features: TransportProducerFeatures,
        partition_options: PartitionPublishingOptions,
        retry_policy: RP,
    ) -> Result<Self::Producer, Self::OpenProducerError>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        let operation_timeout = retry_policy.calculate_try_timeout(0);
        let fut = self.connection_scope.open_producer_link(
            partition_id,
            requested_features,
            partition_options,
            producer_identifier,
        );
        let producer = util::time::timeout(operation_timeout, fut).await??;

        Ok(producer)
    }

    async fn create_consumer<RP>(
        &mut self,
        consumer_group: String,
        partition_id: String,
        consumer_identifier: Option<String>,
        event_position: EventPosition,
        retry_policy: RP,
        track_last_enqueued_event_properties: bool,
        invalidate_consumer_when_partition_stolen: bool,
        owner_level: Option<i64>,
        prefetch_count: Option<u32>,
    ) -> Result<Self::Consumer, Self::OpenConsumerError>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        let try_timeout = retry_policy.calculate_try_timeout(0);
        let fut = self.connection_scope.open_consumer_link(
            consumer_group,
            partition_id,
            event_position,
            prefetch_count.unwrap_or(DEFAULT_PREFETCH_COUNT),
            owner_level,
            track_last_enqueued_event_properties,
            invalidate_consumer_when_partition_stolen,
            consumer_identifier,
        );
        let consumer = util::time::timeout(try_timeout, fut).await??;

        Ok(consumer)
    }
}
