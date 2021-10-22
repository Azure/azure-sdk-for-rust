use crate::core::{
    shared_access_signature::{format_date, format_form, sign, SasProtocol, SasToken},
    No, ToAssign,
};
use chrono::{DateTime, Utc};
use std::{fmt, marker::PhantomData};

/// Service version of the shared access signature ([Azure documentation](https://docs.microsoft.com/rest/api/storageservices/create-service-sas#specifying-the-signed-version-field)).
#[derive(Copy, Clone)]
pub enum AccountSasVersion {
    V20181109,
    V20150405,
    V20130815,
    V20120212,
}

impl fmt::Display for AccountSasVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::V20181109 => write!(f, "2018-11-09"),
            Self::V20150405 => write!(f, "2015-04-05"),
            Self::V20130815 => write!(f, "2013-08-15"),
            Self::V20120212 => write!(f, "2012-02-12"),
        }
    }
}

#[derive(Copy, Clone)]
pub enum AccountSasService {
    Blob,
    Queue,
    Table,
    File,
}

impl fmt::Display for AccountSasService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Blob => write!(f, "b"),
            Self::Queue => write!(f, "q"),
            Self::Table => write!(f, "t"),
            Self::File => write!(f, "f"),
        }
    }
}

/// Which resources are accessible via the shared access signature ([Azure documentation](https://docs.microsoft.com/rest/api/storageservices/create-service-sas#specifying-the-signed-resource-blob-service-only)).
#[derive(Copy, Clone)]
pub enum AccountSasResource {
    Blob,
    Queue,
    Table,
    File,
}

impl fmt::Display for AccountSasResource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Blob => write!(f, "b"),
            Self::Queue => write!(f, "q"),
            Self::Table => write!(f, "t"),
            Self::File => write!(f, "f"),
        }
    }
}

#[derive(Copy, Clone)]
pub enum AccountSasResourceType {
    Service,
    Container,
    Object,
}

impl fmt::Display for AccountSasResourceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Service => write!(f, "s"),
            Self::Container => write!(f, "c"),
            Self::Object => write!(f, "o"),
        }
    }
}

/// Indicate which operations a key_client may perform on the resource ([Azure documentation](https://docs.microsoft.com/rest/api/storageservices/create-service-sas#specifying-permissions)).
#[derive(Copy, Clone)]
pub enum AccountSasPermissions {
    Read,
    Write,
    Delete,
    List,
    Add,
    Create,
    Update,
    Process,
}

impl fmt::Display for AccountSasPermissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Read => write!(f, "r"),
            Self::Write => write!(f, "w"),
            Self::Delete => write!(f, "d"),
            Self::List => write!(f, "l"),
            Self::Add => write!(f, "a"),
            Self::Create => write!(f, "c"),
            Self::Update => write!(f, "u"),
            Self::Process => write!(f, "p"),
        }
    }
}

pub struct AccountSharedAccessSignature {
    account: String,
    key: String,

    signed_version: AccountSasVersion,
    signed_resource: AccountSasResource,
    signed_resource_type: AccountSasResourceType,
    signed_start: Option<DateTime<Utc>>,
    signed_expiry: DateTime<Utc>,
    signed_permissions: AccountSasPermissions,
    signed_ip: Option<String>,
    signed_protocol: Option<SasProtocol>,
}

