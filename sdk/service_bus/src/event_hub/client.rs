use crate::event_hub::{
    delete_message, peek_lock, peek_lock_full, receive_and_delete, renew_lock, send_event,
    unlock_message, PeekLockResponse,
};
use azure_core::errors::AzureError;
use chrono::Duration;
use hyper_rustls::HttpsConnector;
use ring::hmac::Key;

type HttpClient = hyper::Client<HttpsConnector<hyper::client::HttpConnector>>;

pub struct Client {
    namespace: String,
    event_hub: String,
    policy_name: String,
    signing_key: Key,
    http_client: HttpClient,
}

impl Client {
    pub fn new<N, E, P, K>(
        namespace: N,
        event_hub: E,
        policy_name: P,
        key: K,
    ) -> Result<Client, AzureError>
    where
        N: Into<String>,
        E: Into<String>,
        P: Into<String>,
        K: AsRef<str>,
    {
        let signing_key = Key::new(ring::hmac::HMAC_SHA256, key.as_ref().as_bytes());
        let http_client = hyper::Client::builder().build(HttpsConnector::with_native_roots());

        Ok(Client {
            namespace: namespace.into(),
            event_hub: event_hub.into(),
            policy_name: policy_name.into(),
            signing_key,
            http_client,
        })
    }

    pub async fn send_event(
        &mut self,
        event_body: &str,
        duration: Duration,
    ) -> Result<(), AzureError> {
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
    ) -> Result<String, AzureError> {
        peek_lock(
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

    pub async fn peek_lock_full(
        &mut self,
        duration: Duration,
        timeout: Option<Duration>,
    ) -> Result<PeekLockResponse, AzureError> {
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

    pub async fn receive_and_delete(&mut self, duration: Duration) -> Result<String, AzureError> {
        receive_and_delete(
            &self.http_client,
            &self.namespace,
            &self.event_hub,
            &self.policy_name,
            &self.signing_key,
            duration,
        )
        .await
    }

    pub async fn unlock_message(
        &mut self,
        message_id: &str,
        lock_token: &str,
        duration: Duration,
    ) -> Result<(), AzureError> {
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
    ) -> Result<(), AzureError> {
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
    ) -> Result<(), AzureError> {
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

        let c = Client::new("namespace", "event_hub", "policy", "key").unwrap();

        let sig = hmac::sign(&c.signing_key, str_to_sign.as_bytes());
        let sig = ::base64::encode(sig.as_ref());

        assert_eq!(sig, "2UNXaoPpeJBAhh6qxmTqXyNzTpOflGO6IhxegeUQBcU=");
    }
}
