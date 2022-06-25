//! Request properties used in datalake rest api operations
use azure_core::AppendToUrlQuery;
use azure_core::Header;
use azure_storage::core::headers;

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
pub enum PathGetPropertiesAction {
    CheckAccess,
    GetAccessControl,
    GetStatus,
}

impl AppendToUrlQuery for PathGetPropertiesAction {
    fn append_to_url_query(&self, url: &mut url::Url) {
        let action = match self {
            Self::CheckAccess => "checkAccess",
            Self::GetAccessControl => "getAccessControl",
            Self::GetStatus => "getStatus",
        };
        url.query_pairs_mut().append_pair("action", action);
    }
}

#[derive(Debug, Clone)]
pub struct Recursive(bool);

impl AppendToUrlQuery for Recursive {
    fn append_to_url_query(&self, url: &mut url::Url) {
        let recursive = if self.0 { "true" } else { "false" };
        url.query_pairs_mut().append_pair("recursive", recursive);
    }
}

impl From<bool> for Recursive {
    fn from(recursive: bool) -> Self {
        Self(recursive)
    }
}

#[derive(Debug, Clone)]
pub struct Upn(bool);

impl AppendToUrlQuery for Upn {
    fn append_to_url_query(&self, url: &mut url::Url) {
        let upn = if self.0 { "true" } else { "false" };
        url.query_pairs_mut().append_pair("upn", upn);
    }
}

impl From<bool> for Upn {
    fn from(upn: bool) -> Self {
        Self(upn)
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

impl AppendToUrlQuery for Position {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("position", &self.0.to_string());
    }
}

#[derive(Debug, Clone)]
pub struct Close(bool);

impl Close {
    pub fn new(close: bool) -> Self {
        Self(close)
    }
}

impl From<bool> for Close {
    fn from(position: bool) -> Self {
        Self::new(position)
    }
}

impl AppendToUrlQuery for Close {
    fn append_to_url_query(&self, url: &mut url::Url) {
        let close = if self.0 { "true" } else { "false" };
        url.query_pairs_mut().append_pair("close", close);
    }
}

#[derive(Debug, Clone)]
pub struct RetainUncommittedData(bool);

impl RetainUncommittedData {
    pub fn new(close: bool) -> Self {
        Self(close)
    }
}

impl From<bool> for RetainUncommittedData {
    fn from(position: bool) -> Self {
        Self::new(position)
    }
}

impl AppendToUrlQuery for RetainUncommittedData {
    fn append_to_url_query(&self, url: &mut url::Url) {
        let retain = if self.0 { "true" } else { "false" };
        url.query_pairs_mut()
            .append_pair("retainUncommittedData", retain);
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

impl Header for RenameSource {
    fn name(&self) -> azure_core::headers::HeaderName {
        headers::RENAME_SOURCE
    }

    fn value(&self) -> azure_core::headers::HeaderValue {
        url::form_urlencoded::byte_serialize(self.0.as_bytes())
            .collect::<String>()
            .into()
    }
}

#[derive(Debug, Clone)]
pub struct Directory(String);

impl<S> From<S> for Directory
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl AppendToUrlQuery for Directory {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut().append_pair("directory", &self.0);
    }
}

#[derive(Debug, Clone)]
pub struct AccessControlList(String);

impl<S> From<S> for AccessControlList
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        Self(s.into())
    }
}

impl Header for AccessControlList {
    fn name(&self) -> azure_core::headers::HeaderName {
        azure_core::headers::ACL
    }

    fn value(&self) -> azure_core::headers::HeaderValue {
        self.0.to_owned().into()
    }
}
