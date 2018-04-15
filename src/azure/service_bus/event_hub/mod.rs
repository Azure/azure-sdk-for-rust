use futures::future::*;
use hyper_tls;
use tokio_core;

use azure::core::errors::{check_status_extract_body, AzureError};
use azure::core::COMPLETE_ENCODE_SET;

use std::str::FromStr;

use hyper;
use hyper::header::ContentLength;
use hyper::StatusCode;

use chrono;
use time::Duration;

use base64;
use std::ops::Add;

use url::form_urlencoded::Serializer;
use url::percent_encoding::utf8_percent_encode;

use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;

mod client;
pub use self::client::Client;

header! { (Authorization, "Authorization") => [String] }

#[inline]
fn send_event_prepare(
    handle: &tokio_core::reactor::Handle,
    namespace: &str,
    event_hub: &str,
    policy_name: &str,
    hmac: &mut Hmac<Sha256>,
    event_body: &str,
    duration: Duration,
) -> Result<hyper::client::FutureResponse, AzureError> {
    // prepare the url to call
    let url = format!(
        "https://{}.servicebus.windows.net/{}/messages",
        namespace, event_hub
    );
    let url = hyper::Uri::from_str(&url)?;
    debug!("url == {:?}", url);

    // generate sas signature based on key name, key value, url and duration.
    let sas = generate_signature(policy_name, hmac, &url.to_string(), duration);
    debug!("sas == {}", sas);

    let client = hyper::Client::configure()
        .connector(hyper_tls::HttpsConnector::new(4, handle)?)
        .build(handle);

    let mut request = hyper::Request::new(hyper::Method::Post, url);

    request.headers_mut().set(Authorization(sas));
    request
        .headers_mut()
        .set(ContentLength(event_body.len() as u64));
    debug!("request.headers() == {:?}", request.headers());

    request.set_body(event_body.to_string());

    Ok(client.request(request))
}

fn send_event(
    handle: &tokio_core::reactor::Handle,
    namespace: &str,
    event_hub: &str,
    policy_name: &str,
    hmac: &mut Hmac<Sha256>,
    event_body: &str,
    duration: Duration,
) -> impl Future<Item = (), Error = AzureError> {
    let req = send_event_prepare(
        handle,
        namespace,
        event_hub,
        policy_name,
        hmac,
        event_body,
        duration,
    );

    done(req).from_err().and_then(move |future_response| {
        check_status_extract_body(future_response, StatusCode::Created).and_then(|_| ok(()))
    })
}

fn generate_signature(
    policy_name: &str,
    hmac: &mut Hmac<Sha256>,
    url: &str,
    ttl: Duration,
) -> String {
    let expiry = chrono::Utc::now().add(ttl).timestamp();
    debug!("expiry == {:?}", expiry);

    let url_encoded = utf8_percent_encode(url, COMPLETE_ENCODE_SET);
    //debug!("url_encoded == {:?}", url_encoded);

    let str_to_sign = format!("{}\n{}", url_encoded, expiry);
    debug!("str_to_sign == {:?}", str_to_sign);

    hmac.reset();
    hmac.input(str_to_sign.as_bytes());
    let sig = {
        let sig = base64::encode(hmac.result().code());
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
