use crate::BaseClient;
use azure_core::{Context, Method, Pipeline, Request, Response, Url};

pub struct BlobClient<'a> {
    // At the moment, we aren't really using this for anything so comes up as "dead fields"
    account_name: &'a str,
    container_name: &'a str,
    blob_name: &'a str,
    blob_url: Url,
    pipeline: Pipeline,
}

impl<'a> BlobClient<'a> {
    pub fn new(
        account_name: &'a str,
        credential: &'a str,
        container_name: &'a str,
        blob_name: &'a str,
    ) -> Self {
        // Create Base Client
        let base_client = BaseClient::new(account_name, "blob", credential);

        // Build URL from Input (No validation atm)
        let blob_url = base_client.base_url.to_string() + &container_name + "/" + &blob_name;

        // Build our BlobClient
        Self {
            account_name: account_name,
            container_name: container_name,
            blob_name: blob_name,
            blob_url: Url::parse(&blob_url).expect("Something went wrong with URL parsing!"),
            pipeline: base_client.pipeline,
        }
    }

    pub async fn download_blob(&self) -> String {
        // Build the download request itself
        let mut request = Request::new(self.blob_url.to_owned(), Method::Get); // This is technically cloning
        request = BaseClient::finalize_request(request);

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
        let mut request = Request::new(self.blob_url.to_owned(), Method::Head); // This is technically cloning
        request = BaseClient::finalize_request(request);

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
            "vincenttranstock",
            "throwaway",
            "acontainer108f32e8",
            "hello.txt",
        );

        // Assert equality
        assert_eq!(my_blob_client.download_blob().await, "rustaceans")
    }

    #[tokio::test]
    async fn test_get_blob_properties() {
        // Create a Blob Client
        let my_blob_client = BlobClient::new(
            "vincenttranstock",
            "throwaway",
            "acontainer108f32e8",
            "hello.txt",
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
