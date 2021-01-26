use azure_core::errors::AzureError;
use azure_core::headers::CommonStorageResponseHeaders;
use bytes::Bytes;
use http::response::Response;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetQueueServicePropertiesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub logging: Logging,
    pub hour_metrics: Metrics,
    pub minute_metrics: Metrics,
    pub cors: Cors,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct GetQueueServicePropertiesResponseInternal {
    pub logging: Logging,
    pub hour_metrics: Metrics,
    pub minute_metrics: Metrics,
    pub cors: Cors,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RetentionPolicy {
    pub enabled: bool,
    pub days: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Logging {
    pub version: String,
    pub delete: bool,
    pub read: bool,
    pub write: bool,
    pub retention_policy: RetentionPolicy,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Metrics {
    pub version: String,
    pub enabled: bool,
    #[serde(rename = "IncludeAPIs")]
    pub include_apis: Option<bool>,
    pub retention_policy: RetentionPolicy,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Cors {
    pub cors_rule: Option<Vec<CorsRule>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CorsRule {
    pub allowed_origins: String,
    pub allowed_methods: String,
    pub max_age_in_seconds: u64,
    pub exposed_headers: String,
    pub allowed_headers: String,
}

impl std::convert::TryFrom<&Response<Bytes>> for GetQueueServicePropertiesResponse {
    type Error = AzureError;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        let headers = response.headers();
        let body = response.body();

        debug!("headers == {:?}", headers);

        debug!("receieved == {:#?}", &std::str::from_utf8(body)?[3..]);
        let response: GetQueueServicePropertiesResponseInternal =
            serde_xml_rs::from_reader(&body[3..])?;
        debug!("deserde == {:#?}", response);

        Ok(GetQueueServicePropertiesResponse {
            common_storage_response_headers: headers.try_into()?,
            logging: response.logging,
            hour_metrics: response.hour_metrics,
            minute_metrics: response.minute_metrics,
            cors: response.cors,
        })
    }
}
