#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "AcsChat channel definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcsChatChannel {
    #[serde(flatten)]
    pub channel: Channel,
}
impl AcsChatChannel {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}
#[doc = "Alexa channel definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlexaChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[doc = "The parameters to provide for the Alexa channel."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlexaChannelProperties>,
}
impl AlexaChannel {
    pub fn new(channel: Channel) -> Self {
        Self { channel, properties: None }
    }
}
#[doc = "The parameters to provide for the Alexa channel."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlexaChannelProperties {
    #[doc = "The Alexa skill Id"]
    #[serde(rename = "alexaSkillId")]
    pub alexa_skill_id: String,
    #[doc = "Url fragment used in part of the Uri configured in Alexa"]
    #[serde(rename = "urlFragment", default, skip_serializing_if = "Option::is_none")]
    pub url_fragment: Option<String>,
    #[doc = "Full Uri used to configured the skill in Alexa"]
    #[serde(rename = "serviceEndpointUri", default, skip_serializing_if = "Option::is_none")]
    pub service_endpoint_uri: Option<String>,
    #[doc = "Whether this channel is enabled for the bot"]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
impl AlexaChannelProperties {
    pub fn new(alexa_skill_id: String, is_enabled: bool) -> Self {
        Self {
            alexa_skill_id,
            url_fragment: None,
            service_endpoint_uri: None,
            is_enabled,
        }
    }
}
#[doc = "Bot resource definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Bot {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The parameters to provide for the Bot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BotProperties>,
}
impl Bot {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Bot channel resource definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BotChannel {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Channel definition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Channel>,
}
impl BotChannel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters to provide for the Bot."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BotProperties {
    #[doc = "The Name of the bot"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The description of the bot"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The Icon Url of the bot"]
    #[serde(rename = "iconUrl", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[doc = "The bot's endpoint"]
    pub endpoint: String,
    #[doc = "The bot's endpoint version"]
    #[serde(rename = "endpointVersion", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_version: Option<String>,
    #[doc = "Contains resource all settings defined as key/value pairs."]
    #[serde(rename = "allSettings", default, skip_serializing_if = "Option::is_none")]
    pub all_settings: Option<serde_json::Value>,
    #[doc = "Contains resource parameters defined as key/value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "The bot's manifest url"]
    #[serde(rename = "manifestUrl", default, skip_serializing_if = "Option::is_none")]
    pub manifest_url: Option<String>,
    #[doc = "Microsoft App Type for the bot"]
    #[serde(rename = "msaAppType", default, skip_serializing_if = "Option::is_none")]
    pub msa_app_type: Option<bot_properties::MsaAppType>,
    #[doc = "Microsoft App Id for the bot"]
    #[serde(rename = "msaAppId")]
    pub msa_app_id: String,
    #[doc = "Microsoft App Tenant Id for the bot"]
    #[serde(rename = "msaAppTenantId", default, skip_serializing_if = "Option::is_none")]
    pub msa_app_tenant_id: Option<String>,
    #[doc = "Microsoft App Managed Identity Resource Id for the bot"]
    #[serde(rename = "msaAppMSIResourceId", default, skip_serializing_if = "Option::is_none")]
    pub msa_app_msi_resource_id: Option<String>,
    #[doc = "Collection of channels for which the bot is configured"]
    #[serde(
        rename = "configuredChannels",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub configured_channels: Vec<String>,
    #[doc = "Collection of channels for which the bot is enabled"]
    #[serde(
        rename = "enabledChannels",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enabled_channels: Vec<String>,
    #[doc = "The Application Insights key"]
    #[serde(rename = "developerAppInsightKey", default, skip_serializing_if = "Option::is_none")]
    pub developer_app_insight_key: Option<String>,
    #[doc = "The Application Insights Api Key"]
    #[serde(rename = "developerAppInsightsApiKey", default, skip_serializing_if = "Option::is_none")]
    pub developer_app_insights_api_key: Option<String>,
    #[doc = "The Application Insights App Id"]
    #[serde(rename = "developerAppInsightsApplicationId", default, skip_serializing_if = "Option::is_none")]
    pub developer_app_insights_application_id: Option<String>,
    #[doc = "Collection of LUIS App Ids"]
    #[serde(
        rename = "luisAppIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub luis_app_ids: Vec<String>,
    #[doc = "The LUIS Key"]
    #[serde(rename = "luisKey", default, skip_serializing_if = "Option::is_none")]
    pub luis_key: Option<String>,
    #[doc = "Whether Cmek is enabled"]
    #[serde(rename = "isCmekEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_cmek_enabled: Option<bool>,
    #[doc = "The CMK Url"]
    #[serde(rename = "cmekKeyVaultUrl", default, skip_serializing_if = "Option::is_none")]
    pub cmek_key_vault_url: Option<String>,
    #[doc = "The CMK encryption status"]
    #[serde(rename = "cmekEncryptionStatus", default, skip_serializing_if = "Option::is_none")]
    pub cmek_encryption_status: Option<String>,
    #[doc = "The Tenant Id for the bot"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Whether the bot is in an isolated network"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<bot_properties::PublicNetworkAccess>,
    #[doc = "Whether the bot is streaming supported"]
    #[serde(rename = "isStreamingSupported", default, skip_serializing_if = "Option::is_none")]
    pub is_streaming_supported: Option<bool>,
    #[doc = "Whether the bot is developerAppInsightsApiKey set"]
    #[serde(rename = "isDeveloperAppInsightsApiKeySet", default, skip_serializing_if = "Option::is_none")]
    pub is_developer_app_insights_api_key_set: Option<bool>,
    #[doc = "Token used to migrate non Azure bot to azure subscription"]
    #[serde(rename = "migrationToken", default, skip_serializing_if = "Option::is_none")]
    pub migration_token: Option<String>,
    #[doc = "Opt-out of local authentication and ensure only MSI and AAD can be used exclusively for authentication."]
    #[serde(rename = "disableLocalAuth", default, skip_serializing_if = "Option::is_none")]
    pub disable_local_auth: Option<bool>,
    #[doc = "The channel schema transformation version for the bot"]
    #[serde(rename = "schemaTransformationVersion", default, skip_serializing_if = "Option::is_none")]
    pub schema_transformation_version: Option<String>,
    #[doc = "The storage resourceId for the bot"]
    #[serde(rename = "storageResourceId", default, skip_serializing_if = "Option::is_none")]
    pub storage_resource_id: Option<String>,
    #[doc = "List of Private Endpoint Connections configured for the bot"]
    #[serde(
        rename = "privateEndpointConnections",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "The hint to browser (e.g. protocol handler) on how to open the bot for authoring"]
    #[serde(rename = "openWithHint", default, skip_serializing_if = "Option::is_none")]
    pub open_with_hint: Option<String>,
    #[doc = "The hint (e.g. keyVault secret resourceId) on how to fetch the app secret"]
    #[serde(rename = "appPasswordHint", default, skip_serializing_if = "Option::is_none")]
    pub app_password_hint: Option<String>,
    #[doc = "Provisioning state of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Publishing credentials of the resource"]
    #[serde(rename = "publishingCredentials", default, skip_serializing_if = "Option::is_none")]
    pub publishing_credentials: Option<String>,
}
impl BotProperties {
    pub fn new(display_name: String, endpoint: String, msa_app_id: String) -> Self {
        Self {
            display_name,
            description: None,
            icon_url: None,
            endpoint,
            endpoint_version: None,
            all_settings: None,
            parameters: None,
            manifest_url: None,
            msa_app_type: None,
            msa_app_id,
            msa_app_tenant_id: None,
            msa_app_msi_resource_id: None,
            configured_channels: Vec::new(),
            enabled_channels: Vec::new(),
            developer_app_insight_key: None,
            developer_app_insights_api_key: None,
            developer_app_insights_application_id: None,
            luis_app_ids: Vec::new(),
            luis_key: None,
            is_cmek_enabled: None,
            cmek_key_vault_url: None,
            cmek_encryption_status: None,
            tenant_id: None,
            public_network_access: None,
            is_streaming_supported: None,
            is_developer_app_insights_api_key_set: None,
            migration_token: None,
            disable_local_auth: None,
            schema_transformation_version: None,
            storage_resource_id: None,
            private_endpoint_connections: Vec::new(),
            open_with_hint: None,
            app_password_hint: None,
            provisioning_state: None,
            publishing_credentials: None,
        }
    }
}
pub mod bot_properties {
    use super::*;
    #[doc = "Microsoft App Type for the bot"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MsaAppType")]
    pub enum MsaAppType {
        #[serde(rename = "UserAssignedMSI")]
        UserAssignedMsi,
        SingleTenant,
        MultiTenant,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MsaAppType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MsaAppType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MsaAppType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::UserAssignedMsi => serializer.serialize_unit_variant("MsaAppType", 0u32, "UserAssignedMSI"),
                Self::SingleTenant => serializer.serialize_unit_variant("MsaAppType", 1u32, "SingleTenant"),
                Self::MultiTenant => serializer.serialize_unit_variant("MsaAppType", 2u32, "MultiTenant"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Whether the bot is in an isolated network"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicNetworkAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicNetworkAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicNetworkAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for PublicNetworkAccess {
        fn default() -> Self {
            Self::Enabled
        }
    }
}
#[doc = "The list of  bot service operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BotResponseList {
    #[doc = "The link used to get the next page of bot service resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Gets the list of bot service results and their properties."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Bot>,
}
impl azure_core::Continuable for BotResponseList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BotResponseList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Channel definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    #[doc = "The channel name"]
    #[serde(rename = "channelName")]
    pub channel_name: String,
    #[doc = "Entity Tag of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Provisioning state of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Specifies the location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl Channel {
    pub fn new(channel_name: String) -> Self {
        Self {
            channel_name,
            etag: None,
            provisioning_state: None,
            location: None,
        }
    }
}
#[doc = "The list of bot service channel operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChannelResponseList {
    #[doc = "The link used to get the next page of bot service channel resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Gets the list of bot service channel results and their properties."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BotChannel>,
}
impl azure_core::Continuable for ChannelResponseList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ChannelResponseList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Channel settings definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChannelSettings {
    #[doc = "The extensionKey1"]
    #[serde(rename = "extensionKey1", default, skip_serializing_if = "Option::is_none")]
    pub extension_key1: Option<String>,
    #[doc = "The extensionKey2"]
    #[serde(rename = "extensionKey2", default, skip_serializing_if = "Option::is_none")]
    pub extension_key2: Option<String>,
    #[doc = "The list of sites"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sites: Vec<Site>,
    #[doc = "The channel id"]
    #[serde(rename = "channelId", default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[doc = "The channel display name"]
    #[serde(rename = "channelDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub channel_display_name: Option<String>,
    #[doc = "The bot id"]
    #[serde(rename = "botId", default, skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[doc = "The bot icon url"]
    #[serde(rename = "botIconUrl", default, skip_serializing_if = "Option::is_none")]
    pub bot_icon_url: Option<String>,
    #[doc = "Whether this channel is enabled for the bot"]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Opt-out of local authentication and ensure only MSI and AAD can be used exclusively for authentication."]
    #[serde(rename = "disableLocalAuth", default, skip_serializing_if = "Option::is_none")]
    pub disable_local_auth: Option<bool>,
    #[doc = "Whether customer needs to agree to new terms."]
    #[serde(rename = "requireTermsAgreement", default, skip_serializing_if = "Option::is_none")]
    pub require_terms_agreement: Option<bool>,
}
impl ChannelSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The request body for a request to Bot Service Management to check availability of a bot name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityRequestBody {
    #[doc = "the name of the bot for which availability needs to be checked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "the type of the bot for which availability needs to be checked"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CheckNameAvailabilityRequestBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response body returned for a request to Bot Service Management to check availability of a bot name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResponseBody {
    #[doc = "indicates if the bot name is valid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
    #[doc = "additional message from the bot management api showing why a bot name is not available"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "response code from ABS"]
    #[serde(rename = "absCode", default, skip_serializing_if = "Option::is_none")]
    pub abs_code: Option<String>,
}
impl CheckNameAvailabilityResponseBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The display name of a connection Item Setting registered with the Bot"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionItemName {
    #[doc = "Connection Item name that has been added in the API"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ConnectionItemName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Bot channel resource definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionSetting {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties for a Connection Setting Item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConnectionSettingProperties>,
}
impl ConnectionSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Extra Parameter in a Connection Setting Properties to indicate service provider specific properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionSettingParameter {
    #[doc = "Key for the Connection Setting Parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "Value associated with the Connection Setting Parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ConnectionSettingParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for a Connection Setting Item"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionSettingProperties {
    #[doc = "Id of the Connection Setting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the Connection Setting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Client Id associated with the Connection Setting."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "Setting Id set by the service for the Connection Setting."]
    #[serde(rename = "settingId", default, skip_serializing_if = "Option::is_none")]
    pub setting_id: Option<String>,
    #[doc = "Client Secret associated with the Connection Setting"]
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[doc = "Scopes associated with the Connection Setting"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<String>,
    #[doc = "Service Provider Id associated with the Connection Setting"]
    #[serde(rename = "serviceProviderId", default, skip_serializing_if = "Option::is_none")]
    pub service_provider_id: Option<String>,
    #[doc = "Service Provider Display Name associated with the Connection Setting"]
    #[serde(rename = "serviceProviderDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub service_provider_display_name: Option<String>,
    #[doc = "Service Provider Parameters associated with the Connection Setting"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parameters: Vec<ConnectionSettingParameter>,
    #[doc = "Provisioning state of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ConnectionSettingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of bot service connection settings response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionSettingResponseList {
    #[doc = "The link used to get the next page of bot service connection setting resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Gets the list of bot service connection settings and their properties."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ConnectionSetting>,
}
impl azure_core::Continuable for ConnectionSettingResponseList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ConnectionSettingResponseList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Direct Line channel definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectLineChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[doc = "The parameters to provide for the Direct Line channel."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DirectLineChannelProperties>,
}
impl DirectLineChannel {
    pub fn new(channel: Channel) -> Self {
        Self { channel, properties: None }
    }
}
#[doc = "The parameters to provide for the Direct Line channel."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DirectLineChannelProperties {
    #[doc = "The list of Direct Line sites"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sites: Vec<DirectLineSite>,
    #[doc = "The extensionKey1"]
    #[serde(rename = "extensionKey1", default, skip_serializing_if = "Option::is_none")]
    pub extension_key1: Option<String>,
    #[doc = "The extensionKey2"]
    #[serde(rename = "extensionKey2", default, skip_serializing_if = "Option::is_none")]
    pub extension_key2: Option<String>,
    #[doc = "Direct Line embed code of the resource"]
    #[serde(rename = "DirectLineEmbedCode", default, skip_serializing_if = "Option::is_none")]
    pub direct_line_embed_code: Option<String>,
}
impl DirectLineChannelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A site for the Direct Line channel"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectLineSite {
    #[serde(flatten)]
    pub site: Site,
}
impl DirectLineSite {
    pub fn new(site: Site) -> Self {
        Self { site }
    }
}
#[doc = "DirectLine Speech channel definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectLineSpeechChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[doc = "The parameters to provide for the DirectLine Speech channel."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DirectLineSpeechChannelProperties>,
}
impl DirectLineSpeechChannel {
    pub fn new(channel: Channel) -> Self {
        Self { channel, properties: None }
    }
}
#[doc = "The parameters to provide for the DirectLine Speech channel."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DirectLineSpeechChannelProperties {
    #[doc = "The cognitive service id with this channel registration."]
    #[serde(rename = "cognitiveServiceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub cognitive_service_resource_id: Option<String>,
    #[doc = "The cognitive service region with this channel registration."]
    #[serde(rename = "cognitiveServiceRegion", default, skip_serializing_if = "Option::is_none")]
    pub cognitive_service_region: Option<String>,
    #[doc = "The cognitive service subscription key to use with this channel registration."]
    #[serde(rename = "cognitiveServiceSubscriptionKey", default, skip_serializing_if = "Option::is_none")]
    pub cognitive_service_subscription_key: Option<String>,
    #[doc = "Whether this channel is enabled or not."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Custom speech model id (optional)."]
    #[serde(rename = "customVoiceDeploymentId", default, skip_serializing_if = "Option::is_none")]
    pub custom_voice_deployment_id: Option<String>,
    #[doc = "Custom voice deployment id (optional)."]
    #[serde(rename = "customSpeechModelId", default, skip_serializing_if = "Option::is_none")]
    pub custom_speech_model_id: Option<String>,
    #[doc = "Make this a default bot for chosen cognitive service account."]
    #[serde(rename = "isDefaultBotForCogSvcAccount", default, skip_serializing_if = "Option::is_none")]
    pub is_default_bot_for_cog_svc_account: Option<bool>,
}
impl DirectLineSpeechChannelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Email channel definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[doc = "The parameters to provide for the Email channel."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EmailChannelProperties>,
}
impl EmailChannel {
    pub fn new(channel: Channel) -> Self {
        Self { channel, properties: None }
    }
}
#[doc = "Email channel auth method. 0 Password (Default); 1 Graph."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EmailChannelAuthMethod {}
#[doc = "The parameters to provide for the Email channel."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailChannelProperties {
    #[doc = "The email address"]
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    #[doc = "Email channel auth method. 0 Password (Default); 1 Graph."]
    #[serde(rename = "authMethod", default, skip_serializing_if = "Option::is_none")]
    pub auth_method: Option<EmailChannelAuthMethod>,
    #[doc = "The password for the email address. Value only returned through POST to the action Channel List API, otherwise empty."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "The magic code for setting up the modern authentication."]
    #[serde(rename = "magicCode", default, skip_serializing_if = "Option::is_none")]
    pub magic_code: Option<String>,
    #[doc = "Whether this channel is enabled for the bot"]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
