use std::sync::{atomic::Ordering, Arc};

use async_trait::async_trait;
use fe2o3_amqp::Session;
use url::Url;

use crate::{
    amqp::amqp_management::event_hub_properties::EventHubPropertiesRequest,
    authorization::{event_hub_token_credential::EventHubTokenCredential, event_hub_claim},
    consumer::EventPosition,
    core::{RecoverableTransport, TransportClient, TransportProducerFeatures},
    event_hubs_connection_option::EventHubConnectionOptions,
    event_hubs_properties::EventHubProperties,
    event_hubs_retry_policy::EventHubsRetryPolicy,
    producer::PartitionPublishingOptions,
    util::{self, sharable::Sharable, IntoAzureCoreError},
    PartitionProperties,
};

use super::{
    amqp_connection_scope::AmqpConnectionScope,
    amqp_consumer::AmqpConsumer,
    amqp_management::partition_properties::PartitionPropertiesRequest,
    amqp_management_link::AmqpManagementLink,
    amqp_producer::AmqpProducer,
    error::{
        AmqpClientError, AmqpConnectionScopeError, DisposeError, OpenConsumerError,
        OpenProducerError, RecoverConsumeError, RecoverProducerError, RecoverTransportClientError,
    },
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
    ) -> Result<Self, AmqpClientError> {
        // Scheme of service endpoint must always be either "amqp" or "amqps"
        let service_endpoint = format!("{}://{}", options.transport_type.url_scheme(), host);
        let service_endpoint = Url::parse(&service_endpoint)?;

        let connection_endpoint = match options.custom_endpoint_address {
            Some(mut url) => {
                url.set_scheme(options.transport_type.url_scheme())
                    .map_err(|_| AmqpClientError::SetUrlScheme)?;
                url
            }
            None => service_endpoint.clone(),
        };

        // Create AmqpConnectionScope
        let mut connection_scope = AmqpConnectionScope::new(
            service_endpoint,
            connection_endpoint,
            event_hub_name,
            credential,
            options.transport_type,
            options.connection_idle_timeout,
            None,
        )
        .await?;

        // Create AmqpManagementLink
        let management_link = connection_scope.open_management_link().await?;

        Ok(Self {
            connection_scope,
            management_link,
        })
    }
}

#[async_trait]
impl TransportClient for AmqpClient {
    type Producer<RP> = AmqpProducer<RP> where RP: EventHubsRetryPolicy + Send;
    type Consumer<RP> = AmqpConsumer<RP> where RP: EventHubsRetryPolicy + Send;

    type OpenProducerError = OpenProducerError;
    type RecoverProducerError = RecoverProducerError;
    type OpenConsumerError = OpenConsumerError;
    type RecoverConsumerError = RecoverConsumeError;
    type DisposeError = DisposeError;

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
    ) -> Result<Self::Producer<RP>, Self::OpenProducerError>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        let operation_timeout = retry_policy.calculate_try_timeout(0);
        let fut = self.connection_scope.open_producer_link(
            partition_id,
            requested_features,
            partition_options,
            producer_identifier,
            retry_policy,
        );
        let producer = util::time::timeout(operation_timeout, fut).await??;

        Ok(producer)
    }

    async fn recover_producer<RP>(
        &mut self,
        producer: &mut Self::Producer<RP>,
    ) -> Result<(), Self::RecoverProducerError>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        // Seems like event hubs doesn't support resuming a sender

        let endpoint = producer.endpoint.to_string();
        let resource = endpoint.clone();
        let required_claims = vec![event_hub_claim::SEND.to_string()];
        self.connection_scope.request_refreshable_authorization_using_cbs(producer.link_identifier, endpoint, resource, required_claims).await?;

        if producer.session_handle.is_ended() {
            let new_session = Session::begin(&mut self.connection_scope.connection.handle).await?;
            producer
                .sender
                .detach_then_resume_on_session(&new_session)
                .await?;
            producer.session_handle = new_session;
        } else {
            producer
                .sender
                .detach_then_resume_on_session(&producer.session_handle)
                .await?
        };

        Ok(())
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
    ) -> Result<Self::Consumer<RP>, Self::OpenConsumerError>
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
            retry_policy,
        );
        let consumer = util::time::timeout(try_timeout, fut).await??;

        Ok(consumer)
    }

    async fn recover_consumer<RP>(
        &mut self,
        consumer: &mut Self::Consumer<RP>,
    ) -> Result<(), Self::RecoverConsumerError>
    where
        RP: EventHubsRetryPolicy + Send,
    {
        if consumer.session_handle.is_ended() {
            let new_session = Session::begin(&mut self.connection_scope.connection.handle).await?;
            consumer
                .receiver
                .detach_then_resume_on_session(&new_session)
                .await?;
            let mut old_session = std::mem::replace(&mut consumer.session_handle, new_session);
            let _ = old_session.end().await;
        } else {
            consumer
                .receiver
                .detach_then_resume_on_session(&consumer.session_handle)
                .await?;
        }

        Ok(())
    }

    async fn close(&mut self) -> Result<(), Self::DisposeError> {
        self.connection_scope.dispose().await
    }
}

#[async_trait]
impl RecoverableTransport for AmqpClient {
    type RecoverError = RecoverTransportClientError;

    async fn recover(&mut self) -> Result<(), Self::RecoverError> {
        self.connection_scope.recover().await?;
        self.connection_scope
            .recover_management_link(&mut self.management_link)
            .await?;
        Ok(())
    }
}

#[async_trait]
impl RecoverableTransport for Sharable<AmqpClient> {
    type RecoverError = RecoverTransportClientError;

    async fn recover(&mut self) -> Result<(), Self::RecoverError> {
        match self {
            Sharable::Owned(c) => c.recover().await,
            Sharable::Shared(c) => c.lock().await.recover().await,
            Sharable::None => Err(AmqpConnectionScopeError::ScopeDisposed.into()),
        }
    }
}
