#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueueServiceProperties {
    pub logging: Logging,
    pub hour_metrics: Metrics,
    pub minute_metrics: Metrics,
    pub cors: Cors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RetentionPolicy {
    pub enabled: bool,
    pub days: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Logging {
    pub version: String,
    pub delete: bool,
    pub read: bool,
    pub write: bool,
    pub retention_policy: RetentionPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Metrics {
    pub version: String,
    pub enabled: bool,
    #[serde(rename = "IncludeAPIs")]
    pub include_apis: Option<bool>,
    pub retention_policy: RetentionPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Cors {
    pub cors_rule: Option<Vec<CorsRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CorsRule {
    pub allowed_origins: String,
    pub allowed_methods: String,
    pub max_age_in_seconds: u64,
    pub exposed_headers: String,
    pub allowed_headers: String,
}
