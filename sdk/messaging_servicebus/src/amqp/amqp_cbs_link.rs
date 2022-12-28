use fe2o3_amqp::link::DetachError;
use futures_util::StreamExt;
use std::collections::HashMap;
use std::time::Duration as StdDuration;
use std::time::Instant as StdInstant;

use fe2o3_amqp_cbs::{client::CbsClient, AsyncCbsTokenProvider};
use time::OffsetDateTime;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tokio_util::time::delay_queue;
use tokio_util::time::DelayQueue;

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

pub(crate) struct AmqpCbsLink {
    pub stop: CancellationToken,
    pub commands: mpsc::Receiver<Command>,
    pub active_link_identifiers: HashMap<LinkIdentifier, delay_queue::Key>,
    pub delay_queue: DelayQueue<Refresher>,
    pub cbs_token_provider: CbsTokenProvider,
    pub cbs_client: CbsClient,
}

impl AmqpCbsLink {
    pub(crate) fn spawn(
        cbs_token_provider: CbsTokenProvider,
        cbs_client: CbsClient,
    ) -> AmqpCbsLinkHandle {
        let (command_sender, commands) = mpsc::channel(CBS_LINK_COMMAND_QUEUE_SIZE);
        let stop_sender = CancellationToken::new();
        let stop = stop_sender.child_token();
        let mut delay_queue = DelayQueue::new();
        delay_queue.insert(
            Refresher::Placeholder,
            DELAY_QUEUE_PLACEHOLDER_REFRESH_DURATION,
        );

        let amqp_cbs_link = AmqpCbsLink {
            stop,
            commands,
            active_link_identifiers: HashMap::new(),
            delay_queue,
            cbs_token_provider,
            cbs_client,
        };

        let join_handle = tokio::spawn(amqp_cbs_link.event_loop());
        AmqpCbsLinkHandle {
            command_sender,
            stop_sender,
            join_handle,
        }
    }

    async fn request_authorization_using_cbs(
        &mut self,
        endpoint: impl AsRef<str>,
        resource: impl AsRef<str>,
        required_claims: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<Option<StdInstant>, CbsAuthError> {
        let resource = resource.as_ref();
        let token = self
            .cbs_token_provider
            .get_token_async(endpoint, resource, required_claims)
            .await?;

        // find the smallest timeout
        let expires_at_utc = token.expires_at_utc().clone().map(OffsetDateTime::from);

        // TODO: Is there any way to convert directly from OffsetDateTime/Timestamp to StdInstant?
        let expires_at_instant = expires_at_utc.map(|expires_at| {
            let now_instant = time::Instant::now();
            let now = OffsetDateTime::now_utc(); // TODO: is there any way to convert instant to datetime?
            let timespan = expires_at - now;
            now_instant + timespan
        });

        self.cbs_client.put_token(resource, token).await?;

        Ok(expires_at_instant.map(|t| t.into_inner()))
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
                            if expires_at > StdInstant::now() {
                                let link_identifier = auth.link_identifier;
                                let when = tokio::time::Instant::from_std(expires_at);
                                let key = self
                                    .delay_queue
                                    .insert_at(Refresher::Authorization(auth), when);
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
                            if expires_at > StdInstant::now() {
                                let when = tokio::time::Instant::from_std(expires_at);
                                let key = self
                                    .delay_queue
                                    .insert_at(Refresher::Authorization(auth), when);
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

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_delay_queue() {
        use futures_util::StreamExt;
        use std::time::Duration;
        use tokio_util::time::DelayQueue;

        let mut delay_queue = DelayQueue::new();
        delay_queue.insert("a", Duration::from_secs(1));
        delay_queue.insert("b", Duration::from_secs(2));
        delay_queue.insert("c", Duration::from_secs(3));

        let mut count = 0;
        while let Some(_) = delay_queue.next().await {
            count += 1;
        }

        assert_eq!(count, 3);
    }

    // TODO: mock tests
}
