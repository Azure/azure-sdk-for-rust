use crate::core::{
    hmac::sign,
    shared_access_signature::{format_date, format_form, SasProtocol, SasToken},
};
use std::fmt;
use time::OffsetDateTime;

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
    version: AccountSasVersion,
    resource: AccountSasResource,
    resource_type: AccountSasResourceType,
    expiry: OffsetDateTime,
    permissions: AccountSasPermissions,
    start: Option<OffsetDateTime>,
    ip: Option<String>,
    protocol: Option<SasProtocol>,
}

impl AccountSharedAccessSignature {
    pub fn new(
        account: String,
        key: String,
        resource: AccountSasResource,
        resource_type: AccountSasResourceType,
        expiry: OffsetDateTime,
        permissions: AccountSasPermissions,
    ) -> Self {
        Self {
            account,
            key,
            version: AccountSasVersion::V20181109,
            resource,
            resource_type,
            expiry,
            permissions,
            start: None,
            ip: None,
            protocol: None,
        }
    }

    setters! {
        version: AccountSasVersion => version,
        start: OffsetDateTime => Some(start),
        ip: String => Some(ip),
        protocol: SasProtocol => Some(protocol),
    }

    // Azure documentation: https://docs.microsoft.com/rest/api/storageservices/create-service-sas#constructing-the-signature-string
    fn signature(&self) -> String {
        match self.version {
            AccountSasVersion::V20181109 => {
                let string_to_sign = format!(
                    "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
                    self.account,
                    self.permissions,
                    self.resource,
                    self.resource_type,
                    self.start.map_or("".to_string(), format_date),
                    format_date(self.expiry),
                    self.ip.clone().unwrap_or_else(|| "".to_string()),
                    self.protocol
                        .as_ref()
                        .map_or("".to_string(), |v| v.to_string()),
                    self.version,
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
            format!("sv={}", self.version),
            format!("ss={}", self.resource),
            format!("srt={}", self.resource_type),
            format!("se={}", format_form(format_date(self.expiry))),
            format!("sp={}", self.permissions),
        ];

        if let Some(start) = &self.start {
            elements.push(format!("st={}", format_form(format_date(*start))))
        }
        if let Some(ip) = &self.ip {
            elements.push(format!("sip={}", ip))
        }
        if let Some(protocol) = &self.protocol {
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
