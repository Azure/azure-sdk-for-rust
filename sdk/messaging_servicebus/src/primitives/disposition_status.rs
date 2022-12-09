use serde_amqp::Value;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum DispositionStatus {
    Completed = 1,
    Defered = 2,
    Suspended = 3,
    Abandoned = 4,
    Renewed = 5,
}

// TODO: Is this how it is serialized?
impl From<DispositionStatus> for String {
    fn from(status: DispositionStatus) -> Self {
        match status {
            DispositionStatus::Completed => "completed".to_string(),
            DispositionStatus::Abandoned => "abandoned".to_string(),
            DispositionStatus::Suspended => "suspended".to_string(),
            DispositionStatus::Defered => "defered".to_string(),
            DispositionStatus::Renewed => "renewed".to_string(),
        }
    }
}

impl From<DispositionStatus> for Value {
    fn from(status: DispositionStatus) -> Self {
        match status {
            DispositionStatus::Completed => Value::String("completed".into()),
            DispositionStatus::Abandoned => Value::String("abandoned".into()),
            DispositionStatus::Suspended => Value::String("suspended".into()),
            DispositionStatus::Defered => Value::String("defered".into()),
            DispositionStatus::Renewed => Value::String("renewed".into()),
        }
    }
}
