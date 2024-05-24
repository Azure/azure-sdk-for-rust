use azure_core::{
    auth::TokenCredential, ClientOptions, Context, Pipeline, Request, Response, TimeoutPolicy,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct UsageClient {
    pipeline: Pipeline,
    account_name: String,
    account_location: String,
}

impl UsageClient {
    pub fn new(account_name: impl Into<String>, account_location: impl Into<String>) -> Self {
        Self {
            pipeline: new_pipeline_from_options(),
            account_name: account_name.into(),
            account_location: account_location.into(),
        }
    }

    pub(crate) async fn send(
        &self,
        context: &mut Context,
        request: &mut Request,
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
