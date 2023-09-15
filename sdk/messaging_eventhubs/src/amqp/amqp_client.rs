use std::sync::{atomic::Ordering, Arc};

use async_trait::async_trait;
use fe2o3_amqp::link::ReceiverAttachExchange;
use fe2o3_amqp_types::messaging::{Body, FilterSet, Modified, Source};
use serde_amqp::{described::Described, Value};
use url::Url;

use crate::{
    amqp::amqp_management::event_hub_properties::EventHubPropertiesRequest,
    authorization::{event_hub_claim, event_hub_token_credential::EventHubTokenCredential},
    consumer::EventPosition,
    core::{RecoverableError, RecoverableTransport, TransportClient, TransportProducerFeatures},
    event_hubs_connection_option::EventHubConnectionOptions,
    event_hubs_properties::EventHubProperties,
    event_hubs_retry_policy::EventHubsRetryPolicy,
    producer::PartitionPublishingOptions,
    util::{self, sharable::Sharable},
    PartitionProperties,
};

use super::{
    amqp_connection_scope::AmqpConnectionScope,
    amqp_consumer::AmqpConsumer,
    amqp_filter::{self, ConsumerFilter},
    amqp_management::partition_properties::PartitionPropertiesRequest,
    amqp_management_link::AmqpManagementLink,
    amqp_producer::AmqpProducer,
    error::{
        AmqpClientError, DisposeError, OpenConsumerError,
        OpenProducerError, RecoverAndCallError, RecoverConsumerError, RecoverProducerError,
        RecoverTransportClientError,
    },
};

const DEFAULT_PREFETCH_COUNT: u32 = 300;

#[derive(Debug)]
pub struct AmqpClient {
    pub(crate) connection_scope: AmqpConnectionScope,
    pub(crate) management_link: Sharable<AmqpManagementLink>,
}

impl AmqpClient {
    pub(crate) fn clone_as_shared(&mut self) -> Self {
        let shared_mgmt_link = self.management_link.clone_as_shared();
        let shared_mgmt_link = match shared_mgmt_link {
            Some(shared_mgmt_link) => Sharable::Shared(shared_mgmt_link),
            None => Sharable::None,
        };

        Self {
            connection_scope: self.connection_scope.clone_as_shared(),
            management_link: shared_mgmt_link,
        }
    }