impl EmailChannelProperties {
    pub fn new(email_address: String, is_enabled: bool) -> Self {
        Self {
            email_address,
            auth_method: None,
            password: None,
            magic_code: None,
            is_enabled,
        }
    }
}
#[doc = "Bot Service error object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "Bot Service error body."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorBody>,
}
impl azure_core::Continuable for Error {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Bot Service error body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorBody {
    #[doc = "error code"]
    pub code: String,
    #[doc = "error message"]
    pub message: String,
}
impl ErrorBody {
    pub fn new(code: String, message: String) -> Self {
        Self { code, message }
    }
}
#[doc = "Facebook channel definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacebookChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[doc = "The parameters to provide for the Facebook channel."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FacebookChannelProperties>,
}
impl FacebookChannel {
    pub fn new(channel: Channel) -> Self {
        Self { channel, properties: None }
    }
}
#[doc = "The parameters to provide for the Facebook channel."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacebookChannelProperties {
    #[doc = "Verify token. Value only returned through POST to the action Channel List API, otherwise empty."]
    #[serde(rename = "verifyToken", default, skip_serializing_if = "Option::is_none")]
    pub verify_token: Option<String>,
    #[doc = "The list of Facebook pages"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pages: Vec<FacebookPage>,
    #[doc = "Facebook application id"]
    #[serde(rename = "appId")]
    pub app_id: String,
    #[doc = "Facebook application secret. Value only returned through POST to the action Channel List API, otherwise empty."]
    #[serde(rename = "appSecret", default, skip_serializing_if = "Option::is_none")]
    pub app_secret: Option<String>,
    #[doc = "Callback Url"]
    #[serde(rename = "callbackUrl", default, skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    #[doc = "Whether this channel is enabled for the bot"]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
