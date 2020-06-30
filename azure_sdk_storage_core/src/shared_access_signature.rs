use super::{KeyClient, KeyClientRequired};
use crate::client_endpoint::ClientEndpoint;
use azure_sdk_core::{No, ToAssign};
use base64::encode;
use chrono::{DateTime, Utc};
use ring::hmac;
use std::fmt;
use std::marker::PhantomData;
use url::form_urlencoded;

/// Service version of the shared access signature ([Azure documentation](https://docs.microsoft.com/en-us/rest/api/storageservices/create-service-sas#specifying-the-signed-version-field)).
#[derive(Copy, Clone)]
pub enum SasVersion {
    V20181109,
    V20150405,
    V20130815,
    V20120212,
}

impl fmt::Display for SasVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SasVersion::V20181109 => write!(f, "2018-11-09"),
            SasVersion::V20150405 => write!(f, "2015-04-05"),
            SasVersion::V20130815 => write!(f, "2013-08-15"),
            SasVersion::V20120212 => write!(f, "2012-02-12"),
        }
    }
}

#[derive(Copy, Clone)]
pub enum SasService {
    Blob,
    Queue,
    Table,
    File,
}

impl fmt::Display for SasService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SasService::Blob => write!(f, "b"),
            SasService::Queue => write!(f, "q"),
            SasService::Table => write!(f, "t"),
            SasService::File => write!(f, "f"),
        }
    }
}

/// Specifies the protocol permitted for a request made with the SAS ([Azure documentation](https://docs.microsoft.com/en-us/rest/api/storageservices/create-service-sas#specifying-the-http-protocol)).
#[derive(Copy, Clone)]
pub enum SasProtocol {
    Https,
    HttpHttps,
}

impl fmt::Display for SasProtocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SasProtocol::Https => write!(f, "https"),
            SasProtocol::HttpHttps => write!(f, "http,https"),
        }
    }
}

/// Which resources are accessible via the shared access signature ([Azure documentation](https://docs.microsoft.com/en-us/rest/api/storageservices/create-service-sas#specifying-the-signed-resource-blob-service-only)).
#[derive(Copy, Clone)]
pub enum SasResource {
    Blob,
    Queue,
    Table,
    File,
}

impl fmt::Display for SasResource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SasResource::Blob => write!(f, "b"),
            SasResource::Queue => write!(f, "q"),
            SasResource::Table => write!(f, "t"),
            SasResource::File => write!(f, "f"),
        }
    }
}

#[derive(Copy, Clone)]
pub enum SasResourceType {
    Service,
    Container,
    Object,
}

impl fmt::Display for SasResourceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SasResourceType::Service => write!(f, "s"),
            SasResourceType::Container => write!(f, "c"),
            SasResourceType::Object => write!(f, "o"),
        }
    }
}

/// Indicate which operations a key_client may perform on the resource ([Azure documentation](https://docs.microsoft.com/en-us/rest/api/storageservices/create-service-sas#specifying-permissions)).
#[derive(Copy, Clone)]
pub enum SasPermissions {
    Read,
    Write,
    Delete,
    List,
    Add,
    Create,
    Update,
    Process,
}

impl fmt::Display for SasPermissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SasPermissions::Read => write!(f, "r"),
            SasPermissions::Write => write!(f, "w"),
            SasPermissions::Delete => write!(f, "d"),
            SasPermissions::List => write!(f, "l"),
            SasPermissions::Add => write!(f, "a"),
            SasPermissions::Create => write!(f, "c"),
            SasPermissions::Update => write!(f, "u"),
            SasPermissions::Process => write!(f, "p"),
        }
    }
}

pub struct SharedAccessSignature {
    account: String,
    key: String,

    signed_version: SasVersion,
    signed_resource: SasResource,
    signed_resource_type: SasResourceType,
    signed_start: Option<DateTime<Utc>>,
    signed_expiry: DateTime<Utc>,
    signed_permissions: SasPermissions,
    signed_ip: Option<String>,
    signed_protocol: Option<SasProtocol>,
}

impl SharedAccessSignature {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(key_client: &KeyClient) -> SharedAccessSignatureBuilder<No, No, No, No> {
        SharedAccessSignatureBuilder::new(key_client)
    }

    fn format_date(d: DateTime<Utc>) -> String {
        d.format("%Y-%m-%dT%H:%M:%SZ").to_string()
    }

