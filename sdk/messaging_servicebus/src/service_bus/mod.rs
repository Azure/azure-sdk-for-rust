mod queue_client;
mod topic_client;

pub use self::{
    queue_client::QueueClient,
    topic_client::{SubscriptionReceiver, TopicClient, TopicSender},
};
use crate::utils::{craft_peek_lock_url, get_head_url};
use azure_core::{
    auth::Secret,
    error::Error,
    from_json,
    headers::{self, HeaderName, HeaderValue, CONTENT_TYPE},
    hmac::hmac_sha256,
    CollectedResponse, HttpClient, Method, Request, StatusCode, Url,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Add, str::FromStr, sync::Arc, time::Duration};
use time::OffsetDateTime;
use url::form_urlencoded::{self, Serializer};

/// Default duration for the SAS token in days — We might want to make this configurable at some point
const DEFAULT_SAS_DURATION: u64 = 3_600; // seconds = 1 hour
const BROKER_PROPERTIES: HeaderName = HeaderName::from_static("brokerproperties");

/// Prepares an HTTP request
fn finalize_request(
    url: &str,
    method: azure_core::Method,
    body: Option<String>,
    policy_name: &str,
    signing_key: &Secret,
) -> azure_core::Result<Request> {
    // generate sas auth
    let sas = generate_signature(
        policy_name,
        signing_key,
        url,
        Duration::from_secs(DEFAULT_SAS_DURATION),
    )?;

    // create request builder
    let mut request = Request::new(Url::parse(url)?, method);

    // add auth header with sas
    request.insert_header(headers::AUTHORIZATION, sas);

    // get req body to return
    if let Some(body) = body {
        request.set_body(body);
    } else {
        request.insert_header(headers::CONTENT_LENGTH, "0"); // added to avoid truncation errors
        request.set_body(azure_core::EMPTY_BODY);
    }

    Ok(request)
}

/// Generates a SAS signature
fn generate_signature(
    policy_name: &str,
    signing_key: &Secret,
    url: &str,
    ttl: Duration,
) -> azure_core::Result<String> {
    let sr: String = form_urlencoded::byte_serialize(url.as_bytes()).collect(); // <namespace>.servicebus.windows.net
    let se = OffsetDateTime::now_utc().add(ttl).unix_timestamp(); // token expiry instant

    let str_to_sign = format!("{sr}\n{se}");
    let sig = hmac_sha256(&str_to_sign, signing_key)?;

    // shadow sig
    let sig = {
        let mut ser = Serializer::new(String::new());
        ser.append_pair("sig", &sig);
        ser.finish()
    };

    // format sas
    Ok(format!(
        "SharedAccessSignature sr={sr}&{sig}&se={se}&skn={policy_name}"
    ))
}

/// Sends a message to the queue or topic
async fn send_message(
    http_client: &Arc<dyn HttpClient>,
    namespace: &str,
    queue_or_topic: &str,
    policy_name: &str,
    signing_key: &Secret,
    msg: &str,
    send_message_options: Option<SendMessageOptions>,
) -> azure_core::Result<()> {
    let url = format!("https://{namespace}.servicebus.windows.net/{queue_or_topic}/messages");

    let mut req = finalize_request(
        &url,
        Method::Post,
        Some(msg.to_string()),
        policy_name,
        signing_key,
    )?;

    req.insert_headers(&send_message_options);

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
    queue_or_topic: &str,
    policy_name: &str,
    signing_key: &Secret,
    subscription: Option<&str>,
) -> azure_core::Result<CollectedResponse> {
    let url = get_head_url(namespace, queue_or_topic, subscription);
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
    queue_or_topic: &str,
    policy_name: &str,
    signing_key: &Secret,
    timeout: Option<Duration>,
    subscription: Option<&str>,
) -> azure_core::Result<CollectedResponse> {
    let url = craft_peek_lock_url(namespace, queue_or_topic, timeout, subscription)?;

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
    queue_or_topic: &str,
    policy_name: &str,
    signing_key: &Secret,
    timeout: Option<Duration>,
    subscription: Option<&str>,
) -> azure_core::Result<PeekLockResponse> {
    let url = craft_peek_lock_url(namespace, queue_or_topic, timeout, subscription)?;

    let req = finalize_request(url.as_ref(), Method::Post, None, policy_name, signing_key)?;

    let res = http_client.execute_request(&req).await?;

    let status = res.status();
    let headers = res.headers().clone();
    let broker_properties = res
        .headers()
        .get_optional_as(&headers::HeaderName::from("brokerproperties"))?;
    let lock_location = headers
        .get_optional_string(&headers::LOCATION)
        .unwrap_or_default();
    let body = res.into_body().collect_string().await?;

    Ok(PeekLockResponse {
        body,
        headers,
        broker_properties,
        lock_location,
        status,
        http_client: http_client.clone(),
        policy_name: policy_name.to_owned(),
        signing_key: signing_key.to_owned(),
    })
}

