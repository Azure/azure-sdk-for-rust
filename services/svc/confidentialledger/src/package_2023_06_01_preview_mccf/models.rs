#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "A compact summary of the service's state up to a certain point in time, updated and signed by members to indicate their participation in and approval of the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcksStateDigest {
    #[doc = "Hex-encoding of SHA-256 hash of the root of the service's merkle tree. This should be signed by a new member and submitted as an ACK to mark that member as Active."]
    pub digest: String,
}
impl AcksStateDigest {
    pub fn new(digest: String) -> Self {
        Self { digest }
    }
}
#[doc = "The error object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCoreFoundationsError {
    #[doc = "One of a server-defined set of error codes."]
    pub code: String,
    #[doc = "A human-readable representation of the error."]
    pub message: String,
    #[doc = "The target of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "An array of details about specific errors that led to this reported error."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<AzureCoreFoundationsError>,
    #[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<AzureCoreFoundationsInnerError>,
}
impl AzureCoreFoundationsError {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
            innererror: None,
        }
    }
}
#[doc = "A response containing error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCoreFoundationsErrorResponse {
    #[doc = "The error object."]
    pub error: AzureCoreFoundationsError,
}
impl azure_core::Continuable for AzureCoreFoundationsErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AzureCoreFoundationsErrorResponse {
    pub fn new(error: AzureCoreFoundationsError) -> Self {
        Self { error }
    }
}
#[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureCoreFoundationsInnerError {
    #[doc = "One of a server-defined set of error codes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<Box<AzureCoreFoundationsInnerError>>,
}
impl AzureCoreFoundationsInnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged collection of Member items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedMember {
    #[doc = "The Member items on this page"]
    pub value: Vec<ServiceStateMember>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedMember {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedMember {
    pub fn new(value: Vec<ServiceStateMember>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of Node items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedNode {
    #[doc = "The Node items on this page"]
    pub value: Vec<ServiceStateNode>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedNode {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedNode {
    pub fn new(value: Vec<ServiceStateNode>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of Proposal items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedProposal {
    #[doc = "The Proposal items on this page"]
    pub value: Vec<ProposalsProposal>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedProposal {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedProposal {
    pub fn new(value: Vec<ProposalsProposal>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "A single step in a proposed change to the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProposalsAction {
    #[doc = "The name of the action to perform. This should match an action defined in the service's constitution, so that it can be invoked by the `apply` function of the constitution if the proposal is accepted."]
    pub name: String,
    #[doc = "Arguments to modify the behavior of this action. The schema is determined by the action implementation, and should be validated by a `validate` call in the constitution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<serde_json::Value>,
}
impl ProposalsAction {
    pub fn new(name: String) -> Self {
        Self { name, args: None }
    }
}
#[doc = "The source script of an executable vote from a member, regarding a proposed change to the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProposalsBallot {
    #[doc = "The script to execute, returning the voter's support of this proposal."]
    pub script: String,
}
impl ProposalsBallot {
    pub fn new(script: String) -> Self {
        Self { script }
    }
}
#[doc = "Description of why governance execution failed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProposalsFailureInfo {
    #[doc = "Error message describing reason for failure."]
    pub reason: String,
    #[doc = "Call stack showing where failure occurred, if available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace: Option<String>,
}
impl ProposalsFailureInfo {
    pub fn new(reason: String) -> Self {
        Self { reason, trace: None }
    }
}
#[doc = "Each key is a memberId, and the corresponding value is the result of their ballot."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProposalsMemberVotes {}
impl ProposalsMemberVotes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of a proposed change to the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProposalsProposal {
    #[doc = "Hex encoding of SHA-256 of proposed actions and merkle root of store at proposal creation. Unlike other IDs, this is generated on the service and will not be known in advance."]
    #[serde(rename = "proposalId")]
    pub proposal_id: ProposalId,
    #[doc = "Hex encoding of SHA-256 of a member certificate's fingerprint."]
    #[serde(rename = "proposerId")]
    pub proposer_id: MemberId,
    #[doc = "Possible states for a proposal."]
    #[serde(rename = "proposalState")]
    pub proposal_state: ProposalsProposalState,
    #[doc = "A non-negative JSON-safe integer (ie max is 2^53 - 1)"]
    #[serde(rename = "ballotCount")]
    pub ballot_count: Safeuint,
    #[doc = "Each key is a memberId, and the corresponding value is the result of their ballot."]
    #[serde(rename = "finalVotes", default, skip_serializing_if = "Option::is_none")]
    pub final_votes: Option<ProposalsMemberVotes>,
    #[doc = "Each key is a memberId, and the corresponding value explains why execution of their ballot failed."]
    #[serde(rename = "voteFailures", default, skip_serializing_if = "Option::is_none")]
    pub vote_failures: Option<ProposalsVoteFailures>,
    #[doc = "Description of why governance execution failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure: Option<ProposalsFailureInfo>,
}
impl ProposalsProposal {
    pub fn new(proposal_id: ProposalId, proposer_id: MemberId, proposal_state: ProposalsProposalState, ballot_count: Safeuint) -> Self {
        Self {
            proposal_id,
            proposer_id,
            proposal_state,
            ballot_count,
            final_votes: None,
            vote_failures: None,
            failure: None,
        }
    }
}
#[doc = "The actions contained in a proposal, describing the proposal's changes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProposalsProposalActions {
    #[doc = "A list of actions to apply. Each action is considered, in order, for both proposal and execution of the proposal. All actions are validated and, if the proposal is accepted, applied atomically."]
    pub actions: Vec<ProposalsAction>,
}
impl ProposalsProposalActions {
    pub fn new(actions: Vec<ProposalsAction>) -> Self {
        Self { actions }
    }
}
#[doc = "Possible states for a proposal."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProposalsProposalState")]
pub enum ProposalsProposalState {
    Open,
    Accepted,
    Withdrawn,
    Rejected,
    Failed,
    Dropped,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProposalsProposalState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProposalsProposalState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProposalsProposalState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Open => serializer.serialize_unit_variant("ProposalsProposalState", 0u32, "Open"),
            Self::Accepted => serializer.serialize_unit_variant("ProposalsProposalState", 1u32, "Accepted"),
            Self::Withdrawn => serializer.serialize_unit_variant("ProposalsProposalState", 2u32, "Withdrawn"),
            Self::Rejected => serializer.serialize_unit_variant("ProposalsProposalState", 3u32, "Rejected"),
            Self::Failed => serializer.serialize_unit_variant("ProposalsProposalState", 4u32, "Failed"),
            Self::Dropped => serializer.serialize_unit_variant("ProposalsProposalState", 5u32, "Dropped"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Each key is a memberId, and the corresponding value explains why execution of their ballot failed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProposalsVoteFailures {}
impl ProposalsVoteFailures {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A share of a recovery key, granted for a specific recovery member, and encrypted with that member's share-encryption key. This is safe to share in the ledger or amongst untrusted callers, as only the intended member will be able to decrypt and access the secret content."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryEncryptedRecoveryShare {
    #[doc = "Base-64 encoding of a member's encrypted share."]
    #[serde(rename = "encryptedShare")]
    pub encrypted_share: String,
}
impl RecoveryEncryptedRecoveryShare {
    pub fn new(encrypted_share: String) -> Self {
        Self { encrypted_share }
    }
}
#[doc = "Recovery-specific details for a given member."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoveryMember {}
impl RecoveryMember {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response to a submitted recovery share."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryRecoveryResponse {
    #[doc = "Human-readable description of current recovery progress."]
    pub message: String,
    #[doc = "A non-negative JSON-safe integer (ie max is 2^53 - 1)"]
    #[serde(rename = "submittedCount")]
    pub submitted_count: Safeuint,
    #[doc = "A non-negative JSON-safe integer (ie max is 2^53 - 1)"]
    #[serde(rename = "recoveryThreshold")]
    pub recovery_threshold: Safeuint,
}
impl RecoveryRecoveryResponse {
    pub fn new(message: String, submitted_count: Safeuint, recovery_threshold: Safeuint) -> Self {
        Self {
            message,
            submitted_count,
            recovery_threshold,
        }
    }
}
#[doc = "Container for a member's raw recovery share. This should be carefully guarded, and only submitted to a trusted service over a secure channel, for the purposes of recovery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryRecoveryShare {
    #[doc = "Base-64 encoding of a member's raw recovery share (unencrypted)."]
    pub share: String,
}
impl RecoveryRecoveryShare {
    pub fn new(share: String) -> Self {
        Self { share }
    }
}
#[doc = "Constitution used to make governance decisions on the service. All governance changes are presented as proposals which must be validated, approved, and applied by the code in this constitution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceStateConstitutionResponse {
    #[doc = "Javascript source code of constitution."]
    pub constitution: String,
}
impl ServiceStateConstitutionResponse {
    pub fn new(constitution: String) -> Self {
        Self { constitution }
    }
}
#[doc = "Describes the forwarding behavior of a specific endpoint. Write requests cannot be executed on a backup, so will generally be forwarded by any backup node which receives them to the current primary. Future requests on the same session may then be forwarded to maintain session consistency."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServiceStateForwardingRequired")]
pub enum ServiceStateForwardingRequired {
    Sometimes,
    Always,
    Never,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServiceStateForwardingRequired {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServiceStateForwardingRequired {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServiceStateForwardingRequired {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Sometimes => serializer.serialize_unit_variant("ServiceStateForwardingRequired", 0u32, "Sometimes"),
            Self::Always => serializer.serialize_unit_variant("ServiceStateForwardingRequired", 1u32, "Always"),
            Self::Never => serializer.serialize_unit_variant("ServiceStateForwardingRequired", 2u32, "Never"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Collection of all policies which determine currently acceptable nodes, across multiple platforms."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceStateJoinPolicies {
    #[doc = "Describes what a joining node must present, in order to join the service."]
    pub sgx: ServiceStateJoinPolicy,
    #[doc = "Join policy fields specific to nodes running on AMD SEV-SNP hardware."]
    pub snp: ServiceStateSnpJoinPolicy,
}
impl ServiceStateJoinPolicies {
    pub fn new(sgx: ServiceStateJoinPolicy, snp: ServiceStateSnpJoinPolicy) -> Self {
        Self { sgx, snp }
    }
}
#[doc = "Describes what a joining node must present, in order to join the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceStateJoinPolicy {
    #[doc = "Code measurements of acceptable enclaves."]
    pub measurements: Vec<String>,
}
impl ServiceStateJoinPolicy {
    pub fn new(measurements: Vec<String>) -> Self {
        Self { measurements }
    }
}
#[doc = "Describes the currently installed JavaScript application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceStateJsApp {
    #[doc = "The collection of endpoints exposed by the application. Keyed by path."]
    pub endpoints: serde_json::Value,
}
impl ServiceStateJsApp {
    pub fn new(endpoints: serde_json::Value) -> Self {
        Self { endpoints }
    }
}
#[doc = "Describes an endpoint implemented by a Javascript handler."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceStateJsEndpointInfo {
    #[doc = "The name of the module where the endpoint function is located."]
    #[serde(rename = "jsModule")]
    pub js_module: String,
    #[doc = "The name of the exported function which implements this endpoint."]
    #[serde(rename = "jsFunction")]
    pub js_function: String,
    #[doc = "Describes the forwarding behavior of a specific endpoint. Write requests cannot be executed on a backup, so will generally be forwarded by any backup node which receives them to the current primary. Future requests on the same session may then be forwarded to maintain session consistency."]
    #[serde(rename = "forwardingRequired")]
    pub forwarding_required: ServiceStateForwardingRequired,
    #[doc = "The authentication policies which restrict access to this endpoint"]
    #[serde(rename = "authnPolicies")]
    pub authn_policies: Vec<String>,
    #[doc = "Describes the execution requirements of a specific endpoint."]
    pub mode: ServiceStateJsExecMode,
    #[doc = "An OpenAPI Operation object (https://swagger.io/specification/#operation-object) describing this operation. This is merged into the auto-generated OpenAPI to describe the current application's API."]
    #[serde(rename = "openApi")]
    pub open_api: serde_json::Value,
}
impl ServiceStateJsEndpointInfo {
    pub fn new(
        js_module: String,
        js_function: String,
        forwarding_required: ServiceStateForwardingRequired,
        authn_policies: Vec<String>,
        mode: ServiceStateJsExecMode,
        open_api: serde_json::Value,
    ) -> Self {
        Self {
            js_module,
            js_function,
            forwarding_required,
            authn_policies,
            mode,
            open_api,
        }
    }
}
#[doc = "Describes the execution requirements of a specific endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServiceStateJsExecMode")]
pub enum ServiceStateJsExecMode {
    ReadWrite,
    ReadOnly,
    Historical,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServiceStateJsExecMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServiceStateJsExecMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServiceStateJsExecMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ReadWrite => serializer.serialize_unit_variant("ServiceStateJsExecMode", 0u32, "ReadWrite"),
            Self::ReadOnly => serializer.serialize_unit_variant("ServiceStateJsExecMode", 1u32, "ReadOnly"),
            Self::Historical => serializer.serialize_unit_variant("ServiceStateJsExecMode", 2u32, "Historical"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The collection of operations available on each path. Keyed by HTTP method."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceStateJsOperations {}
impl ServiceStateJsOperations {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes what Javascript Web Tokens (JWTs) are accepted by the service, and how they will be validated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceStateJwkInfo {
    #[doc = "Collection of JWT issuers. Keyed by issuer ID."]
    pub issuers: serde_json::Value,
    #[doc = "Collection of CAs used to authenticate connections with issuers. Keyed by governance-controlled bundle names."]
    #[serde(rename = "caCertBundles")]
    pub ca_cert_bundles: serde_json::Value,
}
impl ServiceStateJwkInfo {
    pub fn new(issuers: serde_json::Value, ca_cert_bundles: serde_json::Value) -> Self {
        Self { issuers, ca_cert_bundles }
    }
}
#[doc = "Description of a JWT issuer or identity provider that the current service will trust tokens from."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceStateJwtIssuer {
    #[doc = "Possible restrictions on what keys will be accepted from a JWT issuer."]
    #[serde(rename = "keyFilter")]
    pub key_filter: ServiceStateJwtIssuerKeyFilter,
    #[doc = "Collection of claims which must be present in SGX attestation to permit updates from this issuer."]
    #[serde(rename = "keyPolicy", default, skip_serializing_if = "Option::is_none")]
    pub key_policy: Option<serde_json::Value>,
    #[doc = "Whether this issuer's keys are periodically refreshed with a fetch from the current primary. If false, these will only be updated by governance."]
    #[serde(rename = "autoRefresh")]
    pub auto_refresh: bool,
    #[doc = "Name of bundle used to authenticate issuer when auto-refreshing."]
    #[serde(rename = "caCertBundleName", default, skip_serializing_if = "Option::is_none")]
    pub ca_cert_bundle_name: Option<String>,
}
impl ServiceStateJwtIssuer {
    pub fn new(key_filter: ServiceStateJwtIssuerKeyFilter, auto_refresh: bool) -> Self {
        Self {
            key_filter,
            key_policy: None,
            auto_refresh,
            ca_cert_bundle_name: None,
        }
    }
}
#[doc = "Possible restrictions on what keys will be accepted from a JWT issuer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServiceStateJwtIssuerKeyFilter")]
pub enum ServiceStateJwtIssuerKeyFilter {
    All,
    Sgx,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServiceStateJwtIssuerKeyFilter {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServiceStateJwtIssuerKeyFilter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServiceStateJwtIssuerKeyFilter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::All => serializer.serialize_unit_variant("ServiceStateJwtIssuerKeyFilter", 0u32, "All"),
            Self::Sgx => serializer.serialize_unit_variant("ServiceStateJwtIssuerKeyFilter", 1u32, "Sgx"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Information on individual members within a consortium."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceStateMember {
    #[doc = "Hex encoding of SHA-256 of a member certificate's fingerprint."]
    #[serde(rename = "memberId")]
    pub member_id: MemberId,
    #[doc = "Possible states for a CCF governing member."]
    pub status: ServiceStateMemberStatus,
    #[doc = "Arbitrary service-defined metadata about this member. May be used by constitution or application code, but will not affect any core framework decisions."]
    #[serde(rename = "memberData", default, skip_serializing_if = "Option::is_none")]
    pub member_data: Option<serde_json::Value>,
    #[doc = "PEM encoding of a cryptographic identifier. Contains a base64-encoded payload wrapped in content type identifiers."]
    pub certificate: ServiceStatePem,
}
impl ServiceStateMember {
    pub fn new(member_id: MemberId, status: ServiceStateMemberStatus, certificate: ServiceStatePem) -> Self {
        Self {
            member_id,
            status,
            member_data: None,
            certificate,
        }
    }
}
#[doc = "Possible states for a CCF governing member."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServiceStateMemberStatus")]
pub enum ServiceStateMemberStatus {
    Accepted,
    Active,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServiceStateMemberStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServiceStateMemberStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServiceStateMemberStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Accepted => serializer.serialize_unit_variant("ServiceStateMemberStatus", 0u32, "Accepted"),
            Self::Active => serializer.serialize_unit_variant("ServiceStateMemberStatus", 1u32, "Active"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Details of how to contact a CCF node. Each node may listen on multiple interfaces, for different kinds of traffic."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceStateNetworkInterface {
    #[doc = "The network address where this node believes it is publicly accessible, in the format <host>[:<port>]."]
    #[serde(rename = "publishedAddress")]
    pub published_address: String,
    #[doc = "The application layer protocol which the node expects on this interface. Currently supports \"http1\" and \"http2\", more protocols may be added in future."]
    pub protocol: String,
}
impl ServiceStateNetworkInterface {
    pub fn new(published_address: String, protocol: String) -> Self {
        Self {
            published_address,
            protocol,
        }
    }
}
#[doc = "Information on individual nodes within a service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceStateNode {
    #[doc = "Hex encoding of SHA-256 of a node's public key."]
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    #[doc = "Lifecycle state of a CCF node. Nodes will generally start as Pending, then transition to Trusted, then to Retired. They are only full participants in the service while they are Trusted."]
    pub status: ServiceStateNodeStatus,
    #[doc = "Arbitrary service-defined metadata about this node. May be used by constitution or application code, but will not affect any core framework decisions."]
    #[serde(rename = "nodeData", default, skip_serializing_if = "Option::is_none")]
    pub node_data: Option<serde_json::Value>,
    #[doc = "PEM encoding of a cryptographic identifier. Contains a base64-encoded payload wrapped in content type identifiers."]
    pub certificate: ServiceStatePem,
    #[doc = "This is false during node's normal operation. It transitions to true once the node has been retired, and that retirement has been committed by the service. At this point it is safe to terminate a node. Terminating a node any earlier may affect liveness of the service."]
    #[serde(rename = "retiredCommitted")]
    pub retired_committed: bool,
    #[doc = "Common type for attestation information, describing the cryptographically-endorsed claim of what code is executing, and what platform it is executing on. Derived types contain platform-specific details."]
    #[serde(rename = "quoteInfo")]
    pub quote_info: ServiceStateQuoteInfoUnion,
    #[doc = "A collection of interfaces by which this node may be contacted. Some may be limited to private networks, and others may be DNS names or internet-public network addresses. The keys are arbitrary strings determined by the node operator."]
    #[serde(rename = "rpcInterfaces")]
    pub rpc_interfaces: serde_json::Value,
}
impl ServiceStateNode {
    pub fn new(
        node_id: NodeId,
        status: ServiceStateNodeStatus,
        certificate: ServiceStatePem,
        retired_committed: bool,
        quote_info: ServiceStateQuoteInfoUnion,
        rpc_interfaces: serde_json::Value,
    ) -> Self {
        Self {
            node_id,
            status,
            node_data: None,
            certificate,
            retired_committed,
            quote_info,
            rpc_interfaces,
        }
    }
}
#[doc = "Lifecycle state of a CCF node. Nodes will generally start as Pending, then transition to Trusted, then to Retired. They are only full participants in the service while they are Trusted."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServiceStateNodeStatus")]
pub enum ServiceStateNodeStatus {
    Pending,
    Trusted,
    Retired,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServiceStateNodeStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServiceStateNodeStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServiceStateNodeStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pending => serializer.serialize_unit_variant("ServiceStateNodeStatus", 0u32, "Pending"),
            Self::Trusted => serializer.serialize_unit_variant("ServiceStateNodeStatus", 1u32, "Trusted"),
            Self::Retired => serializer.serialize_unit_variant("ServiceStateNodeStatus", 2u32, "Retired"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Discriminator property for QuoteInfo."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "format")]
pub enum ServiceStateQuoteInfoUnion {
    #[serde(rename = "OE_SGX_v1")]
    OeSgxV1(ServiceStateSgxQuoteInfo),
    #[serde(rename = "AMD_SEV_SNP_v1")]
    AmdSevSnpV1(ServiceStateSnpQuoteInfo),
}
#[doc = "General information about the current service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceStateServiceInfo {
    #[doc = "State machine values for current service lifetime. New services start in Opening, and transition to Open via a governance proposal. They will only accept user transactions on the `/app` endpoints once they are Open. Recovery services have additional states where they must wait for members to submit sufficient recovery shares to access the previous ledger secrets, and while they are decrypting and replaying the previous ledger contents."]
    pub status: ServiceStateServiceStatus,
    #[doc = "PEM encoding of a cryptographic identifier. Contains a base64-encoded payload wrapped in content type identifiers."]
    pub certificate: ServiceStatePem,
    #[doc = "A non-negative JSON-safe integer (ie max is 2^53 - 1)"]
    #[serde(rename = "recoveryCount")]
    pub recovery_count: Safeuint,
    #[doc = "Uniquely identifies an atomic transaction within a CCF service. Composed of a term number and sequence number. Sequence numbers increase monotonically, apart from during elections where the service may reuse an existing sequence number. Each election will result in a new, higher term number being used for the conflicting and future sequence numbers."]
    #[serde(rename = "creationTransactionId")]
    pub creation_transaction_id: TransactionId,
    #[doc = "PEM encoding of a cryptographic identifier. Contains a base64-encoded payload wrapped in content type identifiers."]
    #[serde(rename = "previousServiceCreationTransactionId", default, skip_serializing_if = "Option::is_none")]
    pub previous_service_creation_transaction_id: Option<ServiceStatePem>,
    #[doc = "Arbitrary service-defined metadata about this service. May be used by constitution or application code, but will not affect any core framework decisions."]
    #[serde(rename = "serviceData", default, skip_serializing_if = "Option::is_none")]
    pub service_data: Option<serde_json::Value>,
    #[doc = "Lists governance-controlled configuration parameters of this service, which will be used by core framework code."]
    pub configuration: service_state_service_info::Configuration,
}
impl ServiceStateServiceInfo {
    pub fn new(
        status: ServiceStateServiceStatus,
        certificate: ServiceStatePem,
        recovery_count: Safeuint,
        creation_transaction_id: TransactionId,
        configuration: service_state_service_info::Configuration,
    ) -> Self {
        Self {
            status,
            certificate,
            recovery_count,
            creation_transaction_id,
            previous_service_creation_transaction_id: None,
            service_data: None,
            configuration,
        }
    }
}
pub mod service_state_service_info {
    use super::*;
    #[doc = "Lists governance-controlled configuration parameters of this service, which will be used by core framework code."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Configuration {
        #[doc = "A non-negative JSON-safe integer (ie max is 2^53 - 1)"]
        #[serde(rename = "maximumNodeCertificateValidityDays")]
        pub maximum_node_certificate_validity_days: Safeuint,
        #[doc = "A non-negative JSON-safe integer (ie max is 2^53 - 1)"]
        #[serde(rename = "recentCoseProposalsWindowSize")]
        pub recent_cose_proposals_window_size: Safeuint,
    }
    impl Configuration {
        pub fn new(maximum_node_certificate_validity_days: Safeuint, recent_cose_proposals_window_size: Safeuint) -> Self {
            Self {
                maximum_node_certificate_validity_days,
                recent_cose_proposals_window_size,
            }
        }
    }
}
#[doc = "State machine values for current service lifetime. New services start in Opening, and transition to Open via a governance proposal. They will only accept user transactions on the `/app` endpoints once they are Open. Recovery services have additional states where they must wait for members to submit sufficient recovery shares to access the previous ledger secrets, and while they are decrypting and replaying the previous ledger contents."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServiceStateServiceStatus")]
pub enum ServiceStateServiceStatus {
    Opening,
    Open,
    WaitingForRecoveryShares,
    Recovering,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServiceStateServiceStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServiceStateServiceStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServiceStateServiceStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Opening => serializer.serialize_unit_variant("ServiceStateServiceStatus", 0u32, "Opening"),
            Self::Open => serializer.serialize_unit_variant("ServiceStateServiceStatus", 1u32, "Open"),
            Self::WaitingForRecoveryShares => {
                serializer.serialize_unit_variant("ServiceStateServiceStatus", 2u32, "WaitingForRecoveryShares")
            }
            Self::Recovering => serializer.serialize_unit_variant("ServiceStateServiceStatus", 3u32, "Recovering"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Attestation information for Intel SGX enclaves."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceStateSgxQuoteInfo {
    #[doc = "Base-64 encoded SGX quote."]
    pub quote: String,
    #[doc = "Base-64 encoded SGX endorsements."]
    pub endorsements: String,
}
impl ServiceStateSgxQuoteInfo {
    pub fn new(quote: String, endorsements: String) -> Self {
        Self { quote, endorsements }
    }
}
#[doc = "Join policy fields specific to nodes running on AMD SEV-SNP hardware."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceStateSnpJoinPolicy {
    #[doc = "Code measurements of acceptable enclaves."]
    pub measurements: Vec<String>,
    #[doc = "Collection of acceptable host data values."]
    #[serde(rename = "hostData")]
    pub host_data: serde_json::Value,
    #[doc = "Collection of acceptable UVM endorsements."]
    #[serde(rename = "uvmEndorsements")]
    pub uvm_endorsements: serde_json::Value,
}
impl ServiceStateSnpJoinPolicy {
    pub fn new(measurements: Vec<String>, host_data: serde_json::Value, uvm_endorsements: serde_json::Value) -> Self {
        Self {
            measurements,
            host_data,
            uvm_endorsements,
        }
    }
}
#[doc = "Attestation information for AMD SEV-SNP containers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceStateSnpQuoteInfo {
    #[doc = "Base-64 encoded SNP UVM endorsements."]
    #[serde(rename = "uvmEndorsements")]
    pub uvm_endorsements: String,
    #[doc = "Base-64 encoded SNP TCB endorsements."]
    #[serde(rename = "endorsedTcb")]
    pub endorsed_tcb: String,
}
impl ServiceStateSnpQuoteInfo {
    pub fn new(uvm_endorsements: String, endorsed_tcb: String) -> Self {
        Self {
            uvm_endorsements,
            endorsed_tcb,
        }
    }
}
pub type ServiceStateCaCertBundle = String;
pub type ServiceStatePem = String;
#[doc = "Description of latest committed transaction."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionsCommittedTransaction {
    #[doc = "Uniquely identifies an atomic transaction within a CCF service. Composed of a term number and sequence number. Sequence numbers increase monotonically, apart from during elections where the service may reuse an existing sequence number. Each election will result in a new, higher term number being used for the conflicting and future sequence numbers."]
    #[serde(rename = "transactionId")]
    pub transaction_id: TransactionId,
    #[doc = "Possible states for a CCF transaction. See docs for details: https://microsoft.github.io/CCF/main/use_apps/verify_tx.html#checking-for-commit"]
    pub status: TransactionsTransactionStatus,
}
impl TransactionsCommittedTransaction {
    pub fn new(transaction_id: TransactionId, status: TransactionsTransactionStatus) -> Self {
        Self { transaction_id, status }
    }
}
#[doc = "Description of a named transaction's current state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionsNamedTransaction {
    #[doc = "Possible states for a CCF transaction. See docs for details: https://microsoft.github.io/CCF/main/use_apps/verify_tx.html#checking-for-commit"]
    pub status: TransactionsTransactionStatus,
}
impl TransactionsNamedTransaction {
    pub fn new(status: TransactionsTransactionStatus) -> Self {
        Self { status }
    }
}
#[doc = "Common transaction information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionsTransaction {
    #[doc = "Possible states for a CCF transaction. See docs for details: https://microsoft.github.io/CCF/main/use_apps/verify_tx.html#checking-for-commit"]
    pub status: TransactionsTransactionStatus,
}
impl TransactionsTransaction {
    pub fn new(status: TransactionsTransactionStatus) -> Self {
        Self { status }
    }
}
#[doc = "Possible states for a CCF transaction. See docs for details: https://microsoft.github.io/CCF/main/use_apps/verify_tx.html#checking-for-commit"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TransactionsTransactionStatus")]
pub enum TransactionsTransactionStatus {
    Unknown,
    Pending,
    Committed,
    Invalid,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TransactionsTransactionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TransactionsTransactionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TransactionsTransactionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("TransactionsTransactionStatus", 0u32, "Unknown"),
            Self::Pending => serializer.serialize_unit_variant("TransactionsTransactionStatus", 1u32, "Pending"),
            Self::Committed => serializer.serialize_unit_variant("TransactionsTransactionStatus", 2u32, "Committed"),
            Self::Invalid => serializer.serialize_unit_variant("TransactionsTransactionStatus", 3u32, "Invalid"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Versions")]
pub enum Versions {
    #[serde(rename = "2023-06-01-preview")]
    N2023_06_01_preview,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Versions {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Versions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Versions {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::N2023_06_01_preview => serializer.serialize_unit_variant("Versions", 0u32, "2023-06-01-preview"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type MemberId = String;
pub type NodeId = String;
pub type ProposalId = String;
pub type Safeuint = i64;
pub type TransactionId = String;
pub type UserId = String;
