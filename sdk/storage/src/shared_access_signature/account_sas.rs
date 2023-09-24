use crate::{
    hmac::sign,
    shared_access_signature::{format_date, SasProtocol, SasToken},
};
use std::fmt;
use time::OffsetDateTime;
use url::form_urlencoded;

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

/// Indicate which operations a `key_client` may perform on the resource ([Azure documentation](https://docs.microsoft.com/rest/api/storageservices/create-service-sas#specifying-permissions)).
#[allow(clippy::struct_excessive_bools)]
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
    fn sign(&self) -> String {
        match self.version {
            AccountSasVersion::V20181109 => {
                let string_to_sign = format!(
                    "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
                    self.account,
                    self.permissions,
                    self.resource,
                    self.resource_type,
                    self.start.map_or(String::new(), format_date),
                    format_date(self.expiry),
                    self.ip.clone().unwrap_or_default(),
                    self.protocol
                        .as_ref()
                        .map_or(String::new(), ToString::to_string),
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
        let mut form = form_urlencoded::Serializer::new(String::new());
        form.extend_pairs(&[
            ("sv", &self.version.to_string()),
            ("ss", &self.resource.to_string()),
            ("srt", &self.resource_type.to_string()),
            ("se", &format_date(self.expiry)),
            ("sp", &self.permissions.to_string()),
        ]);

        if let Some(start) = &self.start {
            form.append_pair("st", &format_date(*start));
        }
        if let Some(ip) = &self.ip {
            form.append_pair("sip", ip);
        }
        if let Some(protocol) = &self.protocol {
            form.append_pair("spr", &protocol.to_string());
        }
        let sig = self.sign();
        form.append_pair("sig", &sig);
        form.finish()
    }
}

impl PartialEq for AccountSharedAccessSignature {
    fn eq(&self, other: &Self) -> bool {
        self.sign() == other.sign()
    }
}

impl std::fmt::Debug for AccountSharedAccessSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "SharedAccessSignature {{{}}}", self.sign())
    }
}