impl AccountSharedAccessSignature {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<'a>(
        account: &'a str,
        key: &'a str,
    ) -> AccountSharedAccessSignatureBuilder<'a, No, No, No, No> {
        AccountSharedAccessSignatureBuilder::new(account, key)
    }

    // Azure documentation: https://docs.microsoft.com/rest/api/storageservices/create-service-sas#constructing-the-signature-string
    fn signature(&self) -> String {
        match self.signed_version {
            AccountSasVersion::V20181109 => {
                let string_to_sign = format!(
                    "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
                    self.account,
                    self.signed_permissions,
                    self.signed_resource,
                    self.signed_resource_type,
                    self.signed_start.map_or("".to_string(), format_date),
                    format_date(self.signed_expiry),
                    self.signed_ip.clone().unwrap_or_else(|| "".to_string()),
                    self.signed_protocol
                        .as_ref()
                        .map_or("".to_string(), |v| v.to_string()),
                    self.signed_version,
                );

                sign(&self.key, &string_to_sign)
            }
            _ => {
                // TODO: support other version tags?
                unimplemented!("Versions older than 2018-11-09 not supported");
            }
        }
    }
}

impl SasToken for AccountSharedAccessSignature {
    /// [Example](https://docs.microsoft.com/rest/api/storageservices/create-service-sas#service-sas-example) from Azure documentation.
    fn token(&self) -> String {
        let mut elements: Vec<String> = vec![
            format!("sv={}", self.signed_version),
            format!("ss={}", self.signed_resource),
            format!("srt={}", self.signed_resource_type),
            format!("se={}", format_form(format_date(self.signed_expiry))),
            format!("sp={}", self.signed_permissions),
        ];

        if let Some(start) = &self.signed_start {
            elements.push(format!("st={}", format_form(format_date(*start))))
        }
        if let Some(ip) = &self.signed_ip {
            elements.push(format!("sip={}", ip))
        }
        if let Some(protocol) = &self.signed_protocol {
            elements.push(format!("spr={}", protocol))
        }
        let sig = AccountSharedAccessSignature::signature(self);
        elements.push(format!("sig={}", format_form(sig)));

        elements.join("&")
    }
}

impl PartialEq for AccountSharedAccessSignature {
    fn eq(&self, other: &Self) -> bool {
        self.signature() == other.signature()
    }
}

impl std::fmt::Debug for AccountSharedAccessSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SharedAccessSignature {{{}}}", self.signature())
    }
}

pub struct AccountSharedAccessSignatureBuilder<
    'a,
    SasResourceSet,
    SasResourceTypeSet,
    SasExpirySet,
    SasPermissionsSet,
> where
    SasResourceSet: ToAssign,
    SasResourceTypeSet: ToAssign,
    SasExpirySet: ToAssign,
    SasPermissionsSet: ToAssign,
{
    account: &'a str,
    key: &'a str,
    signed_version: AccountSasVersion,
    p_signed_resource: PhantomData<SasResourceSet>,
    signed_resource: Option<AccountSasResource>,
    p_signed_resource_type: PhantomData<SasResourceTypeSet>,
    signed_resource_type: Option<AccountSasResourceType>,
    signed_start: Option<DateTime<Utc>>,
    p_signed_expiry: PhantomData<SasExpirySet>,
    signed_expiry: Option<DateTime<Utc>>,
    p_signed_permissions: PhantomData<SasPermissionsSet>,
    signed_permissions: Option<AccountSasPermissions>,
    signed_ip: Option<String>,
    signed_protocol: Option<SasProtocol>,
}

impl<'a> AccountSharedAccessSignatureBuilder<'a, No, No, No, No> {
    pub fn new(account: &'a str, key: &'a str) -> Self {
        Self {
            account,
            key,
            signed_version: AccountSasVersion::V20181109,
            p_signed_resource: PhantomData {},
            signed_resource: None,
            p_signed_resource_type: PhantomData {},
            signed_resource_type: None,
            signed_start: None,
            p_signed_expiry: PhantomData {},
            signed_expiry: None,
            p_signed_permissions: PhantomData {},
            signed_permissions: None,
            signed_ip: None,
            signed_protocol: None,
        }
    }