    // Azure documentation: https://docs.microsoft.com/en-us/rest/api/storageservices/create-service-sas#constructing-the-signature-string
    fn signature(&self) -> String {
        match self.signed_version {
            SasVersion::V20181109 => {
                let string_to_sign = format!(
                    "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
                    self.account,
                    self.signed_permissions,
                    self.signed_resource,
                    self.signed_resource_type,
                    self.signed_start
                        .map_or("".to_string(), SharedAccessSignature::format_date),
                    SharedAccessSignature::format_date(self.signed_expiry),
                    self.signed_ip.clone().unwrap_or_else(|| "".to_string()),
                    self.signed_protocol
                        .as_ref()
                        .map_or("".to_string(), |v| v.to_string()),
                    self.signed_version,
                );

                let key =
                    hmac::Key::new(ring::hmac::HMAC_SHA256, &base64::decode(&self.key).unwrap());
                let sig_bytes = hmac::sign(&key, string_to_sign.as_bytes());

                encode(&sig_bytes)
            }
            _ => {
                // TODO: support other version tags?
                unimplemented!("Versions older than 2018-11-09 not supported");
            }
        }
    }

    /// [Example](https://docs.microsoft.com/en-us/rest/api/storageservices/create-service-sas#service-sas-example) from Azure documentation.
    pub fn token(&self) -> String {
        let mut elements: Vec<String> = vec![
            format!("sv={}", self.signed_version),
            format!("ss={}", self.signed_resource),
            format!("srt={}", self.signed_resource_type),
            format!(
                "se={}",
                form_urlencoded::byte_serialize(
                    SharedAccessSignature::format_date(self.signed_expiry).as_bytes()
                )
                .collect::<String>()
            ),
            format!("sp={}", self.signed_permissions),
        ];

        if let Some(start) = &self.signed_start {
            elements.push(format!(
                "st={}",
                form_urlencoded::byte_serialize(
                    SharedAccessSignature::format_date(*start).as_bytes()
                )
                .collect::<String>()
            ))
        }
        if let Some(ip) = &self.signed_ip {
            elements.push(format!("sip={}", ip))
        }
        if let Some(protocol) = &self.signed_protocol {
            elements.push(format!("spr={}", protocol))
        }
        let sig = SharedAccessSignature::signature(self);
        elements.push(format!(
            "sig={}",
            form_urlencoded::byte_serialize(sig.as_bytes()).collect::<String>()
        ));

        elements.join("&")
    }
}

impl PartialEq for SharedAccessSignature {
    fn eq(&self, other: &Self) -> bool {
        self.signature() == other.signature()
    }
}

impl std::fmt::Debug for SharedAccessSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SharedAccessSignature {{{}}}", self.signature())
    }
}

pub struct SharedAccessSignatureBuilder<
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
    key_client: &'a KeyClient,
    signed_version: SasVersion,
    p_signed_resource: PhantomData<SasResourceSet>,
    signed_resource: Option<SasResource>,
    p_signed_resource_type: PhantomData<SasResourceTypeSet>,
    signed_resource_type: Option<SasResourceType>,
    signed_start: Option<DateTime<Utc>>,
    p_signed_expiry: PhantomData<SasExpirySet>,
    signed_expiry: Option<DateTime<Utc>>,
    p_signed_permissions: PhantomData<SasPermissionsSet>,
    signed_permissions: Option<SasPermissions>,
    signed_ip: Option<String>,
    signed_protocol: Option<SasProtocol>,
}