impl FacebookChannelProperties {
    pub fn new(app_id: String, is_enabled: bool) -> Self {
        Self {
            verify_token: None,
            pages: Vec::new(),
            app_id,
            app_secret: None,
            callback_url: None,
            is_enabled,
        }
    }
}
#[doc = "A Facebook page for Facebook channel registration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FacebookPage {
    #[doc = "Page id"]
    pub id: String,
    #[doc = "Facebook application access token. Value only returned through POST to the action Channel List API, otherwise empty."]
    #[serde(rename = "accessToken", default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
}
impl FacebookPage {
    pub fn new(id: String) -> Self {
        Self { id, access_token: None }
    }
}
#[doc = "The response body returned for a request to Bot Service Management to check per subscription hostSettings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HostSettingsResponse {
    #[doc = "For in-conversation bot user authentication"]
    #[serde(rename = "OAuthUrl", default, skip_serializing_if = "Option::is_none")]
    pub o_auth_url: Option<String>,
    #[doc = "For verifying incoming tokens from the channels"]
    #[serde(rename = "ToBotFromChannelOpenIdMetadataUrl", default, skip_serializing_if = "Option::is_none")]
    pub to_bot_from_channel_open_id_metadata_url: Option<String>,
    #[doc = "For verifying incoming tokens from the channels"]
    #[serde(rename = "ToBotFromChannelTokenIssuer", default, skip_serializing_if = "Option::is_none")]
    pub to_bot_from_channel_token_issuer: Option<String>,
    #[doc = "For verifying incoming tokens from bot emulator"]
    #[serde(rename = "ToBotFromEmulatorOpenIdMetadataUrl", default, skip_serializing_if = "Option::is_none")]
    pub to_bot_from_emulator_open_id_metadata_url: Option<String>,
    #[doc = "For getting access token to channels from bot host"]
    #[serde(rename = "ToChannelFromBotLoginUrl", default, skip_serializing_if = "Option::is_none")]
    pub to_channel_from_bot_login_url: Option<String>,
    #[doc = "For getting access token to channels from bot host"]
    #[serde(rename = "ToChannelFromBotOAuthScope", default, skip_serializing_if = "Option::is_none")]
    pub to_channel_from_bot_o_auth_scope: Option<String>,
    #[doc = "Per cloud OAuth setting on whether authority is validated"]
    #[serde(rename = "ValidateAuthority", default, skip_serializing_if = "Option::is_none")]
    pub validate_authority: Option<bool>,
    #[doc = "Same as toBotFromChannelOpenIdMetadataUrl, used by SDK < v4.12"]
    #[serde(rename = "BotOpenIdMetadata", default, skip_serializing_if = "Option::is_none")]
    pub bot_open_id_metadata: Option<String>,
}
impl HostSettingsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Kik channel definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KikChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[doc = "The parameters to provide for the Kik channel."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<KikChannelProperties>,
}
impl KikChannel {
    pub fn new(channel: Channel) -> Self {
        Self { channel, properties: None }
    }
}
#[doc = "The parameters to provide for the Kik channel."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KikChannelProperties {
    #[doc = "The Kik user name"]
    #[serde(rename = "userName")]
    pub user_name: String,
    #[doc = "Kik API key. Value only returned through POST to the action Channel List API, otherwise empty."]
    #[serde(rename = "apiKey", default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[doc = "Whether this channel is validated for the bot"]
    #[serde(rename = "isValidated", default, skip_serializing_if = "Option::is_none")]
    pub is_validated: Option<bool>,
    #[doc = "Whether this channel is enabled for the bot"]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
