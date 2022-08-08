#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "class that captures details of a struct-attribute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasAttributeDef {
    #[doc = "single-valued attribute or multi-valued attribute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cardinality: Option<Cardinality>,
    #[doc = "An array of constraints."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "The base model object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasBaseModelObject {
    #[doc = "The GUID of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
}
impl AtlasBaseModelObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class that captures common-attributes for all Atlas types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasBaseTypeDef {
    #[doc = "The enum of type category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<TypeCategory>,
    #[doc = "The created time of the record."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
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
    pub update_time: Option<f64>,
    #[doc = "The user who updated the record."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "The version of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<f64>,
    #[doc = "ETag for concurrency control."]
    #[serde(rename = "lastModifiedTS", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_ts: Option<LastModifiedTs>,
}
impl AtlasBaseTypeDef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "class that captures details of a struct-type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasBusinessMetadataDef {
    #[serde(flatten)]
    pub atlas_struct_def: AtlasStructDef,
}
impl AtlasBusinessMetadataDef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An instance of a classification; it doesn't have an identity, this object exists only when associated with an entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasClassification {
    #[serde(flatten)]
    pub atlas_struct: AtlasStruct,
    #[doc = "The GUID of the entity."]
    #[serde(rename = "entityGuid", default, skip_serializing_if = "Option::is_none")]
    pub entity_guid: Option<String>,
    #[doc = "Status of the entity - can be active or deleted. Deleted entities are not removed from Atlas store."]
    #[serde(rename = "entityStatus", default, skip_serializing_if = "Option::is_none")]
    pub entity_status: Option<Status>,
    #[doc = "Determines if propagations will be removed on entity deletion."]
    #[serde(rename = "removePropagationsOnEntityDelete", default, skip_serializing_if = "Option::is_none")]
    pub remove_propagations_on_entity_delete: Option<bool>,
    #[doc = "An array of time boundaries indicating validity periods."]
    #[serde(rename = "validityPeriods", default, skip_serializing_if = "Vec::is_empty")]
    pub validity_periods: Vec<TimeBoundary>,
    #[doc = "indicate the source who create the classification detail"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "more detail on source information"]
    #[serde(rename = "sourceDetails", default, skip_serializing_if = "Option::is_none")]
    pub source_details: Option<serde_json::Value>,
}
impl AtlasClassification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "class that captures details of a classification-type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasClassificationDef {
    #[serde(flatten)]
    pub atlas_struct_def: AtlasStructDef,
    #[doc = "Specifying a list of entityType names in the classificationDef, ensures that classifications can\nonly be applied to those entityTypes.\n<ul>\n<li>Any subtypes of the entity types inherit the restriction</li>\n<li>Any classificationDef subtypes inherit the parents entityTypes restrictions</li>\n<li>Any classificationDef subtypes can further restrict the parents entityTypes restrictions by specifying a subset of the entityTypes</li>\n<li>An empty entityTypes list when there are no parent restrictions means there are no restrictions</li>\n<li>An empty entityTypes list when there are parent restrictions means that the subtype picks up the parents restrictions</li>\n<li>If a list of entityTypes are supplied, where one inherits from another, this will be rejected. This should encourage cleaner classificationsDefs</li>\n</ul>"]
    #[serde(rename = "entityTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub entity_types: Vec<String>,
    #[doc = "An array of sub types."]
    #[serde(rename = "subTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub sub_types: Vec<String>,
    #[doc = "An array of super types."]
    #[serde(rename = "superTypes", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(flatten)]
    pub p_list: PList,
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
#[doc = "An instance of an entity along with extended info - like hive_table, hive_database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasEntitiesWithExtInfo {
    #[serde(flatten)]
    pub atlas_entity_ext_info: AtlasEntityExtInfo,
    #[doc = "An array of entities."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(flatten)]
    pub atlas_struct: AtlasStruct,
    #[doc = "Business Attributes"]
    #[serde(rename = "businessAttributes", default, skip_serializing_if = "Option::is_none")]
    pub business_attributes: Option<serde_json::Value>,
    #[doc = "An array of classifications."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifications: Vec<AtlasClassification>,
    #[doc = "The created time of the record."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
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
    #[doc = "Whether it is a shell entity"]
    #[serde(rename = "isIncomplete", default, skip_serializing_if = "Option::is_none")]
    pub is_incomplete: Option<bool>,
    #[doc = "labels"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    #[doc = "An array of term assignment headers indicating the meanings of the entity."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub meanings: Vec<AtlasTermAssignmentHeader>,
    #[doc = "Used to record the provenance of an instance of an entity or relationship."]
    #[serde(rename = "provenanceType", default, skip_serializing_if = "Option::is_none")]
    pub provenance_type: Option<f64>,
    #[doc = "Determines if there's a proxy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy: Option<bool>,
    #[doc = "The attributes of relationship."]
    #[serde(rename = "relationshipAttributes", default, skip_serializing_if = "Option::is_none")]
    pub relationship_attributes: Option<serde_json::Value>,
    #[doc = "Status of the entity - can be active or deleted. Deleted entities are not removed from Atlas store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[doc = "The update time of the record."]
    #[serde(rename = "updateTime", default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,
    #[doc = "The user who updated the record."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "The version of the entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<f64>,
    #[doc = "indicate the source who create the classification detail"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "more detail on source information"]
    #[serde(rename = "sourceDetails", default, skip_serializing_if = "Option::is_none")]
    pub source_details: Option<serde_json::Value>,
    #[doc = "The dictionary of contacts for terms. Key could be Expert or Owner."]
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
    #[serde(flatten)]
    pub atlas_struct_def: AtlasStructDef,
    #[doc = "An array of sub types."]
    #[serde(rename = "subTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub sub_types: Vec<String>,
    #[doc = "An array of super types."]
    #[serde(rename = "superTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub super_types: Vec<String>,
    #[doc = "An array of relationship attributes."]
    #[serde(rename = "relationshipAttributeDefs", default, skip_serializing_if = "Vec::is_empty")]
    pub relationship_attribute_defs: Vec<AtlasRelationshipAttributeDef>,
}
impl AtlasEntityDef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An instance of an entity along with extended info - like hive_table, hive_database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasEntityExtInfo {
    #[doc = "The referred entities."]
    #[serde(rename = "referredEntities", default, skip_serializing_if = "Option::is_none")]
    pub referred_entities: Option<serde_json::Value>,
}
impl AtlasEntityExtInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An instance of an entity - like hive_table, hive_database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasEntityHeader {
    #[serde(flatten)]
    pub atlas_struct: AtlasStruct,
    #[doc = "An array of classification names."]
    #[serde(rename = "classificationNames", default, skip_serializing_if = "Vec::is_empty")]
    pub classification_names: Vec<String>,
    #[doc = "An array of classifications."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    #[doc = "An array of meanings."]
    #[serde(rename = "meaningNames", default, skip_serializing_if = "Vec::is_empty")]
    pub meaning_names: Vec<String>,
    #[doc = "An array of term assignment headers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub meanings: Vec<AtlasTermAssignmentHeader>,
    #[doc = "Status of the entity - can be active or deleted. Deleted entities are not removed from Atlas store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
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
#[doc = "An instance of an entity along with extended info - like hive_table, hive_database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasEntityWithExtInfo {
    #[serde(flatten)]
    pub atlas_entity_ext_info: AtlasEntityExtInfo,
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
    #[serde(flatten)]
    pub atlas_base_type_def: AtlasBaseTypeDef,
    #[doc = "The default value."]
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[doc = "An array of enum element definitions."]
    #[serde(rename = "elementDefs", default, skip_serializing_if = "Vec::is_empty")]
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
    pub ordinal: Option<f64>,
    #[doc = "The value of the enum element definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl AtlasEnumElementDef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Extra properties for a type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasExtraTypeDef {
    #[doc = "Specifying a list of entityType names in the classificationDef, ensures that classifications can\nonly be applied to those entityTypes.\n<ul>\n<li>Any subtypes of the entity types inherit the restriction</li>\n<li>Any classificationDef subtypes inherit the parents entityTypes restrictions</li>\n<li>Any classificationDef subtypes can further restrict the parents entityTypes restrictions by specifying a subset of the entityTypes</li>\n<li>An empty entityTypes list when there are no parent restrictions means there are no restrictions</li>\n<li>An empty entityTypes list when there are parent restrictions means that the subtype picks up the parents restrictions</li>\n<li>If a list of entityTypes are supplied, where one inherits from another, this will be rejected. This should encourage cleaner classificationsDefs</li>\n</ul>"]
    #[serde(rename = "entityTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub entity_types: Vec<String>,
    #[doc = "An array of sub types."]
    #[serde(rename = "subTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub sub_types: Vec<String>,
    #[doc = "An array of super types."]
    #[serde(rename = "superTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub super_types: Vec<String>,
    #[doc = "An array of relationship attributes."]
    #[serde(rename = "relationshipAttributeDefs", default, skip_serializing_if = "Vec::is_empty")]
    pub relationship_attribute_defs: Vec<AtlasRelationshipAttributeDef>,
    #[doc = "The default value."]
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[doc = "An array of enum element definitions."]
    #[serde(rename = "elementDefs", default, skip_serializing_if = "Vec::is_empty")]
    pub element_defs: Vec<AtlasEnumElementDef>,
    #[doc = "The relationshipEndDef represents an end of the relationship. The end of the relationship is defined by a type, an\nattribute name, cardinality and whether it  is the container end of the relationship."]
    #[serde(rename = "endDef1", default, skip_serializing_if = "Option::is_none")]
    pub end_def1: Option<AtlasRelationshipEndDef>,
    #[doc = "The relationshipEndDef represents an end of the relationship. The end of the relationship is defined by a type, an\nattribute name, cardinality and whether it  is the container end of the relationship."]
    #[serde(rename = "endDef2", default, skip_serializing_if = "Option::is_none")]
    pub end_def2: Option<AtlasRelationshipEndDef>,
    #[doc = "The Relationship category determines the style of relationship around containment and lifecycle.\nUML terminology is used for the values.\n<p>\nASSOCIATION is a relationship with no containment. <br>\nCOMPOSITION and AGGREGATION are containment relationships.\n<p>\nThe difference being in the lifecycles of the container and its children. In the COMPOSITION case,\nthe children cannot exist without the container. For AGGREGATION, the life cycles\nof the container and children are totally independent."]
    #[serde(rename = "relationshipCategory", default, skip_serializing_if = "Option::is_none")]
    pub relationship_category: Option<RelationshipCategory>,
    #[doc = "The label of the relationship."]
    #[serde(rename = "relationshipLabel", default, skip_serializing_if = "Option::is_none")]
    pub relationship_label: Option<String>,
    #[doc = "An array of attribute definitions."]
    #[serde(rename = "attributeDefs", default, skip_serializing_if = "Vec::is_empty")]
    pub attribute_defs: Vec<AtlasAttributeDef>,
}
impl AtlasExtraTypeDef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The glossary object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasGlossary {
    #[serde(flatten)]
    pub atlas_glossary_base_object: AtlasGlossaryBaseObject,
    #[doc = "An array of categories."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<AtlasRelatedCategoryHeader>,
    #[doc = "The language of the glossary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[doc = "An array of related term headers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "The glossary base object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasGlossaryBaseObject {
    #[serde(flatten)]
    pub atlas_base_model_object: AtlasBaseModelObject,
    #[doc = "An array of classifications."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    pub last_modified_ts: Option<LastModifiedTs>,
}
impl AtlasGlossaryBaseObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The glossary category."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasGlossaryCategory {
    #[serde(flatten)]
    pub atlas_glossary_base_object: AtlasGlossaryBaseObject,
    #[doc = "The glossary header with basic information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<AtlasGlossaryHeader>,
    #[doc = "An array of children categories."]
    #[serde(rename = "childrenCategories", default, skip_serializing_if = "Vec::is_empty")]
    pub children_categories: Vec<AtlasRelatedCategoryHeader>,
    #[doc = "The header of the related category."]
    #[serde(rename = "parentCategory", default, skip_serializing_if = "Option::is_none")]
    pub parent_category: Option<AtlasRelatedCategoryHeader>,
    #[doc = "An array of related term headers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(flatten)]
    pub atlas_glossary: AtlasGlossary,
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
    #[serde(flatten)]
    pub atlas_glossary_base_object: AtlasGlossaryBaseObject,
    #[doc = "The abbreviation of the term."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abbreviation: Option<String>,
    #[serde(rename = "templateName", default, skip_serializing_if = "Vec::is_empty")]
    pub template_name: Vec<serde_json::Value>,
    #[doc = "The glossary header with basic information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<AtlasGlossaryHeader>,
    #[doc = "An array of related term headers as antonyms."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub antonyms: Vec<AtlasRelatedTermHeader>,
    #[doc = "The created time of the record."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[doc = "The user who created the record."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The update time of the record."]
    #[serde(rename = "updateTime", default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,
    #[doc = "The user who updated the record."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "Status of the AtlasGlossaryTerm"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<TermStatus>,
    #[doc = "An array of resource link for term"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<ResourceLink>,
    #[doc = "The dictionary of contacts for terms. Key could be Expert or Steward."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contacts: Option<serde_json::Value>,
    #[doc = "The custom attributes of the term, which is map<string,map<string,object>>.\nThe key of the first layer map is term template name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<TermCustomAttributes>,
    #[doc = "An array of related object IDs."]
    #[serde(rename = "assignedEntities", default, skip_serializing_if = "Vec::is_empty")]
    pub assigned_entities: Vec<AtlasRelatedObjectId>,
    #[doc = "An array of term categorization headers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<AtlasTermCategorizationHeader>,
    #[doc = "An array of related term headers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifies: Vec<AtlasRelatedTermHeader>,
    #[doc = "An array of examples."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<String>,
    #[doc = "An array of related term headers indicating the is-a relationship."]
    #[serde(rename = "isA", default, skip_serializing_if = "Vec::is_empty")]
    pub is_a: Vec<AtlasRelatedTermHeader>,
    #[doc = "An array of preferred related term headers."]
    #[serde(rename = "preferredTerms", default, skip_serializing_if = "Vec::is_empty")]
    pub preferred_terms: Vec<AtlasRelatedTermHeader>,
    #[doc = "An array of related term headers that are preferred to."]
    #[serde(rename = "preferredToTerms", default, skip_serializing_if = "Vec::is_empty")]
    pub preferred_to_terms: Vec<AtlasRelatedTermHeader>,
    #[doc = "An array of related term headers that are replaced by."]
    #[serde(rename = "replacedBy", default, skip_serializing_if = "Vec::is_empty")]
    pub replaced_by: Vec<AtlasRelatedTermHeader>,
    #[doc = "An array of related term headers for replacement."]
    #[serde(rename = "replacementTerms", default, skip_serializing_if = "Vec::is_empty")]
    pub replacement_terms: Vec<AtlasRelatedTermHeader>,
    #[doc = "An array of related term headers for see also."]
    #[serde(rename = "seeAlso", default, skip_serializing_if = "Vec::is_empty")]
    pub see_also: Vec<AtlasRelatedTermHeader>,
    #[doc = "An array of related term headers as synonyms."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub synonyms: Vec<AtlasRelatedTermHeader>,
    #[doc = "An array of translated related term headers."]
    #[serde(rename = "translatedTerms", default, skip_serializing_if = "Vec::is_empty")]
    pub translated_terms: Vec<AtlasRelatedTermHeader>,
    #[doc = "An array of related term headers for translation."]
    #[serde(rename = "translationTerms", default, skip_serializing_if = "Vec::is_empty")]
    pub translation_terms: Vec<AtlasRelatedTermHeader>,
    #[doc = "The usage of the term."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
    #[doc = "An array of related term headers as valid values."]
    #[serde(rename = "validValues", default, skip_serializing_if = "Vec::is_empty")]
    pub valid_values: Vec<AtlasRelatedTermHeader>,
    #[doc = "An array of related term headers as valid values for other records."]
    #[serde(rename = "validValuesFor", default, skip_serializing_if = "Vec::is_empty")]
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
    #[doc = "True to return the parent of the base entity."]
    #[serde(rename = "includeParent", default, skip_serializing_if = "Option::is_none")]
    pub include_parent: Option<bool>,
    #[doc = "The number of children node."]
    #[serde(rename = "childrenCount", default, skip_serializing_if = "Option::is_none")]
    pub children_count: Option<i32>,
    #[doc = "The enum of lineage direction."]
    #[serde(rename = "lineageDirection", default, skip_serializing_if = "Option::is_none")]
    pub lineage_direction: Option<LineageDirection>,
    #[doc = "An array of parentRelations relations."]
    #[serde(rename = "parentRelations", default, skip_serializing_if = "Vec::is_empty")]
    pub parent_relations: Vec<ParentRelation>,
    #[doc = "An array of lineage relations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relations: Vec<LineageRelation>,
}
impl AtlasLineageInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The lineage direction"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasLineageInfoExtraProperties {}
impl AtlasLineageInfoExtraProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to an object-instance of an Atlas type - like entity."]
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
#[doc = "Reference to an object-instance of AtlasEntity type used in relationship attribute values"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasRelatedObjectId {
    #[serde(flatten)]
    pub atlas_object_id: AtlasObjectId,
    #[doc = "The display text."]
    #[serde(rename = "displayText", default, skip_serializing_if = "Option::is_none")]
    pub display_text: Option<String>,
    #[doc = "Status of the entity - can be active or deleted. Deleted entities are not removed from Atlas store."]
    #[serde(rename = "entityStatus", default, skip_serializing_if = "Option::is_none")]
    pub entity_status: Option<Status>,
    #[serde(rename = "relationshipType", default, skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<String>,
    #[doc = "Captures details of struct contents. Not instantiated directly, used only via AtlasEntity, AtlasClassification."]
    #[serde(rename = "relationshipAttributes", default, skip_serializing_if = "Option::is_none")]
    pub relationship_attributes: Option<AtlasStruct>,
    #[doc = "The GUID of the relationship."]
    #[serde(rename = "relationshipGuid", default, skip_serializing_if = "Option::is_none")]
    pub relationship_guid: Option<String>,
    #[doc = "The enum of relationship status."]
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
    #[doc = "The source of the term."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "The status of term relationship."]
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
    #[serde(flatten)]
    pub atlas_struct: AtlasStruct,
    #[doc = "The created time of the record."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[doc = "The user who created the record."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "Reference to an object-instance of an Atlas type - like entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end1: Option<AtlasObjectId>,
    #[doc = "Reference to an object-instance of an Atlas type - like entity."]
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
    pub provenance_type: Option<f64>,
    #[doc = "The enum of relationship status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusAtlasRelationship>,
    #[doc = "The update time of the record."]
    #[serde(rename = "updateTime", default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,
    #[doc = "The user who updated the record."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "The version of the relationship."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<f64>,
}
impl AtlasRelationship {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The relationshipEndDef represents an end of the relationship. The end of the relationship is defined by a type, an\nattribute name, cardinality and whether it  is the container end of the relationship."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasRelationshipAttributeDef {
    #[serde(flatten)]
    pub atlas_attribute_def: AtlasAttributeDef,
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
#[doc = "AtlasRelationshipDef is a TypeDef that defines a relationship.\n<p>\nAs with other typeDefs the AtlasRelationshipDef has a name. Once created the RelationshipDef has a guid.\nThe name and the guid are the 2 ways that the RelationshipDef is identified.\n<p>\nRelationshipDefs have 2 ends, each of which specify cardinality, an EntityDef type name and name and optionally\nwhether the end is a container.\n<p>\nRelationshipDefs can have AttributeDefs - though only primitive types are allowed. <br>\nRelationshipDefs have a relationshipCategory specifying the UML type of relationship required <br>\nThe way EntityDefs and RelationshipDefs are intended to be used is that EntityDefs will define AttributeDefs these AttributeDefs\nwill not specify an EntityDef type name as their types.\n<p>\nRelationshipDefs introduce new attributes to the entity instances. For example\n<p>\nEntityDef A might have attributes attr1,attr2,attr3 <br>\nEntityDef B might have attributes attr4,attr5,attr6 <br>\nRelationshipDef AtoB might define 2 ends <br>\n\n<pre>\n   end1:  type A, name attr7\n   end2:  type B, name attr8  </pre>\n\n<p>\nWhen an instance of EntityDef A is created, it will have attributes attr1,attr2,attr3,attr7 <br>\nWhen an instance of EntityDef B is created, it will have attributes attr4,attr5,attr6,attr8\n<p>\nIn this way relationshipDefs can be authored separately from entityDefs and can inject relationship attributes into\nthe entity instances"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasRelationshipDef {
    #[serde(flatten)]
    pub atlas_struct_def: AtlasStructDef,
    #[doc = "The relationshipEndDef represents an end of the relationship. The end of the relationship is defined by a type, an\nattribute name, cardinality and whether it  is the container end of the relationship."]
    #[serde(rename = "endDef1", default, skip_serializing_if = "Option::is_none")]
    pub end_def1: Option<AtlasRelationshipEndDef>,
    #[doc = "The relationshipEndDef represents an end of the relationship. The end of the relationship is defined by a type, an\nattribute name, cardinality and whether it  is the container end of the relationship."]
    #[serde(rename = "endDef2", default, skip_serializing_if = "Option::is_none")]
    pub end_def2: Option<AtlasRelationshipEndDef>,
    #[doc = "The Relationship category determines the style of relationship around containment and lifecycle.\nUML terminology is used for the values.\n<p>\nASSOCIATION is a relationship with no containment. <br>\nCOMPOSITION and AGGREGATION are containment relationships.\n<p>\nThe difference being in the lifecycles of the container and its children. In the COMPOSITION case,\nthe children cannot exist without the container. For AGGREGATION, the life cycles\nof the container and children are totally independent."]
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
#[doc = "The relationshipEndDef represents an end of the relationship. The end of the relationship is defined by a type, an\nattribute name, cardinality and whether it  is the container end of the relationship."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasRelationshipEndDef {
    #[doc = "single-valued attribute or multi-valued attribute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cardinality: Option<Cardinality>,
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
#[doc = "Captures details of struct contents. Not instantiated directly, used only via AtlasEntity, AtlasClassification."]
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
    pub last_modified_ts: Option<LastModifiedTs>,
}
impl AtlasStruct {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "class that captures details of a struct-type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasStructDef {
    #[serde(flatten)]
    pub atlas_base_type_def: AtlasBaseTypeDef,
    #[doc = "An array of attribute definitions."]
    #[serde(rename = "attributeDefs", default, skip_serializing_if = "Vec::is_empty")]
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
    #[doc = "The GUID of the relationship."]
    #[serde(rename = "relationGuid", default, skip_serializing_if = "Option::is_none")]
    pub relation_guid: Option<String>,
    #[doc = "The source of the term."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "The status of terms assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AtlasTermAssignmentStatus>,
    #[doc = "The steward of the term."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steward: Option<String>,
    #[doc = "The GUID of the term."]
    #[serde(rename = "termGuid", default, skip_serializing_if = "Option::is_none")]
    pub term_guid: Option<String>,
}
impl AtlasTermAssignmentHeader {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of terms assignment."]
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
    #[doc = "The GUID of the category."]
    #[serde(rename = "categoryGuid", default, skip_serializing_if = "Option::is_none")]
    pub category_guid: Option<String>,
    #[doc = "The description of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The display text."]
    #[serde(rename = "displayText", default, skip_serializing_if = "Option::is_none")]
    pub display_text: Option<String>,
    #[doc = "The GUID of the relationship."]
    #[serde(rename = "relationGuid", default, skip_serializing_if = "Option::is_none")]
    pub relation_guid: Option<String>,
    #[doc = "The status of term relationship."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AtlasTermRelationshipStatus>,
}
impl AtlasTermCategorizationHeader {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of term relationship."]
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
    #[serde(flatten)]
    pub atlas_base_type_def: AtlasBaseTypeDef,
    #[serde(flatten)]
    pub atlas_extra_type_def: AtlasExtraTypeDef,
}
impl AtlasTypeDef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The basic information of the type definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AtlasTypeDefHeader {
    #[doc = "The enum of type category."]
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
    #[serde(rename = "businessMetadataDefs", default, skip_serializing_if = "Vec::is_empty")]
    pub business_metadata_defs: Vec<AtlasBusinessMetadataDef>,
    #[doc = "An array of classification definitions."]
    #[serde(rename = "classificationDefs", default, skip_serializing_if = "Vec::is_empty")]
    pub classification_defs: Vec<AtlasClassificationDef>,
    #[doc = "An array of entity definitions."]
    #[serde(rename = "entityDefs", default, skip_serializing_if = "Vec::is_empty")]
    pub entity_defs: Vec<AtlasEntityDef>,
    #[doc = "An array of enum definitions."]
    #[serde(rename = "enumDefs", default, skip_serializing_if = "Vec::is_empty")]
    pub enum_defs: Vec<AtlasEnumDef>,
    #[doc = "An array of relationship definitions."]
    #[serde(rename = "relationshipDefs", default, skip_serializing_if = "Vec::is_empty")]
    pub relationship_defs: Vec<AtlasRelationshipDef>,
    #[doc = "An array of struct definitions."]
    #[serde(rename = "structDefs", default, skip_serializing_if = "Vec::is_empty")]
    pub struct_defs: Vec<AtlasStructDef>,
    #[doc = "An array of term template definitions."]
    #[serde(rename = "termTemplateDefs", default, skip_serializing_if = "Vec::is_empty")]
    pub term_template_defs: Vec<TermTemplateDef>,
}
impl AtlasTypesDef {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The query of autocomplete request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoCompleteRequest {
    #[doc = "The keywords applied to all fields that support autocomplete operation. It must be at least 1 character, and no more than 100 characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[doc = "The number of autocomplete results we hope to return. The default value is 50. The value must be a number between 1 and 100."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[doc = "The filter for the autocomplete request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
}
impl AutoCompleteRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of the autocomplete request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoCompleteResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "The criteria of browse request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BrowseRequest {
    #[doc = "The entity type to browse as the root level entry point."]
    #[serde(rename = "entityType", default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[doc = "The path to browse the next level child entities."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The number of browse items we hope to return. The maximum value is 10000."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[doc = "The offset. The default value is 0. The maximum value is 100000."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}
impl BrowseRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result item of the browse request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BrowseResult {
    #[doc = "The total number of browse results."]
    #[serde(rename = "@search.count", default, skip_serializing_if = "Option::is_none")]
    pub search_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BrowseResultValue>,
}
impl BrowseResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The value item of the browse owner."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BrowseResultOwner {
    #[doc = "The GUID of the owner."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The display name of the owner."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The mail of the owner."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
    #[doc = "The contact type of the owner. The value will be Owner."]
    #[serde(rename = "contactType", default, skip_serializing_if = "Option::is_none")]
    pub contact_type: Option<String>,
}
impl BrowseResultOwner {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The value item of the browse result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BrowseResultValue {
    #[doc = "The type name of the record."]
    #[serde(rename = "entityType", default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[doc = "The GUID of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "If the record is a leaf entity."]
    #[serde(rename = "isLeaf", default, skip_serializing_if = "Option::is_none")]
    pub is_leaf: Option<bool>,
    #[doc = "The name of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The owners of the record."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub owner: Vec<BrowseResultOwner>,
    #[doc = "The path of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The qualified name of the record."]
    #[serde(rename = "qualifiedName", default, skip_serializing_if = "Option::is_none")]
    pub qualified_name: Option<String>,
}
impl BrowseResultValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "example"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BulkImportResponse {
    #[doc = "failed importInfoList"]
    #[serde(rename = "failedImportInfoList", default, skip_serializing_if = "Vec::is_empty")]
    pub failed_import_info_list: Vec<ImportInfo>,
    #[doc = "successful importInfoList"]
    #[serde(rename = "successImportInfoList", default, skip_serializing_if = "Vec::is_empty")]
    pub success_import_info_list: Vec<ImportInfo>,
}
impl BulkImportResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "single-valued attribute or multi-valued attribute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Cardinality")]
pub enum Cardinality {
    #[serde(rename = "SINGLE")]
    Single,
    #[serde(rename = "LIST")]
    List,
    #[serde(rename = "SET")]
    Set,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Cardinality {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Cardinality {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Cardinality {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Single => serializer.serialize_unit_variant("Cardinality", 0u32, "SINGLE"),
            Self::List => serializer.serialize_unit_variant("Cardinality", 1u32, "LIST"),
            Self::Set => serializer.serialize_unit_variant("Cardinality", 2u32, "SET"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The request for classification association."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClassificationAssociateRequest {
    #[doc = "An instance of a classification; it doesn't have an identity, this object exists only when associated with an entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<AtlasClassification>,
    #[doc = "The GUID of the entity."]
    #[serde(rename = "entityGuids", default, skip_serializing_if = "Vec::is_empty")]
    pub entity_guids: Vec<String>,
}
impl ClassificationAssociateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContactBasic {
    #[doc = "Azure Active Directory object Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "additional information to describe this contact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
}
impl ContactBasic {
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
    #[doc = "The type of the contact. It can be Expert or Owner for an entity. It can be Expert or Steward for a glossary term."]
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
    #[serde(rename = "availableLocales", default, skip_serializing_if = "Vec::is_empty")]
    pub available_locales: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar: Option<f64>,
    #[doc = "The date format."]
    #[serde(rename = "dateInstance", default, skip_serializing_if = "Option::is_none")]
    pub date_instance: Box<Option<DateFormat>>,
    #[doc = "The date format."]
    #[serde(rename = "dateTimeInstance", default, skip_serializing_if = "Option::is_none")]
    pub date_time_instance: Box<Option<DateFormat>>,
    #[doc = "The date format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Box<Option<DateFormat>>,
    #[doc = "Determines the leniency of the date format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lenient: Option<bool>,
    #[doc = "The number format."]
    #[serde(rename = "numberFormat", default, skip_serializing_if = "Option::is_none")]
    pub number_format: Option<NumberFormat>,
    #[doc = "The date format."]
    #[serde(rename = "timeInstance", default, skip_serializing_if = "Option::is_none")]
    pub time_instance: Box<Option<DateFormat>>,
    #[doc = "The timezone information."]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<TimeZone>,
}
impl DateFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The mutation response of entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityMutationResponse {
    #[doc = "A map of GUID assignments with entities."]
    #[serde(rename = "guidAssignments", default, skip_serializing_if = "Option::is_none")]
    pub guid_assignments: Option<serde_json::Value>,
    #[doc = "The entity headers of mutated entities."]
    #[serde(rename = "mutatedEntities", default, skip_serializing_if = "Option::is_none")]
    pub mutated_entities: Option<serde_json::Value>,
    #[doc = "An array of entity headers that partially updated."]
    #[serde(rename = "partialUpdatedEntities", default, skip_serializing_if = "Vec::is_empty")]
    pub partial_updated_entities: Vec<AtlasEntityHeader>,
}
impl EntityMutationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The request ID."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "The error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "The error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of import csv operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportCsvOperation {
    #[doc = "guid string"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Enum of the status of import csv operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ImportCsvOperationStatus>,
    #[doc = "The created time of the record."]
    #[serde(rename = "createTime", default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[doc = "The last updated time of the record."]
    #[serde(rename = "lastUpdateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImportCsvOperationProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ImportCsvOperationError>,
}
impl ImportCsvOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportCsvOperationError {
    #[doc = "Error code from async import job if fail"]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i64>,
    #[doc = "Error message from async import job if fail"]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl ImportCsvOperationError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportCsvOperationProperties {
    #[doc = "Term numbers that already imported successfully"]
    #[serde(rename = "importedTerms", default, skip_serializing_if = "Option::is_none")]
    pub imported_terms: Option<String>,
    #[doc = "Total term numbers that detected in csv"]
    #[serde(rename = "totalTermsDetected", default, skip_serializing_if = "Option::is_none")]
    pub total_terms_detected: Option<String>,
}
impl ImportCsvOperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enum of the status of import csv operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ImportCsvOperationStatus")]
pub enum ImportCsvOperationStatus {
    NotStarted,
    Succeeded,
    Failed,
    Running,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ImportCsvOperationStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ImportCsvOperationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ImportCsvOperationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotStarted => serializer.serialize_unit_variant("ImportCsvOperationStatus", 0u32, "NotStarted"),
            Self::Succeeded => serializer.serialize_unit_variant("ImportCsvOperationStatus", 1u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ImportCsvOperationStatus", 2u32, "Failed"),
            Self::Running => serializer.serialize_unit_variant("ImportCsvOperationStatus", 3u32, "Running"),
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
    #[doc = "ImportStatus"]
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
#[doc = "ImportStatus"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ImportStatus {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILED")]
    Failed,
}
pub type LastModifiedTs = String;
#[doc = "The enum of lineage direction."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveEntitiesRequest {
    #[doc = "An array of entity guids to be moved to target collection."]
    #[serde(rename = "entityGuids", default, skip_serializing_if = "Vec::is_empty")]
    pub entity_guids: Vec<String>,
}
impl MoveEntitiesRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The number format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NumberFormat {
    #[doc = "The number format."]
    #[serde(rename = "availableLocales", default, skip_serializing_if = "Vec::is_empty")]
    pub available_locales: Vec<String>,
    #[doc = "The currency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The number format."]
    #[serde(rename = "currencyInstance", default, skip_serializing_if = "Option::is_none")]
    pub currency_instance: Box<Option<NumberFormat>>,
    #[doc = "Determines if grouping is used."]
    #[serde(rename = "groupingUsed", default, skip_serializing_if = "Option::is_none")]
    pub grouping_used: Option<bool>,
    #[doc = "The number format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Box<Option<NumberFormat>>,
    #[doc = "The number format."]
    #[serde(rename = "integerInstance", default, skip_serializing_if = "Option::is_none")]
    pub integer_instance: Box<Option<NumberFormat>>,
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
    pub number_instance: Box<Option<NumberFormat>>,
    #[doc = "Determines if only integer is parsed."]
    #[serde(rename = "parseIntegerOnly", default, skip_serializing_if = "Option::is_none")]
    pub parse_integer_only: Option<bool>,
    #[doc = "The number format."]
    #[serde(rename = "percentInstance", default, skip_serializing_if = "Option::is_none")]
    pub percent_instance: Box<Option<NumberFormat>>,
    #[doc = "The enum of rounding mode."]
    #[serde(rename = "roundingMode", default, skip_serializing_if = "Option::is_none")]
    pub rounding_mode: Option<RoundingMode>,
}
impl NumberFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paginated-list, for returning search results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PList {
    #[doc = "An array of objects."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub list: Vec<serde_json::Value>,
    #[doc = "The size of the page."]
    #[serde(rename = "pageSize", default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[doc = "The sorted by field."]
    #[serde(rename = "sortBy", default, skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[doc = "to specify whether the result should be sorted? If yes, whether asc or desc."]
    #[serde(rename = "sortType", default, skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<SortType>,
    #[doc = "The start index of the page."]
    #[serde(rename = "startIndex", default, skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
    #[doc = "The total count of items."]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}
impl PList {
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
#[doc = "The Relationship category determines the style of relationship around containment and lifecycle.\nUML terminology is used for the values.\n<p>\nASSOCIATION is a relationship with no containment. <br>\nCOMPOSITION and AGGREGATION are containment relationships.\n<p>\nThe difference being in the lifecycles of the container and its children. In the COMPOSITION case,\nthe children cannot exist without the container. For AGGREGATION, the life cycles\nof the container and children are totally independent."]
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
#[doc = "The enum of rounding mode."]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<serde_json::Value>,
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
#[doc = "A facet list that consists of index fields assetType ,classification, contactId, and label. When the facet is specified in the request, the value of the facet is returned as an element of @search.facets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchFacetResultValue {
    #[serde(rename = "assetType", default, skip_serializing_if = "Vec::is_empty")]
    pub asset_type: Vec<SearchFacetItemValue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classification: Vec<SearchFacetItemValue>,
    #[serde(rename = "classificationCategory", default, skip_serializing_if = "Vec::is_empty")]
    pub classification_category: Vec<SearchFacetItemValue>,
    #[serde(rename = "contactId", default, skip_serializing_if = "Vec::is_empty")]
    pub contact_id: Vec<SearchFacetItemValue>,
    #[serde(rename = "fileExtension", default, skip_serializing_if = "Vec::is_empty")]
    pub file_extension: Vec<SearchFacetItemValue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub label: Vec<SearchFacetItemValue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub term: Vec<SearchFacetItemValue>,
}
impl SearchFacetResultValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A highlight list that consists of index fields id ,qualifiedName, name, description, entityType. When the keyword appears in those fields, the value of the field, attached with emphasis mark, is returned as an element of @search.highlights."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchHighlights {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub id: Vec<String>,
    #[serde(rename = "qualifiedName", default, skip_serializing_if = "Vec::is_empty")]
    pub qualified_name: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub description: Vec<String>,
    #[serde(rename = "entityType", default, skip_serializing_if = "Vec::is_empty")]
    pub entity_type: Vec<String>,
}
impl SearchHighlights {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The search query of advanced search request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchRequest {
    #[doc = "The keywords applied to all searchable fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[doc = "The offset. The default value is 0. The maximum value is 100000."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[doc = "The limit of the number of the search result. default value is 50; maximum value is 1000."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[doc = "The filter for the search. See examples for the usage of supported filters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub facets: Vec<SearchFacetItem>,
    #[serde(rename = "taxonomySetting", default, skip_serializing_if = "Option::is_none")]
    pub taxonomy_setting: Option<search_request::TaxonomySetting>,
}
impl SearchRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod search_request {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct TaxonomySetting {
        #[serde(rename = "assetTypes", default, skip_serializing_if = "Vec::is_empty")]
        pub asset_types: Vec<String>,
        #[doc = "The content of a search facet result item."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub facet: Option<SearchFacetItem>,
    }
    impl TaxonomySetting {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The result of the search result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchResult {
    #[doc = "The total number of search results (not the number of documents in a single page)."]
    #[serde(rename = "@search.count", default, skip_serializing_if = "Option::is_none")]
    pub search_count: Option<i32>,
    #[doc = "A facet list that consists of index fields assetType ,classification, contactId, and label. When the facet is specified in the request, the value of the facet is returned as an element of @search.facets."]
    #[serde(rename = "@search.facets", default, skip_serializing_if = "Option::is_none")]
    pub search_facets: Option<SearchFacetResultValue>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SearchResultValue>,
}
impl SearchResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The value item of the search result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchResultValue {
    #[doc = "The search score calculated by the search engine. The results are ordered by search score by default."]
    #[serde(rename = "@search.score", default, skip_serializing_if = "Option::is_none")]
    pub search_score: Option<f32>,
    #[doc = "A highlight list that consists of index fields id ,qualifiedName, name, description, entityType. When the keyword appears in those fields, the value of the field, attached with emphasis mark, is returned as an element of @search.highlights."]
    #[serde(rename = "@search.highlights", default, skip_serializing_if = "Option::is_none")]
    pub search_highlights: Option<SearchHighlights>,
    #[doc = "The target text that contains the keyword as prefix. The keyword is wrapped with emphasis mark."]
    #[serde(rename = "@search.text", default, skip_serializing_if = "Option::is_none")]
    pub search_text: Option<String>,
    #[doc = "The description of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The GUID of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The owner of the record. This is an Atlas native attribute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[doc = "The qualified name of the record."]
    #[serde(rename = "qualifiedName", default, skip_serializing_if = "Option::is_none")]
    pub qualified_name: Option<String>,
    #[doc = "The type name of the record."]
    #[serde(rename = "entityType", default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[doc = "The classifications of the record."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classification: Vec<String>,
    #[doc = "The labels of the record."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub label: Vec<String>,
    #[doc = "The terms assigned to the record."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub term: Vec<TermSearchResultValue>,
    #[doc = "The contacts of the record."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<ContactSearchResultValue>,
    #[doc = "The asset types of the record."]
    #[serde(rename = "assetType", default, skip_serializing_if = "Vec::is_empty")]
    pub asset_type: Vec<String>,
}
impl SearchResultValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "to specify whether the result should be sorted? If yes, whether asc or desc."]
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
#[doc = "Status of the entity - can be active or deleted. Deleted entities are not removed from Atlas store."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Status")]
pub enum Status {
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "DELETED")]
    Deleted,
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
            Self::Active => serializer.serialize_unit_variant("Status", 0u32, "ACTIVE"),
            Self::Deleted => serializer.serialize_unit_variant("Status", 1u32, "DELETED"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The enum of relationship status."]
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
#[doc = "The query of suggest request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SuggestRequest {
    #[doc = "The keywords applied to all fields that support suggest operation. It must be at least 1 character, and no more than 100 characters. In the index schema we defined a default suggester which lists all the supported fields and specifies a search mode."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[doc = "The number of suggestions we hope to return. The default value is 5. The value must be a number between 1 and 100."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[doc = "The filter for the search."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
}
impl SuggestRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result item of the search suggest."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SuggestResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[doc = "The search score calculated by the search engine. The results are ordered by search score by default."]
    #[serde(rename = "@search.score", default, skip_serializing_if = "Option::is_none")]
    pub search_score: Option<f32>,
    #[doc = "The target text that contains the keyword as prefix. The keyword is wrapped with emphasis mark."]
    #[serde(rename = "@search.text", default, skip_serializing_if = "Option::is_none")]
    pub search_text: Option<String>,
    #[doc = "The description of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The GUID of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The owner of the record. This is an Atlas native attribute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[doc = "The qualified name of the record."]
    #[serde(rename = "qualifiedName", default, skip_serializing_if = "Option::is_none")]
    pub qualified_name: Option<String>,
    #[doc = "The type name of the record."]
    #[serde(rename = "entityType", default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[doc = "The classifications of the record."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classification: Vec<String>,
    #[doc = "The labels of the record."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub label: Vec<String>,
    #[doc = "The terms assigned to the record."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub term: Vec<TermSearchResultValue>,
    #[doc = "The contacts of the record."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact: Vec<ContactSearchResultValue>,
    #[doc = "The asset types of the record."]
    #[serde(rename = "assetType", default, skip_serializing_if = "Vec::is_empty")]
    pub asset_type: Vec<String>,
}
impl SuggestResultValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The custom attributes of the term, which is map<string,map<string,object>>.\nThe key of the first layer map is term template name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TermCustomAttributes {}
impl TermCustomAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The term attribute name and attribute value, which is map<string,object>"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TermCustomAttributesExtraProperties {}
impl TermCustomAttributesExtraProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type TermGuid = String;
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
#[doc = "Status of the AtlasGlossaryTerm"]
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
#[doc = "term template definition for glossary term."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TermTemplateDef {
    #[serde(flatten)]
    pub atlas_struct_def: AtlasStructDef,
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
    #[serde(rename = "availableIds", default, skip_serializing_if = "Vec::is_empty")]
    pub available_ids: Vec<String>,
    #[doc = "The timezone information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Box<Option<TimeZone>>,
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
#[doc = "The enum of type category."]
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
