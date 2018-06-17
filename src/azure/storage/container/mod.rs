pub mod requests;
use azure::core::{
    enumerations,
    errors::{AzureError, TraversingError},
    headers::BLOB_PUBLIC_ACCESS,
    lease::{LeaseDuration, LeaseState, LeaseStatus},
    parsing::{cast_must, cast_optional, traverse, FromStringOptional},
};
use chrono::{DateTime, Utc};
use http::request::Builder;
use http::HeaderMap;
use std::collections::HashMap;
use std::{fmt, str::FromStr};
use xml::{Element, Xml};

create_enum!(PublicAccess, (None, "none"), (Container, "container"), (Blob, "blob"));

pub(crate) fn public_access_from_header(header_map: &HeaderMap) -> Result<PublicAccess, AzureError> {
    let pa = match header_map.get(BLOB_PUBLIC_ACCESS) {
        Some(pa) => PublicAccess::from_str(pa.to_str()?)?,
        None => PublicAccess::None,
    };
    Ok(pa)
}

pub trait PublicAccessSupport {
    type O;
    fn with_public_access(self, pa: PublicAccess) -> Self::O;
}

pub trait PublicAccessRequired {
    fn public_access(&self) -> PublicAccess;

    fn add_header(&self, builder: &mut Builder) {
        if self.public_access() != PublicAccess::None {
            builder.header(BLOB_PUBLIC_ACCESS, self.public_access().as_ref());
        }
    }
}

#[derive(Debug, Clone)]
pub struct Container {
    pub name: String,
    pub last_modified: DateTime<Utc>,
    pub e_tag: String,
    pub lease_status: LeaseStatus,
    pub lease_state: LeaseState,
    pub lease_duration: Option<LeaseDuration>,
    pub public_access: PublicAccess,
    pub has_immutability_policy: bool,
    pub has_legal_hold: bool,
    pub metadata: HashMap<String, String>,
}

impl AsRef<str> for Container {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl Container {
    pub fn new(name: &str) -> Container {
        Container {
            name: name.to_owned(),
            last_modified: Utc::now(),
            e_tag: "".to_owned(),
            lease_status: LeaseStatus::Unlocked,
            lease_state: LeaseState::Available,
            lease_duration: None,
            public_access: PublicAccess::None,
            has_immutability_policy: false,
            has_legal_hold: false,
            metadata: HashMap::new(),
        }
    }

    fn parse(elem: &Element) -> Result<Container, AzureError> {
        let name = cast_must::<String>(elem, &["Name"])?;
        let last_modified = cast_must::<DateTime<Utc>>(elem, &["Properties", "Last-Modified"])?;
        let e_tag = cast_must::<String>(elem, &["Properties", "Etag"])?;

        let lease_state = cast_must::<LeaseState>(elem, &["Properties", "LeaseState"])?;

        let lease_duration = cast_optional::<LeaseDuration>(elem, &["Properties", "LeaseDuration"])?;

        let lease_status = cast_must::<LeaseStatus>(elem, &["Properties", "LeaseStatus"])?;

        let public_access = match cast_optional::<PublicAccess>(elem, &["Properties", "PublicAccess"])? {
            Some(pa) => pa,
            None => PublicAccess::None,
        };

        let has_immutability_policy = cast_must::<bool>(elem, &["Properties", "HasImmutabilityPolicy"])?;
        let has_legal_hold = cast_must::<bool>(elem, &["Properties", "HasLegalHold"])?;

        let metadata = {
            let mut hm = HashMap::new();
            let metadata = traverse(elem, &["Metadata"], true)?;

            for m in metadata {
                for key in &m.children {
                    let elem = match key {
                        Xml::ElementNode(elem) => elem,
                        _ => {
                            return Err(AzureError::UnexpectedXMLError(String::from(
                                "Metadata should contain an ElementNode",
                            )))
                        }
                    };

                    let key = elem.name.to_owned();

                    if elem.children.is_empty() {
                        return Err(AzureError::UnexpectedXMLError(String::from("Metadata node should not be empty")));
                    }

                    let content = {
                        match elem.children[0] {
                            Xml::CharacterNode(ref content) => content.to_owned(),
                            _ => {
                                return Err(AzureError::UnexpectedXMLError(String::from(
                                    "Metadata node should contain a CharacterNode with metadata value",
                                )))
                            }
                        }
                    };

                    hm.insert(key, content);
                }
            }

            hm
        };

        Ok(Container {
            name,
            last_modified,
            e_tag,
            lease_status,
            lease_state,
            lease_duration,
            public_access,
            has_immutability_policy,
            has_legal_hold,
            metadata,
        })
    }
}
