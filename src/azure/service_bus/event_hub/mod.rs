use azure::core::{
    errors::{check_status_extract_body, AzureError},
    COMPLETE_ENCODE_SET,
};
use futures::future::{self, Future};
use hyper::{self, header, StatusCode};
use ring::hmac;
use std::ops::Add;
use time::Duration;

mod client;
pub use self::client::Client;

type HttpClient = hyper::Client<::hyper_tls::HttpsConnector<hyper::client::HttpConnector>>;

#[inline]
fn send_event_prepare<B: Into<String>>(
    http_client: &HttpClient,
    namespace: &str,
    event_hub: &str,
    policy_name: &str,
    signing_key: &hmac::SigningKey,
    event_body: B,
    duration: Duration,
) -> Result<hyper::client::ResponseFuture, AzureError> {
    // prepare the url to call
    let url = format!("https://{}.servicebus.windows.net/{}/messages", namespace, event_hub);
    debug!("url == {:?}", url);

    // generate sas signature based on key name, key value, url and duration.
    let sas = generate_signature(policy_name, signing_key, &url, duration);
    debug!("sas == {}", sas);

    let event_body = event_body.into();
    let request = hyper::Request::post(url)
        .header(header::AUTHORIZATION, ::bytes::Bytes::from(sas))
        .body(event_body.into())?;

    Ok(http_client.request(request))
}

fn send_event(
    http_client: &HttpClient,
    namespace: &str,
    event_hub: &str,
    policy_name: &str,
    hmac: &hmac::SigningKey,
    event_body: &str,
    duration: Duration,
) -> impl Future<Item = (), Error = AzureError> {
    let req = send_event_prepare(http_client, namespace, event_hub, policy_name, hmac, event_body, duration);

    future::result(req)
        .from_err()
        .and_then(move |future_response| check_status_extract_body(future_response, StatusCode::CREATED))
        .and_then(|_| Ok(()))
}

fn generate_signature(policy_name: &str, signing_key: &hmac::SigningKey, url: &str, ttl: Duration) -> String {
    use url::{form_urlencoded::Serializer, percent_encoding::utf8_percent_encode};

    let expiry = ::chrono::Utc::now().add(ttl).timestamp();
    debug!("expiry == {:?}", expiry);

    let url_encoded = utf8_percent_encode(url, COMPLETE_ENCODE_SET);
    //debug!("url_encoded == {:?}", url_encoded);

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
