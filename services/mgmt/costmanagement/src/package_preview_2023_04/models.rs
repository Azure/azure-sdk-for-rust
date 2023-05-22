#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An individual alert."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Alert {
    #[serde(flatten)]
    pub cost_management_proxy_resource: CostManagementProxyResource,
    #[doc = "Alert properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertProperties>,
}
impl Alert {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Alert properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertProperties {
    #[doc = "defines the type of alert"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<alert_properties::Definition>,
    #[doc = "Alert description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Source of alert"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<alert_properties::Source>,
    #[doc = "Alert details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<alert_properties::Details>,
    #[doc = "related budget"]
    #[serde(rename = "costEntityId", default, skip_serializing_if = "Option::is_none")]
    pub cost_entity_id: Option<String>,
    #[doc = "alert status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<alert_properties::Status>,
    #[doc = "dateTime in which alert was created"]
    #[serde(rename = "creationTime", default, skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[doc = "dateTime in which alert was closed"]
    #[serde(rename = "closeTime", default, skip_serializing_if = "Option::is_none")]
    pub close_time: Option<String>,
    #[doc = "dateTime in which alert was last modified"]
    #[serde(rename = "modificationTime", default, skip_serializing_if = "Option::is_none")]
    pub modification_time: Option<String>,
    #[doc = "User who last modified the alert"]
    #[serde(rename = "statusModificationUserName", default, skip_serializing_if = "Option::is_none")]
    pub status_modification_user_name: Option<String>,
    #[doc = "dateTime in which the alert status was last modified"]
    #[serde(rename = "statusModificationTime", default, skip_serializing_if = "Option::is_none")]
    pub status_modification_time: Option<String>,
}
impl AlertProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod alert_properties {
    use super::*;
    #[doc = "defines the type of alert"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Definition {
        #[doc = "type of alert"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<definition::Type>,
        #[doc = "Alert category"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub category: Option<definition::Category>,
        #[doc = "Criteria that triggered alert"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub criteria: Option<definition::Criteria>,
    }
    impl Definition {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod definition {
        use super::*;
        #[doc = "type of alert"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Type")]
        pub enum Type {
            Budget,
            Invoice,
            Credit,
            Quota,
            General,
            #[serde(rename = "xCloud")]
            XCloud,
            BudgetForecast,
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
                    Self::Budget => serializer.serialize_unit_variant("Type", 0u32, "Budget"),
                    Self::Invoice => serializer.serialize_unit_variant("Type", 1u32, "Invoice"),
                    Self::Credit => serializer.serialize_unit_variant("Type", 2u32, "Credit"),
                    Self::Quota => serializer.serialize_unit_variant("Type", 3u32, "Quota"),
                    Self::General => serializer.serialize_unit_variant("Type", 4u32, "General"),
                    Self::XCloud => serializer.serialize_unit_variant("Type", 5u32, "xCloud"),
                    Self::BudgetForecast => serializer.serialize_unit_variant("Type", 6u32, "BudgetForecast"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = "Alert category"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Category")]
        pub enum Category {
            Cost,
            Usage,
            Billing,
            System,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for Category {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for Category {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for Category {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Cost => serializer.serialize_unit_variant("Category", 0u32, "Cost"),
                    Self::Usage => serializer.serialize_unit_variant("Category", 1u32, "Usage"),
                    Self::Billing => serializer.serialize_unit_variant("Category", 2u32, "Billing"),
                    Self::System => serializer.serialize_unit_variant("Category", 3u32, "System"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = "Criteria that triggered alert"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Criteria")]
        pub enum Criteria {
            CostThresholdExceeded,
            UsageThresholdExceeded,
            CreditThresholdApproaching,
            CreditThresholdReached,
            QuotaThresholdApproaching,
            QuotaThresholdReached,
            MultiCurrency,
            ForecastCostThresholdExceeded,
            ForecastUsageThresholdExceeded,
            InvoiceDueDateApproaching,
            InvoiceDueDateReached,
            CrossCloudNewDataAvailable,
            CrossCloudCollectionError,
            GeneralThresholdError,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for Criteria {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for Criteria {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for Criteria {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::CostThresholdExceeded => serializer.serialize_unit_variant("Criteria", 0u32, "CostThresholdExceeded"),
                    Self::UsageThresholdExceeded => serializer.serialize_unit_variant("Criteria", 1u32, "UsageThresholdExceeded"),
                    Self::CreditThresholdApproaching => serializer.serialize_unit_variant("Criteria", 2u32, "CreditThresholdApproaching"),
                    Self::CreditThresholdReached => serializer.serialize_unit_variant("Criteria", 3u32, "CreditThresholdReached"),
                    Self::QuotaThresholdApproaching => serializer.serialize_unit_variant("Criteria", 4u32, "QuotaThresholdApproaching"),
                    Self::QuotaThresholdReached => serializer.serialize_unit_variant("Criteria", 5u32, "QuotaThresholdReached"),
                    Self::MultiCurrency => serializer.serialize_unit_variant("Criteria", 6u32, "MultiCurrency"),
                    Self::ForecastCostThresholdExceeded => {
                        serializer.serialize_unit_variant("Criteria", 7u32, "ForecastCostThresholdExceeded")
                    }
                    Self::ForecastUsageThresholdExceeded => {
                        serializer.serialize_unit_variant("Criteria", 8u32, "ForecastUsageThresholdExceeded")
                    }
                    Self::InvoiceDueDateApproaching => serializer.serialize_unit_variant("Criteria", 9u32, "InvoiceDueDateApproaching"),
                    Self::InvoiceDueDateReached => serializer.serialize_unit_variant("Criteria", 10u32, "InvoiceDueDateReached"),
                    Self::CrossCloudNewDataAvailable => serializer.serialize_unit_variant("Criteria", 11u32, "CrossCloudNewDataAvailable"),
                    Self::CrossCloudCollectionError => serializer.serialize_unit_variant("Criteria", 12u32, "CrossCloudCollectionError"),
                    Self::GeneralThresholdError => serializer.serialize_unit_variant("Criteria", 13u32, "GeneralThresholdError"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
    #[doc = "Source of alert"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Source")]
    pub enum Source {
        Preset,
        User,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Source {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Source {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Source {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Preset => serializer.serialize_unit_variant("Source", 0u32, "Preset"),
                Self::User => serializer.serialize_unit_variant("Source", 1u32, "User"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Alert details"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Details {
        #[doc = "Type of timegrain cadence"]
        #[serde(rename = "timeGrainType", default, skip_serializing_if = "Option::is_none")]
        pub time_grain_type: Option<details::TimeGrainType>,
        #[doc = "datetime of periodStartDate"]
        #[serde(rename = "periodStartDate", default, skip_serializing_if = "Option::is_none")]
        pub period_start_date: Option<String>,
        #[doc = "notificationId that triggered this alert"]
        #[serde(rename = "triggeredBy", default, skip_serializing_if = "Option::is_none")]
        pub triggered_by: Option<String>,
        #[doc = "array of resourceGroups to filter by"]
        #[serde(
            rename = "resourceGroupFilter",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub resource_group_filter: Vec<serde_json::Value>,
        #[doc = "array of resources to filter by"]
        #[serde(
            rename = "resourceFilter",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub resource_filter: Vec<serde_json::Value>,
        #[doc = "array of meters to filter by"]
        #[serde(
            rename = "meterFilter",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub meter_filter: Vec<serde_json::Value>,
        #[doc = "tags to filter by"]
        #[serde(rename = "tagFilter", default, skip_serializing_if = "Option::is_none")]
        pub tag_filter: Option<serde_json::Value>,
        #[doc = "notification threshold percentage as a decimal which activated this alert"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub threshold: Option<f64>,
        #[doc = "operator used to compare currentSpend with amount"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operator: Option<details::Operator>,
        #[doc = "budget threshold amount"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
        #[doc = "unit of currency being used"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub unit: Option<String>,
        #[doc = "current spend"]
        #[serde(rename = "currentSpend", default, skip_serializing_if = "Option::is_none")]
        pub current_spend: Option<f64>,
        #[doc = "list of emails to contact"]
        #[serde(
            rename = "contactEmails",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub contact_emails: Vec<String>,
        #[doc = "list of action groups to broadcast to"]
        #[serde(
            rename = "contactGroups",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub contact_groups: Vec<String>,
        #[doc = "list of contact roles"]
        #[serde(
            rename = "contactRoles",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub contact_roles: Vec<String>,
        #[doc = "overriding alert"]
        #[serde(rename = "overridingAlert", default, skip_serializing_if = "Option::is_none")]
        pub overriding_alert: Option<String>,
        #[doc = "department name"]
        #[serde(rename = "departmentName", default, skip_serializing_if = "Option::is_none")]
        pub department_name: Option<String>,
        #[doc = "company name"]
        #[serde(rename = "companyName", default, skip_serializing_if = "Option::is_none")]
        pub company_name: Option<String>,
        #[doc = "enrollment number"]
        #[serde(rename = "enrollmentNumber", default, skip_serializing_if = "Option::is_none")]
        pub enrollment_number: Option<String>,
        #[doc = "datetime of enrollmentStartDate"]
        #[serde(rename = "enrollmentStartDate", default, skip_serializing_if = "Option::is_none")]
        pub enrollment_start_date: Option<String>,
        #[doc = "datetime of enrollmentEndDate"]
        #[serde(rename = "enrollmentEndDate", default, skip_serializing_if = "Option::is_none")]
        pub enrollment_end_date: Option<String>,
        #[doc = "invoicing threshold"]
        #[serde(rename = "invoicingThreshold", default, skip_serializing_if = "Option::is_none")]
        pub invoicing_threshold: Option<f64>,
    }
    impl Details {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod details {
        use super::*;
        #[doc = "Type of timegrain cadence"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "TimeGrainType")]
        pub enum TimeGrainType {
            None,
            Monthly,
            Quarterly,
            Annually,
            BillingMonth,
            BillingQuarter,
            BillingAnnual,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for TimeGrainType {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for TimeGrainType {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for TimeGrainType {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::None => serializer.serialize_unit_variant("TimeGrainType", 0u32, "None"),
                    Self::Monthly => serializer.serialize_unit_variant("TimeGrainType", 1u32, "Monthly"),
                    Self::Quarterly => serializer.serialize_unit_variant("TimeGrainType", 2u32, "Quarterly"),
                    Self::Annually => serializer.serialize_unit_variant("TimeGrainType", 3u32, "Annually"),
                    Self::BillingMonth => serializer.serialize_unit_variant("TimeGrainType", 4u32, "BillingMonth"),
                    Self::BillingQuarter => serializer.serialize_unit_variant("TimeGrainType", 5u32, "BillingQuarter"),
                    Self::BillingAnnual => serializer.serialize_unit_variant("TimeGrainType", 6u32, "BillingAnnual"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = "operator used to compare currentSpend with amount"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Operator")]
        pub enum Operator {
            None,
            EqualTo,
            GreaterThan,
            GreaterThanOrEqualTo,
            LessThan,
            LessThanOrEqualTo,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for Operator {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for Operator {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for Operator {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::None => serializer.serialize_unit_variant("Operator", 0u32, "None"),
                    Self::EqualTo => serializer.serialize_unit_variant("Operator", 1u32, "EqualTo"),
                    Self::GreaterThan => serializer.serialize_unit_variant("Operator", 2u32, "GreaterThan"),
                    Self::GreaterThanOrEqualTo => serializer.serialize_unit_variant("Operator", 3u32, "GreaterThanOrEqualTo"),
                    Self::LessThan => serializer.serialize_unit_variant("Operator", 4u32, "LessThan"),
                    Self::LessThanOrEqualTo => serializer.serialize_unit_variant("Operator", 5u32, "LessThanOrEqualTo"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
    #[doc = "alert status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        None,
        Active,
        Overridden,
        Resolved,
        Dismissed,
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
                Self::None => serializer.serialize_unit_variant("Status", 0u32, "None"),
                Self::Active => serializer.serialize_unit_variant("Status", 1u32, "Active"),
                Self::Overridden => serializer.serialize_unit_variant("Status", 2u32, "Overridden"),
                Self::Resolved => serializer.serialize_unit_variant("Status", 3u32, "Resolved"),
                Self::Dismissed => serializer.serialize_unit_variant("Status", 4u32, "Dismissed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Result of alerts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertsResult {
    #[doc = "List of alerts."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Alert>,
    #[doc = "URL to get the next set of alerts results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl AlertsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Kind/type of the benefit."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BenefitKind")]
pub enum BenefitKind {
    IncludedQuantity,
    Reservation,
    SavingsPlan,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BenefitKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BenefitKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BenefitKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::IncludedQuantity => serializer.serialize_unit_variant("BenefitKind", 0u32, "IncludedQuantity"),
            Self::Reservation => serializer.serialize_unit_variant("BenefitKind", 1u32, "Reservation"),
            Self::SavingsPlan => serializer.serialize_unit_variant("BenefitKind", 2u32, "SavingsPlan"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The benefit resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BenefitResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Kind/type of the benefit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<BenefitKind>,
}
impl BenefitResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of benefit utilization summaries."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BenefitUtilizationSummariesListResult {
    #[doc = "The list of benefit utilization summaries."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BenefitUtilizationSummary>,
    #[doc = "The link (URL) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BenefitUtilizationSummariesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BenefitUtilizationSummariesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Benefit utilization summary resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BenefitUtilizationSummary {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Kind/type of the benefit."]
    pub kind: BenefitKind,
}
impl BenefitUtilizationSummary {
    pub fn new(kind: BenefitKind) -> Self {
        Self {
            resource: Resource::default(),
            kind,
        }
    }
}
#[doc = "The properties of a benefit utilization summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BenefitUtilizationSummaryProperties {
    #[doc = "ARM SKU name. For example, 'Compute_Savings_Plan' for savings plan."]
    #[serde(rename = "armSkuName", default, skip_serializing_if = "Option::is_none")]
    pub arm_sku_name: Option<String>,
    #[doc = "The benefit ID is the identifier of the benefit."]
    #[serde(rename = "benefitId", default, skip_serializing_if = "Option::is_none")]
    pub benefit_id: Option<String>,
    #[doc = "The benefit order ID is the identifier for a benefit purchase."]
    #[serde(rename = "benefitOrderId", default, skip_serializing_if = "Option::is_none")]
    pub benefit_order_id: Option<String>,
    #[doc = "Kind/type of the benefit."]
    #[serde(rename = "benefitType", default, skip_serializing_if = "Option::is_none")]
    pub benefit_type: Option<BenefitKind>,
    #[doc = "Date corresponding to the utilization summary record. If the grain of data is monthly, value for this field will be first day of the month."]
    #[serde(rename = "usageDate", default, with = "azure_core::date::rfc3339::option")]
    pub usage_date: Option<time::OffsetDateTime>,
}
impl BenefitUtilizationSummaryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The blob information generated by this operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlobInfo {
    #[doc = "Link to the blob to download file."]
    #[serde(rename = "blobLink", default, skip_serializing_if = "Option::is_none")]
    pub blob_link: Option<String>,
    #[doc = "Bytes in the blob."]
    #[serde(rename = "byteCount", default, skip_serializing_if = "Option::is_none")]
    pub byte_count: Option<i64>,
}
impl BlobInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A budget resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Budget {
    #[serde(flatten)]
    pub cost_management_proxy_resource: CostManagementProxyResource,
    #[doc = "The properties of the budget."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BudgetProperties>,
}
impl Budget {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The comparison expression to be used in the budgets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BudgetComparisonExpression {
    #[doc = "The name of the column to use in comparison."]
    pub name: String,
    #[doc = "The operator to use for comparison."]
    pub operator: budget_comparison_expression::Operator,
    #[doc = "Array of values to use for comparison"]
    pub values: Vec<String>,
}
impl BudgetComparisonExpression {
    pub fn new(name: String, operator: budget_comparison_expression::Operator, values: Vec<String>) -> Self {
        Self { name, operator, values }
    }
}
pub mod budget_comparison_expression {
    use super::*;
    #[doc = "The operator to use for comparison."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        In,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::In => serializer.serialize_unit_variant("Operator", 0u32, "In"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "May be used to filter budgets by user-specified dimensions and/or tags.\n\n Supported for CategoryType(s): Cost, ReservationUtilization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BudgetFilter {
    #[doc = "The logical \"AND\" expression. Must have at least 2 items.\n\n Supported for CategoryType(s): Cost."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub and: Vec<BudgetFilterProperties>,
    #[doc = "The comparison expression to be used in the budgets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<BudgetComparisonExpression>,
    #[doc = "The comparison expression to be used in the budgets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BudgetComparisonExpression>,
}
impl BudgetFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Dimensions or Tags to filter a budget by.\n\n Supported for CategoryType(s): Cost, ReservationUtilization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BudgetFilterProperties {
    #[doc = "The comparison expression to be used in the budgets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<BudgetComparisonExpression>,
    #[doc = "The comparison expression to be used in the budgets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BudgetComparisonExpression>,
}
impl BudgetFilterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the budget."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BudgetProperties {
    #[doc = "The category of the budget.\n- 'Cost' defines a Budget.\n- 'ReservationUtilization' defines a Reservation Utilization Alert Rule."]
    pub category: budget_properties::Category,
    #[doc = "The total amount of cost to track with the budget.\n\n Supported for CategoryType(s): Cost.\n\n Required for CategoryType(s): Cost."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[doc = "The time covered by a budget. Tracking of the amount will be reset based on the time grain.\n\nSupported for CategoryType(s): Cost, ReservationUtilization.\n\n Supported timeGrainTypes for **CategoryType: Cost**\n\n- Monthly\n- Quarterly\n- Annually\n- BillingMonth*\n- BillingQuarter*\n- BillingAnnual*\n\n  *only supported for Web Direct customers.\n\n Supported timeGrainTypes for **CategoryType: ReservationUtilization**\n- Last7Days\n- Last30Days\n\n Required for CategoryType(s): Cost, ReservationUtilization."]
    #[serde(rename = "timeGrain")]
    pub time_grain: budget_properties::TimeGrain,
    #[doc = "The time period that defines the active period of the budget. The budget will evaluate data on or after the startDate and will expire on the endDate.\n\n Supported for CategoryType(s): Cost, ReservationUtilization.\n\n Required for CategoryType(s): Cost, ReservationUtilization."]
    #[serde(rename = "timePeriod")]
    pub time_period: BudgetTimePeriod,
    #[doc = "May be used to filter budgets by user-specified dimensions and/or tags.\n\n Supported for CategoryType(s): Cost, ReservationUtilization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<BudgetFilter>,
    #[doc = "The current amount of cost which is being tracked for a budget.\n\n Supported for CategoryType(s): Cost."]
    #[serde(rename = "currentSpend", default, skip_serializing_if = "Option::is_none")]
    pub current_spend: Option<CurrentSpend>,
    #[doc = "Dictionary of notifications associated with the budget.\n\n Supported for CategoryType(s): Cost, ReservationUtilization.\n\n- Constraints for **CategoryType: Cost** - Budget can have up to 5 notifications with thresholdType: Actual and 5 notifications with thresholdType: Forecasted.\n- Constraints for **CategoryType: ReservationUtilization** - Only one notification allowed. thresholdType is not applicable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notifications: Option<serde_json::Value>,
    #[doc = "The forecasted cost which is being tracked for a budget.\n\n Supported for CategoryType(s): Cost."]
    #[serde(rename = "forecastSpend", default, skip_serializing_if = "Option::is_none")]
    pub forecast_spend: Option<ForecastSpend>,
}
impl BudgetProperties {
    pub fn new(category: budget_properties::Category, time_grain: budget_properties::TimeGrain, time_period: BudgetTimePeriod) -> Self {
        Self {
            category,
            amount: None,
            time_grain,
            time_period,
            filter: None,
            current_spend: None,
            notifications: None,
            forecast_spend: None,
        }
    }
}
pub mod budget_properties {
    use super::*;
    #[doc = "The category of the budget.\n- 'Cost' defines a Budget.\n- 'ReservationUtilization' defines a Reservation Utilization Alert Rule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Category")]
    pub enum Category {
        Cost,
        ReservationUtilization,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Category {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Category {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Category {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Cost => serializer.serialize_unit_variant("Category", 0u32, "Cost"),
                Self::ReservationUtilization => serializer.serialize_unit_variant("Category", 1u32, "ReservationUtilization"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The time covered by a budget. Tracking of the amount will be reset based on the time grain.\n\nSupported for CategoryType(s): Cost, ReservationUtilization.\n\n Supported timeGrainTypes for **CategoryType: Cost**\n\n- Monthly\n- Quarterly\n- Annually\n- BillingMonth*\n- BillingQuarter*\n- BillingAnnual*\n\n  *only supported for Web Direct customers.\n\n Supported timeGrainTypes for **CategoryType: ReservationUtilization**\n- Last7Days\n- Last30Days\n\n Required for CategoryType(s): Cost, ReservationUtilization."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TimeGrain")]
    pub enum TimeGrain {
        Monthly,
        Quarterly,
        Annually,
        BillingMonth,
        BillingQuarter,
        BillingAnnual,
        Last7Days,
        Last30Days,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TimeGrain {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TimeGrain {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TimeGrain {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Monthly => serializer.serialize_unit_variant("TimeGrain", 0u32, "Monthly"),
                Self::Quarterly => serializer.serialize_unit_variant("TimeGrain", 1u32, "Quarterly"),
                Self::Annually => serializer.serialize_unit_variant("TimeGrain", 2u32, "Annually"),
                Self::BillingMonth => serializer.serialize_unit_variant("TimeGrain", 3u32, "BillingMonth"),
                Self::BillingQuarter => serializer.serialize_unit_variant("TimeGrain", 4u32, "BillingQuarter"),
                Self::BillingAnnual => serializer.serialize_unit_variant("TimeGrain", 5u32, "BillingAnnual"),
                Self::Last7Days => serializer.serialize_unit_variant("TimeGrain", 6u32, "Last7Days"),
                Self::Last30Days => serializer.serialize_unit_variant("TimeGrain", 7u32, "Last30Days"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The time period that defines the active period of the budget. The budget will evaluate data on or after the startDate and will expire on the endDate.\n\n Supported for CategoryType(s): Cost, ReservationUtilization.\n\n Required for CategoryType(s): Cost, ReservationUtilization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BudgetTimePeriod {
    #[doc = "The start date for the budget.\n\n- Constraints for **CategoryType: Cost** - Must be first of the month and should be less than the end date. Budget start date must be on or after June 1, 2017. Future start date should not be more than twelve months. Past start date should  be selected within the timegrain period.\n\n- Constraints for **CategoryType: ReservationUtilization** - Must be on or after the current date and less than the end date."]
    #[serde(rename = "startDate", with = "azure_core::date::rfc3339")]
    pub start_date: time::OffsetDateTime,
    #[doc = "The end date for the budget.\n\n- Constraints for **CategoryType: Cost** - No constraints. If not provided, we default this to 10 years from the start date.\n\n- Constraints for **CategoryType: ReservationUtilization** - End date cannot be more than 3 years after the start date."]
    #[serde(rename = "endDate", default, with = "azure_core::date::rfc3339::option")]
    pub end_date: Option<time::OffsetDateTime>,
}
impl BudgetTimePeriod {
    pub fn new(start_date: time::OffsetDateTime) -> Self {
        Self {
            start_date,
            end_date: None,
        }
    }
}
#[doc = "Result of listing budgets. It contains a list of available budgets in the scope provided."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BudgetsListResult {
    #[doc = "The list of budgets."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Budget>,
    #[doc = "The link (url) to the next page of results.\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BudgetsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BudgetsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The check availability request body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityRequest {
    #[doc = "The name of the resource for which availability needs to be checked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CheckNameAvailabilityRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The check availability result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResponse {
    #[doc = "Indicates if the resource name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason why the given name is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_response::Reason>,
    #[doc = "Detailed reason why the given name is available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_response {
    use super::*;
    #[doc = "The reason why the given name is not available."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Reason")]
    pub enum Reason {
        Invalid,
        AlreadyExists,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Reason {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Reason {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Reason {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("Reason", 0u32, "Invalid"),
                Self::AlreadyExists => serializer.serialize_unit_variant("Reason", 1u32, "AlreadyExists"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The common properties of the export."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommonExportProperties {
    #[doc = "The format of the export being delivered. Currently only 'Csv' is supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<common_export_properties::Format>,
    #[doc = "The delivery information associated with a export."]
    #[serde(rename = "deliveryInfo")]
    pub delivery_info: ExportDeliveryInfo,
    #[doc = "The definition of an export."]
    pub definition: ExportDefinition,
    #[doc = "Result of listing the run history of an export."]
    #[serde(rename = "runHistory", default, skip_serializing_if = "Option::is_none")]
    pub run_history: Option<ExportExecutionListResult>,
    #[doc = "If set to true, exported data will be partitioned by size and placed in a blob directory together with a manifest file. Note: this option is currently available only for Microsoft Customer Agreement commerce scopes."]
    #[serde(rename = "partitionData", default, skip_serializing_if = "Option::is_none")]
    pub partition_data: Option<bool>,
    #[doc = "If the export has an active schedule, provides an estimate of the next run time."]
    #[serde(rename = "nextRunTimeEstimate", default, with = "azure_core::date::rfc3339::option")]
    pub next_run_time_estimate: Option<time::OffsetDateTime>,
}
impl CommonExportProperties {
    pub fn new(delivery_info: ExportDeliveryInfo, definition: ExportDefinition) -> Self {
        Self {
            format: None,
            delivery_info,
            definition,
            run_history: None,
            partition_data: None,
            next_run_time_estimate: None,
        }
    }
}
pub mod common_export_properties {
    use super::*;
    #[doc = "The format of the export being delivered. Currently only 'Csv' is supported."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Format")]
    pub enum Format {
        Csv,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Format {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Format {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Format {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Csv => serializer.serialize_unit_variant("Format", 0u32, "Csv"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The result of the long running operation for cost details Api."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CostDetailsOperationResults {
    #[doc = "The id of the long running operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the long running operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the long running operation."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The status of the cost details operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<cost_details_operation_results::Status>,
    #[doc = "The manifest of the report generated by the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest: Option<ReportManifest>,
    #[doc = "The time at which report URL becomes invalid/expires in UTC e.g. 2020-12-08T05:55:59.4394737Z."]
    #[serde(rename = "validTill", default, with = "azure_core::date::rfc3339::option")]
    pub valid_till: Option<time::OffsetDateTime>,
    #[doc = "The details of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}
impl CostDetailsOperationResults {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cost_details_operation_results {
    use super::*;
    #[doc = "The status of the cost details operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Completed,
        NoDataFound,
        Failed,
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
                Self::Completed => serializer.serialize_unit_variant("Status", 0u32, "Completed"),
                Self::NoDataFound => serializer.serialize_unit_variant("Status", 1u32, "NoDataFound"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The start and end date for pulling data for the cost detailed report. API only allows data to be pulled for 1 month or less and no older than 13 months."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CostDetailsTimePeriod {
    #[doc = "The start date to pull data from. example format 2020-03-15"]
    pub start: String,
    #[doc = "The end date to pull data to. example format 2020-03-15"]
    pub end: String,
}
impl CostDetailsTimePeriod {
    pub fn new(start: String, end: String) -> Self {
        Self { start, end }
    }
}
#[doc = "A Cost management REST API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CostManagementOperation {
    #[serde(flatten)]
    pub operation: Operation,
    #[doc = "Operation id: {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl CostManagementOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CostManagementProxyResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "eTag of the resource. To handle concurrent update scenario, this field will be used to determine whether the user is updating the latest version or not."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
impl CostManagementProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CostManagementResource {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "SKU of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "ETag of the resource."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl CostManagementResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The current amount of cost which is being tracked for a budget.\n\n Supported for CategoryType(s): Cost."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CurrentSpend {
    #[doc = "The total amount of cost which is being tracked by the budget."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[doc = "The unit of measure for the budget amount."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
impl CurrentSpend {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Days of Week."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DaysOfWeek")]
pub enum DaysOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DaysOfWeek {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DaysOfWeek {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DaysOfWeek {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Monday => serializer.serialize_unit_variant("DaysOfWeek", 0u32, "Monday"),
            Self::Tuesday => serializer.serialize_unit_variant("DaysOfWeek", 1u32, "Tuesday"),
            Self::Wednesday => serializer.serialize_unit_variant("DaysOfWeek", 2u32, "Wednesday"),
            Self::Thursday => serializer.serialize_unit_variant("DaysOfWeek", 3u32, "Thursday"),
            Self::Friday => serializer.serialize_unit_variant("DaysOfWeek", 4u32, "Friday"),
            Self::Saturday => serializer.serialize_unit_variant("DaysOfWeek", 5u32, "Saturday"),
            Self::Sunday => serializer.serialize_unit_variant("DaysOfWeek", 6u32, "Sunday"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "List of Dimension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dimension {
    #[serde(flatten)]
    pub cost_management_resource: CostManagementResource,
    #[doc = "Dimension properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DimensionProperties>,
}
impl Dimension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dimension properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DimensionProperties {
    #[doc = "Dimension description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Filter enabled."]
    #[serde(rename = "filterEnabled", default, skip_serializing_if = "Option::is_none")]
    pub filter_enabled: Option<bool>,
    #[doc = "Grouping enabled."]
    #[serde(rename = "groupingEnabled", default, skip_serializing_if = "Option::is_none")]
    pub grouping_enabled: Option<bool>,
    #[doc = "Dimension data."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data: Vec<String>,
    #[doc = "Total number of data for the dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    #[doc = "Dimension category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "Usage start."]
    #[serde(rename = "usageStart", default, with = "azure_core::date::rfc3339::option")]
    pub usage_start: Option<time::OffsetDateTime>,
    #[doc = "Usage end."]
    #[serde(rename = "usageEnd", default, with = "azure_core::date::rfc3339::option")]
    pub usage_end: Option<time::OffsetDateTime>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl DimensionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of listing dimensions. It contains a list of available dimensions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DimensionsListResult {
    #[doc = "The list of dimensions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Dimension>,
}
impl azure_core::Continuable for DimensionsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DimensionsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The request payload to update an alert"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DismissAlertPayload {
    #[doc = "Alert properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertProperties>,
}
impl DismissAlertPayload {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The URL to download the generated report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DownloadUrl {
    #[doc = "The time at which report URL becomes invalid/expires in UTC e.g. 2020-12-08T05:55:59.4394737Z."]
    #[serde(rename = "expiryTime", default, with = "azure_core::date::rfc3339::option")]
    pub expiry_time: Option<time::OffsetDateTime>,
    #[doc = "The time at which report URL becomes invalid/expires in UTC e.g. 2020-12-08T05:55:59.4394737Z."]
    #[serde(rename = "validTill", default, with = "azure_core::date::rfc3339::option")]
    pub valid_till: Option<time::OffsetDateTime>,
    #[doc = "The URL to download the generated report."]
    #[serde(rename = "downloadUrl", default, skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
}
impl DownloadUrl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of the error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of the error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetailsWithNestedDetails {
    #[serde(flatten)]
    pub error_details: ErrorDetails,
    #[doc = "The additional details of the error."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetailsWithNestedDetails>,
}
impl ErrorDetailsWithNestedDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates that the service is not able to process the incoming request. The reason is provided in the error message. \n\nSome Error responses: \n\n * 429 TooManyRequests - Request is throttled. Retry after waiting for the time specified in the \"x-ms-ratelimit-microsoft.consumption-retry-after\" header. \n\n * 503 ServiceUnavailable - Service is temporarily unavailable. Retry after waiting for the time specified in the \"Retry-After\" header."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The details of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}
impl azure_core::Continuable for ErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates that the service is not able to process the incoming request. The reason is provided in the error message. \n\nSome Error responses: \n\n * 429 TooManyRequests - Request is throttled. Retry after waiting for the time specified in the \"x-ms-ratelimit-microsoft.consumption-retry-after\" header. \n\n * 503 ServiceUnavailable - Service is temporarily unavailable. Retry after waiting for the time specified in the \"Retry-After\" header."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseWithNestedDetails {
    #[doc = "The details of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetailsWithNestedDetails>,
}
impl azure_core::Continuable for ErrorResponseWithNestedDetails {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponseWithNestedDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An export resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Export {
    #[serde(flatten)]
    pub cost_management_proxy_resource: CostManagementProxyResource,
    #[doc = "The properties of the export."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExportProperties>,
}
impl Export {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The definition for data in the export."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportDataset {
    #[doc = "The granularity of rows in the export. Currently only 'Daily' is supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granularity: Option<export_dataset::Granularity>,
    #[doc = "The export dataset configuration. Allows columns to be selected for the export. If not provided then the export will include all available columns."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ExportDatasetConfiguration>,
}
impl ExportDataset {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod export_dataset {
    use super::*;
    #[doc = "The granularity of rows in the export. Currently only 'Daily' is supported."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Granularity")]
    pub enum Granularity {
        Daily,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Granularity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Granularity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Granularity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Daily => serializer.serialize_unit_variant("Granularity", 0u32, "Daily"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The export dataset configuration. Allows columns to be selected for the export. If not provided then the export will include all available columns."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportDatasetConfiguration {
    #[doc = "Array of column names to be included in the export. If not provided then the export will include all available columns. The available columns can vary by customer channel (see examples)."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub columns: Vec<String>,
}
impl ExportDatasetConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The definition of an export."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportDefinition {
    #[doc = "The type of the export. Note that 'Usage' is equivalent to 'ActualCost' and is applicable to exports that do not yet provide data for charges or amortization for service reservations."]
    #[serde(rename = "type")]
    pub type_: export_definition::Type,
    #[doc = "The time frame for pulling data for the export. If custom, then a specific time period must be provided."]
    pub timeframe: export_definition::Timeframe,
    #[doc = "The date range for data in the export. This should only be specified with timeFrame set to 'Custom'. The maximum date range is 3 months."]
    #[serde(rename = "timePeriod", default, skip_serializing_if = "Option::is_none")]
    pub time_period: Option<ExportTimePeriod>,
    #[doc = "The definition for data in the export."]
    #[serde(rename = "dataSet", default, skip_serializing_if = "Option::is_none")]
    pub data_set: Option<ExportDataset>,
}
impl ExportDefinition {
    pub fn new(type_: export_definition::Type, timeframe: export_definition::Timeframe) -> Self {
        Self {
            type_,
            timeframe,
            time_period: None,
            data_set: None,
        }
    }
}
pub mod export_definition {
    use super::*;
    #[doc = "The type of the export. Note that 'Usage' is equivalent to 'ActualCost' and is applicable to exports that do not yet provide data for charges or amortization for service reservations."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Usage,
        ActualCost,
        AmortizedCost,
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
                Self::Usage => serializer.serialize_unit_variant("Type", 0u32, "Usage"),
                Self::ActualCost => serializer.serialize_unit_variant("Type", 1u32, "ActualCost"),
                Self::AmortizedCost => serializer.serialize_unit_variant("Type", 2u32, "AmortizedCost"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The time frame for pulling data for the export. If custom, then a specific time period must be provided."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Timeframe")]
    pub enum Timeframe {
        MonthToDate,
        BillingMonthToDate,
        TheLastMonth,
        TheLastBillingMonth,
        WeekToDate,
        Custom,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Timeframe {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Timeframe {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Timeframe {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MonthToDate => serializer.serialize_unit_variant("Timeframe", 0u32, "MonthToDate"),
                Self::BillingMonthToDate => serializer.serialize_unit_variant("Timeframe", 1u32, "BillingMonthToDate"),
                Self::TheLastMonth => serializer.serialize_unit_variant("Timeframe", 2u32, "TheLastMonth"),
                Self::TheLastBillingMonth => serializer.serialize_unit_variant("Timeframe", 3u32, "TheLastBillingMonth"),
                Self::WeekToDate => serializer.serialize_unit_variant("Timeframe", 4u32, "WeekToDate"),
                Self::Custom => serializer.serialize_unit_variant("Timeframe", 5u32, "Custom"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "This represents the blob storage account location where exports of costs will be delivered. There are two ways to configure the destination. The approach recommended for most customers is to specify the resourceId of the storage account. This requires a one-time registration of the account's subscription with the Microsoft.CostManagementExports resource provider in order to give Cost Management services access to the storage. When creating an export in the Azure portal this registration is performed automatically but API users may need to register the subscription explicitly (for more information see https://docs.microsoft.com/en-us/azure/azure-resource-manager/resource-manager-supported-services ). Another way to configure the destination is available ONLY to Partners with a Microsoft Partner Agreement plan who are global admins of their billing account. These Partners, instead of specifying the resourceId of a storage account, can specify the storage account name along with a SAS token for the account. This allows exports of costs to a storage account in any tenant. The SAS token should be created for the blob service with Service/Container/Object resource types and with Read/Write/Delete/List/Add/Create permissions (for more information see https://docs.microsoft.com/en-us/azure/cost-management-billing/costs/export-cost-data-storage-account-sas-key )."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportDeliveryDestination {
    #[doc = "The resource id of the storage account where exports will be delivered. This is not required if a sasToken and storageAccount are specified."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The name of the container where exports will be uploaded. If the container does not exist it will be created."]
    pub container: String,
    #[doc = "The name of the directory where exports will be uploaded."]
    #[serde(rename = "rootFolderPath", default, skip_serializing_if = "Option::is_none")]
    pub root_folder_path: Option<String>,
    #[doc = "A SAS token for the storage account. For a restricted set of Azure customers this together with storageAccount can be specified instead of resourceId. Note: the value returned by the API for this property will always be obfuscated. Returning this same obfuscated value will not result in the SAS token being updated. To update this value a new SAS token must be specified."]
    #[serde(rename = "sasToken", default, skip_serializing_if = "Option::is_none")]
    pub sas_token: Option<String>,
    #[doc = "The storage account where exports will be uploaded. For a restricted set of Azure customers this together with sasToken can be specified instead of resourceId."]
    #[serde(rename = "storageAccount", default, skip_serializing_if = "Option::is_none")]
    pub storage_account: Option<String>,
}
impl ExportDeliveryDestination {
    pub fn new(container: String) -> Self {
        Self {
            resource_id: None,
            container,
            root_folder_path: None,
            sas_token: None,
            storage_account: None,
        }
    }
}
#[doc = "The delivery information associated with a export."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportDeliveryInfo {
    #[doc = "This represents the blob storage account location where exports of costs will be delivered. There are two ways to configure the destination. The approach recommended for most customers is to specify the resourceId of the storage account. This requires a one-time registration of the account's subscription with the Microsoft.CostManagementExports resource provider in order to give Cost Management services access to the storage. When creating an export in the Azure portal this registration is performed automatically but API users may need to register the subscription explicitly (for more information see https://docs.microsoft.com/en-us/azure/azure-resource-manager/resource-manager-supported-services ). Another way to configure the destination is available ONLY to Partners with a Microsoft Partner Agreement plan who are global admins of their billing account. These Partners, instead of specifying the resourceId of a storage account, can specify the storage account name along with a SAS token for the account. This allows exports of costs to a storage account in any tenant. The SAS token should be created for the blob service with Service/Container/Object resource types and with Read/Write/Delete/List/Add/Create permissions (for more information see https://docs.microsoft.com/en-us/azure/cost-management-billing/costs/export-cost-data-storage-account-sas-key )."]
    pub destination: ExportDeliveryDestination,
}
impl ExportDeliveryInfo {
    pub fn new(destination: ExportDeliveryDestination) -> Self {
        Self { destination }
    }
}
#[doc = "Result of listing the run history of an export."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportExecutionListResult {
    #[doc = "A list of export runs."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ExportRun>,
}
impl ExportExecutionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of listing exports. It contains a list of available exports in the scope provided."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportListResult {
    #[doc = "The list of exports."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Export>,
}
impl ExportListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the export."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportProperties {
    #[serde(flatten)]
    pub common_export_properties: CommonExportProperties,
    #[doc = "The schedule associated with the export."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<ExportSchedule>,
}
impl ExportProperties {
    pub fn new(common_export_properties: CommonExportProperties) -> Self {
        Self {
            common_export_properties,
            schedule: None,
        }
    }
}
#[doc = "The start and end date for recurrence schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportRecurrencePeriod {
    #[doc = "The start date of recurrence."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub from: time::OffsetDateTime,
    #[doc = "The end date of recurrence."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub to: Option<time::OffsetDateTime>,
}
impl ExportRecurrencePeriod {
    pub fn new(from: time::OffsetDateTime) -> Self {
        Self { from, to: None }
    }
}
#[doc = "An export run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportRun {
    #[serde(flatten)]
    pub cost_management_proxy_resource: CostManagementProxyResource,
    #[doc = "The properties of the export run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExportRunProperties>,
}
impl ExportRun {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the export run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportRunProperties {
    #[doc = "The type of the export run."]
    #[serde(rename = "executionType", default, skip_serializing_if = "Option::is_none")]
    pub execution_type: Option<export_run_properties::ExecutionType>,
    #[doc = "The last known status of the export run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<export_run_properties::Status>,
    #[doc = "The identifier for the entity that triggered the export. For on-demand runs it is the user email. For scheduled runs it is 'System'."]
    #[serde(rename = "submittedBy", default, skip_serializing_if = "Option::is_none")]
    pub submitted_by: Option<String>,
    #[doc = "The time when export was queued to be run."]
    #[serde(rename = "submittedTime", default, with = "azure_core::date::rfc3339::option")]
    pub submitted_time: Option<time::OffsetDateTime>,
    #[doc = "The time when export was picked up to be run."]
    #[serde(rename = "processingStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub processing_start_time: Option<time::OffsetDateTime>,
    #[doc = "The time when the export run finished."]
    #[serde(rename = "processingEndTime", default, with = "azure_core::date::rfc3339::option")]
    pub processing_end_time: Option<time::OffsetDateTime>,
    #[doc = "The name of the exported file."]
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[doc = "The common properties of the export."]
    #[serde(rename = "runSettings", default, skip_serializing_if = "Option::is_none")]
    pub run_settings: Option<CommonExportProperties>,
    #[doc = "The details of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}
impl ExportRunProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod export_run_properties {
    use super::*;
    #[doc = "The type of the export run."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ExecutionType")]
    pub enum ExecutionType {
        OnDemand,
        Scheduled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ExecutionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ExecutionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ExecutionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::OnDemand => serializer.serialize_unit_variant("ExecutionType", 0u32, "OnDemand"),
                Self::Scheduled => serializer.serialize_unit_variant("ExecutionType", 1u32, "Scheduled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The last known status of the export run."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Queued,
        InProgress,
        Completed,
        Failed,
        Timeout,
        NewDataNotAvailable,
        DataNotAvailable,
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
                Self::Queued => serializer.serialize_unit_variant("Status", 0u32, "Queued"),
                Self::InProgress => serializer.serialize_unit_variant("Status", 1u32, "InProgress"),
                Self::Completed => serializer.serialize_unit_variant("Status", 2u32, "Completed"),
                Self::Failed => serializer.serialize_unit_variant("Status", 3u32, "Failed"),
                Self::Timeout => serializer.serialize_unit_variant("Status", 4u32, "Timeout"),
                Self::NewDataNotAvailable => serializer.serialize_unit_variant("Status", 5u32, "NewDataNotAvailable"),
                Self::DataNotAvailable => serializer.serialize_unit_variant("Status", 6u32, "DataNotAvailable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The schedule associated with the export."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportSchedule {
    #[doc = "The status of the export's schedule. If 'Inactive', the export's schedule is paused."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<export_schedule::Status>,
    #[doc = "The schedule recurrence."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<export_schedule::Recurrence>,
    #[doc = "The start and end date for recurrence schedule."]
    #[serde(rename = "recurrencePeriod", default, skip_serializing_if = "Option::is_none")]
    pub recurrence_period: Option<ExportRecurrencePeriod>,
}
impl ExportSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod export_schedule {
    use super::*;
    #[doc = "The status of the export's schedule. If 'Inactive', the export's schedule is paused."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Active,
        Inactive,
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
                Self::Active => serializer.serialize_unit_variant("Status", 0u32, "Active"),
                Self::Inactive => serializer.serialize_unit_variant("Status", 1u32, "Inactive"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The schedule recurrence."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Recurrence")]
    pub enum Recurrence {
        Daily,
        Weekly,
        Monthly,
        Annually,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Recurrence {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Recurrence {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Recurrence {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Daily => serializer.serialize_unit_variant("Recurrence", 0u32, "Daily"),
                Self::Weekly => serializer.serialize_unit_variant("Recurrence", 1u32, "Weekly"),
                Self::Monthly => serializer.serialize_unit_variant("Recurrence", 2u32, "Monthly"),
                Self::Annually => serializer.serialize_unit_variant("Recurrence", 3u32, "Annually"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The date range for data in the export. This should only be specified with timeFrame set to 'Custom'. The maximum date range is 3 months."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportTimePeriod {
    #[doc = "The start date for export data."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub from: time::OffsetDateTime,
    #[doc = "The end date for export data."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub to: time::OffsetDateTime,
}
impl ExportTimePeriod {
    pub fn new(from: time::OffsetDateTime, to: time::OffsetDateTime) -> Self {
        Self { from, to }
    }
}
#[doc = "Destination of the view data. This is optional. Currently only CSV format is supported."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileDestination {
    #[doc = "Destination of the view data. Currently only CSV format is supported."]
    #[serde(
        rename = "fileFormats",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub file_formats: Vec<FileFormat>,
}
impl FileDestination {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Destination of the view data. Currently only CSV format is supported."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FileFormat")]
pub enum FileFormat {
    Csv,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FileFormat {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FileFormat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FileFormat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Csv => serializer.serialize_unit_variant("FileFormat", 0u32, "Csv"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The aggregation expression to be used in the forecast."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForecastAggregation {
    #[doc = "The name of the column to aggregate."]
    pub name: forecast_aggregation::Name,
    #[doc = "The name of the aggregation function to use."]
    pub function: forecast_aggregation::Function,
}
impl ForecastAggregation {
    pub fn new(name: forecast_aggregation::Name, function: forecast_aggregation::Function) -> Self {
        Self { name, function }
    }
}
pub mod forecast_aggregation {
    use super::*;
    #[doc = "The name of the column to aggregate."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        #[serde(rename = "PreTaxCostUSD")]
        PreTaxCostUsd,
        Cost,
        #[serde(rename = "CostUSD")]
        CostUsd,
        PreTaxCost,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Name {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Name {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PreTaxCostUsd => serializer.serialize_unit_variant("Name", 0u32, "PreTaxCostUSD"),
                Self::Cost => serializer.serialize_unit_variant("Name", 1u32, "Cost"),
                Self::CostUsd => serializer.serialize_unit_variant("Name", 2u32, "CostUSD"),
                Self::PreTaxCost => serializer.serialize_unit_variant("Name", 3u32, "PreTaxCost"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The name of the aggregation function to use."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Function")]
    pub enum Function {
        Sum,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Function {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Function {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Function {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Sum => serializer.serialize_unit_variant("Function", 0u32, "Sum"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Forecast column properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ForecastColumn {
    #[doc = "The name of column."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of column."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ForecastColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The comparison expression to be used in the forecast."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForecastComparisonExpression {
    #[doc = "The name of the column to use in comparison."]
    pub name: String,
    #[doc = "The operator to use for comparison."]
    pub operator: forecast_comparison_expression::Operator,
    #[doc = "Array of values to use for comparison"]
    pub values: Vec<String>,
}
impl ForecastComparisonExpression {
    pub fn new(name: String, operator: forecast_comparison_expression::Operator, values: Vec<String>) -> Self {
        Self { name, operator, values }
    }
}
pub mod forecast_comparison_expression {
    use super::*;
    #[doc = "The operator to use for comparison."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        In,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::In => serializer.serialize_unit_variant("Operator", 0u32, "In"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The definition of data present in the forecast."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForecastDataset {
    #[doc = "The granularity of rows in the forecast."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granularity: Option<GranularityType>,
    #[doc = "The configuration of dataset in the forecast."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ForecastDatasetConfiguration>,
    #[doc = "Dictionary of aggregation expression to use in the forecast. The key of each item in the dictionary is the alias for the aggregated column. forecast can have up to 2 aggregation clauses."]
    pub aggregation: serde_json::Value,
    #[doc = "The filter expression to be used in the export."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<ForecastFilter>,
}
impl ForecastDataset {
    pub fn new(aggregation: serde_json::Value) -> Self {
        Self {
            granularity: None,
            configuration: None,
            aggregation,
            filter: None,
        }
    }
}
#[doc = "The configuration of dataset in the forecast."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ForecastDatasetConfiguration {
    #[doc = "Array of column names to be included in the forecast. Any valid forecast column name is allowed. If not provided, then forecast includes all columns."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub columns: Vec<String>,
}
impl ForecastDatasetConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The definition of a forecast."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForecastDefinition {
    #[doc = "The type of the forecast."]
    #[serde(rename = "type")]
    pub type_: ForecastType,
    #[doc = "The time frame for pulling data for the forecast."]
    pub timeframe: ForecastTimeframe,
    #[doc = "Has time period for pulling data for the forecast."]
    #[serde(rename = "timePeriod", default, skip_serializing_if = "Option::is_none")]
    pub time_period: Option<ForecastTimePeriod>,
    #[doc = "The definition of data present in the forecast."]
    pub dataset: ForecastDataset,
    #[doc = "A boolean determining if actualCost will be included."]
    #[serde(rename = "includeActualCost", default, skip_serializing_if = "Option::is_none")]
    pub include_actual_cost: Option<bool>,
    #[doc = "A boolean determining if FreshPartialCost will be included."]
    #[serde(rename = "includeFreshPartialCost", default, skip_serializing_if = "Option::is_none")]
    pub include_fresh_partial_cost: Option<bool>,
}
impl ForecastDefinition {
    pub fn new(type_: ForecastType, timeframe: ForecastTimeframe, dataset: ForecastDataset) -> Self {
        Self {
            type_,
            timeframe,
            time_period: None,
            dataset,
            include_actual_cost: None,
            include_fresh_partial_cost: None,
        }
    }
}
#[doc = "The filter expression to be used in the export."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ForecastFilter {
    #[doc = "The logical \"AND\" expression. Must have at least 2 items."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub and: Vec<ForecastFilter>,
    #[doc = "The logical \"OR\" expression. Must have at least 2 items."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub or: Vec<ForecastFilter>,
    #[doc = "The comparison expression to be used in the forecast."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<ForecastComparisonExpression>,
    #[doc = "The comparison expression to be used in the forecast."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ForecastComparisonExpression>,
}
impl ForecastFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Forecast properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ForecastProperties {
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of columns"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub columns: Vec<ForecastColumn>,
    #[doc = "Array of rows"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rows: Vec<Vec<serde_json::Value>>,
}
impl ForecastProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of forecast. It contains all columns listed under groupings and aggregation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ForecastResult {
    #[serde(flatten)]
    pub cost_management_resource: CostManagementResource,
    #[doc = "Forecast properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ForecastProperties>,
}
impl ForecastResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The forecasted cost which is being tracked for a budget.\n\n Supported for CategoryType(s): Cost."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ForecastSpend {
    #[doc = "The forecasted cost for the total time period which is being tracked by the budget. This value is only provided if the budget contains a forecast alert type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[doc = "The unit of measure for the budget amount."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
impl ForecastSpend {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Has time period for pulling data for the forecast."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForecastTimePeriod {
    #[doc = "The start date to pull data from."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub from: time::OffsetDateTime,
    #[doc = "The end date to pull data to."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub to: time::OffsetDateTime,
}
impl ForecastTimePeriod {
    pub fn new(from: time::OffsetDateTime, to: time::OffsetDateTime) -> Self {
        Self { from, to }
    }
}
#[doc = "The time frame for pulling data for the forecast."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ForecastTimeframe")]
pub enum ForecastTimeframe {
    Custom,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ForecastTimeframe {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ForecastTimeframe {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ForecastTimeframe {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Custom => serializer.serialize_unit_variant("ForecastTimeframe", 0u32, "Custom"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The type of the forecast."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ForecastType")]
pub enum ForecastType {
    Usage,
    ActualCost,
    AmortizedCost,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ForecastType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ForecastType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ForecastType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Usage => serializer.serialize_unit_variant("ForecastType", 0u32, "Usage"),
            Self::ActualCost => serializer.serialize_unit_variant("ForecastType", 1u32, "ActualCost"),
            Self::AmortizedCost => serializer.serialize_unit_variant("ForecastType", 2u32, "AmortizedCost"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Error response indicates that the service is not able to process the incoming request. The reason is provided in the error message. \n\nSome Error responses: \n\n * 400 Bad Request - Invalid Request Payload. Request payload provided is not in a json format or had an invalid member not accepted in the request payload. \n\n * 400 Bad Request - Invalid request payload:  can only have either timePeriod or invoiceId or billingPeriod. API only allows data to be pulled for either timePeriod or invoiceId or billingPeriod. Customer should provide only one of these parameters. \n\n * 400 Bad Request - Start date must be after . API only allows data to be pulled no older than 13 months from now. \n\n * 400 Bad Request - The maximum allowed date range is 1 months. API only allows data to be pulled for 1 month or less. \n\n * 429 TooManyRequests - Request is throttled. Retry after waiting for the time specified in the \"retry-after\" header. \n\n * 503 ServiceUnavailable - Service is temporarily unavailable. Retry after waiting for the time specified in the \"Retry-After\" header."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenerateCostDetailsReportErrorResponse {
    #[doc = "The details of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}
impl GenerateCostDetailsReportErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The definition of a cost detailed report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenerateCostDetailsReportRequestDefinition {
    #[doc = "The type of the detailed report. By default ActualCost is provided"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric: Option<generate_cost_details_report_request_definition::Metric>,
    #[doc = "The start and end date for pulling data for the cost detailed report. API only allows data to be pulled for 1 month or less and no older than 13 months."]
    #[serde(rename = "timePeriod", default, skip_serializing_if = "Option::is_none")]
    pub time_period: Option<CostDetailsTimePeriod>,
    #[doc = "This parameter can be used only by Enterprise Agreement customers. Use the YearMonth(e.g. 202008) format. This parameter cannot be used alongside either the invoiceId or timePeriod parameters. If a timePeriod, invoiceId or billingPeriod parameter is not provided in the request body the API will return the current month's cost."]
    #[serde(rename = "billingPeriod", default, skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<String>,
    #[doc = "This parameter can only be used by Microsoft Customer Agreement customers. Additionally, it can only be used at the Billing Profile or Customer scope. This parameter cannot be used alongside either the billingPeriod or timePeriod parameters. If a timePeriod, invoiceId or billingPeriod parameter is not provided in the request body the API will return the current month's cost."]
    #[serde(rename = "invoiceId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
}
impl GenerateCostDetailsReportRequestDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod generate_cost_details_report_request_definition {
    use super::*;
    #[doc = "The type of the detailed report. By default ActualCost is provided"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Metric")]
    pub enum Metric {
        ActualCost,
        AmortizedCost,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Metric {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Metric {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Metric {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ActualCost => serializer.serialize_unit_variant("Metric", 0u32, "ActualCost"),
                Self::AmortizedCost => serializer.serialize_unit_variant("Metric", 1u32, "AmortizedCost"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The definition of a cost detailed report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenerateDetailedCostReportDefinition {
    #[doc = "The type of the detailed report. By default ActualCost is provided"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric: Option<generate_detailed_cost_report_definition::Metric>,
    #[doc = "The start and end date for pulling data for the cost detailed report."]
    #[serde(rename = "timePeriod", default, skip_serializing_if = "Option::is_none")]
    pub time_period: Option<GenerateDetailedCostReportTimePeriod>,
    #[doc = "Billing period in YearMonth(e.g. 202008) format. Only for legacy enterprise customers can use this. Can only have one of either timePeriod or invoiceId or billingPeriod parameters. If none provided current month cost is provided."]
    #[serde(rename = "billingPeriod", default, skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<String>,
    #[doc = "Invoice ID for Pay-as-you-go and Microsoft Customer Agreement scopes. Can only have one of either timePeriod or invoiceId or billingPeriod parameters. If none provided current month cost is provided."]
    #[serde(rename = "invoiceId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    #[doc = "Customer ID for Microsoft Customer Agreement scopes (Invoice Id is also required for this)."]
    #[serde(rename = "customerId", default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
}
impl GenerateDetailedCostReportDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod generate_detailed_cost_report_definition {
    use super::*;
    #[doc = "The type of the detailed report. By default ActualCost is provided"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Metric")]
    pub enum Metric {
        ActualCost,
        AmortizedCost,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Metric {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Metric {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Metric {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ActualCost => serializer.serialize_unit_variant("Metric", 0u32, "ActualCost"),
                Self::AmortizedCost => serializer.serialize_unit_variant("Metric", 1u32, "AmortizedCost"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Error response indicates that the service is not able to process the incoming request. The reason is provided in the error message. \n\nSome Error responses: \n\n * 413 Request Entity Too Large - Request is throttled. The amount of data required to fulfill the request exceeds the maximum size permitted of 2Gb. Please utilize our Exports feature instead. \n\n * 429 TooManyRequests - Request is throttled. Retry after waiting for the time specified in the \"x-ms-ratelimit-microsoft.consumption-retry-after\" header. \n\n * 503 ServiceUnavailable - Service is temporarily unavailable. Retry after waiting for the time specified in the \"Retry-After\" header."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenerateDetailedCostReportErrorResponse {
    #[doc = "The details of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}
impl GenerateDetailedCostReportErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of the long running operation for cost detailed report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenerateDetailedCostReportOperationResult {
    #[doc = "The ARM resource id of the long running operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the long running operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the long running operation."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The URL to download the generated report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DownloadUrl>,
}
impl GenerateDetailedCostReportOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of the long running operation for cost detailed report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenerateDetailedCostReportOperationStatuses {
    #[doc = "The ID of the long running operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the long running operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The status of the long running operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[doc = "The startTime of the operation."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The endTime of the operation."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The type of the long running operation."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The details of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
    #[doc = "The URL to download the generated report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DownloadUrl>,
}
impl GenerateDetailedCostReportOperationStatuses {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The start and end date for pulling data for the cost detailed report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenerateDetailedCostReportTimePeriod {
    #[doc = "The start date to pull data from. example format 2020-03-15"]
    pub start: String,
    #[doc = "The end date to pull data to. example format 2020-03-15"]
    pub end: String,
}
impl GenerateDetailedCostReportTimePeriod {
    pub fn new(start: String, end: String) -> Self {
        Self { start, end }
    }
}
#[doc = "Grain which corresponds to value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Grain")]
pub enum Grain {
    Hourly,
    Daily,
    Monthly,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Grain {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Grain {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Grain {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Hourly => serializer.serialize_unit_variant("Grain", 0u32, "Hourly"),
            Self::Daily => serializer.serialize_unit_variant("Grain", 1u32, "Daily"),
            Self::Monthly => serializer.serialize_unit_variant("Grain", 2u32, "Monthly"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The granularity of rows in the forecast."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "GranularityType")]
pub enum GranularityType {
    Daily,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for GranularityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for GranularityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for GranularityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Daily => serializer.serialize_unit_variant("GranularityType", 0u32, "Daily"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Included Quantity utilization summary resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IncludedQuantityUtilizationSummary {
    #[serde(flatten)]
    pub benefit_utilization_summary: BenefitUtilizationSummary,
    #[doc = "Included Quantity utilization summary properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IncludedQuantityUtilizationSummaryProperties>,
}
impl IncludedQuantityUtilizationSummary {
    pub fn new(benefit_utilization_summary: BenefitUtilizationSummary) -> Self {
        Self {
            benefit_utilization_summary,
            properties: None,
        }
    }
}
#[doc = "Included Quantity utilization summary properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IncludedQuantityUtilizationSummaryProperties {
    #[serde(flatten)]
    pub benefit_utilization_summary_properties: BenefitUtilizationSummaryProperties,
    #[doc = "This is the utilized percentage for the benefit ID."]
    #[serde(rename = "utilizationPercentage", default, skip_serializing_if = "Option::is_none")]
    pub utilization_percentage: Option<f64>,
}
impl IncludedQuantityUtilizationSummaryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Each KPI must contain a 'type' and 'enabled' key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KpiProperties {
    #[doc = "KPI type (Forecast, Budget)."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<kpi_properties::Type>,
    #[doc = "ID of resource related to metric (budget)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "show the KPI in the UI?"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl KpiProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod kpi_properties {
    use super::*;
    #[doc = "KPI type (Forecast, Budget)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Forecast,
        Budget,
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
                Self::Forecast => serializer.serialize_unit_variant("Type", 0u32, "Forecast"),
                Self::Budget => serializer.serialize_unit_variant("Type", 1u32, "Budget"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type Language = String;
#[doc = "The number of days used to look back."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LookBackPeriod")]
pub enum LookBackPeriod {
    Last7Days,
    Last30Days,
    Last60Days,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LookBackPeriod {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LookBackPeriod {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LookBackPeriod {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Last7Days => serializer.serialize_unit_variant("LookBackPeriod", 0u32, "Last7Days"),
            Self::Last30Days => serializer.serialize_unit_variant("LookBackPeriod", 1u32, "Last30Days"),
            Self::Last60Days => serializer.serialize_unit_variant("LookBackPeriod", 2u32, "Last60Days"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The notification associated with a budget.\n\n Supported for CategoryType(s): Cost, ReservationUtilization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    #[doc = "The notification is enabled or not.\n\n Supported for CategoryType(s): Cost, ReservationUtilization."]
    pub enabled: bool,
    #[doc = "The comparison operator.\n\n Supported for CategoryType(s): Cost, ReservationUtilization.\n\n Supported operators for **CategoryType: Cost**\n- GreaterThan\n- GreaterThanOrEqualTo\n\n Supported operators for **CategoryType: ReservationUtilization**\n- LessThan"]
    pub operator: notification::Operator,
    #[doc = "Threshold value associated with a notification. It is always percent with a maximum of 2 decimal places.\n\n Supported for CategoryType(s): Cost, ReservationUtilization.\n\n **CategoryType: Cost** - Must be between 0 and 1000. Notification is sent when the cost exceeded the threshold.\n\n **CategoryType: ReservationUtilization** - Must be between 0 and 100. Notification is sent when a reservation has a utilization percentage below the threshold."]
    pub threshold: f64,
    #[doc = "Frequency of a notification. Represents how long the notification will be silent after triggering an alert for a threshold breach. If not specified, the frequency will be set by default based on the timeGrain (Weekly when timeGrain: Last7Days, Monthly when timeGrain: Last30Days).\n\n Supported for CategoryType(s): ReservationUtilization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<notification::Frequency>,
    #[doc = "Email addresses to send the notification to when the threshold is breached. Must have at least one contact email or contact group specified at the Subscription or Resource Group scopes. All other scopes must have at least one contact email specified.\n\n Supported for CategoryType(s): Cost, ReservationUtilization."]
    #[serde(rename = "contactEmails")]
    pub contact_emails: Vec<String>,
    #[doc = "Subscription or Resource Group scopes only. Contact roles to send the notification to when the threshold is breached.\n\n Supported for CategoryType(s): Cost."]
    #[serde(
        rename = "contactRoles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub contact_roles: Vec<String>,
    #[doc = "Subscription or Resource Group scopes only. Action groups to send the notification to when the threshold is exceeded. Must be provided as a fully qualified Azure resource id.\n\n Supported for CategoryType(s): Cost."]
    #[serde(
        rename = "contactGroups",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub contact_groups: Vec<String>,
    #[doc = "The type of threshold.\n\n Supported for CategoryType(s): Cost."]
    #[serde(rename = "thresholdType", default, skip_serializing_if = "Option::is_none")]
    pub threshold_type: Option<notification::ThresholdType>,
    #[doc = "Language in which the recipient will receive the notification, \n\n Supported for CategoryType(s): Cost, ReservationUtilization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<notification::Locale>,
}
impl Notification {
    pub fn new(enabled: bool, operator: notification::Operator, threshold: f64, contact_emails: Vec<String>) -> Self {
        Self {
            enabled,
            operator,
            threshold,
            frequency: None,
            contact_emails,
            contact_roles: Vec::new(),
            contact_groups: Vec::new(),
            threshold_type: None,
            locale: None,
        }
    }
}
pub mod notification {
    use super::*;
    #[doc = "The comparison operator.\n\n Supported for CategoryType(s): Cost, ReservationUtilization.\n\n Supported operators for **CategoryType: Cost**\n- GreaterThan\n- GreaterThanOrEqualTo\n\n Supported operators for **CategoryType: ReservationUtilization**\n- LessThan"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        EqualTo,
        GreaterThan,
        GreaterThanOrEqualTo,
        LessThan,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::EqualTo => serializer.serialize_unit_variant("Operator", 0u32, "EqualTo"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 1u32, "GreaterThan"),
                Self::GreaterThanOrEqualTo => serializer.serialize_unit_variant("Operator", 2u32, "GreaterThanOrEqualTo"),
                Self::LessThan => serializer.serialize_unit_variant("Operator", 3u32, "LessThan"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Frequency of a notification. Represents how long the notification will be silent after triggering an alert for a threshold breach. If not specified, the frequency will be set by default based on the timeGrain (Weekly when timeGrain: Last7Days, Monthly when timeGrain: Last30Days).\n\n Supported for CategoryType(s): ReservationUtilization."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Frequency")]
    pub enum Frequency {
        Daily,
        Weekly,
        Monthly,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Frequency {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Frequency {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Frequency {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Daily => serializer.serialize_unit_variant("Frequency", 0u32, "Daily"),
                Self::Weekly => serializer.serialize_unit_variant("Frequency", 1u32, "Weekly"),
                Self::Monthly => serializer.serialize_unit_variant("Frequency", 2u32, "Monthly"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of threshold.\n\n Supported for CategoryType(s): Cost."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ThresholdType")]
    pub enum ThresholdType {
        Actual,
        Forecasted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ThresholdType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ThresholdType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ThresholdType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Actual => serializer.serialize_unit_variant("ThresholdType", 0u32, "Actual"),
                Self::Forecasted => serializer.serialize_unit_variant("ThresholdType", 1u32, "Forecasted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for ThresholdType {
        fn default() -> Self {
            Self::Actual
        }
    }
    #[doc = "Language in which the recipient will receive the notification, \n\n Supported for CategoryType(s): Cost, ReservationUtilization."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Locale")]
    pub enum Locale {
        #[serde(rename = "en-us")]
        EnUs,
        #[serde(rename = "ja-jp")]
        JaJp,
        #[serde(rename = "zh-cn")]
        ZhCn,
        #[serde(rename = "de-de")]
        DeDe,
        #[serde(rename = "es-es")]
        EsEs,
        #[serde(rename = "fr-fr")]
        FrFr,
        #[serde(rename = "it-it")]
        ItIt,
        #[serde(rename = "ko-kr")]
        KoKr,
        #[serde(rename = "pt-br")]
        PtBr,
        #[serde(rename = "ru-ru")]
        RuRu,
        #[serde(rename = "zh-tw")]
        ZhTw,
        #[serde(rename = "cs-cz")]
        CsCz,
        #[serde(rename = "pl-pl")]
        PlPl,
        #[serde(rename = "tr-tr")]
        TrTr,
        #[serde(rename = "da-dk")]
        DaDk,
        #[serde(rename = "en-gb")]
        EnGb,
        #[serde(rename = "hu-hu")]
        HuHu,
        #[serde(rename = "nb-no")]
        NbNo,
        #[serde(rename = "nl-nl")]
        NlNl,
        #[serde(rename = "pt-pt")]
        PtPt,
        #[serde(rename = "sv-se")]
        SvSe,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Locale {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Locale {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Locale {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::EnUs => serializer.serialize_unit_variant("Locale", 0u32, "en-us"),
                Self::JaJp => serializer.serialize_unit_variant("Locale", 1u32, "ja-jp"),
                Self::ZhCn => serializer.serialize_unit_variant("Locale", 2u32, "zh-cn"),
                Self::DeDe => serializer.serialize_unit_variant("Locale", 3u32, "de-de"),
                Self::EsEs => serializer.serialize_unit_variant("Locale", 4u32, "es-es"),
                Self::FrFr => serializer.serialize_unit_variant("Locale", 5u32, "fr-fr"),
                Self::ItIt => serializer.serialize_unit_variant("Locale", 6u32, "it-it"),
                Self::KoKr => serializer.serialize_unit_variant("Locale", 7u32, "ko-kr"),
                Self::PtBr => serializer.serialize_unit_variant("Locale", 8u32, "pt-br"),
                Self::RuRu => serializer.serialize_unit_variant("Locale", 9u32, "ru-ru"),
                Self::ZhTw => serializer.serialize_unit_variant("Locale", 10u32, "zh-tw"),
                Self::CsCz => serializer.serialize_unit_variant("Locale", 11u32, "cs-cz"),
                Self::PlPl => serializer.serialize_unit_variant("Locale", 12u32, "pl-pl"),
                Self::TrTr => serializer.serialize_unit_variant("Locale", 13u32, "tr-tr"),
                Self::DaDk => serializer.serialize_unit_variant("Locale", 14u32, "da-dk"),
                Self::EnGb => serializer.serialize_unit_variant("Locale", 15u32, "en-gb"),
                Self::HuHu => serializer.serialize_unit_variant("Locale", 16u32, "hu-hu"),
                Self::NbNo => serializer.serialize_unit_variant("Locale", 17u32, "nb-no"),
                Self::NlNl => serializer.serialize_unit_variant("Locale", 18u32, "nl-nl"),
                Self::PtPt => serializer.serialize_unit_variant("Locale", 19u32, "pt-pt"),
                Self::SvSe => serializer.serialize_unit_variant("Locale", 20u32, "sv-se"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type NotificationEmail = String;
#[doc = "The properties of the scheduled action notification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationProperties {
    #[doc = "Array of email addresses."]
    pub to: Vec<String>,
    #[doc = "Language in which the email is localized, for ex: 'Fr'. When the specified language is not supported, email will be in english. The supported languages are cs (Czech), da (Danish), de (German), en (English), es (Spanish), fr (French), hu (Hungarian), id (Indonesian), it (Lithuanian), ja (Japanese), ko (Korean), nb (Norwegian Bokmål), nl (Dutch), pl (Polish), pt (Portuguese), ru (Russian), sv (Swedish), tr (Turkish), zh (Chinese) "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    #[doc = "Optional message to be added in the email. Length is limited to 250 characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Language for formatting the date/time and currency values in the email. For ex: 'fr'. Defaulted to English, when the specified regional format is not supported. "]
    #[serde(rename = "regionalFormat", default, skip_serializing_if = "Option::is_none")]
    pub regional_format: Option<RegionalFormat>,
    #[doc = "Subject of the email. Length is limited to 70 characters."]
    pub subject: String,
}
impl NotificationProperties {
    pub fn new(to: Vec<String>, subject: String) -> Self {
        Self {
            to,
            language: None,
            message: None,
            regional_format: None,
            subject,
        }
    }
}
#[doc = "Details of a REST API operation, returned from the Resource Provider Operations API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation, as per Resource-Based Access Control (RBAC). Examples: \"Microsoft.Compute/virtualMachines/write\", \"Microsoft.Compute/virtualMachines/capture/action\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Whether the operation applies to data-plane. This is \"true\" for data-plane operations and \"false\" for ARM/control-plane operations."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Localized display information for this particular operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operation::ActionType>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Localized display information for this particular operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The localized friendly form of the resource provider name, e.g. \"Microsoft Monitoring Insights\" or \"Microsoft Compute\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The localized friendly name of the resource type related to this operation. E.g. \"Virtual Machines\" or \"Job Schedule Collections\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The concise, localized friendly name for the operation; suitable for dropdowns. E.g. \"Create or Update Virtual Machine\", \"Restart Virtual Machine\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The short, localized friendly description of the operation; suitable for tool tips and detailed views."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Origin")]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "user,system")]
        UserSystem,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Origin {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Origin {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Origin {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("Origin", 0u32, "user"),
                Self::System => serializer.serialize_unit_variant("Origin", 1u32, "system"),
                Self::UserSystem => serializer.serialize_unit_variant("Origin", 2u32, "user,system"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActionType")]
    pub enum ActionType {
        Internal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Internal => serializer.serialize_unit_variant("ActionType", 0u32, "Internal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Result of listing cost management operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of cost management operations supported by the Microsoft.CostManagement resource provider."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CostManagementOperation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of the long running operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatus {
    #[doc = "The status of the long running operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<operation_status::Status>,
    #[doc = "The URL to download the generated report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReportUrl>,
}
impl OperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation_status {
    use super::*;
    #[doc = "The status of the long running operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Running,
        Completed,
        Failed,
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
                Self::Running => serializer.serialize_unit_variant("Status", 0u32, "Running"),
                Self::Completed => serializer.serialize_unit_variant("Status", 1u32, "Completed"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Each pivot must contain a 'type' and 'name'."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PivotProperties {
    #[doc = "Data type to show in view."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<pivot_properties::Type>,
    #[doc = "Data field to show in view."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl PivotProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod pivot_properties {
    use super::*;
    #[doc = "Data type to show in view."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Dimension,
        TagKey,
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
                Self::Dimension => serializer.serialize_unit_variant("Type", 0u32, "Dimension"),
                Self::TagKey => serializer.serialize_unit_variant("Type", 1u32, "TagKey"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The resource model definition for a Azure Resource Manager proxy resource. It will not have tags and a location"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The aggregation expression to be used in the query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryAggregation {
    #[doc = "The name of the column to aggregate."]
    pub name: String,
    #[doc = "The name of the aggregation function to use."]
    pub function: query_aggregation::Function,
}
impl QueryAggregation {
    pub fn new(name: String, function: query_aggregation::Function) -> Self {
        Self { name, function }
    }
}
pub mod query_aggregation {
    use super::*;
    #[doc = "The name of the aggregation function to use."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Function")]
    pub enum Function {
        Sum,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Function {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Function {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Function {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Sum => serializer.serialize_unit_variant("Function", 0u32, "Sum"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "QueryColumn properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryColumn {
    #[doc = "The name of column."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of column."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl QueryColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of the column in the export."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "QueryColumnType")]
pub enum QueryColumnType {
    TagKey,
    Dimension,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for QueryColumnType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for QueryColumnType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for QueryColumnType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::TagKey => serializer.serialize_unit_variant("QueryColumnType", 0u32, "TagKey"),
            Self::Dimension => serializer.serialize_unit_variant("QueryColumnType", 1u32, "Dimension"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The comparison expression to be used in the query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryComparisonExpression {
    #[doc = "The name of the column to use in comparison."]
    pub name: String,
    #[doc = "The operator to use for comparison."]
    pub operator: query_comparison_expression::Operator,
    #[doc = "Array of values to use for comparison"]
    pub values: Vec<String>,
}
impl QueryComparisonExpression {
    pub fn new(name: String, operator: query_comparison_expression::Operator, values: Vec<String>) -> Self {
        Self { name, operator, values }
    }
}
pub mod query_comparison_expression {
    use super::*;
    #[doc = "The operator to use for comparison."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        In,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::In => serializer.serialize_unit_variant("Operator", 0u32, "In"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The definition of data present in the query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryDataset {
    #[doc = "The granularity of rows in the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granularity: Option<query_dataset::Granularity>,
    #[doc = "The configuration of dataset in the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<QueryDatasetConfiguration>,
    #[doc = "Dictionary of aggregation expression to use in the query. The key of each item in the dictionary is the alias for the aggregated column. Query can have up to 2 aggregation clauses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<serde_json::Value>,
    #[doc = "Array of group by expression to use in the query. Query can have up to 2 group by clauses."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub grouping: Vec<QueryGrouping>,
    #[doc = "The filter expression to be used in the export."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<QueryFilter>,
}
impl QueryDataset {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod query_dataset {
    use super::*;
    #[doc = "The granularity of rows in the query."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Granularity")]
    pub enum Granularity {
        Daily,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Granularity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Granularity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Granularity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Daily => serializer.serialize_unit_variant("Granularity", 0u32, "Daily"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The configuration of dataset in the query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryDatasetConfiguration {
    #[doc = "Array of column names to be included in the query. Any valid query column name is allowed. If not provided, then query includes all columns."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub columns: Vec<String>,
}
impl QueryDatasetConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The definition of a query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryDefinition {
    #[doc = "The type of the query."]
    #[serde(rename = "type")]
    pub type_: query_definition::Type,
    #[doc = "The time frame for pulling data for the query. If custom, then a specific time period must be provided."]
    pub timeframe: query_definition::Timeframe,
    #[doc = "The start and end date for pulling data for the query."]
    #[serde(rename = "timePeriod", default, skip_serializing_if = "Option::is_none")]
    pub time_period: Option<QueryTimePeriod>,
    #[doc = "The definition of data present in the query."]
    pub dataset: QueryDataset,
}
impl QueryDefinition {
    pub fn new(type_: query_definition::Type, timeframe: query_definition::Timeframe, dataset: QueryDataset) -> Self {
        Self {
            type_,
            timeframe,
            time_period: None,
            dataset,
        }
    }
}
pub mod query_definition {
    use super::*;
    #[doc = "The type of the query."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Usage,
        ActualCost,
        AmortizedCost,
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
                Self::Usage => serializer.serialize_unit_variant("Type", 0u32, "Usage"),
                Self::ActualCost => serializer.serialize_unit_variant("Type", 1u32, "ActualCost"),
                Self::AmortizedCost => serializer.serialize_unit_variant("Type", 2u32, "AmortizedCost"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The time frame for pulling data for the query. If custom, then a specific time period must be provided."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Timeframe")]
    pub enum Timeframe {
        MonthToDate,
        BillingMonthToDate,
        TheLastMonth,
        TheLastBillingMonth,
        WeekToDate,
        Custom,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Timeframe {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Timeframe {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Timeframe {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MonthToDate => serializer.serialize_unit_variant("Timeframe", 0u32, "MonthToDate"),
                Self::BillingMonthToDate => serializer.serialize_unit_variant("Timeframe", 1u32, "BillingMonthToDate"),
                Self::TheLastMonth => serializer.serialize_unit_variant("Timeframe", 2u32, "TheLastMonth"),
                Self::TheLastBillingMonth => serializer.serialize_unit_variant("Timeframe", 3u32, "TheLastBillingMonth"),
                Self::WeekToDate => serializer.serialize_unit_variant("Timeframe", 4u32, "WeekToDate"),
                Self::Custom => serializer.serialize_unit_variant("Timeframe", 5u32, "Custom"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The filter expression to be used in the export."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryFilter {
    #[doc = "The logical \"AND\" expression. Must have at least 2 items."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub and: Vec<QueryFilter>,
    #[doc = "The logical \"OR\" expression. Must have at least 2 items."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub or: Vec<QueryFilter>,
    #[doc = "The comparison expression to be used in the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<QueryComparisonExpression>,
    #[doc = "The comparison expression to be used in the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<QueryComparisonExpression>,
}
impl QueryFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The group by expression to be used in the query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryGrouping {
    #[doc = "The type of the column in the export."]
    #[serde(rename = "type")]
    pub type_: QueryColumnType,
    #[doc = "The name of the column to group."]
    pub name: String,
}
impl QueryGrouping {
    pub fn new(type_: QueryColumnType, name: String) -> Self {
        Self { type_, name }
    }
}
#[doc = "Query properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryProperties {
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of columns"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub columns: Vec<QueryColumn>,
    #[doc = "Array of rows"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rows: Vec<Vec<serde_json::Value>>,
}
impl QueryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of query. It contains all columns listed under groupings and aggregation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryResult {
    #[serde(flatten)]
    pub cost_management_resource: CostManagementResource,
    #[doc = "Query properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QueryProperties>,
}
impl QueryResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The start and end date for pulling data for the query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryTimePeriod {
    #[doc = "The start date to pull data from."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub from: time::OffsetDateTime,
    #[doc = "The end date to pull data to."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub to: time::OffsetDateTime,
}
impl QueryTimePeriod {
    pub fn new(from: time::OffsetDateTime, to: time::OffsetDateTime) -> Self {
        Self { from, to }
    }
}
#[doc = "Kind of the recommendation scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RecommendationScope")]
pub enum RecommendationScope {
    Single,
    Shared,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RecommendationScope {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RecommendationScope {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RecommendationScope {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Single => serializer.serialize_unit_variant("RecommendationScope", 0u32, "Single"),
            Self::Shared => serializer.serialize_unit_variant("RecommendationScope", 1u32, "Shared"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type RegionalFormat = String;
#[doc = "The aggregation expression to be used in the report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigAggregation {
    #[doc = "The name of the column to aggregate."]
    pub name: String,
    #[doc = "The name of the aggregation function to use."]
    pub function: report_config_aggregation::Function,
}
impl ReportConfigAggregation {
    pub fn new(name: String, function: report_config_aggregation::Function) -> Self {
        Self { name, function }
    }
}
pub mod report_config_aggregation {
    use super::*;
    #[doc = "The name of the aggregation function to use."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Function")]
    pub enum Function {
        Sum,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Function {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Function {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Function {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Sum => serializer.serialize_unit_variant("Function", 0u32, "Sum"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The type of the column in the report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReportConfigColumnType")]
pub enum ReportConfigColumnType {
    TagKey,
    Dimension,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReportConfigColumnType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReportConfigColumnType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReportConfigColumnType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::TagKey => serializer.serialize_unit_variant("ReportConfigColumnType", 0u32, "TagKey"),
            Self::Dimension => serializer.serialize_unit_variant("ReportConfigColumnType", 1u32, "Dimension"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The comparison expression to be used in the report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigComparisonExpression {
    #[doc = "The name of the column to use in comparison."]
    pub name: String,
    #[doc = "The operator to use for comparison."]
    pub operator: report_config_comparison_expression::Operator,
    #[doc = "Array of values to use for comparison"]
    pub values: Vec<String>,
}
impl ReportConfigComparisonExpression {
    pub fn new(name: String, operator: report_config_comparison_expression::Operator, values: Vec<String>) -> Self {
        Self { name, operator, values }
    }
}
pub mod report_config_comparison_expression {
    use super::*;
    #[doc = "The operator to use for comparison."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        In,
        Contains,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::In => serializer.serialize_unit_variant("Operator", 0u32, "In"),
                Self::Contains => serializer.serialize_unit_variant("Operator", 1u32, "Contains"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The definition of data present in the report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportConfigDataset {
    #[doc = "The granularity of rows in the report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granularity: Option<report_config_dataset::Granularity>,
    #[doc = "The configuration of dataset in the report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ReportConfigDatasetConfiguration>,
    #[doc = "Dictionary of aggregation expression to use in the report. The key of each item in the dictionary is the alias for the aggregated column. Report can have up to 2 aggregation clauses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<serde_json::Value>,
    #[doc = "Array of group by expression to use in the report. Report can have up to 2 group by clauses."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub grouping: Vec<ReportConfigGrouping>,
    #[doc = "Array of order by expression to use in the report."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sorting: Vec<ReportConfigSorting>,
    #[doc = "The filter expression to be used in the report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<ReportConfigFilter>,
}
impl ReportConfigDataset {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod report_config_dataset {
    use super::*;
    #[doc = "The granularity of rows in the report."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Granularity")]
    pub enum Granularity {
        Daily,
        Monthly,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Granularity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Granularity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Granularity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Daily => serializer.serialize_unit_variant("Granularity", 0u32, "Daily"),
                Self::Monthly => serializer.serialize_unit_variant("Granularity", 1u32, "Monthly"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The configuration of dataset in the report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportConfigDatasetConfiguration {
    #[doc = "Array of column names to be included in the report. Any valid report column name is allowed. If not provided, then report includes all columns."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub columns: Vec<String>,
}
impl ReportConfigDatasetConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The definition of a report config."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigDefinition {
    #[doc = "The type of the report. Usage represents actual usage, forecast represents forecasted data and UsageAndForecast represents both usage and forecasted data. Actual usage and forecasted data can be differentiated based on dates."]
    #[serde(rename = "type")]
    pub type_: report_config_definition::Type,
    #[doc = "The time frame for pulling data for the report. If custom, then a specific time period must be provided."]
    pub timeframe: report_config_definition::Timeframe,
    #[doc = "The start and end date for pulling data for the report."]
    #[serde(rename = "timePeriod", default, skip_serializing_if = "Option::is_none")]
    pub time_period: Option<ReportConfigTimePeriod>,
    #[doc = "The definition of data present in the report."]
    #[serde(rename = "dataSet", default, skip_serializing_if = "Option::is_none")]
    pub data_set: Option<ReportConfigDataset>,
    #[doc = "If true, report includes monetary commitment."]
    #[serde(rename = "includeMonetaryCommitment", default, skip_serializing_if = "Option::is_none")]
    pub include_monetary_commitment: Option<bool>,
}
impl ReportConfigDefinition {
    pub fn new(type_: report_config_definition::Type, timeframe: report_config_definition::Timeframe) -> Self {
        Self {
            type_,
            timeframe,
            time_period: None,
            data_set: None,
            include_monetary_commitment: None,
        }
    }
}
pub mod report_config_definition {
    use super::*;
    #[doc = "The type of the report. Usage represents actual usage, forecast represents forecasted data and UsageAndForecast represents both usage and forecasted data. Actual usage and forecasted data can be differentiated based on dates."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Usage,
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
                Self::Usage => serializer.serialize_unit_variant("Type", 0u32, "Usage"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The time frame for pulling data for the report. If custom, then a specific time period must be provided."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Timeframe")]
    pub enum Timeframe {
        WeekToDate,
        MonthToDate,
        YearToDate,
        Custom,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Timeframe {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Timeframe {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Timeframe {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::WeekToDate => serializer.serialize_unit_variant("Timeframe", 0u32, "WeekToDate"),
                Self::MonthToDate => serializer.serialize_unit_variant("Timeframe", 1u32, "MonthToDate"),
                Self::YearToDate => serializer.serialize_unit_variant("Timeframe", 2u32, "YearToDate"),
                Self::Custom => serializer.serialize_unit_variant("Timeframe", 3u32, "Custom"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The filter expression to be used in the report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportConfigFilter {
    #[doc = "The logical \"AND\" expression. Must have at least 2 items."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub and: Vec<ReportConfigFilter>,
    #[doc = "The logical \"OR\" expression. Must have at least 2 items."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub or: Vec<ReportConfigFilter>,
    #[doc = "The comparison expression to be used in the report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<ReportConfigComparisonExpression>,
    #[doc = "The comparison expression to be used in the report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<ReportConfigComparisonExpression>,
}
impl ReportConfigFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The group by expression to be used in the report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigGrouping {
    #[doc = "The type of the column in the report."]
    #[serde(rename = "type")]
    pub type_: ReportConfigColumnType,
    #[doc = "The name of the column to group. This version supports subscription lowest possible grain."]
    pub name: String,
}
impl ReportConfigGrouping {
    pub fn new(type_: ReportConfigColumnType, name: String) -> Self {
        Self { type_, name }
    }
}
#[doc = "The order by expression to be used in the report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigSorting {
    #[doc = "Direction of sort."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<report_config_sorting::Direction>,
    #[doc = "The name of the column to sort."]
    pub name: String,
}
impl ReportConfigSorting {
    pub fn new(name: String) -> Self {
        Self { direction: None, name }
    }
}
pub mod report_config_sorting {
    use super::*;
    #[doc = "Direction of sort."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Direction")]
    pub enum Direction {
        Ascending,
        Descending,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Direction {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Direction {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Direction {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ascending => serializer.serialize_unit_variant("Direction", 0u32, "Ascending"),
                Self::Descending => serializer.serialize_unit_variant("Direction", 1u32, "Descending"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The start and end date for pulling data for the report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportConfigTimePeriod {
    #[doc = "The start date to pull data from."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub from: time::OffsetDateTime,
    #[doc = "The end date to pull data to."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub to: time::OffsetDateTime,
}
impl ReportConfigTimePeriod {
    pub fn new(from: time::OffsetDateTime, to: time::OffsetDateTime) -> Self {
        Self { from, to }
    }
}
#[doc = "The manifest of the report generated by the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportManifest {
    #[doc = "The Manifest version."]
    #[serde(rename = "manifestVersion", default, skip_serializing_if = "Option::is_none")]
    pub manifest_version: Option<String>,
    #[doc = "The data format of the report"]
    #[serde(rename = "dataFormat", default, skip_serializing_if = "Option::is_none")]
    pub data_format: Option<report_manifest::DataFormat>,
    #[doc = "The total number of bytes in all blobs."]
    #[serde(rename = "byteCount", default, skip_serializing_if = "Option::is_none")]
    pub byte_count: Option<i64>,
    #[doc = "The total number of blobs."]
    #[serde(rename = "blobCount", default, skip_serializing_if = "Option::is_none")]
    pub blob_count: Option<i32>,
    #[doc = "Is the data in compressed format."]
    #[serde(rename = "compressData", default, skip_serializing_if = "Option::is_none")]
    pub compress_data: Option<bool>,
    #[doc = "The context of the Cost Details request."]
    #[serde(rename = "requestContext", default, skip_serializing_if = "Option::is_none")]
    pub request_context: Option<RequestContext>,
    #[doc = "List of blob information generated by this operation."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub blobs: Vec<BlobInfo>,
}
impl ReportManifest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod report_manifest {
    use super::*;
    #[doc = "The data format of the report"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataFormat")]
    pub enum DataFormat {
        Csv,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataFormat {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataFormat {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataFormat {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Csv => serializer.serialize_unit_variant("DataFormat", 0u32, "Csv"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The URL to download the generated report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReportUrl {
    #[doc = "The CSV file from the reportUrl blob link consists of reservation usage data with the following schema at daily granularity"]
    #[serde(rename = "reportUrl", default, skip_serializing_if = "Option::is_none")]
    pub report_url: Option<ReservationReportSchema>,
    #[doc = "The time at which report URL becomes invalid."]
    #[serde(rename = "validUntil", default, with = "azure_core::date::rfc3339::option")]
    pub valid_until: Option<time::OffsetDateTime>,
}
impl ReportUrl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The context of the Cost Details request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestContext {
    #[doc = "The request scope of the request."]
    #[serde(rename = "requestScope", default, skip_serializing_if = "Option::is_none")]
    pub request_scope: Option<String>,
    #[doc = "The definition of a cost detailed report."]
    #[serde(rename = "requestBody", default, skip_serializing_if = "Option::is_none")]
    pub request_body: Option<GenerateCostDetailsReportRequestDefinition>,
}
impl RequestContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The CSV file from the reportUrl blob link consists of reservation usage data with the following schema at daily granularity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReservationReportSchema")]
pub enum ReservationReportSchema {
    InstanceFlexibilityGroup,
    InstanceFlexibilityRatio,
    InstanceId,
    Kind,
    ReservationId,
    ReservationOrderId,
    ReservedHours,
    SkuName,
    TotalReservedQuantity,
    UsageDate,
    UsedHours,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReservationReportSchema {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReservationReportSchema {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReservationReportSchema {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::InstanceFlexibilityGroup => {
                serializer.serialize_unit_variant("ReservationReportSchema", 0u32, "InstanceFlexibilityGroup")
            }
            Self::InstanceFlexibilityRatio => {
                serializer.serialize_unit_variant("ReservationReportSchema", 1u32, "InstanceFlexibilityRatio")
            }
            Self::InstanceId => serializer.serialize_unit_variant("ReservationReportSchema", 2u32, "InstanceId"),
            Self::Kind => serializer.serialize_unit_variant("ReservationReportSchema", 3u32, "Kind"),
            Self::ReservationId => serializer.serialize_unit_variant("ReservationReportSchema", 4u32, "ReservationId"),
            Self::ReservationOrderId => serializer.serialize_unit_variant("ReservationReportSchema", 5u32, "ReservationOrderId"),
            Self::ReservedHours => serializer.serialize_unit_variant("ReservationReportSchema", 6u32, "ReservedHours"),
            Self::SkuName => serializer.serialize_unit_variant("ReservationReportSchema", 7u32, "SkuName"),
            Self::TotalReservedQuantity => serializer.serialize_unit_variant("ReservationReportSchema", 8u32, "TotalReservedQuantity"),
            Self::UsageDate => serializer.serialize_unit_variant("ReservationReportSchema", 9u32, "UsageDate"),
            Self::UsedHours => serializer.serialize_unit_variant("ReservationReportSchema", 10u32, "UsedHours"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Savings plan utilization summary resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavingsPlanUtilizationSummary {
    #[serde(flatten)]
    pub benefit_utilization_summary: BenefitUtilizationSummary,
    #[doc = "Savings plan utilization summary properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SavingsPlanUtilizationSummaryProperties>,
}
impl SavingsPlanUtilizationSummary {
    pub fn new(benefit_utilization_summary: BenefitUtilizationSummary) -> Self {
        Self {
            benefit_utilization_summary,
            properties: None,
        }
    }
}
#[doc = "Savings plan utilization summary properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanUtilizationSummaryProperties {
    #[serde(flatten)]
    pub benefit_utilization_summary_properties: BenefitUtilizationSummaryProperties,
    #[doc = "This is the average hourly utilization for each date range that corresponds to given grain (Daily, Monthly). Suppose the API call is for usageDate > 2023-04-01-preview and usageDate < 2022-10-31 at a daily granularity. There will be one record per benefit id for each day. For a single day, the avgUtilizationPercentage value will be equal to the average of the set of values where the set contains 24 utilization percentage entries one for each hour in a specific day."]
    #[serde(rename = "avgUtilizationPercentage", default, skip_serializing_if = "Option::is_none")]
    pub avg_utilization_percentage: Option<f64>,
    #[doc = "This is the minimum hourly utilization for each date range that corresponds to given grain (Daily, Monthly). Suppose the API call is for usageDate > 2023-04-01-preview and usageDate < 2022-10-31 at a daily granularity. There will be one record per benefit id for each day. For a single day, the minUtilizationPercentage value will be equal to the smallest in the set of values where the set contains 24 utilization percentage entries one for each hour in a specific day. If on the day 2022-10-18, the lowest utilization percentage was 10% at hour 4, then the value for the minUtilizationPercentage in the response will be 10%."]
    #[serde(rename = "minUtilizationPercentage", default, skip_serializing_if = "Option::is_none")]
    pub min_utilization_percentage: Option<f64>,
    #[doc = "This is the maximum hourly utilization for each date range that corresponds to given grain (Daily, Monthly). Suppose the API call is for usageDate > 2023-04-01-preview and usageDate < 2022-10-31 at a daily granularity. There will be one record per benefit id for each day. For a single day, the maxUtilizationPercentage value will be equal to the largest in the set of values where the set contains 24 utilization percentage entries one for each hour in a specific day. If on the day 2022-10-18, the largest utilization percentage was 90% at hour 5, then the value for the maxUtilizationPercentage in the response will be 90%."]
    #[serde(rename = "maxUtilizationPercentage", default, skip_serializing_if = "Option::is_none")]
    pub max_utilization_percentage: Option<f64>,
}
impl SavingsPlanUtilizationSummaryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Frequency of the schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScheduleFrequency")]
pub enum ScheduleFrequency {
    Daily,
    Weekly,
    Monthly,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScheduleFrequency {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScheduleFrequency {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScheduleFrequency {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Daily => serializer.serialize_unit_variant("ScheduleFrequency", 0u32, "Daily"),
            Self::Weekly => serializer.serialize_unit_variant("ScheduleFrequency", 1u32, "Weekly"),
            Self::Monthly => serializer.serialize_unit_variant("ScheduleFrequency", 2u32, "Monthly"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The properties of the schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleProperties {
    #[doc = "Frequency of the schedule."]
    pub frequency: ScheduleFrequency,
    #[doc = "UTC time at which cost analysis data will be emailed."]
    #[serde(rename = "hourOfDay", default, skip_serializing_if = "Option::is_none")]
    pub hour_of_day: Option<i32>,
    #[doc = "Day names in english on which cost analysis data will be emailed. This property is applicable when frequency is Weekly or Monthly."]
    #[serde(
        rename = "daysOfWeek",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub days_of_week: Vec<DaysOfWeek>,
    #[doc = "Weeks in which cost analysis data will be emailed. This property is applicable when frequency is Monthly and used in combination with daysOfWeek."]
    #[serde(
        rename = "weeksOfMonth",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub weeks_of_month: Vec<WeeksOfMonth>,
    #[doc = "UTC day on which cost analysis data will be emailed. Must be between 1 and 31. This property is applicable when frequency is Monthly and overrides weeksOfMonth or daysOfWeek."]
    #[serde(rename = "dayOfMonth", default, skip_serializing_if = "Option::is_none")]
    pub day_of_month: Option<i32>,
    #[doc = "The start date and time of the scheduled action (UTC)."]
    #[serde(rename = "startDate", with = "azure_core::date::rfc3339")]
    pub start_date: time::OffsetDateTime,
    #[doc = "The end date and time of the scheduled action (UTC)."]
    #[serde(rename = "endDate", with = "azure_core::date::rfc3339")]
    pub end_date: time::OffsetDateTime,
}
impl ScheduleProperties {
    pub fn new(frequency: ScheduleFrequency, start_date: time::OffsetDateTime, end_date: time::OffsetDateTime) -> Self {
        Self {
            frequency,
            hour_of_day: None,
            days_of_week: Vec::new(),
            weeks_of_month: Vec::new(),
            day_of_month: None,
            start_date,
            end_date,
        }
    }
}
#[doc = "Scheduled action definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduledAction {
    #[serde(flatten)]
    pub scheduled_action_proxy_resource: ScheduledActionProxyResource,
    #[doc = "The properties of the scheduled action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScheduledActionProperties>,
}
impl ScheduledAction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Kind of the scheduled action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScheduledActionKind")]
pub enum ScheduledActionKind {
    Email,
    InsightAlert,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScheduledActionKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScheduledActionKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScheduledActionKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Email => serializer.serialize_unit_variant("ScheduledActionKind", 0u32, "Email"),
            Self::InsightAlert => serializer.serialize_unit_variant("ScheduledActionKind", 1u32, "InsightAlert"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Scheduled actions list result. It contains a list of scheduled actions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduledActionListResult {
    #[doc = "The list of scheduled actions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ScheduledAction>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScheduledActionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ScheduledActionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the scheduled action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledActionProperties {
    #[doc = "Scheduled action name."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Destination of the view data. This is optional. Currently only CSV format is supported."]
    #[serde(rename = "fileDestination", default, skip_serializing_if = "Option::is_none")]
    pub file_destination: Option<FileDestination>,
    #[doc = "The properties of the scheduled action notification."]
    pub notification: NotificationProperties,
    #[doc = "Email address of the point of contact that should get the unsubscribe requests and notification emails."]
    #[serde(rename = "notificationEmail", default, skip_serializing_if = "Option::is_none")]
    pub notification_email: Option<NotificationEmail>,
    #[doc = "The properties of the schedule."]
    pub schedule: ScheduleProperties,
    #[doc = "Cost Management scope like 'subscriptions/{subscriptionId}' for subscription scope, 'subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}' for resourceGroup scope, 'providers/Microsoft.Billing/billingAccounts/{billingAccountId}' for Billing Account scope, 'providers/Microsoft.Billing/billingAccounts/{billingAccountId}/departments/{departmentId}' for Department scope, 'providers/Microsoft.Billing/billingAccounts/{billingAccountId}/enrollmentAccounts/{enrollmentAccountId}' for EnrollmentAccount scope, 'providers/Microsoft.Billing/billingAccounts/{billingAccountId}/billingProfiles/{billingProfileId}' for BillingProfile scope, 'providers/Microsoft.Billing/billingAccounts/{billingAccountId}/invoiceSections/{invoiceSectionId}' for InvoiceSection scope, '/providers/Microsoft.CostManagement/externalBillingAccounts/{externalBillingAccountName}' for ExternalBillingAccount scope, and '/providers/Microsoft.CostManagement/externalSubscriptions/{externalSubscriptionName}' for ExternalSubscription scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "Status of the scheduled action."]
    pub status: ScheduledActionStatus,
    #[doc = "Cost analysis viewId used for scheduled action. For example, '/providers/Microsoft.CostManagement/views/swaggerExample'"]
    #[serde(rename = "viewId")]
    pub view_id: String,
}
impl ScheduledActionProperties {
    pub fn new(
        display_name: String,
        notification: NotificationProperties,
        schedule: ScheduleProperties,
        status: ScheduledActionStatus,
        view_id: String,
    ) -> Self {
        Self {
            display_name,
            file_destination: None,
            notification,
            notification_email: None,
            schedule,
            scope: None,
            status,
            view_id,
        }
    }
}
#[doc = "The Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduledActionProxyResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Resource Etag. For update calls, eTag is mandatory. Fetch the resource's eTag by doing a 'GET' call first and then including the latest eTag as part of the request body or 'If-Match' header while performing the update. For create calls, eTag is not required."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "Kind of the scheduled action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ScheduledActionKind>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ScheduledActionProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status of the scheduled action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScheduledActionStatus")]
pub enum ScheduledActionStatus {
    Enabled,
    Expired,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScheduledActionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScheduledActionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScheduledActionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("ScheduledActionStatus", 0u32, "Enabled"),
            Self::Expired => serializer.serialize_unit_variant("ScheduledActionStatus", 1u32, "Expired"),
            Self::Disabled => serializer.serialize_unit_variant("ScheduledActionStatus", 2u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The status of the long running operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Status {
    #[doc = "The status of the long running operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<status::Status>,
}
impl Status {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod status {
    use super::*;
    #[doc = "The status of the long running operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        InProgress,
        Completed,
        Failed,
        Queued,
        NoDataFound,
        ReadyToDownload,
        TimedOut,
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
                Self::InProgress => serializer.serialize_unit_variant("Status", 0u32, "InProgress"),
                Self::Completed => serializer.serialize_unit_variant("Status", 1u32, "Completed"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::Queued => serializer.serialize_unit_variant("Status", 3u32, "Queued"),
                Self::NoDataFound => serializer.serialize_unit_variant("Status", 4u32, "NoDataFound"),
                Self::ReadyToDownload => serializer.serialize_unit_variant("Status", 5u32, "ReadyToDownload"),
                Self::TimedOut => serializer.serialize_unit_variant("Status", 6u32, "TimedOut"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Grain which corresponds to value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Term")]
pub enum Term {
    #[serde(rename = "P1Y")]
    P1y,
    #[serde(rename = "P3Y")]
    P3y,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Term {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Term {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Term {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::P1y => serializer.serialize_unit_variant("Term", 0u32, "P1Y"),
            Self::P3y => serializer.serialize_unit_variant("Term", 1u32, "P3Y"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "States and configurations of Cost Analysis."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct View {
    #[serde(flatten)]
    pub cost_management_proxy_resource: CostManagementProxyResource,
    #[doc = "The properties of the view."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ViewProperties>,
}
impl View {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of listing views. It contains a list of available views."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ViewListResult {
    #[doc = "The list of views."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<View>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ViewListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ViewListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the view."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ViewProperties {
    #[doc = "User input name of the view. Required."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Cost Management scope to save the view on. This includes 'subscriptions/{subscriptionId}' for subscription scope, 'subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}' for resourceGroup scope, 'providers/Microsoft.Billing/billingAccounts/{billingAccountId}' for Billing Account scope, 'providers/Microsoft.Billing/billingAccounts/{billingAccountId}/departments/{departmentId}' for Department scope, 'providers/Microsoft.Billing/billingAccounts/{billingAccountId}/enrollmentAccounts/{enrollmentAccountId}' for EnrollmentAccount scope, 'providers/Microsoft.Billing/billingAccounts/{billingAccountId}/billingProfiles/{billingProfileId}' for BillingProfile scope, 'providers/Microsoft.Billing/billingAccounts/{billingAccountId}/invoiceSections/{invoiceSectionId}' for InvoiceSection scope, 'providers/Microsoft.Management/managementGroups/{managementGroupId}' for Management Group scope, '/providers/Microsoft.CostManagement/externalBillingAccounts/{externalBillingAccountName}' for ExternalBillingAccount scope, and '/providers/Microsoft.CostManagement/externalSubscriptions/{externalSubscriptionName}' for ExternalSubscription scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "Date the user created this view."]
    #[serde(rename = "createdOn", default, with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Date when the user last modified this view."]
    #[serde(rename = "modifiedOn", default, with = "azure_core::date::rfc3339::option")]
    pub modified_on: Option<time::OffsetDateTime>,
    #[doc = "Date range of the current view."]
    #[serde(rename = "dateRange", default, skip_serializing_if = "Option::is_none")]
    pub date_range: Option<String>,
    #[doc = "Currency of the current view."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The definition of a report config."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<ReportConfigDefinition>,
    #[doc = "Chart type of the main view in Cost Analysis. Required."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chart: Option<view_properties::Chart>,
    #[doc = "Show costs accumulated over time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accumulated: Option<view_properties::Accumulated>,
    #[doc = "Metric to use when displaying costs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric: Option<view_properties::Metric>,
    #[doc = "List of KPIs to show in Cost Analysis UI."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub kpis: Vec<KpiProperties>,
    #[doc = "Configuration of 3 sub-views in the Cost Analysis UI."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pivots: Vec<PivotProperties>,
}
impl ViewProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod view_properties {
    use super::*;
    #[doc = "Chart type of the main view in Cost Analysis. Required."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Chart")]
    pub enum Chart {
        Area,
        Line,
        StackedColumn,
        GroupedColumn,
        Table,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Chart {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Chart {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Chart {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Area => serializer.serialize_unit_variant("Chart", 0u32, "Area"),
                Self::Line => serializer.serialize_unit_variant("Chart", 1u32, "Line"),
                Self::StackedColumn => serializer.serialize_unit_variant("Chart", 2u32, "StackedColumn"),
                Self::GroupedColumn => serializer.serialize_unit_variant("Chart", 3u32, "GroupedColumn"),
                Self::Table => serializer.serialize_unit_variant("Chart", 4u32, "Table"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Show costs accumulated over time."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Accumulated")]
    pub enum Accumulated {
        #[serde(rename = "true")]
        True,
        #[serde(rename = "false")]
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Accumulated {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Accumulated {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Accumulated {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("Accumulated", 0u32, "true"),
                Self::False => serializer.serialize_unit_variant("Accumulated", 1u32, "false"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Metric to use when displaying costs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Metric")]
    pub enum Metric {
        ActualCost,
        AmortizedCost,
        #[serde(rename = "AHUB")]
        Ahub,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Metric {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Metric {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Metric {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ActualCost => serializer.serialize_unit_variant("Metric", 0u32, "ActualCost"),
                Self::AmortizedCost => serializer.serialize_unit_variant("Metric", 1u32, "AmortizedCost"),
                Self::Ahub => serializer.serialize_unit_variant("Metric", 2u32, "AHUB"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Weeks of month."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WeeksOfMonth")]
pub enum WeeksOfMonth {
    First,
    Second,
    Third,
    Fourth,
    Last,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WeeksOfMonth {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WeeksOfMonth {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WeeksOfMonth {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::First => serializer.serialize_unit_variant("WeeksOfMonth", 0u32, "First"),
            Self::Second => serializer.serialize_unit_variant("WeeksOfMonth", 1u32, "Second"),
            Self::Third => serializer.serialize_unit_variant("WeeksOfMonth", 2u32, "Third"),
            Self::Fourth => serializer.serialize_unit_variant("WeeksOfMonth", 3u32, "Fourth"),
            Self::Last => serializer.serialize_unit_variant("WeeksOfMonth", 4u32, "Last"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Benefit recommendation details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AllSavingsBenefitDetails {
    #[doc = "The difference between total cost and benefit cost for the 'totalHours' in the look-back period."]
    #[serde(rename = "overageCost", default, skip_serializing_if = "Option::is_none")]
    pub overage_cost: Option<f64>,
    #[doc = "The estimated cost with benefit for the 'totalHours' in the look-back period. It's equal to (commitmentAmount * totalHours)"]
    #[serde(rename = "benefitCost", default, skip_serializing_if = "Option::is_none")]
    pub benefit_cost: Option<f64>,
    #[doc = "Total cost, which is sum of benefit cost and overage cost."]
    #[serde(rename = "totalCost", default, skip_serializing_if = "Option::is_none")]
    pub total_cost: Option<f64>,
    #[doc = "The amount saved for the 'totalHours' in the look-back period, by purchasing the recommended quantity of the benefit."]
    #[serde(rename = "savingsAmount", default, skip_serializing_if = "Option::is_none")]
    pub savings_amount: Option<f64>,
    #[doc = "The savings in percentage for the 'totalHours' in the look-back period, by purchasing the recommended quantity of benefit."]
    #[serde(rename = "savingsPercentage", default, skip_serializing_if = "Option::is_none")]
    pub savings_percentage: Option<f64>,
    #[doc = "Estimated benefit coverage percentage for the 'totalHours' in the look-back period, with this commitment."]
    #[serde(rename = "coveragePercentage", default, skip_serializing_if = "Option::is_none")]
    pub coverage_percentage: Option<f64>,
    #[doc = "The commitment amount at the commitmentGranularity."]
    #[serde(rename = "commitmentAmount", default, skip_serializing_if = "Option::is_none")]
    pub commitment_amount: Option<f64>,
    #[doc = "Estimated average utilization percentage for the 'totalHours' in the look-back period, with this commitment."]
    #[serde(rename = "averageUtilizationPercentage", default, skip_serializing_if = "Option::is_none")]
    pub average_utilization_percentage: Option<f64>,
    #[doc = "Estimated unused portion of the 'benefitCost'."]
    #[serde(rename = "wastageCost", default, skip_serializing_if = "Option::is_none")]
    pub wastage_cost: Option<f64>,
}
impl AllSavingsBenefitDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of all benefit recommendations with the recommendation details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AllSavingsList {
    #[doc = "The list of benefit recommendations with the recommendation details.."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AllSavingsBenefitDetails>,
    #[doc = "The link (URL) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl AllSavingsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "benefit plan recommendation details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BenefitRecommendationModel {
    #[serde(flatten)]
    pub benefit_resource: BenefitResource,
    #[doc = "The properties of the benefit recommendations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BenefitRecommendationProperties>,
}
impl BenefitRecommendationModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the benefit recommendations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BenefitRecommendationProperties {
    #[doc = "The first usage date used for looking back for computing the recommendations."]
    #[serde(rename = "firstConsumptionDate", default, with = "azure_core::date::rfc3339::option")]
    pub first_consumption_date: Option<time::OffsetDateTime>,
    #[doc = "The last usage date used for looking back for computing the recommendations."]
    #[serde(rename = "lastConsumptionDate", default, with = "azure_core::date::rfc3339::option")]
    pub last_consumption_date: Option<time::OffsetDateTime>,
    #[doc = "The number of days used to look back."]
    #[serde(rename = "lookBackPeriod", default, skip_serializing_if = "Option::is_none")]
    pub look_back_period: Option<LookBackPeriod>,
    #[doc = "The total hours for which the cost is covered. Its equal to number of records in a property 'properties/usage/charges'."]
    #[serde(rename = "totalHours", default, skip_serializing_if = "Option::is_none")]
    pub total_hours: Option<i32>,
    #[doc = "On-demand charges between firstConsumptionDate and lastConsumptionDate that were used for computing benefit recommendations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<RecommendationUsageDetails>,
    #[doc = "ARM SKU name. 'Compute_Savings_Plan' for SavingsPlan."]
    #[serde(rename = "armSkuName", default, skip_serializing_if = "Option::is_none")]
    pub arm_sku_name: Option<String>,
    #[doc = "Grain which corresponds to value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<Term>,
    #[doc = "Grain which corresponds to value."]
    #[serde(rename = "commitmentGranularity", default, skip_serializing_if = "Option::is_none")]
    pub commitment_granularity: Option<Grain>,
    #[doc = "An ISO 4217 currency code identifier for the costs and savings amounts."]
    #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[doc = "The current cost without benefit, corresponds to 'totalHours' in the look-back period."]
    #[serde(rename = "costWithoutBenefit", default, skip_serializing_if = "Option::is_none")]
    pub cost_without_benefit: Option<f64>,
    #[doc = "Benefit recommendation details."]
    #[serde(rename = "recommendationDetails", default, skip_serializing_if = "Option::is_none")]
    pub recommendation_details: Option<AllSavingsBenefitDetails>,
    #[doc = "The list of all benefit recommendations with the recommendation details."]
    #[serde(rename = "allRecommendationDetails", default, skip_serializing_if = "Option::is_none")]
    pub all_recommendation_details: Option<AllSavingsList>,
    #[doc = "Kind of the recommendation scope."]
    pub scope: RecommendationScope,
}
impl BenefitRecommendationProperties {
    pub fn new(scope: RecommendationScope) -> Self {
        Self {
            first_consumption_date: None,
            last_consumption_date: None,
            look_back_period: None,
            total_hours: None,
            usage: None,
            arm_sku_name: None,
            term: None,
            commitment_granularity: None,
            currency_code: None,
            cost_without_benefit: None,
            recommendation_details: None,
            all_recommendation_details: None,
            scope,
        }
    }
}
#[doc = "Result of listing benefit recommendations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BenefitRecommendationsListResult {
    #[doc = "The list of benefit recommendations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BenefitRecommendationModel>,
    #[doc = "The link (URL) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BenefitRecommendationsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BenefitRecommendationsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "On-demand charges between firstConsumptionDate and lastConsumptionDate that were used for computing benefit recommendations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendationUsageDetails {
    #[doc = "Grain which corresponds to value."]
    #[serde(rename = "usageGrain", default, skip_serializing_if = "Option::is_none")]
    pub usage_grain: Option<Grain>,
    #[doc = "On-demand charges for each hour between firstConsumptionDate and lastConsumptionDate that were used for computing benefit recommendations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub charges: Vec<f64>,
}
impl RecommendationUsageDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the benefit recommendation when scope is 'Shared'."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedScopeBenefitRecommendationProperties {
    #[serde(flatten)]
    pub benefit_recommendation_properties: BenefitRecommendationProperties,
}
impl SharedScopeBenefitRecommendationProperties {
    pub fn new(benefit_recommendation_properties: BenefitRecommendationProperties) -> Self {
        Self {
            benefit_recommendation_properties,
        }
    }
}
#[doc = "The properties of the benefit recommendations when scope is 'Single'."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SingleScopeBenefitRecommendationProperties {
    #[serde(flatten)]
    pub benefit_recommendation_properties: BenefitRecommendationProperties,
    #[doc = "The subscription ID that this single scope recommendation is for. Applicable only if recommendation is for 'Single' scope."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The resource group that this single scope recommendation is for. Applicable only if recommendation is for 'Single' scope and 'ResourceGroup' request scope."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
impl SingleScopeBenefitRecommendationProperties {
    pub fn new(benefit_recommendation_properties: BenefitRecommendationProperties) -> Self {
        Self {
            benefit_recommendation_properties,
            subscription_id: None,
            resource_group: None,
        }
    }
}
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_data {
    use super::*;
    #[doc = "The type of identity that created the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreatedByType")]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CreatedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CreatedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CreatedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("CreatedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("CreatedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("CreatedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("CreatedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of identity that last modified the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastModifiedByType")]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastModifiedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastModifiedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastModifiedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("LastModifiedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("LastModifiedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("LastModifiedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("LastModifiedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