/// `PeekLockResponse` object that is returned by `peek_lock_message2`
pub struct PeekLockResponse {
    body: String,
    headers: headers::Headers,
    broker_properties: Option<BrokerProperties>,
    lock_location: String,
    status: StatusCode,
    http_client: Arc<dyn HttpClient>,
    policy_name: String,
    signing_key: Secret,
}

impl PeekLockResponse {
    /// Get the message in the lock
    pub fn body(&self) -> String {
        self.body.clone()
    }

    /// Get the broker properties from the message in the lock
    #[must_use]
    pub fn broker_properties(&self) -> Option<BrokerProperties> {
        self.broker_properties.clone()
    }

    /// Get custom message headers from the message in the lock
    pub fn custom_properties<T: TryFrom<headers::Headers>>(&self) -> Result<T, T::Error> {
        self.headers.clone().try_into()
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

/// `BrokerProperties` object decoded from the message headers
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BrokerProperties {
    pub delivery_count: i32,
    pub enqueued_sequence_number: Option<i32>,
    #[serde(with = "time::serde::rfc2822::option")]
    pub enqueued_time_utc: Option<OffsetDateTime>,
    pub lock_token: String,
    #[serde(with = "time::serde::rfc2822")]
    pub locked_until_utc: OffsetDateTime,
    pub message_id: String,
    pub sequence_number: i64,
    pub state: String,
    pub time_to_live: f64,
}

impl FromStr for BrokerProperties {
    type Err = azure_core::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        from_json(s)
    }
}

#[derive(Debug, Default)]
pub struct SendMessageOptions {
    pub content_type: Option<String>,
    pub broker_properties: Option<SettableBrokerProperties>,
    pub custom_properties: Option<HashMap<String, String>>,
}

impl headers::AsHeaders for SendMessageOptions {
    type Iter = std::vec::IntoIter<(HeaderName, HeaderValue)>;

    fn as_headers(&self) -> Self::Iter {
        let mut headers: Vec<(HeaderName, HeaderValue)> = vec![];

        if let Some(content_type) = &self.content_type {
            headers.push((CONTENT_TYPE, content_type.into()));
        }

        if let Some(broker_properties) = &self.broker_properties {
            headers.push((
                BROKER_PROPERTIES,
                serde_json::to_string(broker_properties).unwrap().into(),
            ));
        }

        if let Some(custom_properties) = &self.custom_properties {
            headers.extend(
                custom_properties
                    .iter()
                    .map(|(k, v)| (k.to_owned().into(), v.into())),
            );
        }

        headers.into_iter()
    }
}

#[derive(Clone, Debug, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct SettableBrokerProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,

    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "duration_to_seconds_f64"
    )]
    pub time_to_live: Option<Duration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,

    #[serde(
        with = "time::serde::rfc2822::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub scheduled_enqueue_time_utc: Option<OffsetDateTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_session_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<String>,
}

fn duration_to_seconds_f64<S>(duration: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(duration) = duration {
        serializer.serialize_f64(duration.as_secs_f64())
    } else {
        serializer.serialize_none()
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::service_bus::SettableBrokerProperties;

    #[test]
    fn test_duration_serialize() {
        let dur = SettableBrokerProperties {
            time_to_live: Some(Duration::from_millis(4444)),
            ..Default::default()
        };
        let dur_str = serde_json::to_string(&dur).unwrap();
        assert_eq!(dur_str, r#"{"TimeToLive":4.444}"#);
    }
}
