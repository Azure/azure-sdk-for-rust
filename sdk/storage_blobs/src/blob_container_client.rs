use std::sync::Arc;

use azure_core::{
    auth::TokenCredential, BearerTokenCredentialPolicy, ClientOptions, Context, Method, Pipeline,
    Policy, Request, Response, Url,
};
use azure_identity::create_credential;

pub struct BlobContainerClient<'a> {
    // At the moment, we aren't really using this for anything so comes up as "dead fields"
    account_name: &'a str,
    credential: Arc<dyn TokenCredential>,
    container_name: &'a str,
    url: Url,
    pipeline: Pipeline,
}

impl<'a> BlobContainerClient<'a> {
    pub fn new(account_name: &'a str, credential: &'a str, container_name: &'a str) -> Self {
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
            + "?restype=container"; // Something to keep in mind for a "base client" -- conditionally appending this

        // Build our BlobContainerClient
        Self {
            account_name: account_name,
            credential: credential,
            container_name: container_name,
            url: Url::parse(&blob_url).expect("Something went wrong with URL parsing!"),
            pipeline: runner_pipeline,
        }
    }

    // For now, this will handle the x-ms-version issue
    fn finalize_request(mut request: Request) -> Request {
        request.insert_header("x-ms-version", "2023-11-03");
        request
    }

    pub async fn get_container_properties(&self) -> Response {
        // Build the get properties request itself
        let mut request = Request::new(self.url.to_owned(), Method::Head); // This is technically cloning
        request = BlobContainerClient::finalize_request(request);

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

    use crate::BlobContainerClient;

    #[tokio::test]
    async fn test_get_container_properties() {
        // Create a Container Client
        let my_blob_container_client =
            BlobContainerClient::new("vincenttranstock", "throwaway", "acontainer108f32e8");

        // Get response
        let ret = my_blob_container_client.get_container_properties().await;
        let (status_code, headers, response_body) = ret.deconstruct();

        // Assert equality
        assert_eq!(status_code, azure_core::StatusCode::Ok);
        assert_eq!(
            headers
                .get_str(&HeaderName::from_static("content-length"))
                .expect("Failed getting content-length header"),
            "0"
        )
    }
}
