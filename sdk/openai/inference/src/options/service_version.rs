#[derive(Debug, Clone)]
pub enum AzureServiceVersion {
    V2023_09_01Preview,
    V2023_12_01Preview,
    V2024_07_01Preview,
}

impl Default for AzureServiceVersion {
    fn default() -> AzureServiceVersion {
        AzureServiceVersion::get_latest()
    }
}

impl AzureServiceVersion {
    pub fn get_latest() -> AzureServiceVersion {
        AzureServiceVersion::V2024_07_01Preview
    }
}

impl From<AzureServiceVersion> for String {
    fn from(version: AzureServiceVersion) -> String {
        let as_str = match version {
            AzureServiceVersion::V2023_09_01Preview => "2023-09-01-preview",
            AzureServiceVersion::V2023_12_01Preview => "2023-12-01-preview",
            AzureServiceVersion::V2024_07_01Preview => "2024-07-01-preview",
        };
        return String::from(as_str);
    }
}

impl ToString for AzureServiceVersion {
    fn to_string(&self) -> String {
        String::from(self.clone())
    }
}
