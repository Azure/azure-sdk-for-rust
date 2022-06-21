use azure_core::{
    error::{Error, ErrorKind, ResultExt},
    headers::{self, AsHeaders, Headers},
};
pub mod requests;
pub mod responses;

use super::IncompleteVector;
use azure_core::{
    headers::{
        BLOB_PUBLIC_ACCESS, HAS_IMMUTABILITY_POLICY, HAS_LEGAL_HOLD, LEASE_DURATION, LEASE_STATE,
        LEASE_STATUS, META_PREFIX,
    },
    LeaseDuration, LeaseState, LeaseStatus,
};
use azure_storage::parsing_xml::{cast_must, cast_optional, traverse};
use chrono::{DateTime, Utc};
use http::header;
use std::{collections::HashMap, str::FromStr};
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
            PublicAccess::Blob => Some((BLOB_PUBLIC_ACCESS.into(), "blob".into())).into_iter(),
            PublicAccess::Container => {
                Some((BLOB_PUBLIC_ACCESS.into(), "container".into())).into_iter()
            }
            PublicAccess::None => None.into_iter(),
        }
    }
}

pub(crate) fn public_access_from_header(header_map: &Headers) -> azure_core::Result<PublicAccess> {
    let pa = match header_map.get(BLOB_PUBLIC_ACCESS) {
        Some(pa) => PublicAccess::from_str(pa.as_str()).map_kind(ErrorKind::DataConversion)?,
        None => PublicAccess::None,
    };
    Ok(pa)
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

    pub(crate) fn from_response<NAME>(
        name: NAME,
        headers: &Headers,
    ) -> azure_core::Result<Container>
    where
        NAME: Into<String>,
    {
        let last_modified = headers.get_as_str_or_err(header::LAST_MODIFIED)?;
        let last_modified =
            DateTime::parse_from_rfc2822(last_modified).map_kind(ErrorKind::DataConversion)?;
        let last_modified = DateTime::from_utc(last_modified.naive_utc(), Utc);

        let e_tag = headers.get_as_string_or_err(header::ETAG)?;

        let lease_status = headers.get_as_str_or_err(LEASE_STATUS)?;
        let lease_status =
            LeaseStatus::from_str(lease_status).map_kind(ErrorKind::DataConversion)?;

        let lease_state = headers.get_as_str_or_err(LEASE_STATE)?;
        let lease_state = LeaseState::from_str(lease_state).map_kind(ErrorKind::DataConversion)?;

        let lease_duration = headers.get_as_enum(LEASE_DURATION)?;

        let public_access = public_access_from_header(headers)?;

        let has_immutability_policy =
            bool::from_str(headers.get_as_str_or_err(HAS_IMMUTABILITY_POLICY)?)
                .map_kind(ErrorKind::DataConversion)?;

        let has_legal_hold = bool::from_str(headers.get_as_str_or_err(HAS_LEGAL_HOLD)?)
            .map_kind(ErrorKind::DataConversion)?;

        let mut metadata: HashMap<String, String> = HashMap::new();
        for (key, value) in headers.iter() {
            if key.as_str().starts_with(META_PREFIX) {
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

    fn parse(elem: &Element) -> azure_core::Result<Container> {
        let name = cast_must::<String>(elem, &["Name"]).map_kind(ErrorKind::DataConversion)?;
        let last_modified = cast_must::<DateTime<Utc>>(elem, &["Properties", "Last-Modified"])
            .map_kind(ErrorKind::DataConversion)?;
        let e_tag = cast_must::<String>(elem, &["Properties", "Etag"])
            .map_kind(ErrorKind::DataConversion)?;

        let lease_state = cast_must::<LeaseState>(elem, &["Properties", "LeaseState"])
            .map_kind(ErrorKind::DataConversion)?;

        let lease_duration = cast_optional::<LeaseDuration>(elem, &["Properties", "LeaseDuration"])
            .map_kind(ErrorKind::DataConversion)?;

        let lease_status = cast_must::<LeaseStatus>(elem, &["Properties", "LeaseStatus"])
            .map_kind(ErrorKind::DataConversion)?;

        let public_access =
            match cast_optional::<PublicAccess>(elem, &["Properties", "PublicAccess"])
                .map_kind(ErrorKind::DataConversion)?
            {
                Some(pa) => pa,
                None => PublicAccess::None,
            };

        let has_immutability_policy =
            cast_must::<bool>(elem, &["Properties", "HasImmutabilityPolicy"])
                .map_kind(ErrorKind::DataConversion)?;
        let has_legal_hold = cast_must::<bool>(elem, &["Properties", "HasLegalHold"])
            .map_kind(ErrorKind::DataConversion)?;

        let metadata = {
            let mut hm = HashMap::new();
            let metadata =
                traverse(elem, &["Metadata"], true).map_kind(ErrorKind::DataConversion)?;

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

                    let key = elem.name.to_owned();

                    if elem.children.is_empty() {
                        return Err(Error::message(
                            ErrorKind::DataConversion,
                            "Metadata node should not be empty",
                        ));
                    }

                    let content = {
                        match elem.children[0] {
                            Xml::CharacterNode(ref content) => content.to_owned(),
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

pub(crate) fn incomplete_vector_from_container_response(
    body: &str,
) -> azure_core::Result<IncompleteVector<Container>> {
    let elem: Element = body.parse().map_kind(ErrorKind::Other)?;

    let mut v = Vec::new();

    for container in
        traverse(&elem, &["Containers", "Container"], true).map_kind(ErrorKind::Other)?
    {
        v.push(Container::parse(container)?);
    }

    let next_marker =
        match cast_optional::<String>(&elem, &["NextMarker"]).map_kind(ErrorKind::Other)? {
            Some(ref nm) if nm.is_empty() => None,
            Some(nm) => Some(nm.into()),
            None => None,
        };

    Ok(IncompleteVector::new(next_marker, v))
}
