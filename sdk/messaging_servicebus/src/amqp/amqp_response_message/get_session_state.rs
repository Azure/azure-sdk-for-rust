use fe2o3_amqp_management::{error::InvalidType, response::Response};
use fe2o3_amqp_types::primitives::{Array, OrderedMap};

use crate::amqp::management_constants::properties::SESSION_STATE;

type GetSessionStateResponseBody = OrderedMap<String, Array<u8>>;

pub(crate) struct GetSessionStateResponse {
    pub session_state: Array<u8>,
}

impl GetSessionStateResponse {
    pub fn session_state(&self) -> &[u8] {
        &self.session_state
    }

    pub fn into_session_state(self) -> Array<u8> {
        self.session_state
    }
}

impl Response for GetSessionStateResponse {
    const STATUS_CODE: u16 = 200;

    type Body = GetSessionStateResponseBody;

    type Error = super::MgmtError;

    fn decode_message(
        mut message: fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<Self, Self::Error> {
        let session_state = message
            .body
            .remove(SESSION_STATE)
            .ok_or_else(|| InvalidType {
                expected: SESSION_STATE.to_string(),
                actual: "None".to_string(),
            })?;

        Ok(Self { session_state })
    }
}
