use azure::core::incompletevector::IncompleteVector;
use azure::core::RequestId;
use azure::storage::blob::Blob;

#[derive(Debug, Clone)]
pub struct ListBlobsResponse {
    pub incomplete_vector: IncompleteVector<Blob>,
    pub request_id: RequestId,
}
