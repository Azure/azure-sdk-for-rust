mod mock_request;
mod mock_response;
mod mock_transaction;
mod player_policy;
mod recorder_policy;

use mock_transaction::MockTransaction;
use player_policy::MockTransportPlayerPolicy;
use recorder_policy::MockTransportRecorderPolicy;
use std::sync::Arc;

pub const TESTING_MODE_KEY: &str = "TESTING_MODE";
pub const TESTING_MODE_REPLAY: &str = "REPLAY";
pub const TESTING_MODE_RECORD: &str = "RECORD";

/// An error relating to the mock transport framework.
#[cfg(feature = "mock_transport_framework")]
#[derive(Debug, thiserror::Error)]
pub(crate) enum MockFrameworkError {
    #[error("{0}: {1}")]
    IOError(String, std::io::Error),
    #[error("{0}")]
    TransactionStorageError(String),
    #[error("{0}")]
    MissingTransaction(String),
    #[error("mismatched request uri. Actual '{0}', Expected: '{1}'")]
    MismatchedRequestUri(String, String),
    #[error("received request have header {0} but it was not present in the read request")]
    MissingRequestHeader(String),
    #[error("different number of headers in request. Actual: {0}, Expected: {1}")]
    MismatchedRequestHeadersCount(usize, usize),
    #[error("request header {0} value is different. Actual: {1}, Expected: {2}")]
    MismatchedRequestHeader(String, String, String),
    #[error("mismatched HTTP request method. Actual: {0}, Expected: {1}")]
    MismatchedRequestHTTPMethod(http::Method, http::Method),
    #[error("mismatched request body. Actual: {0:?}, Expected: {1:?}")]
    MismatchedRequestBody(Vec<u8>, Vec<u8>),
}

// Replace the default transport policy at runtime
//
// Replacement happens if these two conditions are met:
// 1. The mock_transport_framework is enabled
// 2. The environmental variable TESTING_MODE is either RECORD or PLAY
pub(crate) fn set_mock_transport_policy(
    policy: &mut std::sync::Arc<dyn crate::Policy>,
    transport_options: crate::TransportOptions,
) {
    match std::env::var(TESTING_MODE_KEY)
        .as_deref()
        .unwrap_or(TESTING_MODE_REPLAY)
    {
        TESTING_MODE_RECORD => {
            log::warn!("mock testing framework record mode enabled");
            *policy = Arc::new(MockTransportRecorderPolicy::new(transport_options))
        }
        TESTING_MODE_REPLAY => {
            log::info!("mock testing framework replay mode enabled");
            *policy = Arc::new(MockTransportPlayerPolicy::new(transport_options))
        }
        m => {
            log::error!(
                "invalid TESTING_MODE '{}' selected. Supported options are '{}' and '{}'",
                m,
                TESTING_MODE_RECORD,
                TESTING_MODE_REPLAY
            );
        }
    };
}
