use azure_core::{
    auth::TokenCredential, policies::BearerTokenCredentialPolicy, ClientOptions, Context, Header,
    Method, Pipeline, Policy, Request, Response, Url,
};
use azure_identity::create_credential;
use std::sync::Arc;

pub struct BlobClient {
    account_name: String,
    credential: Arc<dyn TokenCredential>,
    container_name: String,
    blob_name: String,
    url: Url,
    pipeline: Pipeline,
}

impl BlobClient {
    pub fn new(
        account_name: String,
        credential: String,
        container_name: String,
        blob_name: String,
    ) -> Self {
        // Create Respective Authentication Pipeline

        // OAuth Pipeline Policy
        println!("Auth type chosen, Oauth, {}", credential);
        let credential = create_credential().expect("Failed for some reason?");
        let oauth_token_policy = BearerTokenCredentialPolicy::new(
            credential.clone(),
            &["https://storage.azure.com/.default"],
        );

        // Runner Pipeline
        let runner_pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            ClientOptions::default(),
            vec![Arc::new(oauth_token_policy) as Arc<dyn Policy>],
            Vec::new(),
        );

        // Build URL from Input (No validation atm)
        let blob_url = "https://".to_owned()
            + &account_name
            + ".blob.core.windows.net/"
            + &container_name
            + "/"
            + &blob_name;

        // Build our BlobClient
        Self {
            account_name: account_name,
            credential: credential.clone(), // Unsure if clone is the correct move here
            container_name: container_name,
            blob_name: blob_name,
            url: Url::parse(&blob_url).expect("Something went wrong with URL parsing!"),
            pipeline: runner_pipeline,
        }
    }

    // For now, this will handle the x-ms-version issue
    fn finalize_request(mut request: Request) -> Request {
        request.insert_header("x-ms-version", "2023-11-03");
        request
    }

    pub async fn download_blob(&self) -> String {
        // Build the download request itself
        let mut request = Request::new(self.url.to_owned(), Method::Get); // This is technically cloning
        request = BlobClient::finalize_request(request);

        // Send the request
        let response = self.pipeline.send(&(Context::new()), &mut request).await;
        println!("Response headers: {:?}", response);

        // Look at request body
        let response_body = response.unwrap().into_body().collect_string().await;
        println!("Response body: {:?}", response_body);

        // Return the body
        response_body.unwrap()
    }

    pub async fn get_blob_properties(&self) -> Response {
        // Build the get properties request itself
        let mut request = Request::new(self.url.to_owned(), Method::Head); // This is technically cloning
        request = BlobClient::finalize_request(request);

        // Send the request
        let response = self.pipeline.send(&(Context::new()), &mut request).await;
        println!("Response headers: {:?}", response);

        // Return the response headers
        response.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use azure_core::headers::HeaderName;

    use crate::BlobClient;

    #[tokio::test]
    async fn test_download_blob() {
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

    #[tokio::test]
    async fn test_get_blob_properties() {
        // Create a Blob Client
        let my_blob_client = BlobClient::new(
            "vincenttranstock".to_string(),
            "throwaway".to_string(),
            "acontainer108f32e8".to_string(),
            "hello.txt".to_string(),
        );

        // Get response
        let ret = my_blob_client.get_blob_properties().await;
        let (status_code, headers, response_body) = ret.deconstruct();

        // Assert equality
        assert_eq!(status_code, azure_core::StatusCode::Ok);
        assert_eq!(
            headers
                .get_str(&HeaderName::from_static("content-length"))
                .expect("Failed getting content-length header"),
            "10"
        )
    }
}
