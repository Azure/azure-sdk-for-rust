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
