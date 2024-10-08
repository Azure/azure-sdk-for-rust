#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "class that captures details of a struct-attribute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasAttributeDef {
    #[doc = "Cardinality"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cardinality: Option<CardinalityValue>,
    #[doc = "An array of constraints."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub constraints: Vec<AtlasConstraintDef>,
    #[doc = "The default value of the attribute."]
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[doc = "The description of the attribute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Determines if it is included in notification."]
    #[serde(rename = "includeInNotification", default, skip_serializing_if = "Option::is_none")]
    pub include_in_notification: Option<bool>,
    #[doc = "Determines if it is indexable."]
    #[serde(rename = "isIndexable", default, skip_serializing_if = "Option::is_none")]
    pub is_indexable: Option<bool>,
    #[doc = "Determines if it is optional."]
    #[serde(rename = "isOptional", default, skip_serializing_if = "Option::is_none")]
    pub is_optional: Option<bool>,
    #[doc = "Determines if it unique."]
    #[serde(rename = "isUnique", default, skip_serializing_if = "Option::is_none")]
    pub is_unique: Option<bool>,
    #[doc = "The name of the attribute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The options for the attribute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[doc = "The name of the type."]
    #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[doc = "The maximum count of the values."]
    #[serde(rename = "valuesMaxCount", default, skip_serializing_if = "Option::is_none")]
    pub values_max_count: Option<i32>,
    #[doc = "The minimum count of the values."]
    #[serde(rename = "valuesMinCount", default, skip_serializing_if = "Option::is_none")]
    pub values_min_count: Option<i32>,
}
impl AtlasAttributeDef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "class that captures details of a struct-type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasBusinessMetadataDef {
    #[doc = "Type Category"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<TypeCategory>,
    #[doc = "The created time of the record."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[doc = "The user who created the record."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The date format."]
    #[serde(rename = "dateFormatter", default, skip_serializing_if = "Option::is_none")]
    pub date_formatter: Option<DateFormat>,
    #[doc = "The description of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The GUID of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "The name of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The options for the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[doc = "The service type."]
    #[serde(rename = "serviceType", default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    #[doc = "The version of the type."]
    #[serde(rename = "typeVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_version: Option<String>,
    #[doc = "The update time of the record."]
    #[serde(rename = "updateTime", default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[doc = "The user who updated the record."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "The version of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[doc = "ETag for concurrency control."]
    #[serde(rename = "lastModifiedTS", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_ts: Option<String>,
    #[doc = "An array of attribute definitions."]
    #[serde(
        rename = "attributeDefs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub attribute_defs: Vec<AtlasAttributeDef>,
}
impl AtlasBusinessMetadataDef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An instance of a classification; it doesn't have an identity, this object\nexists only when associated with an entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasClassification {
    #[doc = "The attributes of the struct."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    #[doc = "The name of the type."]
    #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[doc = "ETag for concurrency control."]
    #[serde(rename = "lastModifiedTS", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_ts: Option<String>,
    #[doc = "The GUID of the entity."]
    #[serde(rename = "entityGuid", default, skip_serializing_if = "Option::is_none")]
    pub entity_guid: Option<String>,
    #[doc = "Status - can be active or deleted"]
    #[serde(rename = "entityStatus", default, skip_serializing_if = "Option::is_none")]
    pub entity_status: Option<EntityStatus>,
    #[doc = "Determines if propagations will be removed on entity deletion."]
    #[serde(rename = "removePropagationsOnEntityDelete", default, skip_serializing_if = "Option::is_none")]
    pub remove_propagations_on_entity_delete: Option<bool>,
    #[doc = "An array of time boundaries indicating validity periods."]
    #[serde(
        rename = "validityPeriods",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub validity_periods: Vec<TimeBoundary>,
}
impl AtlasClassification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "class that captures details of a classification-type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasClassificationDef {
    #[doc = "Type Category"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<TypeCategory>,
    #[doc = "The created time of the record."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[doc = "The user who created the record."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The date format."]
    #[serde(rename = "dateFormatter", default, skip_serializing_if = "Option::is_none")]
    pub date_formatter: Option<DateFormat>,
    #[doc = "The description of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The GUID of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "The name of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The options for the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[doc = "The service type."]
    #[serde(rename = "serviceType", default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    #[doc = "The version of the type."]
    #[serde(rename = "typeVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_version: Option<String>,
    #[doc = "The update time of the record."]
    #[serde(rename = "updateTime", default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[doc = "The user who updated the record."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "The version of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[doc = "ETag for concurrency control."]
    #[serde(rename = "lastModifiedTS", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_ts: Option<String>,
    #[doc = "An array of attribute definitions."]
    #[serde(
        rename = "attributeDefs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub attribute_defs: Vec<AtlasAttributeDef>,
    #[doc = "Specifying a list of entityType names in the classificationDef, ensures that\nclassifications can\nonly be applied to those entityTypes.\n\nAny subtypes of the entity types inherit the restriction.\n\nAny classificationDef subtypes inherit the parents entityTypes restrictions.\n\nAny classificationDef subtypes can further restrict the parents entityTypes\nrestrictions by specifying a subset of the entityTypes.\n\nAn empty entityTypes list when there are no parent restrictions means there are no\nrestrictions.\n\nAn empty entityTypes list when there are parent\nrestrictions means that the subtype picks up the parents\nrestrictions.\n\nIf a list of entityTypes are supplied, where one inherits\nfrom another, this will be rejected. This should encourage cleaner\nclassificationsDefs.\n"]
    #[serde(
        rename = "entityTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub entity_types: Vec<String>,
    #[doc = "An array of sub types."]
    #[serde(
        rename = "subTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sub_types: Vec<String>,
    #[doc = "An array of super types."]
    #[serde(
        rename = "superTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub super_types: Vec<String>,
}
impl AtlasClassificationDef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "REST serialization friendly list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasClassifications {
    #[doc = "An array of objects."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub list: Vec<serde_json::Value>,
    #[doc = "The size of the page."]
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[doc = "The sorted by field."]
    #[serde(rename = "sortBy", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[doc = "Type for sorting"]
    #[serde(rename = "sortType", default, skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<SortType>,
    #[doc = "The start index of the page."]
    #[serde(rename = "startIndex", default, skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
    #[doc = "The total count of items."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}
