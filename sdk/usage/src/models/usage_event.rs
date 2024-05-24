use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct UsageEvent {
    pub event_id: Uuid, // TODO: or string?
    pub subject_name: String,
    pub subject_id: String,
}
