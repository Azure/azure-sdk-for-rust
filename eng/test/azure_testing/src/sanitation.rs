use azure_core::headers::{HeaderName, HeaderValue};
use http_types::Url;

pub fn sanitize_header(header_name: &HeaderName, header_value: &HeaderValue) -> HeaderValue {
    match header_name.as_str() {
        // It'd be nice to replace the actual token with the Mock token format used by MockTokenCredential,
        // but we don't have access to the test context at this point in the pipeline.
        "authorization" => HeaderValue::from_static("<<REDACTED>>"),
        "content-location" => HeaderValue::from_cow(sanitize_url(header_value.as_str())),
        _ => header_value.clone(),
    }
}

pub fn sanitize_url(url: &str) -> String {
    let Ok(mut url) = Url::parse(url) else {
        // Not a valid URL, don't sanitize it.
        return url.to_string();
    };

    // Strip the host and replace it with a generic example.com domain.
    if url.has_host() {
        url.set_host(Some("azure_testing.example.com")).unwrap();
    }

    url.to_string()
}
