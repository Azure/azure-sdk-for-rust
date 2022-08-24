#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Details of a commitment plan SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CatalogSku {
    #[doc = "Resource type name"]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "SKU name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "SKU tier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "Regions where the SKU is available."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "Describes scaling information of a SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<SkuCapacity>,
    #[doc = "The capability information for the specified SKU."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<SkuCapability>,
    #[doc = "The cost information for the specified SKU."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub costs: Vec<SkuCost>,
    #[doc = "Restrictions which would prevent a SKU from being used. This is empty if there are no restrictions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<SkuRestrictions>,
}
impl CatalogSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the association between a commitment plan and some other resource, such as a Machine Learning web service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitmentAssociation {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "An entity tag used to enforce optimistic concurrency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Properties of an Azure ML commitment association."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CommitmentAssociationProperties>,
}
impl CommitmentAssociation {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            etag: None,
            properties: None,
        }
    }
}
#[doc = "A page of commitment association resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommitmentAssociationListResult {
    #[doc = "A URI to retrieve the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The set of results for this page."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CommitmentAssociation>,
}
impl azure_core::Continuable for CommitmentAssociationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CommitmentAssociationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an Azure ML commitment association."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommitmentAssociationProperties {
    #[doc = "The ID of the resource this association points to, such as the ARM ID of an Azure ML web service."]
    #[serde(rename = "associatedResourceId", default, skip_serializing_if = "Option::is_none")]
    pub associated_resource_id: Option<String>,
    #[doc = "The ARM ID of the parent Azure ML commitment plan."]
    #[serde(rename = "commitmentPlanId", default, skip_serializing_if = "Option::is_none")]
    pub commitment_plan_id: Option<String>,
    #[doc = "The date at which this commitment association was created, in ISO 8601 format."]
    #[serde(rename = "creationDate", default, with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
}
impl CommitmentAssociationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure ML commitment plan resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitmentPlan {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "An entity tag used to enforce optimistic concurrency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Properties of an Azure ML commitment plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CommitmentPlanProperties>,
    #[doc = "The SKU of a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
}
impl CommitmentPlan {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            etag: None,
            properties: None,
            sku: None,
        }
    }
}
#[doc = "A page of commitment plan resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommitmentPlanListResult {
    #[doc = "A URI to retrieve the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The set of results for this page."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CommitmentPlan>,
}
impl azure_core::Continuable for CommitmentPlanListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CommitmentPlanListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a commitment plan which may be updated via PATCH."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommitmentPlanPatchPayload {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[doc = "The SKU of a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
}
impl CommitmentPlanPatchPayload {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an Azure ML commitment plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommitmentPlanProperties {
    #[doc = "Indicates whether usage beyond the commitment plan's included quantities will be charged."]
    #[serde(rename = "chargeForOverage", default, skip_serializing_if = "Option::is_none")]
    pub charge_for_overage: Option<bool>,
    #[doc = "Indicates whether the commitment plan will incur a charge."]
    #[serde(rename = "chargeForPlan", default, skip_serializing_if = "Option::is_none")]
    pub charge_for_plan: Option<bool>,
    #[doc = "The date at which this commitment plan was created, in ISO 8601 format."]
    #[serde(rename = "creationDate", default, with = "azure_core::date::rfc3339::option")]
    pub creation_date: Option<time::OffsetDateTime>,
    #[doc = "The included resource quantities this plan gives you."]
    #[serde(rename = "includedQuantities", default, skip_serializing_if = "Option::is_none")]
    pub included_quantities: Option<serde_json::Value>,
    #[doc = "The maximum number of commitment associations that can be children of this commitment plan."]
    #[serde(rename = "maxAssociationLimit", default, skip_serializing_if = "Option::is_none")]
    pub max_association_limit: Option<i32>,
    #[doc = "The maximum scale-out capacity for this commitment plan."]
    #[serde(rename = "maxCapacityLimit", default, skip_serializing_if = "Option::is_none")]
    pub max_capacity_limit: Option<i32>,
    #[doc = "The minimum scale-out capacity for this commitment plan."]
    #[serde(rename = "minCapacityLimit", default, skip_serializing_if = "Option::is_none")]
    pub min_capacity_limit: Option<i32>,
    #[doc = "The Azure meter which will be used to charge for this commitment plan."]
    #[serde(rename = "planMeter", default, skip_serializing_if = "Option::is_none")]
    pub plan_meter: Option<String>,
    #[doc = "The frequency at which this commitment plan's included quantities are refilled."]
    #[serde(rename = "refillFrequencyInDays", default, skip_serializing_if = "Option::is_none")]
    pub refill_frequency_in_days: Option<i32>,
    #[doc = "Indicates whether this commitment plan will be moved into a suspended state if usage goes beyond the commitment plan's included quantities."]
    #[serde(rename = "suspendPlanOnOverage", default, skip_serializing_if = "Option::is_none")]
    pub suspend_plan_on_overage: Option<bool>,
}
impl CommitmentPlanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the destination Azure ML commitment plan for a move operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MoveCommitmentAssociationRequest {
    #[doc = "The ARM ID of the commitment plan to re-parent the commitment association to."]
    #[serde(rename = "destinationPlanId", default, skip_serializing_if = "Option::is_none")]
    pub destination_plan_id: Option<String>,
}
impl MoveCommitmentAssociationRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The API operation info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplayInfo {
    #[doc = "The description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The action that users can perform, based on their permission level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The service provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The resource on which the operation is performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
impl OperationDisplayInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationEntity {
    #[doc = "Operation name: {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The API operation info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayInfo>,
}
impl OperationEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of REST API operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationEntityListResult {
    #[doc = "The list of operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationEntity>,
}
impl azure_core::Continuable for OperationEntityListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationEntityListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the quantity a commitment plan provides of a metered resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanQuantity {
    #[doc = "The quantity added to the commitment plan at an interval specified by its allowance frequency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowance: Option<f64>,
    #[doc = "The quantity available to the plan the last time usage was calculated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[doc = "The Azure meter for usage against included quantities."]
    #[serde(rename = "includedQuantityMeter", default, skip_serializing_if = "Option::is_none")]
    pub included_quantity_meter: Option<String>,
    #[doc = "The Azure meter for usage which exceeds included quantities."]
    #[serde(rename = "overageMeter", default, skip_serializing_if = "Option::is_none")]
    pub overage_meter: Option<String>,
}
impl PlanQuantity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents historical information about usage of the Azure resources associated with a commitment plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanUsageHistory {
    #[doc = "Overage incurred as a result of deleting a commitment plan."]
    #[serde(rename = "planDeletionOverage", default, skip_serializing_if = "Option::is_none")]
    pub plan_deletion_overage: Option<serde_json::Value>,
    #[doc = "Overage incurred as a result of migrating a commitment plan from one SKU to another."]
    #[serde(rename = "planMigrationOverage", default, skip_serializing_if = "Option::is_none")]
    pub plan_migration_overage: Option<serde_json::Value>,
    #[doc = "Included quantities remaining after usage against the commitment plan's associated resources was calculated."]
    #[serde(rename = "planQuantitiesAfterUsage", default, skip_serializing_if = "Option::is_none")]
    pub plan_quantities_after_usage: Option<serde_json::Value>,
    #[doc = "Included quantities remaining before usage against the commitment plan's associated resources was calculated."]
    #[serde(rename = "planQuantitiesBeforeUsage", default, skip_serializing_if = "Option::is_none")]
    pub plan_quantities_before_usage: Option<serde_json::Value>,
    #[doc = "Usage against the commitment plan's associated resources which was not covered by included quantities and is therefore overage."]
    #[serde(rename = "planUsageOverage", default, skip_serializing_if = "Option::is_none")]
    pub plan_usage_overage: Option<serde_json::Value>,
    #[doc = "Usage against the commitment plan's associated resources."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<serde_json::Value>,
    #[doc = "The date of usage, in ISO 8601 format."]
    #[serde(rename = "usageDate", default, with = "azure_core::date::rfc3339::option")]
    pub usage_date: Option<time::OffsetDateTime>,
}
impl PlanUsageHistory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A page of usage history."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanUsageHistoryListResult {
    #[doc = "A URI to retrieve the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The set of results for this page."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PlanUsageHistory>,
}
impl azure_core::Continuable for PlanUsageHistoryListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PlanUsageHistoryListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common properties of an ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource location."]
    pub location: String,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            location,
            type_: None,
            tags: None,
        }
    }
}
#[doc = "The SKU of a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSku {
    #[doc = "The scale-out capacity of the resource. 1 is 1x, 2 is 2x, etc. This impacts the quantities and cost of any commitment plan resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[doc = "The SKU name. Along with tier, uniquely identifies the SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The SKU tier. Along with name, uniquely identifies the SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
