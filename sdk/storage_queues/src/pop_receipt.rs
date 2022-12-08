/// This struct encapsulates the pop (get from queue) receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PopReceipt {
    message_id: String,
    pop_receipt: String,
}

impl PopReceipt {
    pub fn new(message_id: impl Into<String>, pop_receipt: impl Into<String>) -> Self {
        Self {
            message_id: message_id.into(),
            pop_receipt: pop_receipt.into(),
        }
    }

    pub fn message_id(&self) -> &str {
        &self.message_id
    }

    pub fn pop_receipt(&self) -> &str {
        &self.pop_receipt
    }
}
