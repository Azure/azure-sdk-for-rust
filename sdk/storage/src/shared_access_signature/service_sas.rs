use crate::{
    hmac,
    shared_access_signature::{format_date, format_form, SasProtocol, SasToken},
};
use std::fmt;
use time::OffsetDateTime;

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
    pub read: bool,             // r - Container | Directory | Blob
    pub add: bool,              // a - Container | Directory | Blob
    pub create: bool,           // c - Container | Directory | Blob
    pub write: bool,            // w - Container | Directory | Blob
    pub delete: bool,           // d - Container | Directory | Blob
    pub delete_version: bool,   // x - Container | Blob
    pub permanent_delete: bool, // y - Blob
    pub list: bool,             // l - Container | Directory
    pub tags: bool,             // t - Tags
    pub move_: bool,            // m - Container | Directory | Blob
    pub execute: bool,          // e - Container | Directory | Blob
    pub ownership: bool,        // o - Container | Directory | Blob
    pub permissions: bool,      // p - Container | Directory | Blob
                                // SetImmunabilityPolicy: bool, // i  -- container
}

impl fmt::Display for BlobSasPermissions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.read {
            write!(f, "r")?;
        };
        if self.add {
            write!(f, "a")?;
        };
        if self.create {
            write!(f, "c")?;
        };
        if self.write {
            write!(f, "w")?;
        };
        if self.delete {
            write!(f, "d")?;
        };
        if self.delete_version {
            write!(f, "x")?;
        };
        if self.permanent_delete {
            write!(f, "y")?;
        };
        if self.list {
            write!(f, "l")?;
        };
        if self.tags {
            write!(f, "t")?;
        };
        if self.move_ {
            write!(f, "m")?;
        };
        if self.execute {
            write!(f, "e")?;
        };
        if self.ownership {
            write!(f, "o")?;
        };
        if self.permissions {
            write!(f, "p")?;
        };
        Ok(())
    }
}

pub struct BlobSharedAccessSignature {
    key: String,
    canonicalized_resource: String,
    resource: BlobSignedResource,
    permissions: BlobSasPermissions, // sp
    expiry: OffsetDateTime,          // se
    start: Option<OffsetDateTime>,   // st
    identifier: Option<String>,
    ip: Option<String>,
    protocol: Option<SasProtocol>,
    signed_directory_depth: Option<usize>, // sdd
}

impl BlobSharedAccessSignature {
    pub fn new(
        key: String,
        canonicalized_resource: String,
        permissions: BlobSasPermissions,
        expiry: OffsetDateTime,
        resource: BlobSignedResource,
    ) -> Self {
        Self {
            key,
            canonicalized_resource,
            resource,
            permissions,
            expiry,
            start: None,
            identifier: None,
            ip: None,
            protocol: None,
            signed_directory_depth: None,
        }
    }

    setters! {
        start: OffsetDateTime => Some(start),
        identifier: String => Some(identifier),
        ip: String => Some(ip),
        protocol: SasProtocol => Some(protocol),
        signed_directory_depth: usize => Some(signed_directory_depth),
    }

    fn sign(&self) -> String {
        let content = vec![
            self.permissions.to_string(),
            self.start.map_or("".to_string(), format_date),
            format_date(self.expiry),
            self.canonicalized_resource.clone(),
            self.identifier
                .as_ref()
                .unwrap_or(&"".to_string())
                .to_string(),
            self.ip.as_ref().unwrap_or(&"".to_string()).to_string(),
            self.protocol
                .map(|x| x.to_string())
                .unwrap_or_else(|| "".to_string()),
            SERVICE_SAS_VERSION.to_string(),
            self.resource.to_string(),
            "".to_string(), // snapshot time
            "".to_string(), // rscd
            "".to_string(), // rscc
            "".to_string(), // rsce
            "".to_string(), // rscl
            "".to_string(), // rsct
        ];

        hmac::sign(&content.join("\n"), &self.key).expect("HMAC signing failed")
    }
}

impl SasToken for BlobSharedAccessSignature {
    fn token(&self) -> String {
        let mut elements: Vec<String> = vec![
            format!("sv={SERVICE_SAS_VERSION}"),
            format!("sp={}", self.permissions),
            format!("sr={}", self.resource),
            format!("se={}", format_form(format_date(self.expiry))),
        ];

        if let Some(start) = &self.start {
            elements.push(format!("st={}", format_form(format_date(*start))));
        }

        if let Some(ip) = &self.ip {
            elements.push(format!("sip={ip}"));
        }

        if let Some(protocol) = &self.protocol {
            elements.push(format!("spr={protocol}"));
        }

        if let Some(signed_directory_depth) = &self.signed_directory_depth {
            elements.push(format!("sdd={signed_directory_depth}"));
        }

        let sig = self.sign();
        elements.push(format!("sig={}", format_form(sig)));

        elements.join("&")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;
    use time::Duration;

    const MOCK_SECRET_KEY: &str = "RZfi3m1W7eyQ5zD4ymSmGANVdJ2SDQmg4sE89SW104s=";
    const MOCK_CANONICALIZED_RESOURCE: &str = "/blob/STORAGE_ACCOUNT_NAME/CONTAINER_NAME/";

    #[test]
    fn test_blob_scoped_sas_token() {
        let permissions = BlobSasPermissions {
            read: true,
            ..Default::default()
        };
        let signed_token = BlobSharedAccessSignature::new(
            String::from(MOCK_SECRET_KEY),
            String::from(MOCK_CANONICALIZED_RESOURCE),
            permissions,
            OffsetDateTime::UNIX_EPOCH + Duration::days(7),
            BlobSignedResource::Blob,
        )
        .token();

        // splitting by & is only safe if & is not part of any fields
        // if that changes in the future we might want to use a proper query string parser
        let elements = signed_token.split('&').collect::<HashSet<_>>();

        // BlobSignedResource::Blob
        assert!(elements.contains("sr=b"));
        // signed_directory_depth NOT set
        assert!(!elements.iter().any(|element| element.starts_with("sdd=")));

        assert_eq!(signed_token, "sv=2020-06-12&sp=r&sr=b&se=1970-01-08T00%3A00%3A00Z&sig=alEGfKjtiLs5LO%2FyrfPkzjQBHbk4Uda9XOezbRyKwEM%3D");
    }

    #[test]
    fn test_directory_scoped_sas_token() {
        let permissions = BlobSasPermissions {
            read: true,
            ..Default::default()
        };
        let signed_token = BlobSharedAccessSignature::new(
            String::from(MOCK_SECRET_KEY),
            String::from(MOCK_CANONICALIZED_RESOURCE),
            permissions,
            OffsetDateTime::UNIX_EPOCH + Duration::days(7),
            BlobSignedResource::Directory,
        )
        .signed_directory_depth(2_usize)
        .token();

        // splitting by & is only safe if & is not part of any fields
        // if that changes in the future we might want to use a proper query string parser
        let elements = signed_token.split('&').collect::<HashSet<_>>();

        // BlobSignedResource::Blob
        assert!(elements.contains("sr=d"));
        // signed_directory_depth = 2
        assert!(elements.contains("sdd=2"));

        assert_eq!(signed_token, "sv=2020-06-12&sp=r&sr=d&se=1970-01-08T00%3A00%3A00Z&sdd=2&sig=e0eoY169%2Bex4AnI9ZAOiOaX49snoJiuvyJ22XV6qW2k%3D");
    }
}
