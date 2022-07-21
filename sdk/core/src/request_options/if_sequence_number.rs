use crate::{headers, Header};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IfSequenceNumber {
    Less(u64),
    LessOrEqual(u64),
    Equal(u64),
}

impl Header for IfSequenceNumber {
    fn name(&self) -> headers::HeaderName {
        match self {
            IfSequenceNumber::Equal(_) => headers::IF_SEQUENCE_NUMBER_EQ,
            IfSequenceNumber::LessOrEqual(_) => headers::IF_SEQUENCE_NUMBER_LE,
            IfSequenceNumber::Less(_) => headers::IF_SEQUENCE_NUMBER_LT,
        }
    }

    fn value(&self) -> headers::HeaderValue {
        match self {
            IfSequenceNumber::Equal(val)
            | IfSequenceNumber::LessOrEqual(val)
            | IfSequenceNumber::Less(val) => val.to_string().into(),
        }
    }
}
