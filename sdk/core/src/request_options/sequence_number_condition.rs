use crate::{headers, Header};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SequenceNumberCondition {
    Less(u64),
    LessOrEqual(u64),
    Equal(u64),
}

impl Header for SequenceNumberCondition {
    fn name(&self) -> headers::HeaderName {
        match self {
            SequenceNumberCondition::Equal(_) => headers::IF_SEQUENCE_NUMBER_EQ,
            SequenceNumberCondition::LessOrEqual(_) => headers::IF_SEQUENCE_NUMBER_LE,
            SequenceNumberCondition::Less(_) => headers::IF_SEQUENCE_NUMBER_LT,
        }
    }

    fn value(&self) -> headers::HeaderValue {
        match self {
            SequenceNumberCondition::Equal(val) => val.to_string(),
            SequenceNumberCondition::LessOrEqual(val) => val.to_string(),
            SequenceNumberCondition::Less(val) => val.to_string(),
        }
        .into()
    }
}
