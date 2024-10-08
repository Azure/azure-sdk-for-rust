#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The arm template RE."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmResourceDefinitionResourceElementTemplate {
    #[doc = "The template type."]
    #[serde(rename = "templateType", default, skip_serializing_if = "Option::is_none")]
    pub template_type: Option<TemplateType>,
    #[doc = "Name and value pairs that define the parameter values. It can be  a well formed escaped JSON string."]
    #[serde(rename = "parameterValues", default, skip_serializing_if = "Option::is_none")]
    pub parameter_values: Option<String>,
    #[doc = "Artifact profile properties."]
    #[serde(rename = "artifactProfile", default, skip_serializing_if = "Option::is_none")]
    pub artifact_profile: Option<NsdArtifactProfile>,
}
impl ArmResourceDefinitionResourceElementTemplate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The arm resource definition resource element template details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmResourceDefinitionResourceElementTemplateDetails {
    #[serde(flatten)]
    pub resource_element_template: ResourceElementTemplate,
    #[doc = "The arm template RE."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ArmResourceDefinitionResourceElementTemplate>,
}
impl ArmResourceDefinitionResourceElementTemplateDetails {
    pub fn new(resource_element_template: ResourceElementTemplate) -> Self {
        Self {
            resource_element_template,
            configuration: None,
        }
    }
}
#[doc = "Template artifact profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmTemplateArtifactProfile {
    #[doc = "Template name."]
    #[serde(rename = "templateName", default, skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[doc = "Template version."]
    #[serde(rename = "templateVersion", default, skip_serializing_if = "Option::is_none")]
    pub template_version: Option<String>,
}
impl ArmTemplateArtifactProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Template mapping rule profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmTemplateMappingRuleProfile {
    #[doc = "List of template parameters."]
    #[serde(rename = "templateParameters", default, skip_serializing_if = "Option::is_none")]
    pub template_parameters: Option<String>,
}
impl ArmTemplateMappingRuleProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The credential type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "credentialType")]
pub enum ArtifactAccessCredentialUnion {
    AzureContainerRegistryScopedToken(AzureContainerRegistryScopedTokenCredential),
    AzureStorageAccountToken(AzureStorageAccountCredential),
}
#[doc = "The artifact updating request payload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactChangeState {
    #[doc = "The artifact update state properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ArtifactChangeStateProperties>,
}
impl ArtifactChangeState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The artifact update state properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactChangeStateProperties {
    #[doc = "The artifact state."]
    #[serde(rename = "artifactState", default, skip_serializing_if = "Option::is_none")]
    pub artifact_state: Option<ArtifactState>,
}
impl ArtifactChangeStateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Artifact manifest properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtifactManifest {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Artifact manifest properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ArtifactManifestPropertiesFormat>,
}
impl ArtifactManifest {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "A list of artifact manifests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactManifestListResult {
    #[doc = "A list of artifact manifests."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ArtifactManifest>,
    #[doc = "The URI to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ArtifactManifestListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ArtifactManifestListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Artifact manifest properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactManifestPropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The artifact manifest state."]
    #[serde(rename = "artifactManifestState", default, skip_serializing_if = "Option::is_none")]
    pub artifact_manifest_state: Option<ArtifactManifestState>,
    #[doc = "The artifacts list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub artifacts: Vec<ManifestArtifactFormat>,
}
impl ArtifactManifestPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The artifact manifest state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ArtifactManifestState")]
pub enum ArtifactManifestState {
    Unknown,
    Uploading,
    Uploaded,
    Validating,
    ValidationFailed,
    Succeeded,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ArtifactManifestState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ArtifactManifestState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ArtifactManifestState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("ArtifactManifestState", 0u32, "Unknown"),
            Self::Uploading => serializer.serialize_unit_variant("ArtifactManifestState", 1u32, "Uploading"),
            Self::Uploaded => serializer.serialize_unit_variant("ArtifactManifestState", 2u32, "Uploaded"),
            Self::Validating => serializer.serialize_unit_variant("ArtifactManifestState", 3u32, "Validating"),
            Self::ValidationFailed => serializer.serialize_unit_variant("ArtifactManifestState", 4u32, "ValidationFailed"),
            Self::Succeeded => serializer.serialize_unit_variant("ArtifactManifestState", 5u32, "Succeeded"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The artifact manifest updating request payload. Only the 'Uploaded' state is allowed for updates. Other states are used for internal state transitioning."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactManifestUpdateState {
    #[doc = "The artifact manifest state."]
    #[serde(rename = "artifactManifestState", default, skip_serializing_if = "Option::is_none")]
    pub artifact_manifest_state: Option<ArtifactManifestState>,
}
impl ArtifactManifestUpdateState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Artifact profile properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactProfile {
    #[doc = "Reference to another resource."]
    #[serde(rename = "artifactStore", default, skip_serializing_if = "Option::is_none")]
    pub artifact_store: Option<ReferencedResource>,
}
impl ArtifactProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The artifact state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ArtifactState")]
pub enum ArtifactState {
    Unknown,
    Preview,
    Active,
    Deprecated,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ArtifactState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ArtifactState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ArtifactState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("ArtifactState", 0u32, "Unknown"),
            Self::Preview => serializer.serialize_unit_variant("ArtifactState", 1u32, "Preview"),
            Self::Active => serializer.serialize_unit_variant("ArtifactState", 2u32, "Active"),
            Self::Deprecated => serializer.serialize_unit_variant("ArtifactState", 3u32, "Deprecated"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Artifact store properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArtifactStore {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Artifact store properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ArtifactStorePropertiesFormat>,
}
impl ArtifactStore {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "A list of artifact stores."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactStoreListResult {
    #[doc = "A list of artifact stores."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ArtifactStore>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ArtifactStoreListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ArtifactStoreListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Artifact store properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactStorePropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The artifact store type."]
    #[serde(rename = "storeType", default, skip_serializing_if = "Option::is_none")]
    pub store_type: Option<artifact_store_properties_format::StoreType>,
    #[doc = "The replication strategy."]
    #[serde(rename = "replicationStrategy", default, skip_serializing_if = "Option::is_none")]
    pub replication_strategy: Option<artifact_store_properties_format::ReplicationStrategy>,
    #[serde(rename = "managedResourceGroupConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub managed_resource_group_configuration: Option<artifact_store_properties_format::ManagedResourceGroupConfiguration>,
    #[doc = "The created storage resource id"]
    #[serde(rename = "storageResourceId", default, skip_serializing_if = "Option::is_none")]
    pub storage_resource_id: Option<String>,
}
impl ArtifactStorePropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod artifact_store_properties_format {
    use super::*;
    #[doc = "The artifact store type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StoreType")]
    pub enum StoreType {
        Unknown,
        AzureContainerRegistry,
        AzureStorageAccount,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StoreType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StoreType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StoreType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("StoreType", 0u32, "Unknown"),
                Self::AzureContainerRegistry => serializer.serialize_unit_variant("StoreType", 1u32, "AzureContainerRegistry"),
                Self::AzureStorageAccount => serializer.serialize_unit_variant("StoreType", 2u32, "AzureStorageAccount"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The replication strategy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReplicationStrategy")]
    pub enum ReplicationStrategy {
        Unknown,
        SingleReplication,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReplicationStrategy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReplicationStrategy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReplicationStrategy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("ReplicationStrategy", 0u32, "Unknown"),
                Self::SingleReplication => serializer.serialize_unit_variant("ReplicationStrategy", 1u32, "SingleReplication"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ManagedResourceGroupConfiguration {
        #[doc = "The managed resource group name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[doc = "The managed resource group location."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub location: Option<String>,
    }
    impl ManagedResourceGroupConfiguration {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The artifact type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ArtifactType")]
pub enum ArtifactType {
    Unknown,
    #[serde(rename = "OCIArtifact")]
    OciArtifact,
    VhdImageFile,
    ArmTemplate,
    ImageFile,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ArtifactType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ArtifactType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ArtifactType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("ArtifactType", 0u32, "Unknown"),
            Self::OciArtifact => serializer.serialize_unit_variant("ArtifactType", 1u32, "OCIArtifact"),
            Self::VhdImageFile => serializer.serialize_unit_variant("ArtifactType", 2u32, "VhdImageFile"),
            Self::ArmTemplate => serializer.serialize_unit_variant("ArtifactType", 3u32, "ArmTemplate"),
            Self::ImageFile => serializer.serialize_unit_variant("ArtifactType", 4u32, "ImageFile"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The AzureArcK8sCluster NFVI detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureArcK8sClusterNfviDetails {
    #[serde(flatten)]
    pub nfv_is: NfvIs,
    #[doc = "Reference to another resource."]
    #[serde(rename = "customLocationReference", default, skip_serializing_if = "Option::is_none")]
    pub custom_location_reference: Option<ReferencedResource>,
}
impl AzureArcK8sClusterNfviDetails {
    pub fn new(nfv_is: NfvIs) -> Self {
        Self {
            nfv_is,
            custom_location_reference: None,
        }
    }
}
#[doc = "Azure arc kubernetes artifact profile properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureArcKubernetesArtifactProfile {
    #[serde(flatten)]
    pub artifact_profile: ArtifactProfile,
    #[doc = "Helm artifact profile."]
    #[serde(rename = "helmArtifactProfile", default, skip_serializing_if = "Option::is_none")]
    pub helm_artifact_profile: Option<HelmArtifactProfile>,
}
impl AzureArcKubernetesArtifactProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure arc kubernetes deploy mapping rule profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureArcKubernetesDeployMappingRuleProfile {
    #[serde(flatten)]
    pub mapping_rule_profile: MappingRuleProfile,
    #[doc = "Helm mapping rule profile"]
    #[serde(rename = "helmMappingRuleProfile", default, skip_serializing_if = "Option::is_none")]
    pub helm_mapping_rule_profile: Option<HelmMappingRuleProfile>,
}
impl AzureArcKubernetesDeployMappingRuleProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure arc kubernetes helm application configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureArcKubernetesHelmApplication {
    #[serde(flatten)]
    pub azure_arc_kubernetes_network_function_application: AzureArcKubernetesNetworkFunctionApplication,
    #[doc = "Azure arc kubernetes artifact profile properties."]
    #[serde(rename = "artifactProfile", default, skip_serializing_if = "Option::is_none")]
    pub artifact_profile: Option<AzureArcKubernetesArtifactProfile>,
    #[doc = "Azure arc kubernetes deploy mapping rule profile."]
    #[serde(rename = "deployParametersMappingRuleProfile", default, skip_serializing_if = "Option::is_none")]
    pub deploy_parameters_mapping_rule_profile: Option<AzureArcKubernetesDeployMappingRuleProfile>,
}
impl AzureArcKubernetesHelmApplication {
    pub fn new(azure_arc_kubernetes_network_function_application: AzureArcKubernetesNetworkFunctionApplication) -> Self {
        Self {
            azure_arc_kubernetes_network_function_application,
            artifact_profile: None,
            deploy_parameters_mapping_rule_profile: None,
        }
    }
}
#[doc = "Azure arc kubernetes network function application definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureArcKubernetesNetworkFunctionApplication {
    #[serde(flatten)]
    pub network_function_application: NetworkFunctionApplication,
}
impl AzureArcKubernetesNetworkFunctionApplication {
    pub fn new() -> Self {
        Self {
            network_function_application: NetworkFunctionApplication::default(),
        }
    }
}
#[doc = "The artifact type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "artifactType")]
pub enum AzureArcKubernetesNetworkFunctionApplicationUnion {
    HelmPackage(AzureArcKubernetesHelmApplication),
}
#[doc = "Azure Arc kubernetes network function template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureArcKubernetesNetworkFunctionTemplate {
    #[doc = "Network function applications."]
    #[serde(
        rename = "networkFunctionApplications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub network_function_applications: Vec<AzureArcKubernetesNetworkFunctionApplicationUnion>,
}
impl AzureArcKubernetesNetworkFunctionTemplate {
    pub fn new() -> Self {
        Self {
            network_function_applications: Vec::new(),
        }
    }
}
#[doc = "The azure container registry scoped token credential definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureContainerRegistryScopedTokenCredential {
    #[doc = "The username of the credential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "The credential value."]
    #[serde(rename = "acrToken", default, skip_serializing_if = "Option::is_none")]
    pub acr_token: Option<String>,
    #[doc = "The Acr server url"]
    #[serde(rename = "acrServerUrl", default, skip_serializing_if = "Option::is_none")]
    pub acr_server_url: Option<String>,
    #[doc = "The repositories that could be accessed using the current credential."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub repositories: Vec<String>,
    #[doc = "The UTC time when credential will expire."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub expiry: Option<::time::OffsetDateTime>,
}
impl AzureContainerRegistryScopedTokenCredential {
    pub fn new() -> Self {
        Self {
            username: None,
            acr_token: None,
            acr_server_url: None,
            repositories: Vec::new(),
            expiry: None,
        }
    }
}
#[doc = "Azure template artifact profile properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureCoreArmTemplateArtifactProfile {
    #[serde(flatten)]
    pub artifact_profile: ArtifactProfile,
    #[doc = "Template artifact profile."]
    #[serde(rename = "templateArtifactProfile", default, skip_serializing_if = "Option::is_none")]
    pub template_artifact_profile: Option<ArmTemplateArtifactProfile>,
}
impl AzureCoreArmTemplateArtifactProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure template deploy mapping rule profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureCoreArmTemplateDeployMappingRuleProfile {
    #[serde(flatten)]
    pub mapping_rule_profile: MappingRuleProfile,
    #[doc = "Template mapping rule profile"]
    #[serde(rename = "templateMappingRuleProfile", default, skip_serializing_if = "Option::is_none")]
    pub template_mapping_rule_profile: Option<ArmTemplateMappingRuleProfile>,
}
impl AzureCoreArmTemplateDeployMappingRuleProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Azure Core NFVI detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCoreNfviDetails {
    #[serde(flatten)]
    pub nfv_is: NfvIs,
    #[doc = "Location of the Azure core."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl AzureCoreNfviDetails {
    pub fn new(nfv_is: NfvIs) -> Self {
        Self { nfv_is, location: None }
    }
}
#[doc = "Azure virtual network function application definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCoreNetworkFunctionApplication {
    #[serde(flatten)]
    pub network_function_application: NetworkFunctionApplication,
}
impl AzureCoreNetworkFunctionApplication {
    pub fn new() -> Self {
        Self {
            network_function_application: NetworkFunctionApplication::default(),
        }
    }
}
#[doc = "The artifact type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "artifactType")]
pub enum AzureCoreNetworkFunctionApplicationUnion {
    ArmTemplate(AzureCoreNetworkFunctionArmTemplateApplication),
    VhdImageFile(AzureCoreNetworkFunctionVhdApplication),
}
#[doc = "Azure core network function Template application definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCoreNetworkFunctionArmTemplateApplication {
    #[serde(flatten)]
    pub azure_core_network_function_application: AzureCoreNetworkFunctionApplication,
    #[doc = "Azure template artifact profile properties."]
    #[serde(rename = "artifactProfile", default, skip_serializing_if = "Option::is_none")]
    pub artifact_profile: Option<AzureCoreArmTemplateArtifactProfile>,
    #[doc = "Azure template deploy mapping rule profile."]
    #[serde(rename = "deployParametersMappingRuleProfile", default, skip_serializing_if = "Option::is_none")]
    pub deploy_parameters_mapping_rule_profile: Option<AzureCoreArmTemplateDeployMappingRuleProfile>,
}
impl AzureCoreNetworkFunctionArmTemplateApplication {
    pub fn new(azure_core_network_function_application: AzureCoreNetworkFunctionApplication) -> Self {
        Self {
            azure_core_network_function_application,
            artifact_profile: None,
            deploy_parameters_mapping_rule_profile: None,
        }
    }
}
#[doc = "Azure virtual network function template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCoreNetworkFunctionTemplate {
    #[doc = "Network function applications."]
    #[serde(
        rename = "networkFunctionApplications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub network_function_applications: Vec<AzureCoreNetworkFunctionApplicationUnion>,
}
impl AzureCoreNetworkFunctionTemplate {
    pub fn new() -> Self {
        Self {
            network_function_applications: Vec::new(),
        }
    }
}
#[doc = "Azure core network function vhd application definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCoreNetworkFunctionVhdApplication {
    #[serde(flatten)]
    pub azure_core_network_function_application: AzureCoreNetworkFunctionApplication,
    #[doc = "Azure vhd artifact profile properties."]
    #[serde(rename = "artifactProfile", default, skip_serializing_if = "Option::is_none")]
    pub artifact_profile: Option<AzureCoreVhdImageArtifactProfile>,
    #[doc = "Azure vhd deploy mapping rule profile."]
    #[serde(rename = "deployParametersMappingRuleProfile", default, skip_serializing_if = "Option::is_none")]
    pub deploy_parameters_mapping_rule_profile: Option<AzureCoreVhdImageDeployMappingRuleProfile>,
}
impl AzureCoreNetworkFunctionVhdApplication {
    pub fn new(azure_core_network_function_application: AzureCoreNetworkFunctionApplication) -> Self {
        Self {
            azure_core_network_function_application,
            artifact_profile: None,
            deploy_parameters_mapping_rule_profile: None,
        }
    }
}
#[doc = "Azure vhd artifact profile properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureCoreVhdImageArtifactProfile {
    #[serde(flatten)]
    pub artifact_profile: ArtifactProfile,
    #[doc = "Vhd artifact profile."]
    #[serde(rename = "vhdArtifactProfile", default, skip_serializing_if = "Option::is_none")]
    pub vhd_artifact_profile: Option<VhdImageArtifactProfile>,
}
impl AzureCoreVhdImageArtifactProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure vhd deploy mapping rule profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureCoreVhdImageDeployMappingRuleProfile {
    #[serde(flatten)]
    pub mapping_rule_profile: MappingRuleProfile,
    #[doc = "Vhd mapping rule profile"]
    #[serde(rename = "vhdImageMappingRuleProfile", default, skip_serializing_if = "Option::is_none")]
    pub vhd_image_mapping_rule_profile: Option<VhdImageMappingRuleProfile>,
}
impl AzureCoreVhdImageDeployMappingRuleProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Operator Distributed Services vhd artifact profile properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureOperatorNexusArmTemplateArtifactProfile {
    #[serde(flatten)]
    pub artifact_profile: ArtifactProfile,
    #[doc = "Template artifact profile."]
    #[serde(rename = "templateArtifactProfile", default, skip_serializing_if = "Option::is_none")]
    pub template_artifact_profile: Option<ArmTemplateArtifactProfile>,
}
impl AzureOperatorNexusArmTemplateArtifactProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Operator Distributed Services template deploy mapping rule profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureOperatorNexusArmTemplateDeployMappingRuleProfile {
    #[serde(flatten)]
    pub mapping_rule_profile: MappingRuleProfile,
    #[doc = "Template mapping rule profile"]
    #[serde(rename = "templateMappingRuleProfile", default, skip_serializing_if = "Option::is_none")]
    pub template_mapping_rule_profile: Option<ArmTemplateMappingRuleProfile>,
}
impl AzureOperatorNexusArmTemplateDeployMappingRuleProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The AzureOperatorNexusCluster NFVI detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureOperatorNexusClusterNfviDetails {
    #[serde(flatten)]
    pub nfv_is: NfvIs,
    #[doc = "Reference to another resource."]
    #[serde(rename = "customLocationReference", default, skip_serializing_if = "Option::is_none")]
    pub custom_location_reference: Option<ReferencedResource>,
}
impl AzureOperatorNexusClusterNfviDetails {
    pub fn new(nfv_is: NfvIs) -> Self {
        Self {
            nfv_is,
            custom_location_reference: None,
        }
    }
}
#[doc = "Azure Operator Distributed Services image artifact profile properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureOperatorNexusImageArtifactProfile {
    #[serde(flatten)]
    pub artifact_profile: ArtifactProfile,
    #[doc = "Image artifact profile."]
    #[serde(rename = "imageArtifactProfile", default, skip_serializing_if = "Option::is_none")]
    pub image_artifact_profile: Option<ImageArtifactProfile>,
}
impl AzureOperatorNexusImageArtifactProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Operator Distributed Services image deploy mapping rule profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureOperatorNexusImageDeployMappingRuleProfile {
    #[serde(flatten)]
    pub mapping_rule_profile: MappingRuleProfile,
    #[doc = "Image mapping rule profile"]
    #[serde(rename = "imageMappingRuleProfile", default, skip_serializing_if = "Option::is_none")]
    pub image_mapping_rule_profile: Option<ImageMappingRuleProfile>,
}
impl AzureOperatorNexusImageDeployMappingRuleProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Operator Distributed Services network function application definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureOperatorNexusNetworkFunctionApplication {
    #[serde(flatten)]
    pub network_function_application: NetworkFunctionApplication,
}
impl AzureOperatorNexusNetworkFunctionApplication {
    pub fn new() -> Self {
        Self {
            network_function_application: NetworkFunctionApplication::default(),
        }
    }
}
#[doc = "The artifact type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "artifactType")]
pub enum AzureOperatorNexusNetworkFunctionApplicationUnion {
    ArmTemplate(AzureOperatorNexusNetworkFunctionArmTemplateApplication),
    ImageFile(AzureOperatorNexusNetworkFunctionImageApplication),
}
#[doc = "Azure Operator Distributed Services network function Template application definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureOperatorNexusNetworkFunctionArmTemplateApplication {
    #[serde(flatten)]
    pub azure_operator_nexus_network_function_application: AzureOperatorNexusNetworkFunctionApplication,
    #[doc = "Azure Operator Distributed Services vhd artifact profile properties."]
    #[serde(rename = "artifactProfile", default, skip_serializing_if = "Option::is_none")]
    pub artifact_profile: Option<AzureOperatorNexusArmTemplateArtifactProfile>,
    #[doc = "Azure Operator Distributed Services template deploy mapping rule profile."]
    #[serde(rename = "deployParametersMappingRuleProfile", default, skip_serializing_if = "Option::is_none")]
    pub deploy_parameters_mapping_rule_profile: Option<AzureOperatorNexusArmTemplateDeployMappingRuleProfile>,
}
impl AzureOperatorNexusNetworkFunctionArmTemplateApplication {
    pub fn new(azure_operator_nexus_network_function_application: AzureOperatorNexusNetworkFunctionApplication) -> Self {
        Self {
            azure_operator_nexus_network_function_application,
            artifact_profile: None,
            deploy_parameters_mapping_rule_profile: None,
        }
    }
}
#[doc = "Azure Operator Distributed Services network function image application definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureOperatorNexusNetworkFunctionImageApplication {
    #[serde(flatten)]
    pub azure_operator_nexus_network_function_application: AzureOperatorNexusNetworkFunctionApplication,
    #[doc = "Azure Operator Distributed Services image artifact profile properties."]
    #[serde(rename = "artifactProfile", default, skip_serializing_if = "Option::is_none")]
    pub artifact_profile: Option<AzureOperatorNexusImageArtifactProfile>,
    #[doc = "Azure Operator Distributed Services image deploy mapping rule profile."]
    #[serde(rename = "deployParametersMappingRuleProfile", default, skip_serializing_if = "Option::is_none")]
    pub deploy_parameters_mapping_rule_profile: Option<AzureOperatorNexusImageDeployMappingRuleProfile>,
}
impl AzureOperatorNexusNetworkFunctionImageApplication {
    pub fn new(azure_operator_nexus_network_function_application: AzureOperatorNexusNetworkFunctionApplication) -> Self {
        Self {
            azure_operator_nexus_network_function_application,
            artifact_profile: None,
            deploy_parameters_mapping_rule_profile: None,
        }
    }
}
#[doc = "Azure Operator Distributed Services network function template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureOperatorNexusNetworkFunctionTemplate {
    #[doc = "Network function applications."]
    #[serde(
        rename = "networkFunctionApplications",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub network_function_applications: Vec<AzureOperatorNexusNetworkFunctionApplicationUnion>,
}
impl AzureOperatorNexusNetworkFunctionTemplate {
    pub fn new() -> Self {
        Self {
            network_function_applications: Vec::new(),
        }
    }
}
#[doc = "The azure storage account container credential definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureStorageAccountContainerCredential {
    #[doc = "The storage account container name"]
    #[serde(rename = "containerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[doc = "The storage account container sas uri"]
    #[serde(rename = "containerSasUri", default, skip_serializing_if = "Option::is_none")]
    pub container_sas_uri: Option<String>,
}
impl AzureStorageAccountContainerCredential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The azure storage account credential definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureStorageAccountCredential {
    #[doc = "The storage account Id"]
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[doc = "The containers that could be accessed using the current credential."]
    #[serde(
        rename = "containerCredentials",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub container_credentials: Vec<AzureStorageAccountContainerCredential>,
    #[doc = "The UTC time when credential will expire."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub expiry: Option<::time::OffsetDateTime>,
}
impl AzureStorageAccountCredential {
    pub fn new() -> Self {
        Self {
            storage_account_id: None,
            container_credentials: Vec::new(),
            expiry: None,
        }
    }
}
#[doc = "The component sub resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Component {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The component properties of the network function."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ComponentProperties>,
}
impl Component {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for list component API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComponentListResult {
    #[doc = "A list of component resources in a networkFunction."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Component>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ComponentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ComponentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The component properties of the network function."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComponentProperties {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The JSON-serialized deployment profile of the component resource."]
    #[serde(rename = "deploymentProfile", default, skip_serializing_if = "Option::is_none")]
    pub deployment_profile: Option<String>,
    #[doc = "The deployment status properties of the network function component."]
    #[serde(rename = "deploymentStatus", default, skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<DeploymentStatusProperties>,
}
impl ComponentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration generation type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ConfigurationGenerationType")]
pub enum ConfigurationGenerationType {
    Unknown,
    HandlebarTemplate,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ConfigurationGenerationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ConfigurationGenerationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ConfigurationGenerationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("ConfigurationGenerationType", 0u32, "Unknown"),
            Self::HandlebarTemplate => serializer.serialize_unit_variant("ConfigurationGenerationType", 1u32, "HandlebarTemplate"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The configuration group in form of {key, value}."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationGroup {}
impl ConfigurationGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration group schema resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationGroupSchema {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Configuration group schema properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfigurationGroupSchemaPropertiesFormat>,
}
impl ConfigurationGroupSchema {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "A list of configuration group schema resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationGroupSchemaListResult {
    #[doc = "A list of configuration group schema."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ConfigurationGroupSchema>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ConfigurationGroupSchemaListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ConfigurationGroupSchemaListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configuration group schema properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationGroupSchemaPropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The configuration group schema state."]
    #[serde(rename = "versionState", default, skip_serializing_if = "Option::is_none")]
    pub version_state: Option<ConfigurationGroupSchemaVersionState>,
    #[doc = "Description of what schema can contain."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Name and value pairs that define the configuration value. It can be a well formed escaped JSON string."]
    #[serde(rename = "schemaDefinition", default, skip_serializing_if = "Option::is_none")]
    pub schema_definition: Option<String>,
}
impl ConfigurationGroupSchemaPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration group schema state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ConfigurationGroupSchemaVersionState")]
pub enum ConfigurationGroupSchemaVersionState {
    Unknown,
    Preview,
    Active,
    Deprecated,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ConfigurationGroupSchemaVersionState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ConfigurationGroupSchemaVersionState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ConfigurationGroupSchemaVersionState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("ConfigurationGroupSchemaVersionState", 0u32, "Unknown"),
            Self::Preview => serializer.serialize_unit_variant("ConfigurationGroupSchemaVersionState", 1u32, "Preview"),
            Self::Active => serializer.serialize_unit_variant("ConfigurationGroupSchemaVersionState", 2u32, "Active"),
            Self::Deprecated => serializer.serialize_unit_variant("ConfigurationGroupSchemaVersionState", 3u32, "Deprecated"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Publisher configuration group schema update request definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationGroupSchemaVersionUpdateState {
    #[doc = "The configuration group schema state."]
    #[serde(rename = "versionState", default, skip_serializing_if = "Option::is_none")]
    pub version_state: Option<ConfigurationGroupSchemaVersionState>,
}
impl ConfigurationGroupSchemaVersionUpdateState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration object for the specified cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationGroups {}
impl ConfigurationGroups {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The ConfigurationValue with secrets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationValueWithSecrets {
    #[serde(flatten)]
    pub configuration_group_value_properties_format: ConfigurationGroupValuePropertiesFormat,
    #[doc = "Name and value pairs that define the configuration value secrets. It can be a well formed escaped JSON string."]
    #[serde(rename = "secretConfigurationValue", default, skip_serializing_if = "Option::is_none")]
    pub secret_configuration_value: Option<String>,
}
impl ConfigurationValueWithSecrets {
    pub fn new(configuration_group_value_properties_format: ConfigurationGroupValuePropertiesFormat) -> Self {
        Self {
            configuration_group_value_properties_format,
            secret_configuration_value: None,
        }
    }
}
#[doc = "The ConfigurationValue with no secrets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationValueWithoutSecrets {
    #[serde(flatten)]
    pub configuration_group_value_properties_format: ConfigurationGroupValuePropertiesFormat,
    #[doc = "Name and value pairs that define the configuration value. It can be a well formed escaped JSON string."]
    #[serde(rename = "configurationValue", default, skip_serializing_if = "Option::is_none")]
    pub configuration_value: Option<String>,
}
impl ConfigurationValueWithoutSecrets {
    pub fn new(configuration_group_value_properties_format: ConfigurationGroupValuePropertiesFormat) -> Self {
        Self {
            configuration_group_value_properties_format,
            configuration_value: None,
        }
    }
}
#[doc = "Containerized network function network function definition version properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerizedNetworkFunctionDefinitionVersion {
    #[serde(flatten)]
    pub network_function_definition_version_properties_format: NetworkFunctionDefinitionVersionPropertiesFormat,
    #[doc = "Containerized network function template."]
    #[serde(rename = "networkFunctionTemplate", default, skip_serializing_if = "Option::is_none")]
    pub network_function_template: Option<ContainerizedNetworkFunctionTemplateUnion>,
}
impl ContainerizedNetworkFunctionDefinitionVersion {
    pub fn new(network_function_definition_version_properties_format: NetworkFunctionDefinitionVersionPropertiesFormat) -> Self {
        Self {
            network_function_definition_version_properties_format,
            network_function_template: None,
        }
    }
}
#[doc = "The network function type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "nfviType")]
pub enum ContainerizedNetworkFunctionTemplateUnion {
    AzureArcKubernetes(AzureArcKubernetesNetworkFunctionTemplate),
}
#[doc = "Reference to an Azure ARC custom location resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomLocationResourceId {
    #[doc = "Azure ARC custom location resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl CustomLocationResourceId {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Helm DaemonSet status properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DaemonSet {
    #[doc = "The name of the daemonSet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The namespace of the daemonSet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "Desired number of pods"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desired: Option<i32>,
    #[doc = "Current number of pods"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current: Option<i32>,
    #[doc = "Number of Ready pods"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<i32>,
    #[doc = "Number of  upto date pods"]
    #[serde(rename = "upToDate", default, skip_serializing_if = "Option::is_none")]
    pub up_to_date: Option<i32>,
    #[doc = "Number of available pods."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available: Option<i32>,
    #[doc = "Creation Time of daemonSet."]
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<::time::OffsetDateTime>,
}
impl DaemonSet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Depends on profile definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DependsOnProfile {
    #[doc = "Application installation operation dependency."]
    #[serde(
        rename = "installDependsOn",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub install_depends_on: Vec<String>,
    #[doc = "Application deletion operation dependency."]
    #[serde(
        rename = "uninstallDependsOn",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub uninstall_depends_on: Vec<String>,
    #[doc = "Application update operation dependency."]
    #[serde(
        rename = "updateDependsOn",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub update_depends_on: Vec<String>,
}
impl DependsOnProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Helm Deployment status properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Deployment {
    #[doc = "The name of the deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The namespace of the deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "Desired number of pods"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desired: Option<i32>,
    #[doc = "Number of ready pods."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<i32>,
    #[doc = "Number of upto date pods."]
    #[serde(rename = "upToDate", default, skip_serializing_if = "Option::is_none")]
    pub up_to_date: Option<i32>,
    #[doc = "Number of available pods."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available: Option<i32>,
    #[doc = "Creation Time of deployment."]
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<::time::OffsetDateTime>,
}
impl Deployment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource reference arm id type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "idType")]
pub enum DeploymentResourceIdReferenceUnion {
    Open(OpenDeploymentResourceReference),
    Secret(SecretDeploymentResourceReference),
}
#[doc = "The deployment status properties of the network function component."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentStatusProperties {
    #[doc = "The component resource deployment status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[doc = "The resources of the network function component."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Resources>,
    #[doc = "The next expected update of deployment status."]
    #[serde(rename = "nextExpectedUpdateAt", default, with = "azure_core::date::rfc3339::option")]
    pub next_expected_update_at: Option<::time::OffsetDateTime>,
}
impl DeploymentStatusProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource management error additional info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[doc = "The additional info type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The additional info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
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
#[doc = "Payload for execute request post call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecuteRequestParameters {
    #[doc = "The endpoint of service to call."]
    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: String,
    #[doc = "Request metadata of execute request post call payload."]
    #[serde(rename = "requestMetadata")]
    pub request_metadata: RequestMetadata,
}
impl ExecuteRequestParameters {
    pub fn new(service_endpoint: String, request_metadata: RequestMetadata) -> Self {
        Self {
            service_endpoint,
            request_metadata,
        }
    }
}
#[doc = "The parameters for the generic object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenericParameters {}
impl GenericParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Helm artifact profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HelmArtifactProfile {
    #[doc = "Helm package name."]
    #[serde(rename = "helmPackageName", default, skip_serializing_if = "Option::is_none")]
    pub helm_package_name: Option<String>,
    #[doc = "Helm package version range."]
    #[serde(rename = "helmPackageVersionRange", default, skip_serializing_if = "Option::is_none")]
    pub helm_package_version_range: Option<String>,
    #[doc = "The registry values path list."]
    #[serde(
        rename = "registryValuesPaths",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub registry_values_paths: Vec<String>,
    #[doc = "The image pull secrets values path list."]
    #[serde(
        rename = "imagePullSecretsValuesPaths",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub image_pull_secrets_values_paths: Vec<String>,
}
impl HelmArtifactProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The helm deployment install options"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HelmInstallOptions {
    #[doc = "The helm deployment atomic options"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub atomic: Option<String>,
    #[doc = "The helm deployment wait options"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait: Option<String>,
    #[doc = "The helm deployment timeout options"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}
