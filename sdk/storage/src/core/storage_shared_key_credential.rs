/// A StorageSharedKeyCredential is a credential backed by a Storage Account's name and one of its access keys.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageSharedKeyCredential {
    /// The name of the Storage Account.
    pub account_name: String,
    /// A Storage Account access key.
    pub account_key: String,
}

impl StorageSharedKeyCredential {
    /// Initializes a new instance of the StorageSharedKeyCredential class.
    pub fn new(account_name: String, account_key: String) -> Self {
        Self {
            account_name,
            account_key,
        }
    }
}
