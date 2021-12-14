//! The Azure Core prelude.

pub use crate::etag::Etag;
pub use crate::pageable::*;
pub use crate::policies::CustomHeaders;
pub use crate::request_options::*;
pub use crate::{
    new_http_client, AddAsHeader, AppendToUrlQuery, Context, HttpClient, RequestId, SessionToken,
    EMPTY_BODY,
};
