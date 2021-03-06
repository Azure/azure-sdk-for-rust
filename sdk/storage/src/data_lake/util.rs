use azure_core::errors::AzureError;
use http::HeaderMap;

pub(crate) fn namespace_enabled_from_headers(headers: &HeaderMap) -> Result<bool, AzureError> {
    Ok(headers
        .get("x-ms-namespace-enabled")
        .ok_or(AzureError::HeaderNotFound(
            "x-ms-namespace-enabled".to_owned(),
        ))?
        .to_str()?
        .parse()?)
}
