use crate::event_hub::{
    delete_message, peek_lock, peek_lock_full, receive_and_delete, renew_lock, send_event,
    unlock_message, PeekLockResponse,
};
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
    ) -> Result<Client, azure_core::Error>
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
    ) -> Result<(), azure_core::Error> {
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
    ) -> Result<String, azure_core::Error> {
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
    ) -> Result<PeekLockResponse, azure_core::Error> {
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

    pub async fn receive_and_delete(&mut self, duration: Duration) -> Result<String, azure_core::Error> {
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
    ) -> Result<(), azure_core::Error> {
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
    ) -> Result<(), azure_core::Error> {
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
    ) -> Result<(), azure_core::Error> {
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
