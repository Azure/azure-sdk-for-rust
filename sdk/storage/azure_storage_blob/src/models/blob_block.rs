use azure_core::base64;

pub struct BlobBlock {
    data: Vec<String>,
}

impl BlobBlock {
    pub fn new(input: Vec<String>) -> Self {
        let encoded_data = input.into_iter().map(base64::encode).collect();
        BlobBlock { data: encoded_data }
    }
}

impl From<BlobBlock> for Vec<String> {
    fn from(blob: BlobBlock) -> Self {
        blob.data
    }
}
