#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An individual alert."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Alert {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertProperties>,
}
impl Alert {
    pub fn new() -> Self {
        Self::default()
    }
}
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
    #[doc = ""]
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
    #[doc = "Result of listing the execution history of an export."]
    #[serde(rename = "runHistory", default, skip_serializing_if = "Option::is_none")]
    pub run_history: Option<ExportExecutionListResult>,
    #[doc = "If the export has an active schedule, provides an estimate of the next execution time."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dimension {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DimensionProperties>,
}
impl Dimension {
    pub fn new() -> Self {
        Self::default()
    }
}
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
    #[doc = "The URL to download the generated report."]
    #[serde(rename = "downloadUrl", default, skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[doc = "The time at which report URL becomes invalid/expires in UTC e.g. 2020-12-08T05:55:59.4394737Z."]
    #[serde(rename = "validTill", default, with = "azure_core::date::rfc3339::option")]
    pub valid_till: Option<time::OffsetDateTime>,
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
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorDetails {
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
#[doc = "An export resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Export {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
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
#[doc = "This represents the blob storage account location where exports of costs will be delivered. There are two ways to configure the destination. The approach recommended for most customers is to specify the resourceId of the storage account. This requires a one-time registration of the account's subscription with the Microsoft.CostManagementExports resource provider in order to give Azure Cost Management services access to the storage. When creating an export in the Azure portal this registration is performed automatically but API users may need to register the subscription explicitly (for more information see https://docs.microsoft.com/en-us/azure/azure-resource-manager/resource-manager-supported-services ). Another way to configure the destination is available ONLY to Partners with a Microsoft Partner Agreement plan who are global admins of their billing account. These Partners, instead of specifying the resourceId of a storage account, can specify the storage account name along with a SAS token for the account. This allows exports of costs to a storage account in any tenant. The SAS token should be created for the blob service with Service/Container/Object resource types and with Read/Write/Delete/List/Add/Create permissions (for more information see https://docs.microsoft.com/en-us/azure/cost-management-billing/costs/export-cost-data-storage-account-sas-key )."]
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
    #[doc = "This represents the blob storage account location where exports of costs will be delivered. There are two ways to configure the destination. The approach recommended for most customers is to specify the resourceId of the storage account. This requires a one-time registration of the account's subscription with the Microsoft.CostManagementExports resource provider in order to give Azure Cost Management services access to the storage. When creating an export in the Azure portal this registration is performed automatically but API users may need to register the subscription explicitly (for more information see https://docs.microsoft.com/en-us/azure/azure-resource-manager/resource-manager-supported-services ). Another way to configure the destination is available ONLY to Partners with a Microsoft Partner Agreement plan who are global admins of their billing account. These Partners, instead of specifying the resourceId of a storage account, can specify the storage account name along with a SAS token for the account. This allows exports of costs to a storage account in any tenant. The SAS token should be created for the blob service with Service/Container/Object resource types and with Read/Write/Delete/List/Add/Create permissions (for more information see https://docs.microsoft.com/en-us/azure/cost-management-billing/costs/export-cost-data-storage-account-sas-key )."]
    pub destination: ExportDeliveryDestination,
}
impl ExportDeliveryInfo {
    pub fn new(destination: ExportDeliveryDestination) -> Self {
        Self { destination }
    }
}
#[doc = "An export execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportExecution {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of the export execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExportExecutionProperties>,
}
impl ExportExecution {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of listing the execution history of an export."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportExecutionListResult {
    #[doc = "A list of export executions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ExportExecution>,
}
impl ExportExecutionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the export execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportExecutionProperties {
    #[doc = "The type of the export execution."]
    #[serde(rename = "executionType", default, skip_serializing_if = "Option::is_none")]
    pub execution_type: Option<export_execution_properties::ExecutionType>,
    #[doc = "The last known status of the export execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<export_execution_properties::Status>,
    #[doc = "The identifier for the entity that executed the export. For OnDemand executions it is the user email. For scheduled executions it is 'System'."]
    #[serde(rename = "submittedBy", default, skip_serializing_if = "Option::is_none")]
    pub submitted_by: Option<String>,
    #[doc = "The time when export was queued to be executed."]
    #[serde(rename = "submittedTime", default, with = "azure_core::date::rfc3339::option")]
    pub submitted_time: Option<time::OffsetDateTime>,
    #[doc = "The time when export was picked up to be executed."]
    #[serde(rename = "processingStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub processing_start_time: Option<time::OffsetDateTime>,
    #[doc = "The time when the export execution finished."]
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
impl ExportExecutionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod export_execution_properties {
    use super::*;
    #[doc = "The type of the export execution."]
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
    #[doc = "The last known status of the export execution."]
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
#[doc = "The definition of data present in the forecast."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ForecastDataset {
    #[doc = "The granularity of rows in the forecast."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granularity: Option<forecast_dataset::Granularity>,
    #[doc = "The configuration of dataset in the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<QueryDatasetConfiguration>,
    #[doc = "Dictionary of aggregation expression to use in the forecast. The key of each item in the dictionary is the alias for the aggregated column. forecast can have up to 2 aggregation clauses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<serde_json::Value>,
    #[doc = "The filter expression to be used in the export."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<QueryFilter>,
}
impl ForecastDataset {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod forecast_dataset {
    use super::*;
    #[doc = "The granularity of rows in the forecast."]
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
#[doc = "The definition of a forecast."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForecastDefinition {
    #[doc = "The type of the forecast."]
    #[serde(rename = "type")]
    pub type_: forecast_definition::Type,
    #[doc = "The time frame for pulling data for the forecast. If custom, then a specific time period must be provided."]
    pub timeframe: forecast_definition::Timeframe,
    #[doc = "The start and end date for pulling data for the query."]
    #[serde(rename = "timePeriod", default, skip_serializing_if = "Option::is_none")]
    pub time_period: Option<QueryTimePeriod>,
    #[doc = "The definition of data present in the forecast."]
    pub dataset: ForecastDataset,
    #[doc = "a boolean determining if actualCost will be included"]
    #[serde(rename = "includeActualCost", default, skip_serializing_if = "Option::is_none")]
    pub include_actual_cost: Option<bool>,
    #[doc = "a boolean determining if FreshPartialCost will be included"]
    #[serde(rename = "includeFreshPartialCost", default, skip_serializing_if = "Option::is_none")]
    pub include_fresh_partial_cost: Option<bool>,
}
impl ForecastDefinition {
    pub fn new(type_: forecast_definition::Type, timeframe: forecast_definition::Timeframe, dataset: ForecastDataset) -> Self {
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
pub mod forecast_definition {
    use super::*;
    #[doc = "The type of the forecast."]
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
    #[doc = "The time frame for pulling data for the forecast. If custom, then a specific time period must be provided."]
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
#[doc = "The definition of a cost detailed report."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenerateDetailedCostReportDefinition {
    #[doc = "The type of the detailed report. By default ActualCost is provided"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric: Option<generate_detailed_cost_report_definition::Metric>,
    #[doc = "The start and end date for pulling data for the cost detailed report."]
    #[serde(rename = "timePeriod", default, skip_serializing_if = "Option::is_none")]
    pub time_period: Option<GenerateDetailedCostReportTimePeriod>,
    #[doc = "Billing Period in YearMonth(e.g. 202008) format. Only for legacy enterprise customers can use this. Can only have one of either timePeriod or invoiceId or billingPeriod parameters. If none provided current month cost is provided."]
    #[serde(rename = "billingPeriod", default, skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<String>,
    #[doc = "Invoice Id for PayAsYouGo customers and Modern billing profile scope. Can only have one of either timePeriod or invoiceId or billingPeriod parameters. If none provided current month cost is provided."]
    #[serde(rename = "invoiceId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    #[doc = "Customer Id for Modern (Invoice Id and billing profile is also required for this)."]
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
    #[doc = "The id of the long running operation."]
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
    #[doc = "The id of the long running operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the long running operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The status of the long running operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
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
#[doc = "A Cost management REST API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "The object that represents the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft.CostManagement."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Dimensions, Query."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
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
    pub value: Vec<Operation>,
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
#[doc = "The Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "eTag of the resource. To handle concurrent update scenario, this field will be used to determine whether the user is updating the latest version or not."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
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
    Tag,
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
            Self::Tag => serializer.serialize_unit_variant("QueryColumnType", 0u32, "Tag"),
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
    #[doc = "The filter expression to be used in the export."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not: Box<Option<QueryFilter>>,
    #[doc = "The comparison expression to be used in the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension: Option<QueryComparisonExpression>,
    #[doc = "The comparison expression to be used in the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<QueryComparisonExpression>,
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
    pub resource: Resource,
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
    Tag,
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
            Self::Tag => serializer.serialize_unit_variant("ReportConfigColumnType", 0u32, "Tag"),
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dataset: Option<ReportConfigDataset>,
}
impl ReportConfigDefinition {
    pub fn new(type_: report_config_definition::Type, timeframe: report_config_definition::Timeframe) -> Self {
        Self {
            type_,
            timeframe,
            time_period: None,
            dataset: None,
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
    #[doc = "The filter expression to be used in the report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not: Box<Option<ReportConfigFilter>>,
    #[doc = "The comparison expression to be used in the report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension: Option<ReportConfigComparisonExpression>,
    #[doc = "The comparison expression to be used in the report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<ReportConfigComparisonExpression>,
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
    pub enum Direction {
        Ascending,
        Descending,
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
#[doc = "The Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "States and configurations of Cost Analysis."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct View {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
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
