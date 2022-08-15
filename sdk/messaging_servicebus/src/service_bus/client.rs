use std::sync::Arc;

use crate::{
    service_bus::{
        peek_lock_message, peek_lock_message2, receive_and_delete_message, send_message,
        PeekLockResponse,
    },
    utils::body_bytes_to_utf8,
};
use ring::hmac::Key;
use std::time::Duration;

use azure_core::{error::Error, HttpClient};

/// Client object that allows interaction with the ServiceBus API
#[derive(Debug, Clone)]
pub struct Client {
    http_client: Arc<dyn HttpClient>,
    namespace: String,
    queue: String,
    policy_name: String,
    signing_key: Key,
}

impl Client {
    /// Creates a new client instance
    pub fn new<N, Q, P, K>(
        http_client: Arc<dyn HttpClient>,
        namespace: N,
        queue: Q,
        policy_name: P,
        policy_key: K,
    ) -> Result<Client, Error>
    where
        N: Into<String>,
        Q: Into<String>,
        P: Into<String>,
        K: AsRef<str>,
    {
        let signing_key = Key::new(ring::hmac::HMAC_SHA256, policy_key.as_ref().as_bytes());

        Ok(Client {
            http_client,
            namespace: namespace.into(),
            queue: queue.into(),
            policy_name: policy_name.into(),
            signing_key,
        })
    }

    /// Sends a message to the queue
    pub async fn send_message(&self, msg: &str) -> Result<(), Error> {
        send_message(
            &self.http_client,
            &self.namespace,
            &self.queue,
            &self.policy_name,
            &self.signing_key,
            msg,
        )
        .await
    }

    /// Receive and delete a message
    pub async fn receive_and_delete_message(&self) -> Result<String, Error> {
        body_bytes_to_utf8(
            receive_and_delete_message(
                &self.http_client,
                &self.namespace,
                &self.queue,
                &self.policy_name,
                &self.signing_key,
            )
            .await?
            .body(),
        )
    }

    /// Non-destructively read a message
    ///
    /// Note: This function does not return the delete location
    /// of the message, so, after reading, you will lose
    /// "track" of it until the lock expiry runs out and
    /// the message can be consumed by others. If you want to keep
    /// track of this message (i.e., have the possibility of deletion),
    /// use `peek_lock_message2`.
    pub async fn peek_lock_message(&self, lock_expiry: Option<Duration>) -> Result<String, Error> {
        body_bytes_to_utf8(
            peek_lock_message(
                &self.http_client,
                &self.namespace,
                &self.queue,
                &self.policy_name,
                &self.signing_key,
                lock_expiry,
            )
            .await?
            .body(),
        )
    }

    /// Non-destructively read a message but track it
    ///
    /// Note: This function returns a `PeekLockResponse`
    /// that contains a helper `delete_message` function.
    pub async fn peek_lock_message2(
        &self,
        timeout: Option<Duration>,
    ) -> Result<PeekLockResponse, Error> {
        peek_lock_message2(
            &self.http_client,
            &self.namespace,
            &self.queue,
            &self.policy_name,
            &self.signing_key,
            timeout,
        )
        .await
    }
}