impl ResourceSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes The SKU capabilities object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuCapability {
    #[doc = "The capability name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The capability value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl SkuCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes scaling information of a SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuCapacity {
    #[doc = "The minimum capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    #[doc = "The maximum capacity that can be set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[doc = "The default capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<i64>,
    #[doc = "The scale type applicable to the sku."]
    #[serde(rename = "scaleType", default, skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<sku_capacity::ScaleType>,
}
impl SkuCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sku_capacity {
    use super::*;
    #[doc = "The scale type applicable to the sku."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScaleType")]
    pub enum ScaleType {
        Automatic,
        Manual,
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScaleType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScaleType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScaleType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Automatic => serializer.serialize_unit_variant("ScaleType", 0u32, "Automatic"),
                Self::Manual => serializer.serialize_unit_variant("ScaleType", 1u32, "Manual"),
                Self::None => serializer.serialize_unit_variant("ScaleType", 2u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Describes metadata for SKU cost info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuCost {
    #[doc = "The meter used for this part of a SKU's cost."]
    #[serde(rename = "meterID", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[doc = "The multiplier for the meter ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[doc = "The overall duration represented by the quantity."]
    #[serde(rename = "extendedUnit", default, skip_serializing_if = "Option::is_none")]
    pub extended_unit: Option<String>,
}
impl SkuCost {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of commitment plan SKUs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CatalogSku>,
}
impl azure_core::Continuable for SkuListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl SkuListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes restrictions which would prevent a SKU from being used."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuRestrictions {
    #[doc = "The type of restrictions."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<sku_restrictions::Type>,
    #[doc = "The value of restrictions. If the restriction type is set to location. This would be different locations where the SKU is restricted."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
    #[doc = "The reason for restriction."]
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<sku_restrictions::ReasonCode>,
}
impl SkuRestrictions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sku_restrictions {
    use super::*;
    #[doc = "The type of restrictions."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "location")]
        Location,
        #[serde(rename = "zone")]
        Zone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Location => serializer.serialize_unit_variant("Type", 0u32, "location"),
                Self::Zone => serializer.serialize_unit_variant("Type", 1u32, "zone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The reason for restriction."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReasonCode")]
    pub enum ReasonCode {
        QuotaId,
        NotAvailableForSubscription,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReasonCode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReasonCode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReasonCode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::QuotaId => serializer.serialize_unit_variant("ReasonCode", 0u32, "QuotaId"),
                Self::NotAvailableForSubscription => serializer.serialize_unit_variant("ReasonCode", 1u32, "NotAvailableForSubscription"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Resource tags"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tags {}
impl Tags {
    pub fn new() -> Self {
        Self::default()
    }
}
