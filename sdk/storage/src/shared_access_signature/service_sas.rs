use crate::shared_access_signature::{format_date, SasProtocol, SasToken};
use azure_core::{auth::Secret, date::iso8601, hmac::hmac_sha256};
use std::fmt;
use time::OffsetDateTime;
use url::form_urlencoded;
use uuid::Uuid;

const SERVICE_SAS_VERSION: &str = "2022-11-02";

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

#[allow(clippy::struct_excessive_bools)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct UserDeligationKey {
    pub signed_oid: Uuid,
    pub signed_tid: Uuid,
    #[serde(with = "iso8601")]
    pub signed_start: OffsetDateTime,
    #[serde(with = "iso8601")]
    pub signed_expiry: OffsetDateTime,
    pub signed_service: String,
    pub signed_version: String,
    pub value: Secret,
}

pub enum SasKey {
    Key(Secret),
    UserDelegationKey(UserDeligationKey),
}

impl From<Secret> for SasKey {
    fn from(key: Secret) -> Self {
        Self::Key(key)
    }
}

impl From<UserDeligationKey> for SasKey {
    fn from(key: UserDeligationKey) -> Self {
        Self::UserDelegationKey(key)
    }
}

pub struct BlobSharedAccessSignature {
    key: SasKey,
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
    pub fn new<K>(
        key: K,
        canonicalized_resource: String,
        permissions: BlobSasPermissions,
        expiry: OffsetDateTime,
        resource: BlobSignedResource,
    ) -> Self
    where
        K: Into<SasKey>,
    {
        Self {
            key: key.into(),
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

    fn sign(&self) -> azure_core::Result<String> {
        let mut content = vec![
            self.permissions.to_string(),
            self.start.map_or(String::new(), format_date),
            format_date(self.expiry),
            self.canonicalized_resource.clone(),
        ];

        let key = match &self.key {
            SasKey::Key(key) => {
                content.extend([self
                    .identifier
                    .as_ref()
                    .unwrap_or(&String::new())
                    .to_string()]);
                key
            }
            SasKey::UserDelegationKey(key) => {
                let user_delegated = [
                    key.signed_oid.to_string(),
                    key.signed_tid.to_string(),
                    format_date(key.signed_start),
                    format_date(key.signed_expiry),
                    key.signed_service.to_string(),
                    key.signed_version.to_string(),
                    String::new(), // SIGNED AUTHORIZED_OID
                    String::new(), // SIGNED UNAUTHORIZED_OID
                    String::new(), // SIGNED CORRELATION ID
                ];

                content.extend(user_delegated);
                &key.value
            }
        };

        content.extend([
            self.ip.as_ref().unwrap_or(&String::new()).to_string(),
            self.protocol.map(|x| x.to_string()).unwrap_or_default(),
            SERVICE_SAS_VERSION.to_string(),
            self.resource.to_string(),
            String::new(), // snapshot time
            String::new(), // SIGNED ENCRYPTION SCOPE
            String::new(), // SIGNED CACHE CONTROL
            String::new(), // SIGNED CONTENT DISPOSITION
            String::new(), // SIGNED CONTENT ENCODING
            String::new(), // SIGNED CONTENT LANGUAGE
            String::new(), // SIGNED CONTENT TYPE
        ]);

        hmac_sha256(&content.join("\n"), key)
    }
}

impl SasToken for BlobSharedAccessSignature {
    fn token(&self) -> azure_core::Result<String> {
        let mut form = form_urlencoded::Serializer::new(String::new());

        if let SasKey::UserDelegationKey(key) = &self.key {
            form.extend_pairs(&[
                ("skoid", &key.signed_oid.to_string()),
                ("sktid", &key.signed_tid.to_string()),
                ("skt", &format_date(key.signed_start)),
                ("ske", &format_date(key.signed_expiry)),
                ("sks", &key.signed_service),
                ("skv", &key.signed_version),
            ]);
        }

        form.extend_pairs(&[
            ("sv", SERVICE_SAS_VERSION),
            ("sp", &self.permissions.to_string()),
            ("sr", &self.resource.to_string()),
            ("se", &format_date(self.expiry)),
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

        if let Some(signed_directory_depth) = &self.signed_directory_depth {
            form.append_pair("sdd", &signed_directory_depth.to_string());
        }

        let sig = self.sign()?;
        form.append_pair("sig", &sig);
        Ok(form.finish())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use time::Duration;

    const MOCK_SECRET_KEY: &str = "RZfi3m1W7eyQ5zD4ymSmGANVdJ2SDQmg4sE89SW104s=";
    const MOCK_CANONICALIZED_RESOURCE: &str = "/blob/STORAGE_ACCOUNT_NAME/CONTAINER_NAME/";

    #[test]
    fn test_blob_scoped_sas_token() -> azure_core::Result<()> {
        let permissions = BlobSasPermissions {
            read: true,
            ..Default::default()
        };
        let signed_token = BlobSharedAccessSignature::new(
            Secret::new(MOCK_SECRET_KEY),
            String::from(MOCK_CANONICALIZED_RESOURCE),
            permissions,
            OffsetDateTime::UNIX_EPOCH + Duration::days(7),
            BlobSignedResource::Blob,
        )
        .token()?;

        assert_eq!(signed_token, "sv=2022-11-02&sp=r&sr=b&se=1970-01-08T00%3A00%3A00Z&sig=VRZjVZ1c%2FLz7IXCp17Sdx9%2BR9JDrnJdzE3NW56DMjNs%3D");

        let mut parsed = url::form_urlencoded::parse(&signed_token.as_bytes());

        // BlobSignedResource::Blob
        assert!(parsed.find(|(k, v)| k == "sr" && v == "b").is_some());

        // signed_directory_depth NOT set
        assert!(parsed.find(|(k, _)| k == "sdd").is_none());
        Ok(())
    }

    #[test]
    fn test_directory_scoped_sas_token() -> azure_core::Result<()> {
        let permissions = BlobSasPermissions {
            read: true,
            ..Default::default()
        };
        let signed_token = BlobSharedAccessSignature::new(
            Secret::new(MOCK_SECRET_KEY),
            String::from(MOCK_CANONICALIZED_RESOURCE),
            permissions,
            OffsetDateTime::UNIX_EPOCH + Duration::days(7),
            BlobSignedResource::Directory,
        )
        .signed_directory_depth(2_usize)
        .token()?;

        assert_eq!(signed_token, "sv=2022-11-02&sp=r&sr=d&se=1970-01-08T00%3A00%3A00Z&sdd=2&sig=zVN%2FRgDWllHZH6%2FqWt5gFrV89vzp4EU6ULDTdYoHils%3D");

        let mut parsed = url::form_urlencoded::parse(&signed_token.as_bytes());

        // BlobSignedResource::Directory
        assert!(parsed.find(|(k, v)| k == "sr" && v == "d").is_some());

        // signed_directory_depth set
        assert!(parsed.find(|(k, v)| k == "sdd" && v == "2").is_some());
        Ok(())
    }
}
