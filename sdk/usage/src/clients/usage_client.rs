use crate::operations::*;
use azure_core::{ClientOptions, Context, Pipeline, Request, Response, TimeoutPolicy};

#[derive(Debug, Clone)]
pub struct UsageClient {
    pipeline: Pipeline,
    cloud_suffix: String,
    account_name: String,
    account_location: String,
}

impl UsageClient {
    pub fn new(
        cloud_suffix: impl Into<String>,
        account_name: impl Into<String>,
        account_location: impl Into<String>,
    ) -> Self {
        Self {
            pipeline: new_pipeline_from_options(),
            cloud_suffix: cloud_suffix.into(),
            account_name: account_name.into(),
            account_location: account_location.into(),
        }
    }

    pub fn connect(&self, input: impl Into<String>) -> ConnectBuilder {
        ConnectBuilder::new(self.clone(), input.into())
    }

    /// Prepares' an `azure_core::Request`.
    ///
    /// This function will add the cloud location to the URI suffix and generate
    /// a Request with the specified HTTP Method. It will also set the body
    /// to an empty `Bytes` instance.
    pub(crate) fn request(&self, uri_path: &str, http_method: azure_core::Method) -> Request {
        let uri = format!(
            "{}.{}/{}",
            self.account_location, self.cloud_suffix, uri_path
        );
        Request::new(uri.parse().unwrap(), http_method)
    }

    pub(crate) async fn send(
        &self,
        request: &mut Request,
        context: &mut Context,
    ) -> azure_core::Result<Response> {
        self.pipeline.send(context, request).await
    }
}

fn new_pipeline_from_options() -> Pipeline {
    // TODO: as we move to the builder pattern for the clients, these should be
    // set there.
    let client_options = ClientOptions::default();
    let timeout_policy = TimeoutPolicy::new(None);

    Pipeline::new(
        option_env!("CARGO_PKG_NAME"),
        option_env!("CARGO_PKG_VERSION"),
        client_options,
        Vec::new(),
        Vec::new(),
    )
}
