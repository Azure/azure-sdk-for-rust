pub trait ClientEndpoint {
    fn account(&self) -> &str;
    fn key(&self) -> &str;
}
