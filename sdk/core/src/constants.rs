/// Endpoints for Azure Resource Manager in different Azure clouds
pub mod resource_manager_endpoint {
    /// Azure Resource Manager China cloud endpoint
    pub const AZURE_CHINA_CLOUD: &str = "https://management.chinacloudapi.cn";

    /// Azure Resource Manager Germany cloud endpoint
    pub const AZURE_GERMANY_CLOUD: &str = "https://management.microsoftazure.de";

    /// Azure Resource Manager public cloud endpoint
    pub const AZURE_PUBLIC_CLOUD: &str = "https://management.azure.com";

    /// Azure Resource Manager US government cloud endpoint
    pub const AZURE_US_GOVERNMENT_CLOUD: &str = "https://management.usgovcloudapi.net";
}

/// Constants related to the Content-Type header
///
/// <https://developer.mozilla.org/docs/Web/HTTP/Headers/Content-Type>
pub mod content_type {
    use crate::headers::HeaderValue;

    // Form content types
    // https://www.w3.org/TR/html401/interact/forms.html#h-17.13.4

    pub const MULTIPART_FORM_DATA: HeaderValue = HeaderValue::from_static("multipart/form-data");
    pub const APPLICATION_X_WWW_FORM_URLENCODED: HeaderValue =
        HeaderValue::from_static("application/x-www-form-urlencoded");

    pub const APPLICATION_XML: HeaderValue = HeaderValue::from_static("application/xml");
    pub const APPLICATION_JSON: HeaderValue = HeaderValue::from_static("application/json");
    pub const APPLICATION_OCTET_STREAM: HeaderValue =
        HeaderValue::from_static("application/octet-stream");
    pub const TEXT_PLAIN: HeaderValue = HeaderValue::from_static("text/plain");
}

/// Constants related to the Content-Type header
///
/// <https://developer.mozilla.org/docs/Web/HTTP/Headers/Content-Type>
pub mod query_param {
    pub const API_VERSION: &str = "api-version";
}
