use fe2o3_amqp::link::DetachError;
use futures_util::StreamExt;
use tokio::task::JoinError;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration as StdDuration;

use fe2o3_amqp_cbs::{client::CbsClient, AsyncCbsTokenProvider};
use time::OffsetDateTime;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::util::sharable::Sharable;
use crate::util::time::{DelayQueue, Key};

use super::error::AmqpCbsEventLoopStopped;
use super::{cbs_token_provider::CbsTokenProvider, error::CbsAuthError};

const DELAY_QUEUE_PLACEHOLDER_REFRESH_DURATION: StdDuration = StdDuration::from_secs(30 * 60);
const CBS_LINK_COMMAND_QUEUE_SIZE: usize = 128;

// This is a monotonically incrementing identifier that is assigned when a new link is created.
type LinkIdentifier = u32;

pub(crate) enum Command {
    NewAuthorizationRefresher {
        auth: AuthorizationRefresher,
        result_sender: oneshot::Sender<Result<(), CbsAuthError>>,
    },
    RemoveAuthorizationRefresher(LinkIdentifier),
}

pub(crate) enum Refresher {
    /// This is a placeholder that is only used to avoid spinning the runtime when the
    /// delay queue is exhausted.
    Placeholder,
    Authorization(AuthorizationRefresher),
}

pub(crate) struct AuthorizationRefresher {
    link_identifier: LinkIdentifier,
    endpoint: String,
    resource: String,
    required_claims: Vec<String>,
}

pub(crate) struct AmqpCbsLinkHandle {
    command_sender: mpsc::Sender<Command>,
    stop_sender: CancellationToken,
    join_handle: JoinHandle<Result<(), DetachError>>,
}

impl std::fmt::Debug for AmqpCbsLinkHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AmqpCbsLinkHandle").finish()
    }
}

impl AmqpCbsLinkHandle {
    pub(crate) fn command_sender(&self) -> &mpsc::Sender<Command> {
        &self.command_sender
    }

    pub(crate) async fn request_refreshable_authorization(
        &mut self,
        link_identifier: u32,
        endpoint: String,
        resource: String,
        required_claims: Vec<String>,
    ) -> Result<Result<(), CbsAuthError>, AmqpCbsEventLoopStopped> {
        let auth = AuthorizationRefresher {
            link_identifier,
            endpoint,
            resource,
            required_claims,
        };
        let (result_sender, result) = oneshot::channel();
        let command = Command::NewAuthorizationRefresher {
            auth,
            result_sender,
        };
        self.command_sender
            .send(command)
            .await
            .map_err(|_| AmqpCbsEventLoopStopped {})?;

        result.await.map_err(|_| AmqpCbsEventLoopStopped {})
    }

    pub(crate) fn stop(&self) {
        self.stop_sender.cancel();
    }

    pub(crate) fn join_handle_mut(&mut self) -> &mut JoinHandle<Result<(), DetachError>> {
        &mut self.join_handle
    }
}

impl Sharable<AmqpCbsLinkHandle> {
    pub(crate) async fn request_refreshable_authorization(
        &mut self,
        link_identifier: u32,
        endpoint: String,
        resource: String,
        required_claims: Vec<String>,
    ) -> Result<Result<(), CbsAuthError>, AmqpCbsEventLoopStopped> {
        match self {
            Self::Owned(link) => {
                link.request_refreshable_authorization(
                    link_identifier,
                    endpoint,
                    resource,
                    required_claims,
                )
                .await
            }
            Self::Shared(link) => {
                link.write()
                    .await
                    .request_refreshable_authorization(
                        link_identifier,
                        endpoint,
                        resource,
                        required_claims,
                    )
                    .await
            }
            Self::None => unreachable!(),
        }
    }

    pub(crate) async fn command_sender(&self) -> mpsc::Sender<Command> {
        match self {
            Self::Owned(link) => link.command_sender().clone(),
            Self::Shared(link) => link.read().await.command_sender().clone(),
            Self::None => unreachable!(),
        }
    }

