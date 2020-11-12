use azure_core::errors::{AzureError, UnexpectedHTTPResult};
use azure_core::HttpClient;
use chrono::Duration;
use http::{header, status::StatusCode, Request, Response};
use ring::hmac::Key;
use url::{form_urlencoded, Url};

//type HttpClient = hyper::Client<HttpsConnector<hyper::client::HttpConnector>>;

pub struct Client {
    namespace: String,
    event_hub: String,
    policy_name: String,
    signing_key: Key,
    http_client: Box<dyn HttpClient>,
}

impl Client {
    pub fn new<N, E, P, K>(
        http_client: Box<dyn HttpClient>,
        namespace: N,
        event_hub: E,
        policy_name: P,
        key: K,
    ) -> Result<Self, AzureError>
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

    pub async fn send_event(
        &mut self,
        event_body: &str,
        duration: Duration,
    ) -> Result<Response<Vec<u8>>, Box<dyn std::error::Error + Send + Sync>> {
        // prepare the url to call
        let url = format!(
            "https://{}.servicebus.windows.net/{}/messages",
            self.namespace, self.event_hub
        );
        debug!("url == {:?}", url);

        // generate sas signature based on key name, key value, url and duration.
        let sas = crate::event_hub::generate_signature(
            &self.policy_name,
            &self.signing_key,
            &url,
            duration,
        );
        debug!("sas == {}", sas);

        let event_body: &str = event_body.into();
        let event_body = event_body.as_bytes();

        let request = Request::builder()
            .uri(url)
            .header(header::AUTHORIZATION, sas)
            .body(event_body)?;

        Ok(self
            .http_client
            .execute_request_check_status(request, StatusCode::OK)
            .await?)
    }

    pub async fn peek_lock(
        &mut self,
        duration: Duration,
        timeout: Option<Duration>,
    ) -> Result<Response<Vec<u8>>, Box<dyn std::error::Error + Send + Sync>> {
        // prepare the url to call
        let mut url = Url::parse(&format!(
            "https://{}.servicebus.windows.net/{}/messages/head",
            self.namespace, self.event_hub
        ))?;
        if let Some(t) = timeout {
            url.query_pairs_mut()
                .append_pair("timeout", &t.num_seconds().to_string());
        }
        debug!("url == {:?}", url);

        // generate sas signature based on key name, key value, url and duration.
        let sas = crate::event_hub::generate_signature(
            &self.policy_name,
            &self.signing_key,
            &url.as_str(),
            duration,
        );
        debug!("sas == {}", sas);

        let request = Request::builder()
            .uri(url.as_str())
            .method("POST")
            .header(header::AUTHORIZATION, sas)
            .header(header::CONTENT_LENGTH, 0)
            .body(azure_core::EMPTY_BODY.as_ref())?;

        let response = self
            .http_client
            .execute_request_check_statuses(
                request,
                &vec![StatusCode::CREATED, StatusCode::NO_CONTENT],
            )
            .await?;

        Ok(response)
    }

    //pub async fn peek_lock_full(
    //    &mut self,
    //    duration: Duration,
    //    timeout: Option<Duration>,
    //) -> Result<PeekLockResponse, AzureError> {
    //    peek_lock_full(
    //        self.http_client.as_ref(),
    //        &self.namespace,
    //        &self.event_hub,
    //        &self.policy_name,
    //        &self.signing_key,
    //        duration,
    //        timeout,
    //    )
    //    .await
    //}

    //pub async fn receive_and_delete(&mut self, duration: Duration) -> Result<String, AzureError> {
    //    receive_and_delete(
    //        self.http_client.as_ref(),
    //        &self.namespace,
    //        &self.event_hub,
    //        &self.policy_name,
    //        &self.signing_key,
    //        duration,
    //    )
    //    .await
    //}

    //pub async fn unlock_message(
    //    &mut self,
    //    message_id: &str,
    //    lock_token: &str,
    //    duration: Duration,
    //) -> Result<(), AzureError> {
    //    unlock_message(
    //        self.http_client.as_ref(),
    //        &self.namespace,
    //        &self.event_hub,
    //        &self.policy_name,
    //        &self.signing_key,
    //        duration,
    //        message_id,
    //        lock_token,
    //    )
    //    .await
    //}

    //pub async fn delete_message(
    //    &mut self,
    //    message_id: &str,
    //    lock_token: &str,
    //    duration: Duration,
    //) -> Result<(), AzureError> {
    //    delete_message(
    //        self.http_client.as_ref(),
    //        &self.namespace,
    //        &self.event_hub,
    //        &self.policy_name,
    //        &self.signing_key,
    //        duration,
    //        message_id,
    //        lock_token,
    //    )
    //    .await
    //}

    //pub async fn renew_lock(
    //    &mut self,
    //    message_id: &str,
    //    lock_token: &str,
    //    duration: Duration,
    //) -> Result<(), AzureError> {
    //    renew_lock(
    //        self.http_client.as_ref(),
    //        &self.namespace,
    //        &self.event_hub,
    //        &self.policy_name,
    //        &self.signing_key,
    //        duration,
    //        message_id,
    //        lock_token,
    //    )
    //    .await
    //}
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::Client;
    use ring::hmac;

    #[test]
    pub fn client_enc() {
        let str_to_sign = "This must be secret!";

        let c = Client::new(
            Box::new(reqwest::Client::new()),
            "namespace",
            "event_hub",
            "policy",
            "key",
        )
        .unwrap();

        let sig = hmac::sign(&c.signing_key, str_to_sign.as_bytes());
        let sig = ::base64::encode(sig.as_ref());

        assert_eq!(sig, "2UNXaoPpeJBAhh6qxmTqXyNzTpOflGO6IhxegeUQBcU=");
    }
}
