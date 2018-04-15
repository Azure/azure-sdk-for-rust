use futures::future::*;
use tokio_core;

use azure::core::errors::AzureError;
use azure::service_bus::event_hub::send_event;

use time::Duration;

use crypto::hmac::Hmac;
use crypto::sha2::Sha256;

pub struct Client {
    handle: tokio_core::reactor::Handle,
    namespace: String,
    event_hub: String,
    policy_name: String,
    hmac: Hmac<Sha256>,
}

impl Client {
    pub fn new(
        handle: tokio_core::reactor::Handle,
        namespace: &str,
        event_hub: &str,
        policy_name: &str,
        key: &str,
    ) -> Client {
        let mut v_hmac_key: Vec<u8> = Vec::new();
        v_hmac_key.extend(key.as_bytes());
        let hmac = Hmac::new(Sha256::new(), &v_hmac_key);

        Client {
            handle: handle,
            namespace: namespace.to_owned(),
            event_hub: event_hub.to_owned(),
            policy_name: policy_name.to_owned(),
            hmac: hmac,
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
                &mut self.hmac,
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

    #[test]
    pub fn client_enc() {
        use base64;
        use crypto::mac::Mac;
        use tokio_core::reactor::Core;

        let str_to_sign = "This must be secret!";

        let core = Core::new().unwrap();
        let mut c = Client::new(core.handle(), "namespace", "event_hub", "policy", "key");

        c.hmac.input(str_to_sign.as_bytes());
        let sig = base64::encode(c.hmac.result().code());

        assert_eq!(sig, "2UNXaoPpeJBAhh6qxmTqXyNzTpOflGO6IhxegeUQBcU=");
    }
}