impl KikChannelProperties {
    pub fn new(user_name: String, is_enabled: bool) -> Self {
        Self {
            user_name,
            api_key: None,
            is_validated: None,
            is_enabled,
        }
    }
}
#[doc = "Indicates the type of bot service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Kind")]
pub enum Kind {
    #[serde(rename = "sdk")]
    Sdk,
    #[serde(rename = "designer")]
    Designer,
    #[serde(rename = "bot")]
    Bot,
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "azurebot")]
    Azurebot,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Kind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Kind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Kind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Sdk => serializer.serialize_unit_variant("Kind", 0u32, "sdk"),
            Self::Designer => serializer.serialize_unit_variant("Kind", 1u32, "designer"),
            Self::Bot => serializer.serialize_unit_variant("Kind", 2u32, "bot"),
            Self::Function => serializer.serialize_unit_variant("Kind", 3u32, "function"),
            Self::Azurebot => serializer.serialize_unit_variant("Kind", 4u32, "azurebot"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Line channel definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LineChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[doc = "The parameters to provide for the Line channel."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LineChannelProperties>,
}
impl LineChannel {
    pub fn new(channel: Channel) -> Self {
        Self { channel, properties: None }
    }
}
#[doc = "The parameters to provide for the Line channel."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LineChannelProperties {
    #[doc = "The list of line channel registrations"]
    #[serde(rename = "lineRegistrations")]
    pub line_registrations: Vec<LineRegistration>,
    #[doc = "Callback Url to enter in line registration."]
    #[serde(rename = "callbackUrl", default, skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    #[doc = "Whether this channel is validated for the bot"]
    #[serde(rename = "isValidated", default, skip_serializing_if = "Option::is_none")]
    pub is_validated: Option<bool>,
}
impl LineChannelProperties {
    pub fn new(line_registrations: Vec<LineRegistration>) -> Self {
        Self {
            line_registrations,
            callback_url: None,
            is_validated: None,
        }
    }
}
#[doc = "The properties corresponding to a line channel registration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LineRegistration {
    #[doc = "Id generated for the line channel registration"]
    #[serde(rename = "generatedId", default, skip_serializing_if = "Option::is_none")]
    pub generated_id: Option<String>,
    #[doc = "Secret for the line channel registration"]
    #[serde(rename = "channelSecret", default, skip_serializing_if = "Option::is_none")]
    pub channel_secret: Option<String>,
    #[doc = "Access token for the line channel registration"]
    #[serde(rename = "channelAccessToken", default, skip_serializing_if = "Option::is_none")]
    pub channel_access_token: Option<String>,
}
impl LineRegistration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The ARM channel of list channel with keys operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListChannelWithKeysResponse {
    #[serde(flatten)]
    pub bot_channel: BotChannel,
    #[doc = "Channel definition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<Channel>,
    #[doc = "Channel settings definition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub setting: Option<ChannelSettings>,
    #[doc = "Provisioning state of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Entity tag of the resource"]
    #[serde(rename = "entityTag", default, skip_serializing_if = "Option::is_none")]
    pub entity_tag: Option<String>,
    #[doc = "Changed time of the resource"]
    #[serde(rename = "changedTime", default, skip_serializing_if = "Option::is_none")]
    pub changed_time: Option<String>,
}
impl ListChannelWithKeysResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "M365 Extensions definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct M365Extensions {
    #[serde(flatten)]
    pub channel: Channel,
}
impl M365Extensions {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}
#[doc = "Microsoft Teams channel definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MsTeamsChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[doc = "The parameters to provide for the Microsoft Teams channel."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MsTeamsChannelProperties>,
}
impl MsTeamsChannel {
    pub fn new(channel: Channel) -> Self {
        Self { channel, properties: None }
    }
}
#[doc = "The parameters to provide for the Microsoft Teams channel."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MsTeamsChannelProperties {
    #[doc = "Enable calling for Microsoft Teams channel"]
    #[serde(rename = "enableCalling", default, skip_serializing_if = "Option::is_none")]
    pub enable_calling: Option<bool>,
    #[doc = "Webhook for Microsoft Teams channel calls"]
    #[serde(rename = "callingWebhook", default, skip_serializing_if = "Option::is_none")]
    pub calling_webhook: Option<String>,
    #[doc = "Whether this channel is enabled for the bot"]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    #[doc = "Webhook for Microsoft Teams channel calls"]
    #[serde(rename = "incomingCallRoute", default, skip_serializing_if = "Option::is_none")]
    pub incoming_call_route: Option<String>,
    #[doc = "Deployment environment for Microsoft Teams channel calls"]
    #[serde(rename = "deploymentEnvironment", default, skip_serializing_if = "Option::is_none")]
    pub deployment_environment: Option<String>,
    #[doc = "Whether this channel accepted terms"]
    #[serde(rename = "acceptedTerms", default, skip_serializing_if = "Option::is_none")]
    pub accepted_terms: Option<bool>,
}
impl MsTeamsChannelProperties {
    pub fn new(is_enabled: bool) -> Self {
        Self {
            enable_calling: None,
            calling_webhook: None,
            is_enabled,
            incoming_call_route: None,
            deployment_environment: None,
            accepted_terms: None,
        }
    }
}
#[doc = "Omnichannel channel definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Omnichannel {
    #[serde(flatten)]
    pub channel: Channel,
}
impl Omnichannel {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}
#[doc = "The operation supported by Bot Service Management."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplayInfo {
    #[doc = "The description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The action that users can perform, based on their permission level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Service provider: Microsoft Bot Service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource on which the operation is performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
