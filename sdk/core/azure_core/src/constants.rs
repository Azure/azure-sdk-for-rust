/// Endpoints for Azure Resource Manager in different Azure clouds
pub mod resource_manager_endpoint {
    static_url!(
        /// Azure Resource Manager China cloud endpoint
        AZURE_CHINA_CLOUD,
        "https://management.chinacloudapi.cn"
    );

    static_url!(
        /// Azure Resource Manager Germany cloud endpoint
        AZURE_GERMANY_CLOUD,
        "https://management.microsoftazure.de"
    );

    static_url!(
        /// Azure Resource Manager public cloud endpoint
        AZURE_PUBLIC_CLOUD,
        "https://management.azure.com"
    );

    static_url!(
        /// Azure Resource Manager US government cloud endpoint
        AZURE_US_GOVERNMENT_CLOUD,
        "https://management.usgovcloudapi.net"
    );
}

/// A list of known Azure authority hosts
pub mod authority_hosts {
    static_url!(
        /// China-based Azure Authority Host
        AZURE_CHINA_CLOUD,
        "https://login.chinacloudapi.cn"
    );

    static_url!(
        /// Germany-based Azure Authority Host
        AZURE_GERMANY_CLOUD,
        "https://login.microsoftonline.de"
    );

    static_url!(
        /// US Government Azure Authority Host
        AZURE_US_GOVERNMENT_CLOUD,
        "https://login.microsoftonline.us"
    );

    static_url!(
        /// Public Cloud Azure Authority Host
        AZURE_PUBLIC_CLOUD,
        "https://login.microsoftonline.com"
    );
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

/// Constants related to query parameters
pub mod query_param {
    pub const API_VERSION: &str = "api-version";
}