    pub fn finalize(&self) -> AccountSharedAccessSignature {
        AccountSharedAccessSignature {
            account: self.account.to_owned(),
            key: self.key.to_owned(),
            signed_version: self.signed_version,
            signed_resource: self.signed_resource.unwrap(),
            signed_resource_type: self.signed_resource_type.unwrap(),
            signed_start: self.signed_start,
            signed_expiry: self.signed_expiry.unwrap(),
            signed_permissions: self.signed_permissions.unwrap(),
            signed_ip: self.signed_ip.clone(),
            signed_protocol: self.signed_protocol,
        }
    }
}

pub trait ClientAccountSharedAccessSignature {
    fn shared_access_signature(
        &self,
    ) -> Result<AccountSharedAccessSignatureBuilder<'_, No, No, No, No>, crate::Error>;
}

pub trait SasResourceRequired {
    fn resource(&self) -> AccountSasResource;
}

impl<'a, SasResourceSet, SasResourceTypeSet, SasExpirySet, SasPermissionsSet> SasResourceRequired
    for AccountSharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >
where
    SasResourceSet: ToAssign,
    SasResourceTypeSet: ToAssign,
    SasExpirySet: ToAssign,
    SasPermissionsSet: ToAssign,
{
    #[inline]
    fn resource(&self) -> AccountSasResource {
        self.signed_resource.unwrap()
    }
}

pub trait SasResourceSupport<'a> {
    type O;
    fn with_resource(self, resource: AccountSasResource) -> Self::O;
}

impl<'a, SasResourceSet, SasResourceTypeSet, SasExpirySet, SasPermissionsSet> SasResourceSupport<'a>
    for AccountSharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >
where
    SasResourceSet: ToAssign,
    SasResourceTypeSet: ToAssign,
    SasExpirySet: ToAssign,
    SasPermissionsSet: ToAssign,
{
    type O = AccountSharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >;

    #[inline]
    fn with_resource(self, resource: AccountSasResource) -> Self::O {
        AccountSharedAccessSignatureBuilder {
            account: self.account,
            key: self.key,
            signed_version: self.signed_version,
            p_signed_resource: PhantomData {},
            signed_resource: Some(resource),
            p_signed_resource_type: PhantomData {},
            signed_resource_type: self.signed_resource_type,
            signed_start: self.signed_start,
            p_signed_expiry: PhantomData {},
            signed_expiry: self.signed_expiry,
            p_signed_permissions: PhantomData {},
            signed_permissions: self.signed_permissions,
            signed_ip: self.signed_ip,
            signed_protocol: self.signed_protocol,
        }
    }
}

pub trait SasResourceTypeRequired {
    fn resource_type(&self) -> AccountSasResourceType;
}

impl<'a, SasResourceSet, SasResourceTypeSet, SasExpirySet, SasPermissionsSet>
    SasResourceTypeRequired
    for AccountSharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >
where
    SasResourceSet: ToAssign,
    SasResourceTypeSet: ToAssign,
    SasExpirySet: ToAssign,
    SasPermissionsSet: ToAssign,
{
    #[inline]
    fn resource_type(&self) -> AccountSasResourceType {
        self.signed_resource_type.unwrap()
    }
}

pub trait SasResourceTypeSupport<'a> {
    type O;
    fn with_resource_type(self, resource_type: AccountSasResourceType) -> Self::O;
}

impl<'a, SasResourceSet, SasResourceTypeSet, SasExpirySet, SasPermissionsSet>
    SasResourceTypeSupport<'a>
    for AccountSharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >
where
    SasResourceSet: ToAssign,
    SasResourceTypeSet: ToAssign,
    SasExpirySet: ToAssign,
    SasPermissionsSet: ToAssign,
{
    type O = AccountSharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >;

    #[inline]
    fn with_resource_type(self, resource_type: AccountSasResourceType) -> Self::O {
        AccountSharedAccessSignatureBuilder {
            account: self.account,
            key: self.key,
            signed_version: self.signed_version,
            p_signed_resource: PhantomData {},
            signed_resource: self.signed_resource,
            p_signed_resource_type: PhantomData {},
            signed_resource_type: Some(resource_type),
            signed_start: self.signed_start,
            p_signed_expiry: PhantomData {},
            signed_expiry: self.signed_expiry,
            p_signed_permissions: PhantomData {},
            signed_permissions: self.signed_permissions,
            signed_ip: self.signed_ip,
            signed_protocol: self.signed_protocol,
        }
    }
}