impl HelmInstallOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Helm mapping rule profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HelmMappingRuleProfile {
    #[doc = "Helm release namespace."]
    #[serde(rename = "releaseNamespace", default, skip_serializing_if = "Option::is_none")]
    pub release_namespace: Option<String>,
    #[doc = "Helm release name."]
    #[serde(rename = "releaseName", default, skip_serializing_if = "Option::is_none")]
    pub release_name: Option<String>,
    #[doc = "Helm package version."]
    #[serde(rename = "helmPackageVersion", default, skip_serializing_if = "Option::is_none")]
    pub helm_package_version: Option<String>,
    #[doc = "Helm release values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<String>,
    #[doc = "The helm deployment options"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<helm_mapping_rule_profile::Options>,
}
impl HelmMappingRuleProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod helm_mapping_rule_profile {
    use super::*;
    #[doc = "The helm deployment options"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Options {
        #[doc = "The helm deployment install options"]
        #[serde(rename = "installOptions", default, skip_serializing_if = "Option::is_none")]
        pub install_options: Option<HelmInstallOptions>,
        #[doc = "The helm deployment install options"]
        #[serde(rename = "upgradeOptions", default, skip_serializing_if = "Option::is_none")]
        pub upgrade_options: Option<HelmUpgradeOptions>,
    }
    impl Options {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The helm deployment install options"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HelmUpgradeOptions {
    #[doc = "The helm deployment atomic options"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub atomic: Option<String>,
    #[doc = "The helm deployment wait options"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait: Option<String>,
    #[doc = "The helm deployment timeout options"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}
impl HelmUpgradeOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource reference arm id type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IdType")]
pub enum IdType {
    Unknown,
    Open,
    Secret,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IdType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IdType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IdType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("IdType", 0u32, "Unknown"),
            Self::Open => serializer.serialize_unit_variant("IdType", 1u32, "Open"),
            Self::Secret => serializer.serialize_unit_variant("IdType", 2u32, "Secret"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for IdType {
    fn default() -> Self {
        Self::Open
    }
}
#[doc = "Image artifact profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageArtifactProfile {
    #[doc = "Image name."]
    #[serde(rename = "imageName", default, skip_serializing_if = "Option::is_none")]
    pub image_name: Option<String>,
    #[doc = "Image version."]
    #[serde(rename = "imageVersion", default, skip_serializing_if = "Option::is_none")]
    pub image_version: Option<String>,
}
impl ImageArtifactProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Image mapping rule profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageMappingRuleProfile {
    #[doc = "List of values."]
    #[serde(rename = "userConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub user_configuration: Option<String>,
}
impl ImageMappingRuleProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed resource group configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedResourceGroupConfiguration {
    #[doc = "Managed resource group name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Managed resource group location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl ManagedResourceGroupConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed service identity (system assigned and/or user assigned identities)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedServiceIdentity {
    #[doc = "The service principal ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
    #[serde(rename = "type")]
    pub type_: ManagedServiceIdentityType,
    #[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentities>,
}
impl ManagedServiceIdentity {
    pub fn new(type_: ManagedServiceIdentityType) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
            user_assigned_identities: None,
        }
    }
}
#[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedServiceIdentityType")]
pub enum ManagedServiceIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
    #[serde(rename = "SystemAssigned,UserAssigned")]
    SystemAssignedUserAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedServiceIdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedServiceIdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedServiceIdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ManagedServiceIdentityType", 0u32, "None"),
            Self::SystemAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 1u32, "SystemAssigned"),
            Self::UserAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 2u32, "UserAssigned"),
            Self::SystemAssignedUserAssigned => {
                serializer.serialize_unit_variant("ManagedServiceIdentityType", 3u32, "SystemAssigned,UserAssigned")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Manifest artifact properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManifestArtifactFormat {
    #[doc = "The artifact name"]
    #[serde(rename = "artifactName", default, skip_serializing_if = "Option::is_none")]
    pub artifact_name: Option<String>,
    #[doc = "The artifact type."]
    #[serde(rename = "artifactType", default, skip_serializing_if = "Option::is_none")]
    pub artifact_type: Option<ArtifactType>,
    #[doc = "The artifact version."]
    #[serde(rename = "artifactVersion", default, skip_serializing_if = "Option::is_none")]
    pub artifact_version: Option<String>,
}
impl ManifestArtifactFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Mapping rule profile properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MappingRuleProfile {
    #[doc = "The application enablement."]
    #[serde(rename = "applicationEnablement", default, skip_serializing_if = "Option::is_none")]
    pub application_enablement: Option<mapping_rule_profile::ApplicationEnablement>,
}
impl MappingRuleProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod mapping_rule_profile {
    use super::*;
    #[doc = "The application enablement."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ApplicationEnablement")]
    pub enum ApplicationEnablement {
        Unknown,
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ApplicationEnablement {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ApplicationEnablement {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ApplicationEnablement {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("ApplicationEnablement", 0u32, "Unknown"),
                Self::Enabled => serializer.serialize_unit_variant("ApplicationEnablement", 1u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("ApplicationEnablement", 2u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The NFVI type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NfviType")]
pub enum NfviType {
    Unknown,
    AzureArcKubernetes,
    AzureCore,
    AzureOperatorNexus,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NfviType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NfviType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NfviType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("NfviType", 0u32, "Unknown"),
            Self::AzureArcKubernetes => serializer.serialize_unit_variant("NfviType", 1u32, "AzureArcKubernetes"),
            Self::AzureCore => serializer.serialize_unit_variant("NfviType", 2u32, "AzureCore"),
            Self::AzureOperatorNexus => serializer.serialize_unit_variant("NfviType", 3u32, "AzureOperatorNexus"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The NFVI object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NfvIs {
    #[doc = "Name of the nfvi."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl NfvIs {
    pub fn new() -> Self {
        Self { name: None }
    }
}
#[doc = "The NFVI type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "nfviType")]
pub enum NfvIsUnion {
    AzureArcKubernetes(AzureArcK8sClusterNfviDetails),
    AzureCore(AzureCoreNfviDetails),
    AzureOperatorNexus(AzureOperatorNexusClusterNfviDetails),
}
#[doc = "Artifact profile properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NsdArtifactProfile {
    #[doc = "Reference to another resource."]
    #[serde(rename = "artifactStoreReference", default, skip_serializing_if = "Option::is_none")]
    pub artifact_store_reference: Option<ReferencedResource>,
    #[doc = "Artifact name."]
    #[serde(rename = "artifactName", default, skip_serializing_if = "Option::is_none")]
    pub artifact_name: Option<String>,
    #[doc = "Artifact version."]
    #[serde(rename = "artifactVersion", default, skip_serializing_if = "Option::is_none")]
    pub artifact_version: Option<String>,
}
impl NsdArtifactProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The network service design version state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NsdVersionState")]
pub enum NsdVersionState {
    Unknown,
    Preview,
    Active,
    Deprecated,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NsdVersionState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NsdVersionState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NsdVersionState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("NsdVersionState", 0u32, "Unknown"),
            Self::Preview => serializer.serialize_unit_variant("NsdVersionState", 1u32, "Preview"),
            Self::Active => serializer.serialize_unit_variant("NsdVersionState", 2u32, "Active"),
            Self::Deprecated => serializer.serialize_unit_variant("NsdVersionState", 3u32, "Deprecated"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Network function resource response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkFunction {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Network function properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkFunctionPropertiesFormatUnion>,
    #[doc = "A unique read-only string that changes whenever the resource is updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl NetworkFunction {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            etag: None,
            identity: None,
        }
    }
}
#[doc = "Network function application definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFunctionApplication {
    #[doc = "The name of the network function application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Depends on profile definition."]
    #[serde(rename = "dependsOnProfile", default, skip_serializing_if = "Option::is_none")]
    pub depends_on_profile: Option<DependsOnProfile>,
}
impl NetworkFunctionApplication {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The secret type which indicates if secret or not."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NetworkFunctionConfigurationType")]
pub enum NetworkFunctionConfigurationType {
    Unknown,
    Secret,
    Open,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NetworkFunctionConfigurationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NetworkFunctionConfigurationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NetworkFunctionConfigurationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("NetworkFunctionConfigurationType", 0u32, "Unknown"),
            Self::Secret => serializer.serialize_unit_variant("NetworkFunctionConfigurationType", 1u32, "Secret"),
            Self::Open => serializer.serialize_unit_variant("NetworkFunctionConfigurationType", 2u32, "Open"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Network function definition group resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkFunctionDefinitionGroup {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Network function definition group properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkFunctionDefinitionGroupPropertiesFormat>,
}
impl NetworkFunctionDefinitionGroup {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "A list of network function definition group resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFunctionDefinitionGroupListResult {
    #[doc = "A list of network function definition group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NetworkFunctionDefinitionGroup>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkFunctionDefinitionGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NetworkFunctionDefinitionGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network function definition group properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFunctionDefinitionGroupPropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The network function definition group description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl NetworkFunctionDefinitionGroupPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The network function definition resource element template details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkFunctionDefinitionResourceElementTemplateDetails {
    #[serde(flatten)]
    pub resource_element_template: ResourceElementTemplate,
    #[doc = "The arm template RE."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ArmResourceDefinitionResourceElementTemplate>,
}
impl NetworkFunctionDefinitionResourceElementTemplateDetails {
    pub fn new(resource_element_template: ResourceElementTemplate) -> Self {
        Self {
            resource_element_template,
            configuration: None,
        }
    }
}
#[doc = "Network function definition version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkFunctionDefinitionVersion {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Network function definition version properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkFunctionDefinitionVersionPropertiesFormatUnion>,
}
impl NetworkFunctionDefinitionVersion {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "A list of network function definition versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFunctionDefinitionVersionListResult {
    #[doc = "A list of network function definition versions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NetworkFunctionDefinitionVersion>,
    #[doc = "The URI to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkFunctionDefinitionVersionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NetworkFunctionDefinitionVersionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network function definition version properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkFunctionDefinitionVersionPropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The network function definition version state."]
    #[serde(rename = "versionState", default, skip_serializing_if = "Option::is_none")]
    pub version_state: Option<VersionState>,
    #[doc = "The network function definition version description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The deployment parameters of the network function definition version."]
    #[serde(rename = "deployParameters", default, skip_serializing_if = "Option::is_none")]
    pub deploy_parameters: Option<String>,
}
impl NetworkFunctionDefinitionVersionPropertiesFormat {
    pub fn new() -> Self {
        Self {
            provisioning_state: None,
            version_state: None,
            description: None,
            deploy_parameters: None,
        }
    }
}
#[doc = "The network function type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "networkFunctionType")]
pub enum NetworkFunctionDefinitionVersionPropertiesFormatUnion {
    ContainerizedNetworkFunction(ContainerizedNetworkFunctionDefinitionVersion),
    VirtualNetworkFunction(VirtualNetworkFunctionNetworkFunctionDefinitionVersion),
}
#[doc = "Publisher network function definition version update request definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFunctionDefinitionVersionUpdateState {
    #[doc = "The network function definition version state."]
    #[serde(rename = "versionState", default, skip_serializing_if = "Option::is_none")]
    pub version_state: Option<VersionState>,
}
impl NetworkFunctionDefinitionVersionUpdateState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for network function API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkFunctionListResult {
    #[doc = "A list of network function resources in a subscription or resource group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NetworkFunction>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkFunctionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NetworkFunctionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network function properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkFunctionPropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The publisher name for the network function."]
    #[serde(rename = "publisherName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_name: Option<String>,
    #[doc = "Publisher Scope."]
    #[serde(rename = "publisherScope", default, skip_serializing_if = "Option::is_none")]
    pub publisher_scope: Option<PublisherScope>,
    #[doc = "The network function definition group name for the network function."]
    #[serde(rename = "networkFunctionDefinitionGroupName", default, skip_serializing_if = "Option::is_none")]
    pub network_function_definition_group_name: Option<String>,
    #[doc = "The network function definition version for the network function."]
    #[serde(rename = "networkFunctionDefinitionVersion", default, skip_serializing_if = "Option::is_none")]
    pub network_function_definition_version: Option<String>,
    #[doc = "The location of the network function definition offering."]
    #[serde(
        rename = "networkFunctionDefinitionOfferingLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub network_function_definition_offering_location: Option<String>,
    #[doc = "The azure resource reference which is used for deployment."]
    #[serde(
        rename = "networkFunctionDefinitionVersionResourceReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub network_function_definition_version_resource_reference: Option<DeploymentResourceIdReferenceUnion>,
    #[doc = "The NFVI type."]
    #[serde(rename = "nfviType", default, skip_serializing_if = "Option::is_none")]
    pub nfvi_type: Option<NfviType>,
    #[doc = "The nfviId for the network function."]
    #[serde(rename = "nfviId", default, skip_serializing_if = "Option::is_none")]
    pub nfvi_id: Option<String>,
    #[doc = "Indicates if software updates are allowed during deployment."]
    #[serde(rename = "allowSoftwareUpdate", default, skip_serializing_if = "Option::is_none")]
    pub allow_software_update: Option<bool>,
    #[doc = "The role configuration override values from the user."]
    #[serde(
        rename = "roleOverrideValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub role_override_values: Vec<RoleOverrideValues>,
}
impl NetworkFunctionPropertiesFormat {
    pub fn new() -> Self {
        Self {
            provisioning_state: None,
            publisher_name: None,
            publisher_scope: None,
            network_function_definition_group_name: None,
            network_function_definition_version: None,
            network_function_definition_offering_location: None,
            network_function_definition_version_resource_reference: None,
            nfvi_type: None,
            nfvi_id: None,
            allow_software_update: None,
            role_override_values: Vec::new(),
        }
    }
}
#[doc = "The secret type which indicates if secret or not."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "configurationType")]
pub enum NetworkFunctionPropertiesFormatUnion {
    Secret(NetworkFunctionValueWithSecrets),
    Open(NetworkFunctionValueWithoutSecrets),
}
#[doc = "The network function type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "NetworkFunctionType")]
pub enum NetworkFunctionType {
    Unknown,
    VirtualNetworkFunction,
    ContainerizedNetworkFunction,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for NetworkFunctionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for NetworkFunctionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for NetworkFunctionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("NetworkFunctionType", 0u32, "Unknown"),
            Self::VirtualNetworkFunction => serializer.serialize_unit_variant("NetworkFunctionType", 1u32, "VirtualNetworkFunction"),
            Self::ContainerizedNetworkFunction => {
                serializer.serialize_unit_variant("NetworkFunctionType", 2u32, "ContainerizedNetworkFunction")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "NetworkFunction with secrets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkFunctionValueWithSecrets {
    #[serde(flatten)]
    pub network_function_properties_format: NetworkFunctionPropertiesFormat,
    #[doc = "The JSON-serialized secret deployment values from the user. This contains secrets like passwords,keys etc"]
    #[serde(rename = "secretDeploymentValues", default, skip_serializing_if = "Option::is_none")]
    pub secret_deployment_values: Option<String>,
}
impl NetworkFunctionValueWithSecrets {
    pub fn new(network_function_properties_format: NetworkFunctionPropertiesFormat) -> Self {
        Self {
            network_function_properties_format,
            secret_deployment_values: None,
        }
    }
}
#[doc = "NetworkFunction with no secrets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkFunctionValueWithoutSecrets {
    #[serde(flatten)]
    pub network_function_properties_format: NetworkFunctionPropertiesFormat,
    #[doc = "The JSON-serialized deployment values from the user."]
    #[serde(rename = "deploymentValues", default, skip_serializing_if = "Option::is_none")]
    pub deployment_values: Option<String>,
}
impl NetworkFunctionValueWithoutSecrets {
    pub fn new(network_function_properties_format: NetworkFunctionPropertiesFormat) -> Self {
        Self {
            network_function_properties_format,
            deployment_values: None,
        }
    }
}
#[doc = "network service design group resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkServiceDesignGroup {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "network service design group properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkServiceDesignGroupPropertiesFormat>,
}
impl NetworkServiceDesignGroup {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "A list of network service design group resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkServiceDesignGroupListResult {
    #[doc = "A list of network service design group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NetworkServiceDesignGroup>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkServiceDesignGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NetworkServiceDesignGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "network service design group properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkServiceDesignGroupPropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The network service design group description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl NetworkServiceDesignGroupPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Publisher network service design version update request definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkServiceDesignVersionUpdateState {
    #[doc = "The network service design version state."]
    #[serde(rename = "versionState", default, skip_serializing_if = "Option::is_none")]
    pub version_state: Option<NsdVersionState>,
}
impl NetworkServiceDesignVersionUpdateState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The nfvi details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NfviDetails {
    #[doc = "The nfvi name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The nfvi type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl NfviDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Non secret deployment resource id reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpenDeploymentResourceReference {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl OpenDeploymentResourceReference {
    pub fn new() -> Self {
        Self { id: None }
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
#[doc = "A list of REST API operations supported by an Azure Resource Provider. It contains an URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the resource provider"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results (if there are any)."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Helm Pod status properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Pod {
    #[doc = "The name of the Pod."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The namespace of the Pod."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "Desired number of containers"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desired: Option<i32>,
    #[doc = "Number of ready containers."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<i32>,
    #[doc = "The status of a Pod."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PodStatus>,
    #[doc = "Creation Time of Pod."]
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<::time::OffsetDateTime>,
    #[doc = "Last 5 Pod events."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub events: Vec<PodEvent>,
}
impl Pod {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Pod Event  properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PodEvent {
    #[doc = "The type of pod event."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<PodEventType>,
    #[doc = "Event reason."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "Event message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Event Last seen."]
    #[serde(rename = "lastSeenTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_seen_time: Option<::time::OffsetDateTime>,
}
impl PodEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of pod event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PodEventType")]
pub enum PodEventType {
    Normal,
    Warning,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PodEventType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PodEventType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PodEventType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Normal => serializer.serialize_unit_variant("PodEventType", 0u32, "Normal"),
            Self::Warning => serializer.serialize_unit_variant("PodEventType", 1u32, "Warning"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The status of a Pod."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PodStatus")]
pub enum PodStatus {
    Unknown,
    Succeeded,
    Failed,
    Running,
    Pending,
    Terminating,
    NotReady,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PodStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PodStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PodStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("PodStatus", 0u32, "Unknown"),
            Self::Succeeded => serializer.serialize_unit_variant("PodStatus", 1u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("PodStatus", 2u32, "Failed"),
            Self::Running => serializer.serialize_unit_variant("PodStatus", 3u32, "Running"),
            Self::Pending => serializer.serialize_unit_variant("PodStatus", 4u32, "Pending"),
            Self::Terminating => serializer.serialize_unit_variant("PodStatus", 5u32, "Terminating"),
            Self::NotReady => serializer.serialize_unit_variant("PodStatus", 6u32, "NotReady"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Unknown,
    Succeeded,
    Accepted,
    Deleting,
    Failed,
    Canceled,
    Deleted,
    Converging,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Unknown"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Accepted"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Canceled"),
            Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Deleted"),
            Self::Converging => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Converging"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The proxy artifact overview."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyArtifactListOverview {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
}
impl ProxyArtifactListOverview {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The proxy artifact overview."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyArtifactOverview {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Proxy Artifact overview properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProxyArtifactOverviewPropertiesFormat>,
}
impl ProxyArtifactOverview {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The proxy artifact list result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyArtifactOverviewListResult {
    #[doc = "A list of available proxy artifacts."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ProxyArtifactListOverview>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProxyArtifactOverviewListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ProxyArtifactOverviewListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Proxy Artifact overview properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyArtifactOverviewPropertiesFormat {
    #[doc = "The proxy artifact overview properties."]
    #[serde(
        rename = "artifactVersions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub artifact_versions: Vec<ProxyArtifactOverviewPropertiesValue>,
}
impl ProxyArtifactOverviewPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyArtifactOverviewPropertiesValue {
    #[doc = "The artifact type."]
    #[serde(rename = "artifactType", default, skip_serializing_if = "Option::is_none")]
    pub artifact_type: Option<ArtifactType>,
    #[doc = "The artifact version."]
    #[serde(rename = "artifactVersion", default, skip_serializing_if = "Option::is_none")]
    pub artifact_version: Option<String>,
    #[doc = "The artifact state."]
    #[serde(rename = "artifactState", default, skip_serializing_if = "Option::is_none")]
    pub artifact_state: Option<ArtifactState>,
}
impl ProxyArtifactOverviewPropertiesValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The proxy artifact overview."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyArtifactVersionsListOverview {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProxyArtifactOverviewPropertiesValue>,
}
impl ProxyArtifactVersionsListOverview {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The proxy artifact list result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyArtifactVersionsOverviewListResult {
    #[doc = "A list of available proxy artifacts."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ProxyArtifactVersionsListOverview>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProxyArtifactVersionsOverviewListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ProxyArtifactVersionsOverviewListResult {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "publisher resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Publisher {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "publisher properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PublisherPropertiesFormat>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl Publisher {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            identity: None,
        }
    }
}
#[doc = "A list of publishers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublisherListResult {
    #[doc = "A list of publishers."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Publisher>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PublisherListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PublisherListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "publisher properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublisherPropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Publisher Scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<PublisherScope>,
}
impl PublisherPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Publisher Scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PublisherScope")]
pub enum PublisherScope {
    Unknown,
    Private,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PublisherScope {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PublisherScope {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PublisherScope {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("PublisherScope", 0u32, "Unknown"),
            Self::Private => serializer.serialize_unit_variant("PublisherScope", 1u32, "Private"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Reference to another resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReferencedResource {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ReferencedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Helm ReplicaSet status properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicaSet {
    #[doc = "The name of the replicaSet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The namespace of the replicaSet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "Desired number of pods"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desired: Option<i32>,
    #[doc = "Number of ready pods."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<i32>,
    #[doc = "Number of current pods."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current: Option<i32>,
    #[doc = "Creation Time of replicaSet."]
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<::time::OffsetDateTime>,
}
impl ReplicaSet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request metadata of execute request post call payload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestMetadata {
    #[doc = "The relative path of the request."]
    #[serde(rename = "relativePath")]
    pub relative_path: String,
    #[doc = "The http method of the request."]
    #[serde(rename = "httpMethod")]
    pub http_method: request_metadata::HttpMethod,
    #[doc = "The serialized body of the request."]
    #[serde(rename = "serializedBody")]
    pub serialized_body: String,
    #[doc = "The api version of the request."]
    #[serde(rename = "apiVersion", default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
}
impl RequestMetadata {
    pub fn new(relative_path: String, http_method: request_metadata::HttpMethod, serialized_body: String) -> Self {
        Self {
            relative_path,
            http_method,
            serialized_body,
            api_version: None,
        }
    }
}
pub mod request_metadata {
    use super::*;
    #[doc = "The http method of the request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HttpMethod")]
    pub enum HttpMethod {
        Unknown,
        Post,
        Put,
        Get,
        Patch,
        Delete,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HttpMethod {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HttpMethod {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HttpMethod {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("HttpMethod", 0u32, "Unknown"),
                Self::Post => serializer.serialize_unit_variant("HttpMethod", 1u32, "Post"),
                Self::Put => serializer.serialize_unit_variant("HttpMethod", 2u32, "Put"),
                Self::Get => serializer.serialize_unit_variant("HttpMethod", 3u32, "Get"),
                Self::Patch => serializer.serialize_unit_variant("HttpMethod", 4u32, "Patch"),
                Self::Delete => serializer.serialize_unit_variant("HttpMethod", 5u32, "Delete"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource element template object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceElementTemplate {
    #[doc = "Name of the resource element template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Depends on profile definition."]
    #[serde(rename = "dependsOnProfile", default, skip_serializing_if = "Option::is_none")]
    pub depends_on_profile: Option<DependsOnProfile>,
}
impl ResourceElementTemplate {
    pub fn new() -> Self {
        Self {
            name: None,
            depends_on_profile: None,
        }
    }
}
#[doc = "The resource element template type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResourceElementTemplateUnion {
    ArmResourceDefinition(ArmResourceDefinitionResourceElementTemplateDetails),
    NetworkFunctionDefinition(NetworkFunctionDefinitionResourceElementTemplateDetails),
}
#[doc = "The resources of the network function component."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resources {
    #[doc = "Deployments that are related to component resource."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub deployments: Vec<Deployment>,
    #[doc = "Pods related to component resource."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pods: Vec<Pod>,
    #[doc = "Replica sets related to component resource."]
    #[serde(
        rename = "replicaSets",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub replica_sets: Vec<ReplicaSet>,
    #[doc = "Stateful sets related to component resource."]
    #[serde(
        rename = "statefulSets",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub stateful_sets: Vec<StatefulSet>,
    #[doc = "Daemonsets related to component resource."]
    #[serde(
        rename = "daemonSets",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub daemon_sets: Vec<DaemonSet>,
}
impl Resources {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type RoleOverrideValues = String;
#[doc = "Secret deployment resource id reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretDeploymentResourceReference {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SecretDeploymentResourceReference {
    pub fn new() -> Self {
        Self { id: None }
    }
}
#[doc = "Site resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Site {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Site properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SitePropertiesFormat>,
}
impl Site {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Response for sites API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteListResult {
    #[doc = "A list of sites in a resource group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Site>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SiteListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SiteListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Site network service resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SiteNetworkService {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Site network service properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SiteNetworkServicePropertiesFormat>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Sku, to be associated with a SiteNetworkService."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl SiteNetworkService {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            identity: None,
            sku: None,
        }
    }
}
#[doc = "Response for site network services API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteNetworkServiceListResult {
    #[doc = "A list of site network services in a resource group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SiteNetworkService>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SiteNetworkServiceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SiteNetworkServiceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Site network service properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SiteNetworkServicePropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Managed resource group configuration."]
    #[serde(rename = "managedResourceGroupConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub managed_resource_group_configuration: Option<ManagedResourceGroupConfiguration>,
    #[doc = "Reference to another resource."]
    #[serde(rename = "siteReference", default, skip_serializing_if = "Option::is_none")]
    pub site_reference: Option<ReferencedResource>,
    #[doc = "The publisher name for the site network service."]
    #[serde(rename = "publisherName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_name: Option<String>,
    #[doc = "Publisher Scope."]
    #[serde(rename = "publisherScope", default, skip_serializing_if = "Option::is_none")]
    pub publisher_scope: Option<PublisherScope>,
    #[doc = "The network service design group name for the site network service."]
    #[serde(rename = "networkServiceDesignGroupName", default, skip_serializing_if = "Option::is_none")]
    pub network_service_design_group_name: Option<String>,
    #[doc = "The network service design version for the site network service."]
    #[serde(rename = "networkServiceDesignVersionName", default, skip_serializing_if = "Option::is_none")]
    pub network_service_design_version_name: Option<String>,
    #[doc = "The location of the network service design offering."]
    #[serde(
        rename = "networkServiceDesignVersionOfferingLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub network_service_design_version_offering_location: Option<String>,
    #[doc = "The azure resource reference which is used for deployment."]
    #[serde(
        rename = "networkServiceDesignVersionResourceReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub network_service_design_version_resource_reference: Option<DeploymentResourceIdReferenceUnion>,
    #[doc = "The goal state of the site network service resource. This has references to the configuration group value objects that describe the desired state of the site network service."]
    #[serde(
        rename = "desiredStateConfigurationGroupValueReferences",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub desired_state_configuration_group_value_references: Option<serde_json::Value>,
    #[doc = "The network service design version for the site network service."]
    #[serde(
        rename = "lastStateNetworkServiceDesignVersionName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_state_network_service_design_version_name: Option<String>,
    #[doc = "The last state of the site network service resource."]
    #[serde(
        rename = "lastStateConfigurationGroupValueReferences",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_state_configuration_group_value_references: Option<serde_json::Value>,
}
impl SiteNetworkServicePropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Site properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SitePropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "List of NFVIs"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub nfvis: Vec<NfvIsUnion>,
    #[doc = "The list of site network services on the site."]
    #[serde(
        rename = "siteNetworkServiceReferences",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub site_network_service_references: Vec<ReferencedResource>,
}
impl SitePropertiesFormat {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sku, to be associated with a SiteNetworkService."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "Name of this Sku"]
    pub name: sku::Name,
    #[doc = "The SKU tier based on the SKU name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
}
impl Sku {
    pub fn new(name: sku::Name) -> Self {
        Self { name, tier: None }
    }
}
pub mod sku {
    use super::*;
    #[doc = "Name of this Sku"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        Basic,
        Standard,
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
                Self::Basic => serializer.serialize_unit_variant("Name", 0u32, "Basic"),
                Self::Standard => serializer.serialize_unit_variant("Name", 1u32, "Standard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The SKU tier based on the SKU name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Basic,
        Standard,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Tier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Tier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Tier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Basic => serializer.serialize_unit_variant("Tier", 0u32, "Basic"),
                Self::Standard => serializer.serialize_unit_variant("Tier", 1u32, "Standard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Helm StatefulSet status properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StatefulSet {
    #[doc = "The name of the statefulset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The namespace of the statefulset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "Desired number of pods"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desired: Option<i32>,
    #[doc = "Number of ready pods."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<i32>,
    #[doc = "Creation Time of statefulset."]
    #[serde(rename = "creationTime", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<::time::OffsetDateTime>,
}
impl StatefulSet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The component resource deployment status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Status")]
pub enum Status {
    Unknown,
    Deployed,
    Uninstalled,
    Superseded,
    Failed,
    Uninstalling,
    #[serde(rename = "Pending-Install")]
    PendingInstall,
    #[serde(rename = "Pending-Upgrade")]
    PendingUpgrade,
    #[serde(rename = "Pending-Rollback")]
    PendingRollback,
    Downloading,
    Installing,
    Reinstalling,
    Rollingback,
    Upgrading,
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
            Self::Unknown => serializer.serialize_unit_variant("Status", 0u32, "Unknown"),
            Self::Deployed => serializer.serialize_unit_variant("Status", 1u32, "Deployed"),
            Self::Uninstalled => serializer.serialize_unit_variant("Status", 2u32, "Uninstalled"),
            Self::Superseded => serializer.serialize_unit_variant("Status", 3u32, "Superseded"),
            Self::Failed => serializer.serialize_unit_variant("Status", 4u32, "Failed"),
            Self::Uninstalling => serializer.serialize_unit_variant("Status", 5u32, "Uninstalling"),
            Self::PendingInstall => serializer.serialize_unit_variant("Status", 6u32, "Pending-Install"),
            Self::PendingUpgrade => serializer.serialize_unit_variant("Status", 7u32, "Pending-Upgrade"),
            Self::PendingRollback => serializer.serialize_unit_variant("Status", 8u32, "Pending-Rollback"),
            Self::Downloading => serializer.serialize_unit_variant("Status", 9u32, "Downloading"),
            Self::Installing => serializer.serialize_unit_variant("Status", 10u32, "Installing"),
            Self::Reinstalling => serializer.serialize_unit_variant("Status", 11u32, "Reinstalling"),
            Self::Rollingback => serializer.serialize_unit_variant("Status", 12u32, "Rollingback"),
            Self::Upgrading => serializer.serialize_unit_variant("Status", 13u32, "Upgrading"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Tags object for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsObject {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TagsObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The template type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TemplateType")]
pub enum TemplateType {
    Unknown,
    ArmTemplate,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TemplateType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TemplateType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TemplateType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("TemplateType", 0u32, "Unknown"),
            Self::ArmTemplate => serializer.serialize_unit_variant("TemplateType", 1u32, "ArmTemplate"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resource model definition for an Azure Resource Manager tracked top level resource which has 'tags' and a 'location'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives"]
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
            location,
        }
    }
}
#[doc = "The resource element template type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Type")]
pub enum Type {
    Unknown,
    ArmResourceDefinition,
    NetworkFunctionDefinition,
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
            Self::Unknown => serializer.serialize_unit_variant("Type", 0u32, "Unknown"),
            Self::ArmResourceDefinition => serializer.serialize_unit_variant("Type", 1u32, "ArmResourceDefinition"),
            Self::NetworkFunctionDefinition => serializer.serialize_unit_variant("Type", 2u32, "NetworkFunctionDefinition"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentities {}
impl UserAssignedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User assigned identity properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The principal ID of the assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client ID of the assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The network function definition version state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VersionState")]
pub enum VersionState {
    Unknown,
    Preview,
    Validating,
    ValidationFailed,
    Active,
    Deprecated,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VersionState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VersionState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VersionState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("VersionState", 0u32, "Unknown"),
            Self::Preview => serializer.serialize_unit_variant("VersionState", 1u32, "Preview"),
            Self::Validating => serializer.serialize_unit_variant("VersionState", 2u32, "Validating"),
            Self::ValidationFailed => serializer.serialize_unit_variant("VersionState", 3u32, "ValidationFailed"),
            Self::Active => serializer.serialize_unit_variant("VersionState", 4u32, "Active"),
            Self::Deprecated => serializer.serialize_unit_variant("VersionState", 5u32, "Deprecated"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Vhd artifact profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VhdImageArtifactProfile {
    #[doc = "Vhd name."]
    #[serde(rename = "vhdName", default, skip_serializing_if = "Option::is_none")]
    pub vhd_name: Option<String>,
    #[doc = "Vhd version."]
    #[serde(rename = "vhdVersion", default, skip_serializing_if = "Option::is_none")]
    pub vhd_version: Option<String>,
}
impl VhdImageArtifactProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Vhd mapping rule profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VhdImageMappingRuleProfile {
    #[doc = "List of values."]
    #[serde(rename = "userConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub user_configuration: Option<String>,
}
impl VhdImageMappingRuleProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual network function network function definition version properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkFunctionNetworkFunctionDefinitionVersion {
    #[serde(flatten)]
    pub network_function_definition_version_properties_format: NetworkFunctionDefinitionVersionPropertiesFormat,
    #[doc = "Virtual network function template."]
    #[serde(rename = "networkFunctionTemplate", default, skip_serializing_if = "Option::is_none")]
    pub network_function_template: Option<VirtualNetworkFunctionTemplateUnion>,
}
impl VirtualNetworkFunctionNetworkFunctionDefinitionVersion {
    pub fn new(network_function_definition_version_properties_format: NetworkFunctionDefinitionVersionPropertiesFormat) -> Self {
        Self {
            network_function_definition_version_properties_format,
            network_function_template: None,
        }
    }
}
#[doc = "The network function type."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "nfviType")]
pub enum VirtualNetworkFunctionTemplateUnion {
    AzureCore(AzureCoreNetworkFunctionTemplate),
    AzureOperatorNexus(AzureOperatorNexusNetworkFunctionTemplate),
}
#[doc = "Hybrid configuration group value resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationGroupValue {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Hybrid configuration group value properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfigurationGroupValuePropertiesFormatUnion>,
}
impl ConfigurationGroupValue {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The secret type which indicates if secret or not."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ConfigurationGroupValueConfigurationType")]
pub enum ConfigurationGroupValueConfigurationType {
    Unknown,
    Secret,
    Open,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ConfigurationGroupValueConfigurationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ConfigurationGroupValueConfigurationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ConfigurationGroupValueConfigurationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("ConfigurationGroupValueConfigurationType", 0u32, "Unknown"),
            Self::Secret => serializer.serialize_unit_variant("ConfigurationGroupValueConfigurationType", 1u32, "Secret"),
            Self::Open => serializer.serialize_unit_variant("ConfigurationGroupValueConfigurationType", 2u32, "Open"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Response for hybrid configurationGroups API service call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationGroupValueListResult {
    #[doc = "A list of hybrid configurationGroups."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ConfigurationGroupValue>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ConfigurationGroupValueListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ConfigurationGroupValueListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Hybrid configuration group value properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationGroupValuePropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The publisher name for the configuration group schema."]
    #[serde(rename = "publisherName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_name: Option<String>,
    #[doc = "Publisher Scope."]
    #[serde(rename = "publisherScope", default, skip_serializing_if = "Option::is_none")]
    pub publisher_scope: Option<PublisherScope>,
    #[doc = "The configuration group schema name."]
    #[serde(rename = "configurationGroupSchemaName", default, skip_serializing_if = "Option::is_none")]
    pub configuration_group_schema_name: Option<String>,
    #[doc = "The location of the configuration group schema offering."]
    #[serde(
        rename = "configurationGroupSchemaOfferingLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration_group_schema_offering_location: Option<String>,
    #[doc = "The azure resource reference which is used for deployment."]
    #[serde(
        rename = "configurationGroupSchemaResourceReference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration_group_schema_resource_reference: Option<DeploymentResourceIdReferenceUnion>,
}
impl ConfigurationGroupValuePropertiesFormat {
    pub fn new() -> Self {
        Self {
            provisioning_state: None,
            publisher_name: None,
            publisher_scope: None,
            configuration_group_schema_name: None,
            configuration_group_schema_offering_location: None,
            configuration_group_schema_resource_reference: None,
        }
    }
}
#[doc = "The secret type which indicates if secret or not."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "configurationType")]
pub enum ConfigurationGroupValuePropertiesFormatUnion {
    Secret(ConfigurationValueWithSecrets),
    Open(ConfigurationValueWithoutSecrets),
}
#[doc = "network service design version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkServiceDesignVersion {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "network service design version properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkServiceDesignVersionPropertiesFormat>,
}
impl NetworkServiceDesignVersion {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "A list of network service design versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkServiceDesignVersionListResult {
    #[doc = "A list of network service design versions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NetworkServiceDesignVersion>,
    #[doc = "The URI to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkServiceDesignVersionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NetworkServiceDesignVersionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "network service design version properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkServiceDesignVersionPropertiesFormat {
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The network service design version state."]
    #[serde(rename = "versionState", default, skip_serializing_if = "Option::is_none")]
    pub version_state: Option<NsdVersionState>,
    #[doc = "The network service design version description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The configuration schemas to used to define the values."]
    #[serde(rename = "configurationGroupSchemaReferences", default, skip_serializing_if = "Option::is_none")]
    pub configuration_group_schema_references: Option<serde_json::Value>,
    #[doc = "The nfvis from the site."]
    #[serde(rename = "nfvisFromSite", default, skip_serializing_if = "Option::is_none")]
    pub nfvis_from_site: Option<serde_json::Value>,
    #[doc = "List of resource element template"]
    #[serde(
        rename = "resourceElementTemplates",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resource_element_templates: Vec<ResourceElementTemplateUnion>,
}
impl NetworkServiceDesignVersionPropertiesFormat {
    pub fn new() -> Self {
        Self::default()
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
    pub created_at: Option<::time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<::time::OffsetDateTime>,
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