impl OperationDisplayInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The operations supported by Bot Service Management."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationEntity {
    #[doc = "Operation name: {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The operation supported by Bot Service Management."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayInfo>,
    #[doc = "The origin of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Additional properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl OperationEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of bot service operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationEntityListResult {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of operations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OperationEntity>,
}
impl azure_core::Continuable for OperationEntityListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationEntityListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties indicating the operation result of an operation on a service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResultsDescription {
    #[doc = "The ID of the operation returned."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the operation result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The status of the operation being performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<operation_results_description::Status>,
    #[doc = "The time that the operation was started."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
}
impl OperationResultsDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation_results_description {
    use super::*;
    #[doc = "The status of the operation being performed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Canceled,
        Succeeded,
        Failed,
        Requested,
        Running,
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
                Self::Canceled => serializer.serialize_unit_variant("Status", 0u32, "Canceled"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 2u32, "Failed"),
                Self::Requested => serializer.serialize_unit_variant("Status", 3u32, "Requested"),
                Self::Running => serializer.serialize_unit_variant("Status", 4u32, "Running"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Outlook channel definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutlookChannel {
    #[serde(flatten)]
    pub channel: Channel,
}
impl OutlookChannel {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}
#[doc = "The Private Endpoint resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "The ARM identifier for Private Endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Private Endpoint Connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub private_link_resource_base: PrivateLinkResourceBase,
    #[doc = "Properties of the PrivateEndpointConnectProperties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of private endpoint connection associated with the specified storage account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "Array of private endpoint connections"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PrivateEndpointConnection>,
}
impl azure_core::Continuable for PrivateEndpointConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl PrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the PrivateEndpointConnectProperties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "The Private Endpoint resource."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "A collection of information about the state of the connection between service consumer and provider."]
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
    #[doc = "Group ids"]
    #[serde(
        rename = "groupIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group_ids: Vec<String>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new(private_link_service_connection_state: PrivateLinkServiceConnectionState) -> Self {
        Self {
            private_endpoint: None,
            private_link_service_connection_state,
            provisioning_state: None,
            group_ids: Vec::new(),
        }
    }
}
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointConnectionProvisioningState")]
pub enum PrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Deleting,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateEndpointConnectionProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateEndpointConnectionProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateEndpointConnectionProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 0u32, "Succeeded"),
            Self::Creating => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 1u32, "Creating"),
            Self::Deleting => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 2u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 3u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The private endpoint connection status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointServiceConnectionStatus")]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateEndpointServiceConnectionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateEndpointServiceConnectionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateEndpointServiceConnectionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pending => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 0u32, "Pending"),
            Self::Approved => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 1u32, "Approved"),
            Self::Rejected => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 2u32, "Rejected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub private_link_resource_base: PrivateLinkResourceBase,
    #[doc = "Properties of a private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common fields that are returned in the response for all BotService Private Link Resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceBase {
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
impl PrivateLinkResourceBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceListResult {
    #[doc = "Array of private link resources"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PrivateLinkResource>,
}
impl PrivateLinkResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "The private link resource group id."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The private link resource required member names."]
    #[serde(
        rename = "requiredMembers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_members: Vec<String>,
    #[doc = "The private link resource Private link DNS zone name."]
    #[serde(
        rename = "requiredZoneNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of information about the state of the connection between service consumer and provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "The private endpoint connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
    #[doc = "The reason for approval/rejection of the connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A message indicating if changes on the service provider require any updates on the consumer."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Specifies the resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Specifies the name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Specifies the type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Contains resource tags defined as key/value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The SKU of the cognitive services account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Indicates the type of bot service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    #[doc = "Entity Tag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Entity zones"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub zones: Vec<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SearchAssistant definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchAssistant {
    #[serde(flatten)]
    pub channel: Channel,
}
impl SearchAssistant {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
}
#[doc = "Service Provider Definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceProvider {
    #[doc = "The Object used to describe a Service Provider supported by Bot Service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceProviderProperties>,
}
impl ServiceProvider {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Extra Parameters specific to each Service Provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceProviderParameter {
    #[doc = "Name of the Service Provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the Service Provider"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Display Name of the Service Provider"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Description of the Service Provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Help Url for the  Service Provider"]
    #[serde(rename = "helpUrl", default, skip_serializing_if = "Option::is_none")]
    pub help_url: Option<String>,
    #[doc = "Default Name for the Service Provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[doc = "Meta data for the Service Provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<service_provider_parameter::Metadata>,
}
impl ServiceProviderParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod service_provider_parameter {
    use super::*;
    #[doc = "Meta data for the Service Provider"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Metadata {
        #[doc = "the constraints of the bot meta data."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub constraints: Option<metadata::Constraints>,
    }
    impl Metadata {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod metadata {
        use super::*;
        #[doc = "the constraints of the bot meta data."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Constraints {
            #[doc = "Whether required the constraints of the bot meta data."]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub required: Option<bool>,
        }
        impl Constraints {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
}
#[doc = "The Object used to describe a Service Provider supported by Bot Service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceProviderProperties {
    #[doc = "Id for Service Provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display Name of the Service Provider"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Name of the Service Provider"]
    #[serde(rename = "serviceProviderName", default, skip_serializing_if = "Option::is_none")]
    pub service_provider_name: Option<String>,
    #[doc = "URL of Dev Portal"]
    #[serde(rename = "devPortalUrl", default, skip_serializing_if = "Option::is_none")]
    pub dev_portal_url: Option<String>,
    #[doc = "The URL of icon"]
    #[serde(rename = "iconUrl", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[doc = "The list of parameters for the Service Provider"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parameters: Vec<ServiceProviderParameter>,
}
impl ServiceProviderProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of bot service providers response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceProviderResponseList {
    #[doc = "The link used to get the next page of bot service providers."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Gets the list of bot service providers and their properties."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ServiceProvider>,
}
impl ServiceProviderResponseList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A site for the channel"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Site {
    #[doc = "Tenant Id"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Site Id"]
    #[serde(rename = "siteId", default, skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    #[doc = "Site name"]
    #[serde(rename = "siteName")]
    pub site_name: String,
    #[doc = "Primary key. Value only returned through POST to the action Channel List API, otherwise empty."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "Secondary key. Value only returned through POST to the action Channel List API, otherwise empty."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key2: Option<String>,
    #[doc = "Whether this site is enabled for DirectLine channel"]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    #[doc = "Whether this site is token enabled for channel"]
    #[serde(rename = "isTokenEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_token_enabled: Option<bool>,
    #[doc = "Whether this site is EndpointParameters enabled for channel"]
    #[serde(rename = "isEndpointParametersEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_endpoint_parameters_enabled: Option<bool>,
    #[doc = "Whether this site is disabled detailed logging for"]
    #[serde(rename = "isDetailedLoggingEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_detailed_logging_enabled: Option<bool>,
    #[doc = "Whether this site is enabled for block user upload."]
    #[serde(rename = "isBlockUserUploadEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_block_user_upload_enabled: Option<bool>,
    #[doc = "Whether this no-storage site is disabled detailed logging for"]
    #[serde(rename = "isNoStorageEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_no_storage_enabled: Option<bool>,
    #[doc = "Entity Tag"]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "DirectLine application id"]
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[doc = "Whether this site is enabled for Bot Framework V1 protocol."]
    #[serde(rename = "isV1Enabled", default, skip_serializing_if = "Option::is_none")]
    pub is_v1_enabled: Option<bool>,
    #[doc = "Whether this site is enabled for Bot Framework V3 protocol."]
    #[serde(rename = "isV3Enabled", default, skip_serializing_if = "Option::is_none")]
    pub is_v3_enabled: Option<bool>,
    #[doc = "Whether this site is enabled for authentication with Bot Framework."]
    #[serde(rename = "isSecureSiteEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_secure_site_enabled: Option<bool>,
    #[doc = "List of Trusted Origin URLs for this site. This field is applicable only if isSecureSiteEnabled is True."]
    #[serde(
        rename = "trustedOrigins",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub trusted_origins: Vec<String>,
    #[doc = "Whether this site is enabled for Webchat Speech"]
    #[serde(rename = "isWebChatSpeechEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_web_chat_speech_enabled: Option<bool>,
    #[doc = "Whether this site is enabled for preview versions of Webchat"]
    #[serde(rename = "isWebchatPreviewEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_webchat_preview_enabled: Option<bool>,
}
impl Site {
    pub fn new(site_name: String, is_enabled: bool) -> Self {
        Self {
            tenant_id: None,
            site_id: None,
            site_name,
            key: None,
            key2: None,
            is_enabled,
            is_token_enabled: None,
            is_endpoint_parameters_enabled: None,
            is_detailed_logging_enabled: None,
            is_block_user_upload_enabled: None,
            is_no_storage_enabled: None,
            e_tag: None,
            app_id: None,
            is_v1_enabled: None,
            is_v3_enabled: None,
            is_secure_site_enabled: None,
            trusted_origins: Vec::new(),
            is_web_chat_speech_enabled: None,
            is_webchat_preview_enabled: None,
        }
    }
}
#[doc = "Site information for WebChat or DirectLine Channels to identify which site to regenerate keys for."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SiteInfo {
    #[doc = "The site name"]
    #[serde(rename = "siteName")]
    pub site_name: String,
    #[doc = "Determines which key is to be regenerated"]
    pub key: site_info::Key,
}
impl SiteInfo {
    pub fn new(site_name: String, key: site_info::Key) -> Self {
        Self { site_name, key }
    }
}
pub mod site_info {
    use super::*;
    #[doc = "Determines which key is to be regenerated"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Key {
        #[serde(rename = "key1")]
        Key1,
        #[serde(rename = "key2")]
        Key2,
    }
}
#[doc = "The SKU of the cognitive services account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The name of SKU."]
    pub name: SkuName,
    #[doc = "Gets the sku tier. This is based on the SKU name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
}
impl Sku {
    pub fn new(name: SkuName) -> Self {
        Self { name, tier: None }
    }
}
pub mod sku {
    use super::*;
    #[doc = "Gets the sku tier. This is based on the SKU name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Free,
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
                Self::Free => serializer.serialize_unit_variant("Tier", 0u32, "Free"),
                Self::Standard => serializer.serialize_unit_variant("Tier", 1u32, "Standard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The name of SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SkuName")]
