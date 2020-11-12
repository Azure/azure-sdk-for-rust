use azure_core::errors::{check_status_extract_body, extract_location_status_and_body, AzureError};
use azure_core::HttpClient;
use chrono::Duration;
use http::{Request, Response};
use hyper::{self, header, Body, StatusCode};
use hyper_rustls::HttpsConnector;
use ring::hmac;
use std::ops::Add;
use url::{form_urlencoded, Url};

mod client;
pub use self::client::Client;

//type HttpClient = hyper::Client<HttpsConnector<hyper::client::HttpConnector>>;

//fn peek_lock_prepare(
//    http_client: &dyn HttpClient,
//    namespace: &str,
//    event_hub: &str,
//    policy_name: &str,
//    signing_key: &hmac::Key,
//    duration: Duration,
//    timeout: Option<Duration>,
//) -> Result<hyper::client::ResponseFuture, AzureError> {
//    // prepare the url to call
//    let mut url = Url::parse(&format!(
//        "https://{}.servicebus.windows.net/{}/messages/head",
//        namespace, event_hub
//    ))?;
//    if let Some(t) = timeout {
//        url.query_pairs_mut()
//            .append_pair("timeout", &t.num_seconds().to_string());
//    }
//    debug!("url == {:?}", url);
//
//    // generate sas signature based on key name, key value, url and duration.
//    let sas = generate_signature(policy_name, signing_key, &url.as_str(), duration);
//    debug!("sas == {}", sas);
//
//    let request = hyper::Request::post(url.into_string())
//        .header(header::AUTHORIZATION, sas)
//        .header(header::CONTENT_LENGTH, 0)
//        .body(Body::empty())?;
//
//    Ok(http_client.request(request))
//}
//
//async fn peek_lock(
//    http_client: &dyn HttpClient,
//    namespace: &str,
//    event_hub: &str,
//    policy_name: &str,
//    hmac: &hmac::Key,
//    duration: Duration,
//    timeout: Option<Duration>,
//) -> Result<String, AzureError> {
//    let req = peek_lock_prepare(
//        http_client,
//        namespace,
//        event_hub,
//        policy_name,
//        hmac,
//        duration,
//        timeout,
//    );
//
//    check_status_extract_body(req?, StatusCode::CREATED).await
//}
//
//async fn peek_lock_full(
//    http_client: &dyn HttpClient,
//    namespace: &str,
//    event_hub: &str,
//    policy_name: &str,
//    hmac: &hmac::Key,
//    duration: Duration,
//    timeout: Option<Duration>,
//) -> Result<PeekLockResponse, AzureError> {
//    let req = peek_lock_prepare(
//        http_client,
//        namespace,
//        event_hub,
//        policy_name,
//        hmac,
//        duration,
//        timeout,
//    );
//
//    let a = extract_location_status_and_body(req?).await?;
//
//    Ok(PeekLockResponse {
//        http_client: http_client.to_owned(),
//        status: a.0,
//        delete_location: a.1,
//        body: a.2,
//        duration: duration.clone(),
//        policy_name: policy_name.to_owned(),
//        signing_key: hmac.to_owned(),
//    })
//}

//pub struct PeekLockResponse {
//    http_client: &dyn HttpClient,
//    status: StatusCode,
//    delete_location: String,
//    body: String,
//    policy_name: String,
//    signing_key: hmac::Key,
//    duration: Duration,
//}
//
//impl PeekLockResponse {
//    pub fn body(&self) -> String {
//        self.body.clone()
//    }
//    pub fn status(&self) -> StatusCode {
//        self.status
//    }
//    pub async fn delete_message(&self) -> Result<String, AzureError> {
//        let req = delete_message_get_request(
//            &self.http_client,
//            &self.policy_name,
//            &self.signing_key,
//            self.duration,
//            self.delete_location.clone(),
//        );
//
//        check_status_extract_body(req?, StatusCode::OK).await
//    }
//}

//fn receive_and_delete_prepare(
//    http_client: &HttpClient,
//    namespace: &str,
//    event_hub: &str,
//    policy_name: &str,
//    signing_key: &hmac::Key,
//    duration: Duration,
//) -> Result<hyper::client::ResponseFuture, AzureError> {
//    // prepare the url to call
//    let url = format!(
//        "https://{}.servicebus.windows.net/{}/messages/head",
//        namespace, event_hub
//    );
//    debug!("url == {:?}", url);
//
//    // generate sas signature based on key name, key value, url and duration.
//    let sas = generate_signature(policy_name, signing_key, &url, duration);
//    debug!("sas == {}", sas);
//
//    let request = hyper::Request::delete(url)
//        .header(header::AUTHORIZATION, sas)
//        .body(Body::empty())?;
//
//    Ok(http_client.request(request))
//}

