use azure_core::error::{ErrorKind, ResultExt};
use azure_core::headers::Headers;

pub(crate) fn namespace_enabled_from_headers(headers: &Headers) -> azure_core::Result<bool> {
    headers
        .get_as_str_or_err("x-ms-namespace-enabled")?
        .parse()
        .map_kind(ErrorKind::DataConversion)
}
