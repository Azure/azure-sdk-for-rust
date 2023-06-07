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

/// Client object that allows interaction with the `ServiceBus` API
#[derive(Debug, Clone)]
pub struct TopicClient {
    http_client: Arc<dyn HttpClient>,
    namespace: String,
    topic: String,
    policy_name: String,
    signing_key: Key,
}

#[derive(Debug, Clone)]
pub struct TopicSender {
    topic_client: TopicClient,
}

#[derive(Debug, Clone)]
pub struct SubscriptionReceiver {
    topic_client: TopicClient,
    subscription: String,
}

impl TopicClient {
    /// Creates a new topic client instance
    pub fn new<N, T, P, K>(
        http_client: Arc<dyn HttpClient>,
        namespace: N,
        topic: T,
        policy_name: P,
        policy_key: K,
    ) -> Result<TopicClient, Error>
    where
        N: Into<String>,
        T: Into<String>,
        P: Into<String>,
        K: AsRef<str>,
    {
        let signing_key = Key::new(ring::hmac::HMAC_SHA256, policy_key.as_ref().as_bytes());

        Ok(Self {
            http_client,
            namespace: namespace.into(),
            topic: topic.into(),
            policy_name: policy_name.into(),
            signing_key,
        })
    }

    pub fn topic_sender(&self) -> TopicSender {
        TopicSender::new(self.clone())
    }

    pub fn subscription_receiver(&self, subscription: &str) -> SubscriptionReceiver {
        SubscriptionReceiver::new(self.clone(), subscription)
    }
}

impl TopicSender {
    pub fn new(topic_client: TopicClient) -> TopicSender {
        Self { topic_client }
    }
    /// Sends a message to the topic
    pub async fn send_message(&self, msg: &str) -> Result<(), Error> {
        send_message(
            &self.topic_client.http_client,
            &self.topic_client.namespace,
            &self.topic_client.topic,
            &self.topic_client.policy_name,
            &self.topic_client.signing_key,
            msg,
        )
        .await
    }
}

impl SubscriptionReceiver {
    pub fn new<S>(topic_client: TopicClient, subscription: S) -> SubscriptionReceiver
    where
        S: Into<String>,
    {
        Self {
            topic_client,
            subscription: subscription.into(),
        }
    }

    /// Receive and delete a message
    pub async fn receive_and_delete_message(&self) -> Result<String, Error> {
        body_bytes_to_utf8(
            receive_and_delete_message(
                &self.topic_client.http_client,
                &self.topic_client.namespace,
                &self.topic_client.topic,
                &self.topic_client.policy_name,
                &self.topic_client.signing_key,
                Some(&self.subscription),
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
                &self.topic_client.http_client,
                &self.topic_client.namespace,
                &self.topic_client.topic,
                &self.topic_client.policy_name,
                &self.topic_client.signing_key,
                lock_expiry,
                Some(&self.subscription),
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
            &self.topic_client.http_client,
            &self.topic_client.namespace,
            &self.topic_client.topic,
            &self.topic_client.policy_name,
            &self.topic_client.signing_key,
            timeout,
            Some(&self.subscription),
        )
        .await
    }
}
