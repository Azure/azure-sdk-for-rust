use azure_core::error::{ErrorKind, ResultExt};
use azure_core::headers::{HeaderName, Headers};

pub(crate) fn namespace_enabled_from_headers(headers: &Headers) -> azure_core::Result<bool> {
    headers
        .get_as_str_or_err(&HeaderName::from_static("x-ms-namespace-enabled"))?
        .parse()
        .map_kind(ErrorKind::DataConversion)
}
