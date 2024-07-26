// Copyright (c) Microsoft Corp. All Rights Reserved.

//cspell: words amqp eventhub eventhubs mgmt amqps

use super::{
    common::{
        user_agent::{get_package_name, get_package_version, get_platform_info, get_user_agent},
        ManagementInstance,
    },
    error::ErrorKind,
    models::{EventHubPartitionProperties, EventHubProperties, ReceivedEventData, StartPosition},
};

use async_stream::try_stream;
use azure_core::{
    auth::{AccessToken, TokenCredential},
    error::{Error, Result},
    RetryOptions,
};
use azure_core_amqp::{
    cbs::{AmqpClaimsBasedSecurity, AmqpClaimsBasedSecurityTrait},
    connection::{AmqpConnection, AmqpConnectionOptions, AmqpConnectionTrait},
    management::{AmqpManagement, AmqpManagementTrait},
    messaging::{AmqpSource, AmqpSourceFilter},
    receiver::{AmqpReceiver, AmqpReceiverOptions, AmqpReceiverTrait, ReceiverCreditMode},
    session::{AmqpSession, AmqpSessionTrait},
    value::AmqpDescribed,
};

use futures::stream::Stream;
use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, OnceLock},
};
use tokio::sync::Mutex;
use tracing::{debug, info, trace};
use url::Url;

#[derive(Debug, Default)]
pub struct ConsumerClientOptions {
    application_id: Option<String>,
    instance_id: Option<String>,
    retry_options: Option<RetryOptions>,
}

