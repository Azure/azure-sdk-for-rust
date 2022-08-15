#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The AS2 agreement acknowledgement connection settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct As2AcknowledgementConnectionSettings {
    #[doc = "Indicates whether to ignore mismatch in certificate name."]
    #[serde(rename = "ignoreCertificateNameMismatch")]
    pub ignore_certificate_name_mismatch: bool,
    #[doc = "Indicates whether to support HTTP status code 'CONTINUE'."]
    #[serde(rename = "supportHttpStatusCodeContinue")]
    pub support_http_status_code_continue: bool,
    #[doc = "Indicates whether to keep the connection alive."]
    #[serde(rename = "keepHttpConnectionAlive")]
    pub keep_http_connection_alive: bool,
    #[doc = "Indicates whether to unfold the HTTP headers."]
    #[serde(rename = "unfoldHttpHeaders")]
    pub unfold_http_headers: bool,
}
impl As2AcknowledgementConnectionSettings {
    pub fn new(
        ignore_certificate_name_mismatch: bool,
        support_http_status_code_continue: bool,
        keep_http_connection_alive: bool,
        unfold_http_headers: bool,
    ) -> Self {
        Self {
            ignore_certificate_name_mismatch,
            support_http_status_code_continue,
            keep_http_connection_alive,
            unfold_http_headers,
        }
    }
}
#[doc = "The integration account AS2 agreement content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct As2AgreementContent {
    #[doc = "The integration account AS2 one-way agreement."]
    #[serde(rename = "receiveAgreement")]
    pub receive_agreement: As2OneWayAgreement,
    #[doc = "The integration account AS2 one-way agreement."]
    #[serde(rename = "sendAgreement")]
    pub send_agreement: As2OneWayAgreement,
}
impl As2AgreementContent {
    pub fn new(receive_agreement: As2OneWayAgreement, send_agreement: As2OneWayAgreement) -> Self {
        Self {
            receive_agreement,
            send_agreement,
        }
    }
}
#[doc = "The AS2 agreement envelope settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct As2EnvelopeSettings {
    #[doc = "The message content type."]
    #[serde(rename = "messageContentType")]
    pub message_content_type: String,
    #[doc = "The value indicating whether to transmit file name in mime header."]
    #[serde(rename = "transmitFileNameInMimeHeader")]
    pub transmit_file_name_in_mime_header: bool,
    #[doc = "The template for file name."]
    #[serde(rename = "fileNameTemplate")]
    pub file_name_template: String,
    #[doc = "The value indicating whether to suspend message on file name generation error."]
    #[serde(rename = "suspendMessageOnFileNameGenerationError")]
    pub suspend_message_on_file_name_generation_error: bool,
    #[doc = "The value indicating whether to auto generate file name."]
    #[serde(rename = "autogenerateFileName")]
    pub autogenerate_file_name: bool,
}
impl As2EnvelopeSettings {
    pub fn new(
        message_content_type: String,
        transmit_file_name_in_mime_header: bool,
        file_name_template: String,
        suspend_message_on_file_name_generation_error: bool,
        autogenerate_file_name: bool,
    ) -> Self {
        Self {
            message_content_type,
            transmit_file_name_in_mime_header,
            file_name_template,
            suspend_message_on_file_name_generation_error,
            autogenerate_file_name,
        }
    }
}
#[doc = "The AS2 agreement error settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct As2ErrorSettings {
    #[doc = "The value indicating whether to suspend duplicate message."]
    #[serde(rename = "suspendDuplicateMessage")]
    pub suspend_duplicate_message: bool,
    #[doc = "The value indicating whether to resend message If MDN is not received."]
    #[serde(rename = "resendIfMDNNotReceived")]
    pub resend_if_mdn_not_received: bool,
}
impl As2ErrorSettings {
    pub fn new(suspend_duplicate_message: bool, resend_if_mdn_not_received: bool) -> Self {
        Self {
            suspend_duplicate_message,
            resend_if_mdn_not_received,
        }
    }
}
#[doc = "The AS2 agreement mdn settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct As2MdnSettings {
    #[doc = "The value indicating whether to send or request a MDN."]
    #[serde(rename = "needMDN")]
    pub need_mdn: bool,
    #[doc = "The value indicating whether the MDN needs to be signed or not."]
    #[serde(rename = "signMDN")]
    pub sign_mdn: bool,
    #[doc = "The value indicating whether to send the asynchronous MDN."]
    #[serde(rename = "sendMDNAsynchronously")]
    pub send_mdn_asynchronously: bool,
    #[doc = "The receipt delivery URL."]
    #[serde(rename = "receiptDeliveryUrl", default, skip_serializing_if = "Option::is_none")]
    pub receipt_delivery_url: Option<String>,
    #[doc = "The disposition notification to header value."]
    #[serde(rename = "dispositionNotificationTo", default, skip_serializing_if = "Option::is_none")]
    pub disposition_notification_to: Option<String>,
    #[doc = "The value indicating whether to sign the outbound MDN if optional."]
    #[serde(rename = "signOutboundMDNIfOptional")]
    pub sign_outbound_mdn_if_optional: bool,
    #[doc = "The MDN text."]
    #[serde(rename = "mdnText", default, skip_serializing_if = "Option::is_none")]
    pub mdn_text: Option<String>,
    #[doc = "The value indicating whether to send inbound MDN to message box."]
    #[serde(rename = "sendInboundMDNToMessageBox")]
    pub send_inbound_mdn_to_message_box: bool,
    #[doc = "The signing or hashing algorithm."]
    #[serde(rename = "micHashingAlgorithm")]
    pub mic_hashing_algorithm: HashingAlgorithm,
}
impl As2MdnSettings {
    pub fn new(
        need_mdn: bool,
        sign_mdn: bool,
        send_mdn_asynchronously: bool,
        sign_outbound_mdn_if_optional: bool,
        send_inbound_mdn_to_message_box: bool,
        mic_hashing_algorithm: HashingAlgorithm,
    ) -> Self {
        Self {
            need_mdn,
            sign_mdn,
            send_mdn_asynchronously,
            receipt_delivery_url: None,
            disposition_notification_to: None,
            sign_outbound_mdn_if_optional,
            mdn_text: None,
            send_inbound_mdn_to_message_box,
            mic_hashing_algorithm,
        }
    }
}
#[doc = "The AS2 agreement message connection settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct As2MessageConnectionSettings {
    #[doc = "The value indicating whether to ignore mismatch in certificate name."]
    #[serde(rename = "ignoreCertificateNameMismatch")]
    pub ignore_certificate_name_mismatch: bool,
    #[doc = "The value indicating whether to support HTTP status code 'CONTINUE'."]
    #[serde(rename = "supportHttpStatusCodeContinue")]
    pub support_http_status_code_continue: bool,
    #[doc = "The value indicating whether to keep the connection alive."]
    #[serde(rename = "keepHttpConnectionAlive")]
    pub keep_http_connection_alive: bool,
    #[doc = "The value indicating whether to unfold the HTTP headers."]
    #[serde(rename = "unfoldHttpHeaders")]
    pub unfold_http_headers: bool,
}
impl As2MessageConnectionSettings {
    pub fn new(
        ignore_certificate_name_mismatch: bool,
        support_http_status_code_continue: bool,
        keep_http_connection_alive: bool,
        unfold_http_headers: bool,
    ) -> Self {
        Self {
            ignore_certificate_name_mismatch,
            support_http_status_code_continue,
            keep_http_connection_alive,
            unfold_http_headers,
        }
    }
}
#[doc = "The integration account AS2 one-way agreement."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct As2OneWayAgreement {
    #[doc = "The integration account partner's business identity."]
    #[serde(rename = "senderBusinessIdentity")]
    pub sender_business_identity: BusinessIdentity,
    #[doc = "The integration account partner's business identity."]
    #[serde(rename = "receiverBusinessIdentity")]
    pub receiver_business_identity: BusinessIdentity,
    #[doc = "The AS2 agreement protocol settings."]
    #[serde(rename = "protocolSettings")]
    pub protocol_settings: As2ProtocolSettings,
}
impl As2OneWayAgreement {
    pub fn new(
        sender_business_identity: BusinessIdentity,
        receiver_business_identity: BusinessIdentity,
        protocol_settings: As2ProtocolSettings,
    ) -> Self {
        Self {
            sender_business_identity,
            receiver_business_identity,
            protocol_settings,
        }
    }
}
#[doc = "The AS2 agreement protocol settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct As2ProtocolSettings {
    #[doc = "The AS2 agreement message connection settings."]
    #[serde(rename = "messageConnectionSettings")]
    pub message_connection_settings: As2MessageConnectionSettings,
    #[doc = "The AS2 agreement acknowledgement connection settings."]
    #[serde(rename = "acknowledgementConnectionSettings")]
    pub acknowledgement_connection_settings: As2AcknowledgementConnectionSettings,
    #[doc = "The AS2 agreement mdn settings."]
    #[serde(rename = "mdnSettings")]
    pub mdn_settings: As2MdnSettings,
    #[doc = "The AS2 agreement security settings."]
    #[serde(rename = "securitySettings")]
    pub security_settings: As2SecuritySettings,
    #[doc = "The AS2 agreement validation settings."]
    #[serde(rename = "validationSettings")]
    pub validation_settings: As2ValidationSettings,
    #[doc = "The AS2 agreement envelope settings."]
    #[serde(rename = "envelopeSettings")]
    pub envelope_settings: As2EnvelopeSettings,
    #[doc = "The AS2 agreement error settings."]
    #[serde(rename = "errorSettings")]
    pub error_settings: As2ErrorSettings,
}
impl As2ProtocolSettings {
    pub fn new(
        message_connection_settings: As2MessageConnectionSettings,
        acknowledgement_connection_settings: As2AcknowledgementConnectionSettings,
        mdn_settings: As2MdnSettings,
        security_settings: As2SecuritySettings,
        validation_settings: As2ValidationSettings,
        envelope_settings: As2EnvelopeSettings,
        error_settings: As2ErrorSettings,
    ) -> Self {
        Self {
            message_connection_settings,
            acknowledgement_connection_settings,
            mdn_settings,
            security_settings,
            validation_settings,
            envelope_settings,
            error_settings,
        }
    }
}
#[doc = "The AS2 agreement security settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct As2SecuritySettings {
    #[doc = "The value indicating whether to send or request a MDN."]
    #[serde(rename = "overrideGroupSigningCertificate")]
    pub override_group_signing_certificate: bool,
    #[doc = "The name of the signing certificate."]
    #[serde(rename = "signingCertificateName", default, skip_serializing_if = "Option::is_none")]
    pub signing_certificate_name: Option<String>,
    #[doc = "The name of the encryption certificate."]
    #[serde(rename = "encryptionCertificateName", default, skip_serializing_if = "Option::is_none")]
    pub encryption_certificate_name: Option<String>,
    #[doc = "The value indicating whether to enable NRR for inbound encoded messages."]
    #[serde(rename = "enableNRRForInboundEncodedMessages")]
    pub enable_nrr_for_inbound_encoded_messages: bool,
    #[doc = "The value indicating whether to enable NRR for inbound decoded messages."]
    #[serde(rename = "enableNRRForInboundDecodedMessages")]
    pub enable_nrr_for_inbound_decoded_messages: bool,
    #[doc = "The value indicating whether to enable NRR for outbound MDN."]
    #[serde(rename = "enableNRRForOutboundMDN")]
    pub enable_nrr_for_outbound_mdn: bool,
    #[doc = "The value indicating whether to enable NRR for outbound encoded messages."]
    #[serde(rename = "enableNRRForOutboundEncodedMessages")]
    pub enable_nrr_for_outbound_encoded_messages: bool,
    #[doc = "The value indicating whether to enable NRR for outbound decoded messages."]
    #[serde(rename = "enableNRRForOutboundDecodedMessages")]
    pub enable_nrr_for_outbound_decoded_messages: bool,
    #[doc = "The value indicating whether to enable NRR for inbound MDN."]
    #[serde(rename = "enableNRRForInboundMDN")]
    pub enable_nrr_for_inbound_mdn: bool,
    #[doc = "The Sha2 algorithm format. Valid values are Sha2, ShaHashSize, ShaHyphenHashSize, Sha2UnderscoreHashSize."]
    #[serde(rename = "sha2AlgorithmFormat", default, skip_serializing_if = "Option::is_none")]
    pub sha2_algorithm_format: Option<String>,
}
impl As2SecuritySettings {
    pub fn new(
        override_group_signing_certificate: bool,
        enable_nrr_for_inbound_encoded_messages: bool,
        enable_nrr_for_inbound_decoded_messages: bool,
        enable_nrr_for_outbound_mdn: bool,
        enable_nrr_for_outbound_encoded_messages: bool,
        enable_nrr_for_outbound_decoded_messages: bool,
        enable_nrr_for_inbound_mdn: bool,
    ) -> Self {
        Self {
            override_group_signing_certificate,
            signing_certificate_name: None,
            encryption_certificate_name: None,
            enable_nrr_for_inbound_encoded_messages,
            enable_nrr_for_inbound_decoded_messages,
            enable_nrr_for_outbound_mdn,
            enable_nrr_for_outbound_encoded_messages,
            enable_nrr_for_outbound_decoded_messages,
            enable_nrr_for_inbound_mdn,
            sha2_algorithm_format: None,
        }
    }
}
#[doc = "The AS2 agreement validation settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct As2ValidationSettings {
    #[doc = "The value indicating whether to override incoming message properties with those in agreement."]
    #[serde(rename = "overrideMessageProperties")]
    pub override_message_properties: bool,
    #[doc = "The value indicating whether the message has to be encrypted."]
    #[serde(rename = "encryptMessage")]
    pub encrypt_message: bool,
    #[doc = "The value indicating whether the message has to be signed."]
    #[serde(rename = "signMessage")]
    pub sign_message: bool,
    #[doc = "The value indicating whether the message has to be compressed."]
    #[serde(rename = "compressMessage")]
    pub compress_message: bool,
    #[doc = "The value indicating whether to check for duplicate message."]
    #[serde(rename = "checkDuplicateMessage")]
    pub check_duplicate_message: bool,
    #[doc = "The number of days to look back for duplicate interchange."]
    #[serde(rename = "interchangeDuplicatesValidityDays")]
    pub interchange_duplicates_validity_days: i32,
    #[doc = "The value indicating whether to check for certificate revocation list on send."]
    #[serde(rename = "checkCertificateRevocationListOnSend")]
    pub check_certificate_revocation_list_on_send: bool,
    #[doc = "The value indicating whether to check for certificate revocation list on receive."]
    #[serde(rename = "checkCertificateRevocationListOnReceive")]
    pub check_certificate_revocation_list_on_receive: bool,
    #[doc = "The encryption algorithm."]
    #[serde(rename = "encryptionAlgorithm")]
    pub encryption_algorithm: EncryptionAlgorithm,
    #[doc = "The signing or hashing algorithm."]
    #[serde(rename = "signingAlgorithm", default, skip_serializing_if = "Option::is_none")]
    pub signing_algorithm: Option<SigningAlgorithm>,
}
impl As2ValidationSettings {
    pub fn new(
        override_message_properties: bool,
        encrypt_message: bool,
        sign_message: bool,
        compress_message: bool,
        check_duplicate_message: bool,
        interchange_duplicates_validity_days: i32,
        check_certificate_revocation_list_on_send: bool,
        check_certificate_revocation_list_on_receive: bool,
        encryption_algorithm: EncryptionAlgorithm,
    ) -> Self {
        Self {
            override_message_properties,
            encrypt_message,
            sign_message,
            compress_message,
            check_duplicate_message,
            interchange_duplicates_validity_days,
            check_certificate_revocation_list_on_send,
            check_certificate_revocation_list_on_receive,
            encryption_algorithm,
            signing_algorithm: None,
        }
    }
}
#[doc = "The integration account agreement content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgreementContent {
    #[doc = "The integration account AS2 agreement content."]
    #[serde(rename = "aS2", default, skip_serializing_if = "Option::is_none")]
    pub a_s2: Option<As2AgreementContent>,
    #[doc = "The X12 agreement content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x12: Option<X12AgreementContent>,
    #[doc = "The Edifact agreement content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edifact: Option<EdifactAgreementContent>,
}
impl AgreementContent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The agreement type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AgreementType {
    NotSpecified,
    #[serde(rename = "AS2")]
    As2,
    X12,
    Edifact,
}
#[doc = "The API deployment parameter metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiDeploymentParameterMetadata {
    #[doc = "The type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Indicates whether its required."]
    #[serde(rename = "isRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[doc = "The display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The Api deployment parameter visibility."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<ApiDeploymentParameterVisibility>,
}
impl ApiDeploymentParameterMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The API deployment parameters metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiDeploymentParameterMetadataSet {
    #[doc = "The API deployment parameter metadata."]
    #[serde(rename = "packageContentLink", default, skip_serializing_if = "Option::is_none")]
    pub package_content_link: Option<ApiDeploymentParameterMetadata>,
    #[doc = "The API deployment parameter metadata."]
    #[serde(rename = "redisCacheConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub redis_cache_connection_string: Option<ApiDeploymentParameterMetadata>,
}
impl ApiDeploymentParameterMetadataSet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Api deployment parameter visibility."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ApiDeploymentParameterVisibility")]
pub enum ApiDeploymentParameterVisibility {
    NotSpecified,
    Default,
    Internal,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ApiDeploymentParameterVisibility {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ApiDeploymentParameterVisibility {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ApiDeploymentParameterVisibility {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("ApiDeploymentParameterVisibility", 0u32, "NotSpecified"),
            Self::Default => serializer.serialize_unit_variant("ApiDeploymentParameterVisibility", 1u32, "Default"),
            Self::Internal => serializer.serialize_unit_variant("ApiDeploymentParameterVisibility", 2u32, "Internal"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The api operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiOperation {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The api operations properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApiOperationPropertiesDefinition>,
}
impl ApiOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Api Operation Annotation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiOperationAnnotation {
    #[doc = "The status annotation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusAnnotation>,
    #[doc = "The family."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "The revision."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
}
impl ApiOperationAnnotation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of managed API operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiOperationListResult {
    #[doc = "The api operation definitions for an API."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApiOperation>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApiOperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ApiOperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The api operations properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiOperationPropertiesDefinition {
    #[doc = "The summary of the api operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[doc = "The description of the api operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The visibility of the api operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[doc = "The trigger type of api operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
    #[doc = "The trigger hint for the api operation."]
    #[serde(rename = "triggerHint", default, skip_serializing_if = "Option::is_none")]
    pub trigger_hint: Option<String>,
    #[doc = "Indicates whether the api operation is pageable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pageable: Option<bool>,
    #[doc = "The Api Operation Annotation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotation: Option<ApiOperationAnnotation>,
    #[doc = "The Api reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api: Option<ApiReference>,
    #[doc = "The swagger schema."]
    #[serde(rename = "inputsDefinition", default, skip_serializing_if = "Option::is_none")]
    pub inputs_definition: Option<SwaggerSchema>,
    #[doc = "The operation responses definition schemas."]
    #[serde(rename = "responsesDefinition", default, skip_serializing_if = "Option::is_none")]
    pub responses_definition: Option<serde_json::Value>,
    #[doc = "Indicates whether the API operation is webhook or not."]
    #[serde(rename = "isWebhook", default, skip_serializing_if = "Option::is_none")]
    pub is_webhook: Option<bool>,
    #[doc = "Indicates whether the API operation is notification or not."]
    #[serde(rename = "isNotification", default, skip_serializing_if = "Option::is_none")]
    pub is_notification: Option<bool>,
}
impl ApiOperationPropertiesDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Api reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiReference {
    #[serde(flatten)]
    pub resource_reference: ResourceReference,
    #[doc = "The display name of the api."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The description of the api."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The icon uri of the api."]
    #[serde(rename = "iconUri", default, skip_serializing_if = "Option::is_none")]
    pub icon_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swagger: Option<Object>,
    #[doc = "The brand color of the api."]
    #[serde(rename = "brandColor", default, skip_serializing_if = "Option::is_none")]
    pub brand_color: Option<String>,
    #[doc = "The Api tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<ApiTier>,
    #[doc = "The resource reference."]
    #[serde(rename = "integrationServiceEnvironment", default, skip_serializing_if = "Option::is_none")]
    pub integration_service_environment: Option<ResourceReference>,
}
impl ApiReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The API backend service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiResourceBackendService {
    #[doc = "The service URL."]
    #[serde(rename = "serviceUrl", default, skip_serializing_if = "Option::is_none")]
    pub service_url: Option<String>,
}
impl ApiResourceBackendService {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Api resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiResourceDefinitions {
    #[doc = "The original swagger url."]
    #[serde(rename = "originalSwaggerUrl", default, skip_serializing_if = "Option::is_none")]
    pub original_swagger_url: Option<String>,
    #[doc = "The modified swagger url."]
    #[serde(rename = "modifiedSwaggerUrl", default, skip_serializing_if = "Option::is_none")]
    pub modified_swagger_url: Option<String>,
}
impl ApiResourceDefinitions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The API general information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiResourceGeneralInformation {
    #[doc = "The icon url."]
    #[serde(rename = "iconUrl", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[doc = "The display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The terms of use url."]
    #[serde(rename = "termsOfUseUrl", default, skip_serializing_if = "Option::is_none")]
    pub terms_of_use_url: Option<String>,
    #[doc = "The release tag."]
    #[serde(rename = "releaseTag", default, skip_serializing_if = "Option::is_none")]
    pub release_tag: Option<String>,
    #[doc = "The Api tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<ApiTier>,
}
impl ApiResourceGeneralInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The api resource metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiResourceMetadata {
    #[doc = "The source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "The brand color."]
    #[serde(rename = "brandColor", default, skip_serializing_if = "Option::is_none")]
    pub brand_color: Option<String>,
    #[doc = "The hide key."]
    #[serde(rename = "hideKey", default, skip_serializing_if = "Option::is_none")]
    pub hide_key: Option<String>,
    #[doc = "The tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(rename = "ApiType", default, skip_serializing_if = "Option::is_none")]
    pub api_type: Option<ApiType>,
    #[doc = "The WSDL service."]
    #[serde(rename = "wsdlService", default, skip_serializing_if = "Option::is_none")]
    pub wsdl_service: Option<WsdlService>,
    #[doc = "The WSDL import method."]
    #[serde(rename = "wsdlImportMethod", default, skip_serializing_if = "Option::is_none")]
    pub wsdl_import_method: Option<WsdlImportMethod>,
    #[doc = "The connection type."]
    #[serde(rename = "connectionType", default, skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    #[doc = "The workflow provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<WorkflowProvisioningState>,
    #[doc = "The API deployment parameters metadata."]
    #[serde(rename = "deploymentParameters", default, skip_serializing_if = "Option::is_none")]
    pub deployment_parameters: Option<ApiDeploymentParameterMetadataSet>,
}
impl ApiResourceMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The API resource policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiResourcePolicies {
    #[doc = "The API level only policies XML as embedded content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[doc = "The content link to the policies."]
    #[serde(rename = "contentLink", default, skip_serializing_if = "Option::is_none")]
    pub content_link: Option<String>,
}
impl ApiResourcePolicies {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The API resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiResourceProperties {
    #[doc = "The name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The connection parameters."]
    #[serde(rename = "connectionParameters", default, skip_serializing_if = "Option::is_none")]
    pub connection_parameters: Option<serde_json::Value>,
    #[doc = "The api resource metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ApiResourceMetadata>,
    #[doc = "The runtime urls."]
    #[serde(rename = "runtimeUrls", default, skip_serializing_if = "Vec::is_empty")]
    pub runtime_urls: Vec<String>,
    #[doc = "The API general information."]
    #[serde(rename = "generalInformation", default, skip_serializing_if = "Option::is_none")]
    pub general_information: Option<ApiResourceGeneralInformation>,
    #[doc = "The capabilities."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<String>,
    #[doc = "The API backend service."]
    #[serde(rename = "backendService", default, skip_serializing_if = "Option::is_none")]
    pub backend_service: Option<ApiResourceBackendService>,
    #[doc = "The API resource policies."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<ApiResourcePolicies>,
    #[doc = "The API definition."]
    #[serde(rename = "apiDefinitionUrl", default, skip_serializing_if = "Option::is_none")]
    pub api_definition_url: Option<String>,
    #[doc = "The Api resource definition."]
    #[serde(rename = "apiDefinitions", default, skip_serializing_if = "Option::is_none")]
    pub api_definitions: Option<ApiResourceDefinitions>,
    #[doc = "The resource reference."]
    #[serde(rename = "integrationServiceEnvironment", default, skip_serializing_if = "Option::is_none")]
    pub integration_service_environment: Option<ResourceReference>,
    #[doc = "The workflow provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<WorkflowProvisioningState>,
    #[doc = "The Api tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<ApiTier>,
}
impl ApiResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Api tier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ApiTier")]
pub enum ApiTier {
    NotSpecified,
    Enterprise,
    Standard,
    Premium,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ApiTier {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ApiTier {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ApiTier {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("ApiTier", 0u32, "NotSpecified"),
            Self::Enterprise => serializer.serialize_unit_variant("ApiTier", 1u32, "Enterprise"),
            Self::Standard => serializer.serialize_unit_variant("ApiTier", 2u32, "Standard"),
            Self::Premium => serializer.serialize_unit_variant("ApiTier", 3u32, "Premium"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ApiType")]
pub enum ApiType {
    NotSpecified,
    Rest,
    Soap,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ApiType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ApiType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ApiType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("ApiType", 0u32, "NotSpecified"),
            Self::Rest => serializer.serialize_unit_variant("ApiType", 1u32, "Rest"),
            Self::Soap => serializer.serialize_unit_variant("ApiType", 2u32, "Soap"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The artifact content properties definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactContentPropertiesDefinition {
    #[serde(flatten)]
    pub artifact_properties: ArtifactProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
    #[doc = "The content type."]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "The content link."]
    #[serde(rename = "contentLink", default, skip_serializing_if = "Option::is_none")]
    pub content_link: Option<ContentLink>,
}
impl ArtifactContentPropertiesDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The artifact properties definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactProperties {
    #[doc = "The artifact creation time."]
    #[serde(rename = "createdTime", with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "The artifact changed time."]
    #[serde(rename = "changedTime", with = "azure_core::date::rfc3339::option")]
    pub changed_time: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
impl ArtifactProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of assembly definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssemblyCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AssemblyDefinition>,
}
impl azure_core::Continuable for AssemblyCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AssemblyCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The assembly definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssemblyDefinition {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The assembly properties definition."]
    pub properties: AssemblyProperties,
}
impl AssemblyDefinition {
    pub fn new(properties: AssemblyProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "The assembly properties definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssemblyProperties {
    #[serde(flatten)]
    pub artifact_content_properties_definition: ArtifactContentPropertiesDefinition,
    #[doc = "The assembly name."]
    #[serde(rename = "assemblyName")]
    pub assembly_name: String,
    #[doc = "The assembly version."]
    #[serde(rename = "assemblyVersion", default, skip_serializing_if = "Option::is_none")]
    pub assembly_version: Option<String>,
    #[doc = "The assembly culture."]
    #[serde(rename = "assemblyCulture", default, skip_serializing_if = "Option::is_none")]
    pub assembly_culture: Option<String>,
    #[doc = "The assembly public key token."]
    #[serde(rename = "assemblyPublicKeyToken", default, skip_serializing_if = "Option::is_none")]
    pub assembly_public_key_token: Option<String>,
}
impl AssemblyProperties {
    pub fn new(assembly_name: String) -> Self {
        Self {
            artifact_content_properties_definition: ArtifactContentPropertiesDefinition::default(),
            assembly_name,
            assembly_version: None,
            assembly_culture: None,
            assembly_public_key_token: None,
        }
    }
}
#[doc = "The Azure async operation state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureAsyncOperationState")]
pub enum AzureAsyncOperationState {
    Failed,
    Succeeded,
    Pending,
    Canceled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureAsyncOperationState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureAsyncOperationState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureAsyncOperationState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Failed => serializer.serialize_unit_variant("AzureAsyncOperationState", 0u32, "Failed"),
            Self::Succeeded => serializer.serialize_unit_variant("AzureAsyncOperationState", 1u32, "Succeeded"),
            Self::Pending => serializer.serialize_unit_variant("AzureAsyncOperationState", 2u32, "Pending"),
            Self::Canceled => serializer.serialize_unit_variant("AzureAsyncOperationState", 3u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The azure resource error info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResourceErrorInfo {
    #[serde(flatten)]
    pub error_info: ErrorInfo,
    #[doc = "The error message."]
    pub message: String,
    #[doc = "The error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<AzureResourceErrorInfo>,
}
impl AzureResourceErrorInfo {
    pub fn new(error_info: ErrorInfo, message: String) -> Self {
        Self {
            error_info,
            message,
            details: Vec::new(),
        }
    }
}
#[doc = "The B2B partner content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct B2bPartnerContent {
    #[doc = "The list of partner business identities."]
    #[serde(rename = "businessIdentities", default, skip_serializing_if = "Vec::is_empty")]
    pub business_identities: Vec<BusinessIdentity>,
}
impl B2bPartnerContent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The batch configuration resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchConfiguration {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The batch configuration properties definition."]
    pub properties: BatchConfigurationProperties,
}
impl BatchConfiguration {
    pub fn new(properties: BatchConfigurationProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A collection of batch configurations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchConfigurationCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BatchConfiguration>,
}
impl azure_core::Continuable for BatchConfigurationCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl BatchConfigurationCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The batch configuration properties definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchConfigurationProperties {
    #[serde(flatten)]
    pub artifact_properties: ArtifactProperties,
    #[doc = "The name of the batch group."]
    #[serde(rename = "batchGroupName")]
    pub batch_group_name: String,
    #[doc = "The batch release criteria."]
    #[serde(rename = "releaseCriteria")]
    pub release_criteria: BatchReleaseCriteria,
    #[doc = "The created time."]
    #[serde(rename = "createdTime", with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "The changed time."]
    #[serde(rename = "changedTime", with = "azure_core::date::rfc3339::option")]
    pub changed_time: Option<time::OffsetDateTime>,
}
impl BatchConfigurationProperties {
    pub fn new(batch_group_name: String, release_criteria: BatchReleaseCriteria) -> Self {
        Self {
            artifact_properties: ArtifactProperties::default(),
            batch_group_name,
            release_criteria,
            created_time: None,
            changed_time: None,
        }
    }
}
#[doc = "The batch release criteria."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BatchReleaseCriteria {
    #[doc = "The message count."]
    #[serde(rename = "messageCount", default, skip_serializing_if = "Option::is_none")]
    pub message_count: Option<i32>,
    #[doc = "The batch size in bytes."]
    #[serde(rename = "batchSize", default, skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[doc = "The workflow trigger recurrence."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<WorkflowTriggerRecurrence>,
}
impl BatchReleaseCriteria {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration account partner's business identity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessIdentity {
    #[doc = "The business identity qualifier e.g. as2identity, ZZ, ZZZ, 31, 32"]
    pub qualifier: String,
    #[doc = "The user defined business identity value."]
    pub value: String,
}
impl BusinessIdentity {
    pub fn new(qualifier: String, value: String) -> Self {
        Self { qualifier, value }
    }
}
#[doc = "The callback url."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CallbackUrl {
    #[doc = "The URL value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl CallbackUrl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The content hash."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContentHash {
    #[doc = "The algorithm of the content hash."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    #[doc = "The value of the content hash."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ContentHash {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The content link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContentLink {
    #[doc = "The content link URI."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "The content version."]
    #[serde(rename = "contentVersion", default, skip_serializing_if = "Option::is_none")]
    pub content_version: Option<String>,
    #[doc = "The content size."]
    #[serde(rename = "contentSize", default, skip_serializing_if = "Option::is_none")]
    pub content_size: Option<i64>,
    #[doc = "The content hash."]
    #[serde(rename = "contentHash", default, skip_serializing_if = "Option::is_none")]
    pub content_hash: Option<ContentHash>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Object>,
}
impl ContentLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The correlation property."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Correlation {
    #[doc = "The client tracking id."]
    #[serde(rename = "clientTrackingId", default, skip_serializing_if = "Option::is_none")]
    pub client_tracking_id: Option<String>,
}
impl Correlation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The day of the week."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}
#[doc = "The Edifact agreement acknowledgement settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdifactAcknowledgementSettings {
    #[doc = "The value indicating whether technical acknowledgement is needed."]
    #[serde(rename = "needTechnicalAcknowledgement")]
    pub need_technical_acknowledgement: bool,
    #[doc = "The value indicating whether to batch the technical acknowledgements."]
    #[serde(rename = "batchTechnicalAcknowledgements")]
    pub batch_technical_acknowledgements: bool,
    #[doc = "The value indicating whether functional acknowledgement is needed."]
    #[serde(rename = "needFunctionalAcknowledgement")]
    pub need_functional_acknowledgement: bool,
    #[doc = "The value indicating whether to batch functional acknowledgements."]
    #[serde(rename = "batchFunctionalAcknowledgements")]
    pub batch_functional_acknowledgements: bool,
    #[doc = "The value indicating whether a loop is needed for valid messages."]
    #[serde(rename = "needLoopForValidMessages")]
    pub need_loop_for_valid_messages: bool,
    #[doc = "The value indicating whether to send synchronous acknowledgement."]
    #[serde(rename = "sendSynchronousAcknowledgement")]
    pub send_synchronous_acknowledgement: bool,
    #[doc = "The acknowledgement control number prefix."]
    #[serde(rename = "acknowledgementControlNumberPrefix", default, skip_serializing_if = "Option::is_none")]
    pub acknowledgement_control_number_prefix: Option<String>,
    #[doc = "The acknowledgement control number suffix."]
    #[serde(rename = "acknowledgementControlNumberSuffix", default, skip_serializing_if = "Option::is_none")]
    pub acknowledgement_control_number_suffix: Option<String>,
    #[doc = "The acknowledgement control number lower bound."]
    #[serde(rename = "acknowledgementControlNumberLowerBound")]
    pub acknowledgement_control_number_lower_bound: i32,
    #[doc = "The acknowledgement control number upper bound."]
    #[serde(rename = "acknowledgementControlNumberUpperBound")]
    pub acknowledgement_control_number_upper_bound: i32,
    #[doc = "The value indicating whether to rollover acknowledgement control number."]
    #[serde(rename = "rolloverAcknowledgementControlNumber")]
    pub rollover_acknowledgement_control_number: bool,
}
impl EdifactAcknowledgementSettings {
    pub fn new(
        need_technical_acknowledgement: bool,
        batch_technical_acknowledgements: bool,
        need_functional_acknowledgement: bool,
        batch_functional_acknowledgements: bool,
        need_loop_for_valid_messages: bool,
        send_synchronous_acknowledgement: bool,
        acknowledgement_control_number_lower_bound: i32,
        acknowledgement_control_number_upper_bound: i32,
        rollover_acknowledgement_control_number: bool,
    ) -> Self {
        Self {
            need_technical_acknowledgement,
            batch_technical_acknowledgements,
            need_functional_acknowledgement,
            batch_functional_acknowledgements,
            need_loop_for_valid_messages,
            send_synchronous_acknowledgement,
            acknowledgement_control_number_prefix: None,
            acknowledgement_control_number_suffix: None,
            acknowledgement_control_number_lower_bound,
            acknowledgement_control_number_upper_bound,
            rollover_acknowledgement_control_number,
        }
    }
}
#[doc = "The Edifact agreement content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdifactAgreementContent {
    #[doc = "The Edifact one way agreement."]
    #[serde(rename = "receiveAgreement")]
    pub receive_agreement: EdifactOneWayAgreement,
    #[doc = "The Edifact one way agreement."]
    #[serde(rename = "sendAgreement")]
    pub send_agreement: EdifactOneWayAgreement,
}
impl EdifactAgreementContent {
    pub fn new(receive_agreement: EdifactOneWayAgreement, send_agreement: EdifactOneWayAgreement) -> Self {
        Self {
            receive_agreement,
            send_agreement,
        }
    }
}
#[doc = "The edifact character set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EdifactCharacterSet")]
pub enum EdifactCharacterSet {
    NotSpecified,
    #[serde(rename = "UNOB")]
    Unob,
    #[serde(rename = "UNOA")]
    Unoa,
    #[serde(rename = "UNOC")]
    Unoc,
    #[serde(rename = "UNOD")]
    Unod,
    #[serde(rename = "UNOE")]
    Unoe,
    #[serde(rename = "UNOF")]
    Unof,
    #[serde(rename = "UNOG")]
    Unog,
    #[serde(rename = "UNOH")]
    Unoh,
    #[serde(rename = "UNOI")]
    Unoi,
    #[serde(rename = "UNOJ")]
    Unoj,
    #[serde(rename = "UNOK")]
    Unok,
    #[serde(rename = "UNOX")]
    Unox,
    #[serde(rename = "UNOY")]
    Unoy,
    #[serde(rename = "KECA")]
    Keca,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EdifactCharacterSet {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EdifactCharacterSet {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EdifactCharacterSet {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("EdifactCharacterSet", 0u32, "NotSpecified"),
            Self::Unob => serializer.serialize_unit_variant("EdifactCharacterSet", 1u32, "UNOB"),
            Self::Unoa => serializer.serialize_unit_variant("EdifactCharacterSet", 2u32, "UNOA"),
            Self::Unoc => serializer.serialize_unit_variant("EdifactCharacterSet", 3u32, "UNOC"),
            Self::Unod => serializer.serialize_unit_variant("EdifactCharacterSet", 4u32, "UNOD"),
            Self::Unoe => serializer.serialize_unit_variant("EdifactCharacterSet", 5u32, "UNOE"),
            Self::Unof => serializer.serialize_unit_variant("EdifactCharacterSet", 6u32, "UNOF"),
            Self::Unog => serializer.serialize_unit_variant("EdifactCharacterSet", 7u32, "UNOG"),
            Self::Unoh => serializer.serialize_unit_variant("EdifactCharacterSet", 8u32, "UNOH"),
            Self::Unoi => serializer.serialize_unit_variant("EdifactCharacterSet", 9u32, "UNOI"),
            Self::Unoj => serializer.serialize_unit_variant("EdifactCharacterSet", 10u32, "UNOJ"),
            Self::Unok => serializer.serialize_unit_variant("EdifactCharacterSet", 11u32, "UNOK"),
            Self::Unox => serializer.serialize_unit_variant("EdifactCharacterSet", 12u32, "UNOX"),
            Self::Unoy => serializer.serialize_unit_variant("EdifactCharacterSet", 13u32, "UNOY"),
            Self::Keca => serializer.serialize_unit_variant("EdifactCharacterSet", 14u32, "KECA"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The edifact decimal indicator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EdifactDecimalIndicator {
    NotSpecified,
    Comma,
    Decimal,
}
#[doc = "The Edifact delimiter override settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdifactDelimiterOverride {
    #[doc = "The message id."]
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[doc = "The message version."]
    #[serde(rename = "messageVersion", default, skip_serializing_if = "Option::is_none")]
    pub message_version: Option<String>,
    #[doc = "The message release."]
    #[serde(rename = "messageRelease", default, skip_serializing_if = "Option::is_none")]
    pub message_release: Option<String>,
    #[doc = "The data element separator."]
    #[serde(rename = "dataElementSeparator")]
    pub data_element_separator: i32,
    #[doc = "The component separator."]
    #[serde(rename = "componentSeparator")]
    pub component_separator: i32,
    #[doc = "The segment terminator."]
    #[serde(rename = "segmentTerminator")]
    pub segment_terminator: i32,
    #[doc = "The repetition separator."]
    #[serde(rename = "repetitionSeparator")]
    pub repetition_separator: i32,
    #[doc = "The segment terminator suffix."]
    #[serde(rename = "segmentTerminatorSuffix")]
    pub segment_terminator_suffix: SegmentTerminatorSuffix,
    #[doc = "The edifact decimal indicator."]
    #[serde(rename = "decimalPointIndicator")]
    pub decimal_point_indicator: EdifactDecimalIndicator,
    #[doc = "The release indicator."]
    #[serde(rename = "releaseIndicator")]
    pub release_indicator: i32,
    #[doc = "The message association assigned code."]
    #[serde(rename = "messageAssociationAssignedCode", default, skip_serializing_if = "Option::is_none")]
    pub message_association_assigned_code: Option<String>,
    #[doc = "The target namespace on which this delimiter settings has to be applied."]
    #[serde(rename = "targetNamespace", default, skip_serializing_if = "Option::is_none")]
    pub target_namespace: Option<String>,
}
impl EdifactDelimiterOverride {
    pub fn new(
        data_element_separator: i32,
        component_separator: i32,
        segment_terminator: i32,
        repetition_separator: i32,
        segment_terminator_suffix: SegmentTerminatorSuffix,
        decimal_point_indicator: EdifactDecimalIndicator,
        release_indicator: i32,
    ) -> Self {
        Self {
            message_id: None,
            message_version: None,
            message_release: None,
            data_element_separator,
            component_separator,
            segment_terminator,
            repetition_separator,
            segment_terminator_suffix,
            decimal_point_indicator,
            release_indicator,
            message_association_assigned_code: None,
            target_namespace: None,
        }
    }
}
#[doc = "The Edifact envelope override settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EdifactEnvelopeOverride {
    #[doc = "The message id on which this envelope settings has to be applied."]
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[doc = "The message version on which this envelope settings has to be applied."]
    #[serde(rename = "messageVersion", default, skip_serializing_if = "Option::is_none")]
    pub message_version: Option<String>,
    #[doc = "The message release version on which this envelope settings has to be applied."]
    #[serde(rename = "messageRelease", default, skip_serializing_if = "Option::is_none")]
    pub message_release: Option<String>,
    #[doc = "The message association assigned code."]
    #[serde(rename = "messageAssociationAssignedCode", default, skip_serializing_if = "Option::is_none")]
    pub message_association_assigned_code: Option<String>,
    #[doc = "The target namespace on which this envelope settings has to be applied."]
    #[serde(rename = "targetNamespace", default, skip_serializing_if = "Option::is_none")]
    pub target_namespace: Option<String>,
    #[doc = "The functional group id."]
    #[serde(rename = "functionalGroupId", default, skip_serializing_if = "Option::is_none")]
    pub functional_group_id: Option<String>,
    #[doc = "The sender application qualifier."]
    #[serde(rename = "senderApplicationQualifier", default, skip_serializing_if = "Option::is_none")]
    pub sender_application_qualifier: Option<String>,
    #[doc = "The sender application id."]
    #[serde(rename = "senderApplicationId", default, skip_serializing_if = "Option::is_none")]
    pub sender_application_id: Option<String>,
    #[doc = "The receiver application qualifier."]
    #[serde(rename = "receiverApplicationQualifier", default, skip_serializing_if = "Option::is_none")]
    pub receiver_application_qualifier: Option<String>,
    #[doc = "The receiver application id."]
    #[serde(rename = "receiverApplicationId", default, skip_serializing_if = "Option::is_none")]
    pub receiver_application_id: Option<String>,
    #[doc = "The controlling agency code."]
    #[serde(rename = "controllingAgencyCode", default, skip_serializing_if = "Option::is_none")]
    pub controlling_agency_code: Option<String>,
    #[doc = "The group header message version."]
    #[serde(rename = "groupHeaderMessageVersion", default, skip_serializing_if = "Option::is_none")]
    pub group_header_message_version: Option<String>,
    #[doc = "The group header message release."]
    #[serde(rename = "groupHeaderMessageRelease", default, skip_serializing_if = "Option::is_none")]
    pub group_header_message_release: Option<String>,
    #[doc = "The association assigned code."]
    #[serde(rename = "associationAssignedCode", default, skip_serializing_if = "Option::is_none")]
    pub association_assigned_code: Option<String>,
    #[doc = "The application password."]
    #[serde(rename = "applicationPassword", default, skip_serializing_if = "Option::is_none")]
    pub application_password: Option<String>,
}
impl EdifactEnvelopeOverride {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Edifact agreement envelope settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdifactEnvelopeSettings {
    #[doc = "The group association assigned code."]
    #[serde(rename = "groupAssociationAssignedCode", default, skip_serializing_if = "Option::is_none")]
    pub group_association_assigned_code: Option<String>,
    #[doc = "The communication agreement id."]
    #[serde(rename = "communicationAgreementId", default, skip_serializing_if = "Option::is_none")]
    pub communication_agreement_id: Option<String>,
    #[doc = "The value indicating whether to apply delimiter string advice."]
    #[serde(rename = "applyDelimiterStringAdvice")]
    pub apply_delimiter_string_advice: bool,
    #[doc = "The value indicating whether to create grouping segments."]
    #[serde(rename = "createGroupingSegments")]
    pub create_grouping_segments: bool,
    #[doc = "The value indicating whether to enable default group headers."]
    #[serde(rename = "enableDefaultGroupHeaders")]
    pub enable_default_group_headers: bool,
    #[doc = "The recipient reference password value."]
    #[serde(rename = "recipientReferencePasswordValue", default, skip_serializing_if = "Option::is_none")]
    pub recipient_reference_password_value: Option<String>,
    #[doc = "The recipient reference password qualifier."]
    #[serde(rename = "recipientReferencePasswordQualifier", default, skip_serializing_if = "Option::is_none")]
    pub recipient_reference_password_qualifier: Option<String>,
    #[doc = "The application reference id."]
    #[serde(rename = "applicationReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub application_reference_id: Option<String>,
    #[doc = "The processing priority code."]
    #[serde(rename = "processingPriorityCode", default, skip_serializing_if = "Option::is_none")]
    pub processing_priority_code: Option<String>,
    #[doc = "The interchange control number lower bound."]
    #[serde(rename = "interchangeControlNumberLowerBound")]
    pub interchange_control_number_lower_bound: i64,
    #[doc = "The interchange control number upper bound."]
    #[serde(rename = "interchangeControlNumberUpperBound")]
    pub interchange_control_number_upper_bound: i64,
    #[doc = "The value indicating whether to rollover interchange control number."]
    #[serde(rename = "rolloverInterchangeControlNumber")]
    pub rollover_interchange_control_number: bool,
    #[doc = "The interchange control number prefix."]
    #[serde(rename = "interchangeControlNumberPrefix", default, skip_serializing_if = "Option::is_none")]
    pub interchange_control_number_prefix: Option<String>,
    #[doc = "The interchange control number suffix."]
    #[serde(rename = "interchangeControlNumberSuffix", default, skip_serializing_if = "Option::is_none")]
    pub interchange_control_number_suffix: Option<String>,
    #[doc = "The sender reverse routing address."]
    #[serde(rename = "senderReverseRoutingAddress", default, skip_serializing_if = "Option::is_none")]
    pub sender_reverse_routing_address: Option<String>,
    #[doc = "The receiver reverse routing address."]
    #[serde(rename = "receiverReverseRoutingAddress", default, skip_serializing_if = "Option::is_none")]
    pub receiver_reverse_routing_address: Option<String>,
    #[doc = "The functional group id."]
    #[serde(rename = "functionalGroupId", default, skip_serializing_if = "Option::is_none")]
    pub functional_group_id: Option<String>,
    #[doc = "The group controlling agency code."]
    #[serde(rename = "groupControllingAgencyCode", default, skip_serializing_if = "Option::is_none")]
    pub group_controlling_agency_code: Option<String>,
    #[doc = "The group message version."]
    #[serde(rename = "groupMessageVersion", default, skip_serializing_if = "Option::is_none")]
    pub group_message_version: Option<String>,
    #[doc = "The group message release."]
    #[serde(rename = "groupMessageRelease", default, skip_serializing_if = "Option::is_none")]
    pub group_message_release: Option<String>,
    #[doc = "The group control number lower bound."]
    #[serde(rename = "groupControlNumberLowerBound")]
    pub group_control_number_lower_bound: i64,
    #[doc = "The group control number upper bound."]
    #[serde(rename = "groupControlNumberUpperBound")]
    pub group_control_number_upper_bound: i64,
    #[doc = "The value indicating whether to rollover group control number."]
    #[serde(rename = "rolloverGroupControlNumber")]
    pub rollover_group_control_number: bool,
    #[doc = "The group control number prefix."]
    #[serde(rename = "groupControlNumberPrefix", default, skip_serializing_if = "Option::is_none")]
    pub group_control_number_prefix: Option<String>,
    #[doc = "The group control number suffix."]
    #[serde(rename = "groupControlNumberSuffix", default, skip_serializing_if = "Option::is_none")]
    pub group_control_number_suffix: Option<String>,
    #[doc = "The group application receiver qualifier."]
    #[serde(rename = "groupApplicationReceiverQualifier", default, skip_serializing_if = "Option::is_none")]
    pub group_application_receiver_qualifier: Option<String>,
    #[doc = "The group application receiver id."]
    #[serde(rename = "groupApplicationReceiverId", default, skip_serializing_if = "Option::is_none")]
    pub group_application_receiver_id: Option<String>,
    #[doc = "The group application sender qualifier."]
    #[serde(rename = "groupApplicationSenderQualifier", default, skip_serializing_if = "Option::is_none")]
    pub group_application_sender_qualifier: Option<String>,
    #[doc = "The group application sender id."]
    #[serde(rename = "groupApplicationSenderId", default, skip_serializing_if = "Option::is_none")]
    pub group_application_sender_id: Option<String>,
    #[doc = "The group application password."]
    #[serde(rename = "groupApplicationPassword", default, skip_serializing_if = "Option::is_none")]
    pub group_application_password: Option<String>,
    #[doc = "The value indicating whether to overwrite existing transaction set control number."]
    #[serde(rename = "overwriteExistingTransactionSetControlNumber")]
    pub overwrite_existing_transaction_set_control_number: bool,
    #[doc = "The transaction set control number prefix."]
    #[serde(rename = "transactionSetControlNumberPrefix", default, skip_serializing_if = "Option::is_none")]
    pub transaction_set_control_number_prefix: Option<String>,
    #[doc = "The transaction set control number suffix."]
    #[serde(rename = "transactionSetControlNumberSuffix", default, skip_serializing_if = "Option::is_none")]
    pub transaction_set_control_number_suffix: Option<String>,
    #[doc = "The transaction set control number lower bound."]
    #[serde(rename = "transactionSetControlNumberLowerBound")]
    pub transaction_set_control_number_lower_bound: i64,
    #[doc = "The transaction set control number upper bound."]
    #[serde(rename = "transactionSetControlNumberUpperBound")]
    pub transaction_set_control_number_upper_bound: i64,
    #[doc = "The value indicating whether to rollover transaction set control number."]
    #[serde(rename = "rolloverTransactionSetControlNumber")]
    pub rollover_transaction_set_control_number: bool,
    #[doc = "The value indicating whether the message is a test interchange."]
    #[serde(rename = "isTestInterchange")]
    pub is_test_interchange: bool,
    #[doc = "The sender internal identification."]
    #[serde(rename = "senderInternalIdentification", default, skip_serializing_if = "Option::is_none")]
    pub sender_internal_identification: Option<String>,
    #[doc = "The sender internal sub identification."]
    #[serde(rename = "senderInternalSubIdentification", default, skip_serializing_if = "Option::is_none")]
    pub sender_internal_sub_identification: Option<String>,
    #[doc = "The receiver internal identification."]
    #[serde(rename = "receiverInternalIdentification", default, skip_serializing_if = "Option::is_none")]
    pub receiver_internal_identification: Option<String>,
    #[doc = "The receiver internal sub identification."]
    #[serde(rename = "receiverInternalSubIdentification", default, skip_serializing_if = "Option::is_none")]
    pub receiver_internal_sub_identification: Option<String>,
}
impl EdifactEnvelopeSettings {
    pub fn new(
        apply_delimiter_string_advice: bool,
        create_grouping_segments: bool,
        enable_default_group_headers: bool,
        interchange_control_number_lower_bound: i64,
        interchange_control_number_upper_bound: i64,
        rollover_interchange_control_number: bool,
        group_control_number_lower_bound: i64,
        group_control_number_upper_bound: i64,
        rollover_group_control_number: bool,
        overwrite_existing_transaction_set_control_number: bool,
        transaction_set_control_number_lower_bound: i64,
        transaction_set_control_number_upper_bound: i64,
        rollover_transaction_set_control_number: bool,
        is_test_interchange: bool,
    ) -> Self {
        Self {
            group_association_assigned_code: None,
            communication_agreement_id: None,
            apply_delimiter_string_advice,
            create_grouping_segments,
            enable_default_group_headers,
            recipient_reference_password_value: None,
            recipient_reference_password_qualifier: None,
            application_reference_id: None,
            processing_priority_code: None,
            interchange_control_number_lower_bound,
            interchange_control_number_upper_bound,
            rollover_interchange_control_number,
            interchange_control_number_prefix: None,
            interchange_control_number_suffix: None,
            sender_reverse_routing_address: None,
            receiver_reverse_routing_address: None,
            functional_group_id: None,
            group_controlling_agency_code: None,
            group_message_version: None,
            group_message_release: None,
            group_control_number_lower_bound,
            group_control_number_upper_bound,
            rollover_group_control_number,
            group_control_number_prefix: None,
            group_control_number_suffix: None,
            group_application_receiver_qualifier: None,
            group_application_receiver_id: None,
            group_application_sender_qualifier: None,
            group_application_sender_id: None,
            group_application_password: None,
            overwrite_existing_transaction_set_control_number,
            transaction_set_control_number_prefix: None,
            transaction_set_control_number_suffix: None,
            transaction_set_control_number_lower_bound,
            transaction_set_control_number_upper_bound,
            rollover_transaction_set_control_number,
            is_test_interchange,
            sender_internal_identification: None,
            sender_internal_sub_identification: None,
            receiver_internal_identification: None,
            receiver_internal_sub_identification: None,
        }
    }
}
#[doc = "The Edifact agreement framing settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdifactFramingSettings {
    #[doc = "The service code list directory version."]
    #[serde(rename = "serviceCodeListDirectoryVersion", default, skip_serializing_if = "Option::is_none")]
    pub service_code_list_directory_version: Option<String>,
    #[doc = "The character encoding."]
    #[serde(rename = "characterEncoding", default, skip_serializing_if = "Option::is_none")]
    pub character_encoding: Option<String>,
    #[doc = "The protocol version."]
    #[serde(rename = "protocolVersion")]
    pub protocol_version: i32,
    #[doc = "The data element separator."]
    #[serde(rename = "dataElementSeparator")]
    pub data_element_separator: i32,
    #[doc = "The component separator."]
    #[serde(rename = "componentSeparator")]
    pub component_separator: i32,
    #[doc = "The segment terminator."]
    #[serde(rename = "segmentTerminator")]
    pub segment_terminator: i32,
    #[doc = "The release indicator."]
    #[serde(rename = "releaseIndicator")]
    pub release_indicator: i32,
    #[doc = "The repetition separator."]
    #[serde(rename = "repetitionSeparator")]
    pub repetition_separator: i32,
    #[doc = "The edifact character set."]
    #[serde(rename = "characterSet")]
    pub character_set: EdifactCharacterSet,
    #[doc = "The edifact decimal indicator."]
    #[serde(rename = "decimalPointIndicator")]
    pub decimal_point_indicator: EdifactDecimalIndicator,
    #[doc = "The segment terminator suffix."]
    #[serde(rename = "segmentTerminatorSuffix")]
    pub segment_terminator_suffix: SegmentTerminatorSuffix,
}
impl EdifactFramingSettings {
    pub fn new(
        protocol_version: i32,
        data_element_separator: i32,
        component_separator: i32,
        segment_terminator: i32,
        release_indicator: i32,
        repetition_separator: i32,
        character_set: EdifactCharacterSet,
        decimal_point_indicator: EdifactDecimalIndicator,
        segment_terminator_suffix: SegmentTerminatorSuffix,
    ) -> Self {
        Self {
            service_code_list_directory_version: None,
            character_encoding: None,
            protocol_version,
            data_element_separator,
            component_separator,
            segment_terminator,
            release_indicator,
            repetition_separator,
            character_set,
            decimal_point_indicator,
            segment_terminator_suffix,
        }
    }
}
#[doc = "The Edifact message filter for odata query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdifactMessageFilter {
    #[doc = "The message filter type."]
    #[serde(rename = "messageFilterType")]
    pub message_filter_type: MessageFilterType,
}
impl EdifactMessageFilter {
    pub fn new(message_filter_type: MessageFilterType) -> Self {
        Self { message_filter_type }
    }
}
#[doc = "The Edifact message identifier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdifactMessageIdentifier {
    #[doc = "The message id on which this envelope settings has to be applied."]
    #[serde(rename = "messageId")]
    pub message_id: String,
}
impl EdifactMessageIdentifier {
    pub fn new(message_id: String) -> Self {
        Self { message_id }
    }
}
#[doc = "The Edifact one way agreement."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdifactOneWayAgreement {
    #[doc = "The integration account partner's business identity."]
    #[serde(rename = "senderBusinessIdentity")]
    pub sender_business_identity: BusinessIdentity,
    #[doc = "The integration account partner's business identity."]
    #[serde(rename = "receiverBusinessIdentity")]
    pub receiver_business_identity: BusinessIdentity,
    #[doc = "The Edifact agreement protocol settings."]
    #[serde(rename = "protocolSettings")]
    pub protocol_settings: EdifactProtocolSettings,
}
impl EdifactOneWayAgreement {
    pub fn new(
        sender_business_identity: BusinessIdentity,
        receiver_business_identity: BusinessIdentity,
        protocol_settings: EdifactProtocolSettings,
    ) -> Self {
        Self {
            sender_business_identity,
            receiver_business_identity,
            protocol_settings,
        }
    }
}
#[doc = "The Edifact agreement protocol settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdifactProcessingSettings {
    #[doc = "The value indicating whether to mask security information."]
    #[serde(rename = "maskSecurityInfo")]
    pub mask_security_info: bool,
    #[doc = "The value indicating whether to preserve interchange."]
    #[serde(rename = "preserveInterchange")]
    pub preserve_interchange: bool,
    #[doc = "The value indicating whether to suspend interchange on error."]
    #[serde(rename = "suspendInterchangeOnError")]
    pub suspend_interchange_on_error: bool,
    #[doc = "The value indicating whether to create empty xml tags for trailing separators."]
    #[serde(rename = "createEmptyXmlTagsForTrailingSeparators")]
    pub create_empty_xml_tags_for_trailing_separators: bool,
    #[doc = "The value indicating whether to use dot as decimal separator."]
    #[serde(rename = "useDotAsDecimalSeparator")]
    pub use_dot_as_decimal_separator: bool,
}
impl EdifactProcessingSettings {
    pub fn new(
        mask_security_info: bool,
        preserve_interchange: bool,
        suspend_interchange_on_error: bool,
        create_empty_xml_tags_for_trailing_separators: bool,
        use_dot_as_decimal_separator: bool,
    ) -> Self {
        Self {
            mask_security_info,
            preserve_interchange,
            suspend_interchange_on_error,
            create_empty_xml_tags_for_trailing_separators,
            use_dot_as_decimal_separator,
        }
    }
}
#[doc = "The Edifact agreement protocol settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdifactProtocolSettings {
    #[doc = "The Edifact agreement validation settings."]
    #[serde(rename = "validationSettings")]
    pub validation_settings: EdifactValidationSettings,
    #[doc = "The Edifact agreement framing settings."]
    #[serde(rename = "framingSettings")]
    pub framing_settings: EdifactFramingSettings,
    #[doc = "The Edifact agreement envelope settings."]
    #[serde(rename = "envelopeSettings")]
    pub envelope_settings: EdifactEnvelopeSettings,
    #[doc = "The Edifact agreement acknowledgement settings."]
    #[serde(rename = "acknowledgementSettings")]
    pub acknowledgement_settings: EdifactAcknowledgementSettings,
    #[doc = "The Edifact message filter for odata query."]
    #[serde(rename = "messageFilter")]
    pub message_filter: EdifactMessageFilter,
    #[doc = "The Edifact agreement protocol settings."]
    #[serde(rename = "processingSettings")]
    pub processing_settings: EdifactProcessingSettings,
    #[doc = "The EDIFACT envelope override settings."]
    #[serde(rename = "envelopeOverrides", default, skip_serializing_if = "Vec::is_empty")]
    pub envelope_overrides: Vec<EdifactEnvelopeOverride>,
    #[doc = "The EDIFACT message filter list."]
    #[serde(rename = "messageFilterList", default, skip_serializing_if = "Vec::is_empty")]
    pub message_filter_list: Vec<EdifactMessageIdentifier>,
    #[doc = "The EDIFACT schema references."]
    #[serde(rename = "schemaReferences")]
    pub schema_references: Vec<EdifactSchemaReference>,
    #[doc = "The EDIFACT validation override settings."]
    #[serde(rename = "validationOverrides", default, skip_serializing_if = "Vec::is_empty")]
    pub validation_overrides: Vec<EdifactValidationOverride>,
    #[doc = "The EDIFACT delimiter override settings."]
    #[serde(rename = "edifactDelimiterOverrides", default, skip_serializing_if = "Vec::is_empty")]
    pub edifact_delimiter_overrides: Vec<EdifactDelimiterOverride>,
}
impl EdifactProtocolSettings {
    pub fn new(
        validation_settings: EdifactValidationSettings,
        framing_settings: EdifactFramingSettings,
        envelope_settings: EdifactEnvelopeSettings,
        acknowledgement_settings: EdifactAcknowledgementSettings,
        message_filter: EdifactMessageFilter,
        processing_settings: EdifactProcessingSettings,
        schema_references: Vec<EdifactSchemaReference>,
    ) -> Self {
        Self {
            validation_settings,
            framing_settings,
            envelope_settings,
            acknowledgement_settings,
            message_filter,
            processing_settings,
            envelope_overrides: Vec::new(),
            message_filter_list: Vec::new(),
            schema_references,
            validation_overrides: Vec::new(),
            edifact_delimiter_overrides: Vec::new(),
        }
    }
}
#[doc = "The Edifact schema reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdifactSchemaReference {
    #[doc = "The message id."]
    #[serde(rename = "messageId")]
    pub message_id: String,
    #[doc = "The message version."]
    #[serde(rename = "messageVersion")]
    pub message_version: String,
    #[doc = "The message release version."]
    #[serde(rename = "messageRelease")]
    pub message_release: String,
    #[doc = "The sender application id."]
    #[serde(rename = "senderApplicationId", default, skip_serializing_if = "Option::is_none")]
    pub sender_application_id: Option<String>,
    #[doc = "The sender application qualifier."]
    #[serde(rename = "senderApplicationQualifier", default, skip_serializing_if = "Option::is_none")]
    pub sender_application_qualifier: Option<String>,
    #[doc = "The association assigned code."]
    #[serde(rename = "associationAssignedCode", default, skip_serializing_if = "Option::is_none")]
    pub association_assigned_code: Option<String>,
    #[doc = "The schema name."]
    #[serde(rename = "schemaName")]
    pub schema_name: String,
}
impl EdifactSchemaReference {
    pub fn new(message_id: String, message_version: String, message_release: String, schema_name: String) -> Self {
        Self {
            message_id,
            message_version,
            message_release,
            sender_application_id: None,
            sender_application_qualifier: None,
            association_assigned_code: None,
            schema_name,
        }
    }
}
#[doc = "The Edifact validation override settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdifactValidationOverride {
    #[doc = "The message id on which the validation settings has to be applied."]
    #[serde(rename = "messageId")]
    pub message_id: String,
    #[doc = "The value indicating whether to validate character Set."]
    #[serde(rename = "enforceCharacterSet")]
    pub enforce_character_set: bool,
    #[doc = "The value indicating whether to validate EDI types."]
    #[serde(rename = "validateEDITypes")]
    pub validate_edi_types: bool,
    #[doc = "The value indicating whether to validate XSD types."]
    #[serde(rename = "validateXSDTypes")]
    pub validate_xsd_types: bool,
    #[doc = "The value indicating whether to allow leading and trailing spaces and zeroes."]
    #[serde(rename = "allowLeadingAndTrailingSpacesAndZeroes")]
    pub allow_leading_and_trailing_spaces_and_zeroes: bool,
    #[doc = "The trailing separator policy."]
    #[serde(rename = "trailingSeparatorPolicy")]
    pub trailing_separator_policy: TrailingSeparatorPolicy,
    #[doc = "The value indicating whether to trim leading and trailing spaces and zeroes."]
    #[serde(rename = "trimLeadingAndTrailingSpacesAndZeroes")]
    pub trim_leading_and_trailing_spaces_and_zeroes: bool,
}
impl EdifactValidationOverride {
    pub fn new(
        message_id: String,
        enforce_character_set: bool,
        validate_edi_types: bool,
        validate_xsd_types: bool,
        allow_leading_and_trailing_spaces_and_zeroes: bool,
        trailing_separator_policy: TrailingSeparatorPolicy,
        trim_leading_and_trailing_spaces_and_zeroes: bool,
    ) -> Self {
        Self {
            message_id,
            enforce_character_set,
            validate_edi_types,
            validate_xsd_types,
            allow_leading_and_trailing_spaces_and_zeroes,
            trailing_separator_policy,
            trim_leading_and_trailing_spaces_and_zeroes,
        }
    }
}
#[doc = "The Edifact agreement validation settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdifactValidationSettings {
    #[doc = "The value indicating whether to validate character set in the message."]
    #[serde(rename = "validateCharacterSet")]
    pub validate_character_set: bool,
    #[doc = "The value indicating whether to check for duplicate interchange control number."]
    #[serde(rename = "checkDuplicateInterchangeControlNumber")]
    pub check_duplicate_interchange_control_number: bool,
    #[doc = "The validity period of interchange control number."]
    #[serde(rename = "interchangeControlNumberValidityDays")]
    pub interchange_control_number_validity_days: i32,
    #[doc = "The value indicating whether to check for duplicate group control number."]
    #[serde(rename = "checkDuplicateGroupControlNumber")]
    pub check_duplicate_group_control_number: bool,
    #[doc = "The value indicating whether to check for duplicate transaction set control number."]
    #[serde(rename = "checkDuplicateTransactionSetControlNumber")]
    pub check_duplicate_transaction_set_control_number: bool,
    #[doc = "The value indicating whether to Whether to validate EDI types."]
    #[serde(rename = "validateEDITypes")]
    pub validate_edi_types: bool,
    #[doc = "The value indicating whether to Whether to validate XSD types."]
    #[serde(rename = "validateXSDTypes")]
    pub validate_xsd_types: bool,
    #[doc = "The value indicating whether to allow leading and trailing spaces and zeroes."]
    #[serde(rename = "allowLeadingAndTrailingSpacesAndZeroes")]
    pub allow_leading_and_trailing_spaces_and_zeroes: bool,
    #[doc = "The value indicating whether to trim leading and trailing spaces and zeroes."]
    #[serde(rename = "trimLeadingAndTrailingSpacesAndZeroes")]
    pub trim_leading_and_trailing_spaces_and_zeroes: bool,
    #[doc = "The trailing separator policy."]
    #[serde(rename = "trailingSeparatorPolicy")]
    pub trailing_separator_policy: TrailingSeparatorPolicy,
}
impl EdifactValidationSettings {
    pub fn new(
        validate_character_set: bool,
        check_duplicate_interchange_control_number: bool,
        interchange_control_number_validity_days: i32,
        check_duplicate_group_control_number: bool,
        check_duplicate_transaction_set_control_number: bool,
        validate_edi_types: bool,
        validate_xsd_types: bool,
        allow_leading_and_trailing_spaces_and_zeroes: bool,
        trim_leading_and_trailing_spaces_and_zeroes: bool,
        trailing_separator_policy: TrailingSeparatorPolicy,
    ) -> Self {
        Self {
            validate_character_set,
            check_duplicate_interchange_control_number,
            interchange_control_number_validity_days,
            check_duplicate_group_control_number,
            check_duplicate_transaction_set_control_number,
            validate_edi_types,
            validate_xsd_types,
            allow_leading_and_trailing_spaces_and_zeroes,
            trim_leading_and_trailing_spaces_and_zeroes,
            trailing_separator_policy,
        }
    }
}
#[doc = "The encryption algorithm."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EncryptionAlgorithm")]
pub enum EncryptionAlgorithm {
    NotSpecified,
    None,
    #[serde(rename = "DES3")]
    Des3,
    #[serde(rename = "RC2")]
    Rc2,
    #[serde(rename = "AES128")]
    Aes128,
    #[serde(rename = "AES192")]
    Aes192,
    #[serde(rename = "AES256")]
    Aes256,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EncryptionAlgorithm {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EncryptionAlgorithm {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EncryptionAlgorithm {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("EncryptionAlgorithm", 0u32, "NotSpecified"),
            Self::None => serializer.serialize_unit_variant("EncryptionAlgorithm", 1u32, "None"),
            Self::Des3 => serializer.serialize_unit_variant("EncryptionAlgorithm", 2u32, "DES3"),
            Self::Rc2 => serializer.serialize_unit_variant("EncryptionAlgorithm", 3u32, "RC2"),
            Self::Aes128 => serializer.serialize_unit_variant("EncryptionAlgorithm", 4u32, "AES128"),
            Self::Aes192 => serializer.serialize_unit_variant("EncryptionAlgorithm", 5u32, "AES192"),
            Self::Aes256 => serializer.serialize_unit_variant("EncryptionAlgorithm", 6u32, "AES256"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The error info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorInfo {
    #[doc = "The error code."]
    pub code: String,
}
impl ErrorInfo {
    pub fn new(code: String) -> Self {
        Self { code }
    }
}
#[doc = "Error properties indicate why the Logic service was not able to process the incoming request. The reason is provided in the error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorProperties {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates Logic service is not able to process the incoming request. The error property contains the error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error properties indicate why the Logic service was not able to process the incoming request. The reason is provided in the error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorProperties>,
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
#[doc = "The error response code."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ErrorResponseCode")]
pub enum ErrorResponseCode {
    NotSpecified,
    IntegrationServiceEnvironmentNotFound,
    InternalServerError,
    InvalidOperationId,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ErrorResponseCode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ErrorResponseCode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ErrorResponseCode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("ErrorResponseCode", 0u32, "NotSpecified"),
            Self::IntegrationServiceEnvironmentNotFound => {
                serializer.serialize_unit_variant("ErrorResponseCode", 1u32, "IntegrationServiceEnvironmentNotFound")
            }
            Self::InternalServerError => serializer.serialize_unit_variant("ErrorResponseCode", 2u32, "InternalServerError"),
            Self::InvalidOperationId => serializer.serialize_unit_variant("ErrorResponseCode", 3u32, "InvalidOperationId"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The event level."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EventLevel {
    LogAlways,
    Critical,
    Error,
    Warning,
    Informational,
    Verbose,
}
#[doc = "The expression."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Expression {
    #[doc = "The text."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[doc = "The sub expressions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subexpressions: Vec<Expression>,
    #[doc = "The azure resource error info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<AzureResourceErrorInfo>,
}
impl Expression {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The expression root."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpressionRoot {
    #[serde(flatten)]
    pub expression: Expression,
    #[doc = "The path."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl ExpressionRoot {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The expression traces."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpressionTraces {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<ExpressionRoot>,
}
impl azure_core::Continuable for ExpressionTraces {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ExpressionTraces {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The extended error info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtendedErrorInfo {
    #[doc = "The error response code."]
    pub code: ErrorResponseCode,
    #[doc = "The error message."]
    pub message: String,
    #[doc = "The error message details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ExtendedErrorInfo>,
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<Object>,
}
impl ExtendedErrorInfo {
    pub fn new(code: ErrorResponseCode, message: String) -> Self {
        Self {
            code,
            message,
            details: Vec::new(),
            inner_error: None,
        }
    }
}
#[doc = "The access control configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FlowAccessControlConfiguration {
    #[doc = "The access control configuration policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub triggers: Option<FlowAccessControlConfigurationPolicy>,
    #[doc = "The access control configuration policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<FlowAccessControlConfigurationPolicy>,
    #[doc = "The access control configuration policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<FlowAccessControlConfigurationPolicy>,
    #[doc = "The access control configuration policy."]
    #[serde(rename = "workflowManagement", default, skip_serializing_if = "Option::is_none")]
    pub workflow_management: Option<FlowAccessControlConfigurationPolicy>,
}
impl FlowAccessControlConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The access control configuration policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FlowAccessControlConfigurationPolicy {
    #[doc = "The allowed caller IP address ranges."]
    #[serde(rename = "allowedCallerIpAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_caller_ip_addresses: Vec<IpAddressRange>,
    #[doc = "AuthenticationPolicy of type Open."]
    #[serde(rename = "openAuthenticationPolicies", default, skip_serializing_if = "Option::is_none")]
    pub open_authentication_policies: Option<OpenAuthenticationAccessPolicies>,
}
impl FlowAccessControlConfigurationPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The flow endpoints configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FlowEndpoints {
    #[doc = "The outgoing ip address."]
    #[serde(rename = "outgoingIpAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub outgoing_ip_addresses: Vec<IpAddress>,
    #[doc = "The access endpoint ip address."]
    #[serde(rename = "accessEndpointIpAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub access_endpoint_ip_addresses: Vec<IpAddress>,
}
impl FlowEndpoints {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The endpoints configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FlowEndpointsConfiguration {
    #[doc = "The flow endpoints configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow: Option<FlowEndpoints>,
    #[doc = "The flow endpoints configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connector: Option<FlowEndpoints>,
}
impl FlowEndpointsConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters to generate upgraded definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenerateUpgradedDefinitionParameters {
    #[doc = "The target schema version."]
    #[serde(rename = "targetSchemaVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_schema_version: Option<String>,
}
impl GenerateUpgradedDefinitionParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The callback url parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetCallbackUrlParameters {
    #[doc = "The expiry time."]
    #[serde(rename = "notAfter", with = "azure_core::date::rfc3339::option")]
    pub not_after: Option<time::OffsetDateTime>,
    #[doc = "The key type."]
    #[serde(rename = "keyType", default, skip_serializing_if = "Option::is_none")]
    pub key_type: Option<KeyType>,
}
impl GetCallbackUrlParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The signing or hashing algorithm."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HashingAlgorithm")]
pub enum HashingAlgorithm {
    NotSpecified,
    None,
    #[serde(rename = "MD5")]
    Md5,
    #[serde(rename = "SHA1")]
    Sha1,
    #[serde(rename = "SHA2256")]
    Sha2256,
    #[serde(rename = "SHA2384")]
    Sha2384,
    #[serde(rename = "SHA2512")]
    Sha2512,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HashingAlgorithm {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HashingAlgorithm {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HashingAlgorithm {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("HashingAlgorithm", 0u32, "NotSpecified"),
            Self::None => serializer.serialize_unit_variant("HashingAlgorithm", 1u32, "None"),
            Self::Md5 => serializer.serialize_unit_variant("HashingAlgorithm", 2u32, "MD5"),
            Self::Sha1 => serializer.serialize_unit_variant("HashingAlgorithm", 3u32, "SHA1"),
            Self::Sha2256 => serializer.serialize_unit_variant("HashingAlgorithm", 4u32, "SHA2256"),
            Self::Sha2384 => serializer.serialize_unit_variant("HashingAlgorithm", 5u32, "SHA2384"),
            Self::Sha2512 => serializer.serialize_unit_variant("HashingAlgorithm", 6u32, "SHA2512"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The integration account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationAccount {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The integration account properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IntegrationAccountProperties>,
    #[doc = "The integration account sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<IntegrationAccountSku>,
}
impl IntegrationAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration account agreement."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationAccountAgreement {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The integration account agreement properties."]
    pub properties: IntegrationAccountAgreementProperties,
}
impl IntegrationAccountAgreement {
    pub fn new(properties: IntegrationAccountAgreementProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "The integration account agreement filter for odata query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationAccountAgreementFilter {
    #[doc = "The agreement type."]
    #[serde(rename = "agreementType")]
    pub agreement_type: AgreementType,
}
impl IntegrationAccountAgreementFilter {
    pub fn new(agreement_type: AgreementType) -> Self {
        Self { agreement_type }
    }
}
#[doc = "The list of integration account agreements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationAccountAgreementListResult {
    #[doc = "The list of integration account agreements."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IntegrationAccountAgreement>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IntegrationAccountAgreementListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IntegrationAccountAgreementListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration account agreement properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationAccountAgreementProperties {
    #[doc = "The created time."]
    #[serde(rename = "createdTime", with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "The changed time."]
    #[serde(rename = "changedTime", with = "azure_core::date::rfc3339::option")]
    pub changed_time: Option<time::OffsetDateTime>,
    #[doc = "The metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The agreement type."]
    #[serde(rename = "agreementType")]
    pub agreement_type: AgreementType,
    #[doc = "The integration account partner that is set as host partner for this agreement."]
    #[serde(rename = "hostPartner")]
    pub host_partner: String,
    #[doc = "The integration account partner that is set as guest partner for this agreement."]
    #[serde(rename = "guestPartner")]
    pub guest_partner: String,
    #[doc = "The integration account partner's business identity."]
    #[serde(rename = "hostIdentity")]
    pub host_identity: BusinessIdentity,
    #[doc = "The integration account partner's business identity."]
    #[serde(rename = "guestIdentity")]
    pub guest_identity: BusinessIdentity,
    #[doc = "The integration account agreement content."]
    pub content: AgreementContent,
}
impl IntegrationAccountAgreementProperties {
    pub fn new(
        agreement_type: AgreementType,
        host_partner: String,
        guest_partner: String,
        host_identity: BusinessIdentity,
        guest_identity: BusinessIdentity,
        content: AgreementContent,
    ) -> Self {
        Self {
            created_time: None,
            changed_time: None,
            metadata: None,
            agreement_type,
            host_partner,
            guest_partner,
            host_identity,
            guest_identity,
            content,
        }
    }
}
#[doc = "The integration account certificate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationAccountCertificate {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The integration account certificate properties."]
    pub properties: IntegrationAccountCertificateProperties,
}
impl IntegrationAccountCertificate {
    pub fn new(properties: IntegrationAccountCertificateProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "The list of integration account certificates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationAccountCertificateListResult {
    #[doc = "The list of integration account certificates."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IntegrationAccountCertificate>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IntegrationAccountCertificateListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IntegrationAccountCertificateListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration account certificate properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationAccountCertificateProperties {
    #[doc = "The created time."]
    #[serde(rename = "createdTime", with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "The changed time."]
    #[serde(rename = "changedTime", with = "azure_core::date::rfc3339::option")]
    pub changed_time: Option<time::OffsetDateTime>,
    #[doc = "The metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The reference to the key vault key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<KeyVaultKeyReference>,
    #[doc = "The public certificate."]
    #[serde(rename = "publicCertificate", default, skip_serializing_if = "Option::is_none")]
    pub public_certificate: Option<String>,
}
impl IntegrationAccountCertificateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of integration accounts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationAccountListResult {
    #[doc = "The list of integration accounts."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IntegrationAccount>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IntegrationAccountListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IntegrationAccountListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration account map."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationAccountMap {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The integration account map."]
    pub properties: IntegrationAccountMapProperties,
}
impl IntegrationAccountMap {
    pub fn new(properties: IntegrationAccountMapProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "The integration account map filter for odata query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationAccountMapFilter {
    #[doc = "The map type."]
    #[serde(rename = "mapType")]
    pub map_type: MapType,
}
impl IntegrationAccountMapFilter {
    pub fn new(map_type: MapType) -> Self {
        Self { map_type }
    }
}
#[doc = "The list of integration account maps."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationAccountMapListResult {
    #[doc = "The list of integration account maps."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IntegrationAccountMap>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IntegrationAccountMapListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IntegrationAccountMapListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration account map."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationAccountMapProperties {
    #[doc = "The map type."]
    #[serde(rename = "mapType")]
    pub map_type: MapType,
    #[doc = "The parameters schema of integration account map."]
    #[serde(rename = "parametersSchema", default, skip_serializing_if = "Option::is_none")]
    pub parameters_schema: Option<integration_account_map_properties::ParametersSchema>,
    #[doc = "The created time."]
    #[serde(rename = "createdTime", with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "The changed time."]
    #[serde(rename = "changedTime", with = "azure_core::date::rfc3339::option")]
    pub changed_time: Option<time::OffsetDateTime>,
    #[doc = "The content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[doc = "The content type."]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "The content link."]
    #[serde(rename = "contentLink", default, skip_serializing_if = "Option::is_none")]
    pub content_link: Option<ContentLink>,
    #[doc = "The metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
impl IntegrationAccountMapProperties {
    pub fn new(map_type: MapType) -> Self {
        Self {
            map_type,
            parameters_schema: None,
            created_time: None,
            changed_time: None,
            content: None,
            content_type: None,
            content_link: None,
            metadata: None,
        }
    }
}
pub mod integration_account_map_properties {
    use super::*;
    #[doc = "The parameters schema of integration account map."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ParametersSchema {
        #[doc = "The reference name."]
        #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
        pub ref_: Option<String>,
    }
    impl ParametersSchema {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The integration account partner."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationAccountPartner {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The integration account partner properties."]
    pub properties: IntegrationAccountPartnerProperties,
}
impl IntegrationAccountPartner {
    pub fn new(properties: IntegrationAccountPartnerProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "The integration account partner filter for odata query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationAccountPartnerFilter {
    #[doc = "The partner type."]
    #[serde(rename = "partnerType")]
    pub partner_type: PartnerType,
}
impl IntegrationAccountPartnerFilter {
    pub fn new(partner_type: PartnerType) -> Self {
        Self { partner_type }
    }
}
#[doc = "The list of integration account partners."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationAccountPartnerListResult {
    #[doc = "The list of integration account partners."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IntegrationAccountPartner>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IntegrationAccountPartnerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IntegrationAccountPartnerListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration account partner properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationAccountPartnerProperties {
    #[doc = "The partner type."]
    #[serde(rename = "partnerType")]
    pub partner_type: PartnerType,
    #[doc = "The created time."]
    #[serde(rename = "createdTime", with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "The changed time."]
    #[serde(rename = "changedTime", with = "azure_core::date::rfc3339::option")]
    pub changed_time: Option<time::OffsetDateTime>,
    #[doc = "The metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The integration account partner content."]
    pub content: PartnerContent,
}
impl IntegrationAccountPartnerProperties {
    pub fn new(partner_type: PartnerType, content: PartnerContent) -> Self {
        Self {
            partner_type,
            created_time: None,
            changed_time: None,
            metadata: None,
            content,
        }
    }
}
#[doc = "The integration account properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationAccountProperties {
    #[doc = "The resource reference."]
    #[serde(rename = "integrationServiceEnvironment", default, skip_serializing_if = "Option::is_none")]
    pub integration_service_environment: Option<ResourceReference>,
    #[doc = "The workflow state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<WorkflowState>,
}
impl IntegrationAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration account schema."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationAccountSchema {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The integration account schema properties."]
    pub properties: IntegrationAccountSchemaProperties,
}
impl IntegrationAccountSchema {
    pub fn new(properties: IntegrationAccountSchemaProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "The integration account schema filter for odata query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationAccountSchemaFilter {
    #[doc = "The schema type."]
    #[serde(rename = "schemaType")]
    pub schema_type: SchemaType,
}
impl IntegrationAccountSchemaFilter {
    pub fn new(schema_type: SchemaType) -> Self {
        Self { schema_type }
    }
}
#[doc = "The list of integration account schemas."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationAccountSchemaListResult {
    #[doc = "The list of integration account schemas."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IntegrationAccountSchema>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IntegrationAccountSchemaListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IntegrationAccountSchemaListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration account schema properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationAccountSchemaProperties {
    #[doc = "The schema type."]
    #[serde(rename = "schemaType")]
    pub schema_type: SchemaType,
    #[doc = "The target namespace of the schema."]
    #[serde(rename = "targetNamespace", default, skip_serializing_if = "Option::is_none")]
    pub target_namespace: Option<String>,
    #[doc = "The document name."]
    #[serde(rename = "documentName", default, skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    #[doc = "The file name."]
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[doc = "The created time."]
    #[serde(rename = "createdTime", with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "The changed time."]
    #[serde(rename = "changedTime", with = "azure_core::date::rfc3339::option")]
    pub changed_time: Option<time::OffsetDateTime>,
    #[doc = "The metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "The content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[doc = "The content type."]
    #[serde(rename = "contentType", default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "The content link."]
    #[serde(rename = "contentLink", default, skip_serializing_if = "Option::is_none")]
    pub content_link: Option<ContentLink>,
}
impl IntegrationAccountSchemaProperties {
    pub fn new(schema_type: SchemaType) -> Self {
        Self {
            schema_type,
            target_namespace: None,
            document_name: None,
            file_name: None,
            created_time: None,
            changed_time: None,
            metadata: None,
            content: None,
            content_type: None,
            content_link: None,
        }
    }
}
#[doc = "The integration account session."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationAccountSession {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The integration account session properties."]
    pub properties: IntegrationAccountSessionProperties,
}
impl IntegrationAccountSession {
    pub fn new(properties: IntegrationAccountSessionProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "The integration account session filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationAccountSessionFilter {
    #[doc = "The changed time of integration account sessions."]
    #[serde(rename = "changedTime", with = "azure_core::date::rfc3339")]
    pub changed_time: time::OffsetDateTime,
}
impl IntegrationAccountSessionFilter {
    pub fn new(changed_time: time::OffsetDateTime) -> Self {
        Self { changed_time }
    }
}
#[doc = "The list of integration account sessions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationAccountSessionListResult {
    #[doc = "The list of integration account sessions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IntegrationAccountSession>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IntegrationAccountSessionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IntegrationAccountSessionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration account session properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationAccountSessionProperties {
    #[doc = "The created time."]
    #[serde(rename = "createdTime", with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "The changed time."]
    #[serde(rename = "changedTime", with = "azure_core::date::rfc3339::option")]
    pub changed_time: Option<time::OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Object>,
}
impl IntegrationAccountSessionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration account sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationAccountSku {
    #[doc = "The integration account sku name."]
    pub name: IntegrationAccountSkuName,
}
impl IntegrationAccountSku {
    pub fn new(name: IntegrationAccountSkuName) -> Self {
        Self { name }
    }
}
#[doc = "The integration account sku name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IntegrationAccountSkuName")]
pub enum IntegrationAccountSkuName {
    NotSpecified,
    Free,
    Basic,
    Standard,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IntegrationAccountSkuName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IntegrationAccountSkuName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IntegrationAccountSkuName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("IntegrationAccountSkuName", 0u32, "NotSpecified"),
            Self::Free => serializer.serialize_unit_variant("IntegrationAccountSkuName", 1u32, "Free"),
            Self::Basic => serializer.serialize_unit_variant("IntegrationAccountSkuName", 2u32, "Basic"),
            Self::Standard => serializer.serialize_unit_variant("IntegrationAccountSkuName", 3u32, "Standard"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The encryption configuration for the integration service environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationServiceEnvironmenEncryptionConfiguration {
    #[doc = "The encryption key details for the integration service environment."]
    #[serde(rename = "encryptionKeyReference", default, skip_serializing_if = "Option::is_none")]
    pub encryption_key_reference: Option<IntegrationServiceEnvironmenEncryptionKeyReference>,
}
impl IntegrationServiceEnvironmenEncryptionConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The encryption key details for the integration service environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationServiceEnvironmenEncryptionKeyReference {
    #[doc = "The resource reference."]
    #[serde(rename = "keyVault", default, skip_serializing_if = "Option::is_none")]
    pub key_vault: Option<ResourceReference>,
    #[doc = "Gets the key name in the Key Vault."]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[doc = "Gets the version of the key specified in the keyName property."]
    #[serde(rename = "keyVersion", default, skip_serializing_if = "Option::is_none")]
    pub key_version: Option<String>,
}
impl IntegrationServiceEnvironmenEncryptionKeyReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration service environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationServiceEnvironment {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The integration service environment properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IntegrationServiceEnvironmentProperties>,
    #[doc = "The integration service environment sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<IntegrationServiceEnvironmentSku>,
    #[doc = "Managed service identity properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl IntegrationServiceEnvironment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration service environment access endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationServiceEnvironmentAccessEndpoint {
    #[doc = "The integration service environment access endpoint type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<IntegrationServiceEnvironmentAccessEndpointType>,
}
impl IntegrationServiceEnvironmentAccessEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration service environment access endpoint type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IntegrationServiceEnvironmentAccessEndpointType")]
pub enum IntegrationServiceEnvironmentAccessEndpointType {
    NotSpecified,
    External,
    Internal,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IntegrationServiceEnvironmentAccessEndpointType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IntegrationServiceEnvironmentAccessEndpointType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IntegrationServiceEnvironmentAccessEndpointType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => {
                serializer.serialize_unit_variant("IntegrationServiceEnvironmentAccessEndpointType", 0u32, "NotSpecified")
            }
            Self::External => serializer.serialize_unit_variant("IntegrationServiceEnvironmentAccessEndpointType", 1u32, "External"),
            Self::Internal => serializer.serialize_unit_variant("IntegrationServiceEnvironmentAccessEndpointType", 2u32, "Internal"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The list of integration service environments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationServiceEnvironmentListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IntegrationServiceEnvironment>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IntegrationServiceEnvironmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IntegrationServiceEnvironmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration service environment managed api."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationServiceEnvironmentManagedApi {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The integration service environment managed api properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<IntegrationServiceEnvironmentManagedApiProperties>,
}
impl IntegrationServiceEnvironmentManagedApi {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration service environment managed api deployment parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationServiceEnvironmentManagedApiDeploymentParameters {
    #[doc = "The content link."]
    #[serde(rename = "contentLinkDefinition", default, skip_serializing_if = "Option::is_none")]
    pub content_link_definition: Option<ContentLink>,
}
impl IntegrationServiceEnvironmentManagedApiDeploymentParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of integration service environment managed APIs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationServiceEnvironmentManagedApiListResult {
    #[doc = "The integration service environment managed APIs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IntegrationServiceEnvironmentManagedApi>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IntegrationServiceEnvironmentManagedApiListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IntegrationServiceEnvironmentManagedApiListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration service environment managed api properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationServiceEnvironmentManagedApiProperties {
    #[serde(flatten)]
    pub api_resource_properties: ApiResourceProperties,
    #[doc = "The integration service environment managed api deployment parameters."]
    #[serde(rename = "deploymentParameters", default, skip_serializing_if = "Option::is_none")]
    pub deployment_parameters: Option<IntegrationServiceEnvironmentManagedApiDeploymentParameters>,
}
impl IntegrationServiceEnvironmentManagedApiProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The azure async operation resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationServiceEnvironmentNetworkDependency {
    #[doc = "The integration service environment network dependency category type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<IntegrationServiceEnvironmentNetworkDependencyCategoryType>,
    #[doc = "The display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The endpoints."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<IntegrationServiceEnvironmentNetworkEndpoint>,
}
impl IntegrationServiceEnvironmentNetworkDependency {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration service environment network dependency category type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IntegrationServiceEnvironmentNetworkDependencyCategoryType")]
pub enum IntegrationServiceEnvironmentNetworkDependencyCategoryType {
    NotSpecified,
    AzureStorage,
    AzureManagement,
    AzureActiveDirectory,
    #[serde(rename = "SSLCertificateVerification")]
    SslCertificateVerification,
    DiagnosticLogsAndMetrics,
    IntegrationServiceEnvironmentConnectors,
    RedisCache,
    AccessEndpoints,
    RecoveryService,
    #[serde(rename = "SQL")]
    Sql,
    RegionalService,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IntegrationServiceEnvironmentNetworkDependencyCategoryType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IntegrationServiceEnvironmentNetworkDependencyCategoryType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IntegrationServiceEnvironmentNetworkDependencyCategoryType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => {
                serializer.serialize_unit_variant("IntegrationServiceEnvironmentNetworkDependencyCategoryType", 0u32, "NotSpecified")
            }
            Self::AzureStorage => {
                serializer.serialize_unit_variant("IntegrationServiceEnvironmentNetworkDependencyCategoryType", 1u32, "AzureStorage")
            }
            Self::AzureManagement => serializer.serialize_unit_variant(
                "IntegrationServiceEnvironmentNetworkDependencyCategoryType",
                2u32,
                "AzureManagement",
            ),
            Self::AzureActiveDirectory => serializer.serialize_unit_variant(
                "IntegrationServiceEnvironmentNetworkDependencyCategoryType",
                3u32,
                "AzureActiveDirectory",
            ),
            Self::SslCertificateVerification => serializer.serialize_unit_variant(
                "IntegrationServiceEnvironmentNetworkDependencyCategoryType",
                4u32,
                "SSLCertificateVerification",
            ),
            Self::DiagnosticLogsAndMetrics => serializer.serialize_unit_variant(
                "IntegrationServiceEnvironmentNetworkDependencyCategoryType",
                5u32,
                "DiagnosticLogsAndMetrics",
            ),
            Self::IntegrationServiceEnvironmentConnectors => serializer.serialize_unit_variant(
                "IntegrationServiceEnvironmentNetworkDependencyCategoryType",
                6u32,
                "IntegrationServiceEnvironmentConnectors",
            ),
            Self::RedisCache => {
                serializer.serialize_unit_variant("IntegrationServiceEnvironmentNetworkDependencyCategoryType", 7u32, "RedisCache")
            }
            Self::AccessEndpoints => serializer.serialize_unit_variant(
                "IntegrationServiceEnvironmentNetworkDependencyCategoryType",
                8u32,
                "AccessEndpoints",
            ),
            Self::RecoveryService => serializer.serialize_unit_variant(
                "IntegrationServiceEnvironmentNetworkDependencyCategoryType",
                9u32,
                "RecoveryService",
            ),
            Self::Sql => serializer.serialize_unit_variant("IntegrationServiceEnvironmentNetworkDependencyCategoryType", 10u32, "SQL"),
            Self::RegionalService => serializer.serialize_unit_variant(
                "IntegrationServiceEnvironmentNetworkDependencyCategoryType",
                11u32,
                "RegionalService",
            ),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The integration service environment subnet network health."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationServiceEnvironmentNetworkDependencyHealth {
    #[doc = "The extended error info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ExtendedErrorInfo>,
    #[doc = "The integration service environment network dependency health state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<IntegrationServiceEnvironmentNetworkDependencyHealthState>,
}
impl IntegrationServiceEnvironmentNetworkDependencyHealth {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration service environment network dependency health state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IntegrationServiceEnvironmentNetworkDependencyHealthState")]
pub enum IntegrationServiceEnvironmentNetworkDependencyHealthState {
    NotSpecified,
    Healthy,
    Unhealthy,
    Unknown,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IntegrationServiceEnvironmentNetworkDependencyHealthState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IntegrationServiceEnvironmentNetworkDependencyHealthState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IntegrationServiceEnvironmentNetworkDependencyHealthState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => {
                serializer.serialize_unit_variant("IntegrationServiceEnvironmentNetworkDependencyHealthState", 0u32, "NotSpecified")
            }
            Self::Healthy => {
                serializer.serialize_unit_variant("IntegrationServiceEnvironmentNetworkDependencyHealthState", 1u32, "Healthy")
            }
            Self::Unhealthy => {
                serializer.serialize_unit_variant("IntegrationServiceEnvironmentNetworkDependencyHealthState", 2u32, "Unhealthy")
            }
            Self::Unknown => {
                serializer.serialize_unit_variant("IntegrationServiceEnvironmentNetworkDependencyHealthState", 3u32, "Unknown")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The integration service environment network endpoint accessibility state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IntegrationServiceEnvironmentNetworkEndPointAccessibilityState")]
pub enum IntegrationServiceEnvironmentNetworkEndPointAccessibilityState {
    NotSpecified,
    Unknown,
    Available,
    NotAvailable,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IntegrationServiceEnvironmentNetworkEndPointAccessibilityState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IntegrationServiceEnvironmentNetworkEndPointAccessibilityState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IntegrationServiceEnvironmentNetworkEndPointAccessibilityState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant(
                "IntegrationServiceEnvironmentNetworkEndPointAccessibilityState",
                0u32,
                "NotSpecified",
            ),
            Self::Unknown => {
                serializer.serialize_unit_variant("IntegrationServiceEnvironmentNetworkEndPointAccessibilityState", 1u32, "Unknown")
            }
            Self::Available => {
                serializer.serialize_unit_variant("IntegrationServiceEnvironmentNetworkEndPointAccessibilityState", 2u32, "Available")
            }
            Self::NotAvailable => serializer.serialize_unit_variant(
                "IntegrationServiceEnvironmentNetworkEndPointAccessibilityState",
                3u32,
                "NotAvailable",
            ),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The network endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationServiceEnvironmentNetworkEndpoint {
    #[doc = "The integration service environment network endpoint accessibility state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accessibility: Option<IntegrationServiceEnvironmentNetworkEndPointAccessibilityState>,
    #[doc = "The domain name."]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "The ports."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<String>,
}
impl IntegrationServiceEnvironmentNetworkEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration service environment network health of all the subnets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationServiceEnvironmentNetworkHealth {}
impl IntegrationServiceEnvironmentNetworkHealth {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration service environment properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationServiceEnvironmentProperties {
    #[doc = "The workflow provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<WorkflowProvisioningState>,
    #[doc = "The workflow state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<WorkflowState>,
    #[doc = "Gets the tracking id."]
    #[serde(rename = "integrationServiceEnvironmentId", default, skip_serializing_if = "Option::is_none")]
    pub integration_service_environment_id: Option<String>,
    #[doc = "The endpoints configuration."]
    #[serde(rename = "endpointsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub endpoints_configuration: Option<FlowEndpointsConfiguration>,
    #[doc = "The network configuration."]
    #[serde(rename = "networkConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[doc = "The encryption configuration for the integration service environment."]
    #[serde(rename = "encryptionConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<IntegrationServiceEnvironmenEncryptionConfiguration>,
}
impl IntegrationServiceEnvironmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration service environment sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationServiceEnvironmentSku {
    #[doc = "The integration service environment sku name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<IntegrationServiceEnvironmentSkuName>,
    #[doc = "The sku capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl IntegrationServiceEnvironmentSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration service environment sku capacity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationServiceEnvironmentSkuCapacity {
    #[doc = "The minimum capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,
    #[doc = "The maximum capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,
    #[doc = "The default capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<i32>,
    #[doc = "The integration service environment sku scale type."]
    #[serde(rename = "scaleType", default, skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<IntegrationServiceEnvironmentSkuScaleType>,
}
impl IntegrationServiceEnvironmentSkuCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration service environment sku definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationServiceEnvironmentSkuDefinition {
    #[doc = "The resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The sku."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<integration_service_environment_sku_definition::Sku>,
    #[doc = "The integration service environment sku capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<IntegrationServiceEnvironmentSkuCapacity>,
}
impl IntegrationServiceEnvironmentSkuDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod integration_service_environment_sku_definition {
    use super::*;
    #[doc = "The sku."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Sku {
        #[doc = "The integration service environment sku name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<IntegrationServiceEnvironmentSkuName>,
        #[doc = "The sku tier."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tier: Option<String>,
    }
    impl Sku {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The list of integration service environment skus."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationServiceEnvironmentSkuList {
    #[doc = "The list of integration service environment skus."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IntegrationServiceEnvironmentSkuDefinition>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IntegrationServiceEnvironmentSkuList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IntegrationServiceEnvironmentSkuList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The integration service environment sku name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IntegrationServiceEnvironmentSkuName")]
pub enum IntegrationServiceEnvironmentSkuName {
    NotSpecified,
    Premium,
    Developer,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IntegrationServiceEnvironmentSkuName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IntegrationServiceEnvironmentSkuName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IntegrationServiceEnvironmentSkuName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("IntegrationServiceEnvironmentSkuName", 0u32, "NotSpecified"),
            Self::Premium => serializer.serialize_unit_variant("IntegrationServiceEnvironmentSkuName", 1u32, "Premium"),
            Self::Developer => serializer.serialize_unit_variant("IntegrationServiceEnvironmentSkuName", 2u32, "Developer"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The integration service environment sku scale type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IntegrationServiceEnvironmentSkuScaleType")]
pub enum IntegrationServiceEnvironmentSkuScaleType {
    Manual,
    Automatic,
    None,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IntegrationServiceEnvironmentSkuScaleType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IntegrationServiceEnvironmentSkuScaleType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IntegrationServiceEnvironmentSkuScaleType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Manual => serializer.serialize_unit_variant("IntegrationServiceEnvironmentSkuScaleType", 0u32, "Manual"),
            Self::Automatic => serializer.serialize_unit_variant("IntegrationServiceEnvironmentSkuScaleType", 1u32, "Automatic"),
            Self::None => serializer.serialize_unit_variant("IntegrationServiceEnvironmentSkuScaleType", 2u32, "None"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The integration service environment subnet network health."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationServiceEnvironmentSubnetNetworkHealth {
    #[doc = "The outbound network dependencies."]
    #[serde(rename = "outboundNetworkDependencies", default, skip_serializing_if = "Vec::is_empty")]
    pub outbound_network_dependencies: Vec<IntegrationServiceEnvironmentNetworkDependency>,
    #[doc = "The integration service environment subnet network health."]
    #[serde(rename = "outboundNetworkHealth", default, skip_serializing_if = "Option::is_none")]
    pub outbound_network_health: Option<IntegrationServiceEnvironmentNetworkDependencyHealth>,
    #[doc = "The integration service environment network endpoint accessibility state."]
    #[serde(rename = "networkDependencyHealthState")]
    pub network_dependency_health_state: IntegrationServiceEnvironmentNetworkEndPointAccessibilityState,
}
impl IntegrationServiceEnvironmentSubnetNetworkHealth {
    pub fn new(network_dependency_health_state: IntegrationServiceEnvironmentNetworkEndPointAccessibilityState) -> Self {
        Self {
            outbound_network_dependencies: Vec::new(),
            outbound_network_health: None,
            network_dependency_health_state,
        }
    }
}
#[doc = "The ip address."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpAddress {
    #[doc = "The address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}
impl IpAddress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The ip address range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpAddressRange {
    #[doc = "The IP address range."]
    #[serde(rename = "addressRange", default, skip_serializing_if = "Option::is_none")]
    pub address_range: Option<String>,
}
impl IpAddressRange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON schema."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JsonSchema {
    #[doc = "The JSON title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The JSON content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
impl JsonSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The key type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "KeyType")]
pub enum KeyType {
    NotSpecified,
    Primary,
    Secondary,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for KeyType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for KeyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for KeyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("KeyType", 0u32, "NotSpecified"),
            Self::Primary => serializer.serialize_unit_variant("KeyType", 1u32, "Primary"),
            Self::Secondary => serializer.serialize_unit_variant("KeyType", 2u32, "Secondary"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The key vault key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultKey {
    #[doc = "The key id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[doc = "The key attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<key_vault_key::Attributes>,
}
impl KeyVaultKey {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod key_vault_key {
    use super::*;
    #[doc = "The key attributes."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Attributes {
        #[doc = "Whether the key is enabled or not."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[doc = "When the key was created."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub created: Option<i64>,
        #[doc = "When the key was updated."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub updated: Option<i64>,
    }
    impl Attributes {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Collection of key vault keys."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultKeyCollection {
    #[doc = "The key vault keys."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<KeyVaultKey>,
    #[doc = "The skip token."]
    #[serde(rename = "skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
}
impl azure_core::Continuable for KeyVaultKeyCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl KeyVaultKeyCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The reference to the key vault key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultKeyReference {
    #[doc = "The key vault reference."]
    #[serde(rename = "keyVault")]
    pub key_vault: key_vault_key_reference::KeyVault,
    #[doc = "The private key name in key vault."]
    #[serde(rename = "keyName")]
    pub key_name: String,
    #[doc = "The private key version in key vault."]
    #[serde(rename = "keyVersion", default, skip_serializing_if = "Option::is_none")]
    pub key_version: Option<String>,
}
impl KeyVaultKeyReference {
    pub fn new(key_vault: key_vault_key_reference::KeyVault, key_name: String) -> Self {
        Self {
            key_vault,
            key_name,
            key_version: None,
        }
    }
}
pub mod key_vault_key_reference {
    use super::*;
    #[doc = "The key vault reference."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct KeyVault {
        #[doc = "The resource id."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "The resource name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[doc = "The resource type."]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }
    impl KeyVault {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The key vault reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultReference {
    #[serde(flatten)]
    pub resource_reference: ResourceReference,
    #[doc = "The key vault name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl KeyVaultReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list key vault keys definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListKeyVaultKeysDefinition {
    #[doc = "The key vault reference."]
    #[serde(rename = "keyVault")]
    pub key_vault: KeyVaultReference,
    #[doc = "The skip token."]
    #[serde(rename = "skipToken", default, skip_serializing_if = "Option::is_none")]
    pub skip_token: Option<String>,
}
impl ListKeyVaultKeysDefinition {
    pub fn new(key_vault: KeyVaultReference) -> Self {
        Self {
            key_vault,
            skip_token: None,
        }
    }
}
#[doc = "The managed api definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedApi {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The API resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApiResourceProperties>,
}
impl ManagedApi {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of managed APIs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedApiListResult {
    #[doc = "The managed APIs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedApi>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ManagedApiListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed service identity properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedServiceIdentity {
    #[doc = "Type of managed service identity. The type 'SystemAssigned' includes an implicitly created identity. The type 'None' will remove any identities from the resource."]
    #[serde(rename = "type")]
    pub type_: managed_service_identity::Type,
    #[doc = "Tenant of managed service identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Principal Id of managed service identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The list of user assigned identities associated with the resource. The user identity dictionary key references will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}"]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl ManagedServiceIdentity {
    pub fn new(type_: managed_service_identity::Type) -> Self {
        Self {
            type_,
            tenant_id: None,
            principal_id: None,
            user_assigned_identities: None,
        }
    }
}
pub mod managed_service_identity {
    use super::*;
    #[doc = "Type of managed service identity. The type 'SystemAssigned' includes an implicitly created identity. The type 'None' will remove any identities from the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        None,
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
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 0u32, "SystemAssigned"),
                Self::UserAssigned => serializer.serialize_unit_variant("Type", 1u32, "UserAssigned"),
                Self::None => serializer.serialize_unit_variant("Type", 2u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The map type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MapType")]
pub enum MapType {
    NotSpecified,
    Xslt,
    Xslt20,
    Xslt30,
    Liquid,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MapType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MapType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MapType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("MapType", 0u32, "NotSpecified"),
            Self::Xslt => serializer.serialize_unit_variant("MapType", 1u32, "Xslt"),
            Self::Xslt20 => serializer.serialize_unit_variant("MapType", 2u32, "Xslt20"),
            Self::Xslt30 => serializer.serialize_unit_variant("MapType", 3u32, "Xslt30"),
            Self::Liquid => serializer.serialize_unit_variant("MapType", 4u32, "Liquid"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The message filter type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MessageFilterType")]
pub enum MessageFilterType {
    NotSpecified,
    Include,
    Exclude,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MessageFilterType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MessageFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MessageFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("MessageFilterType", 0u32, "NotSpecified"),
            Self::Include => serializer.serialize_unit_variant("MessageFilterType", 1u32, "Include"),
            Self::Exclude => serializer.serialize_unit_variant("MessageFilterType", 2u32, "Exclude"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The network configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkConfiguration {
    #[doc = "Gets the virtual network address space."]
    #[serde(rename = "virtualNetworkAddressSpace", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_address_space: Option<String>,
    #[doc = "The integration service environment access endpoint."]
    #[serde(rename = "accessEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub access_endpoint: Option<IntegrationServiceEnvironmentAccessEndpoint>,
    #[doc = "The subnets."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subnets: Vec<ResourceReference>,
}
impl NetworkConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Object {}
impl Object {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AuthenticationPolicy of type Open."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenAuthenticationAccessPolicies {
    #[doc = "Open authentication policies."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<serde_json::Value>,
}
impl OpenAuthenticationAccessPolicies {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Open authentication access policy defined by user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenAuthenticationAccessPolicy {
    #[doc = "Open authentication policy provider type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<OpenAuthenticationProviderType>,
    #[doc = "The access policy claims."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub claims: Vec<OpenAuthenticationPolicyClaim>,
}
impl OpenAuthenticationAccessPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Open authentication policy claim."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenAuthenticationPolicyClaim {
    #[doc = "The name of the claim."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of the claim."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl OpenAuthenticationPolicyClaim {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Open authentication policy provider type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OpenAuthenticationProviderType")]
pub enum OpenAuthenticationProviderType {
    #[serde(rename = "AAD")]
    Aad,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OpenAuthenticationProviderType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OpenAuthenticationProviderType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OpenAuthenticationProviderType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Aad => serializer.serialize_unit_variant("OpenAuthenticationProviderType", 0u32, "AAD"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Logic REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation: origin"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Object>,
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
        #[doc = "Service provider: Microsoft.Logic"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Profile, endpoint, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Operation: description."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list Logic operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Logic operations supported by the Logic resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "The operation result definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResult {
    #[serde(flatten)]
    pub operation_result_properties: OperationResultProperties,
    #[doc = "Gets the tracking id."]
    #[serde(rename = "trackingId", default, skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Object>,
    #[doc = "The content link."]
    #[serde(rename = "inputsLink", default, skip_serializing_if = "Option::is_none")]
    pub inputs_link: Option<ContentLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Object>,
    #[doc = "The content link."]
    #[serde(rename = "outputsLink", default, skip_serializing_if = "Option::is_none")]
    pub outputs_link: Option<ContentLink>,
    #[serde(rename = "trackedProperties", default, skip_serializing_if = "Option::is_none")]
    pub tracked_properties: Option<Object>,
    #[doc = "Gets the retry histories."]
    #[serde(rename = "retryHistory", default, skip_serializing_if = "Vec::is_empty")]
    pub retry_history: Vec<RetryHistory>,
    #[serde(rename = "iterationCount", default, skip_serializing_if = "Option::is_none")]
    pub iteration_count: Option<i32>,
}
impl OperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The run operation result properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResultProperties {
    #[doc = "The start time of the workflow scope repetition."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time of the workflow scope repetition."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The workflow run action correlation properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correlation: Option<RunActionCorrelation>,
    #[doc = "The workflow status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkflowStatus>,
    #[doc = "The workflow scope repetition code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
}
impl OperationResultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameter type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ParameterType")]
pub enum ParameterType {
    NotSpecified,
    String,
    SecureString,
    Int,
    Float,
    Bool,
    Array,
    Object,
    SecureObject,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ParameterType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ParameterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ParameterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("ParameterType", 0u32, "NotSpecified"),
            Self::String => serializer.serialize_unit_variant("ParameterType", 1u32, "String"),
            Self::SecureString => serializer.serialize_unit_variant("ParameterType", 2u32, "SecureString"),
            Self::Int => serializer.serialize_unit_variant("ParameterType", 3u32, "Int"),
            Self::Float => serializer.serialize_unit_variant("ParameterType", 4u32, "Float"),
            Self::Bool => serializer.serialize_unit_variant("ParameterType", 5u32, "Bool"),
            Self::Array => serializer.serialize_unit_variant("ParameterType", 6u32, "Array"),
            Self::Object => serializer.serialize_unit_variant("ParameterType", 7u32, "Object"),
            Self::SecureObject => serializer.serialize_unit_variant("ParameterType", 8u32, "SecureObject"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The integration account partner content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartnerContent {
    #[doc = "The B2B partner content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub b2b: Option<B2bPartnerContent>,
}
impl PartnerContent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The partner type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PartnerType")]
pub enum PartnerType {
    NotSpecified,
    #[serde(rename = "B2B")]
    B2b,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PartnerType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PartnerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PartnerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("PartnerType", 0u32, "NotSpecified"),
            Self::B2b => serializer.serialize_unit_variant("PartnerType", 1u32, "B2B"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The recurrence frequency."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RecurrenceFrequency")]
pub enum RecurrenceFrequency {
    NotSpecified,
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RecurrenceFrequency {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RecurrenceFrequency {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RecurrenceFrequency {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("RecurrenceFrequency", 0u32, "NotSpecified"),
            Self::Second => serializer.serialize_unit_variant("RecurrenceFrequency", 1u32, "Second"),
            Self::Minute => serializer.serialize_unit_variant("RecurrenceFrequency", 2u32, "Minute"),
            Self::Hour => serializer.serialize_unit_variant("RecurrenceFrequency", 3u32, "Hour"),
            Self::Day => serializer.serialize_unit_variant("RecurrenceFrequency", 4u32, "Day"),
            Self::Week => serializer.serialize_unit_variant("RecurrenceFrequency", 5u32, "Week"),
            Self::Month => serializer.serialize_unit_variant("RecurrenceFrequency", 6u32, "Month"),
            Self::Year => serializer.serialize_unit_variant("RecurrenceFrequency", 7u32, "Year"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The recurrence schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecurrenceSchedule {
    #[doc = "The minutes."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub minutes: Vec<i32>,
    #[doc = "The hours."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hours: Vec<i32>,
    #[doc = "The days of the week."]
    #[serde(rename = "weekDays", default, skip_serializing_if = "Vec::is_empty")]
    pub week_days: Vec<String>,
    #[doc = "The month days."]
    #[serde(rename = "monthDays", default, skip_serializing_if = "Vec::is_empty")]
    pub month_days: Vec<i32>,
    #[doc = "The monthly occurrences."]
    #[serde(rename = "monthlyOccurrences", default, skip_serializing_if = "Vec::is_empty")]
    pub monthly_occurrences: Vec<RecurrenceScheduleOccurrence>,
}
impl RecurrenceSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The recurrence schedule occurrence."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecurrenceScheduleOccurrence {
    #[doc = "The day of the week."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<DayOfWeek>,
    #[doc = "The occurrence."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occurrence: Option<i32>,
}
impl RecurrenceScheduleOccurrence {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The access key regenerate action content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegenerateActionParameter {
    #[doc = "The key type."]
    #[serde(rename = "keyType", default, skip_serializing_if = "Option::is_none")]
    pub key_type: Option<KeyType>,
}
impl RegenerateActionParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow run action repetition index."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepetitionIndex {
    #[doc = "The scope."]
    #[serde(rename = "scopeName", default, skip_serializing_if = "Option::is_none")]
    pub scope_name: Option<String>,
    #[doc = "The index."]
    #[serde(rename = "itemIndex")]
    pub item_index: i32,
}
impl RepetitionIndex {
    pub fn new(item_index: i32) -> Self {
        Self {
            scope_name: None,
            item_index,
        }
    }
}
#[doc = "A request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Request {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Object>,
    #[doc = "The destination for the request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "The HTTP method used for the request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}
impl Request {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The request history."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestHistory {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The request history."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RequestHistoryProperties>,
}
impl RequestHistory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of workflow request histories."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestHistoryListResult {
    #[doc = "A list of workflow request histories."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RequestHistory>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RequestHistoryListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RequestHistoryListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The request history."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestHistoryProperties {
    #[doc = "The time the request started."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The time the request ended."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "A request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<Request>,
    #[doc = "A response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<Response>,
}
impl RequestHistoryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The base resource type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "The resource id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets the resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceReference {
    #[doc = "The resource id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets the resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ResourceReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Object>,
    #[doc = "The status code of the response."]
    #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
    #[doc = "The content link."]
    #[serde(rename = "bodyLink", default, skip_serializing_if = "Option::is_none")]
    pub body_link: Option<ContentLink>,
}
impl Response {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The retry history."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RetryHistory {
    #[doc = "Gets the start time."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the end time."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the status code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Gets the client request Id."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "Gets the service request Id."]
    #[serde(rename = "serviceRequestId", default, skip_serializing_if = "Option::is_none")]
    pub service_request_id: Option<String>,
    #[doc = "Error response indicates Logic service is not able to process the incoming request. The error property contains the error details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl RetryHistory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow run action correlation properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunActionCorrelation {
    #[serde(flatten)]
    pub run_correlation: RunCorrelation,
    #[doc = "The action tracking identifier."]
    #[serde(rename = "actionTrackingId", default, skip_serializing_if = "Option::is_none")]
    pub action_tracking_id: Option<String>,
}
impl RunActionCorrelation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The correlation properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunCorrelation {
    #[doc = "The client tracking identifier."]
    #[serde(rename = "clientTrackingId", default, skip_serializing_if = "Option::is_none")]
    pub client_tracking_id: Option<String>,
    #[doc = "The client keywords."]
    #[serde(rename = "clientKeywords", default, skip_serializing_if = "Vec::is_empty")]
    pub client_keywords: Vec<String>,
}
impl RunCorrelation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The schema type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SchemaType")]
pub enum SchemaType {
    NotSpecified,
    Xml,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SchemaType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SchemaType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SchemaType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("SchemaType", 0u32, "NotSpecified"),
            Self::Xml => serializer.serialize_unit_variant("SchemaType", 1u32, "Xml"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The segment terminator suffix."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SegmentTerminatorSuffix {
    NotSpecified,
    None,
    #[serde(rename = "CR")]
    Cr,
    #[serde(rename = "LF")]
    Lf,
    #[serde(rename = "CRLF")]
    Crlf,
}
#[doc = "The set trigger state action definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetTriggerStateActionDefinition {
    #[doc = "The workflow trigger reference."]
    pub source: WorkflowTriggerReference,
}
impl SetTriggerStateActionDefinition {
    pub fn new(source: WorkflowTriggerReference) -> Self {
        Self { source }
    }
}
#[doc = "The signing or hashing algorithm."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SigningAlgorithm")]
pub enum SigningAlgorithm {
    NotSpecified,
    Default,
    #[serde(rename = "SHA1")]
    Sha1,
    #[serde(rename = "SHA2256")]
    Sha2256,
    #[serde(rename = "SHA2384")]
    Sha2384,
    #[serde(rename = "SHA2512")]
    Sha2512,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SigningAlgorithm {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SigningAlgorithm {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SigningAlgorithm {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("SigningAlgorithm", 0u32, "NotSpecified"),
            Self::Default => serializer.serialize_unit_variant("SigningAlgorithm", 1u32, "Default"),
            Self::Sha1 => serializer.serialize_unit_variant("SigningAlgorithm", 2u32, "SHA1"),
            Self::Sha2256 => serializer.serialize_unit_variant("SigningAlgorithm", 3u32, "SHA2256"),
            Self::Sha2384 => serializer.serialize_unit_variant("SigningAlgorithm", 4u32, "SHA2384"),
            Self::Sha2512 => serializer.serialize_unit_variant("SigningAlgorithm", 5u32, "SHA2512"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The sku type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The sku name."]
    pub name: SkuName,
    #[doc = "The resource reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<ResourceReference>,
}
impl Sku {
    pub fn new(name: SkuName) -> Self {
        Self { name, plan: None }
    }
}
#[doc = "The sku name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SkuName")]
pub enum SkuName {
    NotSpecified,
    Free,
    Shared,
    Basic,
    Standard,
    Premium,
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
            Self::NotSpecified => serializer.serialize_unit_variant("SkuName", 0u32, "NotSpecified"),
            Self::Free => serializer.serialize_unit_variant("SkuName", 1u32, "Free"),
            Self::Shared => serializer.serialize_unit_variant("SkuName", 2u32, "Shared"),
            Self::Basic => serializer.serialize_unit_variant("SkuName", 3u32, "Basic"),
            Self::Standard => serializer.serialize_unit_variant("SkuName", 4u32, "Standard"),
            Self::Premium => serializer.serialize_unit_variant("SkuName", 5u32, "Premium"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The status annotation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "StatusAnnotation")]
pub enum StatusAnnotation {
    NotSpecified,
    Preview,
    Production,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for StatusAnnotation {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for StatusAnnotation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for StatusAnnotation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("StatusAnnotation", 0u32, "NotSpecified"),
            Self::Preview => serializer.serialize_unit_variant("StatusAnnotation", 1u32, "Preview"),
            Self::Production => serializer.serialize_unit_variant("StatusAnnotation", 2u32, "Production"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The sub resource type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResource {
    #[doc = "The resource id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SubResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The swagger custom dynamic list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SwaggerCustomDynamicList {
    #[doc = "The operation id to fetch dynamic schema."]
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[doc = "The built in operation."]
    #[serde(rename = "builtInOperation", default, skip_serializing_if = "Option::is_none")]
    pub built_in_operation: Option<String>,
    #[doc = "The path to a response property (relative to the response object, not the response body) which contains an array of dynamic value items."]
    #[serde(rename = "itemsPath", default, skip_serializing_if = "Option::is_none")]
    pub items_path: Option<String>,
    #[doc = "The path to a property which defines the value which should be used."]
    #[serde(rename = "itemValuePath", default, skip_serializing_if = "Option::is_none")]
    pub item_value_path: Option<String>,
    #[doc = "The path to an item property which defines the display name of the item."]
    #[serde(rename = "itemTitlePath", default, skip_serializing_if = "Option::is_none")]
    pub item_title_path: Option<String>,
    #[doc = "The parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
impl SwaggerCustomDynamicList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The swagger custom dynamic properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SwaggerCustomDynamicProperties {
    #[doc = "The operation id to fetch dynamic schema."]
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[doc = "Json pointer to the dynamic schema on the response body."]
    #[serde(rename = "valuePath", default, skip_serializing_if = "Option::is_none")]
    pub value_path: Option<String>,
    #[doc = "The operation parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
impl SwaggerCustomDynamicProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The swagger custom dynamic schema."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SwaggerCustomDynamicSchema {
    #[doc = "The operation id to fetch dynamic schema."]
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[doc = "Json pointer to the dynamic schema on the response body."]
    #[serde(rename = "valuePath", default, skip_serializing_if = "Option::is_none")]
    pub value_path: Option<String>,
    #[doc = "The operation parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
impl SwaggerCustomDynamicSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The swagger custom dynamic tree."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SwaggerCustomDynamicTree {
    #[doc = "The swagger custom dynamic tree settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<SwaggerCustomDynamicTreeSettings>,
    #[doc = "The swagger tree command."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open: Option<SwaggerCustomDynamicTreeCommand>,
    #[doc = "The swagger tree command."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browse: Option<SwaggerCustomDynamicTreeCommand>,
}
impl SwaggerCustomDynamicTree {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The swagger tree command."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SwaggerCustomDynamicTreeCommand {
    #[doc = "The path to an item property which defines the display name of the item."]
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[doc = "The path to an item property which defines the display name of the item."]
    #[serde(rename = "itemsPath", default, skip_serializing_if = "Option::is_none")]
    pub items_path: Option<String>,
    #[doc = "The path to an item property which defines the display name of the item."]
    #[serde(rename = "itemValuePath", default, skip_serializing_if = "Option::is_none")]
    pub item_value_path: Option<String>,
    #[doc = "The path to an item property which defines the display name of the item."]
    #[serde(rename = "itemTitlePath", default, skip_serializing_if = "Option::is_none")]
    pub item_title_path: Option<String>,
    #[doc = "The path to an item property which defines the display name of the item."]
    #[serde(rename = "itemFullTitlePath", default, skip_serializing_if = "Option::is_none")]
    pub item_full_title_path: Option<String>,
    #[doc = "The path to an item property which defines the display name of the item."]
    #[serde(rename = "itemIsParent", default, skip_serializing_if = "Option::is_none")]
    pub item_is_parent: Option<String>,
    #[doc = "The path to an item property which defines the display name of the item."]
    #[serde(rename = "selectableFilter", default, skip_serializing_if = "Option::is_none")]
    pub selectable_filter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
impl SwaggerCustomDynamicTreeCommand {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The swagger custom dynamic tree parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SwaggerCustomDynamicTreeParameter {
    #[doc = "Gets or sets a path to a property in the currently selected item to pass as a value to a parameter for the given operation."]
    #[serde(rename = "selectedItemValuePath", default, skip_serializing_if = "Option::is_none")]
    pub selected_item_value_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Object>,
    #[doc = "The parameter reference."]
    #[serde(rename = "parameterReference", default, skip_serializing_if = "Option::is_none")]
    pub parameter_reference: Option<String>,
    #[doc = "Indicates whether the parameter is required."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}
impl SwaggerCustomDynamicTreeParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The swagger custom dynamic tree settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SwaggerCustomDynamicTreeSettings {
    #[doc = "Indicates whether parent nodes can be selected."]
    #[serde(rename = "CanSelectParentNodes", default, skip_serializing_if = "Option::is_none")]
    pub can_select_parent_nodes: Option<bool>,
    #[doc = "Indicates whether leaf nodes can be selected."]
    #[serde(rename = "CanSelectLeafNodes", default, skip_serializing_if = "Option::is_none")]
    pub can_select_leaf_nodes: Option<bool>,
}
impl SwaggerCustomDynamicTreeSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The swagger external documentation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SwaggerExternalDocumentation {
    #[doc = "The document description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The documentation Uri."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "The vendor extensions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<serde_json::Value>,
}
impl SwaggerExternalDocumentation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The swagger schema."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SwaggerSchema {
    #[doc = "The reference."]
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,
    #[doc = "The swagger schema type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<SwaggerSchemaType>,
    #[doc = "The title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The swagger schema."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Box<Option<SwaggerSchema>>,
    #[doc = "The object properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<Object>,
    #[doc = "The object required properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,
    #[doc = "The maximum number of allowed properties."]
    #[serde(rename = "maxProperties", default, skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<i64>,
    #[doc = "The minimum number of allowed properties."]
    #[serde(rename = "minProperties", default, skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<i64>,
    #[doc = "The schemas which must pass validation when this schema is used."]
    #[serde(rename = "allOf", default, skip_serializing_if = "Vec::is_empty")]
    pub all_of: Vec<SwaggerSchema>,
    #[doc = "The discriminator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<String>,
    #[doc = "Indicates whether this property must be present in the a request."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "The Swagger XML."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xml: Option<SwaggerXml>,
    #[doc = "The swagger external documentation"]
    #[serde(rename = "externalDocs", default, skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<SwaggerExternalDocumentation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<Object>,
    #[doc = "Indicates the notification url extension. If this is set, the property's value should be a callback url for a webhook."]
    #[serde(rename = "notificationUrlExtension", default, skip_serializing_if = "Option::is_none")]
    pub notification_url_extension: Option<bool>,
    #[doc = "The swagger custom dynamic schema."]
    #[serde(rename = "dynamicSchemaOld", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_schema_old: Option<SwaggerCustomDynamicSchema>,
    #[doc = "The swagger custom dynamic properties."]
    #[serde(rename = "dynamicSchemaNew", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_schema_new: Option<SwaggerCustomDynamicProperties>,
    #[doc = "The swagger custom dynamic list."]
    #[serde(rename = "dynamicListNew", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_list_new: Option<SwaggerCustomDynamicList>,
    #[doc = "The swagger custom dynamic tree."]
    #[serde(rename = "dynamicTree", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_tree: Option<SwaggerCustomDynamicTree>,
}
impl SwaggerSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The swagger schema type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SwaggerSchemaType")]
pub enum SwaggerSchemaType {
    String,
    Number,
    Integer,
    Boolean,
    Array,
    File,
    Object,
    Null,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SwaggerSchemaType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SwaggerSchemaType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SwaggerSchemaType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::String => serializer.serialize_unit_variant("SwaggerSchemaType", 0u32, "String"),
            Self::Number => serializer.serialize_unit_variant("SwaggerSchemaType", 1u32, "Number"),
            Self::Integer => serializer.serialize_unit_variant("SwaggerSchemaType", 2u32, "Integer"),
            Self::Boolean => serializer.serialize_unit_variant("SwaggerSchemaType", 3u32, "Boolean"),
            Self::Array => serializer.serialize_unit_variant("SwaggerSchemaType", 4u32, "Array"),
            Self::File => serializer.serialize_unit_variant("SwaggerSchemaType", 5u32, "File"),
            Self::Object => serializer.serialize_unit_variant("SwaggerSchemaType", 6u32, "Object"),
            Self::Null => serializer.serialize_unit_variant("SwaggerSchemaType", 7u32, "Null"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The Swagger XML."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SwaggerXml {
    #[doc = "The xml element or attribute name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The xml namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "The name prefix."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[doc = "Indicates whether the property should be an attribute instead of an element."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribute: Option<bool>,
    #[doc = "Indicates whether the array elements are wrapped in a container element."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wrapped: Option<bool>,
    #[doc = "The vendor extensions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<serde_json::Value>,
}
impl SwaggerXml {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The track events operation options."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TrackEventsOperationOptions")]
pub enum TrackEventsOperationOptions {
    None,
    DisableSourceInfoEnrich,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TrackEventsOperationOptions {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TrackEventsOperationOptions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TrackEventsOperationOptions {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("TrackEventsOperationOptions", 0u32, "None"),
            Self::DisableSourceInfoEnrich => {
                serializer.serialize_unit_variant("TrackEventsOperationOptions", 1u32, "DisableSourceInfoEnrich")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The tracking event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackingEvent {
    #[doc = "The event level."]
    #[serde(rename = "eventLevel")]
    pub event_level: EventLevel,
    #[doc = "The event time."]
    #[serde(rename = "eventTime", with = "azure_core::date::rfc3339")]
    pub event_time: time::OffsetDateTime,
    #[doc = "The tracking record type."]
    #[serde(rename = "recordType")]
    pub record_type: TrackingRecordType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record: Option<Object>,
    #[doc = "The tracking event error info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<TrackingEventErrorInfo>,
}
impl TrackingEvent {
    pub fn new(event_level: EventLevel, event_time: time::OffsetDateTime, record_type: TrackingRecordType) -> Self {
        Self {
            event_level,
            event_time,
            record_type,
            record: None,
            error: None,
        }
    }
}
#[doc = "The tracking event error info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackingEventErrorInfo {
    #[doc = "The message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}
impl TrackingEventErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The tracking events definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackingEventsDefinition {
    #[doc = "The source type."]
    #[serde(rename = "sourceType")]
    pub source_type: String,
    #[doc = "The track events operation options."]
    #[serde(rename = "trackEventsOptions", default, skip_serializing_if = "Option::is_none")]
    pub track_events_options: Option<TrackEventsOperationOptions>,
    #[doc = "The events."]
    pub events: Vec<TrackingEvent>,
}
impl TrackingEventsDefinition {
    pub fn new(source_type: String, events: Vec<TrackingEvent>) -> Self {
        Self {
            source_type,
            track_events_options: None,
            events,
        }
    }
}
#[doc = "The tracking record type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TrackingRecordType")]
pub enum TrackingRecordType {
    NotSpecified,
    Custom,
    #[serde(rename = "AS2Message")]
    As2Message,
    #[serde(rename = "AS2MDN")]
    As2mdn,
    X12Interchange,
    X12FunctionalGroup,
    X12TransactionSet,
    X12InterchangeAcknowledgment,
    X12FunctionalGroupAcknowledgment,
    X12TransactionSetAcknowledgment,
    EdifactInterchange,
    EdifactFunctionalGroup,
    EdifactTransactionSet,
    EdifactInterchangeAcknowledgment,
    EdifactFunctionalGroupAcknowledgment,
    EdifactTransactionSetAcknowledgment,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TrackingRecordType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TrackingRecordType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TrackingRecordType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("TrackingRecordType", 0u32, "NotSpecified"),
            Self::Custom => serializer.serialize_unit_variant("TrackingRecordType", 1u32, "Custom"),
            Self::As2Message => serializer.serialize_unit_variant("TrackingRecordType", 2u32, "AS2Message"),
            Self::As2mdn => serializer.serialize_unit_variant("TrackingRecordType", 3u32, "AS2MDN"),
            Self::X12Interchange => serializer.serialize_unit_variant("TrackingRecordType", 4u32, "X12Interchange"),
            Self::X12FunctionalGroup => serializer.serialize_unit_variant("TrackingRecordType", 5u32, "X12FunctionalGroup"),
            Self::X12TransactionSet => serializer.serialize_unit_variant("TrackingRecordType", 6u32, "X12TransactionSet"),
            Self::X12InterchangeAcknowledgment => {
                serializer.serialize_unit_variant("TrackingRecordType", 7u32, "X12InterchangeAcknowledgment")
            }
            Self::X12FunctionalGroupAcknowledgment => {
                serializer.serialize_unit_variant("TrackingRecordType", 8u32, "X12FunctionalGroupAcknowledgment")
            }
            Self::X12TransactionSetAcknowledgment => {
                serializer.serialize_unit_variant("TrackingRecordType", 9u32, "X12TransactionSetAcknowledgment")
            }
            Self::EdifactInterchange => serializer.serialize_unit_variant("TrackingRecordType", 10u32, "EdifactInterchange"),
            Self::EdifactFunctionalGroup => serializer.serialize_unit_variant("TrackingRecordType", 11u32, "EdifactFunctionalGroup"),
            Self::EdifactTransactionSet => serializer.serialize_unit_variant("TrackingRecordType", 12u32, "EdifactTransactionSet"),
            Self::EdifactInterchangeAcknowledgment => {
                serializer.serialize_unit_variant("TrackingRecordType", 13u32, "EdifactInterchangeAcknowledgment")
            }
            Self::EdifactFunctionalGroupAcknowledgment => {
                serializer.serialize_unit_variant("TrackingRecordType", 14u32, "EdifactFunctionalGroupAcknowledgment")
            }
            Self::EdifactTransactionSetAcknowledgment => {
                serializer.serialize_unit_variant("TrackingRecordType", 15u32, "EdifactTransactionSetAcknowledgment")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The trailing separator policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TrailingSeparatorPolicy")]
pub enum TrailingSeparatorPolicy {
    NotSpecified,
    NotAllowed,
    Optional,
    Mandatory,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TrailingSeparatorPolicy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TrailingSeparatorPolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TrailingSeparatorPolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("TrailingSeparatorPolicy", 0u32, "NotSpecified"),
            Self::NotAllowed => serializer.serialize_unit_variant("TrailingSeparatorPolicy", 1u32, "NotAllowed"),
            Self::Optional => serializer.serialize_unit_variant("TrailingSeparatorPolicy", 2u32, "Optional"),
            Self::Mandatory => serializer.serialize_unit_variant("TrailingSeparatorPolicy", 3u32, "Mandatory"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The usage indicator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UsageIndicator")]
pub enum UsageIndicator {
    NotSpecified,
    Test,
    Information,
    Production,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UsageIndicator {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UsageIndicator {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UsageIndicator {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("UsageIndicator", 0u32, "NotSpecified"),
            Self::Test => serializer.serialize_unit_variant("UsageIndicator", 1u32, "Test"),
            Self::Information => serializer.serialize_unit_variant("UsageIndicator", 2u32, "Information"),
            Self::Production => serializer.serialize_unit_variant("UsageIndicator", 3u32, "Production"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The workflow type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Workflow {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The workflow properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkflowProperties>,
    #[doc = "Managed service identity properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl Workflow {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowFilter {
    #[doc = "The workflow state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<WorkflowState>,
}
impl WorkflowFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of workflows."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowListResult {
    #[doc = "The list of workflows."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workflow>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkflowListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkflowListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow output parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowOutputParameter {
    #[serde(flatten)]
    pub workflow_parameter: WorkflowParameter,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Object>,
}
impl WorkflowOutputParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowParameter {
    #[doc = "The parameter type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ParameterType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Object>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Object>,
    #[doc = "The description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl WorkflowParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowProperties {
    #[doc = "The workflow provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<WorkflowProvisioningState>,
    #[doc = "Gets the created time."]
    #[serde(rename = "createdTime", with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the changed time."]
    #[serde(rename = "changedTime", with = "azure_core::date::rfc3339::option")]
    pub changed_time: Option<time::OffsetDateTime>,
    #[doc = "The workflow state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<WorkflowState>,
    #[doc = "Gets the version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Gets the access endpoint."]
    #[serde(rename = "accessEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub access_endpoint: Option<String>,
    #[doc = "The endpoints configuration."]
    #[serde(rename = "endpointsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub endpoints_configuration: Option<FlowEndpointsConfiguration>,
    #[doc = "The access control configuration."]
    #[serde(rename = "accessControl", default, skip_serializing_if = "Option::is_none")]
    pub access_control: Option<FlowAccessControlConfiguration>,
    #[doc = "The sku type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "The resource reference."]
    #[serde(rename = "integrationAccount", default, skip_serializing_if = "Option::is_none")]
    pub integration_account: Option<ResourceReference>,
    #[doc = "The resource reference."]
    #[serde(rename = "integrationServiceEnvironment", default, skip_serializing_if = "Option::is_none")]
    pub integration_service_environment: Option<ResourceReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<Object>,
    #[doc = "The parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
impl WorkflowProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WorkflowProvisioningState")]
pub enum WorkflowProvisioningState {
    NotSpecified,
    Accepted,
    Running,
    Ready,
    Creating,
    Created,
    Deleting,
    Deleted,
    Canceled,
    Failed,
    Succeeded,
    Moving,
    Updating,
    Registering,
    Registered,
    Unregistering,
    Unregistered,
    Completed,
    Renewing,
    Pending,
    Waiting,
    InProgress,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WorkflowProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WorkflowProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WorkflowProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("WorkflowProvisioningState", 0u32, "NotSpecified"),
            Self::Accepted => serializer.serialize_unit_variant("WorkflowProvisioningState", 1u32, "Accepted"),
            Self::Running => serializer.serialize_unit_variant("WorkflowProvisioningState", 2u32, "Running"),
            Self::Ready => serializer.serialize_unit_variant("WorkflowProvisioningState", 3u32, "Ready"),
            Self::Creating => serializer.serialize_unit_variant("WorkflowProvisioningState", 4u32, "Creating"),
            Self::Created => serializer.serialize_unit_variant("WorkflowProvisioningState", 5u32, "Created"),
            Self::Deleting => serializer.serialize_unit_variant("WorkflowProvisioningState", 6u32, "Deleting"),
            Self::Deleted => serializer.serialize_unit_variant("WorkflowProvisioningState", 7u32, "Deleted"),
            Self::Canceled => serializer.serialize_unit_variant("WorkflowProvisioningState", 8u32, "Canceled"),
            Self::Failed => serializer.serialize_unit_variant("WorkflowProvisioningState", 9u32, "Failed"),
            Self::Succeeded => serializer.serialize_unit_variant("WorkflowProvisioningState", 10u32, "Succeeded"),
            Self::Moving => serializer.serialize_unit_variant("WorkflowProvisioningState", 11u32, "Moving"),
            Self::Updating => serializer.serialize_unit_variant("WorkflowProvisioningState", 12u32, "Updating"),
            Self::Registering => serializer.serialize_unit_variant("WorkflowProvisioningState", 13u32, "Registering"),
            Self::Registered => serializer.serialize_unit_variant("WorkflowProvisioningState", 14u32, "Registered"),
            Self::Unregistering => serializer.serialize_unit_variant("WorkflowProvisioningState", 15u32, "Unregistering"),
            Self::Unregistered => serializer.serialize_unit_variant("WorkflowProvisioningState", 16u32, "Unregistered"),
            Self::Completed => serializer.serialize_unit_variant("WorkflowProvisioningState", 17u32, "Completed"),
            Self::Renewing => serializer.serialize_unit_variant("WorkflowProvisioningState", 18u32, "Renewing"),
            Self::Pending => serializer.serialize_unit_variant("WorkflowProvisioningState", 19u32, "Pending"),
            Self::Waiting => serializer.serialize_unit_variant("WorkflowProvisioningState", 20u32, "Waiting"),
            Self::InProgress => serializer.serialize_unit_variant("WorkflowProvisioningState", 21u32, "InProgress"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The workflow reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowReference {
    #[serde(flatten)]
    pub resource_reference: ResourceReference,
    #[doc = "The workflow name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl WorkflowReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowRun {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The workflow run properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkflowRunProperties>,
    #[doc = "Gets the workflow run name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the workflow run type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl WorkflowRun {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow run action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowRunAction {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The workflow run action properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkflowRunActionProperties>,
    #[doc = "Gets the workflow run action name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the workflow run action type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl WorkflowRunAction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow run action filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowRunActionFilter {
    #[doc = "The workflow status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkflowStatus>,
}
impl WorkflowRunActionFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of workflow run actions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowRunActionListResult {
    #[doc = "A list of workflow run actions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkflowRunAction>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkflowRunActionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkflowRunActionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow run action properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowRunActionProperties {
    #[doc = "Gets the start time."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the end time."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The workflow status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkflowStatus>,
    #[doc = "Gets the code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Object>,
    #[doc = "Gets the tracking id."]
    #[serde(rename = "trackingId", default, skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[doc = "The workflow run action correlation properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correlation: Option<RunActionCorrelation>,
    #[doc = "The content link."]
    #[serde(rename = "inputsLink", default, skip_serializing_if = "Option::is_none")]
    pub inputs_link: Option<ContentLink>,
    #[doc = "The content link."]
    #[serde(rename = "outputsLink", default, skip_serializing_if = "Option::is_none")]
    pub outputs_link: Option<ContentLink>,
    #[serde(rename = "trackedProperties", default, skip_serializing_if = "Option::is_none")]
    pub tracked_properties: Option<Object>,
    #[doc = "Gets the retry histories."]
    #[serde(rename = "retryHistory", default, skip_serializing_if = "Vec::is_empty")]
    pub retry_history: Vec<RetryHistory>,
}
impl WorkflowRunActionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow run action repetition definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkflowRunActionRepetitionDefinition {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The workflow run action repetition properties definition."]
    pub properties: WorkflowRunActionRepetitionProperties,
}
impl WorkflowRunActionRepetitionDefinition {
    pub fn new(properties: WorkflowRunActionRepetitionProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[doc = "A collection of workflow run action repetitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowRunActionRepetitionDefinitionCollection {
    #[doc = "The link used to get the next page of recommendations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkflowRunActionRepetitionDefinition>,
}
impl azure_core::Continuable for WorkflowRunActionRepetitionDefinitionCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl WorkflowRunActionRepetitionDefinitionCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow run action repetition properties definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowRunActionRepetitionProperties {
    #[serde(flatten)]
    pub operation_result: OperationResult,
    #[doc = "The repetition indexes."]
    #[serde(rename = "repetitionIndexes", default, skip_serializing_if = "Vec::is_empty")]
    pub repetition_indexes: Vec<RepetitionIndex>,
}
impl WorkflowRunActionRepetitionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow run filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowRunFilter {
    #[doc = "The workflow status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkflowStatus>,
}
impl WorkflowRunFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of workflow runs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowRunListResult {
    #[doc = "A list of workflow runs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkflowRun>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkflowRunListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkflowRunListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow run properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowRunProperties {
    #[doc = "Gets the wait end time."]
    #[serde(rename = "waitEndTime", with = "azure_core::date::rfc3339::option")]
    pub wait_end_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the start time."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the end time."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The workflow status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkflowStatus>,
    #[doc = "Gets the code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Object>,
    #[doc = "Gets the correlation id."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The correlation property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correlation: Option<Correlation>,
    #[doc = "The resource reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow: Option<ResourceReference>,
    #[doc = "The workflow run trigger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<WorkflowRunTrigger>,
    #[doc = "Gets the outputs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
    #[doc = "The workflow run trigger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<WorkflowRunTrigger>,
}
impl WorkflowRunProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow run trigger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowRunTrigger {
    #[doc = "Gets the name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Object>,
    #[doc = "The content link."]
    #[serde(rename = "inputsLink", default, skip_serializing_if = "Option::is_none")]
    pub inputs_link: Option<ContentLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Object>,
    #[doc = "The content link."]
    #[serde(rename = "outputsLink", default, skip_serializing_if = "Option::is_none")]
    pub outputs_link: Option<ContentLink>,
    #[doc = "Gets the scheduled time."]
    #[serde(rename = "scheduledTime", with = "azure_core::date::rfc3339::option")]
    pub scheduled_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the start time."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the end time."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the tracking id."]
    #[serde(rename = "trackingId", default, skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[doc = "The correlation property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correlation: Option<Correlation>,
    #[doc = "Gets the code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The workflow status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkflowStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Object>,
    #[serde(rename = "trackedProperties", default, skip_serializing_if = "Option::is_none")]
    pub tracked_properties: Option<Object>,
}
impl WorkflowRunTrigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WorkflowState")]
pub enum WorkflowState {
    NotSpecified,
    Completed,
    Enabled,
    Disabled,
    Deleted,
    Suspended,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WorkflowState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WorkflowState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WorkflowState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("WorkflowState", 0u32, "NotSpecified"),
            Self::Completed => serializer.serialize_unit_variant("WorkflowState", 1u32, "Completed"),
            Self::Enabled => serializer.serialize_unit_variant("WorkflowState", 2u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("WorkflowState", 3u32, "Disabled"),
            Self::Deleted => serializer.serialize_unit_variant("WorkflowState", 4u32, "Deleted"),
            Self::Suspended => serializer.serialize_unit_variant("WorkflowState", 5u32, "Suspended"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The workflow status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WorkflowStatus")]
pub enum WorkflowStatus {
    NotSpecified,
    Paused,
    Running,
    Waiting,
    Succeeded,
    Skipped,
    Suspended,
    Cancelled,
    Failed,
    Faulted,
    TimedOut,
    Aborted,
    Ignored,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WorkflowStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WorkflowStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WorkflowStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("WorkflowStatus", 0u32, "NotSpecified"),
            Self::Paused => serializer.serialize_unit_variant("WorkflowStatus", 1u32, "Paused"),
            Self::Running => serializer.serialize_unit_variant("WorkflowStatus", 2u32, "Running"),
            Self::Waiting => serializer.serialize_unit_variant("WorkflowStatus", 3u32, "Waiting"),
            Self::Succeeded => serializer.serialize_unit_variant("WorkflowStatus", 4u32, "Succeeded"),
            Self::Skipped => serializer.serialize_unit_variant("WorkflowStatus", 5u32, "Skipped"),
            Self::Suspended => serializer.serialize_unit_variant("WorkflowStatus", 6u32, "Suspended"),
            Self::Cancelled => serializer.serialize_unit_variant("WorkflowStatus", 7u32, "Cancelled"),
            Self::Failed => serializer.serialize_unit_variant("WorkflowStatus", 8u32, "Failed"),
            Self::Faulted => serializer.serialize_unit_variant("WorkflowStatus", 9u32, "Faulted"),
            Self::TimedOut => serializer.serialize_unit_variant("WorkflowStatus", 10u32, "TimedOut"),
            Self::Aborted => serializer.serialize_unit_variant("WorkflowStatus", 11u32, "Aborted"),
            Self::Ignored => serializer.serialize_unit_variant("WorkflowStatus", 12u32, "Ignored"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The workflow trigger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowTrigger {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The workflow trigger properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkflowTriggerProperties>,
    #[doc = "Gets the workflow trigger name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the workflow trigger type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl WorkflowTrigger {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow trigger callback URL."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowTriggerCallbackUrl {
    #[doc = "Gets the workflow trigger callback URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Gets the workflow trigger callback URL HTTP method."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[doc = "Gets the workflow trigger callback URL base path."]
    #[serde(rename = "basePath", default, skip_serializing_if = "Option::is_none")]
    pub base_path: Option<String>,
    #[doc = "Gets the workflow trigger callback URL relative path."]
    #[serde(rename = "relativePath", default, skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
    #[doc = "Gets the workflow trigger callback URL relative path parameters."]
    #[serde(rename = "relativePathParameters", default, skip_serializing_if = "Vec::is_empty")]
    pub relative_path_parameters: Vec<String>,
    #[doc = "Gets the workflow trigger callback URL query parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queries: Option<WorkflowTriggerListCallbackUrlQueries>,
}
impl WorkflowTriggerCallbackUrl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow trigger filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowTriggerFilter {
    #[doc = "The workflow state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<WorkflowState>,
}
impl WorkflowTriggerFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow trigger history."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowTriggerHistory {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The workflow trigger history properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkflowTriggerHistoryProperties>,
    #[doc = "Gets the workflow trigger history name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the workflow trigger history type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl WorkflowTriggerHistory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow trigger history filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowTriggerHistoryFilter {
    #[doc = "The workflow status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkflowStatus>,
}
impl WorkflowTriggerHistoryFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of workflow trigger histories."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowTriggerHistoryListResult {
    #[doc = "A list of workflow trigger histories."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkflowTriggerHistory>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkflowTriggerHistoryListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkflowTriggerHistoryListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow trigger history properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowTriggerHistoryProperties {
    #[doc = "Gets the start time."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the end time."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The scheduled time."]
    #[serde(rename = "scheduledTime", with = "azure_core::date::rfc3339::option")]
    pub scheduled_time: Option<time::OffsetDateTime>,
    #[doc = "The workflow status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkflowStatus>,
    #[doc = "Gets the code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Object>,
    #[doc = "Gets the tracking id."]
    #[serde(rename = "trackingId", default, skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
    #[doc = "The correlation property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correlation: Option<Correlation>,
    #[doc = "The content link."]
    #[serde(rename = "inputsLink", default, skip_serializing_if = "Option::is_none")]
    pub inputs_link: Option<ContentLink>,
    #[doc = "The content link."]
    #[serde(rename = "outputsLink", default, skip_serializing_if = "Option::is_none")]
    pub outputs_link: Option<ContentLink>,
    #[doc = "The value indicating whether trigger was fired."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fired: Option<bool>,
    #[doc = "The resource reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run: Option<ResourceReference>,
}
impl WorkflowTriggerHistoryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Gets the workflow trigger callback URL query parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowTriggerListCallbackUrlQueries {
    #[doc = "The api version."]
    #[serde(rename = "api-version", default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[doc = "The SAS permissions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sp: Option<String>,
    #[doc = "The SAS version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sv: Option<String>,
    #[doc = "The SAS signature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sig: Option<String>,
    #[doc = "The SAS timestamp."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub se: Option<String>,
}
impl WorkflowTriggerListCallbackUrlQueries {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of workflow triggers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowTriggerListResult {
    #[doc = "A list of workflow triggers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkflowTrigger>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkflowTriggerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkflowTriggerListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow trigger properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowTriggerProperties {
    #[doc = "The workflow trigger provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<WorkflowTriggerProvisioningState>,
    #[doc = "Gets the created time."]
    #[serde(rename = "createdTime", with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the changed time."]
    #[serde(rename = "changedTime", with = "azure_core::date::rfc3339::option")]
    pub changed_time: Option<time::OffsetDateTime>,
    #[doc = "The workflow state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<WorkflowState>,
    #[doc = "The workflow status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkflowStatus>,
    #[doc = "Gets the last execution time."]
    #[serde(rename = "lastExecutionTime", with = "azure_core::date::rfc3339::option")]
    pub last_execution_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the next execution time."]
    #[serde(rename = "nextExecutionTime", with = "azure_core::date::rfc3339::option")]
    pub next_execution_time: Option<time::OffsetDateTime>,
    #[doc = "The workflow trigger recurrence."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<WorkflowTriggerRecurrence>,
    #[doc = "The resource reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow: Option<ResourceReference>,
}
impl WorkflowTriggerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow trigger provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WorkflowTriggerProvisioningState")]
pub enum WorkflowTriggerProvisioningState {
    NotSpecified,
    Accepted,
    Running,
    Ready,
    Creating,
    Created,
    Deleting,
    Deleted,
    Canceled,
    Failed,
    Succeeded,
    Moving,
    Updating,
    Registering,
    Registered,
    Unregistering,
    Unregistered,
    Completed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WorkflowTriggerProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WorkflowTriggerProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WorkflowTriggerProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("WorkflowTriggerProvisioningState", 0u32, "NotSpecified"),
            Self::Accepted => serializer.serialize_unit_variant("WorkflowTriggerProvisioningState", 1u32, "Accepted"),
            Self::Running => serializer.serialize_unit_variant("WorkflowTriggerProvisioningState", 2u32, "Running"),
            Self::Ready => serializer.serialize_unit_variant("WorkflowTriggerProvisioningState", 3u32, "Ready"),
            Self::Creating => serializer.serialize_unit_variant("WorkflowTriggerProvisioningState", 4u32, "Creating"),
            Self::Created => serializer.serialize_unit_variant("WorkflowTriggerProvisioningState", 5u32, "Created"),
            Self::Deleting => serializer.serialize_unit_variant("WorkflowTriggerProvisioningState", 6u32, "Deleting"),
            Self::Deleted => serializer.serialize_unit_variant("WorkflowTriggerProvisioningState", 7u32, "Deleted"),
            Self::Canceled => serializer.serialize_unit_variant("WorkflowTriggerProvisioningState", 8u32, "Canceled"),
            Self::Failed => serializer.serialize_unit_variant("WorkflowTriggerProvisioningState", 9u32, "Failed"),
            Self::Succeeded => serializer.serialize_unit_variant("WorkflowTriggerProvisioningState", 10u32, "Succeeded"),
            Self::Moving => serializer.serialize_unit_variant("WorkflowTriggerProvisioningState", 11u32, "Moving"),
            Self::Updating => serializer.serialize_unit_variant("WorkflowTriggerProvisioningState", 12u32, "Updating"),
            Self::Registering => serializer.serialize_unit_variant("WorkflowTriggerProvisioningState", 13u32, "Registering"),
            Self::Registered => serializer.serialize_unit_variant("WorkflowTriggerProvisioningState", 14u32, "Registered"),
            Self::Unregistering => serializer.serialize_unit_variant("WorkflowTriggerProvisioningState", 15u32, "Unregistering"),
            Self::Unregistered => serializer.serialize_unit_variant("WorkflowTriggerProvisioningState", 16u32, "Unregistered"),
            Self::Completed => serializer.serialize_unit_variant("WorkflowTriggerProvisioningState", 17u32, "Completed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The workflow trigger recurrence."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowTriggerRecurrence {
    #[doc = "The recurrence frequency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<RecurrenceFrequency>,
    #[doc = "The interval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i32>,
    #[doc = "The start time."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The end time."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The time zone."]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[doc = "The recurrence schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<RecurrenceSchedule>,
}
impl WorkflowTriggerRecurrence {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow trigger reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowTriggerReference {
    #[serde(flatten)]
    pub resource_reference: ResourceReference,
    #[doc = "The workflow trigger resource reference name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The workflow name."]
    #[serde(rename = "flowName", default, skip_serializing_if = "Option::is_none")]
    pub flow_name: Option<String>,
    #[doc = "The workflow trigger name."]
    #[serde(rename = "triggerName", default, skip_serializing_if = "Option::is_none")]
    pub trigger_name: Option<String>,
}
impl WorkflowTriggerReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowVersion {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The workflow version properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkflowVersionProperties>,
}
impl WorkflowVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of workflow versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowVersionListResult {
    #[doc = "A list of workflow versions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WorkflowVersion>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkflowVersionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkflowVersionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workflow version properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowVersionProperties {
    #[doc = "The workflow provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<WorkflowProvisioningState>,
    #[doc = "Gets the created time."]
    #[serde(rename = "createdTime", with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "Gets the changed time."]
    #[serde(rename = "changedTime", with = "azure_core::date::rfc3339::option")]
    pub changed_time: Option<time::OffsetDateTime>,
    #[doc = "The workflow state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<WorkflowState>,
    #[doc = "Gets the version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Gets the access endpoint."]
    #[serde(rename = "accessEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub access_endpoint: Option<String>,
    #[doc = "The endpoints configuration."]
    #[serde(rename = "endpointsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub endpoints_configuration: Option<FlowEndpointsConfiguration>,
    #[doc = "The access control configuration."]
    #[serde(rename = "accessControl", default, skip_serializing_if = "Option::is_none")]
    pub access_control: Option<FlowAccessControlConfiguration>,
    #[doc = "The sku type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "The resource reference."]
    #[serde(rename = "integrationAccount", default, skip_serializing_if = "Option::is_none")]
    pub integration_account: Option<ResourceReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<Object>,
    #[doc = "The parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
impl WorkflowVersionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The WSDL import method."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WsdlImportMethod")]
pub enum WsdlImportMethod {
    NotSpecified,
    SoapToRest,
    SoapPassThrough,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WsdlImportMethod {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WsdlImportMethod {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WsdlImportMethod {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("WsdlImportMethod", 0u32, "NotSpecified"),
            Self::SoapToRest => serializer.serialize_unit_variant("WsdlImportMethod", 1u32, "SoapToRest"),
            Self::SoapPassThrough => serializer.serialize_unit_variant("WsdlImportMethod", 2u32, "SoapPassThrough"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The WSDL service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WsdlService {
    #[doc = "The qualified name."]
    #[serde(rename = "qualifiedName", default, skip_serializing_if = "Option::is_none")]
    pub qualified_name: Option<String>,
    #[doc = "The list of endpoints' qualified names."]
    #[serde(rename = "EndpointQualifiedNames", default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint_qualified_names: Vec<String>,
}
impl WsdlService {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The X12 agreement acknowledgement settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X12AcknowledgementSettings {
    #[doc = "The value indicating whether technical acknowledgement is needed."]
    #[serde(rename = "needTechnicalAcknowledgement")]
    pub need_technical_acknowledgement: bool,
    #[doc = "The value indicating whether to batch the technical acknowledgements."]
    #[serde(rename = "batchTechnicalAcknowledgements")]
    pub batch_technical_acknowledgements: bool,
    #[doc = "The value indicating whether functional acknowledgement is needed."]
    #[serde(rename = "needFunctionalAcknowledgement")]
    pub need_functional_acknowledgement: bool,
    #[doc = "The functional acknowledgement version."]
    #[serde(rename = "functionalAcknowledgementVersion", default, skip_serializing_if = "Option::is_none")]
    pub functional_acknowledgement_version: Option<String>,
    #[doc = "The value indicating whether to batch functional acknowledgements."]
    #[serde(rename = "batchFunctionalAcknowledgements")]
    pub batch_functional_acknowledgements: bool,
    #[doc = "The value indicating whether implementation acknowledgement is needed."]
    #[serde(rename = "needImplementationAcknowledgement")]
    pub need_implementation_acknowledgement: bool,
    #[doc = "The implementation acknowledgement version."]
    #[serde(rename = "implementationAcknowledgementVersion", default, skip_serializing_if = "Option::is_none")]
    pub implementation_acknowledgement_version: Option<String>,
    #[doc = "The value indicating whether to batch implementation acknowledgements."]
    #[serde(rename = "batchImplementationAcknowledgements")]
    pub batch_implementation_acknowledgements: bool,
    #[doc = "The value indicating whether a loop is needed for valid messages."]
    #[serde(rename = "needLoopForValidMessages")]
    pub need_loop_for_valid_messages: bool,
    #[doc = "The value indicating whether to send synchronous acknowledgement."]
    #[serde(rename = "sendSynchronousAcknowledgement")]
    pub send_synchronous_acknowledgement: bool,
    #[doc = "The acknowledgement control number prefix."]
    #[serde(rename = "acknowledgementControlNumberPrefix", default, skip_serializing_if = "Option::is_none")]
    pub acknowledgement_control_number_prefix: Option<String>,
    #[doc = "The acknowledgement control number suffix."]
    #[serde(rename = "acknowledgementControlNumberSuffix", default, skip_serializing_if = "Option::is_none")]
    pub acknowledgement_control_number_suffix: Option<String>,
    #[doc = "The acknowledgement control number lower bound."]
    #[serde(rename = "acknowledgementControlNumberLowerBound")]
    pub acknowledgement_control_number_lower_bound: i32,
    #[doc = "The acknowledgement control number upper bound."]
    #[serde(rename = "acknowledgementControlNumberUpperBound")]
    pub acknowledgement_control_number_upper_bound: i32,
    #[doc = "The value indicating whether to rollover acknowledgement control number."]
    #[serde(rename = "rolloverAcknowledgementControlNumber")]
    pub rollover_acknowledgement_control_number: bool,
}
impl X12AcknowledgementSettings {
    pub fn new(
        need_technical_acknowledgement: bool,
        batch_technical_acknowledgements: bool,
        need_functional_acknowledgement: bool,
        batch_functional_acknowledgements: bool,
        need_implementation_acknowledgement: bool,
        batch_implementation_acknowledgements: bool,
        need_loop_for_valid_messages: bool,
        send_synchronous_acknowledgement: bool,
        acknowledgement_control_number_lower_bound: i32,
        acknowledgement_control_number_upper_bound: i32,
        rollover_acknowledgement_control_number: bool,
    ) -> Self {
        Self {
            need_technical_acknowledgement,
            batch_technical_acknowledgements,
            need_functional_acknowledgement,
            functional_acknowledgement_version: None,
            batch_functional_acknowledgements,
            need_implementation_acknowledgement,
            implementation_acknowledgement_version: None,
            batch_implementation_acknowledgements,
            need_loop_for_valid_messages,
            send_synchronous_acknowledgement,
            acknowledgement_control_number_prefix: None,
            acknowledgement_control_number_suffix: None,
            acknowledgement_control_number_lower_bound,
            acknowledgement_control_number_upper_bound,
            rollover_acknowledgement_control_number,
        }
    }
}
#[doc = "The X12 agreement content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X12AgreementContent {
    #[doc = "The X12 one-way agreement."]
    #[serde(rename = "receiveAgreement")]
    pub receive_agreement: X12OneWayAgreement,
    #[doc = "The X12 one-way agreement."]
    #[serde(rename = "sendAgreement")]
    pub send_agreement: X12OneWayAgreement,
}
impl X12AgreementContent {
    pub fn new(receive_agreement: X12OneWayAgreement, send_agreement: X12OneWayAgreement) -> Self {
        Self {
            receive_agreement,
            send_agreement,
        }
    }
}
#[doc = "The X12 character set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "X12CharacterSet")]
pub enum X12CharacterSet {
    NotSpecified,
    Basic,
    Extended,
    #[serde(rename = "UTF8")]
    Utf8,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for X12CharacterSet {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for X12CharacterSet {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for X12CharacterSet {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("X12CharacterSet", 0u32, "NotSpecified"),
            Self::Basic => serializer.serialize_unit_variant("X12CharacterSet", 1u32, "Basic"),
            Self::Extended => serializer.serialize_unit_variant("X12CharacterSet", 2u32, "Extended"),
            Self::Utf8 => serializer.serialize_unit_variant("X12CharacterSet", 3u32, "UTF8"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The x12 date format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "X12DateFormat")]
pub enum X12DateFormat {
    NotSpecified,
    #[serde(rename = "CCYYMMDD")]
    Ccyymmdd,
    #[serde(rename = "YYMMDD")]
    Yymmdd,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for X12DateFormat {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for X12DateFormat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for X12DateFormat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("X12DateFormat", 0u32, "NotSpecified"),
            Self::Ccyymmdd => serializer.serialize_unit_variant("X12DateFormat", 1u32, "CCYYMMDD"),
            Self::Yymmdd => serializer.serialize_unit_variant("X12DateFormat", 2u32, "YYMMDD"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The X12 delimiter override settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X12DelimiterOverrides {
    #[doc = "The protocol version."]
    #[serde(rename = "protocolVersion", default, skip_serializing_if = "Option::is_none")]
    pub protocol_version: Option<String>,
    #[doc = "The message id."]
    #[serde(rename = "messageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[doc = "The data element separator."]
    #[serde(rename = "dataElementSeparator")]
    pub data_element_separator: i32,
    #[doc = "The component separator."]
    #[serde(rename = "componentSeparator")]
    pub component_separator: i32,
    #[doc = "The segment terminator."]
    #[serde(rename = "segmentTerminator")]
    pub segment_terminator: i32,
    #[doc = "The segment terminator suffix."]
    #[serde(rename = "segmentTerminatorSuffix")]
    pub segment_terminator_suffix: SegmentTerminatorSuffix,
    #[doc = "The replacement character."]
    #[serde(rename = "replaceCharacter")]
    pub replace_character: i32,
    #[doc = "The value indicating whether to replace separators in payload."]
    #[serde(rename = "replaceSeparatorsInPayload")]
    pub replace_separators_in_payload: bool,
    #[doc = "The target namespace on which this delimiter settings has to be applied."]
    #[serde(rename = "targetNamespace", default, skip_serializing_if = "Option::is_none")]
    pub target_namespace: Option<String>,
}
impl X12DelimiterOverrides {
    pub fn new(
        data_element_separator: i32,
        component_separator: i32,
        segment_terminator: i32,
        segment_terminator_suffix: SegmentTerminatorSuffix,
        replace_character: i32,
        replace_separators_in_payload: bool,
    ) -> Self {
        Self {
            protocol_version: None,
            message_id: None,
            data_element_separator,
            component_separator,
            segment_terminator,
            segment_terminator_suffix,
            replace_character,
            replace_separators_in_payload,
            target_namespace: None,
        }
    }
}
#[doc = "The X12 envelope override settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X12EnvelopeOverride {
    #[doc = "The target namespace on which this envelope settings has to be applied."]
    #[serde(rename = "targetNamespace")]
    pub target_namespace: String,
    #[doc = "The protocol version on which this envelope settings has to be applied."]
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    #[doc = "The message id on which this envelope settings has to be applied."]
    #[serde(rename = "messageId")]
    pub message_id: String,
    #[doc = "The responsible agency code."]
    #[serde(rename = "responsibleAgencyCode")]
    pub responsible_agency_code: String,
    #[doc = "The header version."]
    #[serde(rename = "headerVersion")]
    pub header_version: String,
    #[doc = "The sender application id."]
    #[serde(rename = "senderApplicationId")]
    pub sender_application_id: String,
    #[doc = "The receiver application id."]
    #[serde(rename = "receiverApplicationId")]
    pub receiver_application_id: String,
    #[doc = "The functional identifier code."]
    #[serde(rename = "functionalIdentifierCode", default, skip_serializing_if = "Option::is_none")]
    pub functional_identifier_code: Option<String>,
    #[doc = "The x12 date format."]
    #[serde(rename = "dateFormat")]
    pub date_format: X12DateFormat,
    #[doc = "The x12 time format."]
    #[serde(rename = "timeFormat")]
    pub time_format: X12TimeFormat,
}
impl X12EnvelopeOverride {
    pub fn new(
        target_namespace: String,
        protocol_version: String,
        message_id: String,
        responsible_agency_code: String,
        header_version: String,
        sender_application_id: String,
        receiver_application_id: String,
        date_format: X12DateFormat,
        time_format: X12TimeFormat,
    ) -> Self {
        Self {
            target_namespace,
            protocol_version,
            message_id,
            responsible_agency_code,
            header_version,
            sender_application_id,
            receiver_application_id,
            functional_identifier_code: None,
            date_format,
            time_format,
        }
    }
}
#[doc = "The X12 agreement envelope settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X12EnvelopeSettings {
    #[doc = "The controls standards id."]
    #[serde(rename = "controlStandardsId")]
    pub control_standards_id: i32,
    #[doc = "The value indicating whether to use control standards id as repetition character."]
    #[serde(rename = "useControlStandardsIdAsRepetitionCharacter")]
    pub use_control_standards_id_as_repetition_character: bool,
    #[doc = "The sender application id."]
    #[serde(rename = "senderApplicationId")]
    pub sender_application_id: String,
    #[doc = "The receiver application id."]
    #[serde(rename = "receiverApplicationId")]
    pub receiver_application_id: String,
    #[doc = "The control version number."]
    #[serde(rename = "controlVersionNumber")]
    pub control_version_number: String,
    #[doc = "The interchange  control number lower bound."]
    #[serde(rename = "interchangeControlNumberLowerBound")]
    pub interchange_control_number_lower_bound: i32,
    #[doc = "The interchange  control number upper bound."]
    #[serde(rename = "interchangeControlNumberUpperBound")]
    pub interchange_control_number_upper_bound: i32,
    #[doc = "The value indicating whether to rollover interchange control number."]
    #[serde(rename = "rolloverInterchangeControlNumber")]
    pub rollover_interchange_control_number: bool,
    #[doc = "The value indicating whether to enable default group headers."]
    #[serde(rename = "enableDefaultGroupHeaders")]
    pub enable_default_group_headers: bool,
    #[doc = "The functional group id."]
    #[serde(rename = "functionalGroupId", default, skip_serializing_if = "Option::is_none")]
    pub functional_group_id: Option<String>,
    #[doc = "The group control number lower bound."]
    #[serde(rename = "groupControlNumberLowerBound")]
    pub group_control_number_lower_bound: i32,
    #[doc = "The group control number upper bound."]
    #[serde(rename = "groupControlNumberUpperBound")]
    pub group_control_number_upper_bound: i32,
    #[doc = "The value indicating whether to rollover group control number."]
    #[serde(rename = "rolloverGroupControlNumber")]
    pub rollover_group_control_number: bool,
    #[doc = "The group header agency code."]
    #[serde(rename = "groupHeaderAgencyCode")]
    pub group_header_agency_code: String,
    #[doc = "The group header version."]
    #[serde(rename = "groupHeaderVersion")]
    pub group_header_version: String,
    #[doc = "The transaction set control number lower bound."]
    #[serde(rename = "transactionSetControlNumberLowerBound")]
    pub transaction_set_control_number_lower_bound: i32,
    #[doc = "The transaction set control number upper bound."]
    #[serde(rename = "transactionSetControlNumberUpperBound")]
    pub transaction_set_control_number_upper_bound: i32,
    #[doc = "The value indicating whether to rollover transaction set control number."]
    #[serde(rename = "rolloverTransactionSetControlNumber")]
    pub rollover_transaction_set_control_number: bool,
    #[doc = "The transaction set control number prefix."]
    #[serde(rename = "transactionSetControlNumberPrefix", default, skip_serializing_if = "Option::is_none")]
    pub transaction_set_control_number_prefix: Option<String>,
    #[doc = "The transaction set control number suffix."]
    #[serde(rename = "transactionSetControlNumberSuffix", default, skip_serializing_if = "Option::is_none")]
    pub transaction_set_control_number_suffix: Option<String>,
    #[doc = "The value indicating whether to overwrite existing transaction set control number."]
    #[serde(rename = "overwriteExistingTransactionSetControlNumber")]
    pub overwrite_existing_transaction_set_control_number: bool,
    #[doc = "The x12 date format."]
    #[serde(rename = "groupHeaderDateFormat")]
    pub group_header_date_format: X12DateFormat,
    #[doc = "The x12 time format."]
    #[serde(rename = "groupHeaderTimeFormat")]
    pub group_header_time_format: X12TimeFormat,
    #[doc = "The usage indicator."]
    #[serde(rename = "usageIndicator")]
    pub usage_indicator: UsageIndicator,
}
impl X12EnvelopeSettings {
    pub fn new(
        control_standards_id: i32,
        use_control_standards_id_as_repetition_character: bool,
        sender_application_id: String,
        receiver_application_id: String,
        control_version_number: String,
        interchange_control_number_lower_bound: i32,
        interchange_control_number_upper_bound: i32,
        rollover_interchange_control_number: bool,
        enable_default_group_headers: bool,
        group_control_number_lower_bound: i32,
        group_control_number_upper_bound: i32,
        rollover_group_control_number: bool,
        group_header_agency_code: String,
        group_header_version: String,
        transaction_set_control_number_lower_bound: i32,
        transaction_set_control_number_upper_bound: i32,
        rollover_transaction_set_control_number: bool,
        overwrite_existing_transaction_set_control_number: bool,
        group_header_date_format: X12DateFormat,
        group_header_time_format: X12TimeFormat,
        usage_indicator: UsageIndicator,
    ) -> Self {
        Self {
            control_standards_id,
            use_control_standards_id_as_repetition_character,
            sender_application_id,
            receiver_application_id,
            control_version_number,
            interchange_control_number_lower_bound,
            interchange_control_number_upper_bound,
            rollover_interchange_control_number,
            enable_default_group_headers,
            functional_group_id: None,
            group_control_number_lower_bound,
            group_control_number_upper_bound,
            rollover_group_control_number,
            group_header_agency_code,
            group_header_version,
            transaction_set_control_number_lower_bound,
            transaction_set_control_number_upper_bound,
            rollover_transaction_set_control_number,
            transaction_set_control_number_prefix: None,
            transaction_set_control_number_suffix: None,
            overwrite_existing_transaction_set_control_number,
            group_header_date_format,
            group_header_time_format,
            usage_indicator,
        }
    }
}
#[doc = "The X12 agreement framing settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X12FramingSettings {
    #[doc = "The data element separator."]
    #[serde(rename = "dataElementSeparator")]
    pub data_element_separator: i32,
    #[doc = "The component separator."]
    #[serde(rename = "componentSeparator")]
    pub component_separator: i32,
    #[doc = "The value indicating whether to replace separators in payload."]
    #[serde(rename = "replaceSeparatorsInPayload")]
    pub replace_separators_in_payload: bool,
    #[doc = "The replacement character."]
    #[serde(rename = "replaceCharacter")]
    pub replace_character: i32,
    #[doc = "The segment terminator."]
    #[serde(rename = "segmentTerminator")]
    pub segment_terminator: i32,
    #[doc = "The X12 character set."]
    #[serde(rename = "characterSet")]
    pub character_set: X12CharacterSet,
    #[doc = "The segment terminator suffix."]
    #[serde(rename = "segmentTerminatorSuffix")]
    pub segment_terminator_suffix: SegmentTerminatorSuffix,
}
impl X12FramingSettings {
    pub fn new(
        data_element_separator: i32,
        component_separator: i32,
        replace_separators_in_payload: bool,
        replace_character: i32,
        segment_terminator: i32,
        character_set: X12CharacterSet,
        segment_terminator_suffix: SegmentTerminatorSuffix,
    ) -> Self {
        Self {
            data_element_separator,
            component_separator,
            replace_separators_in_payload,
            replace_character,
            segment_terminator,
            character_set,
            segment_terminator_suffix,
        }
    }
}
#[doc = "The X12 message filter for odata query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X12MessageFilter {
    #[doc = "The message filter type."]
    #[serde(rename = "messageFilterType")]
    pub message_filter_type: MessageFilterType,
}
impl X12MessageFilter {
    pub fn new(message_filter_type: MessageFilterType) -> Self {
        Self { message_filter_type }
    }
}
#[doc = "The X12 message identifier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X12MessageIdentifier {
    #[doc = "The message id."]
    #[serde(rename = "messageId")]
    pub message_id: String,
}
impl X12MessageIdentifier {
    pub fn new(message_id: String) -> Self {
        Self { message_id }
    }
}
#[doc = "The X12 one-way agreement."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X12OneWayAgreement {
    #[doc = "The integration account partner's business identity."]
    #[serde(rename = "senderBusinessIdentity")]
    pub sender_business_identity: BusinessIdentity,
    #[doc = "The integration account partner's business identity."]
    #[serde(rename = "receiverBusinessIdentity")]
    pub receiver_business_identity: BusinessIdentity,
    #[doc = "The X12 agreement protocol settings."]
    #[serde(rename = "protocolSettings")]
    pub protocol_settings: X12ProtocolSettings,
}
impl X12OneWayAgreement {
    pub fn new(
        sender_business_identity: BusinessIdentity,
        receiver_business_identity: BusinessIdentity,
        protocol_settings: X12ProtocolSettings,
    ) -> Self {
        Self {
            sender_business_identity,
            receiver_business_identity,
            protocol_settings,
        }
    }
}
#[doc = "The X12 processing settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X12ProcessingSettings {
    #[doc = "The value indicating whether to mask security information."]
    #[serde(rename = "maskSecurityInfo")]
    pub mask_security_info: bool,
    #[doc = "The value indicating whether to convert numerical type to implied decimal."]
    #[serde(rename = "convertImpliedDecimal")]
    pub convert_implied_decimal: bool,
    #[doc = "The value indicating whether to preserve interchange."]
    #[serde(rename = "preserveInterchange")]
    pub preserve_interchange: bool,
    #[doc = "The value indicating whether to suspend interchange on error."]
    #[serde(rename = "suspendInterchangeOnError")]
    pub suspend_interchange_on_error: bool,
    #[doc = "The value indicating whether to create empty xml tags for trailing separators."]
    #[serde(rename = "createEmptyXmlTagsForTrailingSeparators")]
    pub create_empty_xml_tags_for_trailing_separators: bool,
    #[doc = "The value indicating whether to use dot as decimal separator."]
    #[serde(rename = "useDotAsDecimalSeparator")]
    pub use_dot_as_decimal_separator: bool,
}
impl X12ProcessingSettings {
    pub fn new(
        mask_security_info: bool,
        convert_implied_decimal: bool,
        preserve_interchange: bool,
        suspend_interchange_on_error: bool,
        create_empty_xml_tags_for_trailing_separators: bool,
        use_dot_as_decimal_separator: bool,
    ) -> Self {
        Self {
            mask_security_info,
            convert_implied_decimal,
            preserve_interchange,
            suspend_interchange_on_error,
            create_empty_xml_tags_for_trailing_separators,
            use_dot_as_decimal_separator,
        }
    }
}
#[doc = "The X12 agreement protocol settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X12ProtocolSettings {
    #[doc = "The X12 agreement validation settings."]
    #[serde(rename = "validationSettings")]
    pub validation_settings: X12ValidationSettings,
    #[doc = "The X12 agreement framing settings."]
    #[serde(rename = "framingSettings")]
    pub framing_settings: X12FramingSettings,
    #[doc = "The X12 agreement envelope settings."]
    #[serde(rename = "envelopeSettings")]
    pub envelope_settings: X12EnvelopeSettings,
    #[doc = "The X12 agreement acknowledgement settings."]
    #[serde(rename = "acknowledgementSettings")]
    pub acknowledgement_settings: X12AcknowledgementSettings,
    #[doc = "The X12 message filter for odata query."]
    #[serde(rename = "messageFilter")]
    pub message_filter: X12MessageFilter,
    #[doc = "The X12 agreement security settings."]
    #[serde(rename = "securitySettings")]
    pub security_settings: X12SecuritySettings,
    #[doc = "The X12 processing settings."]
    #[serde(rename = "processingSettings")]
    pub processing_settings: X12ProcessingSettings,
    #[doc = "The X12 envelope override settings."]
    #[serde(rename = "envelopeOverrides", default, skip_serializing_if = "Vec::is_empty")]
    pub envelope_overrides: Vec<X12EnvelopeOverride>,
    #[doc = "The X12 validation override settings."]
    #[serde(rename = "validationOverrides", default, skip_serializing_if = "Vec::is_empty")]
    pub validation_overrides: Vec<X12ValidationOverride>,
    #[doc = "The X12 message filter list."]
    #[serde(rename = "messageFilterList", default, skip_serializing_if = "Vec::is_empty")]
    pub message_filter_list: Vec<X12MessageIdentifier>,
    #[doc = "The X12 schema references."]
    #[serde(rename = "schemaReferences")]
    pub schema_references: Vec<X12SchemaReference>,
    #[doc = "The X12 delimiter override settings."]
    #[serde(rename = "x12DelimiterOverrides", default, skip_serializing_if = "Vec::is_empty")]
    pub x12_delimiter_overrides: Vec<X12DelimiterOverrides>,
}
impl X12ProtocolSettings {
    pub fn new(
        validation_settings: X12ValidationSettings,
        framing_settings: X12FramingSettings,
        envelope_settings: X12EnvelopeSettings,
        acknowledgement_settings: X12AcknowledgementSettings,
        message_filter: X12MessageFilter,
        security_settings: X12SecuritySettings,
        processing_settings: X12ProcessingSettings,
        schema_references: Vec<X12SchemaReference>,
    ) -> Self {
        Self {
            validation_settings,
            framing_settings,
            envelope_settings,
            acknowledgement_settings,
            message_filter,
            security_settings,
            processing_settings,
            envelope_overrides: Vec::new(),
            validation_overrides: Vec::new(),
            message_filter_list: Vec::new(),
            schema_references,
            x12_delimiter_overrides: Vec::new(),
        }
    }
}
#[doc = "The X12 schema reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X12SchemaReference {
    #[doc = "The message id."]
    #[serde(rename = "messageId")]
    pub message_id: String,
    #[doc = "The sender application id."]
    #[serde(rename = "senderApplicationId", default, skip_serializing_if = "Option::is_none")]
    pub sender_application_id: Option<String>,
    #[doc = "The schema version."]
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[doc = "The schema name."]
    #[serde(rename = "schemaName")]
    pub schema_name: String,
}
impl X12SchemaReference {
    pub fn new(message_id: String, schema_version: String, schema_name: String) -> Self {
        Self {
            message_id,
            sender_application_id: None,
            schema_version,
            schema_name,
        }
    }
}
#[doc = "The X12 agreement security settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X12SecuritySettings {
    #[doc = "The authorization qualifier."]
    #[serde(rename = "authorizationQualifier")]
    pub authorization_qualifier: String,
    #[doc = "The authorization value."]
    #[serde(rename = "authorizationValue", default, skip_serializing_if = "Option::is_none")]
    pub authorization_value: Option<String>,
    #[doc = "The security qualifier."]
    #[serde(rename = "securityQualifier")]
    pub security_qualifier: String,
    #[doc = "The password value."]
    #[serde(rename = "passwordValue", default, skip_serializing_if = "Option::is_none")]
    pub password_value: Option<String>,
}
impl X12SecuritySettings {
    pub fn new(authorization_qualifier: String, security_qualifier: String) -> Self {
        Self {
            authorization_qualifier,
            authorization_value: None,
            security_qualifier,
            password_value: None,
        }
    }
}
#[doc = "The x12 time format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "X12TimeFormat")]
pub enum X12TimeFormat {
    NotSpecified,
    #[serde(rename = "HHMM")]
    Hhmm,
    #[serde(rename = "HHMMSS")]
    Hhmmss,
    #[serde(rename = "HHMMSSdd")]
    HhmmsSdd,
    #[serde(rename = "HHMMSSd")]
    HhmmsSd,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for X12TimeFormat {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for X12TimeFormat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for X12TimeFormat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("X12TimeFormat", 0u32, "NotSpecified"),
            Self::Hhmm => serializer.serialize_unit_variant("X12TimeFormat", 1u32, "HHMM"),
            Self::Hhmmss => serializer.serialize_unit_variant("X12TimeFormat", 2u32, "HHMMSS"),
            Self::HhmmsSdd => serializer.serialize_unit_variant("X12TimeFormat", 3u32, "HHMMSSdd"),
            Self::HhmmsSd => serializer.serialize_unit_variant("X12TimeFormat", 4u32, "HHMMSSd"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The X12 validation override settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X12ValidationOverride {
    #[doc = "The message id on which the validation settings has to be applied."]
    #[serde(rename = "messageId")]
    pub message_id: String,
    #[doc = "The value indicating whether to validate EDI types."]
    #[serde(rename = "validateEDITypes")]
    pub validate_edi_types: bool,
    #[doc = "The value indicating whether to validate XSD types."]
    #[serde(rename = "validateXSDTypes")]
    pub validate_xsd_types: bool,
    #[doc = "The value indicating whether to allow leading and trailing spaces and zeroes."]
    #[serde(rename = "allowLeadingAndTrailingSpacesAndZeroes")]
    pub allow_leading_and_trailing_spaces_and_zeroes: bool,
    #[doc = "The value indicating whether to validate character Set."]
    #[serde(rename = "validateCharacterSet")]
    pub validate_character_set: bool,
    #[doc = "The value indicating whether to trim leading and trailing spaces and zeroes."]
    #[serde(rename = "trimLeadingAndTrailingSpacesAndZeroes")]
    pub trim_leading_and_trailing_spaces_and_zeroes: bool,
    #[doc = "The trailing separator policy."]
    #[serde(rename = "trailingSeparatorPolicy")]
    pub trailing_separator_policy: TrailingSeparatorPolicy,
}
impl X12ValidationOverride {
    pub fn new(
        message_id: String,
        validate_edi_types: bool,
        validate_xsd_types: bool,
        allow_leading_and_trailing_spaces_and_zeroes: bool,
        validate_character_set: bool,
        trim_leading_and_trailing_spaces_and_zeroes: bool,
        trailing_separator_policy: TrailingSeparatorPolicy,
    ) -> Self {
        Self {
            message_id,
            validate_edi_types,
            validate_xsd_types,
            allow_leading_and_trailing_spaces_and_zeroes,
            validate_character_set,
            trim_leading_and_trailing_spaces_and_zeroes,
            trailing_separator_policy,
        }
    }
}
#[doc = "The X12 agreement validation settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct X12ValidationSettings {
    #[doc = "The value indicating whether to validate character set in the message."]
    #[serde(rename = "validateCharacterSet")]
    pub validate_character_set: bool,
    #[doc = "The value indicating whether to check for duplicate interchange control number."]
    #[serde(rename = "checkDuplicateInterchangeControlNumber")]
    pub check_duplicate_interchange_control_number: bool,
    #[doc = "The validity period of interchange control number."]
    #[serde(rename = "interchangeControlNumberValidityDays")]
    pub interchange_control_number_validity_days: i32,
    #[doc = "The value indicating whether to check for duplicate group control number."]
    #[serde(rename = "checkDuplicateGroupControlNumber")]
    pub check_duplicate_group_control_number: bool,
    #[doc = "The value indicating whether to check for duplicate transaction set control number."]
    #[serde(rename = "checkDuplicateTransactionSetControlNumber")]
    pub check_duplicate_transaction_set_control_number: bool,
    #[doc = "The value indicating whether to Whether to validate EDI types."]
    #[serde(rename = "validateEDITypes")]
    pub validate_edi_types: bool,
    #[doc = "The value indicating whether to Whether to validate XSD types."]
    #[serde(rename = "validateXSDTypes")]
    pub validate_xsd_types: bool,
    #[doc = "The value indicating whether to allow leading and trailing spaces and zeroes."]
    #[serde(rename = "allowLeadingAndTrailingSpacesAndZeroes")]
    pub allow_leading_and_trailing_spaces_and_zeroes: bool,
    #[doc = "The value indicating whether to trim leading and trailing spaces and zeroes."]
    #[serde(rename = "trimLeadingAndTrailingSpacesAndZeroes")]
    pub trim_leading_and_trailing_spaces_and_zeroes: bool,
    #[doc = "The trailing separator policy."]
    #[serde(rename = "trailingSeparatorPolicy")]
    pub trailing_separator_policy: TrailingSeparatorPolicy,
}
impl X12ValidationSettings {
    pub fn new(
        validate_character_set: bool,
        check_duplicate_interchange_control_number: bool,
        interchange_control_number_validity_days: i32,
        check_duplicate_group_control_number: bool,
        check_duplicate_transaction_set_control_number: bool,
        validate_edi_types: bool,
        validate_xsd_types: bool,
        allow_leading_and_trailing_spaces_and_zeroes: bool,
        trim_leading_and_trailing_spaces_and_zeroes: bool,
        trailing_separator_policy: TrailingSeparatorPolicy,
    ) -> Self {
        Self {
            validate_character_set,
            check_duplicate_interchange_control_number,
            interchange_control_number_validity_days,
            check_duplicate_group_control_number,
            check_duplicate_transaction_set_control_number,
            validate_edi_types,
            validate_xsd_types,
            allow_leading_and_trailing_spaces_and_zeroes,
            trim_leading_and_trailing_spaces_and_zeroes,
            trailing_separator_policy,
        }
    }
}
#[doc = "User Assigned identity properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "Principal Id of user assigned identity"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Client Id of user assigned identity"]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
