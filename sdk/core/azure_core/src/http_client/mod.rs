#[cfg(not(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls")))]
mod noop;
#[cfg(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls"))]
mod reqwest;

#[cfg(not(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls")))]
use self::noop::new_noop_client;
#[cfg(any(feature = "enable_reqwest", feature = "enable_reqwest_rustls"))]
use self::reqwest::new_reqwest_client;

use async_trait::async_trait;
use std::sync::Arc;

/// Create a new [`HttpClient`].
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
    async fn execute_request(&self, request: &crate::Request) -> crate::Result<crate::Response<crate::ResponseBody>>;
}
