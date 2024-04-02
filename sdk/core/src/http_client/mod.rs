#[cfg(not(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls")))]
mod noop;
#[cfg(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls"))]
mod reqwest;

#[cfg(not(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls")))]
use self::noop::new_noop_client;
#[cfg(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls"))]
use self::reqwest::new_reqwest_client;
use crate::error::ErrorKind;
use async_trait::async_trait;
use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;

/// Construct a new `HttpClient`
pub fn new_http_client() -> Arc<dyn HttpClient> {
    #[cfg(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls"))]
    {
        new_reqwest_client()
    }
    #[cfg(not(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls")))]
    {
        new_noop_client()
    }
}

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
        let body = body.collect().await?;

        if status.is_success() {
            Ok(crate::CollectedResponse::new(status, headers, body))
        } else {
            Err(ErrorKind::http_response_from_parts(status, &headers, &body).into_error())
        }
    }
}

/// Serialize a type to json.
pub fn to_json<T>(value: &T) -> crate::Result<Bytes>
where
    T: ?Sized + Serialize,
{
    Ok(Bytes::from(serde_json::to_vec(value)?))
}

/// Reads the XML from bytes.
pub fn from_json<S, T>(body: S) -> crate::Result<T>
where
    S: AsRef<[u8]>,
    T: DeserializeOwned,
{
    serde_json::from_slice(body.as_ref()).map_err(Into::into)
}
