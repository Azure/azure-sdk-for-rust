use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::primitives::OrderedMap;
use serde_amqp::SerializeComposite;

use crate::amqp::{
    amqp_response_message::peek_session_message::PeekSessionMessageResponse,
    management_constants::operations::PEEK_MESSAGE_OPERATION,
};

#[derive(SerializeComposite)]
#[amqp_contract(encoding = "map", rename_all = "kebab-case")]
struct PeekSessionMessageBody {
    from_sequence_number: i64,
    message_count: i32,
    session_id: String,
}

pub struct PeekSessionMessageRequest {}

// impl Request for PeekSessionMessageRequest {
//     const OPERATION: &'static str = PEEK_MESSAGE_OPERATION;

//     type Response = PeekSessionMessageResponse;

//     type Body;

//     fn encode_body(self) -> Self::Body {
//         todo!()
//     }
// }
