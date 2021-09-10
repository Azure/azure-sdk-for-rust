use chrono::{DateTime, Utc};

pub mod delete_entity;
pub mod get_entity;
pub mod insert_entity;
pub mod insert_or_merge_entity;
pub mod insert_or_replace_entity;
pub mod merge_entity;
pub mod update_entity;

/// This trait represents a table entity.
/// User should implement this trait for thir custom models
pub trait TableEntity<'a> {
    type Entity: serde::Serialize + 'a;

    /// Return partition key value as reference.
    fn partition_key(&self) -> &str;

    /// Return partition key value as reference.
    fn row_key(&self) -> &str;
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityMetadata {
    /// The type name of the containing object.
    #[serde(rename = "odata.type")]
    pub odata_type: String,

    /// The entity ID, which is generally the URL to the resource.
    #[serde(rename = "odata.id")]
    pub odata_id: String,

    /// The ETag of the entity.
    #[serde(rename = "odata.etag")]
    pub etag: String,

    /// The link used to edit/update the entry.
    #[serde(rename = "odata.editLink")]
    pub odata_edit_link: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityResponse<ENTITY> {
    #[serde(flatten)]
    pub metadata: Option<EntityMetadata>,

    #[serde(with = "entity_timestamp_format", rename = "Timestamp")]
    pub timestamp: DateTime<Utc>,

    #[serde(flatten)]
    pub entity: ENTITY,
}

mod entity_timestamp_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    const FMT: &'static str = "%+";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // %+ is equivalent to the following: `%Y-%m-%dT%H:%M:%S%.f%:z`
        serializer.serialize_str(format!("{}", date.format(FMT)).as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FMT)
            .map_err(serde::de::Error::custom)
    }
}
