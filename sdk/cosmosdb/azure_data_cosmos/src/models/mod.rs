use serde::{Deserialize, Serialize};
use time::error::ComponentRange;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemProperties {
    #[serde(rename = "_etag")]
    pub etag: Option<azure_core::Etag>,
    #[serde(rename = "_self")]
    pub self_link: Option<String>,
    #[serde(rename = "_rid")]
    pub resource_id: Option<String>,
    #[serde(rename = "_ts")]
    pub last_modified: Option<CosmosTimestamp>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CosmosTimestamp(i64);

impl TryInto<OffsetDateTime> for CosmosTimestamp {
    type Error = ComponentRange;

    fn try_into(self) -> Result<OffsetDateTime, Self::Error> {
        OffsetDateTime::from_unix_timestamp(self.0)
    }
}

#[derive(Debug, Deserialize)]
pub struct DatabaseProperties {
    pub id: String,

    #[serde(flatten)]
    pub system_properties: SystemProperties,
}
azure_core::json_model!(DatabaseProperties);