pub trait SasExpiryRequired {
    fn signed_expiry(&self) -> DateTime<Utc>;
}

impl<'a, SasResourceSet, SasResourceTypeSet, SasExpirySet, SasPermissionsSet> SasExpiryRequired
    for AccountSharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >
where
    SasResourceSet: ToAssign,
    SasResourceTypeSet: ToAssign,
    SasExpirySet: ToAssign,
    SasPermissionsSet: ToAssign,
{
    #[inline]
    fn signed_expiry(&self) -> DateTime<Utc> {
        self.signed_expiry.unwrap()
    }
}

pub trait SasExpirySupport<'a> {
    type O;
    fn with_expiry(self, expiry: DateTime<Utc>) -> Self::O;
}

impl<'a, SasResourceSet, SasResourceTypeSet, SasExpirySet, SasPermissionsSet> SasExpirySupport<'a>
    for AccountSharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >
where
    SasResourceSet: ToAssign,
    SasResourceTypeSet: ToAssign,
    SasExpirySet: ToAssign,
    SasPermissionsSet: ToAssign,
{
    type O = AccountSharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >;

    #[inline]
    fn with_expiry(self, expiry: DateTime<Utc>) -> Self::O {
        AccountSharedAccessSignatureBuilder {
            account: self.account,
            key: self.key,
            signed_version: self.signed_version,
            p_signed_resource: PhantomData {},
            signed_resource: self.signed_resource,
            p_signed_resource_type: PhantomData {},
            signed_resource_type: self.signed_resource_type,
            signed_start: self.signed_start,
            p_signed_expiry: PhantomData {},
            signed_expiry: Some(expiry),
            p_signed_permissions: PhantomData {},
            signed_permissions: self.signed_permissions,
            signed_ip: self.signed_ip,
            signed_protocol: self.signed_protocol,
        }
    }
}

pub trait SasPermissionsRequired {
    fn signed_permissions(&self) -> AccountSasPermissions;
}

impl<'a, SasResourceSet, SasResourceTypeSet, SasExpirySet, SasPermissionsSet> SasPermissionsRequired
    for AccountSharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >
where
    SasResourceSet: ToAssign,
    SasResourceTypeSet: ToAssign,
    SasExpirySet: ToAssign,
    SasPermissionsSet: ToAssign,
{
    #[inline]
    fn signed_permissions(&self) -> AccountSasPermissions {
        self.signed_permissions.unwrap()
    }
}

pub trait SasPermissionsSupport<'a> {
    type O;
    fn with_permissions(self, permissions: AccountSasPermissions) -> Self::O;
}

impl<'a, SasResourceSet, SasResourceTypeSet, SasExpirySet, SasPermissionsSet>
    SasPermissionsSupport<'a>
    for AccountSharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >
where
    SasResourceSet: ToAssign,
    SasResourceTypeSet: ToAssign,
    SasExpirySet: ToAssign,
    SasPermissionsSet: ToAssign,
{
    type O = AccountSharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >;

    #[inline]
    fn with_permissions(self, permissions: AccountSasPermissions) -> Self::O {
        AccountSharedAccessSignatureBuilder {
            account: self.account,
            key: self.key,
            signed_version: self.signed_version,
            p_signed_resource: PhantomData {},
            signed_resource: self.signed_resource,
            p_signed_resource_type: PhantomData {},
            signed_resource_type: self.signed_resource_type,
            signed_start: self.signed_start,
            p_signed_expiry: PhantomData {},
            signed_expiry: self.signed_expiry,
            p_signed_permissions: PhantomData {},
            signed_permissions: Some(permissions),
            signed_ip: self.signed_ip,
            signed_protocol: self.signed_protocol,
        }
    }
}

