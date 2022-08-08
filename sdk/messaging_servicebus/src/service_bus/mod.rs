use azure_core::{error::Error, headers, CollectedResponse, HttpClient, Request, Url};
use azure_core::{Method, StatusCode};
use ring::hmac;
use std::time::Duration;
use std::{ops::Add, sync::Arc};
use time::OffsetDateTime;
use url::form_urlencoded::{self, Serializer};

mod client;

use crate::utils::craft_peek_lock_url;

pub use self::client::Client;

/// Default duration for the SAS token in days — We might want to make this configurable at some point
const DEFAULT_SAS_DURATION: u64 = 3_600; // seconds = 1 hour

/// Prepares an HTTP request
fn finalize_request(
    url: &str,
    method: azure_core::Method,
    body: Option<String>,
    policy_name: &str,
    signing_key: &hmac::Key,
) -> azure_core::Result<Request> {
    // generate sas auth
    let sas = generate_signature(
        policy_name,
        signing_key,
        url,
        Duration::from_secs(DEFAULT_SAS_DURATION),
    );

    // create request builder
    let mut request = Request::new(Url::parse(url)?, method);

    // add auth header with sas
    request.insert_header(headers::AUTHORIZATION, sas);

    // get req body to return
    match body {
        Some(msg) => request.set_body(msg),
        None => {
            request.insert_header(headers::CONTENT_LENGTH, "0"); // added to avoid truncation errors
            request.set_body(azure_core::EMPTY_BODY);
        }
    }

    Ok(request)
}

/// Generates a SAS signature
fn generate_signature(
    policy_name: &str,
    signing_key: &hmac::Key,
    url: &str,
    ttl: Duration,
) -> String {
    let sr: String = form_urlencoded::byte_serialize(url.as_bytes()).collect(); // <namespace>.servicebus.windows.net
    let se = OffsetDateTime::now_utc().add(ttl).unix_timestamp(); // token expiry instant

    let str_to_sign = format!("{}\n{}", sr, se);
    let sig = hmac::sign(signing_key, str_to_sign.as_bytes()); // shared access key

    // shadow sig
    let sig = {
        let sig = ::base64::encode(sig.as_ref());
        let mut ser = Serializer::new(String::new());
        ser.append_pair("sig", &sig);
        ser.finish()
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
) -> azure_core::Result<()> {
    let url = format!(
        "https://{}.servicebus.windows.net/{}/messages",
        namespace, queue
    );

    let req = finalize_request(
        &url,
        Method::Post,
        Some(msg.to_string()),
        policy_name,
        signing_key,
    )?;

    http_client
        .as_ref()
        .execute_request_check_status(&req)
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
) -> azure_core::Result<CollectedResponse> {
    let url = format!(
        "https://{}.servicebus.windows.net/{}/messages/head",
        namespace, queue
    );

    let req = finalize_request(&url, Method::Delete, None, policy_name, signing_key)?;

    http_client
        .as_ref()
        .execute_request_check_status(&req)
        .await
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
) -> azure_core::Result<CollectedResponse> {
    let url = craft_peek_lock_url(namespace, queue, lock_expiry)?;

    let req = finalize_request(url.as_ref(), Method::Post, None, policy_name, signing_key)?;

    http_client
        .as_ref()
        .execute_request_check_status(&req)
        .await
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
) -> azure_core::Result<PeekLockResponse> {
    let url = craft_peek_lock_url(namespace, queue, lock_expiry)?;

    let req = finalize_request(url.as_ref(), Method::Post, None, policy_name, signing_key)?;

    let res = http_client.execute_request(&req).await?;

    let status = res.status();
    let lock_location = res
        .headers()
        .get_optional_string(&headers::LOCATION)
        .unwrap_or_default();
    let body = res.into_body().collect_string().await?;

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
    status: StatusCode,
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
    pub fn status(&self) -> &StatusCode {
        &self.status
    }

    /// Delete message in the lock
    pub async fn delete_message(&self) -> azure_core::Result<CollectedResponse> {
        let req = finalize_request(
            &self.lock_location.clone(),
            Method::Delete,
            None,
            &self.policy_name,
            &self.signing_key,
        )?;

        self.http_client
            .as_ref()
            .execute_request_check_status(&req)
            .await
    }

    /// Unlock a message in the lock
    pub async fn unlock_message(&self) -> Result<(), Error> {
        let req = finalize_request(
            &self.lock_location.clone(),
            Method::Put,
            None,
            &self.policy_name,
            &self.signing_key,
        )?;

        self.http_client
            .as_ref()
            .execute_request_check_status(&req)
            .await?;
        Ok(())
    }

    /// Renew a message's lock
    pub async fn renew_message_lock(&self) -> Result<(), Error> {
        let req = finalize_request(
            &self.lock_location.clone(),
            Method::Post,
            None,
            &self.policy_name,
            &self.signing_key,
        )?;

        self.http_client
            .as_ref()
            .execute_request_check_status(&req)
            .await?;
        Ok(())
    }
}
