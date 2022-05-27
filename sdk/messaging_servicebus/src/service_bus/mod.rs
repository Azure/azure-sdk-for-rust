use azure_core::{
    error::{Error, ErrorKind, ResultExt},
    HttpClient,
};
use bytes::Bytes;
use chrono::Duration;
use http::{
    header::{AUTHORIZATION, CONTENT_LENGTH},
    method::Method,
    Request, Response,
};
use ring::hmac;
use std::{ops::Add, sync::Arc};
use url::{
    form_urlencoded::{self, Serializer},
    Url,
};

mod client;
pub use self::client::Client;

/// Default duration for the SAS token in days — We might want to make this configurable at some point
const DEFAULT_SAS_DURATION: i64 = 1;

/// Prepares an HTTP request
#[inline]
fn prepare_request(
    url: &str,
    method: http::Method,
    body: Option<String>,
    policy_name: &str,
    signing_key: &hmac::Key,
) -> Result<http::Request<Bytes>, Error> {
    // generate sas auth
    let sas = generate_signature(
        policy_name,
        signing_key,
        &url,
        Duration::hours(DEFAULT_SAS_DURATION),
    );

    // create request builder
    let mut request = Request::builder();

    // add method and uri
    request = request.method(method).uri(url);

    // add auth header with sas
    request = request.header(AUTHORIZATION, sas).header(CONTENT_LENGTH, 0);

    // get req body to return
    let ret = match body {
        Some(msg) => request.body(Bytes::from(msg)),
        None => request.body(azure_core::EMPTY_BODY),
    }?;

    Ok(ret)
}

/// Generates a SAS signature
#[inline]
fn generate_signature(
    policy_name: &str,
    signing_key: &hmac::Key,
    url: &str,
    ttl: Duration,
) -> String {
    let sr: String = form_urlencoded::byte_serialize(url.as_bytes()).collect(); // <namespace>.servicebus.windows.net
    let se = ::chrono::Utc::now().add(ttl).timestamp(); // token expiry instant

    let str_to_sign = format!("{}\n{}", sr, se);
    let sig = hmac::sign(signing_key, str_to_sign.as_bytes()); // shared access key

    // shadow sig
    let sig = {
        let sig = ::base64::encode(sig.as_ref());
        let mut ser = Serializer::new(String::new());
        ser.append_pair("sig", &sig);
        let sig = ser.finish();
        sig
    };

    // format sas
    format!(
        "SharedAccessSignature sr={}&{}&se={}&skn={}",
        sr, sig, se, policy_name
    )
}

/// Sends a message to the queue
async fn send_message(
    http_client: &Arc<dyn HttpClient>,
    namespace: &str,
    queue: &str,
    policy_name: &str,
    signing_key: &hmac::Key,
    msg: &str,
) -> Result<(), Error> {
    let url = format!(
        "https://{}.servicebus.windows.net/{}/messages",
        namespace, queue
    );

    let req = prepare_request(
        &url,
        Method::POST,
        Some(msg.to_string()),
        policy_name,
        signing_key,
    )?;

    http_client
        .execute_request_check_status(req, http::StatusCode::CREATED)
        .await?;
    Ok(())
}

/// Receive and delete a message
async fn receive_and_delete_message(
    http_client: &Arc<dyn HttpClient>,
    namespace: &str,
    queue: &str,
    policy_name: &str,
    signing_key: &hmac::Key,
) -> Result<Response<Bytes>, Error> {
    let url = format!(
        "https://{}.servicebus.windows.net/{}/messages/head",
        namespace, queue
    );

    let req = prepare_request(&url, Method::DELETE, None, policy_name, signing_key)?;

    Ok(http_client
        .execute_request_check_status(req, http::StatusCode::OK)
        .await?)
}

