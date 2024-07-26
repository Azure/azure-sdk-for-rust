// Copyright (c) Microsoft Corp. All Rights Reserved.

//cspell: words amqp eventhub amqps servicebus eventhubs mgmt

use azure_core_amqp::{
    cbs::{AmqpClaimsBasedSecurity, AmqpClaimsBasedSecurityTrait},
    connection::{AmqpConnection, AmqpConnectionOptions, AmqpConnectionTrait},
    management::{AmqpManagement, AmqpManagementTrait},
    sender::{AmqpSendOptions, AmqpSender, AmqpSenderOptions, AmqpSenderTrait},
    session::{AmqpSession, AmqpSessionOptions, AmqpSessionTrait},
};

use crate::{
    common::{
        user_agent::{get_package_name, get_package_version, get_platform_info, get_user_agent},
        ManagementInstance,
    },
    error::ErrorKind,
    models::{EventHubPartitionProperties, EventHubProperties},
};
use azure_core::RetryOptions;
use azure_core::{
    auth::AccessToken,
    error::{Error, Result},
};
use batch::{EventDataBatch, EventDataBatchOptions};
use std::sync::{Arc, OnceLock};
use std::{boxed::Box, collections::HashMap};
use tokio::sync::Mutex;
use tracing::{debug, trace};
use url::Url;

pub mod batch;

pub struct ProducerClientOptions {
    application_id: Option<String>,
    retry_options: Option<RetryOptions>,
    max_message_size: Option<u64>,
}

impl ProducerClientOptions {
    pub fn builder() -> builders::ProducerClientOptionsBuilder {
        builders::ProducerClientOptionsBuilder::new()
    }
}

struct SenderInstance {
    #[allow(dead_code)]
    session: AmqpSession,
    sender: Arc<Mutex<AmqpSender>>,
}

pub struct ProducerClient {
    options: ProducerClientOptions,
    sender_instances: Mutex<HashMap<String, SenderInstance>>,
    mgmt_client: Mutex<OnceLock<ManagementInstance>>,
    connection: OnceLock<AmqpConnection>,
    credential: Box<dyn azure_core::auth::TokenCredential>,
    eventhub: String,
    url: String,
    authorization_scopes: Mutex<HashMap<String, AccessToken>>,
}

impl ProducerClient {
    pub fn new(
        fully_qualified_namespace: impl Into<String>,
        eventhub: impl Into<String>,
        credential: impl azure_core::auth::TokenCredential + 'static,
        options: ProducerClientOptions,
    ) -> Self {
        let eventhub: String = eventhub.into();
        let fully_qualified_namespace: String = fully_qualified_namespace.into();
        Self {
            options,
            connection: OnceLock::new(),
            credential: Box::new(credential),
            url: format!("amqps://{}/{}", fully_qualified_namespace, eventhub),
            eventhub,
            authorization_scopes: Mutex::new(HashMap::new()),
            mgmt_client: Mutex::new(OnceLock::new()),
            sender_instances: Mutex::new(HashMap::new()),
        }
    }

    pub async fn open(&self) -> Result<()> {
        self.ensure_connection(&self.url).await?;
        Ok(())
    }

    pub async fn close(self) -> Result<()> {
        self.connection.get().unwrap().close().await?;
        Ok(())
    }
    const BATCH_MESSAGE_FORMAT: u32 = 0x80013700;
    pub async fn create_batch(
        &self,
        batch_options: Option<EventDataBatchOptions>,
    ) -> Result<EventDataBatch> {
        let mut batch = EventDataBatch::new(self, batch_options);

        batch.attach().await?;
        Ok(batch)
    }