pub enum SkuName {
    F0,
    S1,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SkuName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SkuName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SkuName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::F0 => serializer.serialize_unit_variant("SkuName", 0u32, "F0"),
            Self::S1 => serializer.serialize_unit_variant("SkuName", 1u32, "S1"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Skype channel definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkypeChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[doc = "The parameters to provide for the Microsoft Teams channel."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SkypeChannelProperties>,
}
impl SkypeChannel {
    pub fn new(channel: Channel) -> Self {
        Self { channel, properties: None }
    }
}
#[doc = "The parameters to provide for the Microsoft Teams channel."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkypeChannelProperties {
    #[doc = "Enable messaging for Skype channel"]
    #[serde(rename = "enableMessaging", default, skip_serializing_if = "Option::is_none")]
    pub enable_messaging: Option<bool>,
    #[doc = "Enable media cards for Skype channel"]
    #[serde(rename = "enableMediaCards", default, skip_serializing_if = "Option::is_none")]
    pub enable_media_cards: Option<bool>,
    #[doc = "Enable video for Skype channel"]
    #[serde(rename = "enableVideo", default, skip_serializing_if = "Option::is_none")]
    pub enable_video: Option<bool>,
    #[doc = "Enable calling for Skype channel"]
    #[serde(rename = "enableCalling", default, skip_serializing_if = "Option::is_none")]
    pub enable_calling: Option<bool>,
    #[doc = "Enable screen sharing for Skype channel"]
    #[serde(rename = "enableScreenSharing", default, skip_serializing_if = "Option::is_none")]
    pub enable_screen_sharing: Option<bool>,
    #[doc = "Enable groups for Skype channel"]
    #[serde(rename = "enableGroups", default, skip_serializing_if = "Option::is_none")]
    pub enable_groups: Option<bool>,
    #[doc = "Group mode for Skype channel"]
    #[serde(rename = "groupsMode", default, skip_serializing_if = "Option::is_none")]
    pub groups_mode: Option<String>,
    #[doc = "Calling web hook for Skype channel"]
    #[serde(rename = "callingWebHook", default, skip_serializing_if = "Option::is_none")]
    pub calling_web_hook: Option<String>,
    #[doc = "Incoming call route for Skype channel"]
    #[serde(rename = "incomingCallRoute", default, skip_serializing_if = "Option::is_none")]
    pub incoming_call_route: Option<String>,
    #[doc = "Whether this channel is enabled for the bot"]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
impl SkypeChannelProperties {
    pub fn new(is_enabled: bool) -> Self {
        Self {
            enable_messaging: None,
            enable_media_cards: None,
            enable_video: None,
            enable_calling: None,
            enable_screen_sharing: None,
            enable_groups: None,
            groups_mode: None,
            calling_web_hook: None,
            incoming_call_route: None,
            is_enabled,
        }
    }
}
#[doc = "Slack channel definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SlackChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[doc = "The parameters to provide for the Slack channel."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SlackChannelProperties>,
}
impl SlackChannel {
    pub fn new(channel: Channel) -> Self {
        Self { channel, properties: None }
    }
}
#[doc = "The parameters to provide for the Slack channel."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SlackChannelProperties {
    #[doc = "The Slack client id"]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "The Slack client secret. Value only returned through POST to the action Channel List API, otherwise empty."]
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[doc = "The Slack verification token. Value only returned through POST to the action Channel List API, otherwise empty."]
    #[serde(rename = "verificationToken", default, skip_serializing_if = "Option::is_none")]
    pub verification_token: Option<String>,
    #[doc = "The Slack permission scopes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<String>,
    #[doc = "The Slack landing page Url"]
    #[serde(rename = "landingPageUrl", default, skip_serializing_if = "Option::is_none")]
    pub landing_page_url: Option<String>,
    #[doc = "The Slack redirect action"]
    #[serde(rename = "redirectAction", default, skip_serializing_if = "Option::is_none")]
    pub redirect_action: Option<String>,
    #[doc = "The Sms auth token"]
    #[serde(rename = "lastSubmissionId", default, skip_serializing_if = "Option::is_none")]
    pub last_submission_id: Option<String>,
    #[doc = "Whether to register the settings before OAuth validation is performed. Recommended to True."]
    #[serde(rename = "registerBeforeOAuthFlow", default, skip_serializing_if = "Option::is_none")]
    pub register_before_o_auth_flow: Option<bool>,
    #[doc = "Whether this channel is validated for the bot"]
    #[serde(rename = "IsValidated", default, skip_serializing_if = "Option::is_none")]
    pub is_validated: Option<bool>,
    #[doc = "The Slack signing secret."]
    #[serde(rename = "signingSecret", default, skip_serializing_if = "Option::is_none")]
    pub signing_secret: Option<String>,
    #[doc = "Whether this channel is enabled for the bot"]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
impl SlackChannelProperties {
    pub fn new(is_enabled: bool) -> Self {
        Self {
            client_id: None,
            client_secret: None,
            verification_token: None,
            scopes: None,
            landing_page_url: None,
            redirect_action: None,
            last_submission_id: None,
            register_before_o_auth_flow: None,
            is_validated: None,
            signing_secret: None,
            is_enabled,
        }
    }
}
#[doc = "Sms channel definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[doc = "The parameters to provide for the Sms channel."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SmsChannelProperties>,
}
impl SmsChannel {
    pub fn new(channel: Channel) -> Self {
        Self { channel, properties: None }
    }
}
#[doc = "The parameters to provide for the Sms channel."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsChannelProperties {
    #[doc = "The Sms phone"]
    pub phone: String,
    #[doc = "The Sms account SID. Value only returned through POST to the action Channel List API, otherwise empty."]
    #[serde(rename = "accountSID")]
    pub account_sid: String,
    #[doc = "The Sms auth token. Value only returned through POST to the action Channel List API, otherwise empty."]
    #[serde(rename = "authToken", default, skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[doc = "Whether this channel is validated for the bot"]
    #[serde(rename = "isValidated", default, skip_serializing_if = "Option::is_none")]
    pub is_validated: Option<bool>,
    #[doc = "Whether this channel is enabled for the bot"]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