//async fn receive_and_delete(
//    http_client: &HttpClient,
//    namespace: &str,
//    event_hub: &str,
//    policy_name: &str,
//    hmac: &hmac::Key,
//    duration: Duration,
//) -> Result<String, AzureError> {
//    let req = receive_and_delete_prepare(
//        http_client,
//        namespace,
//        event_hub,
//        policy_name,
//        hmac,
//        duration,
//    );
//
//    check_status_extract_body(req?, StatusCode::OK).await
//}
//
//fn delete_message_prepare(
//    http_client: &HttpClient,
//    namespace: &str,
//    event_hub: &str,
//    policy_name: &str,
//    signing_key: &hmac::Key,
//    duration: Duration,
//    message_id: &str,
//    lock_token: &str,
//) -> Result<hyper::client::ResponseFuture, AzureError> {
//    // prepare the url to call
//    let url = format!(
//        "https://{}.servicebus.windows.net/{}/messages/{}/{}",
//        namespace, event_hub, message_id, lock_token
//    );
//    debug!("url == {:?}", url);
//
//    // generate sas signature based on key name, key value, url and duration.
//
//    delete_message_get_request(http_client, policy_name, signing_key, duration, url)
//}
//
//fn delete_message_get_request(
//    http_client: &HttpClient,
//    policy_name: &str,
//    signing_key: &hmac::Key,
//    duration: Duration,
//    url: String,
//) -> Result<hyper::client::ResponseFuture, AzureError> {
//    let sas = generate_signature(policy_name, signing_key, &url, duration);
//    debug!("sas == {}", sas);
//
//    let request = hyper::Request::delete(url)
//        .header(header::AUTHORIZATION, sas)
//        .body(Body::empty())?;
//
//    Ok(http_client.request(request))
//}
//
//async fn delete_message(
//    http_client: &HttpClient,
//    namespace: &str,
//    event_hub: &str,
//    policy_name: &str,
//    hmac: &hmac::Key,
//    duration: Duration,
//    message_id: &str,
//    lock_token: &str,
//) -> Result<(), AzureError> {
//    check_status_extract_body(
//        delete_message_prepare(
//            http_client,
//            namespace,
//            event_hub,
//            policy_name,
//            hmac,
//            duration,
//            message_id,
//            lock_token,
//        )?,
//        StatusCode::OK,
//    )
//    .await?;
//    Ok(())
//}
//
//fn unlock_message_prepare(
//    http_client: &HttpClient,
//    namespace: &str,
//    event_hub: &str,
//    policy_name: &str,
//    signing_key: &hmac::Key,
//    duration: Duration,
//    message_id: &str,
//    lock_token: &str,
//) -> Result<hyper::client::ResponseFuture, AzureError> {
//    // prepare the url to call
//    let url = format!(
//        "https://{}.servicebus.windows.net/{}/messages/{}/{}",
//        namespace, event_hub, message_id, lock_token
//    );
//    debug!("url == {:?}", url);
//
//    // generate sas signature based on key name, key value, url and duration.
//    let sas = generate_signature(policy_name, signing_key, &url, duration);
//    debug!("sas == {}", sas);
//
//    let request = hyper::Request::put(url)
//        .header(header::AUTHORIZATION, sas)
//        .body(Body::empty())?;
//
//    Ok(http_client.request(request))
//}
//
//async fn unlock_message(
//    http_client: &HttpClient,
//    namespace: &str,
//    event_hub: &str,
//    policy_name: &str,
//    hmac: &hmac::Key,
//    duration: Duration,
//    message_id: &str,
//    lock_token: &str,
//) -> Result<(), AzureError> {
//    check_status_extract_body(
//        unlock_message_prepare(
//            http_client,
//            namespace,
//            event_hub,
//            policy_name,
//            hmac,
//            duration,
//            message_id,
//            lock_token,
//        )?,
//        StatusCode::OK,
//    )
//    .await?;
//    Ok(())
//}
//
//fn renew_lock_prepare(
//    http_client: &HttpClient,
//    namespace: &str,
//    event_hub: &str,
//    policy_name: &str,
//    signing_key: &hmac::Key,
//    duration: Duration,
//    message_id: &str,
//    lock_token: &str,
//) -> Result<hyper::client::ResponseFuture, AzureError> {
//    // prepare the url to call
//    let url = format!(
//        "https://{}.servicebus.windows.net/{}/messages/{}/{}",
//        namespace, event_hub, message_id, lock_token
//    );
//    debug!("url == {:?}", url);
//
//    // generate sas signature based on key name, key value, url and duration.
//    let sas = generate_signature(policy_name, signing_key, &url, duration);
//    debug!("sas == {}", sas);
//
//    let request = hyper::Request::post(url)
//        .header(header::AUTHORIZATION, sas)
//        .body(Body::empty())?;
//
//    Ok(http_client.request(request))
//}
//
//async fn renew_lock(
//    http_client: &HttpClient,
//    namespace: &str,
//    event_hub: &str,
//    policy_name: &str,
//    hmac: &hmac::Key,
//    duration: Duration,
//    message_id: &str,
//    lock_token: &str,
//) -> Result<(), AzureError> {
//    check_status_extract_body(
//        renew_lock_prepare(
//            http_client,
//            namespace,
//            event_hub,
//            policy_name,
//            hmac,
//            duration,
//            message_id,
//            lock_token,
//        )?,
//        StatusCode::OK,
//    )
//    .await?;
//    Ok(())
//}

pub(crate) fn generate_signature(
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
