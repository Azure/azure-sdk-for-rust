#[cfg(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls"))]
#[cfg(not(target_arch = "wasm32"))]
mod reqwest;

#[cfg(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls"))]
#[cfg(not(target_arch = "wasm32"))]
pub use self::reqwest::*;
#[allow(unused_imports)]
use crate::error::{Error, ErrorKind, ResultExt};
#[allow(unused_imports)]
use crate::Body;
#[allow(unused_imports)]
use crate::{headers::Headers, PinnedStream};
use async_trait::async_trait;
use bytes::Bytes;
#[allow(unused_imports)]
use futures::TryStreamExt;
use serde::Serialize;

/// An HTTP client which can send requests.
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait HttpClient: Send + Sync + std::fmt::Debug {
    /// Send out a request using `azure_core`'s types.
    ///
    /// It does not consume the request. Implementors are expected to clone the necessary parts
    /// of the request and pass them to the underlying transport.
    async fn execute_request(&self, request: &crate::Request) -> crate::Result<crate::Response>;

    /// DEPRECATED: the status check will be responsibility of another policy (not the transport one).
    /// Send out the request and collect the response body.
    /// An error is returned if the status is not success.
    async fn execute_request_check_status(
        &self,
        request: &crate::Request,
    ) -> crate::Result<crate::CollectedResponse> {
        let rsp = self.execute_request(request).await?;
        let (status, headers, body) = rsp.deconstruct();
        let body = crate::collect_pinned_stream(body).await?;
        let status_u16 = status as u16;
        if !(200..400).contains(&status_u16) {
            return Err(ErrorKind::http_response_from_body(status_u16, &body).into_error());
        }
        Ok(crate::CollectedResponse::new(status, headers, body))
    }
}

/// Serialize a type to json.
pub fn to_json<T>(value: &T) -> crate::Result<Bytes>
where
    T: ?Sized + Serialize,
{
    Ok(Bytes::from(serde_json::to_vec(value)?))
}