impl ConsumerClientOptions {
    pub fn builder() -> builders::ConsumerClientOptionsBuilder {
        builders::ConsumerClientOptionsBuilder::new()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReceiveOptions {
    owner_level: Option<i64>,
    prefetch: Option<u32>,
    start_position: Option<StartPosition>,
}
impl ReceiveOptions {
    pub fn builder() -> builders::ReceiveOptionsBuilder {
        builders::ReceiveOptionsBuilder::new()
    }
}

#[derive(Debug)]
pub struct ConsumerClient {
    options: ConsumerClientOptions,
    session_instances: Mutex<HashMap<String, Arc<AmqpSession>>>,
    mgmt_client: Mutex<OnceLock<ManagementInstance>>,
    connection: OnceLock<AmqpConnection>,
    receiver: OnceLock<AmqpReceiver>,
    credential: Box<dyn azure_core::auth::TokenCredential>,
    eventhub: String,
    url: String,
    authorization_scopes: Mutex<HashMap<String, AccessToken>>,
}

impl ConsumerClient {
    pub fn new(
        fully_qualified_namespace: impl Into<String>,
        eventhub_name: impl Into<String>,
        consumer_group: Option<String>,
        credential: impl TokenCredential + 'static,
        options: Option<ConsumerClientOptions>,
    ) -> Self {
        let eventhub_name = eventhub_name.into();
        let consumer_group = consumer_group.unwrap_or("$Default".into());
        let url = format!(
            "amqps://{}/{}/ConsumerGroups/{}",
            fully_qualified_namespace.into(),
            eventhub_name,
            consumer_group
        );
        Self {
            options: options.unwrap_or_default(),
            session_instances: Mutex::new(HashMap::new()),
            mgmt_client: Mutex::new(OnceLock::new()),
            connection: OnceLock::new(),
            receiver: OnceLock::new(),
            credential: Box::new(credential),
            eventhub: eventhub_name,
            url: url,
            authorization_scopes: Mutex::new(HashMap::new()),
        }
    }

    #[tracing::instrument]
    pub async fn open(&self) -> Result<()> {
        self.ensure_connection(&self.url).await?;
        Ok(())
    }

    #[tracing::instrument]
    pub async fn close(self) -> Result<()> {
        self.connection.get().unwrap().close().await?;
        Ok(())
    }

    #[tracing::instrument]
    pub async fn receive_events_on_partition(
        &self,
        partition_id: impl Into<String> + Debug + Clone,
        options: Option<ReceiveOptions>,
    ) -> impl Stream<Item = Result<ReceivedEventData>> + '_ {
        let partition_id = partition_id.into();
        let options = options.unwrap_or_default();

        let receiver_name = format!("receiver-{}-{}", partition_id, uuid::Uuid::new_v4());
        let start_expression = StartPosition::start_expression(&options.start_position);
        let source_url = format!("{}/Partitions/{}", self.url, partition_id);

        try_stream! {
            // Authorize access to the source address.
        self.authorize_path(source_url.clone()).await?;

        let session = self.get_session(&partition_id).await?;
        let message_source = AmqpSource::builder()
            .with_address(source_url)
            .add_to_filter(
                AmqpSourceFilter::selector_filter().description(),
                Box::new(AmqpDescribed::new(
                    AmqpSourceFilter::selector_filter().code(),
                    start_expression,
                )),
            )
            .build();
        let receiver = AmqpReceiver::new();
        receiver
            .attach(
                &session,
                message_source,
                Some(
                    AmqpReceiverOptions::builder()
                        //                        .with_auto_accept(true)
                        //                        .with_credit_mode(ReceiverCreditMode::Auto(300))
                        .with_name(receiver_name)
                        .build(),
                ),
            )
            .await?;

            loop{
                let message = receiver.receive().await?;
                let event: ReceivedEventData= message.into();
                yield event;
            }
        }
    }

    pub async fn receive_event_on_partition(
        &self,
        partition_id: impl Into<String> + Debug,
        starting_position: Option<StartPosition>,
    ) -> Result<ReceivedEventData> {
        let partition_id = partition_id.into();

        if self.receiver.get().is_none() {
            let receiver_name = format!("receiver-{}-{}", partition_id, uuid::Uuid::new_v4());
            let start_expression = StartPosition::start_expression(&starting_position);
            let source_address = format!("{}/Partitions/{}", self.url, partition_id);

            // Authorize access to the source address.
            self.authorize_path(source_address.clone()).await?;

            let session = self.get_session(&partition_id).await?;
            let message_source = AmqpSource::builder()
                .with_address(source_address)
                .add_to_filter(
                    AmqpSourceFilter::selector_filter().description(),
                    Box::new(AmqpDescribed::new(
                        AmqpSourceFilter::selector_filter().code(),
                        start_expression,
                    )),
                )
                .build();
            let receiver = AmqpReceiver::new();
            receiver
                .attach(
                    &session,
                    message_source,
                    Some(
                        AmqpReceiverOptions::builder()
                            //                        .with_auto_accept(true)
                            //                        .with_credit_mode(ReceiverCreditMode::Auto(300))
                            .with_name(receiver_name)
                            .build(),
                    ),
                )
                .await?;
            self.receiver.set(receiver).unwrap();
        }

        let message = self.receiver.get().unwrap().receive().await?;
        Ok(message.into())
    }

    #[tracing::instrument]
    pub async fn get_eventhub_properties(&self) -> Result<EventHubProperties> {
        self.ensure_management_client().await?;

        self.mgmt_client
            .lock()
            .await
            .get()
            .unwrap()
            .get_eventhub_properties(&self.eventhub)
            .await
    }

    #[tracing::instrument]
    pub async fn get_partition_properties<T>(
        &self,
        partition_id: T,
    ) -> Result<EventHubPartitionProperties>
    where
        T: Into<String> + Debug,
    {
        self.ensure_management_client().await?;

        self.mgmt_client
            .lock()
            .await
            .get()
            .unwrap()
            .get_eventhub_partition_properties(&self.eventhub, partition_id)
            .await
    }

    #[tracing::instrument]
    async fn ensure_management_client(&self) -> Result<()> {
        trace!("Ensure management client.");

        let mgmt_client = self.mgmt_client.lock().await;

        if mgmt_client.get().is_some() {
            trace!("Management client already exists.");
            return Ok(());
        }

        // Clients must call ensure_connection before calling ensure_management_client.
        if self.connection.get().is_none() {
            return Err(ErrorKind::MissingConnection.into());
        }

        trace!("Create management session.");
        let connection = self.connection.get().unwrap();
        let session = AmqpSession::new();
        session.begin(connection, None).await?;
        trace!("Session created.");

        let management_path = self.url.clone() + "/$management";
        let access_token = self.authorize_path(management_path).await?;

        trace!("Create management client.");
        let management =
            AmqpManagement::new(session, "eventhubs_consumer_management", access_token);
        management.attach().await?;
        mgmt_client
            .set(ManagementInstance::new(management))
            .unwrap();
        trace!("Management client created.");
        Ok(())
    }

    #[tracing::instrument]
    async fn ensure_connection(&self, url: &String) -> Result<()> {
        if self.connection.get().is_none() {
            let connection = AmqpConnection::new();
            connection
                .open(
                    self.options
                        .application_id
                        .clone()
                        .unwrap_or(uuid::Uuid::new_v4().to_string())
                        .as_str(),
                    Url::parse(url).map_err(Error::from)?,
                    Some(
                        AmqpConnectionOptions::builder()
                            .with_properties(vec![
                                ("user-agent", get_user_agent(&self.options.application_id)),
                                ("version", get_package_version().into()),
                                ("platform", get_platform_info().into()),
                                ("product", get_package_name().into()),
                            ])
                            .build(),
                    ),
                )
                .await?;
            self.connection.set(connection).unwrap();
        }
        Ok(())
    }

    async fn authorize_path(&self, url: impl Into<String>) -> Result<AccessToken> {
        let url: String = url.into();
        debug!("Authorizing path: {:?}", url);
        let mut scopes = self.authorization_scopes.lock().await;
        if self.connection.get().is_none() {
            return Err(ErrorKind::MissingConnection.into());
        }
        if !scopes.contains_key(url.as_str()) {
            let connection = self.connection.get().unwrap();

            // Create an ephemeral session to host the authentication.
            let session = AmqpSession::new();
            session.begin(connection, None).await?;

            let cbs = AmqpClaimsBasedSecurity::new(session);
            cbs.attach().await?;

            debug!("Get Token.");
            let token = self
                .credential
                .get_token(&[&"https://eventhubs.azure.net/.default"])
                .await?;
            debug!("Got token: {:?}", token.token.secret());
            let expires_at = token.expires_on;
            cbs.authorize_path(&url, token.token.secret(), expires_at)
                .await?;
            scopes.insert(url.clone(), token);
        }
        Ok(scopes.get(url.as_str()).unwrap().clone())
    }

    async fn get_session(&self, partition_id: &String) -> Result<Arc<AmqpSession>> {
        let mut session_instances = self.session_instances.lock().await;
        if !session_instances.contains_key(partition_id) {
            info!("Creating session for partition: {:?}", partition_id);
            let connection = self.connection.get().unwrap();
            let session = AmqpSession::new();
            session.begin(connection, None).await?;
            session_instances.insert(partition_id.clone(), Arc::new(session));
        }
        let rv = session_instances.get(partition_id).unwrap().clone();
        info!("Cloning session for partition {:?}: {:?}", partition_id, rv);
        Ok(rv)
    }
}

mod builders {
    use super::*;

