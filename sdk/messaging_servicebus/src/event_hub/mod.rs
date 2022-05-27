#![allow(clippy::too_many_arguments)]
use azure_core::{headers::MS_DATE, HttpClient};
use bytes::Bytes;
use chrono::Duration;
use http::{
    header::{AUTHORIZATION, CONTENT_LENGTH},
    Request, Response,
};
use ring::hmac;
use std::{ops::Add, sync::Arc};
use url::{form_urlencoded, Url};

mod client;
pub use self::client::Client;

#[inline]
fn send_event_prepare<B: Into<String>>(
    namespace: &str,
    event_hub: &str,
    policy_name: &str,
    signing_key: &hmac::Key,
    event_body: B,
    duration: Duration,
) -> crate::Result<http::Request<Bytes>> {
    // prepare the url to call
    let url = format!(
        "https://{}.servicebus.windows.net/{}/messages",
        namespace, event_hub
    );
    debug!("url == {:?}", url);

    // generate sas signature based on key name, key value, url and duration.
    let sas = generate_signature(policy_name, signing_key, &url, duration);
    debug!("sas == {}", sas);

    let event_body = event_body.into();

    let dt = chrono::Utc::now();
    let time = format!("{}", dt.format("%a, %d %h %Y %T GMT"));
    let mut request = Request::builder();
    request = request.method(http::Method::POST).uri(url.as_str());
    request = request
        .header(MS_DATE, time)
        .header("x-ms-version", "2019-12-12")
        .header(AUTHORIZATION, sas)
        .header(CONTENT_LENGTH, "0");
    let request = request.body(event_body.into())?;

    Ok(request)
}

async fn send_event(
    http_client: &Arc<dyn HttpClient>,
    namespace: &str,
    event_hub: &str,
    policy_name: &str,
    hmac: &hmac::Key,
    event_body: &str,
    duration: Duration,
) -> crate::Result<()> {
    let req = send_event_prepare(
        namespace,
        event_hub,
        policy_name,
        hmac,
        event_body,
        duration,
    )?;

    http_client
        .execute_request_check_status(req, http::StatusCode::CREATED)
        .await?;
    Ok(())
}

#[inline]
fn peek_lock_prepare(
    namespace: &str,
    event_hub: &str,
    policy_name: &str,
    signing_key: &hmac::Key,
    duration: Duration,
    timeout: Option<Duration>,
) -> crate::Result<http::Request<Bytes>> {
    // prepare the url to call
    let mut url = Url::parse(&format!(
        "https://{}.servicebus.windows.net/{}/messages/head",
        namespace, event_hub
    ))?;

    if let Some(t) = timeout {
        url.query_pairs_mut()
            .append_pair("timeout", &t.num_seconds().to_string());
    }
    debug!("url == {:?}", url);

    // generate sas signature based on key name, key value, url and duration.
    let sas = generate_signature(policy_name, signing_key, url.as_str(), duration);
    debug!("sas == {}", sas);

    let dt = chrono::Utc::now();
    let time = format!("{}", dt.format("%a, %d %h %Y %T GMT"));
    let mut request = Request::builder();
    request = request.method(http::Method::POST).uri(url.as_str());
    request = request
        .header(MS_DATE, time)
        .header("x-ms-version", "2019-12-12")
        .header(AUTHORIZATION, sas)
        .header(CONTENT_LENGTH, "0");
    let request = request.body(azure_core::EMPTY_BODY)?;

    Ok(request)
}

async fn peek_lock(
    http_client: &Arc<dyn HttpClient>,
    namespace: &str,
    event_hub: &str,
    policy_name: &str,
    hmac: &hmac::Key,
    duration: Duration,
    timeout: Option<Duration>,
) -> crate::Result<Response<Bytes>> {
    let req = peek_lock_prepare(namespace, event_hub, policy_name, hmac, duration, timeout)?;

    Ok(http_client
        .execute_request_check_status(req, http::StatusCode::CREATED)
        .await?)
}

async fn peek_lock_full(
    http_client: &Arc<dyn HttpClient>,
    namespace: &str,
    event_hub: &str,
    policy_name: &str,
    hmac: &hmac::Key,
    duration: Duration,
    timeout: Option<Duration>,
) -> crate::Result<PeekLockResponse> {
    let req = peek_lock_prepare(namespace, event_hub, policy_name, hmac, duration, timeout)?;

    let res = http_client.execute_request(req).await?;

    let status = res.status();
    let location: String = match res.headers().get("Location") {
        Some(header_value) => header_value.to_str()?.to_owned(),
        _ => "".to_owned(),
    };
    let body = std::str::from_utf8(res.body())?.to_string();

    Ok(PeekLockResponse {
        http_client: http_client.clone(),
        status,
        delete_location: location,
        body,
        duration,
        policy_name: policy_name.to_owned(),
        signing_key: hmac.to_owned(),
    })
}

