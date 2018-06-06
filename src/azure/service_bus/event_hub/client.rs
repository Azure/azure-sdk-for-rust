use futures::future::*;
use tokio_core;

use azure::core::errors::AzureError;
use azure::service_bus::event_hub::send_event;

use time::Duration;

use ring::{digest::SHA256, hmac::SigningKey};

pub struct Client {
    handle: tokio_core::reactor::Handle,
    namespace: String,
    event_hub: String,
    policy_name: String,
    signing_key: SigningKey,
}

impl Client {
    pub fn new(
        handle: tokio_core::reactor::Handle,
        namespace: &str,
        event_hub: &str,
        policy_name: &str,
        key: &str,
    ) -> Client {
        let signing_key = SigningKey::new(&SHA256, key.as_bytes());

        Client {
            handle,
            namespace: namespace.to_owned(),
            event_hub: event_hub.to_owned(),
            policy_name: policy_name.to_owned(),
            signing_key,
        }
    }

    pub fn send_event(
        &mut self,
        event_body: &str,
        duration: Duration,
    ) -> impl Future<Item = (), Error = AzureError> {
        {
            send_event(
                &self.handle.clone(),
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
        use base64;
        use tokio_core::reactor::Core;

        let str_to_sign = "This must be secret!";

        let core = Core::new().unwrap();
        let c = Client::new(core.handle(), "namespace", "event_hub", "policy", "key");

        let sig = hmac::sign(&c.signing_key, str_to_sign.as_bytes());
        let sig = base64::encode(sig.as_ref());

        assert_eq!(sig, "2UNXaoPpeJBAhh6qxmTqXyNzTpOflGO6IhxegeUQBcU=");
    }
}