impl<'a> SharedAccessSignatureBuilder<'a, No, No, No, No> {
    pub fn new(key_client: &'a KeyClient) -> Self {
        Self {
            key_client,
            signed_version: SasVersion::V20181109,
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

    pub fn finalize(&self) -> SharedAccessSignature {
        SharedAccessSignature {
            account: self.key_client.account().to_string(),
            key: self.key_client.key().to_string(),

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

impl<'a, SasResourceSet, SasResourceTypeSet, SasExpirySet, SasPermissionsSet> KeyClientRequired<'a>
    for SharedAccessSignatureBuilder<
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
    fn key_client(&self) -> &'a KeyClient {
        self.key_client
    }
}

pub trait ClientSharedAccessSignature {
    fn shared_access_signature(&self) -> SharedAccessSignatureBuilder<'_, No, No, No, No>;
}

impl ClientSharedAccessSignature for KeyClient {
    /// Grant restricted access rights to Azure Storage resources ([Azure documentation](https://docs.microsoft.com/en-us/rest/api/storageservices/delegate-access-with-shared-access-signature)).
    fn shared_access_signature(&self) -> SharedAccessSignatureBuilder<'_, No, No, No, No> {
        SharedAccessSignature::new(self)
    }
}

pub trait SasResourceRequired {
    fn resource(&self) -> SasResource;
}

impl<'a, SasResourceSet, SasResourceTypeSet, SasExpirySet, SasPermissionsSet> SasResourceRequired
    for SharedAccessSignatureBuilder<
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
    fn resource(&self) -> SasResource {
        self.signed_resource.unwrap()
    }
}

pub trait SasResourceSupport<'a> {
    type O;
    fn with_resource(self, resource: SasResource) -> Self::O;
}

impl<'a, SasResourceSet, SasResourceTypeSet, SasExpirySet, SasPermissionsSet> SasResourceSupport<'a>
    for SharedAccessSignatureBuilder<
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
    type O = SharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >;

    #[inline]
    fn with_resource(self, resource: SasResource) -> Self::O {
        SharedAccessSignatureBuilder {
            key_client: self.key_client,
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
    fn resource_type(&self) -> SasResourceType;
}

impl<'a, SasResourceSet, SasResourceTypeSet, SasExpirySet, SasPermissionsSet>
    SasResourceTypeRequired
    for SharedAccessSignatureBuilder<
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
    fn resource_type(&self) -> SasResourceType {
        self.signed_resource_type.unwrap()
    }
}

pub trait SasResourceTypeSupport<'a> {
    type O;
    fn with_resource_type(self, resource_type: SasResourceType) -> Self::O;
}

impl<'a, SasResourceSet, SasResourceTypeSet, SasExpirySet, SasPermissionsSet>
    SasResourceTypeSupport<'a>
    for SharedAccessSignatureBuilder<
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
    type O = SharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >;

    #[inline]
    fn with_resource_type(self, resource_type: SasResourceType) -> Self::O {
        SharedAccessSignatureBuilder {
            key_client: self.key_client,
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
    for SharedAccessSignatureBuilder<
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
    for SharedAccessSignatureBuilder<
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
    type O = SharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >;

    #[inline]
    fn with_expiry(self, expiry: DateTime<Utc>) -> Self::O {
        SharedAccessSignatureBuilder {
            key_client: self.key_client,
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
    fn signed_permissions(&self) -> SasPermissions;
}

impl<'a, SasResourceSet, SasResourceTypeSet, SasExpirySet, SasPermissionsSet> SasPermissionsRequired
    for SharedAccessSignatureBuilder<
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
    fn signed_permissions(&self) -> SasPermissions {
        self.signed_permissions.unwrap()
    }
}

pub trait SasPermissionsSupport<'a> {
    type O;
    fn with_permissions(self, permissions: SasPermissions) -> Self::O;
}

impl<'a, SasResourceSet, SasResourceTypeSet, SasExpirySet, SasPermissionsSet>
    SasPermissionsSupport<'a>
    for SharedAccessSignatureBuilder<
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
    type O = SharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >;

    #[inline]
    fn with_permissions(self, permissions: SasPermissions) -> Self::O {
        SharedAccessSignatureBuilder {
            key_client: self.key_client,
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
    for SharedAccessSignatureBuilder<
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
    type O = SharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >;

    #[inline]
    fn with_start(self, start: DateTime<Utc>) -> Self::O {
        SharedAccessSignatureBuilder {
            key_client: self.key_client,
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
    for SharedAccessSignatureBuilder<
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
    type O = SharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >;

    #[inline]
    fn with_ip(self, ip: &str) -> Self::O {
        SharedAccessSignatureBuilder {
            key_client: self.key_client,
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
    for SharedAccessSignatureBuilder<
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
    type O = SharedAccessSignatureBuilder<
        'a,
        SasResourceSet,
        SasResourceTypeSet,
        SasExpirySet,
        SasPermissionsSet,
    >;

    #[inline]
    fn with_protocol(self, protocol: SasProtocol) -> Self::O {
        SharedAccessSignatureBuilder {
            key_client: self.key_client,
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
