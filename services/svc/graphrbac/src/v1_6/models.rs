#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Active Directory group information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdGroup {
    #[serde(flatten)]
    pub directory_object: DirectoryObject,
    #[doc = "The display name of the group."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Whether the group is mail-enabled. Must be false. This is because only pure security groups can be created using the Graph API."]
    #[serde(rename = "mailEnabled", default, skip_serializing_if = "Option::is_none")]
    pub mail_enabled: Option<bool>,
    #[doc = "The mail alias for the group. "]
    #[serde(rename = "mailNickname", default, skip_serializing_if = "Option::is_none")]
    pub mail_nickname: Option<String>,
    #[doc = "Whether the group is security-enable."]
    #[serde(rename = "securityEnabled", default, skip_serializing_if = "Option::is_none")]
    pub security_enabled: Option<bool>,
    #[doc = "The primary email address of the group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
}
impl AdGroup {
    pub fn new(directory_object: DirectoryObject) -> Self {
        Self {
            directory_object,
            display_name: None,
            mail_enabled: None,
            mail_nickname: None,
            security_enabled: None,
            mail: None,
        }
    }
}
#[doc = "Request parameters for adding a owner to an application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddOwnerParameters {
    #[doc = "A owner object URL, such as \"https://graph.windows.net/0b1f9851-1bf0-433f-aec3-cb9272f093dc/directoryObjects/f260bbc4-c254-447b-94cf-293b5ec434dd\", where \"0b1f9851-1bf0-433f-aec3-cb9272f093dc\" is the tenantId and \"f260bbc4-c254-447b-94cf-293b5ec434dd\" is the objectId of the owner (user, application, servicePrincipal, group) to be added."]
    pub url: String,
}
impl AddOwnerParameters {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppRole {
    #[doc = "Unique role identifier inside the appRoles collection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Specifies whether this app role definition can be assigned to users and groups by setting to 'User', or to other applications (that are accessing this application in daemon service scenarios) by setting to 'Application', or to both. "]
    #[serde(rename = "allowedMemberTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_member_types: Vec<String>,
    #[doc = "Permission help text that appears in the admin app assignment and consent experiences."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Display name for the permission that appears in the admin consent and app assignment experiences."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "When creating or updating a role definition, this must be set to true (which is the default). To delete a role, this must first be set to false. At that point, in a subsequent call, this role may be removed."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Specifies the value of the roles claim that the application should expect in the authentication and access tokens."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl AppRole {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AppRoleAssignment information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppRoleAssignment {
    #[serde(flatten)]
    pub directory_object: DirectoryObject,
    #[doc = "The role id that was assigned to the principal. This role must be declared by the target resource application resourceId in its appRoles property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The display name of the principal that was granted the access."]
    #[serde(rename = "principalDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub principal_display_name: Option<String>,
    #[doc = "The unique identifier (objectId) for the principal being granted the access."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The type of principal. This can either be \"User\", \"Group\" or \"ServicePrincipal\"."]
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,
    #[doc = "The display name of the resource to which the assignment was made."]
    #[serde(rename = "resourceDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub resource_display_name: Option<String>,
    #[doc = "The unique identifier (objectId) for the target resource (service principal) for which the assignment was made."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl AppRoleAssignment {
    pub fn new(directory_object: DirectoryObject) -> Self {
        Self {
            directory_object,
            id: None,
            principal_display_name: None,
            principal_id: None,
            principal_type: None,
            resource_display_name: None,
            resource_id: None,
        }
    }
}
#[doc = "AppRoleAssignment list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppRoleAssignmentListResult {
    #[doc = "A collection of AppRoleAssignment."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AppRoleAssignment>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for AppRoleAssignmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl AppRoleAssignmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Active Directory application information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    #[serde(flatten)]
    pub directory_object: DirectoryObject,
    #[doc = "The application ID."]
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[doc = "A property on the application to indicate if the application accepts other IDPs or not or partially accepts."]
    #[serde(rename = "allowGuestsSignIn", default, skip_serializing_if = "Option::is_none")]
    pub allow_guests_sign_in: Option<bool>,
    #[doc = "Indicates that the application supports pass through users who have no presence in the resource tenant."]
    #[serde(rename = "allowPassthroughUsers", default, skip_serializing_if = "Option::is_none")]
    pub allow_passthrough_users: Option<bool>,
    #[doc = "The url for the application logo image stored in a CDN."]
    #[serde(rename = "appLogoUrl", default, skip_serializing_if = "Option::is_none")]
    pub app_logo_url: Option<String>,
    #[doc = "The collection of application roles that an application may declare. These roles can be assigned to users, groups or service principals."]
    #[serde(rename = "appRoles", default, skip_serializing_if = "Vec::is_empty")]
    pub app_roles: Vec<AppRole>,
    #[doc = "The application permissions."]
    #[serde(rename = "appPermissions", default, skip_serializing_if = "Vec::is_empty")]
    pub app_permissions: Vec<String>,
    #[doc = "Whether the application is available to other tenants."]
    #[serde(rename = "availableToOtherTenants", default, skip_serializing_if = "Option::is_none")]
    pub available_to_other_tenants: Option<bool>,
    #[doc = "The display name of the application."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "A URL provided by the author of the application to report errors when using the application."]
    #[serde(rename = "errorUrl", default, skip_serializing_if = "Option::is_none")]
    pub error_url: Option<String>,
    #[doc = "Configures the groups claim issued in a user or OAuth 2.0 access token that the app expects."]
    #[serde(rename = "groupMembershipClaims", default, skip_serializing_if = "Option::is_none")]
    pub group_membership_claims: Option<GroupMembershipClaims>,
    #[doc = "The home page of the application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[doc = "A collection of URIs for the application."]
    #[serde(rename = "identifierUris", default, skip_serializing_if = "Vec::is_empty")]
    pub identifier_uris: Vec<String>,
    #[doc = "Represents a group of URIs that provide terms of service, marketing, support and privacy policy information about an application. The default value for each string is null."]
    #[serde(rename = "informationalUrls", default, skip_serializing_if = "Option::is_none")]
    pub informational_urls: Option<InformationalUrl>,
    #[doc = "Specifies whether this application supports device authentication without a user. The default is false."]
    #[serde(rename = "isDeviceOnlyAuthSupported", default, skip_serializing_if = "Option::is_none")]
    pub is_device_only_auth_supported: Option<bool>,
    #[doc = "A collection of KeyCredential objects."]
    #[serde(rename = "keyCredentials", default, skip_serializing_if = "Vec::is_empty")]
    pub key_credentials: Vec<KeyCredential>,
    #[doc = "Client applications that are tied to this resource application. Consent to any of the known client applications will result in implicit consent to the resource application through a combined consent dialog (showing the OAuth permission scopes required by the client and the resource)."]
    #[serde(rename = "knownClientApplications", default, skip_serializing_if = "Vec::is_empty")]
    pub known_client_applications: Vec<String>,
    #[doc = "the url of the logout page"]
    #[serde(rename = "logoutUrl", default, skip_serializing_if = "Option::is_none")]
    pub logout_url: Option<String>,
    #[doc = "Whether to allow implicit grant flow for OAuth2"]
    #[serde(rename = "oauth2AllowImplicitFlow", default, skip_serializing_if = "Option::is_none")]
    pub oauth2_allow_implicit_flow: Option<bool>,
    #[doc = "Specifies whether during a token Request Azure AD will allow path matching of the redirect URI against the applications collection of replyURLs. The default is false."]
    #[serde(rename = "oauth2AllowUrlPathMatching", default, skip_serializing_if = "Option::is_none")]
    pub oauth2_allow_url_path_matching: Option<bool>,
    #[doc = "The collection of OAuth 2.0 permission scopes that the web API (resource) application exposes to client applications. These permission scopes may be granted to client applications during consent."]
    #[serde(rename = "oauth2Permissions", default, skip_serializing_if = "Vec::is_empty")]
    pub oauth2_permissions: Vec<OAuth2Permission>,
    #[doc = "Specifies whether, as part of OAuth 2.0 token requests, Azure AD will allow POST requests, as opposed to GET requests. The default is false, which specifies that only GET requests will be allowed."]
    #[serde(rename = "oauth2RequirePostResponse", default, skip_serializing_if = "Option::is_none")]
    pub oauth2_require_post_response: Option<bool>,
    #[doc = "A list of tenants allowed to access application."]
    #[serde(rename = "orgRestrictions", default, skip_serializing_if = "Vec::is_empty")]
    pub org_restrictions: Vec<String>,
    #[doc = "Specifying the claims to be included in the token."]
    #[serde(rename = "optionalClaims", default, skip_serializing_if = "Option::is_none")]
    pub optional_claims: Option<OptionalClaims>,
    #[doc = "A collection of PasswordCredential objects"]
    #[serde(rename = "passwordCredentials", default, skip_serializing_if = "Vec::is_empty")]
    pub password_credentials: Vec<PasswordCredential>,
    #[doc = "list of pre-authorized applications."]
    #[serde(rename = "preAuthorizedApplications", default, skip_serializing_if = "Vec::is_empty")]
    pub pre_authorized_applications: Vec<PreAuthorizedApplication>,
    #[doc = "Specifies whether this application is a public client (such as an installed application running on a mobile device). Default is false."]
    #[serde(rename = "publicClient", default, skip_serializing_if = "Option::is_none")]
    pub public_client: Option<bool>,
    #[doc = "Reliable domain which can be used to identify an application."]
    #[serde(rename = "publisherDomain", default, skip_serializing_if = "Option::is_none")]
    pub publisher_domain: Option<String>,
    #[doc = "A collection of reply URLs for the application."]
    #[serde(rename = "replyUrls", default, skip_serializing_if = "Vec::is_empty")]
    pub reply_urls: Vec<String>,
    #[doc = "Specifies resources that this application requires access to and the set of OAuth permission scopes and application roles that it needs under each of those resources. This pre-configuration of required resource access drives the consent experience."]
    #[serde(rename = "requiredResourceAccess", default, skip_serializing_if = "Vec::is_empty")]
    pub required_resource_access: Vec<RequiredResourceAccess>,
    #[doc = "The URL to the SAML metadata for the application."]
    #[serde(rename = "samlMetadataUrl", default, skip_serializing_if = "Option::is_none")]
    pub saml_metadata_url: Option<String>,
    #[doc = "Audience for signing in to the application (AzureADMyOrganization, AzureADAllOrganizations, AzureADAndMicrosoftAccounts)."]
    #[serde(rename = "signInAudience", default, skip_serializing_if = "Option::is_none")]
    pub sign_in_audience: Option<String>,
    #[doc = "The primary Web page."]
    #[serde(rename = "wwwHomepage", default, skip_serializing_if = "Option::is_none")]
    pub www_homepage: Option<String>,
}
impl Application {
    pub fn new(directory_object: DirectoryObject) -> Self {
        Self {
            directory_object,
            app_id: None,
            allow_guests_sign_in: None,
            allow_passthrough_users: None,
            app_logo_url: None,
            app_roles: Vec::new(),
            app_permissions: Vec::new(),
            available_to_other_tenants: None,
            display_name: None,
            error_url: None,
            group_membership_claims: None,
            homepage: None,
            identifier_uris: Vec::new(),
            informational_urls: None,
            is_device_only_auth_supported: None,
            key_credentials: Vec::new(),
            known_client_applications: Vec::new(),
            logout_url: None,
            oauth2_allow_implicit_flow: None,
            oauth2_allow_url_path_matching: None,
            oauth2_permissions: Vec::new(),
            oauth2_require_post_response: None,
            org_restrictions: Vec::new(),
            optional_claims: None,
            password_credentials: Vec::new(),
            pre_authorized_applications: Vec::new(),
            public_client: None,
            publisher_domain: None,
            reply_urls: Vec::new(),
            required_resource_access: Vec::new(),
            saml_metadata_url: None,
            sign_in_audience: None,
            www_homepage: None,
        }
    }
}
#[doc = "Active Directive Application common properties shared among GET, POST and PATCH"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationBase {
    #[doc = "A property on the application to indicate if the application accepts other IDPs or not or partially accepts."]
    #[serde(rename = "allowGuestsSignIn", default, skip_serializing_if = "Option::is_none")]
    pub allow_guests_sign_in: Option<bool>,
    #[doc = "Indicates that the application supports pass through users who have no presence in the resource tenant."]
    #[serde(rename = "allowPassthroughUsers", default, skip_serializing_if = "Option::is_none")]
    pub allow_passthrough_users: Option<bool>,
    #[doc = "The url for the application logo image stored in a CDN."]
    #[serde(rename = "appLogoUrl", default, skip_serializing_if = "Option::is_none")]
    pub app_logo_url: Option<String>,
    #[doc = "The collection of application roles that an application may declare. These roles can be assigned to users, groups or service principals."]
    #[serde(rename = "appRoles", default, skip_serializing_if = "Vec::is_empty")]
    pub app_roles: Vec<AppRole>,
    #[doc = "The application permissions."]
    #[serde(rename = "appPermissions", default, skip_serializing_if = "Vec::is_empty")]
    pub app_permissions: Vec<String>,
    #[doc = "Whether the application is available to other tenants."]
    #[serde(rename = "availableToOtherTenants", default, skip_serializing_if = "Option::is_none")]
    pub available_to_other_tenants: Option<bool>,
    #[doc = "A URL provided by the author of the application to report errors when using the application."]
    #[serde(rename = "errorUrl", default, skip_serializing_if = "Option::is_none")]
    pub error_url: Option<String>,
    #[doc = "Configures the groups claim issued in a user or OAuth 2.0 access token that the app expects."]
    #[serde(rename = "groupMembershipClaims", default, skip_serializing_if = "Option::is_none")]
    pub group_membership_claims: Option<GroupMembershipClaims>,
    #[doc = "The home page of the application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[doc = "Represents a group of URIs that provide terms of service, marketing, support and privacy policy information about an application. The default value for each string is null."]
    #[serde(rename = "informationalUrls", default, skip_serializing_if = "Option::is_none")]
    pub informational_urls: Option<InformationalUrl>,
    #[doc = "Specifies whether this application supports device authentication without a user. The default is false."]
    #[serde(rename = "isDeviceOnlyAuthSupported", default, skip_serializing_if = "Option::is_none")]
    pub is_device_only_auth_supported: Option<bool>,
    #[doc = "A collection of KeyCredential objects."]
    #[serde(rename = "keyCredentials", default, skip_serializing_if = "Vec::is_empty")]
    pub key_credentials: Vec<KeyCredential>,
    #[doc = "Client applications that are tied to this resource application. Consent to any of the known client applications will result in implicit consent to the resource application through a combined consent dialog (showing the OAuth permission scopes required by the client and the resource)."]
    #[serde(rename = "knownClientApplications", default, skip_serializing_if = "Vec::is_empty")]
    pub known_client_applications: Vec<String>,
    #[doc = "the url of the logout page"]
    #[serde(rename = "logoutUrl", default, skip_serializing_if = "Option::is_none")]
    pub logout_url: Option<String>,
    #[doc = "Whether to allow implicit grant flow for OAuth2"]
    #[serde(rename = "oauth2AllowImplicitFlow", default, skip_serializing_if = "Option::is_none")]
    pub oauth2_allow_implicit_flow: Option<bool>,
    #[doc = "Specifies whether during a token Request Azure AD will allow path matching of the redirect URI against the applications collection of replyURLs. The default is false."]
    #[serde(rename = "oauth2AllowUrlPathMatching", default, skip_serializing_if = "Option::is_none")]
    pub oauth2_allow_url_path_matching: Option<bool>,
    #[doc = "The collection of OAuth 2.0 permission scopes that the web API (resource) application exposes to client applications. These permission scopes may be granted to client applications during consent."]
    #[serde(rename = "oauth2Permissions", default, skip_serializing_if = "Vec::is_empty")]
    pub oauth2_permissions: Vec<OAuth2Permission>,
    #[doc = "Specifies whether, as part of OAuth 2.0 token requests, Azure AD will allow POST requests, as opposed to GET requests. The default is false, which specifies that only GET requests will be allowed."]
    #[serde(rename = "oauth2RequirePostResponse", default, skip_serializing_if = "Option::is_none")]
    pub oauth2_require_post_response: Option<bool>,
    #[doc = "A list of tenants allowed to access application."]
    #[serde(rename = "orgRestrictions", default, skip_serializing_if = "Vec::is_empty")]
    pub org_restrictions: Vec<String>,
    #[doc = "Specifying the claims to be included in the token."]
    #[serde(rename = "optionalClaims", default, skip_serializing_if = "Option::is_none")]
    pub optional_claims: Option<OptionalClaims>,
    #[doc = "A collection of PasswordCredential objects"]
    #[serde(rename = "passwordCredentials", default, skip_serializing_if = "Vec::is_empty")]
    pub password_credentials: Vec<PasswordCredential>,
    #[doc = "list of pre-authorized applications."]
    #[serde(rename = "preAuthorizedApplications", default, skip_serializing_if = "Vec::is_empty")]
    pub pre_authorized_applications: Vec<PreAuthorizedApplication>,
    #[doc = "Specifies whether this application is a public client (such as an installed application running on a mobile device). Default is false."]
    #[serde(rename = "publicClient", default, skip_serializing_if = "Option::is_none")]
    pub public_client: Option<bool>,
    #[doc = "Reliable domain which can be used to identify an application."]
    #[serde(rename = "publisherDomain", default, skip_serializing_if = "Option::is_none")]
    pub publisher_domain: Option<String>,
    #[doc = "A collection of reply URLs for the application."]
    #[serde(rename = "replyUrls", default, skip_serializing_if = "Vec::is_empty")]
    pub reply_urls: Vec<String>,
    #[doc = "Specifies resources that this application requires access to and the set of OAuth permission scopes and application roles that it needs under each of those resources. This pre-configuration of required resource access drives the consent experience."]
    #[serde(rename = "requiredResourceAccess", default, skip_serializing_if = "Vec::is_empty")]
    pub required_resource_access: Vec<RequiredResourceAccess>,
    #[doc = "The URL to the SAML metadata for the application."]
    #[serde(rename = "samlMetadataUrl", default, skip_serializing_if = "Option::is_none")]
    pub saml_metadata_url: Option<String>,
    #[doc = "Audience for signing in to the application (AzureADMyOrganization, AzureADAllOrganizations, AzureADAndMicrosoftAccounts)."]
    #[serde(rename = "signInAudience", default, skip_serializing_if = "Option::is_none")]
    pub sign_in_audience: Option<String>,
    #[doc = "The primary Web page."]
    #[serde(rename = "wwwHomepage", default, skip_serializing_if = "Option::is_none")]
    pub www_homepage: Option<String>,
}
impl ApplicationBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request parameters for creating a new application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationCreateParameters {
    #[serde(flatten)]
    pub application_base: ApplicationBase,
    #[doc = "The display name of the application."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "A collection of URIs for the application."]
    #[serde(rename = "identifierUris", default, skip_serializing_if = "Vec::is_empty")]
    pub identifier_uris: Vec<String>,
}
impl ApplicationCreateParameters {
    pub fn new(display_name: String) -> Self {
        Self {
            application_base: ApplicationBase::default(),
            display_name,
            identifier_uris: Vec::new(),
        }
    }
}
#[doc = "Application list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationListResult {
    #[doc = "A collection of applications."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Application>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for ApplicationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl ApplicationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request parameters for updating a new application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationUpdateParameters {
    #[serde(flatten)]
    pub application_base: ApplicationBase,
    #[doc = "The display name of the application."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "A collection of URIs for the application."]
    #[serde(rename = "identifierUris", default, skip_serializing_if = "Vec::is_empty")]
    pub identifier_uris: Vec<String>,
}
impl ApplicationUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request parameters for IsMemberOf API call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckGroupMembershipParameters {
    #[doc = "The object ID of the group to check."]
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[doc = "The object ID of the contact, group, user, or service principal to check for membership in the specified group."]
    #[serde(rename = "memberId")]
    pub member_id: String,
}
impl CheckGroupMembershipParameters {
    pub fn new(group_id: String, member_id: String) -> Self {
        Self { group_id, member_id }
    }
}
#[doc = "Server response for IsMemberOf API call"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckGroupMembershipResult {
    #[doc = "True if the specified user, group, contact, or service principal has either direct or transitive membership in the specified group; otherwise, false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}