/// Non-destructively read a message
///
/// Note: This function does not return the delete location
/// of the message, so, after reading, you will lose
/// "track" of it until the lock expiry runs out and
/// the message can be consumed by others. If you want to keep
/// track of this message (i.e., have the possibility of deletion),
/// use `peek_lock_message2`.
async fn peek_lock_message(
    http_client: &Arc<dyn HttpClient>,
    namespace: &str,
    queue: &str,
    policy_name: &str,
    signing_key: &hmac::Key,
    lock_expiry: Option<Duration>,
) -> Result<Response<Bytes>, Error> {
    let mut url = Url::parse(&format!(
        "https://{}.servicebus.windows.net/{}/messages/head",
        namespace, queue
    ))
    .context(
        ErrorKind::DataConversion,
        "Failed to parse peek_lock_message URL",
    )?;

    // add timeout, if given
    if let Some(t) = lock_expiry {
        url.query_pairs_mut()
            .append_pair("timeout", &t.num_seconds().to_string());
    }

    let req = prepare_request(
        &url.to_string(),
        Method::POST,
        None,
        policy_name,
        signing_key,
    )?;

    Ok(http_client
        .execute_request_check_status(req, http::StatusCode::CREATED)
        .await?)
}

/// Non-destructively read a message but track it
///
/// Note: This function returns a `PeekLockResponse`
/// that contains a helper `delete_message` function.
async fn peek_lock_message2(
    http_client: &Arc<dyn HttpClient>,
    namespace: &str,
    queue: &str,
    policy_name: &str,
    signing_key: &hmac::Key,
    lock_expiry: Option<Duration>,
) -> Result<PeekLockResponse, Error> {
    let mut url = Url::parse(&format!(
        "https://{}.servicebus.windows.net/{}/messages/head",
        namespace, queue
    ))
    .context(
        ErrorKind::DataConversion,
        "Failed to parse peek_lock_message URL",
    )?;

    if let Some(t) = lock_expiry {
        url.query_pairs_mut()
            .append_pair("timeout", &t.num_seconds().to_string());
    }

    let req = prepare_request(
        &url.to_string(),
        Method::POST,
        None,
        policy_name,
        signing_key,
    )?;

    let res = http_client.execute_request(req).await?;

    let status = res.status();
    let lock_location: String = match res.headers().get("Location") {
        Some(header_value) => header_value
            .to_str()
            .context(
                ErrorKind::DataConversion,
                "Failed to get lock location from header",
            )?
            .to_owned(),
        _ => "".to_owned(),
    };
    let body = std::str::from_utf8(res.body())
        .context(
            ErrorKind::DataConversion,
            "Failed to convert body bytes to UTF8",
        )?
        .to_string();

    Ok(PeekLockResponse {
        body,
        lock_location,
        status,
        http_client: http_client.clone(),
        policy_name: policy_name.to_owned(),
        signing_key: signing_key.to_owned(),
    })
}

/// PeekLockResponse object that is returned by `peek_lock_message2`
pub struct PeekLockResponse {
    body: String,
    lock_location: String,
    status: http::StatusCode,
    http_client: Arc<dyn HttpClient>,
    policy_name: String,
    signing_key: hmac::Key,
}

impl PeekLockResponse {
    /// Get the message in the lock
    pub fn body(&self) -> String {
        self.body.clone()
    }

    /// Get the status of the peek
    pub fn status(&self) -> http::StatusCode {
        self.status
    }

    /// Delete message in the lock
    pub async fn delete_message(&self) -> Result<Response<Bytes>, Error> {
        let req = prepare_request(
            &self.lock_location.clone(),
            Method::DELETE,
            None,
            &self.policy_name,
            &self.signing_key,
        )?;

        Ok(self
            .http_client
            .execute_request_check_status(req, http::StatusCode::OK)
            .await?)
    }

    /// Unlock a message in the lock
    pub async fn unlock_message(&self) -> Result<(), Error> {
        let req = prepare_request(
            &self.lock_location.clone(),
            Method::PUT,
            None,
            &self.policy_name,
            &self.signing_key,
        )?;

        self.http_client
            .execute_request_check_status(req, http::StatusCode::OK)
            .await?;

        Ok(())
    }

    /// Renew a message's lock
    pub async fn renew_message_lock(&self) -> Result<(), Error> {
        let req = prepare_request(
            &self.lock_location.clone(),
            Method::POST,
            None,
            &self.policy_name,
            &self.signing_key,
        )?;

        self.http_client
            .execute_request_check_status(req, http::StatusCode::OK)
            .await?;

        Ok(())
    }
}
