use http::HeaderMap;

pub(crate) fn namespace_enabled_from_headers(headers: &HeaderMap) -> crate::Result<bool> {
    Ok(headers
        .get("x-ms-namespace-enabled")
        .ok_or_else(|| crate::Error::HeaderNotFound("x-ms-namespace-enabled".to_owned()))?
        .to_str()?
        .parse()?)
}