impl CheckGroupMembershipResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an Azure Active Directory object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectoryObject {
    #[doc = "The object ID."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "The object type."]
    #[serde(rename = "objectType")]
    pub object_type: String,
    #[doc = "The time at which the directory object was deleted."]
    #[serde(rename = "deletionTimestamp", default, with = "azure_core::date::rfc3339::option")]
    pub deletion_timestamp: Option<time::OffsetDateTime>,
}
impl DirectoryObject {
    pub fn new(object_type: String) -> Self {
        Self {
            object_id: None,
            object_type,
            deletion_timestamp: None,
        }
    }
}
#[doc = "DirectoryObject list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DirectoryObjectListResult {
    #[doc = "A collection of DirectoryObject."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DirectoryObject>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for DirectoryObjectListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl DirectoryObjectListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Active Directory Domain information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Domain {
    #[doc = "the type of the authentication into the domain."]
    #[serde(rename = "authenticationType", default, skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[doc = "if this is the default domain in the tenant."]
    #[serde(rename = "isDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "if this domain's ownership is verified."]
    #[serde(rename = "isVerified", default, skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    #[doc = "the domain name."]
    pub name: String,
}
impl Domain {
    pub fn new(name: String) -> Self {
        Self {
            authentication_type: None,
            is_default: None,
            is_verified: None,
            name,
        }
    }
}
#[doc = "Server response for Get tenant domains API call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainListResult {
    #[doc = "the list of domains."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Domain>,
}
impl azure_core::Continuable for DomainListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DomainListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Active Directory error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorMessage {
    #[doc = "Error message value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ErrorMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request parameters for the GetObjectsByObjectIds API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetObjectsParameters {
    #[doc = "The requested object IDs."]
    #[serde(rename = "objectIds", default, skip_serializing_if = "Vec::is_empty")]
    pub object_ids: Vec<String>,
    #[doc = "The requested object types."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<String>,
    #[doc = "If true, also searches for object IDs in the partner tenant."]
    #[serde(rename = "includeDirectoryObjectReferences", default, skip_serializing_if = "Option::is_none")]
    pub include_directory_object_references: Option<bool>,
}
impl GetObjectsParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Active Directory error information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphError {
    #[doc = "Active Directory OData error information."]
    #[serde(rename = "odata.error", default, skip_serializing_if = "Option::is_none")]
    pub odata_error: Option<OdataError>,
}
impl azure_core::Continuable for GraphError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl GraphError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request parameters for adding a member to a group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupAddMemberParameters {
    #[doc = "A member object URL, such as \"https://graph.windows.net/0b1f9851-1bf0-433f-aec3-cb9272f093dc/directoryObjects/f260bbc4-c254-447b-94cf-293b5ec434dd\", where \"0b1f9851-1bf0-433f-aec3-cb9272f093dc\" is the tenantId and \"f260bbc4-c254-447b-94cf-293b5ec434dd\" is the objectId of the member (user, application, servicePrincipal, group) to be added."]
    pub url: String,
}
impl GroupAddMemberParameters {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}
#[doc = "Request parameters for creating a new group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupCreateParameters {
    #[doc = "Group display name"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Whether the group is mail-enabled. Must be false. This is because only pure security groups can be created using the Graph API."]
    #[serde(rename = "mailEnabled")]
    pub mail_enabled: group_create_parameters::MailEnabled,
    #[doc = "Mail nickname"]
    #[serde(rename = "mailNickname")]
    pub mail_nickname: String,
    #[doc = "Whether the group is a security group. Must be true. This is because only pure security groups can be created using the Graph API."]
    #[serde(rename = "securityEnabled")]
    pub security_enabled: group_create_parameters::SecurityEnabled,
}
impl GroupCreateParameters {
    pub fn new(
        display_name: String,
        mail_enabled: group_create_parameters::MailEnabled,
        mail_nickname: String,
        security_enabled: group_create_parameters::SecurityEnabled,
    ) -> Self {
        Self {
            display_name,
            mail_enabled,
            mail_nickname,
            security_enabled,
        }
    }
}
pub mod group_create_parameters {
    use super::*;
    #[doc = "Whether the group is mail-enabled. Must be false. This is because only pure security groups can be created using the Graph API."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MailEnabled {}
    #[doc = "Whether the group is a security group. Must be true. This is because only pure security groups can be created using the Graph API."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SecurityEnabled {}
}
#[doc = "Request parameters for GetMemberGroups API call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupGetMemberGroupsParameters {
    #[doc = "If true, only membership in security-enabled groups should be checked. Otherwise, membership in all groups should be checked."]
    #[serde(rename = "securityEnabledOnly")]
    pub security_enabled_only: bool,
}
impl GroupGetMemberGroupsParameters {
    pub fn new(security_enabled_only: bool) -> Self {
        Self { security_enabled_only }
    }
}
#[doc = "Server response for GetMemberGroups API call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupGetMemberGroupsResult {
    #[doc = "A collection of group IDs of which the group is a member."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
}
impl azure_core::Continuable for GroupGetMemberGroupsResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl GroupGetMemberGroupsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Server response for Get tenant groups API call"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupListResult {
    #[doc = "A collection of Active Directory groups."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AdGroup>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for GroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl GroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configures the groups claim issued in a user or OAuth 2.0 access token that the app expects."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "GroupMembershipClaims")]
