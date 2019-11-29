use crate::event_hub::send_event;
use azure_sdk_core::errors::AzureError;
use hyper;
use hyper_rustls::HttpsConnector;
use ring::hmac::Key;
use time::Duration;

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
        let http_client = hyper::Client::builder().build(HttpsConnector::new());

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
        {
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
