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

/// https://developer.mozilla.org/docs/Web/HTTP/Headers/Content-Type
pub mod content_type {

    // Form content types
    // https://www.w3.org/TR/html401/interact/forms.html#h-17.13.4

    pub const MULTIPART_FORM_DATA: &str = "multipart/form-data";
    pub const APPLICATION_X_WWW_FORM_URLENCODED: &str = "application/x-www-form-urlencoded";

    pub const APPLICATION_XML: &str = "application/xml";
    pub const APPLICATION_JSON: &str = "application/json";
}
