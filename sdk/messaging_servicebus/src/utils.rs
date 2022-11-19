use azure_core::error::{Error, ErrorKind, ResultExt};
use std::time::Duration;
use url::Url;

pub fn craft_peek_lock_url(
    namespace: &str,
    queue_or_topic: &str,
    lock_expiry: Option<Duration>,
    subscription: Option<&str>,
) -> Result<Url, Error> {
    let url_path = get_head_url(namespace, queue_or_topic, subscription);
    let mut url = Url::parse(&url_path).context(
        ErrorKind::DataConversion,
        "failed to parse peek_lock_message URL",
    )?;

    // add timeout, if given
    if let Some(t) = lock_expiry {
        url.query_pairs_mut()
            .append_pair("timeout", &t.as_secs().to_string());
    };

    Ok(url)
}

pub fn body_bytes_to_utf8(bytes: &[u8]) -> Result<String, Error> {
    Ok(std::str::from_utf8(bytes)
        .context(
            ErrorKind::DataConversion,
            "failed to convert body bytes to UTF8",
        )?
        .to_string())
}

pub fn get_head_url(namespace: &str, queue_or_topic: &str, subscription: Option<&str>) -> String {
    match subscription {
        Some(sub) => format!(
            "https://{}.servicebus.windows.net/{}/subscriptions/{}/messages/head",
            namespace, queue_or_topic, sub
        ),
        None => format!(
            "https://{}.servicebus.windows.net/{}/messages/head",
            namespace, queue_or_topic
        ),
    }
}