    /// Stop regardless of ownership
    pub(crate) async fn stop(&self) {
        match self {
            Self::Owned(link) => link.stop(),
            Self::Shared(link) => link.write().await.stop(),
            Self::None => unreachable!(),
        }
    }

    pub(crate) async fn stop_if_owned(&self) {
        match self {
            Self::Owned(link) => link.stop(),
            Self::Shared(link) => {
                if Arc::strong_count(link) == 1 {
                    link.write().await.stop();
                }
            },
            Self::None => unreachable!(),
        }
    }

    /// Join regardless of ownership
    pub(crate) async fn join(&mut self) -> Result<Result<(), DetachError>, JoinError> {
        match self {
            Self::Owned(link) => link.join_handle_mut().await,
            Self::Shared(link) => {
                let mut link = link.write().await;
                link.join_handle_mut().await
            }
            Self::None => unreachable!(),
        }
    }

    pub(crate) async fn join_if_owned(&mut self) -> Result<Result<(), DetachError>, JoinError> {
        match self {
            Self::Owned(link) => link.join_handle_mut().await,
            Self::Shared(link) => match Arc::strong_count(link) {
                1 => link.write().await.join_handle_mut().await,
                _ => Ok(Ok(())),
            },
            Self::None => unreachable!(),
        }
    }
}

pub(crate) struct AmqpCbsLink {
    pub stop: CancellationToken,
    pub commands: mpsc::Receiver<Command>,
    pub active_link_identifiers: HashMap<LinkIdentifier, Key>,
    pub delay_queue: DelayQueue<Refresher>,
    pub cbs_token_provider: CbsTokenProvider,
    pub cbs_client: CbsClient,
}

impl AmqpCbsLink {
    pub(crate) fn new(
        cbs_token_provider: CbsTokenProvider,
        cbs_client: CbsClient,
        commands: mpsc::Receiver<Command>,
        stop: CancellationToken,
    ) -> Self {
        let mut delay_queue = DelayQueue::new();
        delay_queue.insert(
            Refresher::Placeholder,
            DELAY_QUEUE_PLACEHOLDER_REFRESH_DURATION,
        );

        AmqpCbsLink {
            stop,
            commands,
            active_link_identifiers: HashMap::new(),
            delay_queue,
            cbs_token_provider,
            cbs_client,
        }
    }

    cfg_not_wasm32! {
        pub(crate) fn spawn(
            cbs_token_provider: CbsTokenProvider,
            cbs_client: CbsClient,
        ) -> AmqpCbsLinkHandle {
            let (command_sender, commands) = mpsc::channel(CBS_LINK_COMMAND_QUEUE_SIZE);
            let stop_sender = CancellationToken::new();
            let stop = stop_sender.child_token();
            let amqp_cbs_link = AmqpCbsLink::new(cbs_token_provider, cbs_client, commands, stop);

            let join_handle = tokio::spawn(amqp_cbs_link.event_loop());
            AmqpCbsLinkHandle {
                command_sender,
                stop_sender,
                join_handle,
            }
        }
    }

    cfg_wasm32! {
        pub(crate) fn spawn_local(
            cbs_token_provider: CbsTokenProvider,
            cbs_client: CbsClient,
        ) -> AmqpCbsLinkHandle {
            let (command_sender, commands) = mpsc::channel(CBS_LINK_COMMAND_QUEUE_SIZE);
            let stop_sender = CancellationToken::new();
            let stop = stop_sender.child_token();
            let amqp_cbs_link = AmqpCbsLink::new(cbs_token_provider, cbs_client, commands, stop);

            let join_handle = tokio::task::spawn_local(amqp_cbs_link.event_loop());
            AmqpCbsLinkHandle {
                command_sender,
                stop_sender,
                join_handle,
            }
        }
    }