pub trait SasStartSupport<'a> {
    type O;
    fn with_start(self, start: DateTime<Utc>) -> Self::O;
}

impl<'a, SasResourceSet, SasResourceTypeSet, SasExpirySet, SasPermissionsSet> SasStartSupport<'a>
    for AccountSharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >
where
    SasResourceSet: ToAssign,
    SasResourceTypeSet: ToAssign,
    SasExpirySet: ToAssign,
    SasPermissionsSet: ToAssign,
{
    type O = AccountSharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >;

    #[inline]
    fn with_start(self, start: DateTime<Utc>) -> Self::O {
        AccountSharedAccessSignatureBuilder {
            account: self.account,
            key: self.key,
            signed_version: self.signed_version,
            p_signed_resource: PhantomData {},
            signed_resource: self.signed_resource,
            p_signed_resource_type: PhantomData {},
            signed_resource_type: self.signed_resource_type,
            signed_start: Some(start),
            p_signed_expiry: PhantomData {},
            signed_expiry: self.signed_expiry,
            p_signed_permissions: PhantomData {},
            signed_permissions: self.signed_permissions,
            signed_ip: self.signed_ip,
            signed_protocol: self.signed_protocol,
        }
    }
}

pub trait SasIpSupport<'a> {
    type O;
    fn with_ip(self, ip: &str) -> Self::O;
}

impl<'a, SasResourceSet, SasResourceTypeSet, SasExpirySet, SasPermissionsSet> SasIpSupport<'a>
    for AccountSharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >
where
    SasResourceSet: ToAssign,
    SasResourceTypeSet: ToAssign,
    SasExpirySet: ToAssign,
    SasPermissionsSet: ToAssign,
{
    type O = AccountSharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >;

    #[inline]
    fn with_ip(self, ip: &str) -> Self::O {
        AccountSharedAccessSignatureBuilder {
            account: self.account,
            key: self.key,
            signed_version: self.signed_version,
            p_signed_resource: PhantomData {},
            signed_resource: self.signed_resource,
            p_signed_resource_type: PhantomData {},
            signed_resource_type: self.signed_resource_type,
            signed_start: self.signed_start,
            p_signed_expiry: PhantomData {},
            signed_expiry: self.signed_expiry,
            p_signed_permissions: PhantomData {},
            signed_permissions: self.signed_permissions,
            signed_ip: Some(ip.to_string()),
            signed_protocol: self.signed_protocol,
        }
    }
}

pub trait SasProtocolSupport<'a> {
    type O;
    fn with_protocol(self, protocol: SasProtocol) -> Self::O;
}

impl<'a, SasResourceSet, SasResourceTypeSet, SasExpirySet, SasPermissionsSet> SasProtocolSupport<'a>
    for AccountSharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >
where
    SasResourceSet: ToAssign,
    SasResourceTypeSet: ToAssign,
    SasExpirySet: ToAssign,
    SasPermissionsSet: ToAssign,
{
    type O = AccountSharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >;

    #[inline]
    fn with_protocol(self, protocol: SasProtocol) -> Self::O {
        AccountSharedAccessSignatureBuilder {
            account: self.account,
            key: self.key,
            signed_version: self.signed_version,
            p_signed_resource: PhantomData {},
            signed_resource: self.signed_resource,
            p_signed_resource_type: PhantomData {},
            signed_resource_type: self.signed_resource_type,
            signed_start: self.signed_start,
            p_signed_expiry: PhantomData {},
            signed_expiry: self.signed_expiry,
            p_signed_permissions: PhantomData {},
            signed_permissions: self.signed_permissions,
            signed_ip: self.signed_ip,
            signed_protocol: Some(protocol),
        }
    }
}
