use crate::{
    core::shared_access_signature::{format_date, format_form, SasProtocol, SasToken},
    hmac,
};
use chrono::{DateTime, Utc};
use std::{fmt, marker::PhantomData};

const SERVICE_SAS_VERSION: &str = "2020-06-12";

pub enum BlobSignedResource {
    Blob,         // b
    BlobVersion,  // bv
    BlobSnapshot, // bs
    Container,    // c
    Directory,    // d
}

impl fmt::Display for BlobSignedResource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Blob => write!(f, "b"),
            Self::BlobVersion => write!(f, "bv"),
            Self::BlobSnapshot => write!(f, "bs"),
            Self::Container => write!(f, "c"),
            Self::Directory => write!(f, "d"),
        }
    }
}

#[derive(Default)]
pub struct BlobSasPermissions {
    pub read: bool,              // r - Container | Directory | Blob
    pub add: bool,               // a - Container | Directory | Blob
    pub create: bool,            // c - Container | Directory | Blob
    pub write: bool,             // w - Container | Directory | Blob
    pub delete: bool,            // d - Container | Directory | Blob
    pub delete_version: bool,    // x - Container | Blob
    pub permantent_delete: bool, // y - Blob
    pub list: bool,              // l - Container | Directory
    pub tags: bool,              // t - Tags
    pub move_: bool,             // m - Container | Directory | Blob
    pub execute: bool,           // e - Container | Directory | Blob
    pub ownership: bool,         // o - Container | Directory | Blob
    pub permissions: bool,       // p - Container | Directory | Blob
                                 // SetImmunabilityPolicy: bool, // i  -- container
}

impl fmt::Display for BlobSasPermissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.read {
            write!(f, "r")?
        };
        if self.add {
            write!(f, "a")?
        };
        if self.create {
            write!(f, "c")?
        };
        if self.write {
            write!(f, "w")?
        };
        if self.delete {
            write!(f, "d")?
        };
        if self.delete_version {
            write!(f, "x")?
        };
        if self.permantent_delete {
            write!(f, "y")?
        };
        if self.list {
            write!(f, "l")?
        };
        if self.tags {
            write!(f, "t")?
        };
        if self.move_ {
            write!(f, "m")?
        };
        if self.execute {
            write!(f, "e")?
        };
        if self.ownership {
            write!(f, "o")?
        };
        if self.permissions {
            write!(f, "p")?
        };
        Ok(())
    }
}

pub struct BlobSharedAccessSignature {
    key: String,

    canonicalized_resource: String,
    signed_permissions: BlobSasPermissions, // sp
    signed_start: Option<DateTime<Utc>>,    // st
    signed_expiry: DateTime<Utc>,           // se
    signed_identifier: Option<String>,
    signed_ip: Option<String>,
    signed_protocol: Option<SasProtocol>,
    signed_resource: BlobSignedResource,
}

impl BlobSharedAccessSignature {
    fn sign(&self) -> String {
        let content = vec![
            self.signed_permissions.to_string(),
            self.signed_start.map_or("".to_string(), format_date),
            format_date(self.signed_expiry),
            self.canonicalized_resource.clone(),
            self.signed_identifier
                .as_ref()
                .unwrap_or(&"".to_string())
                .to_string(),
            self.signed_ip
                .as_ref()
                .unwrap_or(&"".to_string())
                .to_string(),
            self.signed_protocol
                .map(|x| x.to_string())
                .unwrap_or_else(|| "".to_string()),
            SERVICE_SAS_VERSION.to_string(),
            self.signed_resource.to_string(),
            "".to_string(), // snapshot time
            "".to_string(), // rscd
            "".to_string(), // rscc
            "".to_string(), // rsce
            "".to_string(), // rscl
            "".to_string(), // rsct
        ];

        hmac::sign(&content.join("\n"), &self.key).unwrap()
    }
}

impl SasToken for BlobSharedAccessSignature {
    fn token(&self) -> String {
        let mut elements: Vec<String> = vec![
            format!("sv={}", SERVICE_SAS_VERSION),
            format!("sp={}", self.signed_permissions),
            format!("sr={}", self.signed_resource),
            format!("se={}", format_form(format_date(self.signed_expiry))),
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

        let sig = self.sign();
        elements.push(format!("sig={}", format_form(sig)));

        elements.join("&")
    }
}

pub struct SetPerms {}
pub struct SetResources {}
pub struct SetExpiry {}

#[derive(Default)]
pub struct BlobSharedAccessSignatureBuilder<T1, T2, T3> {
    _phantom: PhantomData<(T1, T2, T3)>,
    key: String,
    canonicalized_resource: String,

    // required
    signed_permissions: Option<BlobSasPermissions>,
    signed_expiry: Option<DateTime<Utc>>,
    signed_resource: Option<BlobSignedResource>,

