pub mod requests;
pub mod responses;

use std::convert::TryFrom;

use azure_core::errors::AzureError;
use azure_core::headers::{CONTINUATION, PROPERTIES};
use azure_core::incompletevector::IncompleteVector;
use azure_core::parsing::{cast_must, traverse};
use azure_core::util::HeaderMapExt;
use http::{request::Builder, HeaderMap};
use xml::Element;

pub trait FilesystemRequired<'a> {
    fn filesystem(&self) -> &'a str;
}

pub trait FilesystemSupport<'a> {
    type O;
    fn with_filesystem(self, filesystem: &'a str) -> Self::O;
}

pub trait PropertiesOption<'a> {
    fn properties(&self) -> Option<&'a str>;

    #[must_use]
    fn add_header(&self, mut builder: Builder) -> Builder {
        if let Some(properties) = self.properties() {
            builder = builder.header(PROPERTIES, properties);
        }
        builder
    }
}

pub trait PropertiesSupport<'a> {
    type O;
    fn with_properties(self, properties: &'a str) -> Self::O;
}

pub(crate) fn properties_from_headers(headers: &HeaderMap) -> Result<String, AzureError> {
    let properties = headers
        .get_as_str(azure_core::headers::PROPERTIES)
        .ok_or_else(|| AzureError::HeaderNotFound(azure_core::headers::PROPERTIES.to_owned()))?;
    Ok(properties.to_owned())
}

pub(crate) fn namespace_enabled_from_headers(headers: &HeaderMap) -> Result<bool, AzureError> {
    let namespace_enabled = headers
        .get(azure_core::headers::NAMESPACE_ENABLED)
        .ok_or_else(|| {
            AzureError::HeaderNotFound(azure_core::headers::NAMESPACE_ENABLED.to_owned())
        })?
        .to_str()?;

    let namespace_enabled = namespace_enabled.parse::<bool>()?;
    Ok(namespace_enabled)
}

#[derive(Debug, Clone)]
pub struct Filesystem {
    etag: String,
    last_modified: String,
    name: String,
}

impl TryFrom<&Element> for Filesystem {
    type Error = AzureError;

    fn try_from(elem: &Element) -> Result<Filesystem, Self::Error> {
        let etag = cast_must::<String>(elem, &["eTag"])?;
        let last_modified = cast_must::<String>(elem, &["lastModified"])?;
        let name = cast_must::<String>(elem, &["Name"])?;

        Ok(Filesystem {
            etag,
            last_modified,
            name,
        })
    }
}

#[inline]
pub(crate) fn incomplete_vector_from_response(
    headers: &HeaderMap,
    body: &str,
) -> Result<IncompleteVector<Filesystem>, AzureError> {
    trace!("body = {}", body);

    let elem: Element = body.parse()?;

    let continuation = match headers.get_as_string(CONTINUATION) {
        Some(ref ct) if ct == "" => None,
        Some(ct) => Some(ct),
        None => None,
    };

    debug!("continuation == {:?}", continuation);

    let mut v = Vec::new();
    for node_filesystem in traverse(&elem, &["Filesystems", "Filesystem"], true)? {
        v.push(Filesystem::try_from(node_filesystem)?);
    }

    Ok(IncompleteVector::<Filesystem>::new(continuation, v))
}
