pub trait AttachmentName: std::fmt::Debug {
    fn name(&self) -> &str;
}

impl AttachmentName for &str {
    fn name(&self) -> &str {
        self
    }
}

impl AttachmentName for String {
    fn name(&self) -> &str {
        self.as_ref()
    }
}
