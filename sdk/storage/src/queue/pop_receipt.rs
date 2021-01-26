pub trait PopReceipt {
    fn message_id(&self) -> &str;
    fn pop_receipt(&self) -> &str;
}

impl<'a> std::fmt::Debug for &'a dyn PopReceipt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "PopReceipt {{ message_id: {}, pop_receipt: {} }}",
            self.message_id(),
            self.pop_receipt()
        )
    }
}
