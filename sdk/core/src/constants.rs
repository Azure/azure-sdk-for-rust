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
