use crate::{headers, Header};

/// Conditional request header based on the value of the object's sequence number
///
/// Ref: <https://docs.microsoft.com/en-us/rest/api/storageservices/put-page-from-url>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IfSequenceNumber {
    /// If the object's sequence number is less than the specified value, the
    /// request proceeds; otherwise it fails with SequenceNumberConditionNotMet
    /// error (HTTP status code 412 – Precondition Failed).
    LessThan(u64),
    /// If the object's sequence number is less than or equal to the specified
    /// value, the request proceeds; otherwise it fails with the
    /// SequenceNumberConditionNotMet error (HTTP status code 412 – Precondition
    /// Failed).
    LessOrEqual(u64),
    /// If the object’s sequence number is equal to the specified value, the
    /// request proceeds; otherwise it fails with SequenceNumberConditionNotMet
    /// error (HTTP status code 412 – Precondition Failed).
    Equal(u64),
}

impl Header for IfSequenceNumber {
    fn name(&self) -> headers::HeaderName {
        match self {
            IfSequenceNumber::Equal(_) => headers::IF_SEQUENCE_NUMBER_EQ,
            IfSequenceNumber::LessOrEqual(_) => headers::IF_SEQUENCE_NUMBER_LE,
            IfSequenceNumber::LessThan(_) => headers::IF_SEQUENCE_NUMBER_LT,
        }
    }

    fn value(&self) -> headers::HeaderValue {
        match self {
            IfSequenceNumber::Equal(val)
            | IfSequenceNumber::LessOrEqual(val)
            | IfSequenceNumber::LessThan(val) => val.to_string().into(),
        }
    }
}
