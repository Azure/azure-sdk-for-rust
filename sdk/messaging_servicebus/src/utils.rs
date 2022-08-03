use azure_core::error::{Error, ErrorKind, ResultExt};
use std::time::Duration;
use url::Url;

pub fn craft_peek_lock_url(
    namespace: &str,
    queue: &str,
    lock_expiry: Option<Duration>,
) -> Result<Url, Error> {
    let mut url = Url::parse(&format!(
        "https://{}.servicebus.windows.net/{}/messages/head",
        namespace, queue
    ))
    .context(
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
