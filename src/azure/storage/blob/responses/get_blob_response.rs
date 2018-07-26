use azure::core::RequestId;
use azure::storage::blob::Blob;

#[derive(Debug, Clone)]
pub struct GetBlobResponse {
    pub blob: Blob,
    pub request_id: RequestId,
    pub data: Vec<u8>,
}
