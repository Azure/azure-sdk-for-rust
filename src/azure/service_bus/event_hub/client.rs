use azure::service_bus::event_hub::send_event;
use azure::core::errors::AzureError;

use time::Duration;
use std::io::Read;

use crypto::sha2::Sha256;
use crypto::hmac::Hmac;

pub struct Client {
    namespace: String,
    event_hub: String,
    policy_name: String,
    hmac: Hmac<Sha256>,
}

impl Client {
    pub fn new(namespace: &str, event_hub: &str, policy_name: &str, key: &str) -> Client {
        let mut v_hmac_key: Vec<u8> = Vec::new();
        v_hmac_key.extend(key.as_bytes());
        let hmac = Hmac::new(Sha256::new(), &v_hmac_key);

        Client {
            namespace: namespace.to_owned(),
            event_hub: event_hub.to_owned(),
            policy_name: policy_name.to_owned(),
            hmac: hmac,
        }
    }

    pub fn send_event(&mut self,
                      event_body: &mut (&mut Read, u64),
                      duration: Duration)
                      -> Result<(), AzureError> {
        send_event(&self.namespace,
                   &self.event_hub,
                   &self.policy_name,
                   &mut self.hmac,
                   event_body,
                   duration)
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::Client;

    #[test]
    pub fn client_ctor() {
        Client::new("namespace", "event_hub", "policy", "key");
    }

    #[test]
    pub fn client_enc() {
        use crypto::mac::Mac;
        use base64;

        let str_to_sign = "This must be secret!";

        let mut c = Client::new("namespace", "event_hub", "policy", "key");

        c.hmac.input(str_to_sign.as_bytes());
        let sig = base64::encode(c.hmac.result().code());

        assert_eq!(sig, "2UNXaoPpeJBAhh6qxmTqXyNzTpOflGO6IhxegeUQBcU=");
    }
}
