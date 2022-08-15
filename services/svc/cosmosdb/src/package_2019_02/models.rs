#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An Access policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessPolicy {
    #[doc = "The start datetime from which the policy is active."]
    #[serde(rename = "Start", with = "azure_core::date::rfc3339")]
    pub start: time::OffsetDateTime,
    #[doc = "The datetime that the policy expires."]
    #[serde(rename = "Expiry", with = "azure_core::date::rfc3339")]
    pub expiry: time::OffsetDateTime,
    #[doc = "The permissions for the acl policy."]
    #[serde(rename = "Permission")]
    pub permission: String,
}
impl AccessPolicy {
    pub fn new(start: time::OffsetDateTime, expiry: time::OffsetDateTime, permission: String) -> Self {
        Self { start, expiry, permission }
    }
}
#[doc = "CORS is an HTTP feature that enables a web application running under one domain to access resources in another domain. Web browsers implement a security restriction known as same-origin policy that prevents a web page from calling APIs in a different domain; CORS provides a secure way to allow one domain (the origin domain) to call APIs in another domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CorsRule {
    #[doc = "The origin domains that are permitted to make a request against the service via CORS. The origin domain is the domain from which the request originates. Note that the origin must be an exact case-sensitive match with the origin that the user age sends to the service. You can also use the wildcard character '*' to allow all origin domains to make requests via CORS."]
    #[serde(rename = "AllowedOrigins")]
    pub allowed_origins: String,
    #[doc = "The methods (HTTP request verbs) that the origin domain may use for a CORS request. (comma separated)"]
    #[serde(rename = "AllowedMethods")]
    pub allowed_methods: String,
    #[doc = "The request headers that the origin domain may specify on the CORS request."]
    #[serde(rename = "AllowedHeaders")]
    pub allowed_headers: String,
    #[doc = "The response headers that may be sent in the response to the CORS request and exposed by the browser to the request issuer."]
    #[serde(rename = "ExposedHeaders")]
    pub exposed_headers: String,
    #[doc = "The maximum amount time that a browser should cache the preflight OPTIONS request."]
    #[serde(rename = "MaxAgeInSeconds")]
    pub max_age_in_seconds: i64,
}
impl CorsRule {
    pub fn new(
        allowed_origins: String,
        allowed_methods: String,
        allowed_headers: String,
        exposed_headers: String,
        max_age_in_seconds: i64,
    ) -> Self {
        Self {
            allowed_origins,
            allowed_methods,
            allowed_headers,
            exposed_headers,
            max_age_in_seconds,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoReplication {
    #[doc = "The status of the secondary location."]
    #[serde(rename = "Status")]
    pub status: geo_replication::Status,
    #[doc = "A GMT date/time value, to the second. All primary writes preceding this value are guaranteed to be available for read operations at the secondary. Primary writes after this point in time may or may not be available for reads."]
    #[serde(rename = "LastSyncTime", with = "azure_core::date::rfc1123")]
    pub last_sync_time: time::OffsetDateTime,
}
impl GeoReplication {
    pub fn new(status: geo_replication::Status, last_sync_time: time::OffsetDateTime) -> Self {
        Self { status, last_sync_time }
    }
}
pub mod geo_replication {
    use super::*;
    #[doc = "The status of the secondary location."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "live")]
        Live,
        #[serde(rename = "bootstrap")]
        Bootstrap,
        #[serde(rename = "unavailable")]
        Unavailable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Status {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Status {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Status {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Live => serializer.serialize_unit_variant("Status", 0u32, "live"),
                Self::Bootstrap => serializer.serialize_unit_variant("Status", 1u32, "bootstrap"),
                Self::Unavailable => serializer.serialize_unit_variant("Status", 2u32, "unavailable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Azure Analytics Logging settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Logging {
    #[doc = "The version of Analytics to configure."]
    #[serde(rename = "Version")]
    pub version: String,
    #[doc = "Indicates whether all delete requests should be logged."]
    #[serde(rename = "Delete")]
    pub delete: bool,
    #[doc = "Indicates whether all read requests should be logged."]
    #[serde(rename = "Read")]
    pub read: bool,
    #[doc = "Indicates whether all write requests should be logged."]
    #[serde(rename = "Write")]
    pub write: bool,
    #[doc = "The retention policy."]
    #[serde(rename = "RetentionPolicy")]
    pub retention_policy: RetentionPolicy,
}
impl Logging {
    pub fn new(version: String, delete: bool, read: bool, write: bool, retention_policy: RetentionPolicy) -> Self {
        Self {
            version,
            delete,
            read,
            write,
            retention_policy,
        }
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metrics {
    #[doc = "The version of Analytics to configure."]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Indicates whether metrics are enabled for the Table service."]
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[doc = "Indicates whether metrics should generate summary statistics for called API operations."]
    #[serde(rename = "IncludeAPIs", default, skip_serializing_if = "Option::is_none")]
    pub include_ap_is: Option<bool>,
    #[doc = "The retention policy."]
    #[serde(rename = "RetentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
}
impl Metrics {
    pub fn new(enabled: bool) -> Self {
        Self {
            version: None,
            enabled,
            include_ap_is: None,
            retention_policy: None,
        }
    }
}
#[doc = "The retention policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetentionPolicy {
    #[doc = "Indicates whether a retention policy is enabled for the service."]
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[doc = "Indicates the number of days that metrics or logging or soft-deleted data should be retained. All data older than this value will be deleted."]
    #[serde(rename = "Days", default, skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
}
impl RetentionPolicy {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, days: None }
    }
}
#[doc = "A signed identifier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignedIdentifier {
    #[doc = "A unique id."]
    #[serde(rename = "Id")]
    pub id: String,
    #[doc = "An Access policy."]
    #[serde(rename = "AccessPolicy")]
    pub access_policy: AccessPolicy,
}
impl SignedIdentifier {
    pub fn new(id: String, access_policy: AccessPolicy) -> Self {
        Self { id, access_policy }
    }
}
pub type SignedIdentifiers = Vec<SignedIdentifier>;
#[doc = "The other properties of the table entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableEntityProperties {}
impl TableEntityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties for the table entity query response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableEntityQueryResponse {
    #[doc = "The metadata response of the table."]
    #[serde(rename = "odata.metadata", default, skip_serializing_if = "Option::is_none")]
    pub odata_metadata: Option<String>,
    #[doc = "List of table entities."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TableEntityProperties>,
}
impl TableEntityQueryResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties for creating a table."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableProperties {
    #[doc = "The name of the table to create."]
    #[serde(rename = "TableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}
impl TableProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties for the table query response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableQueryResponse {
    #[doc = "The metadata response of the table."]
    #[serde(rename = "odata.metadata", default, skip_serializing_if = "Option::is_none")]
    pub odata_metadata: Option<String>,
    #[doc = "List of tables."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TableResponseProperties>,
}
impl TableQueryResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response for a single table."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableResponse {
    #[serde(flatten)]
    pub table_response_properties: TableResponseProperties,
    #[doc = "The metadata response of the table."]
    #[serde(rename = "odata.metadata", default, skip_serializing_if = "Option::is_none")]
    pub odata_metadata: Option<String>,
}
impl TableResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties for the table response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableResponseProperties {
    #[doc = "The name of the table."]
    #[serde(rename = "TableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[doc = "The odata type of the table."]
    #[serde(rename = "odata.type", default, skip_serializing_if = "Option::is_none")]
    pub odata_type: Option<String>,
    #[doc = "The id of the table."]
    #[serde(rename = "odata.id", default, skip_serializing_if = "Option::is_none")]
    pub odata_id: Option<String>,
    #[doc = "The edit link of the table."]
    #[serde(rename = "odata.editLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_edit_link: Option<String>,
}
impl TableResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Table Service error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableServiceError {
    #[doc = "The error message."]
    #[serde(rename = "Message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl TableServiceError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Table Service Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableServiceProperties {
    #[doc = "Azure Analytics Logging settings."]
    #[serde(rename = "Logging", default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<Logging>,
    #[doc = ""]
    #[serde(rename = "HourMetrics", default, skip_serializing_if = "Option::is_none")]
    pub hour_metrics: Option<Metrics>,
    #[doc = ""]
    #[serde(rename = "MinuteMetrics", default, skip_serializing_if = "Option::is_none")]
    pub minute_metrics: Option<Metrics>,
    #[doc = "The set of CORS rules."]
    #[serde(rename = "Cors", default, skip_serializing_if = "Vec::is_empty")]
    pub cors: Vec<CorsRule>,
}
impl TableServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Stats for the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableServiceStats {
    #[serde(rename = "GeoReplication", default, skip_serializing_if = "Option::is_none")]
    pub geo_replication: Option<GeoReplication>,
}
impl TableServiceStats {
    pub fn new() -> Self {
        Self::default()
    }
}
