mod mock_request;
mod mock_response;
mod mock_transaction;
mod player_policy;
mod recorder_policy;

use mock_transaction::MockTransaction;
use player_policy::MockTransportPlayerPolicy;
use recorder_policy::MockTransportRecorderPolicy;
use std::sync::Arc;

use azure_core::{HttpClient, Policy};

pub const TESTING_MODE_KEY: &str = "TESTING_MODE";
pub const TESTING_MODE_REPLAY: &str = "REPLAY";
pub const TESTING_MODE_RECORD: &str = "RECORD";

/// Create a new mock transport policy.
///
/// Returns a reply mock policy unless the environment variable  "TESTING_MODE" is set to "RECORD".
pub fn new_mock_transport(transaction_name: String) -> Arc<dyn Policy> {
    match std::env::var(TESTING_MODE_KEY)
        .as_deref()
        .unwrap_or(TESTING_MODE_REPLAY)
    {
        TESTING_MODE_RECORD => {
            log::warn!("mock testing framework record mode enabled");
            new_recorder_transport(transaction_name, azure_core::new_http_client())
        }
        _ => {
            log::info!("mock testing framework replay mode enabled");
            new_replay_transport(transaction_name)
        }
    }
}

/// Create a mock transport policy that replays recorded mock requests/responses.
pub fn new_replay_transport(transaction_name: String) -> Arc<dyn Policy> {
    Arc::new(MockTransportPlayerPolicy::new(transaction_name))
}

/// Create a mock transport policy that records live calls.
pub fn new_recorder_transport(
    transaction_name: String,
    http_client: Arc<dyn HttpClient>,
) -> Arc<dyn Policy> {
    Arc::new(MockTransportRecorderPolicy::new(
        transaction_name,
        http_client,
    ))
}