    // optional
    signed_start: Option<DateTime<Utc>>,
    signed_identifier: Option<String>,
    signed_ip: Option<String>,
    signed_protocol: Option<SasProtocol>,
}

impl BlobSharedAccessSignatureBuilder<(), (), ()> {
    #[must_use]
    pub fn new(
        key: String,
        canonicalized_resource: String,
    ) -> BlobSharedAccessSignatureBuilder<(), (), ()> {
        BlobSharedAccessSignatureBuilder {
            _phantom: PhantomData,
            key,
            canonicalized_resource,
            signed_permissions: None,
            signed_expiry: None,
            signed_resource: None,
            signed_start: None,
            signed_identifier: None,
            signed_ip: None,
            signed_protocol: None,
        }
    }
}

impl<T1, T2, T3> BlobSharedAccessSignatureBuilder<T1, T2, T3> {
    #[must_use]
    pub fn with_start(
        self,
        signed_start: DateTime<Utc>,
    ) -> BlobSharedAccessSignatureBuilder<T1, T2, T3> {
        BlobSharedAccessSignatureBuilder {
            _phantom: PhantomData,
            key: self.key,
            canonicalized_resource: self.canonicalized_resource,
            signed_permissions: self.signed_permissions,
            signed_resource: self.signed_resource,
            signed_expiry: self.signed_expiry,
            signed_start: Some(signed_start),
            signed_identifier: self.signed_identifier,
            signed_protocol: self.signed_protocol,
            signed_ip: self.signed_ip,
        }
    }
    #[must_use]
    pub fn with_ip(self, signed_ip: String) -> BlobSharedAccessSignatureBuilder<T1, T2, T3> {
        BlobSharedAccessSignatureBuilder {
            _phantom: PhantomData,
            key: self.key,
            canonicalized_resource: self.canonicalized_resource,
            signed_permissions: self.signed_permissions,
            signed_resource: self.signed_resource,
            signed_expiry: self.signed_expiry,
            signed_start: self.signed_start,
            signed_identifier: self.signed_identifier,
            signed_protocol: self.signed_protocol,
            signed_ip: Some(signed_ip),
        }
    }
    #[must_use]
    pub fn with_identifier(
        self,
        signed_identifier: String,
    ) -> BlobSharedAccessSignatureBuilder<T1, T2, T3> {
        BlobSharedAccessSignatureBuilder {
            _phantom: PhantomData,
            key: self.key,
            canonicalized_resource: self.canonicalized_resource,
            signed_permissions: self.signed_permissions,
            signed_resource: self.signed_resource,
            signed_expiry: self.signed_expiry,
            signed_start: self.signed_start,
            signed_identifier: Some(signed_identifier),
            signed_protocol: self.signed_protocol,
            signed_ip: self.signed_ip,
        }
    }
    #[must_use]
    pub fn with_protocol(
        self,
        signed_protocol: SasProtocol,
    ) -> BlobSharedAccessSignatureBuilder<T1, T2, T3> {
        BlobSharedAccessSignatureBuilder {
            _phantom: PhantomData,
            key: self.key,
            canonicalized_resource: self.canonicalized_resource,
            signed_permissions: self.signed_permissions,
            signed_resource: self.signed_resource,
            signed_expiry: self.signed_expiry,
            signed_start: self.signed_start,
            signed_identifier: self.signed_identifier,
            signed_protocol: Some(signed_protocol),
            signed_ip: self.signed_ip,
        }
    }
}

impl BlobSharedAccessSignatureBuilder<SetPerms, SetResources, SetExpiry> {
    #[must_use]
    pub fn finalize(self) -> BlobSharedAccessSignature {
        BlobSharedAccessSignature {
            key: self.key.clone(),
            canonicalized_resource: self.canonicalized_resource.clone(),
            signed_permissions: self.signed_permissions.unwrap(),
            signed_resource: self.signed_resource.unwrap(),
            signed_expiry: self.signed_expiry.unwrap(),
            signed_start: self.signed_start,
            signed_identifier: self.signed_identifier,
            signed_protocol: self.signed_protocol,
            signed_ip: self.signed_ip,
        }
    }
}

impl<T1, T2> BlobSharedAccessSignatureBuilder<(), T1, T2> {
    #[must_use]
    pub fn with_permissions(
        self,
        permissions: BlobSasPermissions,
    ) -> BlobSharedAccessSignatureBuilder<SetPerms, T1, T2> {
        BlobSharedAccessSignatureBuilder {
            _phantom: PhantomData,
            key: self.key,
            canonicalized_resource: self.canonicalized_resource,
            signed_permissions: Some(permissions),
            signed_resource: self.signed_resource,
            signed_expiry: self.signed_expiry,
            signed_start: self.signed_start,
            signed_identifier: self.signed_identifier,
            signed_protocol: self.signed_protocol,
            signed_ip: self.signed_ip,
        }
    }
}

impl<T1, T2> BlobSharedAccessSignatureBuilder<T1, (), T2> {
    #[must_use]
    pub fn with_resources(
        self,
        signed_resource: BlobSignedResource,
    ) -> BlobSharedAccessSignatureBuilder<T1, SetResources, T2> {
        BlobSharedAccessSignatureBuilder {
            _phantom: PhantomData,
            key: self.key,
            canonicalized_resource: self.canonicalized_resource,
            signed_permissions: self.signed_permissions,
            signed_resource: Some(signed_resource),
            signed_expiry: self.signed_expiry,
            signed_start: self.signed_start,
            signed_identifier: self.signed_identifier,
            signed_protocol: self.signed_protocol,
            signed_ip: self.signed_ip,
        }
    }
}

impl<T1, T2> BlobSharedAccessSignatureBuilder<T1, T2, ()> {
    #[must_use]
    pub fn with_expiry(
        self,
        signed_expiry: DateTime<Utc>,
    ) -> BlobSharedAccessSignatureBuilder<T1, T2, SetExpiry> {
        BlobSharedAccessSignatureBuilder {
            _phantom: PhantomData,
            key: self.key,
            canonicalized_resource: self.canonicalized_resource,
            signed_permissions: self.signed_permissions,
            signed_resource: self.signed_resource,
            signed_expiry: Some(signed_expiry),
            signed_start: self.signed_start,
            signed_identifier: self.signed_identifier,
            signed_protocol: self.signed_protocol,
            signed_ip: self.signed_ip,
        }
    }
}
