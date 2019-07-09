use azure_sdk_core::incompletevector::IncompleteVector;
use azure_sdk_core::RequestId;
use crate::container::Container;

#[derive(Debug, Clone)]
pub struct ListContainersResponse {
    pub incomplete_vector: IncompleteVector<Container>,
    pub request_id: RequestId,
}

impl ListContainersResponse {
    pub fn is_complete(&self) -> bool {
        self.incomplete_vector.is_complete()
    }
}
