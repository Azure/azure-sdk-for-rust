use crate::headers::*;
use crate::Header;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SequenceNumberCondition {
    Less(u64),
    LessOrEqual(u64),
    Equal(u64),
}

impl Header for SequenceNumberCondition {
    fn name(&self) -> &'static str {
        match self {
            SequenceNumberCondition::Equal(_) => IF_SEQUENCE_NUMBER_EQ,
            SequenceNumberCondition::LessOrEqual(_) => IF_SEQUENCE_NUMBER_LE,
            SequenceNumberCondition::Less(_) => IF_SEQUENCE_NUMBER_LT,
        }
    }

    fn value(&self) -> String {
        match self {
            SequenceNumberCondition::Equal(val) => val.to_string(),
            SequenceNumberCondition::LessOrEqual(val) => val.to_string(),
            SequenceNumberCondition::Less(val) => val.to_string(),
        }
    }
}
