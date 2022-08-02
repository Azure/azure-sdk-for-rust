use azure_core::{
    date,
    error::{Error, ErrorKind, ResultExt},
    headers::{self, AsHeaders, Headers},
};
pub mod operations;

use azure_core::{
    headers::{
        BLOB_PUBLIC_ACCESS, HAS_IMMUTABILITY_POLICY, HAS_LEGAL_HOLD, LEASE_DURATION, LEASE_STATE,
        LEASE_STATUS, META_PREFIX,
    },
    LeaseDuration, LeaseState, LeaseStatus,
};
use azure_storage::parsing_xml::{cast_must, cast_optional, traverse};
use std::collections::HashMap;
use time::OffsetDateTime;
use xml::{Element, Xml};

create_enum!(
    PublicAccess,
    (None, "none"),
    (Container, "container"),
    (Blob, "blob")
);

impl AsHeaders for PublicAccess {
    type Iter = std::option::IntoIter<(headers::HeaderName, headers::HeaderValue)>;

    fn as_headers(&self) -> Self::Iter {
        match self {
            PublicAccess::Blob => Some((BLOB_PUBLIC_ACCESS, "blob".into())).into_iter(),
            PublicAccess::Container => Some((BLOB_PUBLIC_ACCESS, "container".into())).into_iter(),
            PublicAccess::None => None.into_iter(),
        }
    }
}

pub(crate) fn public_access_from_header(header_map: &Headers) -> azure_core::Result<PublicAccess> {
    match header_map.get_optional_as(&BLOB_PUBLIC_ACCESS)? {
        Some(p) => Ok(p),
        None => Ok(PublicAccess::None),
    }
}

#[derive(Debug, Clone)]
pub struct Container {
    pub name: String,
    pub last_modified: OffsetDateTime,
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
            last_modified: OffsetDateTime::now_utc(),
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

    pub(crate) fn from_response<NAME>(
        name: NAME,
        headers: &Headers,
    ) -> azure_core::Result<Container>
    where
        NAME: Into<String>,
    {
        let last_modified = headers.get_str(&headers::LAST_MODIFIED)?;
        let last_modified = date::parse_rfc1123(last_modified)?;

        let e_tag = headers.get_as(&headers::ETAG)?;

        let lease_status = headers.get_as(&LEASE_STATUS)?;
        let lease_state = headers.get_as(&LEASE_STATE)?;

        let lease_duration = headers.get_optional_as(&LEASE_DURATION)?;

        let public_access = public_access_from_header(headers)?;

        let has_immutability_policy = headers.get_as(&HAS_IMMUTABILITY_POLICY)?;
        let has_legal_hold = headers.get_as(&HAS_LEGAL_HOLD)?;

        let mut metadata: HashMap<String, String> = HashMap::new();
        for (key, value) in headers.iter() {
            if key.as_str().starts_with(META_PREFIX.as_str()) {
                metadata.insert(key.as_str().to_owned(), value.as_str().to_owned());
            }
        }

        Ok(Container {
            name: name.into(),
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

    pub(crate) fn parse(elem: &Element) -> azure_core::Result<Container> {
        let name = cast_must(elem, &["Name"]).map_kind(ErrorKind::DataConversion)?;
        let last_modified = cast_must(elem, &["Properties", "Last-Modified"])?;
        let e_tag = cast_must(elem, &["Properties", "Etag"])?;
        let lease_state = cast_must(elem, &["Properties", "LeaseState"])?;
        let lease_duration = cast_optional(elem, &["Properties", "LeaseDuration"])?;
        let lease_status = cast_must(elem, &["Properties", "LeaseStatus"])?;
        let public_access =
            cast_optional(elem, &["Properties", "PublicAccess"])?.unwrap_or(PublicAccess::None);
        let has_immutability_policy = cast_must(elem, &["Properties", "HasImmutabilityPolicy"])?;
        let has_legal_hold = cast_must(elem, &["Properties", "HasLegalHold"])?;
        let metadata = {
            let mut hm = HashMap::new();
            let metadata = traverse(elem, &["Metadata"], true)?;

            for m in metadata {
                for key in &m.children {
                    let elem = match key {
                        Xml::ElementNode(elem) => elem,
                        _ => {
                            return Err(Error::message(
                                ErrorKind::DataConversion,
                                "Metadata should contain an ElementNode",
                            ));
                        }
                    };

                    if elem.children.is_empty() {
                        return Err(Error::message(
                            ErrorKind::DataConversion,
                            "Metadata node should not be empty",
                        ));
                    }

                    let key = elem.name.clone();

                    let content = {
                        match &elem.children[0] {
                            Xml::CharacterNode(content) => content.clone(),
                            _ => {
                                return Err(Error::message(ErrorKind::DataConversion,
                                    "Metadata node should contain a CharacterNode with metadata value",
                                ));
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