pub enum GroupMembershipClaims {
    None,
    SecurityGroup,
    All,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for GroupMembershipClaims {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for GroupMembershipClaims {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for GroupMembershipClaims {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("GroupMembershipClaims", 0u32, "None"),
            Self::SecurityGroup => serializer.serialize_unit_variant("GroupMembershipClaims", 1u32, "SecurityGroup"),
            Self::All => serializer.serialize_unit_variant("GroupMembershipClaims", 2u32, "All"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents a group of URIs that provide terms of service, marketing, support and privacy policy information about an application. The default value for each string is null."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InformationalUrl {
    #[doc = "The terms of service URI"]
    #[serde(rename = "termsOfService", default, skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
    #[doc = "The marketing URI"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub marketing: Option<String>,
    #[doc = "The privacy policy URI"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<String>,
    #[doc = "The support URI"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support: Option<String>,
}
impl InformationalUrl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Active Directory Key Credential information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyCredential {
    #[doc = "Start date."]
    #[serde(rename = "startDate", default, with = "azure_core::date::rfc3339::option")]
    pub start_date: Option<time::OffsetDateTime>,
    #[doc = "End date."]
    #[serde(rename = "endDate", default, with = "azure_core::date::rfc3339::option")]
    pub end_date: Option<time::OffsetDateTime>,
    #[doc = "Key value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Key ID."]
    #[serde(rename = "keyId", default, skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[doc = "Usage. Acceptable values are 'Verify' and 'Sign'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
    #[doc = "Type. Acceptable values are 'AsymmetricX509Cert' and 'Symmetric'."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Custom Key Identifier"]
    #[serde(rename = "customKeyIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub custom_key_identifier: Option<String>,
}
impl KeyCredential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "KeyCredential list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyCredentialListResult {
    #[doc = "A collection of KeyCredentials."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<KeyCredential>,
}
impl azure_core::Continuable for KeyCredentialListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl KeyCredentialListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request parameters for a KeyCredentials update operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyCredentialsUpdateParameters {
    #[doc = "A collection of KeyCredentials."]
    pub value: Vec<KeyCredential>,
}
impl KeyCredentialsUpdateParameters {
    pub fn new(value: Vec<KeyCredential>) -> Self {
        Self { value }
    }
}
#[doc = "Represents an OAuth 2.0 delegated permission scope. The specified OAuth 2.0 delegated permission scopes may be requested by client applications (through the requiredResourceAccess collection on the Application object) when calling a resource application. The oauth2Permissions property of the ServicePrincipal entity and of the Application entity is a collection of OAuth2Permission."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OAuth2Permission {
    #[doc = "Permission help text that appears in the admin consent and app assignment experiences."]
    #[serde(rename = "adminConsentDescription", default, skip_serializing_if = "Option::is_none")]
    pub admin_consent_description: Option<String>,
    #[doc = "Display name for the permission that appears in the admin consent and app assignment experiences."]
    #[serde(rename = "adminConsentDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub admin_consent_display_name: Option<String>,
    #[doc = "Unique scope permission identifier inside the oauth2Permissions collection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "When creating or updating a permission, this property must be set to true (which is the default). To delete a permission, this property must first be set to false. At that point, in a subsequent call, the permission may be removed. "]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Specifies whether this scope permission can be consented to by an end user, or whether it is a tenant-wide permission that must be consented to by a Company Administrator. Possible values are \"User\" or \"Admin\"."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Permission help text that appears in the end user consent experience."]
    #[serde(rename = "userConsentDescription", default, skip_serializing_if = "Option::is_none")]
    pub user_consent_description: Option<String>,
    #[doc = "Display name for the permission that appears in the end user consent experience."]
    #[serde(rename = "userConsentDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub user_consent_display_name: Option<String>,
    #[doc = "The value of the scope claim that the resource application should expect in the OAuth 2.0 access token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl OAuth2Permission {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OAuth2PermissionGrant {
    #[doc = "Microsoft.DirectoryServices.OAuth2PermissionGrant"]
    #[serde(rename = "odata.type", default, skip_serializing_if = "Option::is_none")]
    pub odata_type: Option<String>,
    #[doc = "The id of the resource's service principal granted consent to impersonate the user when accessing the resource (represented by the resourceId property)."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "The id of the permission grant"]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Indicates if consent was provided by the administrator (on behalf of the organization) or by an individual."]
    #[serde(rename = "consentType", default, skip_serializing_if = "Option::is_none")]
    pub consent_type: Option<o_auth2_permission_grant::ConsentType>,
    #[doc = "When consent type is Principal, this property specifies the id of the user that granted consent and applies only for that user."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Object Id of the resource you want to grant"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Specifies the value of the scope claim that the resource application should expect in the OAuth 2.0 access token. For example, User.Read"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "Start time for TTL"]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "Expiry time for TTL"]
    #[serde(rename = "expiryTime", default, skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<String>,
}
impl OAuth2PermissionGrant {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod o_auth2_permission_grant {
    use super::*;
    #[doc = "Indicates if consent was provided by the administrator (on behalf of the organization) or by an individual."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConsentType")]
    pub enum ConsentType {
        AllPrincipals,
        Principal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConsentType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConsentType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConsentType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AllPrincipals => serializer.serialize_unit_variant("ConsentType", 0u32, "AllPrincipals"),
                Self::Principal => serializer.serialize_unit_variant("ConsentType", 1u32, "Principal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Server response for get oauth2 permissions grants"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OAuth2PermissionGrantListResult {
    #[doc = "the list of oauth2 permissions grants"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OAuth2PermissionGrant>,
    #[doc = "the URL to get the next set of results."]
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for OAuth2PermissionGrantListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl OAuth2PermissionGrantListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Active Directory OData error information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OdataError {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Active Directory error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<ErrorMessage>,
}
impl OdataError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifying the claims to be included in a token."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OptionalClaim {
    #[doc = "Claim name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Claim source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Is this a required claim."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub essential: Option<bool>,
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<serde_json::Value>,
}
impl OptionalClaim {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifying the claims to be included in the token."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OptionalClaims {
    #[doc = "Optional claims requested to be included in the id token."]
    #[serde(rename = "idToken", default, skip_serializing_if = "Vec::is_empty")]
    pub id_token: Vec<OptionalClaim>,
    #[doc = "Optional claims requested to be included in the access token."]
    #[serde(rename = "accessToken", default, skip_serializing_if = "Vec::is_empty")]
    pub access_token: Vec<OptionalClaim>,
    #[doc = "Optional claims requested to be included in the saml token."]
    #[serde(rename = "samlToken", default, skip_serializing_if = "Vec::is_empty")]
    pub saml_token: Vec<OptionalClaim>,
}
impl OptionalClaims {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Active Directory Password Credential information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PasswordCredential {
    #[doc = "Start date."]
    #[serde(rename = "startDate", default, with = "azure_core::date::rfc3339::option")]
    pub start_date: Option<time::OffsetDateTime>,
    #[doc = "End date."]
    #[serde(rename = "endDate", default, with = "azure_core::date::rfc3339::option")]
    pub end_date: Option<time::OffsetDateTime>,
    #[doc = "Key ID."]
    #[serde(rename = "keyId", default, skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[doc = "Key value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Custom Key Identifier"]
    #[serde(rename = "customKeyIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub custom_key_identifier: Option<String>,
}
impl PasswordCredential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "PasswordCredential list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PasswordCredentialListResult {
    #[doc = "A collection of PasswordCredentials."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PasswordCredential>,
}
impl azure_core::Continuable for PasswordCredentialListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl PasswordCredentialListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request parameters for a PasswordCredentials update operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PasswordCredentialsUpdateParameters {
    #[doc = "A collection of PasswordCredentials."]
    pub value: Vec<PasswordCredential>,
}
impl PasswordCredentialsUpdateParameters {
    pub fn new(value: Vec<PasswordCredential>) -> Self {
        Self { value }
    }
}
#[doc = "The password profile associated with a user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PasswordProfile {
    #[doc = "Password"]
    pub password: String,
    #[doc = "Whether to force a password change on next login."]
    #[serde(rename = "forceChangePasswordNextLogin", default, skip_serializing_if = "Option::is_none")]
    pub force_change_password_next_login: Option<bool>,
}
impl PasswordProfile {
    pub fn new(password: String) -> Self {
        Self {
            password,
            force_change_password_next_login: None,
        }
    }
}
#[doc = "Contains information about pre authorized client application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PreAuthorizedApplication {
    #[doc = "Represents the application id."]
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[doc = "Collection of required app permissions/entitlements from the resource application."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<PreAuthorizedApplicationPermission>,
    #[doc = "Collection of extensions from the resource application."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<PreAuthorizedApplicationExtension>,
}
impl PreAuthorizedApplication {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Representation of an app PreAuthorizedApplicationExtension required by a pre authorized client app."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PreAuthorizedApplicationExtension {
    #[doc = "The extension's conditions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<String>,
}
impl PreAuthorizedApplicationExtension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains information about the pre-authorized permissions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PreAuthorizedApplicationPermission {
    #[doc = "Indicates whether the permission set is DirectAccess or impersonation."]
    #[serde(rename = "directAccessGrant", default, skip_serializing_if = "Option::is_none")]
    pub direct_access_grant: Option<bool>,
    #[doc = "The list of permissions."]
    #[serde(rename = "accessGrants", default, skip_serializing_if = "Vec::is_empty")]
    pub access_grants: Vec<String>,
}
impl PreAuthorizedApplicationPermission {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the set of OAuth 2.0 permission scopes and app roles under the specified resource that an application requires access to. The specified OAuth 2.0 permission scopes may be requested by client applications (through the requiredResourceAccess collection) when calling a resource application. The requiredResourceAccess property of the Application entity is a collection of RequiredResourceAccess."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequiredResourceAccess {
    #[doc = "The list of OAuth2.0 permission scopes and app roles that the application requires from the specified resource."]
    #[serde(rename = "resourceAccess")]
    pub resource_access: Vec<ResourceAccess>,
    #[doc = "The unique identifier for the resource that the application requires access to. This should be equal to the appId declared on the target resource application."]
    #[serde(rename = "resourceAppId", default, skip_serializing_if = "Option::is_none")]
    pub resource_app_id: Option<String>,
}
impl RequiredResourceAccess {
    pub fn new(resource_access: Vec<ResourceAccess>) -> Self {
        Self {
            resource_access,
            resource_app_id: None,
        }
    }
}
#[doc = "Specifies an OAuth 2.0 permission scope or an app role that an application requires. The resourceAccess property of the RequiredResourceAccess type is a collection of ResourceAccess."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceAccess {
    #[doc = "The unique identifier for one of the OAuth2Permission or AppRole instances that the resource application exposes."]
    pub id: String,
    #[doc = "Specifies whether the id property references an OAuth2Permission or an AppRole. Possible values are \"scope\" or \"role\"."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ResourceAccess {
    pub fn new(id: String) -> Self {
        Self { id, type_: None }
    }
}
#[doc = "Active Directory service principal information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePrincipal {
    #[serde(flatten)]
    pub directory_object: DirectoryObject,
    #[doc = "whether or not the service principal account is enabled"]
    #[serde(rename = "accountEnabled", default, skip_serializing_if = "Option::is_none")]
    pub account_enabled: Option<bool>,
    #[doc = "alternative names"]
    #[serde(rename = "alternativeNames", default, skip_serializing_if = "Vec::is_empty")]
    pub alternative_names: Vec<String>,
    #[doc = "The display name exposed by the associated application."]
    #[serde(rename = "appDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub app_display_name: Option<String>,
    #[doc = "The application ID."]
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "appOwnerTenantId", default, skip_serializing_if = "Option::is_none")]
    pub app_owner_tenant_id: Option<String>,
    #[doc = "Specifies whether an AppRoleAssignment to a user or group is required before Azure AD will issue a user or access token to the application."]
    #[serde(rename = "appRoleAssignmentRequired", default, skip_serializing_if = "Option::is_none")]
    pub app_role_assignment_required: Option<bool>,
    #[doc = "The collection of application roles that an application may declare. These roles can be assigned to users, groups or service principals."]
    #[serde(rename = "appRoles", default, skip_serializing_if = "Vec::is_empty")]
    pub app_roles: Vec<AppRole>,
    #[doc = "The display name of the service principal."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "A URL provided by the author of the associated application to report errors when using the application."]
    #[serde(rename = "errorUrl", default, skip_serializing_if = "Option::is_none")]
    pub error_url: Option<String>,
    #[doc = "The URL to the homepage of the associated application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[doc = "The collection of key credentials associated with the service principal."]
    #[serde(rename = "keyCredentials", default, skip_serializing_if = "Vec::is_empty")]
    pub key_credentials: Vec<KeyCredential>,
    #[doc = "A URL provided by the author of the associated application to logout"]
    #[serde(rename = "logoutUrl", default, skip_serializing_if = "Option::is_none")]
    pub logout_url: Option<String>,
    #[doc = "The OAuth 2.0 permissions exposed by the associated application."]
    #[serde(rename = "oauth2Permissions", default, skip_serializing_if = "Vec::is_empty")]
    pub oauth2_permissions: Vec<OAuth2Permission>,
    #[doc = "The collection of password credentials associated with the service principal."]
    #[serde(rename = "passwordCredentials", default, skip_serializing_if = "Vec::is_empty")]
    pub password_credentials: Vec<PasswordCredential>,
    #[doc = "The thumbprint of preferred certificate to sign the token"]
    #[serde(rename = "preferredTokenSigningKeyThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub preferred_token_signing_key_thumbprint: Option<String>,
    #[doc = "The publisher's name of the associated application"]
    #[serde(rename = "publisherName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_name: Option<String>,
    #[doc = "The URLs that user tokens are sent to for sign in with the associated application.  The redirect URIs that the oAuth 2.0 authorization code and access tokens are sent to for the associated application."]
    #[serde(rename = "replyUrls", default, skip_serializing_if = "Vec::is_empty")]
    pub reply_urls: Vec<String>,
    #[doc = "The URL to the SAML metadata of the associated application"]
    #[serde(rename = "samlMetadataUrl", default, skip_serializing_if = "Option::is_none")]
    pub saml_metadata_url: Option<String>,
    #[doc = "A collection of service principal names."]
    #[serde(rename = "servicePrincipalNames", default, skip_serializing_if = "Vec::is_empty")]
    pub service_principal_names: Vec<String>,
    #[doc = "the type of the service principal"]
    #[serde(rename = "servicePrincipalType", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_type: Option<String>,
    #[doc = "Optional list of tags that you can apply to your service principals. Not nullable."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}
impl ServicePrincipal {
    pub fn new(directory_object: DirectoryObject) -> Self {
        Self {
            directory_object,
            account_enabled: None,
            alternative_names: Vec::new(),
            app_display_name: None,
            app_id: None,
            app_owner_tenant_id: None,
            app_role_assignment_required: None,
            app_roles: Vec::new(),
            display_name: None,
            error_url: None,
            homepage: None,
            key_credentials: Vec::new(),
            logout_url: None,
            oauth2_permissions: Vec::new(),
            password_credentials: Vec::new(),
            preferred_token_signing_key_thumbprint: None,
            publisher_name: None,
            reply_urls: Vec::new(),
            saml_metadata_url: None,
            service_principal_names: Vec::new(),
            service_principal_type: None,
            tags: Vec::new(),
        }
    }
}
#[doc = "Active Directory service principal common properties shared among GET, POST and PATCH"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicePrincipalBase {
    #[doc = "whether or not the service principal account is enabled"]
    #[serde(rename = "accountEnabled", default, skip_serializing_if = "Option::is_none")]
    pub account_enabled: Option<bool>,
    #[doc = "Specifies whether an AppRoleAssignment to a user or group is required before Azure AD will issue a user or access token to the application."]
    #[serde(rename = "appRoleAssignmentRequired", default, skip_serializing_if = "Option::is_none")]
    pub app_role_assignment_required: Option<bool>,
    #[doc = "The collection of key credentials associated with the service principal."]
    #[serde(rename = "keyCredentials", default, skip_serializing_if = "Vec::is_empty")]
    pub key_credentials: Vec<KeyCredential>,
    #[doc = "The collection of password credentials associated with the service principal."]
    #[serde(rename = "passwordCredentials", default, skip_serializing_if = "Vec::is_empty")]
    pub password_credentials: Vec<PasswordCredential>,
    #[doc = "the type of the service principal"]
    #[serde(rename = "servicePrincipalType", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_type: Option<String>,
    #[doc = "Optional list of tags that you can apply to your service principals. Not nullable."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}
impl ServicePrincipalBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request parameters for creating a new service principal."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePrincipalCreateParameters {
    #[serde(flatten)]
    pub service_principal_base: ServicePrincipalBase,
    #[doc = "The application ID."]
    #[serde(rename = "appId")]
    pub app_id: String,
}
impl ServicePrincipalCreateParameters {
    pub fn new(app_id: String) -> Self {
        Self {
            service_principal_base: ServicePrincipalBase::default(),
            app_id,
        }
    }
}
#[doc = "Server response for get tenant service principals API call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicePrincipalListResult {
    #[doc = "the list of service principals."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServicePrincipal>,
    #[doc = "the URL to get the next set of results."]
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for ServicePrincipalListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl ServicePrincipalListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Service Principal Object Result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicePrincipalObjectResult {
    #[doc = "The Object ID of the service principal with the specified application ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The URL representing edm equivalent."]
    #[serde(rename = "odata.metadata", default, skip_serializing_if = "Option::is_none")]
    pub odata_metadata: Option<String>,
}
impl ServicePrincipalObjectResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request parameters for update an existing service principal."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServicePrincipalUpdateParameters {
    #[serde(flatten)]
    pub service_principal_base: ServicePrincipalBase,
}
impl ServicePrincipalUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains information about a sign-in name of a local account user in an Azure Active Directory B2C tenant."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SignInName {
    #[doc = "A string value that can be used to classify user sign-in types in your directory, such as 'emailAddress' or 'userName'."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The sign-in used by the local account. Must be unique across the company/tenant. For example, 'johnc@example.com'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl SignInName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Active Directory user information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(flatten)]
    pub directory_object: DirectoryObject,
    #[doc = "This must be specified if you are using a federated domain for the user's userPrincipalName (UPN) property when creating a new user account. It is used to associate an on-premises Active Directory user account with their Azure AD user object."]
    #[serde(rename = "immutableId", default, skip_serializing_if = "Option::is_none")]
    pub immutable_id: Option<String>,
    #[doc = "A two letter country code (ISO standard 3166). Required for users that will be assigned licenses due to legal requirement to check for availability of services in countries. Examples include: \"US\", \"JP\", and \"GB\"."]
    #[serde(rename = "usageLocation", default, skip_serializing_if = "Option::is_none")]
    pub usage_location: Option<String>,
    #[doc = "The given name for the user."]
    #[serde(rename = "givenName", default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[doc = "The user's surname (family name or last name)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    #[doc = "A string value that can be used to classify user types in your directory, such as 'Member' and 'Guest'."]
    #[serde(rename = "userType", default, skip_serializing_if = "Option::is_none")]
    pub user_type: Option<user::UserType>,
    #[doc = "Whether the account is enabled."]
    #[serde(rename = "accountEnabled", default, skip_serializing_if = "Option::is_none")]
    pub account_enabled: Option<bool>,
    #[doc = "The display name of the user."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The principal name of the user."]
    #[serde(rename = "userPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    #[doc = "The mail alias for the user."]
    #[serde(rename = "mailNickname", default, skip_serializing_if = "Option::is_none")]
    pub mail_nickname: Option<String>,
    #[doc = "The primary email address of the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
    #[doc = "The sign-in names of the user."]
    #[serde(rename = "signInNames", default, skip_serializing_if = "Vec::is_empty")]
    pub sign_in_names: Vec<SignInName>,
}
impl User {
    pub fn new(directory_object: DirectoryObject) -> Self {
        Self {
            directory_object,
            immutable_id: None,
            usage_location: None,
            given_name: None,
            surname: None,
            user_type: None,
            account_enabled: None,
            display_name: None,
            user_principal_name: None,
            mail_nickname: None,
            mail: None,
            sign_in_names: Vec::new(),
        }
    }
}
pub mod user {
    use super::*;
    #[doc = "A string value that can be used to classify user types in your directory, such as 'Member' and 'Guest'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UserType")]
    pub enum UserType {
        Member,
        Guest,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UserType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UserType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UserType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Member => serializer.serialize_unit_variant("UserType", 0u32, "Member"),
                Self::Guest => serializer.serialize_unit_variant("UserType", 1u32, "Guest"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserBase {
    #[doc = "This must be specified if you are using a federated domain for the user's userPrincipalName (UPN) property when creating a new user account. It is used to associate an on-premises Active Directory user account with their Azure AD user object."]
    #[serde(rename = "immutableId", default, skip_serializing_if = "Option::is_none")]
    pub immutable_id: Option<String>,
    #[doc = "A two letter country code (ISO standard 3166). Required for users that will be assigned licenses due to legal requirement to check for availability of services in countries. Examples include: \"US\", \"JP\", and \"GB\"."]
    #[serde(rename = "usageLocation", default, skip_serializing_if = "Option::is_none")]
    pub usage_location: Option<String>,
    #[doc = "The given name for the user."]
    #[serde(rename = "givenName", default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[doc = "The user's surname (family name or last name)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    #[doc = "A string value that can be used to classify user types in your directory, such as 'Member' and 'Guest'."]
    #[serde(rename = "userType", default, skip_serializing_if = "Option::is_none")]
    pub user_type: Option<user_base::UserType>,
}
impl UserBase {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod user_base {
    use super::*;
    #[doc = "A string value that can be used to classify user types in your directory, such as 'Member' and 'Guest'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UserType")]
    pub enum UserType {
        Member,
        Guest,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UserType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UserType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UserType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Member => serializer.serialize_unit_variant("UserType", 0u32, "Member"),
                Self::Guest => serializer.serialize_unit_variant("UserType", 1u32, "Guest"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Request parameters for creating a new work or school account user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserCreateParameters {
    #[serde(flatten)]
    pub user_base: UserBase,
    #[doc = "Whether the account is enabled."]
    #[serde(rename = "accountEnabled")]
    pub account_enabled: bool,
    #[doc = "The display name of the user."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The password profile associated with a user."]
    #[serde(rename = "passwordProfile")]
    pub password_profile: PasswordProfile,
    #[doc = "The user principal name (someuser@contoso.com). It must contain one of the verified domains for the tenant."]
    #[serde(rename = "userPrincipalName")]
    pub user_principal_name: String,
    #[doc = "The mail alias for the user."]
    #[serde(rename = "mailNickname")]
    pub mail_nickname: String,
    #[doc = "The primary email address of the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
}
impl UserCreateParameters {
    pub fn new(
        account_enabled: bool,
        display_name: String,
        password_profile: PasswordProfile,
        user_principal_name: String,
        mail_nickname: String,
    ) -> Self {
        Self {
            user_base: UserBase::default(),
            account_enabled,
            display_name,
            password_profile,
            user_principal_name,
            mail_nickname,
            mail: None,
        }
    }
}
#[doc = "Request parameters for GetMemberGroups API call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserGetMemberGroupsParameters {
    #[doc = "If true, only membership in security-enabled groups should be checked. Otherwise, membership in all groups should be checked."]
    #[serde(rename = "securityEnabledOnly")]
    pub security_enabled_only: bool,
}
impl UserGetMemberGroupsParameters {
    pub fn new(security_enabled_only: bool) -> Self {
        Self { security_enabled_only }
    }
}
#[doc = "Server response for GetMemberGroups API call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserGetMemberGroupsResult {
    #[doc = "A collection of group IDs of which the user is a member."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
}
impl azure_core::Continuable for UserGetMemberGroupsResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl UserGetMemberGroupsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Server response for Get tenant users API call."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserListResult {
    #[doc = "the list of users."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<User>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "odata.nextLink", default, skip_serializing_if = "Option::is_none")]
    pub odata_next_link: Option<String>,
}
impl azure_core::Continuable for UserListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.odata_next_link.clone()
    }
}
impl UserListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request parameters for updating an existing work or school account user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserUpdateParameters {
    #[serde(flatten)]
    pub user_base: UserBase,
    #[doc = "Whether the account is enabled."]
    #[serde(rename = "accountEnabled", default, skip_serializing_if = "Option::is_none")]
    pub account_enabled: Option<bool>,
    #[doc = "The display name of the user."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The password profile associated with a user."]
    #[serde(rename = "passwordProfile", default, skip_serializing_if = "Option::is_none")]
    pub password_profile: Option<PasswordProfile>,
    #[doc = "The user principal name (someuser@contoso.com). It must contain one of the verified domains for the tenant."]
    #[serde(rename = "userPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    #[doc = "The mail alias for the user."]
    #[serde(rename = "mailNickname", default, skip_serializing_if = "Option::is_none")]
    pub mail_nickname: Option<String>,
    #[doc = "The primary email address of the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
}
impl UserUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
