pub trait StoredProcedureName: std::fmt::Debug {
    fn name(&self) -> &str;
}

impl StoredProcedureName for &str {
    fn name(&self) -> &str {
        self
    }
}

impl StoredProcedureName for String {
    fn name(&self) -> &str {
        self.as_ref()
    }
}