    async fn recover_and_get_properties<'a>(
        &mut self,
        should_try_recover: bool,
        token_value: &'a str,
    ) -> Result<EventHubProperties, RecoverAndCallError> {
        if should_try_recover {
            self.recover().await?;
        }

        let request =
            EventHubPropertiesRequest::new(&*self.connection_scope.event_hub_name, token_value);

        let res = self.management_link.call(request).await?;
        Ok(res)
    }

    async fn recover_and_get_partition_properties<'a>(
        &mut self,
        should_try_recover: bool,
        partition_id: &'a str,
        token_value: &'a str,
    ) -> Result<PartitionProperties, RecoverAndCallError> {
        if should_try_recover {
            self.recover().await?;
        }

        let request = PartitionPropertiesRequest::new(
            &*self.connection_scope.event_hub_name,
            partition_id,
            token_value,
        );

        let res = self.management_link.call(request).await?;
        Ok(res)
    }
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
        let management_link = Sharable::Owned(management_link);
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
    type RecoverConsumerError = RecoverConsumerError;
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
        let mut should_try_recover = false;

        let access_token = self
            .connection_scope
            .credential
            .get_token_using_default_resource()
            .await?;
        let token_value = access_token.token.secret();
        loop {
            // The request internally uses Cow, so cloning is cheap.
            let fut = self.recover_and_get_properties(should_try_recover, token_value);
            let error = match util::time::timeout(try_timeout, fut).await {
                Ok(Ok(response)) => return Ok(response),
                Ok(Err(err)) => err,
                Err(elapsed) => elapsed.into(),
            };

            failed_attempt += 1;
            let delay = retry_policy.calculate_retry_delay(&error, failed_attempt);
            should_try_recover = error.should_try_recover();
            match delay {
                Some(delay) => {
                    util::time::sleep(delay).await;
                    try_timeout = retry_policy.calculate_try_timeout(failed_attempt);
                }
                None => return Err(error.into()),
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
        let mut should_try_recover = false;

        let access_token = self
            .connection_scope
            .credential
            .get_token_using_default_resource()
            .await?;
        let token_value = access_token.token.secret();
        loop {
            let fut = self.recover_and_get_partition_properties(
                should_try_recover,
                partition_id,
                token_value,
            );
            let error = match util::time::timeout(try_timeout, fut).await {
                Ok(Ok(response)) => return Ok(response),
                Ok(Err(err)) => err,
                Err(elapsed) => elapsed.into(),
            };

            failed_attempt += 1;
            let delay = retry_policy.calculate_retry_delay(&error, failed_attempt);
            should_try_recover = error.should_try_recover();
            match delay {
                Some(delay) => {
                    util::time::sleep(delay).await;
                    try_timeout = retry_policy.calculate_try_timeout(failed_attempt);
                }
                None => return Err(error.into()),
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
        let endpoint = producer.endpoint.to_string();
        let resource = endpoint.clone();
        let required_claims = vec![event_hub_claim::SEND.to_string()];
        self.connection_scope
            .request_refreshable_authorization_using_cbs(
                producer.link_identifier,
                endpoint,
                resource,
                required_claims,
            )
            .await?;

        if producer.session_handle.is_ended() {
            let new_session = self.connection_scope.connection.begin_session().await?;
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
        consumer_group: &str,
        partition_id: &str,
        consumer_identifier: Option<String>,
        event_position: EventPosition,
        retry_policy: RP,
        track_last_enqueued_event_properties: bool,
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
        let endpoint = consumer.endpoint.to_string();
        let resource = endpoint.clone();
        let required_claims = vec![event_hub_claim::LISTEN.to_string()];
        self.connection_scope
            .request_refreshable_authorization_using_cbs(
                consumer.link_identifier,
                endpoint,
                resource,
                required_claims,
            )
            .await?;

        if let Some(Ok(event_position)) = consumer
            .current_event_position
            .clone()
            .map(amqp_filter::build_filter_expression)
        {
            let consumer_filter = Described::<Value>::from(ConsumerFilter(event_position));
            let source = consumer.receiver.source_mut().get_or_insert(
                Source::builder()
                    .address(consumer.endpoint.to_string())
                    .build(),
            );
            let source_filter = source.filter.get_or_insert(FilterSet::new());
            source_filter.insert(
                amqp_filter::CONSUMER_FILTER_NAME.into(),
                consumer_filter.into(),
            );
        }

        let mut exchange = if consumer.session_handle.is_ended() {
            let new_session = self.connection_scope.connection.begin_session().await?;
            let exchange = consumer
                .receiver
                .detach_then_resume_on_session(&new_session)
                .await?;
            let mut old_session = std::mem::replace(&mut consumer.session_handle, new_session);
            let _ = old_session.end().await;
            exchange
        } else {
            consumer
                .receiver
                .detach_then_resume_on_session(&consumer.session_handle)
                .await?
        };

        // `ReceiverAttachExchange::Complete` => Resume is complete
        //
        // `ReceiverAttachExchange::IncompleteUnsettled` => There are unsettled messages, multiple
        // detach and re-attach may happen in order to reduce the number of unsettled messages.
        //
        // `ReceiverAttachExchange::Resume` => There is one message that is partially transferred,
        // so it would be OK to let the user use the receiver to receive the message
        while let ReceiverAttachExchange::IncompleteUnsettled = exchange {
            match consumer.receiver.recv::<Body<Value>>().await {
                Ok(delivery) => {
                    let modified = Modified {
                        delivery_failed: None,
                        undeliverable_here: None,
                        message_annotations: None,
                    };
                    if let Err(err) = consumer.receiver.modify(delivery, modified).await {
                        log::error!("Failed to abandon message: {}", err);
                        exchange = consumer
                            .receiver
                            .detach_then_resume_on_session(&consumer.session_handle)
                            .await?;
                    }
                }
                Err(err) => {
                    log::error!("Failed to receive message while trying to settle (abandon) the unsettled: {}", err);
                    exchange = consumer
                        .receiver
                        .detach_then_resume_on_session(&consumer.session_handle)
                        .await?;
                }
            }
        }

        Ok(())
    }

    async fn close(&mut self) -> Result<(), Self::DisposeError> {
        self.connection_scope.close().await
    }

    async fn close_if_owned(&mut self) -> Result<(), Self::DisposeError> {
        self.connection_scope.close_if_owned().await
    }

    fn is_owned(&self) -> bool {
        self.connection_scope.is_owned()
    }

    fn is_shared(&self) -> bool {
        self.connection_scope.is_shared()
    }
}

#[async_trait]
impl RecoverableTransport for AmqpClient {
    type RecoverError = RecoverTransportClientError;

    async fn recover(&mut self) -> Result<(), Self::RecoverError> {
        self.connection_scope.recover().await?;
        match &mut self.management_link {
            Sharable::Owned(link) => self.connection_scope.recover_management_link(link).await?,
            Sharable::Shared(lock) => {
                let mut link = lock.write().await;
                self.connection_scope.recover_management_link(&mut link).await?
            }
            Sharable::None => {},
        }

        Ok(())
    }
}
