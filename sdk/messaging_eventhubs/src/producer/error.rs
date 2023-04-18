#[derive(Debug)]
pub struct CannotPublishToGateway {
    _sealed: (),
}

impl CannotPublishToGateway {
    pub(crate) fn new() -> Self {
        Self { _sealed: () }
    }
}

impl std::fmt::Display for CannotPublishToGateway {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot publish to gateway")
    }
}

impl std::error::Error for CannotPublishToGateway {}

impl From<CannotPublishToGateway> for azure_core::Error {
    fn from(e: CannotPublishToGateway) -> Self {
        use azure_core::error::ErrorKind;

        Self::new(ErrorKind::Other, e)
    }
}

/// If the state was not initialized and no exception has occurred, then the service is behaving
/// unexpectedly and the client should be considered invalid.
#[derive(Debug)]
pub struct InvalidPartitionState {
    _sealed: (),
}

impl InvalidPartitionState {
    pub(crate) fn new() -> Self {
        Self { _sealed: () }
    }
}

impl std::fmt::Display for InvalidPartitionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid partition state")
    }
}

impl std::error::Error for InvalidPartitionState {}

impl From<InvalidPartitionState> for azure_core::Error {
    fn from(e: InvalidPartitionState) -> Self {
        use azure_core::error::ErrorKind;

        Self::new(ErrorKind::Other, e)
    }
}