    async fn request_authorization_using_cbs(
        &mut self,
        endpoint: impl AsRef<str>,
        resource: impl AsRef<str>,
        required_claims: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<Option<crate::util::time::Instant>, CbsAuthError> {
        let resource = resource.as_ref();
        let token = self
            .cbs_token_provider
            .get_token_async(endpoint, resource, required_claims)
            .await?;

        // find the smallest timeout
        let expires_at_utc = token.expires_at_utc().clone().map(OffsetDateTime::from);

        // TODO: Is there any way to convert directly from OffsetDateTime/Timestamp to StdInstant?
        let expires_at_instant = expires_at_utc.map(|expires_at| {
            let now_instant = crate::util::time::Instant::now();
            let now = crate::util::time::now_utc(); // TODO: is there any way to convert instant to datetime?
            let timespan = expires_at - now;
            now_instant + timespan.unsigned_abs()
        });

        // TODO: There are some custom application properties in the dotnet sdk.
        // Maybe we should have a custom type that supports this?
        self.cbs_client.put_token(resource, token).await?;

        Ok(expires_at_instant)
    }

    async fn handle_command(&mut self, command: Command) {
        match command {
            Command::NewAuthorizationRefresher {
                auth,
                result_sender,
            } => {
                // First request authorization once, and then schedule a refresh.
                let result = self
                    .request_authorization_using_cbs(
                        &auth.endpoint,
                        &auth.resource,
                        &auth.required_claims,
                    )
                    .await;
                match result {
                    Ok(expires_at) => {
                        if let Some(expires_at) = expires_at {
                            if expires_at > crate::util::time::Instant::now() {
                                let link_identifier = auth.link_identifier;
                                let key = self
                                    .delay_queue
                                    .insert_at(Refresher::Authorization(auth), expires_at);
                                self.active_link_identifiers.insert(link_identifier, key);
                            }
                        }
                        let _ = result_sender.send(Ok(()));
                    }
                    Err(err) => {
                        let _ = result_sender.send(Err(err));
                    }
                }
            }
            Command::RemoveAuthorizationRefresher(link_identifier) => {
                let key = self.active_link_identifiers.remove(&link_identifier);
                if let Some(key) = key {
                    self.delay_queue.remove(&key);
                }
            }
        }
    }

    async fn handle_refresher(&mut self, refresher: Refresher) {
        match refresher {
            Refresher::Placeholder => {
                let _key = self.delay_queue.insert(
                    Refresher::Placeholder,
                    DELAY_QUEUE_PLACEHOLDER_REFRESH_DURATION,
                );
            }
            Refresher::Authorization(auth) => {
                let link_identifier = auth.link_identifier;
                let result = self
                    .request_authorization_using_cbs(
                        &auth.endpoint,
                        &auth.resource,
                        &auth.required_claims,
                    )
                    .await;
                match result {
                    Ok(expires_at) => {
                        if let Some(expires_at) = expires_at {
                            if expires_at > crate::util::time::Instant::now() {
                                let key = self
                                    .delay_queue
                                    .insert_at(Refresher::Authorization(auth), expires_at);
                                self.active_link_identifiers.insert(link_identifier, key);
                            }
                        }
                    }
                    Err(err) => {
                        // TODO: log error
                        log::error!("CBS authorization refresh failed: {}", err);
                    }
                }
            }
        }
    }

    pub(crate) async fn event_loop(mut self) -> Result<(), DetachError> {
        loop {
            tokio::select! {
                _stop_cbs_link = self.stop.cancelled() => {
                    return self.cbs_client.close().await
                },
                command = self.commands.recv() => {
                    if let Some(command) = command {
                        self.handle_command(command).await;
                    } else {
                        // All senders including the one held by AmqpConnectionScope have been dropped, so we should stop.
                        return self.cbs_client.close().await
                    }
                },
                refresher = self.delay_queue.next() => {
                    // A `None` is returned if the queue is exhausted. New refresher may still be
                    // added in the future.
                    if let Some(refresher) = refresher {
                        self.handle_refresher(refresher.into_inner()).await;
                    } else {
                        // The delay queue is exhausted. We need to add a placeholder to avoid
                        // spinning the runtime.
                        let _key = self.delay_queue.insert(Refresher::Placeholder, DELAY_QUEUE_PLACEHOLDER_REFRESH_DURATION);
                    }
                }
            }
        }
    }
}
