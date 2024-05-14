use azure_core::{
    auth::TokenCredential, policies::BearerTokenCredentialPolicy, Context, Method, Policy, Request,
    Url,
};
use azure_identity::create_credential;
use std::sync::Arc;

pub struct BlobClient<'a> {
    account_name: &'a str,
    credential: Arc<dyn TokenCredential>,
    container_name: &'a str,
    blob_name: &'a str,
    pipeline: BearerTokenCredentialPolicy,
}

impl BlobClient<'_> {
    pub fn new(
        account_name: &str,
        credential: &str,
        container_name: &str,
        blob_name: &str,
    ) -> Self {
        // Create Respective Authentication Pipeline

        // In this case, we determine it's Oauth
        println!("Auth type chosen, Oauth, {}", credential);
        let credential = create_credential().expect("Failed for some reason?");
        Self {
            account_name: account_name,
            credential: credential.clone(), // Unsure if clone is the correct move here
            container_name: container_name,
            blob_name: blob_name,
            pipeline: BearerTokenCredentialPolicy::new(credential),
        }
    }

    pub async fn download_blob(&self) {
        // Build the things needed for the downlaod request

        // Would probably have a sophisticated blob url builder
        let blob_url = "https://".to_owned()
            + self.account_name
            + ".blob.core.windows.net/"
            + self.container_name
            + "/"
            + self.blob_name;
        let blob_url = Url::parse(&blob_url).expect("Failed to parse URL for some reason");

        // Build the download request itself
        let mut request = Request::new(blob_url, Method::Get);

        // Cast our custom policy to regular policy?
        let copied_policy = vec![Arc::new(self.pipeline.clone()) as Arc<dyn Policy>];

        // Send the request
        let response = self
            .pipeline
            .send(&(Context::new()), &mut request, &copied_policy)
            .await;
        println!("Response headers: {:?}", response);

        // Look at request body
        let response_body = response.unwrap().into_body().collect_string().await;
        println!("Response body: {:?}", response_body);
    }
}
