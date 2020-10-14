pub trait UserDefinedFunctionName: std::fmt::Debug {
    fn name(&self) -> &str;
}

impl UserDefinedFunctionName for &str {
    fn name(&self) -> &str {
        self
    }
}

impl UserDefinedFunctionName for String {
    fn name(&self) -> &str {
        self.as_ref()
    }
}
