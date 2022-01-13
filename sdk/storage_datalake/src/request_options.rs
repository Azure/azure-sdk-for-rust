//! Request properties used in datalake rest api operations
use azure_core::AddAsHeader;
use azure_core::AppendToUrlQuery;
use http::request::Builder;

#[derive(Debug, Clone)]
pub enum ResourceType {
    FileSystem,
    Directory,
    File,
}

impl AppendToUrlQuery for ResourceType {
    fn append_to_url_query(&self, url: &mut url::Url) {
        let resource = match self {
            Self::File => "file",
            Self::Directory => "directory",
            Self::FileSystem => "filesystem",
        };
        url.query_pairs_mut().append_pair("resource", resource);
    }
}

#[derive(Debug, Clone)]
pub enum PathRenameMode {
    Legacy,
    Posix,
}

impl AppendToUrlQuery for PathRenameMode {
    fn append_to_url_query(&self, url: &mut url::Url) {
        let mode = match self {
            Self::Legacy => "legacy",
            Self::Posix => "posix",
        };
        url.query_pairs_mut().append_pair("mode", mode);
    }
}

#[derive(Debug, Clone)]
pub enum PathUpdateAction {
    Append,
    Flush,
    SetAccessControl,
    SetAccessControlRecursive,
    SetProperties,
}

impl AppendToUrlQuery for PathUpdateAction {
    fn append_to_url_query(&self, url: &mut url::Url) {
        let action = match self {
            Self::Append => "append",
            Self::Flush => "flush",
            Self::SetAccessControl => "setAccessControl",
            Self::SetAccessControlRecursive => "setAccessControlRecursive",
            Self::SetProperties => "setProperties",
        };
        url.query_pairs_mut().append_pair("action", action);
    }
}

#[derive(Debug, Clone)]
pub struct Position(i64);

impl Position {
    pub fn new(position: i64) -> Self {
        Self(position)
    }
}

impl From<i64> for Position {
    fn from(position: i64) -> Self {
        Self::new(position)
    }
}

#[derive(Debug, Clone)]
pub struct RenameSource(String);

impl<S> From<S> for RenameSource
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl AddAsHeader for RenameSource {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(
            http::header::CONTENT_LANGUAGE,
            http::HeaderValue::from_str(&self.0).unwrap(),
        )
    }

    fn add_as_header2(
        &self,
        request: &mut azure_core::Request,
    ) -> Result<(), azure_core::HttpHeaderError> {
        request.headers_mut().append(
            http::header::CONTENT_LANGUAGE,
            http::HeaderValue::from_str(&self.0)?,
        );

        Ok(())
    }
}