pub struct PeekLockResponse {
    http_client: Arc<dyn HttpClient>,
    status: http::StatusCode,
    delete_location: String,
    body: String,
    policy_name: String,
    signing_key: hmac::Key,
    duration: Duration,
}

impl PeekLockResponse {
    pub fn body(&self) -> String {
        self.body.clone()
    }
    pub fn status(&self) -> http::StatusCode {
        self.status
    }
    pub async fn delete_message(&self) -> crate::Result<Response<Bytes>> {
        let req = delete_message_get_request(
            &self.policy_name,
            &self.signing_key,
            self.duration,
            self.delete_location.clone(),
        )?;

        Ok(self
            .http_client
            .execute_request_check_status(req, http::StatusCode::CREATED)
            .await?)
    }
}

fn receive_and_delete_prepare(
    namespace: &str,
    event_hub: &str,
    policy_name: &str,
    signing_key: &hmac::Key,
    duration: Duration,
) -> crate::Result<Request<Bytes>> {
    // prepare the url to call
    let url = format!(
        "https://{}.servicebus.windows.net/{}/messages/head",
        namespace, event_hub
    );
    debug!("url == {:?}", url);

    // generate sas signature based on key name, key value, url and duration.
    let sas = generate_signature(policy_name, signing_key, &url, duration);
    debug!("sas == {}", sas);

    let dt = chrono::Utc::now();
    let time = format!("{}", dt.format("%a, %d %h %Y %T GMT"));
    let mut request = Request::builder();
    request = request.method(http::Method::DELETE).uri(url.as_str());
    request = request
        .header(MS_DATE, time)
        .header("x-ms-version", "2019-12-12")
        .header(AUTHORIZATION, sas)
        .header(CONTENT_LENGTH, "0");
    let request = request.body(azure_core::EMPTY_BODY)?;

    Ok(request)
}

async fn receive_and_delete(
    http_client: &Arc<dyn HttpClient>,
    namespace: &str,
    event_hub: &str,
    policy_name: &str,
    hmac: &hmac::Key,
    duration: Duration,
) -> crate::Result<Response<Bytes>> {
    let req = receive_and_delete_prepare(namespace, event_hub, policy_name, hmac, duration)?;

    Ok(http_client
        .execute_request_check_status(req, http::StatusCode::OK)
        .await?)
}

fn delete_message_prepare(
    namespace: &str,
    event_hub: &str,
    policy_name: &str,
    signing_key: &hmac::Key,
    duration: Duration,
    message_id: &str,
    lock_token: &str,
) -> crate::Result<Request<Bytes>> {
    // prepare the url to call
    let url = format!(
        "https://{}.servicebus.windows.net/{}/messages/{}/{}",
        namespace, event_hub, message_id, lock_token
    );
    debug!("url == {:?}", url);

    // generate sas signature based on key name, key value, url and duration.

    delete_message_get_request(policy_name, signing_key, duration, url)
}

fn delete_message_get_request(
    policy_name: &str,
    signing_key: &hmac::Key,
    duration: Duration,
    url: String,
) -> crate::Result<Request<Bytes>> {
    let sas = generate_signature(policy_name, signing_key, &url, duration);
    debug!("sas == {}", sas);

    let dt = chrono::Utc::now();
    let time = format!("{}", dt.format("%a, %d %h %Y %T GMT"));
    let mut request = Request::builder();
    request = request.method(http::Method::DELETE).uri(url.as_str());
    request = request
        .header(MS_DATE, time)
        .header("x-ms-version", "2019-12-12")
        .header(AUTHORIZATION, sas)
        .header(CONTENT_LENGTH, "0");
    let request = request.body(azure_core::EMPTY_BODY)?;

    Ok(request)
}

async fn delete_message(
    http_client: &Arc<dyn HttpClient>,
    namespace: &str,
    event_hub: &str,
    policy_name: &str,
    hmac: &hmac::Key,
    duration: Duration,
    message_id: &str,
    lock_token: &str,
) -> crate::Result<()> {
    http_client
        .execute_request_check_status(
            delete_message_prepare(
                namespace,
                event_hub,
                policy_name,
                hmac,
                duration,
                message_id,
                lock_token,
            )?,
            http::StatusCode::OK,
        )
        .await?;
    Ok(())
}

