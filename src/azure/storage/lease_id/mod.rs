#[derive(Debug, Clone, PartialEq)]
pub struct LeaseId {
    id: String,
}

impl LeaseId {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn set_id(&mut self, id: &str) {
        self.id = id.to_owned();
    }
}