impl SmsChannelProperties {
    pub fn new(phone: String, account_sid: String, is_enabled: bool) -> Self {
        Self {
            phone,
            account_sid,
            auth_token: None,
            is_validated: None,
            is_enabled,
        }
    }
}
#[doc = "Telegram channel definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TelegramChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[doc = "The parameters to provide for the Telegram channel."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TelegramChannelProperties>,
}
impl TelegramChannel {
    pub fn new(channel: Channel) -> Self {
        Self { channel, properties: None }
    }
}
#[doc = "The parameters to provide for the Telegram channel."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TelegramChannelProperties {
    #[doc = "The Telegram access token. Value only returned through POST to the action Channel List API, otherwise empty."]
    #[serde(rename = "accessToken", default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[doc = "Whether this channel is validated for the bot"]
    #[serde(rename = "isValidated", default, skip_serializing_if = "Option::is_none")]
    pub is_validated: Option<bool>,
    #[doc = "Whether this channel is enabled for the bot"]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
impl TelegramChannelProperties {
    pub fn new(is_enabled: bool) -> Self {
        Self {
            access_token: None,
            is_validated: None,
            is_enabled,
        }
    }
}
#[doc = "Telephony channel definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TelephonyChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[doc = "The parameters to provide for the Direct Line channel."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TelephonyChannelProperties>,
}
impl TelephonyChannel {
    pub fn new(channel: Channel) -> Self {
        Self { channel, properties: None }
    }
}
#[doc = "The parameters to provide for the Direct Line channel."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TelephonyChannelProperties {
    #[doc = "The list of Telephony phone numbers"]
    #[serde(
        rename = "phoneNumbers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub phone_numbers: Vec<TelephonyPhoneNumbers>,
    #[doc = "The list of Telephony api configuration"]
    #[serde(
        rename = "apiConfigurations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub api_configurations: Vec<TelephonyChannelResourceApiConfiguration>,
    #[doc = "The extensionKey1"]
    #[serde(rename = "cognitiveServiceSubscriptionKey", default, skip_serializing_if = "Option::is_none")]
    pub cognitive_service_subscription_key: Option<String>,
    #[doc = "The extensionKey2"]
    #[serde(rename = "cognitiveServiceRegion", default, skip_serializing_if = "Option::is_none")]
    pub cognitive_service_region: Option<String>,
    #[doc = "The default locale of the channel"]
    #[serde(rename = "defaultLocale", default, skip_serializing_if = "Option::is_none")]
    pub default_locale: Option<String>,
    #[doc = "The premium SKU applied to the channel"]
    #[serde(rename = "premiumSKU", default, skip_serializing_if = "Option::is_none")]
    pub premium_sku: Option<String>,
    #[doc = "Whether the channel is enabled"]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}
