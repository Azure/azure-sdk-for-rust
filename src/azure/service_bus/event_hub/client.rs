use futures::future::*;

use hyper;
use azure::core::errors::AzureError;
use azure::service_bus::event_hub::send_event;

use time::Duration;

use ring::{digest::SHA256, hmac::SigningKey};

type HttpClient = hyper::Client<::hyper_tls::HttpsConnector<hyper::client::HttpConnector>>;

pub struct Client {
    namespace: String,
    event_hub: String,
    policy_name: String,
    signing_key: SigningKey,
    http_client: HttpClient
}

impl Client {
    pub fn new<N, E, P, K>(
        namespace: N,
        event_hub: E,
        policy_name: P,
        key: K,
    ) -> Result<Client, AzureError>
    where N: Into<String>, E: Into<String>, P: Into<String>, K: AsRef<str>
    {
        let signing_key = SigningKey::new(&SHA256, key.as_ref().as_bytes());
        let http_client = hyper::Client::builder().build(::hyper_tls::HttpsConnector::new(4)?);

        Ok(Client {
            namespace: namespace.into(),
            event_hub: event_hub.into(),
            policy_name: policy_name.into(),
            signing_key,
            http_client
        })
    }

    pub fn send_event(
        &mut self,
        event_body: &str,
        duration: Duration,
    ) -> impl Future<Item = (), Error = AzureError> {
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