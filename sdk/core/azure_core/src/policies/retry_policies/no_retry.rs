use crate::error::{Error, ErrorKind, HttpError};
use crate::policies::{Policy, PolicyResult, Request};
use crate::Context;
use std::sync::Arc;

/// Retry policy that does not retry.
///
/// Use this policy as a stub to disable retry policies altogether.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct NoRetryPolicy {
    _priv: std::marker::PhantomData<u32>,
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl Policy for NoRetryPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        // just call the following policies and bubble up the error
        let response = next[0].send(ctx, request, &next[1..]).await?;

        if response.status().is_success() {
            Ok(response)
        } else {
            let status = response.status();
            let http_error = HttpError::new(response).await;

            let error_kind = ErrorKind::http_response(
                status,
                http_error.error_code().map(std::borrow::ToOwned::to_owned),
            );
            let error = Error::full(
                error_kind,
                http_error,
                format!("server returned error status which will not be retried: {status}"),
            );
            return Err(error);
        }
    }
}
