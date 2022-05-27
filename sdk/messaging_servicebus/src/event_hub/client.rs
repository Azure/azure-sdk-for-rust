use std::sync::Arc;

use crate::event_hub::{
    delete_message, peek_lock, peek_lock_full, receive_and_delete, renew_lock, send_event,
    unlock_message, PeekLockResponse,
};
use chrono::Duration;
use ring::hmac::Key;

use azure_core::HttpClient;

pub struct Client {
    namespace: String,
    event_hub: String,
    policy_name: String,
    signing_key: Key,
    http_client: Arc<dyn HttpClient>,
}

impl Client {
    pub fn new<N, E, P, K>(
        http_client: Arc<dyn HttpClient>,
        namespace: N,
        event_hub: E,
        policy_name: P,
        key: K,
    ) -> Result<Client, azure_core::Error>
    where
        N: Into<String>,
        E: Into<String>,
        P: Into<String>,
        K: AsRef<str>,
    {
        let signing_key = Key::new(ring::hmac::HMAC_SHA256, key.as_ref().as_bytes());

        Ok(Client {
            namespace: namespace.into(),
            event_hub: event_hub.into(),
            policy_name: policy_name.into(),
            signing_key,
            http_client,
        })
    }

    pub async fn send_event(&mut self, event_body: &str, duration: Duration) -> crate::Result<()> {
        send_event(
            &self.http_client,
            &self.namespace,
            &self.event_hub,
            &self.policy_name,
            &self.signing_key,
            event_body,
            duration,
        )
        .await
    }

    pub async fn peek_lock(
        &mut self,
        duration: Duration,
        timeout: Option<Duration>,
    ) -> crate::Result<String> {
        Ok(std::str::from_utf8(
            &peek_lock(
                &self.http_client,
                &self.namespace,
                &self.event_hub,
                &self.policy_name,
                &self.signing_key,
                duration,
                timeout,
            )
            .await?
            .into_body(),
        )?
        .to_string())
    }

    pub async fn peek_lock_full(
        &mut self,
        duration: Duration,
        timeout: Option<Duration>,
    ) -> crate::Result<PeekLockResponse> {
        peek_lock_full(
            &self.http_client,
            &self.namespace,
            &self.event_hub,
            &self.policy_name,
            &self.signing_key,
            duration,
            timeout,
        )
        .await
    }

    pub async fn receive_and_delete(&mut self, duration: Duration) -> crate::Result<String> {
        Ok(std::str::from_utf8(
            &receive_and_delete(
                &self.http_client,
                &self.namespace,
                &self.event_hub,
                &self.policy_name,
                &self.signing_key,
                duration,
            )
            .await?
            .into_body(),
        )?
        .to_string())
    }

    pub async fn unlock_message(
        &mut self,
        message_id: &str,
        lock_token: &str,
        duration: Duration,
    ) -> crate::Result<()> {
        unlock_message(
            &self.http_client,
            &self.namespace,
            &self.event_hub,
            &self.policy_name,
            &self.signing_key,
            duration,
            message_id,
            lock_token,
        )
        .await
    }

    pub async fn delete_message(
        &mut self,
        message_id: &str,
        lock_token: &str,
        duration: Duration,
    ) -> crate::Result<()> {
        delete_message(
            &self.http_client,
            &self.namespace,
            &self.event_hub,
            &self.policy_name,
            &self.signing_key,
            duration,
            message_id,
            lock_token,
        )
        .await
    }

    pub async fn renew_lock(
        &mut self,
        message_id: &str,
        lock_token: &str,
        duration: Duration,
    ) -> crate::Result<()> {
        renew_lock(
            &self.http_client,
            &self.namespace,
            &self.event_hub,
            &self.policy_name,
            &self.signing_key,
            duration,
            message_id,
            lock_token,
        )
        .await
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::Client;
    use ring::hmac;

    #[test]
    pub fn client_enc() {
        let str_to_sign = "This must be secret!";

        let http_client = azure_core::new_http_client();

        let c = Client::new(http_client, "namespace", "event_hub", "policy", "key").unwrap();

        let sig = hmac::sign(&c.signing_key, str_to_sign.as_bytes());
        let sig = ::base64::encode(sig.as_ref());

        assert_eq!(sig, "2UNXaoPpeJBAhh6qxmTqXyNzTpOflGO6IhxegeUQBcU=");
    }
}
