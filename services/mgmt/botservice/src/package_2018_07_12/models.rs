#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
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
    #[doc = "Microsoft App Id for the bot"]
    #[serde(rename = "msaAppId")]
    pub msa_app_id: String,
    #[doc = "Collection of channels for which the bot is configured"]
    #[serde(rename = "configuredChannels", default, skip_serializing_if = "Vec::is_empty")]
    pub configured_channels: Vec<String>,
    #[doc = "Collection of channels for which the bot is enabled"]
    #[serde(rename = "enabledChannels", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(rename = "luisAppIds", default, skip_serializing_if = "Vec::is_empty")]
    pub luis_app_ids: Vec<String>,
    #[doc = "The LUIS Key"]
    #[serde(rename = "luisKey", default, skip_serializing_if = "Option::is_none")]
    pub luis_key: Option<String>,
}
impl BotProperties {
    pub fn new(display_name: String, endpoint: String, msa_app_id: String) -> Self {
        Self {
            display_name,
            description: None,
            icon_url: None,
            endpoint,
            endpoint_version: None,
            msa_app_id,
            configured_channels: Vec::new(),
            enabled_channels: Vec::new(),
            developer_app_insight_key: None,
            developer_app_insights_api_key: None,
            developer_app_insights_application_id: None,
            luis_app_ids: Vec::new(),
            luis_key: None,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
}
impl Channel {
    pub fn new(channel_name: String) -> Self {
        Self { channel_name }
    }
}
#[doc = "The list of bot service channel operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChannelResponseList {
    #[doc = "The link used to get the next page of bot service channel resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Gets the list of bot service channel results and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ConnectionSettingParameter>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sites: Vec<DirectLineSite>,
}
impl DirectLineChannelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A site for the Direct Line channel"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectLineSite {
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
    #[doc = "Whether this site is enabled for DirectLine channel."]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
    #[doc = "Whether this site is enabled for Bot Framework V1 protocol."]
    #[serde(rename = "isV1Enabled")]
    pub is_v1_enabled: bool,
    #[doc = "Whether this site is enabled for Bot Framework V1 protocol."]
    #[serde(rename = "isV3Enabled")]
    pub is_v3_enabled: bool,
    #[doc = "Whether this site is enabled for authentication with Bot Framework."]
    #[serde(rename = "isSecureSiteEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_secure_site_enabled: Option<bool>,
    #[doc = "List of Trusted Origin URLs for this site. This field is applicable only if isSecureSiteEnabled is True."]
    #[serde(rename = "trustedOrigins", default, skip_serializing_if = "Vec::is_empty")]
    pub trusted_origins: Vec<String>,
}
impl DirectLineSite {
    pub fn new(site_name: String, is_enabled: bool, is_v1_enabled: bool, is_v3_enabled: bool) -> Self {
        Self {
            site_id: None,
            site_name,
            key: None,
            key2: None,
            is_enabled,
            is_v1_enabled,
            is_v3_enabled,
            is_secure_site_enabled: None,
            trusted_origins: Vec::new(),
        }
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
#[doc = "The parameters to provide for the Email channel."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailChannelProperties {
    #[doc = "The email address"]
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    #[doc = "The password for the email address. Value only returned through POST to the action Channel List API, otherwise empty."]
    pub password: String,
    #[doc = "Whether this channel is enabled for the bot"]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
impl EmailChannelProperties {
    pub fn new(email_address: String, password: String, is_enabled: bool) -> Self {
        Self {
            email_address,
            password,
            is_enabled,
        }
    }
}
#[doc = "Enterprise Channel resource definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnterpriseChannel {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The parameters to provide for the Enterprise Channel."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnterpriseChannelProperties>,
}
impl EnterpriseChannel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A request to Bot Service Management to check availability of an Enterprise Channel name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnterpriseChannelCheckNameAvailabilityRequest {
    #[doc = "The name of the Enterprise Channel for which availability needs to be checked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl EnterpriseChannelCheckNameAvailabilityRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A request to Bot Service Management to check availability of an Enterprise Channel name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnterpriseChannelCheckNameAvailabilityResponse {
    #[doc = "Indicates if the Enterprise Channel name is valid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
    #[doc = "Additional information about why a bot name is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl EnterpriseChannelCheckNameAvailabilityResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties specific to an Enterprise Channel Node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnterpriseChannelNode {
    #[doc = "Id of Enterprise Channel Node. This is generated by the Bot Framework."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The current state of the Enterprise Channel Node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<enterprise_channel_node::State>,
    #[doc = "The name of the Enterprise Channel Node."]
    pub name: String,
    #[doc = "The sku of the Enterprise Channel Node."]
    #[serde(rename = "azureSku")]
    pub azure_sku: String,
    #[doc = "The location of the Enterprise Channel Node."]
    #[serde(rename = "azureLocation")]
    pub azure_location: String,
}
impl EnterpriseChannelNode {
    pub fn new(name: String, azure_sku: String, azure_location: String) -> Self {
        Self {
            id: None,
            state: None,
            name,
            azure_sku,
            azure_location,
        }
    }
}
pub mod enterprise_channel_node {
    use super::*;
    #[doc = "The current state of the Enterprise Channel Node."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Creating,
        CreateFailed,
        Started,
        Starting,
        StartFailed,
        Stopped,
        Stopping,
        StopFailed,
        Deleting,
        DeleteFailed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for State {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for State {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for State {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Creating => serializer.serialize_unit_variant("State", 0u32, "Creating"),
                Self::CreateFailed => serializer.serialize_unit_variant("State", 1u32, "CreateFailed"),
                Self::Started => serializer.serialize_unit_variant("State", 2u32, "Started"),
                Self::Starting => serializer.serialize_unit_variant("State", 3u32, "Starting"),
                Self::StartFailed => serializer.serialize_unit_variant("State", 4u32, "StartFailed"),
                Self::Stopped => serializer.serialize_unit_variant("State", 5u32, "Stopped"),
                Self::Stopping => serializer.serialize_unit_variant("State", 6u32, "Stopping"),
                Self::StopFailed => serializer.serialize_unit_variant("State", 7u32, "StopFailed"),
                Self::Deleting => serializer.serialize_unit_variant("State", 8u32, "Deleting"),
                Self::DeleteFailed => serializer.serialize_unit_variant("State", 9u32, "DeleteFailed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The parameters to provide for the Enterprise Channel."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnterpriseChannelProperties {
    #[doc = "The current state of the Enterprise Channel."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<enterprise_channel_properties::State>,
    #[doc = "The nodes associated with the Enterprise Channel."]
    pub nodes: Vec<EnterpriseChannelNode>,
}
impl EnterpriseChannelProperties {
    pub fn new(nodes: Vec<EnterpriseChannelNode>) -> Self {
        Self { state: None, nodes }
    }
}
pub mod enterprise_channel_properties {
    use super::*;
    #[doc = "The current state of the Enterprise Channel."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Creating,
        CreateFailed,
        Started,
        Starting,
        StartFailed,
        Stopped,
        Stopping,
        StopFailed,
        Deleting,
        DeleteFailed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for State {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for State {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for State {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Creating => serializer.serialize_unit_variant("State", 0u32, "Creating"),
                Self::CreateFailed => serializer.serialize_unit_variant("State", 1u32, "CreateFailed"),
                Self::Started => serializer.serialize_unit_variant("State", 2u32, "Started"),
                Self::Starting => serializer.serialize_unit_variant("State", 3u32, "Starting"),
                Self::StartFailed => serializer.serialize_unit_variant("State", 4u32, "StartFailed"),
                Self::Stopped => serializer.serialize_unit_variant("State", 5u32, "Stopped"),
                Self::Stopping => serializer.serialize_unit_variant("State", 6u32, "Stopping"),
                Self::StopFailed => serializer.serialize_unit_variant("State", 7u32, "StopFailed"),
                Self::Deleting => serializer.serialize_unit_variant("State", 8u32, "Deleting"),
                Self::DeleteFailed => serializer.serialize_unit_variant("State", 9u32, "DeleteFailed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The list of  bot service operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnterpriseChannelResponseList {
    #[doc = "The link used to get the next page of bot service resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of Enterprise Channels and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EnterpriseChannel>,
}
impl azure_core::Continuable for EnterpriseChannelResponseList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EnterpriseChannelResponseList {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pages: Vec<FacebookPage>,
    #[doc = "Facebook application id"]
    #[serde(rename = "appId")]
    pub app_id: String,
    #[doc = "Facebook application secret. Value only returned through POST to the action Channel List API, otherwise empty."]
    #[serde(rename = "appSecret")]
    pub app_secret: String,
    #[doc = "Callback Url"]
    #[serde(rename = "callbackUrl", default, skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    #[doc = "Whether this channel is enabled for the bot"]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
impl FacebookChannelProperties {
    pub fn new(app_id: String, app_secret: String, is_enabled: bool) -> Self {
        Self {
            verify_token: None,
            pages: Vec::new(),
            app_id,
            app_secret,
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
    #[serde(rename = "accessToken")]
    pub access_token: String,
}
impl FacebookPage {
    pub fn new(id: String, access_token: String) -> Self {
        Self { id, access_token }
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
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[doc = "Whether this channel is validated for the bot"]
    #[serde(rename = "isValidated", default, skip_serializing_if = "Option::is_none")]
    pub is_validated: Option<bool>,
    #[doc = "Whether this channel is enabled for the bot"]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
impl KikChannelProperties {
    pub fn new(user_name: String, api_key: String, is_enabled: bool) -> Self {
        Self {
            user_name,
            api_key,
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
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
    #[serde(rename = "callingWebHook", default, skip_serializing_if = "Option::is_none")]
    pub calling_web_hook: Option<String>,
    #[doc = "Whether this channel is enabled for the bot"]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
impl MsTeamsChannelProperties {
    pub fn new(is_enabled: bool) -> Self {
        Self {
            enable_calling: None,
            calling_web_hook: None,
            is_enabled,
        }
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[doc = "Entity Tag"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
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
}
impl ServiceProviderParameter {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "Display Name of the Service Provider"]
    #[serde(rename = "serviceProviderName", default, skip_serializing_if = "Option::is_none")]
    pub service_provider_name: Option<String>,
    #[doc = "Display Name of the Service Provider"]
    #[serde(rename = "devPortalUrl", default, skip_serializing_if = "Option::is_none")]
    pub dev_portal_url: Option<String>,
    #[doc = "Display Name of the Service Provider"]
    #[serde(rename = "iconUrl", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[doc = "The list of parameters for the Service Provider"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServiceProvider>,
}
impl ServiceProviderResponseList {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[doc = "The Slack client secret. Value only returned through POST to the action Channel List API, otherwise empty."]
    #[serde(rename = "clientSecret")]
    pub client_secret: String,
    #[doc = "The Slack verification token. Value only returned through POST to the action Channel List API, otherwise empty."]
    #[serde(rename = "verificationToken")]
    pub verification_token: String,
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
    #[serde(rename = "isValidated", default, skip_serializing_if = "Option::is_none")]
    pub is_validated: Option<bool>,
    #[doc = "Whether this channel is enabled for the bot"]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
impl SlackChannelProperties {
    pub fn new(client_id: String, client_secret: String, verification_token: String, is_enabled: bool) -> Self {
        Self {
            client_id,
            client_secret,
            verification_token,
            landing_page_url: None,
            redirect_action: None,
            last_submission_id: None,
            register_before_o_auth_flow: None,
            is_validated: None,
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
    #[serde(rename = "authToken")]
    pub auth_token: String,
    #[doc = "Whether this channel is validated for the bot"]
    #[serde(rename = "isValidated", default, skip_serializing_if = "Option::is_none")]
    pub is_validated: Option<bool>,
    #[doc = "Whether this channel is enabled for the bot"]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
impl SmsChannelProperties {
    pub fn new(phone: String, account_sid: String, auth_token: String, is_enabled: bool) -> Self {
        Self {
            phone,
            account_sid,
            auth_token,
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
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[doc = "Whether this channel is validated for the bot"]
    #[serde(rename = "isValidated", default, skip_serializing_if = "Option::is_none")]
    pub is_validated: Option<bool>,
    #[doc = "Whether this channel is enabled for the bot"]
    #[serde(rename = "isEnabled")]
    pub is_enabled: bool,
}
impl TelegramChannelProperties {
    pub fn new(access_token: String, is_enabled: bool) -> Self {
        Self {
            access_token,
            is_validated: None,
            is_enabled,
        }
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[doc = "Whether this site is enabled for preview versions of Webchat"]
    #[serde(rename = "enablePreview")]
    pub enable_preview: bool,
}
impl WebChatSite {
    pub fn new(site_name: String, is_enabled: bool, enable_preview: bool) -> Self {
        Self {
            site_id: None,
            site_name,
            key: None,
            key2: None,
            is_enabled,
            enable_preview,
        }
    }
}
