use crate::core::{
    hmac::sign,
    shared_access_signature::{format_date, format_form, SasProtocol, SasToken},
};
use chrono::{DateTime, Utc};
use std::fmt;

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
#[derive(Copy, Clone, Default)]
pub struct AccountSasPermissions {
    pub read: bool,
    pub write: bool,
    pub delete: bool,
    pub list: bool,
    pub add: bool,
    pub create: bool,
    pub update: bool,
    pub process: bool,
}

impl fmt::Display for AccountSasPermissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // NOTE: order *must* be `racwdxltmeop` per documentation:
        // https://docs.microsoft.com/en-us/rest/api/storageservices/create-service-sas#specifying-permissions

        if self.read {
            write!(f, "r")?;
        }
        if self.add {
            write!(f, "a")?;
        }
        if self.create {
            write!(f, "c")?;
        }
        if self.write {
            write!(f, "w")?;
        }
        if self.delete {
            write!(f, "d")?;
        }
        if self.list {
            write!(f, "l")?;
        }
        if self.update {
            write!(f, "u")?;
        }
        if self.process {
            write!(f, "p")?;
        }

        Ok(())
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
    pub fn new(account: String, key: String) -> AccountSharedAccessSignatureBuilder {
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

                sign(&string_to_sign, &self.key).unwrap()
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

pub struct AccountSharedAccessSignatureBuilder {
    account: String,
    key: String,
    signed_version: AccountSasVersion,
    signed_resource: Option<AccountSasResource>,
    signed_resource_type: Option<AccountSasResourceType>,
    signed_start: Option<DateTime<Utc>>,
    signed_expiry: Option<DateTime<Utc>>,
    signed_permissions: Option<AccountSasPermissions>,
    signed_ip: Option<String>,
    signed_protocol: Option<SasProtocol>,
}

impl AccountSharedAccessSignatureBuilder {
    pub fn new(account: String, key: String) -> Self {
        Self {
            account,
            key,
            signed_version: AccountSasVersion::V20181109,
            signed_resource: None,
            signed_resource_type: None,
            signed_start: None,
            signed_expiry: None,
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

    pub fn version(&self) -> AccountSasVersion {
        self.signed_version
    }

    pub fn with_version(self, version: AccountSasVersion) -> Self {
        Self {
            signed_version: version,
            ..self
        }
    }

    pub fn resource(&self) -> AccountSasResource {
        self.signed_resource.unwrap()
    }

    pub fn with_resource(self, resource: AccountSasResource) -> Self {
        Self {
            signed_resource: Some(resource),
            ..self
        }
    }

    pub fn resource_type_type(&self) -> AccountSasResourceType {
        self.signed_resource_type.unwrap()
    }

    pub fn with_resource_type(self, resource_type: AccountSasResourceType) -> Self {
        Self {
            signed_resource_type: Some(resource_type),
            ..self
        }
    }

    pub fn expiry(&self) -> DateTime<Utc> {
        self.signed_expiry.unwrap()
    }

    pub fn with_expiry(self, expiry: DateTime<Utc>) -> Self {
        Self {
            signed_expiry: Some(expiry),
            ..self
        }
    }

    pub fn signed_permissions(&self) -> AccountSasPermissions {
        self.signed_permissions.unwrap()
    }

    pub fn with_permissions(self, permissions: AccountSasPermissions) -> Self {
        Self {
            signed_permissions: Some(permissions),
            ..self
        }
    }

    pub fn signed_start(&self) -> DateTime<Utc> {
        self.signed_start.unwrap()
    }

    pub fn with_start(self, start: DateTime<Utc>) -> Self {
        Self {
            signed_start: Some(start),
            ..self
        }
    }

    pub fn ip(&self) -> &str {
        self.signed_ip.as_deref().unwrap()
    }

    pub fn with_ip(self, ip: String) -> Self {
        Self {
            signed_ip: Some(ip),
            ..self
        }
    }

    pub fn protocol(&self) -> SasProtocol {
        self.signed_protocol.unwrap()
    }

    pub fn with_protocol(self, protocol: SasProtocol) -> Self {
        Self {
            signed_protocol: Some(protocol),
            ..self
        }
    }
}