    pub struct ConsumerClientOptionsBuilder {
        options: ConsumerClientOptions,
    }

    impl ConsumerClientOptionsBuilder {
        pub(super) fn new() -> Self {
            Self {
                options: ConsumerClientOptions {
                    application_id: None,
                    retry_options: None,
                    instance_id: None,
                },
            }
        }

        pub fn with_application_id<T>(mut self, application_id: T) -> Self
        where
            T: Into<String>,
        {
            self.options.application_id = Some(application_id.into());
            self
        }

        pub fn with_retry_options(mut self, retry_options: RetryOptions) -> Self {
            self.options.retry_options = Some(retry_options);
            self
        }

        pub fn with_instance_id<T>(mut self, instance_id: T) -> Self
        where
            T: Into<String>,
        {
            self.options.instance_id = Some(instance_id.into());
            self
        }

        pub fn build(self) -> ConsumerClientOptions {
            self.options
        }
    }

    pub struct ReceiveOptionsBuilder {
        options: ReceiveOptions,
    }

    impl ReceiveOptionsBuilder {
        pub(super) fn new() -> Self {
            Self {
                options: ReceiveOptions {
                    owner_level: None,
                    prefetch: None,
                    start_position: None,
                },
            }
        }

        pub fn with_owner_level(mut self, owner_level: i64) -> Self {
            self.options.owner_level = Some(owner_level);
            self
        }

        pub fn with_prefetch(mut self, prefetch: u32) -> Self {
            self.options.prefetch = Some(prefetch);
            self
        }

        pub fn with_start_position(mut self, start_position: StartPosition) -> Self {
            self.options.start_position = Some(start_position);
            self
        }

        pub fn build(self) -> ReceiveOptions {
            self.options
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_consumer_options() {
        {
            let options = ConsumerClientOptions::default();
            assert!(options.application_id.is_none());
            assert!(options.instance_id.is_none());
            assert!(options.retry_options.is_none());
        }

        {
            let options = ConsumerClientOptions::builder().build();
            assert!(options.application_id.is_none());
            assert!(options.instance_id.is_none());
            assert!(options.retry_options.is_none());
        }
    }

    #[test]
    fn test_consumer_client_with_options() {
        let options = ConsumerClientOptions::builder()
            .with_application_id("test_app_id")
            .with_retry_options(RetryOptions::default())
            .with_instance_id("test_instance_id")
            .build();

        assert_eq!(options.application_id, Some("test_app_id".to_string()));
        assert_eq!(options.instance_id, Some("test_instance_id".to_string()));
        assert!(options.retry_options.is_some());
    }
}