    pub async fn submit_batch<'a>(&self, batch: &EventDataBatch<'a>) -> Result<()> {
        let sender = self.ensure_sender(batch.get_batch_path()).await?;
        let messages = batch.get_messages();
        sender
            .lock()
            .await
            .send(
                messages,
                AmqpSendOptions::builder()
                    .with_message_format(Self::BATCH_MESSAGE_FORMAT)
                    .build(),
            )
            .await?;
        Ok(())
    }

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

    pub async fn get_partition_properties(
        &self,
        partition_id: impl Into<String>,
    ) -> Result<EventHubPartitionProperties> {
        self.ensure_management_client().await?;

        self.mgmt_client
            .lock()
            .await
            .get()
            .unwrap()
            .get_eventhub_partition_properties(&self.eventhub, partition_id)
            .await
    }

    pub(crate) fn base_url(&self) -> String {
        self.url.clone()
    }

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
        let management = AmqpManagement::new(session, "eventhubs_management", access_token);
        management.attach().await?;
        mgmt_client
            .set(ManagementInstance::new(management))
            .unwrap();
        trace!("Management client created.");
        Ok(())
    }

    async fn ensure_connection(&self, url: &str) -> Result<()> {
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
                                ("version", get_package_version()),
                                ("platform", get_platform_info()),
                                ("product", get_package_name()),
                            ])
                            .build(),
                    ),
                )
                .await?;
            self.connection.set(connection).unwrap();
        }
        Ok(())
    }

    async fn ensure_sender(&self, path: impl Into<String>) -> Result<Arc<Mutex<AmqpSender>>> {
        let path: String = path.into();
        let mut sender_instances = self.sender_instances.lock().await;
        if !sender_instances.contains_key(&path) {
            self.ensure_connection(&path).await?;
            let connection = self.connection.get().unwrap();

            self.authorize_path(path.clone()).await?;
            let session = AmqpSession::new();
            session
                .begin(
                    connection,
                    Some(
                        AmqpSessionOptions::builder()
                            .with_incoming_window(u32::MAX)
                            .with_outgoing_window(u32::MAX)
                            .build(),
                    ),
                )
                .await?;
            let sender = AmqpSender::new();
            sender
                .attach(
                    &session,
                    format!(
                        "{}-rust-sender",
                        self.options.application_id.as_ref().unwrap()
                    ),
                    path.clone(),
                    Some(
                        AmqpSenderOptions::builder()
                            .with_max_message_size(
                                self.options.max_message_size.unwrap_or(u64::MAX),
                            )
                            .build(),
                    ),
                )
                .await?;
            sender_instances.insert(
                path.clone(),
                SenderInstance {
                    session,
                    sender: Arc::new(Mutex::new(sender)),
                },
            );
        }
        Ok(sender_instances.get(&path).unwrap().sender.clone())
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
                .get_token(&["https://eventhubs.azure.net/.default"])
                .await?;
            debug!("Got token: {:?}", token.token.secret());
            let expires_at = token.expires_on;
            cbs.authorize_path(&url, token.token.secret(), expires_at)
                .await?;
            scopes.insert(url.clone(), token);
        }
        Ok(scopes.get(url.as_str()).unwrap().clone())
    }
}

mod builders {
    use super::*;
    pub struct ProducerClientOptionsBuilder {
        options: ProducerClientOptions,
    }

    impl ProducerClientOptionsBuilder {
        pub(super) fn new() -> Self {
            Self {
                options: ProducerClientOptions {
                    application_id: None,
                    retry_options: None,
                    max_message_size: None,
                },
            }
        }

        pub fn with_application_id(mut self, application_id: impl Into<String>) -> Self {
            self.options.application_id = Some(application_id.into());
            self
        }

        pub fn with_retry_options(mut self, retry_options: RetryOptions) -> Self {
            self.options.retry_options = Some(retry_options);
            self
        }

        pub fn with_max_message_size(mut self, max_message_size: u64) -> Self {
            self.options.max_message_size = Some(max_message_size);
            self
        }

        pub fn build(self) -> ProducerClientOptions {
            self.options
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_producer_client_options_builder() {
        let options = ProducerClientOptions::builder()
            .with_application_id("application_id")
            .with_retry_options(RetryOptions::default())
            .build();
        assert_eq!(options.application_id.unwrap(), "application_id");
    }
}