impl AtlasClassifications {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "class that captures details of a constraint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasConstraintDef {
    #[doc = "The parameters of the constraint definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
    #[doc = "The type of the constraint."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl AtlasConstraintDef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An instance of an entity along with extended info - like hive_table,\nhive_database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasEntitiesWithExtInfo {
    #[doc = "The referred entities."]
    #[serde(rename = "referredEntities", default, skip_serializing_if = "Option::is_none")]
    pub referred_entities: Option<serde_json::Value>,
    #[doc = "An array of entities."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub entities: Vec<AtlasEntity>,
}
impl AtlasEntitiesWithExtInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An instance of an entity - like hive_table, hive_database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasEntity {
    #[doc = "The attributes of the struct."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    #[doc = "The name of the type."]
    #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[doc = "ETag for concurrency control."]
    #[serde(rename = "lastModifiedTS", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_ts: Option<String>,
    #[doc = "Business attributes"]
    #[serde(rename = "businessAttributes", default, skip_serializing_if = "Option::is_none")]
    pub business_attributes: Option<serde_json::Value>,
    #[doc = "An array of classifications."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub classifications: Vec<AtlasClassification>,
    #[doc = "The created time of the record."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[doc = "The user who created the record."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "Custom Attribute"]
    #[serde(rename = "customAttributes", default, skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<serde_json::Value>,
    #[doc = "The GUID of the entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "The home ID of the entity."]
    #[serde(rename = "homeId", default, skip_serializing_if = "Option::is_none")]
    pub home_id: Option<String>,
    #[doc = "The collection ID of the entity."]
    #[serde(rename = "collectionId", default, skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[doc = "Whether it is a shell entity"]
    #[serde(rename = "isIncomplete", default, skip_serializing_if = "Option::is_none")]
    pub is_incomplete: Option<bool>,
    #[doc = "labels"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub labels: Vec<String>,
    #[doc = "An array of term assignment headers indicating the meanings of the entity."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub meanings: Vec<AtlasTermAssignmentHeader>,
    #[doc = "Used to record the provenance of an instance of an entity or relationship."]
    #[serde(rename = "provenanceType", default, skip_serializing_if = "Option::is_none")]
    pub provenance_type: Option<i32>,
    #[doc = "Determines if there's a proxy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy: Option<bool>,
    #[doc = "The attributes of relationship."]
    #[serde(rename = "relationshipAttributes", default, skip_serializing_if = "Option::is_none")]
    pub relationship_attributes: Option<serde_json::Value>,
    #[doc = "Status - can be active or deleted"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EntityStatus>,
    #[doc = "The update time of the record."]
    #[serde(rename = "updateTime", default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[doc = "The user who updated the record."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "The version of the entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[doc = "The dictionary of contacts for entities. Key could be Expert or Owner."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contacts: Option<serde_json::Value>,
}
impl AtlasEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "class that captures details of a entity-type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasEntityDef {
    #[doc = "Type Category"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<TypeCategory>,
    #[doc = "The created time of the record."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[doc = "The user who created the record."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The date format."]
    #[serde(rename = "dateFormatter", default, skip_serializing_if = "Option::is_none")]
    pub date_formatter: Option<DateFormat>,
    #[doc = "The description of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The GUID of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "The name of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The options for the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[doc = "The service type."]
    #[serde(rename = "serviceType", default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    #[doc = "The version of the type."]
    #[serde(rename = "typeVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_version: Option<String>,
    #[doc = "The update time of the record."]
    #[serde(rename = "updateTime", default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[doc = "The user who updated the record."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "The version of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[doc = "ETag for concurrency control."]
    #[serde(rename = "lastModifiedTS", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_ts: Option<String>,
    #[doc = "An array of attribute definitions."]
    #[serde(
        rename = "attributeDefs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub attribute_defs: Vec<AtlasAttributeDef>,
    #[doc = "An array of sub types."]
    #[serde(
        rename = "subTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sub_types: Vec<String>,
    #[doc = "An array of super types."]
    #[serde(
        rename = "superTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub super_types: Vec<String>,
    #[doc = "An array of relationship attributes."]
    #[serde(
        rename = "relationshipAttributeDefs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub relationship_attribute_defs: Vec<AtlasRelationshipAttributeDef>,
}
impl AtlasEntityDef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An instance of an entity - like hive_table, hive_database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasEntityHeader {
    #[doc = "The attributes of the struct."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    #[doc = "The name of the type."]
    #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[doc = "ETag for concurrency control."]
    #[serde(rename = "lastModifiedTS", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_ts: Option<String>,
    #[doc = "An array of classification names."]
    #[serde(
        rename = "classificationNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub classification_names: Vec<String>,
    #[doc = "An array of classifications."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub classifications: Vec<AtlasClassification>,
    #[doc = "The display text."]
    #[serde(rename = "displayText", default, skip_serializing_if = "Option::is_none")]
    pub display_text: Option<String>,
    #[doc = "The GUID of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "Whether it is a shell entity"]
    #[serde(rename = "isIncomplete", default, skip_serializing_if = "Option::is_none")]
    pub is_incomplete: Option<bool>,
    #[doc = "labels"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub labels: Vec<String>,
    #[doc = "An array of meanings."]
    #[serde(
        rename = "meaningNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub meaning_names: Vec<String>,
    #[doc = "An array of term assignment headers."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub meanings: Vec<AtlasTermAssignmentHeader>,
    #[doc = "Status - can be active or deleted"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EntityStatus>,
}
impl AtlasEntityHeader {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An instance of an entity header map."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasEntityHeaders {
    #[doc = "The description of the guid header map,"]
    #[serde(rename = "guidHeaderMap", default, skip_serializing_if = "Option::is_none")]
    pub guid_header_map: Option<serde_json::Value>,
}
impl AtlasEntityHeaders {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An instance of an entity along with extended info - like hive_table,\nhive_database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasEntityWithExtInfo {
    #[doc = "The referred entities."]
    #[serde(rename = "referredEntities", default, skip_serializing_if = "Option::is_none")]
    pub referred_entities: Option<serde_json::Value>,
    #[doc = "An instance of an entity - like hive_table, hive_database."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity: Option<AtlasEntity>,
}
impl AtlasEntityWithExtInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "class that captures details of an enum-type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasEnumDef {
    #[doc = "Type Category"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<TypeCategory>,
    #[doc = "The created time of the record."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[doc = "The user who created the record."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The date format."]
    #[serde(rename = "dateFormatter", default, skip_serializing_if = "Option::is_none")]
    pub date_formatter: Option<DateFormat>,
    #[doc = "The description of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The GUID of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "The name of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The options for the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[doc = "The service type."]
    #[serde(rename = "serviceType", default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    #[doc = "The version of the type."]
    #[serde(rename = "typeVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_version: Option<String>,
    #[doc = "The update time of the record."]
    #[serde(rename = "updateTime", default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[doc = "The user who updated the record."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "The version of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[doc = "ETag for concurrency control."]
    #[serde(rename = "lastModifiedTS", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_ts: Option<String>,
    #[doc = "The default value."]
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[doc = "An array of enum element definitions."]
    #[serde(
        rename = "elementDefs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub element_defs: Vec<AtlasEnumElementDef>,
}
impl AtlasEnumDef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "class that captures details of an enum-element."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasEnumElementDef {
    #[doc = "The description of the enum element definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The ordinal of the enum element definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ordinal: Option<i32>,
    #[doc = "The value of the enum element definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl AtlasEnumElementDef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasErrorResponse {
    #[doc = "Universally Unique Identifier"]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<AzureCoreUuid>,
    #[doc = "The error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "The error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl AtlasErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The glossary object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasGlossary {
    #[doc = "The GUID of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "An array of classifications."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub classifications: Vec<AtlasClassification>,
    #[doc = "The long version description."]
    #[serde(rename = "longDescription", default, skip_serializing_if = "Option::is_none")]
    pub long_description: Option<String>,
    #[doc = "The name of the glossary object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The qualified name of the glossary object."]
    #[serde(rename = "qualifiedName", default, skip_serializing_if = "Option::is_none")]
    pub qualified_name: Option<String>,
    #[doc = "The short version of description."]
    #[serde(rename = "shortDescription", default, skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    #[doc = "ETag for concurrency control."]
    #[serde(rename = "lastModifiedTS", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_ts: Option<String>,
    #[doc = "The created time of the record."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[doc = "The user who created the record."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The update time of the record."]
    #[serde(rename = "updateTime", default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[doc = "The user who updated the record."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "An array of categories."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub categories: Vec<AtlasRelatedCategoryHeader>,
    #[doc = "The language of the glossary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[doc = "An array of related term headers."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub terms: Vec<AtlasRelatedTermHeader>,
    #[doc = "The usage of the glossary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
}
impl AtlasGlossary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The glossary category."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasGlossaryCategory {
    #[doc = "The GUID of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "An array of classifications."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub classifications: Vec<AtlasClassification>,
    #[doc = "The long version description."]
    #[serde(rename = "longDescription", default, skip_serializing_if = "Option::is_none")]
    pub long_description: Option<String>,
    #[doc = "The name of the glossary object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The qualified name of the glossary object."]
    #[serde(rename = "qualifiedName", default, skip_serializing_if = "Option::is_none")]
    pub qualified_name: Option<String>,
    #[doc = "The short version of description."]
    #[serde(rename = "shortDescription", default, skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    #[doc = "ETag for concurrency control."]
    #[serde(rename = "lastModifiedTS", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_ts: Option<String>,
    #[doc = "The created time of the record."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[doc = "The user who created the record."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The update time of the record."]
    #[serde(rename = "updateTime", default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[doc = "The user who updated the record."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "The glossary header with basic information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<AtlasGlossaryHeader>,
    #[doc = "An array of children categories."]
    #[serde(
        rename = "childrenCategories",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub children_categories: Vec<AtlasRelatedCategoryHeader>,
    #[doc = "The header of the related category."]
    #[serde(rename = "parentCategory", default, skip_serializing_if = "Option::is_none")]
    pub parent_category: Option<AtlasRelatedCategoryHeader>,
    #[doc = "An array of related term headers."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub terms: Vec<AtlasRelatedTermHeader>,
}
impl AtlasGlossaryCategory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The extended information of glossary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasGlossaryExtInfo {
    #[doc = "The GUID of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "An array of classifications."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub classifications: Vec<AtlasClassification>,
    #[doc = "The long version description."]
    #[serde(rename = "longDescription", default, skip_serializing_if = "Option::is_none")]
    pub long_description: Option<String>,
    #[doc = "The name of the glossary object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The qualified name of the glossary object."]
    #[serde(rename = "qualifiedName", default, skip_serializing_if = "Option::is_none")]
    pub qualified_name: Option<String>,
    #[doc = "The short version of description."]
    #[serde(rename = "shortDescription", default, skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    #[doc = "ETag for concurrency control."]
    #[serde(rename = "lastModifiedTS", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_ts: Option<String>,
    #[doc = "The created time of the record."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[doc = "The user who created the record."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The update time of the record."]
    #[serde(rename = "updateTime", default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[doc = "The user who updated the record."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "An array of categories."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub categories: Vec<AtlasRelatedCategoryHeader>,
    #[doc = "The language of the glossary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[doc = "An array of related term headers."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub terms: Vec<AtlasRelatedTermHeader>,
    #[doc = "The usage of the glossary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
    #[doc = "The glossary category information."]
    #[serde(rename = "categoryInfo", default, skip_serializing_if = "Option::is_none")]
    pub category_info: Option<serde_json::Value>,
    #[doc = "The glossary term information."]
    #[serde(rename = "termInfo", default, skip_serializing_if = "Option::is_none")]
    pub term_info: Option<serde_json::Value>,
}
impl AtlasGlossaryExtInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The glossary header with basic information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasGlossaryHeader {
    #[doc = "The display text."]
    #[serde(rename = "displayText", default, skip_serializing_if = "Option::is_none")]
    pub display_text: Option<String>,
    #[doc = "The GUID of the glossary."]
    #[serde(rename = "glossaryGuid", default, skip_serializing_if = "Option::is_none")]
    pub glossary_guid: Option<String>,
    #[doc = "The GUID of the relationship."]
    #[serde(rename = "relationGuid", default, skip_serializing_if = "Option::is_none")]
    pub relation_guid: Option<String>,
}
impl AtlasGlossaryHeader {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The glossary term."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasGlossaryTerm {
    #[doc = "The GUID of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "An array of classifications."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub classifications: Vec<AtlasClassification>,
    #[doc = "The long version description."]
    #[serde(rename = "longDescription", default, skip_serializing_if = "Option::is_none")]
    pub long_description: Option<String>,
    #[doc = "The name of the glossary object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The qualified name of the glossary object."]
    #[serde(rename = "qualifiedName", default, skip_serializing_if = "Option::is_none")]
    pub qualified_name: Option<String>,
    #[doc = "The short version of description."]
    #[serde(rename = "shortDescription", default, skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    #[doc = "ETag for concurrency control."]
    #[serde(rename = "lastModifiedTS", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_ts: Option<String>,
    #[doc = "The created time of the record."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[doc = "The user who created the record."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The update time of the record."]
    #[serde(rename = "updateTime", default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[doc = "The user who updated the record."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "The abbreviation of the term."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,
    #[doc = "The name of the template."]
    #[serde(
        rename = "templateName",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub template_name: Vec<serde_json::Value>,
    #[doc = "The glossary header with basic information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<AtlasGlossaryHeader>,
    #[doc = "An array of related term headers as antonyms."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub antonyms: Vec<AtlasRelatedTermHeader>,
    #[doc = "Status for term"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<TermStatus>,
    #[doc = "The nick name of the term."]
    #[serde(rename = "nickName", default, skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    #[doc = "The hierarchy information of the term."]
    #[serde(
        rename = "hierarchyInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hierarchy_info: Vec<PurviewObjectId>,
    #[doc = "An array of resource link for term"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resources: Vec<ResourceLink>,
    #[doc = "The dictionary of contacts for terms. Key could be Expert or Steward."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contacts: Option<serde_json::Value>,
    #[doc = "The custom attributes of the term, which is map<string,map<string,object>>.\nThe\nkey of the first layer map is term template name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    #[doc = "An array of related object IDs."]
    #[serde(
        rename = "assignedEntities",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub assigned_entities: Vec<AtlasRelatedObjectId>,
    #[doc = "An array of term categorization headers."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub categories: Vec<AtlasTermCategorizationHeader>,
    #[doc = "An array of related term headers."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub classifies: Vec<AtlasRelatedTermHeader>,
    #[doc = "An array of examples."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub examples: Vec<String>,
    #[doc = "An array of related term headers indicating the is-a relationship."]
    #[serde(
        rename = "isA",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub is_a: Vec<AtlasRelatedTermHeader>,
    #[doc = "An array of preferred related term headers."]
    #[serde(
        rename = "preferredTerms",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub preferred_terms: Vec<AtlasRelatedTermHeader>,
    #[doc = "An array of related term headers that are preferred to."]
    #[serde(
        rename = "preferredToTerms",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub preferred_to_terms: Vec<AtlasRelatedTermHeader>,
    #[doc = "An array of related term headers that are replaced by."]
    #[serde(
        rename = "replacedBy",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub replaced_by: Vec<AtlasRelatedTermHeader>,
    #[doc = "An array of related term headers for replacement."]
    #[serde(
        rename = "replacementTerms",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub replacement_terms: Vec<AtlasRelatedTermHeader>,
    #[doc = "An array of related term headers for see also."]
    #[serde(
        rename = "seeAlso",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub see_also: Vec<AtlasRelatedTermHeader>,
    #[doc = "An array of related term headers as synonyms."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub synonyms: Vec<AtlasRelatedTermHeader>,
    #[doc = "An array of translated related term headers."]
    #[serde(
        rename = "translatedTerms",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub translated_terms: Vec<AtlasRelatedTermHeader>,
    #[doc = "An array of related term headers for translation."]
    #[serde(
        rename = "translationTerms",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub translation_terms: Vec<AtlasRelatedTermHeader>,
    #[doc = "The usage of the term."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
    #[doc = "An array of related term headers as valid values."]
    #[serde(
        rename = "validValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub valid_values: Vec<AtlasRelatedTermHeader>,
    #[doc = "An array of related term headers as valid values for other records."]
    #[serde(
        rename = "validValuesFor",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub valid_values_for: Vec<AtlasRelatedTermHeader>,
}
impl AtlasGlossaryTerm {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The lineage information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasLineageInfo {
    #[doc = "The GUID of the base entity."]
    #[serde(rename = "baseEntityGuid", default, skip_serializing_if = "Option::is_none")]
    pub base_entity_guid: Option<String>,
    #[doc = "The GUID entity map."]
    #[serde(rename = "guidEntityMap", default, skip_serializing_if = "Option::is_none")]
    pub guid_entity_map: Option<serde_json::Value>,
    #[doc = "The entity count in specific direction."]
    #[serde(rename = "widthCounts", default, skip_serializing_if = "Option::is_none")]
    pub width_counts: Option<serde_json::Value>,
    #[doc = "The depth of lineage."]
    #[serde(rename = "lineageDepth", default, skip_serializing_if = "Option::is_none")]
    pub lineage_depth: Option<i32>,
    #[doc = "The width of lineage."]
    #[serde(rename = "lineageWidth", default, skip_serializing_if = "Option::is_none")]
    pub lineage_width: Option<i32>,
    #[doc = "The number of children node."]
    #[serde(rename = "childrenCount", default, skip_serializing_if = "Option::is_none")]
    pub children_count: Option<i32>,
    #[doc = "Lineage direction"]
    #[serde(rename = "lineageDirection", default, skip_serializing_if = "Option::is_none")]
    pub lineage_direction: Option<LineageDirection>,
    #[doc = "An array of parentRelations relations."]
    #[serde(
        rename = "parentRelations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parent_relations: Vec<ParentRelation>,
    #[doc = "An array of lineage relations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub relations: Vec<LineageRelation>,
}
impl AtlasLineageInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to an object-instance of a type - like entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasObjectId {
    #[doc = "The GUID of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "The name of the type."]
    #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[doc = "The unique attributes of the object."]
    #[serde(rename = "uniqueAttributes", default, skip_serializing_if = "Option::is_none")]
    pub unique_attributes: Option<serde_json::Value>,
}
impl AtlasObjectId {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The header of the related category."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasRelatedCategoryHeader {
    #[doc = "The GUID of the category."]
    #[serde(rename = "categoryGuid", default, skip_serializing_if = "Option::is_none")]
    pub category_guid: Option<String>,
    #[doc = "The description of the category header."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The display text."]
    #[serde(rename = "displayText", default, skip_serializing_if = "Option::is_none")]
    pub display_text: Option<String>,
    #[doc = "The GUID of the parent category."]
    #[serde(rename = "parentCategoryGuid", default, skip_serializing_if = "Option::is_none")]
    pub parent_category_guid: Option<String>,
    #[doc = "The GUID of the relationship."]
    #[serde(rename = "relationGuid", default, skip_serializing_if = "Option::is_none")]
    pub relation_guid: Option<String>,
}
impl AtlasRelatedCategoryHeader {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to an object-instance of AtlasEntity type used in relationship\nattribute values"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasRelatedObjectId {
    #[doc = "The GUID of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "The name of the type."]
    #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[doc = "The unique attributes of the object."]
    #[serde(rename = "uniqueAttributes", default, skip_serializing_if = "Option::is_none")]
    pub unique_attributes: Option<serde_json::Value>,
    #[doc = "The display text."]
    #[serde(rename = "displayText", default, skip_serializing_if = "Option::is_none")]
    pub display_text: Option<String>,
    #[doc = "Status - can be active or deleted"]
    #[serde(rename = "entityStatus", default, skip_serializing_if = "Option::is_none")]
    pub entity_status: Option<EntityStatus>,
    #[doc = "Relationship type"]
    #[serde(rename = "relationshipType", default, skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<String>,
    #[doc = "Captures details of struct contents. Not instantiated directly, used only via\nAtlasEntity, AtlasClassification."]
    #[serde(rename = "relationshipAttributes", default, skip_serializing_if = "Option::is_none")]
    pub relationship_attributes: Option<AtlasStruct>,
    #[doc = "Universally Unique Identifier"]
    #[serde(rename = "relationshipGuid", default, skip_serializing_if = "Option::is_none")]
    pub relationship_guid: Option<AzureCoreUuid>,
    #[doc = "Status for atlas relationship"]
    #[serde(rename = "relationshipStatus", default, skip_serializing_if = "Option::is_none")]
    pub relationship_status: Option<StatusAtlasRelationship>,
}
impl AtlasRelatedObjectId {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The header of the related term."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasRelatedTermHeader {
    #[doc = "The description of the related term."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The display text."]
    #[serde(rename = "displayText", default, skip_serializing_if = "Option::is_none")]
    pub display_text: Option<String>,
    #[doc = "The expression of the term."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[doc = "The GUID of the relationship."]
    #[serde(rename = "relationGuid", default, skip_serializing_if = "Option::is_none")]
    pub relation_guid: Option<String>,
    #[doc = "Status for atlas term relationship"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AtlasTermRelationshipStatus>,
    #[doc = "The steward of the term."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steward: Option<String>,
    #[doc = "The GUID of the term."]
    #[serde(rename = "termGuid", default, skip_serializing_if = "Option::is_none")]
    pub term_guid: Option<String>,
}
impl AtlasRelatedTermHeader {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Atlas relationship instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasRelationship {
    #[doc = "The attributes of the struct."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    #[doc = "The name of the type."]
    #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[doc = "ETag for concurrency control."]
    #[serde(rename = "lastModifiedTS", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_ts: Option<String>,
    #[doc = "The created time of the record."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[doc = "The user who created the record."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "Reference to an object-instance of a type - like entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end1: Option<AtlasObjectId>,
    #[doc = "Reference to an object-instance of a type - like entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end2: Option<AtlasObjectId>,
    #[doc = "The GUID of the relationship."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "The home ID of the relationship."]
    #[serde(rename = "homeId", default, skip_serializing_if = "Option::is_none")]
    pub home_id: Option<String>,
    #[doc = "The label of the relationship."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "Used to record the provenance of an instance of an entity or relationship"]
    #[serde(rename = "provenanceType", default, skip_serializing_if = "Option::is_none")]
    pub provenance_type: Option<i32>,
    #[doc = "Status for atlas relationship"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusAtlasRelationship>,
    #[doc = "The update time of the record."]
    #[serde(rename = "updateTime", default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[doc = "The user who updated the record."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "The version of the relationship."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl AtlasRelationship {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The relationshipEndDef represents an end of the relationship. The end of the\nrelationship is defined by a type, an\nattribute name, cardinality and whether\nit  is the container end of the relationship."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasRelationshipAttributeDef {
    #[doc = "Cardinality"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cardinality: Option<CardinalityValue>,
    #[doc = "An array of constraints."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub constraints: Vec<AtlasConstraintDef>,
    #[doc = "The default value of the attribute."]
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[doc = "The description of the attribute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Determines if it is included in notification."]
    #[serde(rename = "includeInNotification", default, skip_serializing_if = "Option::is_none")]
    pub include_in_notification: Option<bool>,
    #[doc = "Determines if it is indexable."]
    #[serde(rename = "isIndexable", default, skip_serializing_if = "Option::is_none")]
    pub is_indexable: Option<bool>,
    #[doc = "Determines if it is optional."]
    #[serde(rename = "isOptional", default, skip_serializing_if = "Option::is_none")]
    pub is_optional: Option<bool>,
    #[doc = "Determines if it unique."]
    #[serde(rename = "isUnique", default, skip_serializing_if = "Option::is_none")]
    pub is_unique: Option<bool>,
    #[doc = "The name of the attribute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The options for the attribute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[doc = "The name of the type."]
    #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[doc = "The maximum count of the values."]
    #[serde(rename = "valuesMaxCount", default, skip_serializing_if = "Option::is_none")]
    pub values_max_count: Option<i32>,
    #[doc = "The minimum count of the values."]
    #[serde(rename = "valuesMinCount", default, skip_serializing_if = "Option::is_none")]
    pub values_min_count: Option<i32>,
    #[doc = "Determines if it is a legacy attribute."]
    #[serde(rename = "isLegacyAttribute", default, skip_serializing_if = "Option::is_none")]
    pub is_legacy_attribute: Option<bool>,
    #[doc = "The name of the relationship type."]
    #[serde(rename = "relationshipTypeName", default, skip_serializing_if = "Option::is_none")]
    pub relationship_type_name: Option<String>,
}
impl AtlasRelationshipAttributeDef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AtlasRelationshipDef is a TypeDef that defines a relationship.\nAs with other typeDefs the AtlasRelationshipDef has a name. Once created the\nRelationshipDef has a guid.\nThe name and the guid are the 2 ways that the\nRelationshipDef is identified.\nRelationshipDefs have 2 ends, each of which\nspecify cardinality, an EntityDef type name and name and optionally\nwhether the\nend is a container.\nRelationshipDefs can have AttributeDefs - though only\nprimitive types are allowed. \nRelationshipDefs have a relationshipCategory\nspecifying the UML type of relationship required \nThe way EntityDefs and\nRelationshipDefs are intended to be used is that EntityDefs will define\nAttributeDefs these AttributeDefs\nwill not specify an EntityDef type name as\ntheir types.\nRelationshipDefs introduce new attributes to the entity\ninstances. For example\nEntityDef A might have attributes attr1,attr2,attr3\n\nEntityDef B might have attributes attr4,attr5,attr6 \nRelationshipDef\nAtoB might define 2 ends \n\nend1:  type A, name attr7\nend2:  type B, name attr8 \n\nWhen an instance of EntityDef A is created, it\nwill have attributes attr1,attr2,attr3,attr7 \nWhen an instance of EntityDef\nB is created, it will have attributes attr4,attr5,attr6,attr8\n\nIn this way\nrelationshipDefs can be authored separately from entityDefs and can inject\nrelationship attributes into\nthe entity instances"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasRelationshipDef {
    #[doc = "Type Category"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<TypeCategory>,
    #[doc = "The created time of the record."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[doc = "The user who created the record."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The date format."]
    #[serde(rename = "dateFormatter", default, skip_serializing_if = "Option::is_none")]
    pub date_formatter: Option<DateFormat>,
    #[doc = "The description of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The GUID of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "The name of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The options for the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[doc = "The service type."]
    #[serde(rename = "serviceType", default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    #[doc = "The version of the type."]
    #[serde(rename = "typeVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_version: Option<String>,
    #[doc = "The update time of the record."]
    #[serde(rename = "updateTime", default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[doc = "The user who updated the record."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "The version of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[doc = "ETag for concurrency control."]
    #[serde(rename = "lastModifiedTS", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_ts: Option<String>,
    #[doc = "An array of attribute definitions."]
    #[serde(
        rename = "attributeDefs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub attribute_defs: Vec<AtlasAttributeDef>,
    #[doc = "The relationshipEndDef represents an end of the relationship. The end of the\nrelationship is defined by a type, an\nattribute name, cardinality and whether\nit  is the container end of the relationship."]
    #[serde(rename = "endDef1", default, skip_serializing_if = "Option::is_none")]
    pub end_def1: Option<AtlasRelationshipEndDef>,
    #[doc = "The relationshipEndDef represents an end of the relationship. The end of the\nrelationship is defined by a type, an\nattribute name, cardinality and whether\nit  is the container end of the relationship."]
    #[serde(rename = "endDef2", default, skip_serializing_if = "Option::is_none")]
    pub end_def2: Option<AtlasRelationshipEndDef>,
    #[doc = "Relationship Category"]
    #[serde(rename = "relationshipCategory", default, skip_serializing_if = "Option::is_none")]
    pub relationship_category: Option<RelationshipCategory>,
    #[doc = "The label of the relationship."]
    #[serde(rename = "relationshipLabel", default, skip_serializing_if = "Option::is_none")]
    pub relationship_label: Option<String>,
}
impl AtlasRelationshipDef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The relationshipEndDef represents an end of the relationship. The end of the\nrelationship is defined by a type, an\nattribute name, cardinality and whether\nit  is the container end of the relationship."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasRelationshipEndDef {
    #[doc = "Cardinality"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cardinality: Option<CardinalityValue>,
    #[doc = "The description of the relationship end definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Determines if it is container."]
    #[serde(rename = "isContainer", default, skip_serializing_if = "Option::is_none")]
    pub is_container: Option<bool>,
    #[doc = "Determines if it is a legacy attribute."]
    #[serde(rename = "isLegacyAttribute", default, skip_serializing_if = "Option::is_none")]
    pub is_legacy_attribute: Option<bool>,
    #[doc = "The name of the relationship end definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the relationship end."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl AtlasRelationshipEndDef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The relationship with extended information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasRelationshipWithExtInfo {
    #[doc = "The referred entity header."]
    #[serde(rename = "referredEntities", default, skip_serializing_if = "Option::is_none")]
    pub referred_entities: Option<serde_json::Value>,
    #[doc = "Atlas relationship instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relationship: Option<AtlasRelationship>,
}
impl AtlasRelationshipWithExtInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Captures details of struct contents. Not instantiated directly, used only via\nAtlasEntity, AtlasClassification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasStruct {
    #[doc = "The attributes of the struct."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    #[doc = "The name of the type."]
    #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[doc = "ETag for concurrency control."]
    #[serde(rename = "lastModifiedTS", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_ts: Option<String>,
}
impl AtlasStruct {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "class that captures details of a struct-type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasStructDef {
    #[doc = "Type Category"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<TypeCategory>,
    #[doc = "The created time of the record."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[doc = "The user who created the record."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The date format."]
    #[serde(rename = "dateFormatter", default, skip_serializing_if = "Option::is_none")]
    pub date_formatter: Option<DateFormat>,
    #[doc = "The description of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The GUID of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "The name of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The options for the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[doc = "The service type."]
    #[serde(rename = "serviceType", default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    #[doc = "The version of the type."]
    #[serde(rename = "typeVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_version: Option<String>,
    #[doc = "The update time of the record."]
    #[serde(rename = "updateTime", default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[doc = "The user who updated the record."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "The version of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[doc = "ETag for concurrency control."]
    #[serde(rename = "lastModifiedTS", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_ts: Option<String>,
    #[doc = "An array of attribute definitions."]
    #[serde(
        rename = "attributeDefs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub attribute_defs: Vec<AtlasAttributeDef>,
}
impl AtlasStructDef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The header for term assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasTermAssignmentHeader {
    #[doc = "The confidence of the term assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<i32>,
    #[doc = "The user who created the record."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The description of the term assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The display text."]
    #[serde(rename = "displayText", default, skip_serializing_if = "Option::is_none")]
    pub display_text: Option<String>,
    #[doc = "The expression of the term assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[doc = "Universally Unique Identifier"]
    #[serde(rename = "relationGuid", default, skip_serializing_if = "Option::is_none")]
    pub relation_guid: Option<AzureCoreUuid>,
    #[doc = "Status for term assignment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AtlasTermAssignmentStatus>,
    #[doc = "The steward of the term."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steward: Option<String>,
    #[doc = "Universally Unique Identifier"]
    #[serde(rename = "termGuid", default, skip_serializing_if = "Option::is_none")]
    pub term_guid: Option<AzureCoreUuid>,
}
impl AtlasTermAssignmentHeader {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status for term assignment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AtlasTermAssignmentStatus")]
pub enum AtlasTermAssignmentStatus {
    #[serde(rename = "DISCOVERED")]
    Discovered,
    #[serde(rename = "PROPOSED")]
    Proposed,
    #[serde(rename = "IMPORTED")]
    Imported,
    #[serde(rename = "VALIDATED")]
    Validated,
    #[serde(rename = "DEPRECATED")]
    Deprecated,
    #[serde(rename = "OBSOLETE")]
    Obsolete,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AtlasTermAssignmentStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AtlasTermAssignmentStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AtlasTermAssignmentStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Discovered => serializer.serialize_unit_variant("AtlasTermAssignmentStatus", 0u32, "DISCOVERED"),
            Self::Proposed => serializer.serialize_unit_variant("AtlasTermAssignmentStatus", 1u32, "PROPOSED"),
            Self::Imported => serializer.serialize_unit_variant("AtlasTermAssignmentStatus", 2u32, "IMPORTED"),
            Self::Validated => serializer.serialize_unit_variant("AtlasTermAssignmentStatus", 3u32, "VALIDATED"),
            Self::Deprecated => serializer.serialize_unit_variant("AtlasTermAssignmentStatus", 4u32, "DEPRECATED"),
            Self::Obsolete => serializer.serialize_unit_variant("AtlasTermAssignmentStatus", 5u32, "OBSOLETE"),
            Self::Other => serializer.serialize_unit_variant("AtlasTermAssignmentStatus", 6u32, "OTHER"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The basic information for term categorization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasTermCategorizationHeader {
    #[doc = "Universally Unique Identifier"]
    #[serde(rename = "categoryGuid", default, skip_serializing_if = "Option::is_none")]
    pub category_guid: Option<AzureCoreUuid>,
    #[doc = "The description of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The display text."]
    #[serde(rename = "displayText", default, skip_serializing_if = "Option::is_none")]
    pub display_text: Option<String>,
    #[doc = "Universally Unique Identifier"]
    #[serde(rename = "relationGuid", default, skip_serializing_if = "Option::is_none")]
    pub relation_guid: Option<AzureCoreUuid>,
    #[doc = "Status for atlas term relationship"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AtlasTermRelationshipStatus>,
}
impl AtlasTermCategorizationHeader {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status for atlas term relationship"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AtlasTermRelationshipStatus")]
pub enum AtlasTermRelationshipStatus {
    #[serde(rename = "DRAFT")]
    Draft,
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "DEPRECATED")]
    Deprecated,
    #[serde(rename = "OBSOLETE")]
    Obsolete,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AtlasTermRelationshipStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AtlasTermRelationshipStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AtlasTermRelationshipStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Draft => serializer.serialize_unit_variant("AtlasTermRelationshipStatus", 0u32, "DRAFT"),
            Self::Active => serializer.serialize_unit_variant("AtlasTermRelationshipStatus", 1u32, "ACTIVE"),
            Self::Deprecated => serializer.serialize_unit_variant("AtlasTermRelationshipStatus", 2u32, "DEPRECATED"),
            Self::Obsolete => serializer.serialize_unit_variant("AtlasTermRelationshipStatus", 3u32, "OBSOLETE"),
            Self::Other => serializer.serialize_unit_variant("AtlasTermRelationshipStatus", 4u32, "OTHER"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The definitions of type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasTypeDef {
    #[doc = "Type Category"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<TypeCategory>,
    #[doc = "The created time of the record."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[doc = "The user who created the record."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The date format."]
    #[serde(rename = "dateFormatter", default, skip_serializing_if = "Option::is_none")]
    pub date_formatter: Option<DateFormat>,
    #[doc = "The description of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The GUID of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "The name of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The options for the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[doc = "The service type."]
    #[serde(rename = "serviceType", default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    #[doc = "The version of the type."]
    #[serde(rename = "typeVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_version: Option<String>,
    #[doc = "The update time of the record."]
    #[serde(rename = "updateTime", default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[doc = "The user who updated the record."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "The version of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[doc = "ETag for concurrency control."]
    #[serde(rename = "lastModifiedTS", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_ts: Option<String>,
    #[doc = "Specifying a list of entityType names in the classificationDef, ensures that\nclassifications can\nonly be applied to those entityTypes.\n\nAny subtypes of the entity types inherit the restriction.\n\nAny classificationDef subtypes inherit the parents entityTypes restrictions.\n\nAny classificationDef subtypes can further restrict the parents entityTypes\nrestrictions by specifying a subset of the entityTypes.\n\nAn empty entityTypes list when there are no parent restrictions means there are no\nrestrictions.\n\nAn empty entityTypes list when there are parent\nrestrictions means that the subtype picks up the parents\nrestrictions.\n\nIf a list of entityTypes are supplied, where one inherits\nfrom another, this will be rejected. This should encourage cleaner\nclassificationsDefs.\n"]
    #[serde(
        rename = "entityTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub entity_types: Vec<String>,
    #[doc = "An array of sub types."]
    #[serde(
        rename = "subTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sub_types: Vec<String>,
    #[doc = "An array of super types."]
    #[serde(
        rename = "superTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub super_types: Vec<String>,
    #[doc = "An array of relationship attributes."]
    #[serde(
        rename = "relationshipAttributeDefs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub relationship_attribute_defs: Vec<AtlasRelationshipAttributeDef>,
    #[doc = "The default value."]
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[doc = "An array of enum element definitions."]
    #[serde(
        rename = "elementDefs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub element_defs: Vec<AtlasEnumElementDef>,
    #[doc = "The relationshipEndDef represents an end of the relationship. The end of the\nrelationship is defined by a type, an\nattribute name, cardinality and whether\nit  is the container end of the relationship."]
    #[serde(rename = "endDef1", default, skip_serializing_if = "Option::is_none")]
    pub end_def1: Option<AtlasRelationshipEndDef>,
    #[doc = "The relationshipEndDef represents an end of the relationship. The end of the\nrelationship is defined by a type, an\nattribute name, cardinality and whether\nit  is the container end of the relationship."]
    #[serde(rename = "endDef2", default, skip_serializing_if = "Option::is_none")]
    pub end_def2: Option<AtlasRelationshipEndDef>,
    #[doc = "Relationship Category"]
    #[serde(rename = "relationshipCategory", default, skip_serializing_if = "Option::is_none")]
    pub relationship_category: Option<RelationshipCategory>,
    #[doc = "The label of the relationship."]
    #[serde(rename = "relationshipLabel", default, skip_serializing_if = "Option::is_none")]
    pub relationship_label: Option<String>,
    #[doc = "An array of attribute definitions."]
    #[serde(
        rename = "attributeDefs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub attribute_defs: Vec<AtlasAttributeDef>,
}
impl AtlasTypeDef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The basic information of the type definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasTypeDefHeader {
    #[doc = "Type Category"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<TypeCategory>,
    #[doc = "The GUID of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "The name of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl AtlasTypeDefHeader {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The definitions of types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasTypesDef {
    #[doc = "businessMetadataDefs"]
    #[serde(
        rename = "businessMetadataDefs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub business_metadata_defs: Vec<AtlasBusinessMetadataDef>,
    #[doc = "An array of classification definitions."]
    #[serde(
        rename = "classificationDefs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub classification_defs: Vec<AtlasClassificationDef>,
    #[doc = "An array of entity definitions."]
    #[serde(
        rename = "entityDefs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub entity_defs: Vec<AtlasEntityDef>,
    #[doc = "An array of enum definitions."]
    #[serde(
        rename = "enumDefs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enum_defs: Vec<AtlasEnumDef>,
    #[doc = "An array of relationship definitions."]
    #[serde(
        rename = "relationshipDefs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub relationship_defs: Vec<AtlasRelationshipDef>,
    #[doc = "An array of struct definitions."]
    #[serde(
        rename = "structDefs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub struct_defs: Vec<AtlasStructDef>,
    #[doc = "An array of term template definitions."]
    #[serde(
        rename = "termTemplateDefs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub term_template_defs: Vec<TermTemplateDef>,
}
impl AtlasTypesDef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The payload of autocomplete request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoCompleteOptions {
    #[doc = "The keywords applied to all fields that support autocomplete operation. It must\nbe at least 1 character, and no more than 100 characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[doc = "The number of autocomplete results we hope to return. The default value is 50.\nThe value must be a number between 1 and 100."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[doc = "The filter for the autocomplete request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
}
impl AutoCompleteOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of the autocomplete request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoCompleteResult {
    #[doc = "The result value"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AutoCompleteResultValue>,
}
impl AutoCompleteResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The value item of the autocomplete suggest."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoCompleteResultValue {
    #[doc = "The completed term or phrase."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[doc = "The completed search query text."]
    #[serde(rename = "queryPlusText", default, skip_serializing_if = "Option::is_none")]
    pub query_plus_text: Option<String>,
}
impl AutoCompleteResultValue {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type AzureCoreUuid = String;
#[doc = "Bulk import result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BulkImportResult {
    #[doc = "failed importInfoList"]
    #[serde(
        rename = "failedImportInfoList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub failed_import_info_list: Vec<ImportInfo>,
    #[doc = "successful importInfoList"]
    #[serde(
        rename = "successImportInfoList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub success_import_info_list: Vec<ImportInfo>,
}
impl BulkImportResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Business metadata to send to the service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessMetadataOptions {
    #[doc = "InputStream of file"]
    pub file: String,
}
impl BusinessMetadataOptions {
    pub fn new(file: String) -> Self {
        Self { file }
    }
}
#[doc = "Cardinality"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CardinalityValue")]
pub enum CardinalityValue {
    #[serde(rename = "SINGLE")]
    Single,
    #[serde(rename = "LIST")]
    List,
    #[serde(rename = "SET")]
    Set,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CardinalityValue {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CardinalityValue {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CardinalityValue {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Single => serializer.serialize_unit_variant("CardinalityValue", 0u32, "SINGLE"),
            Self::List => serializer.serialize_unit_variant("CardinalityValue", 1u32, "LIST"),
            Self::Set => serializer.serialize_unit_variant("CardinalityValue", 2u32, "SET"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The request payload for classification association."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClassificationAssociateOptions {
    #[doc = "An instance of a classification; it doesn't have an identity, this object\nexists only when associated with an entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<AtlasClassification>,
    #[doc = "The GUID of the entity."]
    #[serde(
        rename = "entityGuids",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub entity_guids: Vec<String>,
}
impl ClassificationAssociateOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ContactInfo"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContactInfo {
    #[doc = "Azure Active Directory object Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "additional information to describe this contact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
}
impl ContactInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The contact in the search and suggest result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContactSearchResultValue {
    #[doc = "The GUID of the contact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The description of the contact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    #[doc = "The type of the contact. It can be Expert or Owner for an entity. It can be\nExpert or Steward for a glossary term."]
    #[serde(rename = "contactType", default, skip_serializing_if = "Option::is_none")]
    pub contact_type: Option<String>,
}
impl ContactSearchResultValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The date format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DateFormat {
    #[doc = "An array of available locales."]
    #[serde(
        rename = "availableLocales",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub available_locales: Vec<String>,
    #[doc = "Calendar"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar: Option<f32>,
    #[doc = "The date format."]
    #[serde(rename = "dateInstance", default, skip_serializing_if = "Option::is_none")]
    pub date_instance: Option<Box<DateFormat>>,
    #[doc = "The date format."]
    #[serde(rename = "dateTimeInstance", default, skip_serializing_if = "Option::is_none")]
    pub date_time_instance: Option<Box<DateFormat>>,
    #[doc = "The date format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<Box<DateFormat>>,
    #[doc = "Determines the leniency of the date format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lenient: Option<bool>,
    #[doc = "The number format."]
    #[serde(rename = "numberFormat", default, skip_serializing_if = "Option::is_none")]
    pub number_format: Option<NumberFormat>,
    #[doc = "The date format."]
    #[serde(rename = "timeInstance", default, skip_serializing_if = "Option::is_none")]
    pub time_instance: Option<Box<DateFormat>>,
    #[doc = "The timezone information."]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<TimeZone>,
}
impl DateFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The mutation response result of entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityMutationResult {
    #[doc = "A map of GUID assignments with entities."]
    #[serde(rename = "guidAssignments", default, skip_serializing_if = "Option::is_none")]
    pub guid_assignments: Option<serde_json::Value>,
    #[doc = "The entity headers of mutated entities."]
    #[serde(rename = "mutatedEntities", default, skip_serializing_if = "Option::is_none")]
    pub mutated_entities: Option<serde_json::Value>,
    #[doc = "An array of entity headers that partially updated."]
    #[serde(
        rename = "partialUpdatedEntities",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub partial_updated_entities: Vec<AtlasEntityHeader>,
}
impl EntityMutationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status - can be active or deleted"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EntityStatus")]
pub enum EntityStatus {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "DELETED")]
    Deleted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EntityStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EntityStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EntityStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Active => serializer.serialize_unit_variant("EntityStatus", 0u32, "ACTIVE"),
            Self::Deleted => serializer.serialize_unit_variant("EntityStatus", 1u32, "DELETED"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "ImportInfo"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportInfo {
    #[doc = "childObjectName"]
    #[serde(rename = "childObjectName", default, skip_serializing_if = "Option::is_none")]
    pub child_object_name: Option<String>,
    #[doc = "Status for import"]
    #[serde(rename = "importStatus", default, skip_serializing_if = "Option::is_none")]
    pub import_status: Option<ImportStatus>,
    #[doc = "parentObjectName"]
    #[serde(rename = "parentObjectName", default, skip_serializing_if = "Option::is_none")]
    pub parent_object_name: Option<String>,
    #[doc = "remarks"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
}
impl ImportInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status for import"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ImportStatus")]
pub enum ImportStatus {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ImportStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ImportStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ImportStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Success => serializer.serialize_unit_variant("ImportStatus", 0u32, "SUCCESS"),
            Self::Failed => serializer.serialize_unit_variant("ImportStatus", 1u32, "FAILED"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The identifier of navigation request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemPath {
    #[doc = "The identifier of the item. The path starts with an empty string or a single slash. To navigate further, you need to concatenate with a slash to form the full itemPath using the previous navigation response's relative item path."]
    pub path: String,
    #[doc = "The extended properties of the itemPath are typically obtained from the last navigation response. While not mandatory to provide, including them can enhance performance. Otherwise, there may be some impact on performance."]
    #[serde(rename = "extendedProperties", default, skip_serializing_if = "Option::is_none")]
    pub extended_properties: Option<serde_json::Value>,
}
impl ItemPath {
    pub fn new(path: String) -> Self {
        Self {
            path,
            extended_properties: None,
        }
    }
}
#[doc = "Lineage direction"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LineageDirection")]
pub enum LineageDirection {
    #[serde(rename = "INPUT")]
    Input,
    #[serde(rename = "OUTPUT")]
    Output,
    #[serde(rename = "BOTH")]
    Both,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LineageDirection {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LineageDirection {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LineageDirection {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Input => serializer.serialize_unit_variant("LineageDirection", 0u32, "INPUT"),
            Self::Output => serializer.serialize_unit_variant("LineageDirection", 1u32, "OUTPUT"),
            Self::Both => serializer.serialize_unit_variant("LineageDirection", 2u32, "BOTH"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The lineage relation with GUID of the from and to entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LineageRelation {
    #[doc = "The GUID of from-entity."]
    #[serde(rename = "fromEntityId", default, skip_serializing_if = "Option::is_none")]
    pub from_entity_id: Option<String>,
    #[doc = "The GUID of relationship."]
    #[serde(rename = "relationshipId", default, skip_serializing_if = "Option::is_none")]
    pub relationship_id: Option<String>,
    #[doc = "The GUID of to-entity."]
    #[serde(rename = "toEntityId", default, skip_serializing_if = "Option::is_none")]
    pub to_entity_id: Option<String>,
}
impl LineageRelation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "MoveEntitiesOptions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveEntitiesOptions {
    #[doc = "An array of entity guids to be moved to target collection."]
    #[serde(
        rename = "entityGuids",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub entity_guids: Vec<String>,
}
impl MoveEntitiesOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The item payload of the NavigationResult."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NavigationItem {
    #[doc = "The name of the item."]
    pub name: String,
    #[doc = "The type name of the item. Eg. EntityType."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Indicates whether the item is a leaf node. If it is, further navigation is not possible."]
    #[serde(rename = "isLeafNode")]
    pub is_leaf_node: bool,
    #[doc = "Whether the item is Entity. If yes, can get complete definition of an entity given its itemPath"]
    #[serde(rename = "isEntity")]
    pub is_entity: bool,
    #[doc = "The identifier of navigation request."]
    #[serde(rename = "itemPath")]
    pub item_path: ItemPath,
    #[doc = "The count of the top level asset. Won't return if request payload 'includeNextLevelAssetCount' is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "The additional properties of the navigation element."]
    pub properties: serde_json::Value,
}
impl NavigationItem {
    pub fn new(
        name: String,
        type_: String,
        is_leaf_node: bool,
        is_entity: bool,
        item_path: ItemPath,
        properties: serde_json::Value,
    ) -> Self {
        Self {
            name,
            type_,
            is_leaf_node,
            is_entity,
            item_path,
            count: None,
            properties,
        }
    }
}
#[doc = "The navigation mode."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NavigationMode")]
pub enum NavigationMode {
    #[serde(rename = "assetType")]
    AssetType,
    #[serde(rename = "azureResourceHierarchy")]
    AzureResourceHierarchy,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NavigationMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NavigationMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NavigationMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AssetType => serializer.serialize_unit_variant("NavigationMode", 0u32, "assetType"),
            Self::AzureResourceHierarchy => serializer.serialize_unit_variant("NavigationMode", 1u32, "azureResourceHierarchy"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The request payload of Navigation API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NavigationRequest {
    #[doc = "The navigation mode."]
    #[serde(rename = "navigationMode", default, skip_serializing_if = "Option::is_none")]
    pub navigation_mode: Option<NavigationMode>,
    #[doc = "The identifier of navigation request."]
    #[serde(rename = "itemPath")]
    pub item_path: ItemPath,
    #[doc = "Additional properties of the item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Whether to return the next level asset count."]
    #[serde(rename = "includeNextLevelAssetCount", default, skip_serializing_if = "Option::is_none")]
    pub include_next_level_asset_count: Option<bool>,
}
impl NavigationRequest {
    pub fn new(item_path: ItemPath) -> Self {
        Self {
            navigation_mode: None,
            item_path,
            properties: None,
            include_next_level_asset_count: None,
        }
    }
}
#[doc = "The response payload of the Navigation API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NavigationResult {
    pub items: Vec<NavigationItem>,
    #[doc = "The token used to get next batch of data. Absent if there's no more data."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl NavigationResult {
    pub fn new(items: Vec<NavigationItem>) -> Self {
        Self {
            items,
            continuation_token: None,
        }
    }
}
#[doc = "The number format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NumberFormat {
    #[doc = "The number format."]
    #[serde(
        rename = "availableLocales",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub available_locales: Vec<String>,
    #[doc = "The currency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The number format."]
    #[serde(rename = "currencyInstance", default, skip_serializing_if = "Option::is_none")]
    pub currency_instance: Option<Box<NumberFormat>>,
    #[doc = "Determines if grouping is used."]
    #[serde(rename = "groupingUsed", default, skip_serializing_if = "Option::is_none")]
    pub grouping_used: Option<bool>,
    #[doc = "The number format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<Box<NumberFormat>>,
    #[doc = "The number format."]
    #[serde(rename = "integerInstance", default, skip_serializing_if = "Option::is_none")]
    pub integer_instance: Option<Box<NumberFormat>>,
    #[doc = "The maximum of fraction digits."]
    #[serde(rename = "maximumFractionDigits", default, skip_serializing_if = "Option::is_none")]
    pub maximum_fraction_digits: Option<i32>,
    #[doc = "The maximum of integer digits."]
    #[serde(rename = "maximumIntegerDigits", default, skip_serializing_if = "Option::is_none")]
    pub maximum_integer_digits: Option<i32>,
    #[doc = "The minimum of fraction digits."]
    #[serde(rename = "minimumFractionDigits", default, skip_serializing_if = "Option::is_none")]
    pub minimum_fraction_digits: Option<i32>,
    #[doc = "The minimum of integer digits."]
    #[serde(rename = "minimumIntegerDigits", default, skip_serializing_if = "Option::is_none")]
    pub minimum_integer_digits: Option<i32>,
    #[doc = "The number format."]
    #[serde(rename = "numberInstance", default, skip_serializing_if = "Option::is_none")]
    pub number_instance: Option<Box<NumberFormat>>,
    #[doc = "Determines if only integer is parsed."]
    #[serde(rename = "parseIntegerOnly", default, skip_serializing_if = "Option::is_none")]
    pub parse_integer_only: Option<bool>,
    #[doc = "The number format."]
    #[serde(rename = "percentInstance", default, skip_serializing_if = "Option::is_none")]
    pub percent_instance: Option<Box<NumberFormat>>,
    #[doc = "Rounding Mode"]
    #[serde(rename = "roundingMode", default, skip_serializing_if = "Option::is_none")]
    pub rounding_mode: Option<RoundingMode>,
}
impl NumberFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The lineage parents relation with GUID of the parent entity and to child entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParentRelation {
    #[doc = "The GUID of child entity."]
    #[serde(rename = "childEntityId", default, skip_serializing_if = "Option::is_none")]
    pub child_entity_id: Option<String>,
    #[doc = "The GUID of relationship."]
    #[serde(rename = "relationshipId", default, skip_serializing_if = "Option::is_none")]
    pub relationship_id: Option<String>,
    #[doc = "The GUID of parent entity."]
    #[serde(rename = "parentEntityId", default, skip_serializing_if = "Option::is_none")]
    pub parent_entity_id: Option<String>,
}
impl ParentRelation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "PurviewObjectId"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PurviewObjectId {
    #[doc = "The GUID of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "The name of the type."]
    #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[doc = "The unique attributes of the object."]
    #[serde(rename = "uniqueAttributes", default, skip_serializing_if = "Option::is_none")]
    pub unique_attributes: Option<serde_json::Value>,
    #[doc = "Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display text"]
    #[serde(rename = "displayText", default, skip_serializing_if = "Option::is_none")]
    pub display_text: Option<String>,
    #[doc = "Item path"]
    #[serde(rename = "itemPath", default, skip_serializing_if = "Option::is_none")]
    pub item_path: Option<String>,
    #[doc = "Resource Id"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Dictionary of <any>"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl PurviewObjectId {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The search query of advanced search request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryOptions {
    #[doc = "The keywords applied to all searchable fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[doc = "The limit of the number of the search result. default value is 50; maximum\nvalue is 1000."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[doc = "The token used to get next batch of data. Default 'Null' to get the first\nbatch, and will return new token in each response unless there's no more data."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[doc = "The sort order of search results, can specify multiple fields."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub orderby: Vec<serde_json::Value>,
    #[doc = "The filter for the search. See examples for the usage of supported filters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
    #[doc = "The facets for search. See examples for the usage of supported facets."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub facets: Vec<SearchFacetItem>,
    #[doc = "Taxonomy setting for search request"]
    #[serde(rename = "taxonomySetting", default, skip_serializing_if = "Option::is_none")]
    pub taxonomy_setting: Option<SearchTaxonomySetting>,
}
impl QueryOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of the search result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryResult {
    #[doc = "The total number of search results (not the number of documents in a single\npage)."]
    #[serde(rename = "@search.count", default, skip_serializing_if = "Option::is_none")]
    pub search_count: Option<i32>,
    #[doc = "'True' if the '@search.count' is an approximate value and vise versa."]
    #[serde(rename = "@search.count.approximate", default, skip_serializing_if = "Option::is_none")]
    pub search_count_approximate: Option<bool>,
    #[doc = "The token used to get next batch of data. Absent if there's no more data."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[doc = "A facet list that consists of index fields assetType ,classification,\ncontactId, and label. When the facet is specified in the request, the value of\nthe facet is returned as an element of @search.facets."]
    #[serde(rename = "@search.facets", default, skip_serializing_if = "Option::is_none")]
    pub search_facets: Option<SearchFacetResultValue>,
    #[doc = "Search result value"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SearchResultValue>,
}
impl QueryResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Relationship Category"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RelationshipCategory")]
pub enum RelationshipCategory {
    #[serde(rename = "ASSOCIATION")]
    Association,
    #[serde(rename = "AGGREGATION")]
    Aggregation,
    #[serde(rename = "COMPOSITION")]
    Composition,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RelationshipCategory {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RelationshipCategory {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RelationshipCategory {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Association => serializer.serialize_unit_variant("RelationshipCategory", 0u32, "ASSOCIATION"),
            Self::Aggregation => serializer.serialize_unit_variant("RelationshipCategory", 1u32, "AGGREGATION"),
            Self::Composition => serializer.serialize_unit_variant("RelationshipCategory", 2u32, "COMPOSITION"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "ResourceLink"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceLink {
    #[doc = "Display name for url."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "web url. http or https"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl ResourceLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Rounding Mode"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RoundingMode")]
pub enum RoundingMode {
    #[serde(rename = "UP")]
    Up,
    #[serde(rename = "DOWN")]
    Down,
    #[serde(rename = "CEILING")]
    Ceiling,
    #[serde(rename = "FLOOR")]
    Floor,
    #[serde(rename = "HALF_UP")]
    HalfUp,
    #[serde(rename = "HALF_DOWN")]
    HalfDown,
    #[serde(rename = "HALF_EVEN")]
    HalfEven,
    #[serde(rename = "UNNECESSARY")]
    Unnecessary,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RoundingMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RoundingMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RoundingMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Up => serializer.serialize_unit_variant("RoundingMode", 0u32, "UP"),
            Self::Down => serializer.serialize_unit_variant("RoundingMode", 1u32, "DOWN"),
            Self::Ceiling => serializer.serialize_unit_variant("RoundingMode", 2u32, "CEILING"),
            Self::Floor => serializer.serialize_unit_variant("RoundingMode", 3u32, "FLOOR"),
            Self::HalfUp => serializer.serialize_unit_variant("RoundingMode", 4u32, "HALF_UP"),
            Self::HalfDown => serializer.serialize_unit_variant("RoundingMode", 5u32, "HALF_DOWN"),
            Self::HalfEven => serializer.serialize_unit_variant("RoundingMode", 6u32, "HALF_EVEN"),
            Self::Unnecessary => serializer.serialize_unit_variant("RoundingMode", 7u32, "UNNECESSARY"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The content of a search facet result item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchFacetItem {
    #[doc = "The count of the facet item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "The name of the facet item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facet: Option<String>,
    #[doc = "The sorting criteria"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<SearchFacetSort>,
}
impl SearchFacetItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The content of a search facet result item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchFacetItemValue {
    #[doc = "The count of the facet item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "The name of the facet item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl SearchFacetItemValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A facet list that consists of index fields assetType ,classification,\ncontactId, and label. When the facet is specified in the request, the value of\nthe facet is returned as an element of @search.facets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchFacetResultValue {
    #[doc = "Entity type"]
    #[serde(
        rename = "entityType",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub entity_type: Vec<SearchFacetItemValue>,
    #[doc = "Asset type"]
    #[serde(
        rename = "assetType",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub asset_type: Vec<SearchFacetItemValue>,
    #[doc = "Classification"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub classification: Vec<SearchFacetItemValue>,
    #[doc = "Term"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub term: Vec<SearchFacetItemValue>,
    #[doc = "Contact id"]
    #[serde(
        rename = "contactId",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub contact_id: Vec<SearchFacetItemValue>,
    #[doc = "Contact type"]
    #[serde(
        rename = "contactType",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub contact_type: Vec<SearchFacetItemValue>,
    #[doc = "Label"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub label: Vec<SearchFacetItemValue>,
    #[doc = "Glossary type"]
    #[serde(
        rename = "glossaryType",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub glossary_type: Vec<SearchFacetItemValue>,
    #[doc = "Term status"]
    #[serde(
        rename = "termStatus",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub term_status: Vec<SearchFacetItemValue>,
    #[doc = "Term template"]
    #[serde(
        rename = "termTemplate",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub term_template: Vec<SearchFacetItemValue>,
}
impl SearchFacetResultValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The sorting criteria"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchFacetSort {
    #[doc = "Search sort order"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<SearchSortOrder>,
    #[doc = "Search sort order"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<SearchSortOrder>,
}
impl SearchFacetSort {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A highlight list that consists of index fields id ,qualifiedName, name,\ndescription, entityType. When the keyword appears in those fields, the value of\nthe field, attached with emphasis mark, is returned as an element of\n@search.highlights."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchHighlights {
    #[doc = "Id"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub id: Vec<String>,
    #[doc = "Qualified name"]
    #[serde(
        rename = "qualifiedName",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub qualified_name: Vec<String>,
    #[doc = "Name"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub name: Vec<String>,
    #[doc = "Description"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub description: Vec<String>,
    #[doc = "Entity type"]
    #[serde(
        rename = "entityType",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub entity_type: Vec<String>,
}
impl SearchHighlights {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The value item of the search result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchResultValue {
    #[doc = "The search score calculated by the search engine. The results are ordered by\nsearch score by default."]
    #[serde(rename = "@search.score", default, skip_serializing_if = "Option::is_none")]
    pub search_score: Option<f32>,
    #[doc = "A highlight list that consists of index fields id ,qualifiedName, name,\ndescription, entityType. When the keyword appears in those fields, the value of\nthe field, attached with emphasis mark, is returned as an element of\n@search.highlights."]
    #[serde(rename = "@search.highlights", default, skip_serializing_if = "Option::is_none")]
    pub search_highlights: Option<SearchHighlights>,
    #[doc = "The object type of the record. Object type is the top-level property to\ndistinguish whether a record is an asset or a term."]
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[doc = "The create time of the record. The Unix epoch format."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[doc = "The last update time of the record. The Unix epoch format."]
    #[serde(rename = "updateTime", default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[doc = "The GUID of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The qualified name of the record."]
    #[serde(rename = "qualifiedName", default, skip_serializing_if = "Option::is_none")]
    pub qualified_name: Option<String>,
    #[doc = "The type name of the asset."]
    #[serde(rename = "entityType", default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[doc = "The description of the asset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The endorsement of the asset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endorsement: Option<String>,
    #[doc = "The owner of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[doc = "The classifications of the record."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub classification: Vec<String>,
    #[doc = "The labels of the asset."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub label: Vec<String>,
    #[doc = "The terms assigned to the asset."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub term: Vec<TermSearchResultValue>,
    #[doc = "The contacts of the asset."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub contact: Vec<ContactSearchResultValue>,
    #[doc = "The asset types of the asset."]
    #[serde(
        rename = "assetType",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub asset_type: Vec<String>,
    #[doc = "The type name of the term. Could be AtlasGlossary, AtlasGlossaryTerm or\nAtlasGlossaryCategory."]
    #[serde(rename = "glossaryType", default, skip_serializing_if = "Option::is_none")]
    pub glossary_type: Option<String>,
    #[doc = "The glossary name of the term."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glossary: Option<String>,
    #[doc = "The status of the term."]
    #[serde(rename = "termStatus", default, skip_serializing_if = "Option::is_none")]
    pub term_status: Option<String>,
    #[doc = "The term template names used by the term."]
    #[serde(
        rename = "termTemplate",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub term_template: Vec<String>,
    #[doc = "The definition of the term."]
    #[serde(rename = "longDescription", default, skip_serializing_if = "Option::is_none")]
    pub long_description: Option<String>,
}
impl SearchResultValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Search sort order"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SearchSortOrder")]
pub enum SearchSortOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SearchSortOrder {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SearchSortOrder {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SearchSortOrder {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Asc => serializer.serialize_unit_variant("SearchSortOrder", 0u32, "asc"),
            Self::Desc => serializer.serialize_unit_variant("SearchSortOrder", 1u32, "desc"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Taxonomy setting for search request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchTaxonomySetting {
    #[doc = "Asset types"]
    #[serde(
        rename = "assetTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub asset_types: Vec<String>,
    #[doc = "The content of a search facet result item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facet: Option<SearchFacetItem>,
}
impl SearchTaxonomySetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type for sorting"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SortType")]
pub enum SortType {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "ASC")]
    Asc,
    #[serde(rename = "DESC")]
    Desc,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SortType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SortType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SortType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("SortType", 0u32, "NONE"),
            Self::Asc => serializer.serialize_unit_variant("SortType", 1u32, "ASC"),
            Self::Desc => serializer.serialize_unit_variant("SortType", 2u32, "DESC"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Status for atlas relationship"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "StatusAtlasRelationship")]
pub enum StatusAtlasRelationship {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "DELETED")]
    Deleted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for StatusAtlasRelationship {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for StatusAtlasRelationship {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for StatusAtlasRelationship {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Active => serializer.serialize_unit_variant("StatusAtlasRelationship", 0u32, "ACTIVE"),
            Self::Deleted => serializer.serialize_unit_variant("StatusAtlasRelationship", 1u32, "DELETED"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The payload of suggest request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SuggestOptions {
    #[doc = "The keywords applied to all fields that support suggest operation. It must be\nat least 1 character, and no more than 100 characters. In the index schema we\ndefined a default suggester which lists all the supported fields and specifies\na search mode."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[doc = "The number of suggestions we hope to return. The default value is 5. The value\nmust be a number between 1 and 100."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[doc = "The filter for the search."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
}
impl SuggestOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result item of the search suggest."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SuggestResult {
    #[doc = "The result value"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SuggestResultValue>,
}
impl SuggestResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The value item of the search suggest."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SuggestResultValue {
    #[doc = "The search score calculated by the search engine. The results are ordered by\nsearch score by default."]
    #[serde(rename = "@search.score", default, skip_serializing_if = "Option::is_none")]
    pub search_score: Option<f32>,
    #[doc = "The target text that contains the keyword as prefix. The keyword is wrapped\nwith emphasis mark."]
    #[serde(rename = "@search.text", default, skip_serializing_if = "Option::is_none")]
    pub search_text: Option<String>,
    #[doc = "The object type of the record. Object type is the top-level property to\ndistinguish whether a record is an asset or a term."]
    #[serde(rename = "objectType", default, skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[doc = "The create time of the record. The Unix epoch format."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[doc = "The last update time of the record. The Unix epoch format."]
    #[serde(rename = "updateTime", default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[doc = "The GUID of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The qualified name of the record."]
    #[serde(rename = "qualifiedName", default, skip_serializing_if = "Option::is_none")]
    pub qualified_name: Option<String>,
    #[doc = "The type name of the asset."]
    #[serde(rename = "entityType", default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[doc = "The description of the asset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The endorsement of the asset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endorsement: Option<String>,
    #[doc = "The owner of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[doc = "The classifications of the record."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub classification: Vec<String>,
    #[doc = "The labels of the asset."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub label: Vec<String>,
    #[doc = "The terms assigned to the asset."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub term: Vec<TermSearchResultValue>,
    #[doc = "The contacts of the asset."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub contact: Vec<ContactSearchResultValue>,
    #[doc = "The asset types of the asset."]
    #[serde(
        rename = "assetType",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub asset_type: Vec<String>,
    #[doc = "The type name of the term. Could be AtlasGlossary, AtlasGlossaryTerm or\nAtlasGlossaryCategory."]
    #[serde(rename = "glossaryType", default, skip_serializing_if = "Option::is_none")]
    pub glossary_type: Option<String>,
    #[doc = "The glossary name of the term."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glossary: Option<String>,
    #[doc = "The status of the term."]
    #[serde(rename = "termStatus", default, skip_serializing_if = "Option::is_none")]
    pub term_status: Option<String>,
    #[doc = "The term template names used by the term."]
    #[serde(
        rename = "termTemplate",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub term_template: Vec<String>,
    #[doc = "The definition of the term."]
    #[serde(rename = "longDescription", default, skip_serializing_if = "Option::is_none")]
    pub long_description: Option<String>,
}
impl SuggestResultValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The context."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TermSearchResultValue {
    #[doc = "The name of the term."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The name of the glossary which contains the term."]
    #[serde(rename = "glossaryName", default, skip_serializing_if = "Option::is_none")]
    pub glossary_name: Option<String>,
    #[doc = "The GUID of the term."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
}
impl TermSearchResultValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status for term"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TermStatus")]
pub enum TermStatus {
    Draft,
    Approved,
    Alert,
    Expired,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TermStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TermStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TermStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Draft => serializer.serialize_unit_variant("TermStatus", 0u32, "Draft"),
            Self::Approved => serializer.serialize_unit_variant("TermStatus", 1u32, "Approved"),
            Self::Alert => serializer.serialize_unit_variant("TermStatus", 2u32, "Alert"),
            Self::Expired => serializer.serialize_unit_variant("TermStatus", 3u32, "Expired"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Term template definition for glossary term."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TermTemplateDef {
    #[doc = "Type Category"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<TypeCategory>,
    #[doc = "The created time of the record."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[doc = "The user who created the record."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The date format."]
    #[serde(rename = "dateFormatter", default, skip_serializing_if = "Option::is_none")]
    pub date_formatter: Option<DateFormat>,
    #[doc = "The description of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The GUID of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "The name of the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The options for the type definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[doc = "The service type."]
    #[serde(rename = "serviceType", default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    #[doc = "The version of the type."]
    #[serde(rename = "typeVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_version: Option<String>,
    #[doc = "The update time of the record."]
    #[serde(rename = "updateTime", default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[doc = "The user who updated the record."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "The version of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[doc = "ETag for concurrency control."]
    #[serde(rename = "lastModifiedTS", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_ts: Option<String>,
    #[doc = "An array of attribute definitions."]
    #[serde(
        rename = "attributeDefs",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub attribute_defs: Vec<AtlasAttributeDef>,
}
impl TermTemplateDef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Captures time-boundary details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeBoundary {
    #[doc = "The end of the time boundary."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The start of the time boundary."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The timezone of the time boundary."]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}
impl TimeBoundary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The timezone information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeZone {
    #[doc = "The value of the daylight saving time."]
    #[serde(rename = "dstSavings", default, skip_serializing_if = "Option::is_none")]
    pub dst_savings: Option<i32>,
    #[doc = "The ID of the timezone."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "An array of available IDs."]
    #[serde(
        rename = "availableIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub available_ids: Vec<String>,
    #[doc = "The timezone information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<Box<TimeZone>>,
    #[doc = "The display name of the timezone."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The raw offset of the timezone."]
    #[serde(rename = "rawOffset", default, skip_serializing_if = "Option::is_none")]
    pub raw_offset: Option<i32>,
}
impl TimeZone {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type Category"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TypeCategory")]
pub enum TypeCategory {
    #[serde(rename = "PRIMITIVE")]
    Primitive,
    #[serde(rename = "OBJECT_ID_TYPE")]
    ObjectIdType,
    #[serde(rename = "ENUM")]
    Enum,
    #[serde(rename = "STRUCT")]
    Struct,
    #[serde(rename = "CLASSIFICATION")]
    Classification,
    #[serde(rename = "ENTITY")]
    Entity,
    #[serde(rename = "ARRAY")]
    Array,
    #[serde(rename = "MAP")]
    Map,
    #[serde(rename = "RELATIONSHIP")]
    Relationship,
    #[serde(rename = "TERM_TEMPLATE")]
    TermTemplate,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TypeCategory {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TypeCategory {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TypeCategory {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Primitive => serializer.serialize_unit_variant("TypeCategory", 0u32, "PRIMITIVE"),
            Self::ObjectIdType => serializer.serialize_unit_variant("TypeCategory", 1u32, "OBJECT_ID_TYPE"),
            Self::Enum => serializer.serialize_unit_variant("TypeCategory", 2u32, "ENUM"),
            Self::Struct => serializer.serialize_unit_variant("TypeCategory", 3u32, "STRUCT"),
            Self::Classification => serializer.serialize_unit_variant("TypeCategory", 4u32, "CLASSIFICATION"),
            Self::Entity => serializer.serialize_unit_variant("TypeCategory", 5u32, "ENTITY"),
            Self::Array => serializer.serialize_unit_variant("TypeCategory", 6u32, "ARRAY"),
            Self::Map => serializer.serialize_unit_variant("TypeCategory", 7u32, "MAP"),
            Self::Relationship => serializer.serialize_unit_variant("TypeCategory", 8u32, "RELATIONSHIP"),
            Self::TermTemplate => serializer.serialize_unit_variant("TypeCategory", 9u32, "TERM_TEMPLATE"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