impl TelephonyChannelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A resource Api configuration for the Telephony channel"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TelephonyChannelResourceApiConfiguration {
    #[doc = "The id of config."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The provider name."]
    #[serde(rename = "providerName", default, skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[doc = "The cognitive service subscription key."]
    #[serde(rename = "cognitiveServiceSubscriptionKey", default, skip_serializing_if = "Option::is_none")]
    pub cognitive_service_subscription_key: Option<String>,
    #[doc = "The cognitive service region."]
    #[serde(rename = "cognitiveServiceRegion", default, skip_serializing_if = "Option::is_none")]
    pub cognitive_service_region: Option<String>,
    #[doc = "The cognitive service resourceId."]
    #[serde(rename = "cognitiveServiceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub cognitive_service_resource_id: Option<String>,
    #[doc = "The default locale."]
    #[serde(rename = "defaultLocale", default, skip_serializing_if = "Option::is_none")]
    pub default_locale: Option<String>,
}
impl TelephonyChannelResourceApiConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A telephone number for the Telephony channel"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TelephonyPhoneNumbers {
    #[doc = "The element id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The phone number."]
    #[serde(rename = "phoneNumber", default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[doc = "The endpoint of ACS."]
    #[serde(rename = "acsEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub acs_endpoint: Option<String>,
    #[doc = "The secret of ACS."]
    #[serde(rename = "acsSecret", default, skip_serializing_if = "Option::is_none")]
    pub acs_secret: Option<String>,
    #[doc = "The resource id of ACS."]
    #[serde(rename = "acsResourceId", default, skip_serializing_if = "Option::is_none")]
    pub acs_resource_id: Option<String>,
    #[doc = "The subscription key of cognitive service."]
    #[serde(rename = "cognitiveServiceSubscriptionKey", default, skip_serializing_if = "Option::is_none")]
    pub cognitive_service_subscription_key: Option<String>,
    #[doc = "The service region of cognitive service."]
    #[serde(rename = "cognitiveServiceRegion", default, skip_serializing_if = "Option::is_none")]
    pub cognitive_service_region: Option<String>,
    #[doc = "The resource id of cognitive service."]
    #[serde(rename = "cognitiveServiceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub cognitive_service_resource_id: Option<String>,
    #[doc = "The default locale of the phone number."]
    #[serde(rename = "defaultLocale", default, skip_serializing_if = "Option::is_none")]
    pub default_locale: Option<String>,
    #[doc = "Optional Property that will determine the offering type of the phone."]
    #[serde(rename = "offerType", default, skip_serializing_if = "Option::is_none")]
    pub offer_type: Option<String>,
}
impl TelephonyPhoneNumbers {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Web Chat channel definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebChatChannel {
    #[serde(flatten)]
    pub channel: Channel,
    #[doc = "The parameters to provide for the Web Chat channel."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebChatChannelProperties>,
}
impl WebChatChannel {
    pub fn new(channel: Channel) -> Self {
        Self { channel, properties: None }
    }
}
#[doc = "The parameters to provide for the Web Chat channel."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WebChatChannelProperties {
    #[doc = "Web chat control embed code"]
    #[serde(rename = "webChatEmbedCode", default, skip_serializing_if = "Option::is_none")]
    pub web_chat_embed_code: Option<String>,
    #[doc = "The list of Web Chat sites"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sites: Vec<WebChatSite>,
}
impl WebChatChannelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A site for the Webchat channel"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebChatSite {
    #[serde(flatten)]
    pub site: Site,
}
impl WebChatSite {
    pub fn new(site: Site) -> Self {
        Self { site }
    }
}
