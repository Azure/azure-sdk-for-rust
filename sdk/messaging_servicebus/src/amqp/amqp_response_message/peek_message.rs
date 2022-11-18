use fe2o3_amqp_management::error::Error as MgmtError;
use fe2o3_amqp_management::{
    error::{InvalidType, StatusError},
    mgmt_ext::AmqpMessageManagementExt,
    response::Response,
    status::StatusCode,
};
use fe2o3_amqp_types::primitives::{Array, OrderedMap};

use crate::amqp::management_constants::properties::{MESSAGE, MESSAGES};

type EncodedMessage = Array<u8>;
type EncodedMessages = Vec<OrderedMap<String, EncodedMessage>>;

pub struct PeekMessageResponse {
    pub body: OrderedMap<String, EncodedMessages>,
}

impl PeekMessageResponse {
    const STATUS_CODE_OK: u16 = 200;
    const STATUS_CODE_NO_CONTENT: u16 = 204;

    pub fn into_messages(mut self) -> Option<impl Iterator<Item = Vec<u8>>> {
        let messages = self.body.remove(MESSAGES)?;

        let messages = messages
            .into_iter()
            .filter_map(|mut map| map.remove(MESSAGE).map(|arr| arr.into_inner()));

        Some(messages)
    }
}

impl Response for PeekMessageResponse {
    const STATUS_CODE: u16 = Self::STATUS_CODE_OK; //

    type Body = OrderedMap<String, EncodedMessages>;

    type Error = MgmtError;

    fn verify_status_code(
        message: &mut fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<StatusCode, Self::Error> {
        let status_code = match message
            .remove_status_code()
            .ok_or(MgmtError::StatusCodeNotFound)?
        {
            Ok(code) => code,
            Err(err) => {
                return Err(InvalidType {
                    expected: "u16".to_string(),
                    actual: format!("{:?}", err),
                }
                .into())
            }
        };
        match status_code.0.get() {
            Self::STATUS_CODE_OK | Self::STATUS_CODE_NO_CONTENT => Ok(status_code),
            _ => {
                let description = match message.remove_status_description() {
                    Some(Ok(status_description)) => Some(status_description),
                    Some(Err(err)) => {
                        return Err(InvalidType {
                            expected: "String".to_string(),
                            actual: format!("{:?}", err),
                        }
                        .into())
                    }
                    None => None,
                };
                Err(StatusError {
                    code: status_code,
                    description: description.map(Into::into),
                }
                .into())
            }
        }
    }

    fn decode_message(
        message: fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<Self, Self::Error> {
        Ok(Self { body: message.body })
    }

    fn from_message(
        mut message: fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<Self, Self::Error> {
        let status_code = Self::verify_status_code(&mut message)?;
        match status_code.0.get() {
            Self::STATUS_CODE_OK => {
                let body = message.body;
                Ok(Self { body })
            }
            Self::STATUS_CODE_NO_CONTENT => Ok(Self {
                body: OrderedMap::with_capacity(0),
            }),
            _ => unreachable!(),
        }
    }
}
