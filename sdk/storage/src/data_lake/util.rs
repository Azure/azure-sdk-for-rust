use crate::AzureStorageError;
use http::HeaderMap;

pub(crate) fn namespace_enabled_from_headers(
    headers: &HeaderMap,
) -> Result<bool, AzureStorageError> {
    Ok(headers
        .get("x-ms-namespace-enabled")
        .ok_or_else(|| AzureStorageError::HeaderNotFound("x-ms-namespace-enabled".to_owned()))?
        .to_str()?
        .parse()?)
}
