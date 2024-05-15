use azure_core::{
    auth::TokenCredential, policies::BearerTokenCredentialPolicy, ClientOptions, Context, Error,
    Method, Pipeline, Policy, Request, Response, Url,
};
use azure_identity::create_credential;
use std::sync::Arc;

pub struct BlobClient {
    account_name: String,
    credential: Arc<dyn TokenCredential>,
    container_name: String,
    blob_name: String,
    pipeline: BearerTokenCredentialPolicy,
}

impl BlobClient {
    pub fn new(
        account_name: String,
        credential: String,
        container_name: String,
        blob_name: String,
    ) -> Self {
        // Create Respective Authentication Pipeline

        // In this case, we determine it's Oauth
        println!("Auth type chosen, Oauth, {}", credential);
        let credential = create_credential().expect("Failed for some reason?");
        let runner_pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            ClientOptions::default(),
            Vec::new(),
            Vec::new(),
        );
        let oauth_token_pipeline =
            BearerTokenCredentialPolicy::new(credential.clone(), runner_pipeline);
        Self {
            account_name: account_name,
            credential: credential.clone(), // Unsure if clone is the correct move here
            container_name: container_name,
            blob_name: blob_name,
            pipeline: oauth_token_pipeline,
        }
    }

    pub async fn download_blob(&self) -> String {
        // Build the things needed for the download request

        // Would probably have a sophisticated blob url builder
        let blob_url = "https://".to_owned()
            + &(self.account_name)
            + ".blob.core.windows.net/"
            + &(self.container_name)
            + "/"
            + &(self.blob_name);
        let blob_url = Url::parse(&blob_url).expect("Failed to parse URL for some reason");

        // Build the download request itself
        let mut request = Request::new(blob_url, Method::Get);

        // Send the request
        let response = self
            .pipeline
            .send(&(Context::new()), &mut request, &[])
            .await;
        println!("Response headers: {:?}", response);

        // Look at request body
        let response_body = response.unwrap().into_body().collect_string().await;
        println!("Response body: {:?}", response_body);

        response_body.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::BlobClient;

    #[tokio::test]
    async fn test_blob_client() {
        // Create a Blob Client
        let my_blob_client = BlobClient::new(
            "vincenttranstock".to_string(),
            "throwaway".to_string(),
            "acontainer108f32e8".to_string(),
            "hello.txt".to_string(),
        );

        // Assert equality
        assert_eq!(my_blob_client.download_blob().await, "rustaceans")
    }
}
