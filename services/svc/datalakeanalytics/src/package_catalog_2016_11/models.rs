#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "A Data Lake Analytics catalog access control list (ACL) entry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Acl {
    #[doc = "the access control list (ACL) entry type. UserObj and GroupObj denote the owning user and group, respectively."]
    #[serde(rename = "aceType", default, skip_serializing_if = "Option::is_none")]
    pub ace_type: Option<acl::AceType>,
    #[doc = "the Azure AD object ID of the user or group being specified in the access control list (ACL) entry."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "the permission type of the access control list (ACL) entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<acl::Permission>,
}
impl Acl {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod acl {
    use super::*;
    #[doc = "the access control list (ACL) entry type. UserObj and GroupObj denote the owning user and group, respectively."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AceType")]
    pub enum AceType {
        UserObj,
        GroupObj,
        Other,
        User,
        Group,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::UserObj => serializer.serialize_unit_variant("AceType", 0u32, "UserObj"),
                Self::GroupObj => serializer.serialize_unit_variant("AceType", 1u32, "GroupObj"),
                Self::Other => serializer.serialize_unit_variant("AceType", 2u32, "Other"),
                Self::User => serializer.serialize_unit_variant("AceType", 3u32, "User"),
                Self::Group => serializer.serialize_unit_variant("AceType", 4u32, "Group"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "the permission type of the access control list (ACL) entry."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Permission")]
    pub enum Permission {
        None,
        Use,
        Create,
        Drop,
        Alter,
        Write,
        All,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Permission {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Permission {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Permission {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Permission", 0u32, "None"),
                Self::Use => serializer.serialize_unit_variant("Permission", 1u32, "Use"),
                Self::Create => serializer.serialize_unit_variant("Permission", 2u32, "Create"),
                Self::Drop => serializer.serialize_unit_variant("Permission", 3u32, "Drop"),
                Self::Alter => serializer.serialize_unit_variant("Permission", 4u32, "Alter"),
                Self::Write => serializer.serialize_unit_variant("Permission", 5u32, "Write"),
                Self::All => serializer.serialize_unit_variant("Permission", 6u32, "All"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The parameters used to create or update an access control list (ACL) entry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AclCreateOrUpdateParameters {
    #[doc = "the access control list (ACL) entry type. UserObj and GroupObj denote the owning user and group, respectively."]
    #[serde(rename = "aceType")]
    pub ace_type: acl_create_or_update_parameters::AceType,
    #[doc = "the Azure AD object ID of the user or group being specified in the access control list (ACL) entry."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "the permission type of the access control list (ACL) entry."]
    pub permission: acl_create_or_update_parameters::Permission,
}
impl AclCreateOrUpdateParameters {
    pub fn new(
        ace_type: acl_create_or_update_parameters::AceType,
        principal_id: String,
        permission: acl_create_or_update_parameters::Permission,
    ) -> Self {
        Self {
            ace_type,
            principal_id,
            permission,
        }
    }
}
pub mod acl_create_or_update_parameters {
    use super::*;
    #[doc = "the access control list (ACL) entry type. UserObj and GroupObj denote the owning user and group, respectively."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AceType")]
    pub enum AceType {
        UserObj,
        GroupObj,
        Other,
        User,
        Group,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::UserObj => serializer.serialize_unit_variant("AceType", 0u32, "UserObj"),
                Self::GroupObj => serializer.serialize_unit_variant("AceType", 1u32, "GroupObj"),
                Self::Other => serializer.serialize_unit_variant("AceType", 2u32, "Other"),
                Self::User => serializer.serialize_unit_variant("AceType", 3u32, "User"),
                Self::Group => serializer.serialize_unit_variant("AceType", 4u32, "Group"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "the permission type of the access control list (ACL) entry."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Permission")]
    pub enum Permission {
        None,
        Use,
        Create,
        Drop,
        Alter,
        Write,
        All,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Permission {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Permission {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Permission {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Permission", 0u32, "None"),
                Self::Use => serializer.serialize_unit_variant("Permission", 1u32, "Use"),
                Self::Create => serializer.serialize_unit_variant("Permission", 2u32, "Create"),
                Self::Drop => serializer.serialize_unit_variant("Permission", 3u32, "Drop"),
                Self::Alter => serializer.serialize_unit_variant("Permission", 4u32, "Alter"),
                Self::Write => serializer.serialize_unit_variant("Permission", 5u32, "Write"),
                Self::All => serializer.serialize_unit_variant("Permission", 6u32, "All"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The parameters used to delete an access control list (ACL) entry."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AclDeleteParameters {
    #[doc = "the access control list (ACL) entry type. UserObj and GroupObj denote the owning user and group, respectively."]
    #[serde(rename = "aceType")]
    pub ace_type: acl_delete_parameters::AceType,
    #[doc = "the Azure AD object ID of the user or group being specified in the access control list (ACL) entry."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
}
impl AclDeleteParameters {
    pub fn new(ace_type: acl_delete_parameters::AceType, principal_id: String) -> Self {
        Self { ace_type, principal_id }
    }
}
pub mod acl_delete_parameters {
    use super::*;
    #[doc = "the access control list (ACL) entry type. UserObj and GroupObj denote the owning user and group, respectively."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AceType")]
    pub enum AceType {
        UserObj,
        GroupObj,
        Other,
        User,
        Group,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AceType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AceType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AceType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::UserObj => serializer.serialize_unit_variant("AceType", 0u32, "UserObj"),
                Self::GroupObj => serializer.serialize_unit_variant("AceType", 1u32, "GroupObj"),
                Self::Other => serializer.serialize_unit_variant("AceType", 2u32, "Other"),
                Self::User => serializer.serialize_unit_variant("AceType", 3u32, "User"),
                Self::Group => serializer.serialize_unit_variant("AceType", 4u32, "Group"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A Data Lake Analytics catalog access control list (ACL)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AclList {
    #[serde(flatten)]
    pub catalog_item_list: CatalogItemList,
    #[doc = "the access control list (ACL)."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Acl>,
}
impl azure_core::Continuable for AclList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AclList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CatalogItem {
    #[doc = "the name of the Data Lake Analytics account."]
    #[serde(rename = "computeAccountName", default, skip_serializing_if = "Option::is_none")]
    pub compute_account_name: Option<String>,
    #[doc = "the version of the catalog item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl CatalogItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog item list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CatalogItemList {
    #[doc = "the link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl CatalogItemList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Lake Analytics catalog credential creation parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeAnalyticsCatalogCredentialCreateParameters {
    #[doc = "the password for the credential and user with access to the data source."]
    pub password: String,
    #[doc = "the URI identifier for the data source this credential can connect to in the format <hostname>:<port>"]
    pub uri: String,
    #[doc = "the object identifier for the user associated with this credential with access to the data source."]
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl DataLakeAnalyticsCatalogCredentialCreateParameters {
    pub fn new(password: String, uri: String, user_id: String) -> Self {
        Self { password, uri, user_id }
    }
}
#[doc = "Data Lake Analytics catalog credential deletion parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeAnalyticsCatalogCredentialDeleteParameters {
    #[doc = "the current password for the credential and user with access to the data source. This is required if the requester is not the account owner."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl DataLakeAnalyticsCatalogCredentialDeleteParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Lake Analytics catalog credential update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataLakeAnalyticsCatalogCredentialUpdateParameters {
    #[doc = "the current password for the credential and user with access to the data source. This is required if the requester is not the account owner."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "the new password for the credential and user with access to the data source."]
    #[serde(rename = "newPassword", default, skip_serializing_if = "Option::is_none")]
    pub new_password: Option<String>,
    #[doc = "the URI identifier for the data source this credential can connect to in the format <hostname>:<port>"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "the object identifier for the user associated with this credential with access to the data source."]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}
impl DataLakeAnalyticsCatalogCredentialUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data Lake Analytics catalog secret creation and update parameters. This is deprecated and will be removed in the next release. Please use DataLakeAnalyticsCatalogCredentialCreateOrUpdateParameters instead."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataLakeAnalyticsCatalogSecretCreateOrUpdateParameters {
    #[doc = "the password for the secret to pass in"]
    pub password: String,
    #[doc = "the URI identifier for the secret in the format <hostname>:<port>"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl DataLakeAnalyticsCatalogSecretCreateOrUpdateParameters {
    pub fn new(password: String) -> Self {
        Self { password, uri: None }
    }
}
#[doc = "A Data Lake Analytics DDL name item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DdlName {
    #[doc = "the name of the table associated with this database and schema."]
    #[serde(rename = "firstPart", default, skip_serializing_if = "Option::is_none")]
    pub first_part: Option<String>,
    #[doc = "the name of the table associated with this database and schema."]
    #[serde(rename = "secondPart", default, skip_serializing_if = "Option::is_none")]
    pub second_part: Option<String>,
    #[doc = "the name of the table associated with this database and schema."]
    #[serde(rename = "thirdPart", default, skip_serializing_if = "Option::is_none")]
    pub third_part: Option<String>,
    #[doc = "the name of the table associated with this database and schema."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
}
impl DdlName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog entity identifier object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityId {
    #[doc = "A Data Lake Analytics DDL name item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<DdlName>,
    #[doc = "the version of the external data source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl EntityId {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog external table item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExternalTable {
    #[doc = "the name of the table associated with this database and schema."]
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[doc = "A Data Lake Analytics catalog entity identifier object."]
    #[serde(rename = "dataSource", default, skip_serializing_if = "Option::is_none")]
    pub data_source: Option<EntityId>,
}
impl ExternalTable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog type field information item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TypeFieldInfo {
    #[doc = "the name of the field associated with this type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "the type of the field."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl TypeFieldInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL Assembly."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlAssembly {
    #[serde(flatten)]
    pub catalog_item: CatalogItem,
    #[doc = "the name of the database."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "the name of the assembly."]
    #[serde(rename = "assemblyName", default, skip_serializing_if = "Option::is_none")]
    pub assembly_name: Option<String>,
    #[doc = "the name of the CLR."]
    #[serde(rename = "clrName", default, skip_serializing_if = "Option::is_none")]
    pub clr_name: Option<String>,
    #[doc = "the switch indicating if this assembly is visible or not."]
    #[serde(rename = "isVisible", default, skip_serializing_if = "Option::is_none")]
    pub is_visible: Option<bool>,
    #[doc = "the switch indicating if this assembly is user defined or not."]
    #[serde(rename = "isUserDefined", default, skip_serializing_if = "Option::is_none")]
    pub is_user_defined: Option<bool>,
    #[doc = "the list of files associated with the assembly"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<USqlAssemblyFileInfo>,
    #[doc = "the list of dependencies associated with the assembly"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<USqlAssemblyDependencyInfo>,
}
impl USqlAssembly {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL assembly CLR item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlAssemblyClr {
    #[serde(flatten)]
    pub catalog_item: CatalogItem,
    #[doc = "the name of the database."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "the name of the assembly."]
    #[serde(rename = "assemblyClrName", default, skip_serializing_if = "Option::is_none")]
    pub assembly_clr_name: Option<String>,
    #[doc = "the name of the CLR."]
    #[serde(rename = "clrName", default, skip_serializing_if = "Option::is_none")]
    pub clr_name: Option<String>,
}
impl USqlAssemblyClr {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL dependency information item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlAssemblyDependencyInfo {
    #[doc = "A Data Lake Analytics catalog entity identifier object."]
    #[serde(rename = "entityId", default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<EntityId>,
}
impl USqlAssemblyDependencyInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL assembly file information item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlAssemblyFileInfo {
    #[doc = "the assembly file type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<u_sql_assembly_file_info::Type>,
    #[doc = "The original path to the assembly file."]
    #[serde(rename = "originalPath", default, skip_serializing_if = "Option::is_none")]
    pub original_path: Option<String>,
    #[doc = "The content path to the assembly file."]
    #[serde(rename = "contentPath", default, skip_serializing_if = "Option::is_none")]
    pub content_path: Option<String>,
}
impl USqlAssemblyFileInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod u_sql_assembly_file_info {
    use super::*;
    #[doc = "the assembly file type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Assembly,
        Resource,
        Nodeploy,
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
                Self::Assembly => serializer.serialize_unit_variant("Type", 0u32, "Assembly"),
                Self::Resource => serializer.serialize_unit_variant("Type", 1u32, "Resource"),
                Self::Nodeploy => serializer.serialize_unit_variant("Type", 2u32, "Nodeploy"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL assembly CLR item list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlAssemblyList {
    #[serde(flatten)]
    pub catalog_item_list: CatalogItemList,
    #[doc = "the list of assemblies in the database"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<USqlAssemblyClr>,
}
impl azure_core::Continuable for USqlAssemblyList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl USqlAssemblyList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL credential item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlCredential {
    #[serde(flatten)]
    pub catalog_item: CatalogItem,
    #[doc = "the name of the credential."]
    #[serde(rename = "credentialName", default, skip_serializing_if = "Option::is_none")]
    pub credential_name: Option<String>,
}
impl USqlCredential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL credential item list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlCredentialList {
    #[serde(flatten)]
    pub catalog_item_list: CatalogItemList,
    #[doc = "the list of credentials in the database"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<USqlCredential>,
}
impl azure_core::Continuable for USqlCredentialList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl USqlCredentialList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL database item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlDatabase {
    #[serde(flatten)]
    pub catalog_item: CatalogItem,
    #[doc = "the name of the database."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
}
impl USqlDatabase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL database item list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlDatabaseList {
    #[serde(flatten)]
    pub catalog_item_list: CatalogItemList,
    #[doc = "the list of databases"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<USqlDatabase>,
}
impl azure_core::Continuable for USqlDatabaseList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl USqlDatabaseList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL directed column item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlDirectedColumn {
    #[doc = "the name of the index in the table."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "the switch indicating if the index is descending or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descending: Option<bool>,
}
impl USqlDirectedColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL distribution information object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlDistributionInfo {
    #[doc = "the type of this distribution."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,
    #[doc = "the list of directed columns in the distribution"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<USqlDirectedColumn>,
    #[doc = "the count of indices using this distribution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "the dynamic count of indices using this distribution."]
    #[serde(rename = "dynamicCount", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_count: Option<i32>,
}
impl USqlDistributionInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL external datasource item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlExternalDataSource {
    #[serde(flatten)]
    pub catalog_item: CatalogItem,
    #[doc = "the name of the database."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "the name of the external data source."]
    #[serde(rename = "externalDataSourceName", default, skip_serializing_if = "Option::is_none")]
    pub external_data_source_name: Option<String>,
    #[doc = "the name of the provider for the external data source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "the name of the provider string for the external data source."]
    #[serde(rename = "providerString", default, skip_serializing_if = "Option::is_none")]
    pub provider_string: Option<String>,
    #[doc = "the list of types to push down from the external data source."]
    #[serde(rename = "pushdownTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub pushdown_types: Vec<String>,
}
impl USqlExternalDataSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL external datasource item list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlExternalDataSourceList {
    #[serde(flatten)]
    pub catalog_item_list: CatalogItemList,
    #[doc = "the list of external data sources in the database"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<USqlExternalDataSource>,
}
impl azure_core::Continuable for USqlExternalDataSourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl USqlExternalDataSourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL table index item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlIndex {
    #[doc = "the name of the index in the table."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "the list of directed columns in the index"]
    #[serde(rename = "indexKeys", default, skip_serializing_if = "Vec::is_empty")]
    pub index_keys: Vec<USqlDirectedColumn>,
    #[doc = "the list of columns in the index"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<String>,
    #[doc = "A Data Lake Analytics catalog U-SQL distribution information object."]
    #[serde(rename = "distributionInfo", default, skip_serializing_if = "Option::is_none")]
    pub distribution_info: Option<USqlDistributionInfo>,
    #[doc = "partition function ID for the index."]
    #[serde(rename = "partitionFunction", default, skip_serializing_if = "Option::is_none")]
    pub partition_function: Option<String>,
    #[doc = "the list of partition keys in the index"]
    #[serde(rename = "partitionKeyList", default, skip_serializing_if = "Vec::is_empty")]
    pub partition_key_list: Vec<String>,
    #[doc = "the list of full paths to the streams that contain this index in the DataLake account."]
    #[serde(rename = "streamNames", default, skip_serializing_if = "Vec::is_empty")]
    pub stream_names: Vec<String>,
    #[doc = "the switch indicating if this index is a columnstore index."]
    #[serde(rename = "isColumnstore", default, skip_serializing_if = "Option::is_none")]
    pub is_columnstore: Option<bool>,
    #[doc = "the ID of this index within the table."]
    #[serde(rename = "indexId", default, skip_serializing_if = "Option::is_none")]
    pub index_id: Option<i32>,
    #[doc = "the switch indicating if this index is a unique index."]
    #[serde(rename = "isUnique", default, skip_serializing_if = "Option::is_none")]
    pub is_unique: Option<bool>,
}
impl USqlIndex {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL package item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlPackage {
    #[serde(flatten)]
    pub catalog_item: CatalogItem,
    #[doc = "the name of the database containing the package."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "the name of the schema associated with this package and database."]
    #[serde(rename = "schemaName", default, skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[doc = "the name of the package."]
    #[serde(rename = "packageName", default, skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[doc = "the definition of the package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}
impl USqlPackage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL package item list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlPackageList {
    #[serde(flatten)]
    pub catalog_item_list: CatalogItemList,
    #[doc = "the list of packages in the database and schema combination"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<USqlPackage>,
}
impl azure_core::Continuable for USqlPackageList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl USqlPackageList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL procedure item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlProcedure {
    #[serde(flatten)]
    pub catalog_item: CatalogItem,
    #[doc = "the name of the database."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "the name of the schema associated with this procedure and database."]
    #[serde(rename = "schemaName", default, skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[doc = "the name of the procedure."]
    #[serde(rename = "procName", default, skip_serializing_if = "Option::is_none")]
    pub proc_name: Option<String>,
    #[doc = "the defined query of the procedure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}
impl USqlProcedure {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL procedure item list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlProcedureList {
    #[serde(flatten)]
    pub catalog_item_list: CatalogItemList,
    #[doc = "the list of procedure in the database and schema combination"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<USqlProcedure>,
}
impl azure_core::Continuable for USqlProcedureList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl USqlProcedureList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL schema item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlSchema {
    #[serde(flatten)]
    pub catalog_item: CatalogItem,
    #[doc = "the name of the database."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "the name of the schema."]
    #[serde(rename = "schemaName", default, skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
}
impl USqlSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL schema item list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlSchemaList {
    #[serde(flatten)]
    pub catalog_item_list: CatalogItemList,
    #[doc = "the list of schemas in the database"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<USqlSchema>,
}
impl azure_core::Continuable for USqlSchemaList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl USqlSchemaList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL secret item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlSecret {
    #[serde(flatten)]
    pub catalog_item: CatalogItem,
    #[doc = "the name of the database."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "the name of the secret."]
    #[serde(rename = "secretName", default, skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
    #[doc = "the creation time of the credential object. This is the only information returned about a secret from a GET."]
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "the URI identifier for the secret in the format <hostname>:<port>"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "the password for the secret to pass in"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl USqlSecret {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL table item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlTable {
    #[serde(flatten)]
    pub catalog_item: CatalogItem,
    #[doc = "the name of the database."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "the name of the schema associated with this table and database."]
    #[serde(rename = "schemaName", default, skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[doc = "the name of the table."]
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[doc = "the list of columns in this table"]
    #[serde(rename = "columnList", default, skip_serializing_if = "Vec::is_empty")]
    pub column_list: Vec<USqlTableColumn>,
    #[doc = "the list of indices in this table"]
    #[serde(rename = "indexList", default, skip_serializing_if = "Vec::is_empty")]
    pub index_list: Vec<USqlIndex>,
    #[doc = "the list of partition keys in the table"]
    #[serde(rename = "partitionKeyList", default, skip_serializing_if = "Vec::is_empty")]
    pub partition_key_list: Vec<String>,
    #[doc = "A Data Lake Analytics catalog external table item."]
    #[serde(rename = "externalTable", default, skip_serializing_if = "Option::is_none")]
    pub external_table: Option<ExternalTable>,
    #[doc = "A Data Lake Analytics catalog U-SQL distribution information object."]
    #[serde(rename = "distributionInfo", default, skip_serializing_if = "Option::is_none")]
    pub distribution_info: Option<USqlDistributionInfo>,
}
impl USqlTable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL table column item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlTableColumn {
    #[doc = "the name of the column in the table."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "the object type of the specified column (such as System.String)."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl USqlTableColumn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL table fragment item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlTableFragment {
    #[doc = "the parent object Id of the table fragment. The parent could be a table or table partition."]
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[doc = "the version of the catalog item."]
    #[serde(rename = "fragmentId", default, skip_serializing_if = "Option::is_none")]
    pub fragment_id: Option<String>,
    #[doc = "the ordinal of the index which contains the table fragment."]
    #[serde(rename = "indexId", default, skip_serializing_if = "Option::is_none")]
    pub index_id: Option<i32>,
    #[doc = "the data size of the table fragment in bytes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[doc = "the number of rows in the table fragment."]
    #[serde(rename = "rowCount", default, skip_serializing_if = "Option::is_none")]
    pub row_count: Option<i64>,
    #[doc = "the creation time of the table fragment."]
    #[serde(rename = "createDate", default, with = "azure_core::date::rfc3339::option")]
    pub create_date: Option<time::OffsetDateTime>,
    #[doc = "the relative path for the table fragment location."]
    #[serde(rename = "streamPath", default, skip_serializing_if = "Option::is_none")]
    pub stream_path: Option<String>,
}
impl USqlTableFragment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL table fragment item list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlTableFragmentList {
    #[serde(flatten)]
    pub catalog_item_list: CatalogItemList,
    #[doc = "the list of table fragments in the database, schema and table combination"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<USqlTableFragment>,
}
impl azure_core::Continuable for USqlTableFragmentList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl USqlTableFragmentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL table item list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlTableList {
    #[serde(flatten)]
    pub catalog_item_list: CatalogItemList,
    #[doc = "the list of tables in the database and schema combination"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<USqlTable>,
}
impl azure_core::Continuable for USqlTableList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl USqlTableList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL table partition item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlTablePartition {
    #[serde(flatten)]
    pub catalog_item: CatalogItem,
    #[doc = "the name of the database."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "the name of the schema associated with this table partition and database."]
    #[serde(rename = "schemaName", default, skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[doc = "the name of the table partition."]
    #[serde(rename = "partitionName", default, skip_serializing_if = "Option::is_none")]
    pub partition_name: Option<String>,
    #[doc = "A Data Lake Analytics DDL name item."]
    #[serde(rename = "parentName", default, skip_serializing_if = "Option::is_none")]
    pub parent_name: Option<DdlName>,
    #[doc = "the index ID for this partition."]
    #[serde(rename = "indexId", default, skip_serializing_if = "Option::is_none")]
    pub index_id: Option<i32>,
    #[doc = "the list of labels associated with this partition."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub label: Vec<String>,
    #[doc = "the creation time of the partition"]
    #[serde(rename = "createDate", default, with = "azure_core::date::rfc3339::option")]
    pub create_date: Option<time::OffsetDateTime>,
}
impl USqlTablePartition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL table partition item list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlTablePartitionList {
    #[serde(flatten)]
    pub catalog_item_list: CatalogItemList,
    #[doc = "the list of table partitions in the database, schema and table combination"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<USqlTablePartition>,
}
impl azure_core::Continuable for USqlTablePartitionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl USqlTablePartitionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog table or partition preview rows item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlTablePreview {
    #[doc = "the total number of rows in the table or partition."]
    #[serde(rename = "totalRowCount", default, skip_serializing_if = "Option::is_none")]
    pub total_row_count: Option<i64>,
    #[doc = "the total number of columns in the table or partition."]
    #[serde(rename = "totalColumnCount", default, skip_serializing_if = "Option::is_none")]
    pub total_column_count: Option<i64>,
    #[doc = "the rows of the table or partition preview, where each row is an array of string representations the row's values. Note: Byte arrays will appear as base-64 encoded values, SqlMap and SqlArray objects will appear as escaped JSON objects, and DateTime objects will appear as ISO formatted UTC date-times."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rows: Vec<Vec<String>>,
    #[doc = "true if the amount of data in the response is less than expected due to the preview operation's size limitations. This can occur if the requested rows or row counts are too large."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
    #[doc = "the schema of the table or partition."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schema: Vec<USqlTableColumn>,
}
impl USqlTablePreview {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL table statistics item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlTableStatistics {
    #[serde(flatten)]
    pub catalog_item: CatalogItem,
    #[doc = "the name of the database."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "the name of the schema associated with this table and database."]
    #[serde(rename = "schemaName", default, skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[doc = "the name of the table."]
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[doc = "the name of the table statistics."]
    #[serde(rename = "statisticsName", default, skip_serializing_if = "Option::is_none")]
    pub statistics_name: Option<String>,
    #[doc = "the name of the user statistics."]
    #[serde(rename = "userStatName", default, skip_serializing_if = "Option::is_none")]
    pub user_stat_name: Option<String>,
    #[doc = "the path to the statistics data."]
    #[serde(rename = "statDataPath", default, skip_serializing_if = "Option::is_none")]
    pub stat_data_path: Option<String>,
    #[doc = "the creation time of the statistics."]
    #[serde(rename = "createTime", default, with = "azure_core::date::rfc3339::option")]
    pub create_time: Option<time::OffsetDateTime>,
    #[doc = "the last time the statistics were updated."]
    #[serde(rename = "updateTime", default, with = "azure_core::date::rfc3339::option")]
    pub update_time: Option<time::OffsetDateTime>,
    #[doc = "the switch indicating if these statistics are user created."]
    #[serde(rename = "isUserCreated", default, skip_serializing_if = "Option::is_none")]
    pub is_user_created: Option<bool>,
    #[doc = "the switch indicating if these statistics are automatically created."]
    #[serde(rename = "isAutoCreated", default, skip_serializing_if = "Option::is_none")]
    pub is_auto_created: Option<bool>,
    #[doc = "the switch indicating if these statistics have a filter."]
    #[serde(rename = "hasFilter", default, skip_serializing_if = "Option::is_none")]
    pub has_filter: Option<bool>,
    #[doc = "the filter definition for the statistics."]
    #[serde(rename = "filterDefinition", default, skip_serializing_if = "Option::is_none")]
    pub filter_definition: Option<String>,
    #[doc = "the list of column names associated with these statistics."]
    #[serde(rename = "colNames", default, skip_serializing_if = "Vec::is_empty")]
    pub col_names: Vec<String>,
}
impl USqlTableStatistics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL table statistics item list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlTableStatisticsList {
    #[serde(flatten)]
    pub catalog_item_list: CatalogItemList,
    #[doc = "the list of table statistics in the database, schema and table combination"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<USqlTableStatistics>,
}
impl azure_core::Continuable for USqlTableStatisticsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl USqlTableStatisticsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL table type item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlTableType {
    #[serde(flatten)]
    pub u_sql_type: USqlType,
    #[doc = "the type field information associated with this table type."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<TypeFieldInfo>,
}
impl USqlTableType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL table type item list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlTableTypeList {
    #[serde(flatten)]
    pub catalog_item_list: CatalogItemList,
    #[doc = "the list of table types in the database and schema combination"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<USqlTableType>,
}
impl azure_core::Continuable for USqlTableTypeList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl USqlTableTypeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL table valued function item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlTableValuedFunction {
    #[serde(flatten)]
    pub catalog_item: CatalogItem,
    #[doc = "the name of the database."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "the name of the schema associated with this database."]
    #[serde(rename = "schemaName", default, skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[doc = "the name of the table valued function."]
    #[serde(rename = "tvfName", default, skip_serializing_if = "Option::is_none")]
    pub tvf_name: Option<String>,
    #[doc = "the definition of the table valued function."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}
impl USqlTableValuedFunction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL table valued function item list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlTableValuedFunctionList {
    #[serde(flatten)]
    pub catalog_item_list: CatalogItemList,
    #[doc = "the list of table valued functions in the database and schema combination"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<USqlTableValuedFunction>,
}
impl azure_core::Continuable for USqlTableValuedFunctionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl USqlTableValuedFunctionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL type item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlType {
    #[serde(flatten)]
    pub catalog_item: CatalogItem,
    #[doc = "the name of the database."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "the name of the schema associated with this table and database."]
    #[serde(rename = "schemaName", default, skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[doc = "the name of type for this type."]
    #[serde(rename = "typeName", default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[doc = "the type family for this type."]
    #[serde(rename = "typeFamily", default, skip_serializing_if = "Option::is_none")]
    pub type_family: Option<String>,
    #[doc = "the C# name for this type."]
    #[serde(rename = "cSharpName", default, skip_serializing_if = "Option::is_none")]
    pub c_sharp_name: Option<String>,
    #[doc = "the fully qualified C# name for this type."]
    #[serde(rename = "fullCSharpName", default, skip_serializing_if = "Option::is_none")]
    pub full_c_sharp_name: Option<String>,
    #[doc = "the system type ID for this type."]
    #[serde(rename = "systemTypeId", default, skip_serializing_if = "Option::is_none")]
    pub system_type_id: Option<i32>,
    #[doc = "the user type ID for this type."]
    #[serde(rename = "userTypeId", default, skip_serializing_if = "Option::is_none")]
    pub user_type_id: Option<i32>,
    #[doc = "the schema ID for this type."]
    #[serde(rename = "schemaId", default, skip_serializing_if = "Option::is_none")]
    pub schema_id: Option<i32>,
    #[doc = "the principal ID for this type."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<i32>,
    #[doc = "The switch indicating if this type is nullable."]
    #[serde(rename = "isNullable", default, skip_serializing_if = "Option::is_none")]
    pub is_nullable: Option<bool>,
    #[doc = "The switch indicating if this type is user defined."]
    #[serde(rename = "isUserDefined", default, skip_serializing_if = "Option::is_none")]
    pub is_user_defined: Option<bool>,
    #[doc = "The switch indicating if this type is an assembly type."]
    #[serde(rename = "isAssemblyType", default, skip_serializing_if = "Option::is_none")]
    pub is_assembly_type: Option<bool>,
    #[doc = "The switch indicating if this type is a table type."]
    #[serde(rename = "isTableType", default, skip_serializing_if = "Option::is_none")]
    pub is_table_type: Option<bool>,
    #[doc = "The switch indicating if this type is a complex type."]
    #[serde(rename = "isComplexType", default, skip_serializing_if = "Option::is_none")]
    pub is_complex_type: Option<bool>,
}
impl USqlType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL type item list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlTypeList {
    #[serde(flatten)]
    pub catalog_item_list: CatalogItemList,
    #[doc = "the list of types in the database and schema combination"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<USqlType>,
}
impl azure_core::Continuable for USqlTypeList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl USqlTypeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL view item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlView {
    #[serde(flatten)]
    pub catalog_item: CatalogItem,
    #[doc = "the name of the database."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "the name of the schema associated with this view and database."]
    #[serde(rename = "schemaName", default, skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    #[doc = "the name of the view."]
    #[serde(rename = "viewName", default, skip_serializing_if = "Option::is_none")]
    pub view_name: Option<String>,
    #[doc = "the defined query of the view."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
}
impl USqlView {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Data Lake Analytics catalog U-SQL view item list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct USqlViewList {
    #[serde(flatten)]
    pub catalog_item_list: CatalogItemList,
    #[doc = "the list of view in the database and schema combination"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<USqlView>,
}
impl azure_core::Continuable for USqlViewList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl USqlViewList {
    pub fn new() -> Self {
        Self::default()
    }
}
