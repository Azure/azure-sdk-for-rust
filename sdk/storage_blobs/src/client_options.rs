use azure_core::auth::TokenCredential;

pub struct BlobClientOptions<'a> {
    credential: &'a dyn TokenCredential,
}

impl<'a> BlobClientOptions<'a> {
    pub fn credential(self, credential: &impl TokenCredential) -> Self {
        todo!()
    }
}
