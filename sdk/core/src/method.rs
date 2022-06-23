use crate::error::{Error, ErrorKind};
use std::{ops::Deref, str::FromStr};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Method(pub(crate) http_types::Method);

impl Method {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl Deref for Method {
    type Target = http_types::Method;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Method {
    pub const CONNECT: Method = Method(http_types::Method::Connect);
    pub const DELETE: Method = Method(http_types::Method::Delete);
    pub const GET: Method = Method(http_types::Method::Get);
    pub const HEAD: Method = Method(http_types::Method::Head);
    pub const MERGE: Method = Method(http_types::Method::Merge);
    pub const OPTIONS: Method = Method(http_types::Method::Options);
    pub const PATCH: Method = Method(http_types::Method::Patch);
    pub const POST: Method = Method(http_types::Method::Post);
    pub const PUT: Method = Method(http_types::Method::Put);
    pub const TRACE: Method = Method(http_types::Method::Trace);
}

impl Default for Method {
    fn default() -> Method {
        Method::GET
    }
}

impl FromStr for Method {
    type Err = Error;
    fn from_str(s: &str) -> crate::Result<Self> {
        Ok(Method(http_types::Method::from_str(s).map_err(
            |error| Error::full(ErrorKind::DataConversion, error, "Method::from_str failed"),
        )?))
    }
}
