use azure_core::error::{Error, ErrorKind, ResultExt};
use http::HeaderMap;

pub(crate) fn namespace_enabled_from_headers(headers: &HeaderMap) -> azure_core::Result<bool> {
    headers
        .get("x-ms-namespace-enabled")
        .ok_or_else(|| Error::message(ErrorKind::Other, "Header x-ms-namespace-enabled not found"))?
        .to_str()
        .map_kind(ErrorKind::DataConversion)?
        .parse()
        .map_kind(ErrorKind::DataConversion)
}