fn unlock_message_prepare(
    namespace: &str,
    event_hub: &str,
    policy_name: &str,
    signing_key: &hmac::Key,
    duration: Duration,
    message_id: &str,
    lock_token: &str,
) -> crate::Result<Request<Bytes>> {
    // prepare the url to call
    let url = format!(
        "https://{}.servicebus.windows.net/{}/messages/{}/{}",
        namespace, event_hub, message_id, lock_token
    );
    debug!("url == {:?}", url);

    // generate sas signature based on key name, key value, url and duration.
    let sas = generate_signature(policy_name, signing_key, &url, duration);
    debug!("sas == {}", sas);

    let dt = chrono::Utc::now();
    let time = format!("{}", dt.format("%a, %d %h %Y %T GMT"));
    let mut request = Request::builder();
    request = request.method(http::Method::PUT).uri(url.as_str());
    request = request
        .header(MS_DATE, time)
        .header("x-ms-version", "2019-12-12")
        .header(AUTHORIZATION, sas)
        .header(CONTENT_LENGTH, "0");
    let request = request.body(azure_core::EMPTY_BODY)?;

    Ok(request)
}

async fn unlock_message(
    http_client: &Arc<dyn HttpClient>,
    namespace: &str,
    event_hub: &str,
    policy_name: &str,
    hmac: &hmac::Key,
    duration: Duration,
    message_id: &str,
    lock_token: &str,
) -> crate::Result<()> {
    http_client
        .execute_request_check_status(
            unlock_message_prepare(
                namespace,
                event_hub,
                policy_name,
                hmac,
                duration,
                message_id,
                lock_token,
            )?,
            http::StatusCode::OK,
        )
        .await?;
    Ok(())
}

fn renew_lock_prepare(
    namespace: &str,
    event_hub: &str,
    policy_name: &str,
    signing_key: &hmac::Key,
    duration: Duration,
    message_id: &str,
    lock_token: &str,
) -> crate::Result<Request<Bytes>> {
    // prepare the url to call
    let url = format!(
        "https://{}.servicebus.windows.net/{}/messages/{}/{}",
        namespace, event_hub, message_id, lock_token
    );
    debug!("url == {:?}", url);

    // generate sas signature based on key name, key value, url and duration.
    let sas = generate_signature(policy_name, signing_key, &url, duration);
    debug!("sas == {}", sas);

    let dt = chrono::Utc::now();
    let time = format!("{}", dt.format("%a, %d %h %Y %T GMT"));
    let mut request = Request::builder();
    request = request.method(http::Method::POST).uri(url.as_str());
    request = request
        .header(MS_DATE, time)
        .header("x-ms-version", "2019-12-12")
        .header(AUTHORIZATION, sas)
        .header(CONTENT_LENGTH, "0");
    let request = request.body(azure_core::EMPTY_BODY)?;

    Ok(request)
}

async fn renew_lock(
    http_client: &Arc<dyn HttpClient>,
    namespace: &str,
    event_hub: &str,
    policy_name: &str,
    hmac: &hmac::Key,
    duration: Duration,
    message_id: &str,
    lock_token: &str,
) -> crate::Result<()> {
    http_client
        .execute_request_check_status(
            renew_lock_prepare(
                namespace,
                event_hub,
                policy_name,
                hmac,
                duration,
                message_id,
                lock_token,
            )?,
            http::StatusCode::OK,
        )
        .await?;
    Ok(())
}

fn generate_signature(
    policy_name: &str,
    signing_key: &hmac::Key,
    url: &str,
    ttl: Duration,
) -> String {
    use url::form_urlencoded::Serializer;

    let expiry = ::chrono::Utc::now().add(ttl).timestamp();
    debug!("expiry == {:?}", expiry);

    let url_encoded: String = form_urlencoded::byte_serialize(url.as_bytes()).collect();
    debug!("url_encoded == {:?}", url_encoded);

    let str_to_sign = format!("{}\n{}", url_encoded, expiry);
    debug!("str_to_sign == {:?}", str_to_sign);

    let sig = hmac::sign(signing_key, str_to_sign.as_bytes());
    let sig = {
        let sig = ::base64::encode(sig.as_ref());
        debug!("sig == {}", sig);
        let mut ser = Serializer::new(String::new());
        ser.append_pair("sig", &sig);
        let sig = ser.finish();
        debug!("sig == {}", sig);
        sig
    };

    debug!("sig == {:?}", sig);

    format!(
        "SharedAccessSignature sr={}&{}&se={}&skn={}",
        &url_encoded, sig, expiry, policy_name
    )
}
