/// This struct encapsulates the pop (get from queuE) receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PopReceipt {
    message_id: String,
    pop_receipt: String,
}

impl PopReceipt {
    pub(crate) fn new(message_id: impl Into<String>, pop_receipt: impl Into<String>) -> Self {
        Self {
            message_id: message_id.into(),
            pop_receipt: pop_receipt.into(),
        }
    }

    /// These fields are opaque so they should not
    /// be handled by the SDK user.
    pub(crate) fn message_id(&self) -> &str {
        &self.message_id
    }

    /// These fields are opaque so they should not
    /// be handled by the SDK user.
    pub(crate) fn pop_receipt(&self) -> &str {
        &self.pop_receipt
    }
}
