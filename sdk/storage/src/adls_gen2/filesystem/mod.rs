pub mod requests;
pub mod responses;

use azure_core::errors::AzureError;
use azure_core::headers::PROPERTIES;
use azure_core::util::HeaderMapExt;
use http::{request::Builder, HeaderMap};

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
