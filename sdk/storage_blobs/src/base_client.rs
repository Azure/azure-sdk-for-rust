use azure_core::{
    auth::TokenCredential, policies::BearerTokenCredentialPolicy, ClientOptions, Context, Header,
    Method, Pipeline, Policy, Request, Response, Url,
};
use azure_identity::create_credential;
use std::{borrow::Cow, sync::Arc};

// Good time to ask: Are we going with getter/setter or dealing with access on the struct level? Cursory research seems that people do a getter that matches in name to the field (and has a mutable and non-mutable ret)
pub struct BaseClient<'a> {
    account_name: &'a str,
    pub credential: Arc<dyn TokenCredential>,
    pub base_url: Cow<'a, str>, // I think this is the correct approach to all str living in a struct
    pub pipeline: Pipeline,
}

impl<'a> BaseClient<'a> {
    pub fn new(account_name: &'a str, service: &'a str, credential: &'a str) -> Self {
        // Check Service
        if !(["blob", "queue", "file-share", "dfs"].contains(&service)) {
            println!("Not a valid service. Exiting.");
            std::process::exit(1);
        }

        // In the future we will intelligently sort these to build the necessary pipeline policies, for now it's OAuth or nothing!

        // OAuth Pipeline Policy
        println!("Auth type chosen, Oauth, {}", credential);
        let credential = create_credential().expect("Failed for some reason?");
        let oauth_token_policy = BearerTokenCredentialPolicy::new(
            credential.clone(),
            &["https://storage.azure.com/.default"],
        );

        // Build the pipeline
        let pipeline =
            BaseClient::build_pipeline(vec![Arc::new(oauth_token_policy) as Arc<dyn Policy>]);

        // Build URL from Input (No validation atm)
        let base_url = "https://".to_owned() + &account_name + "." + service + ".core.windows.net/";

        // Build our BaseClient
        Self {
            account_name: account_name,
            credential: credential,
            base_url: base_url.into(),
            pipeline: pipeline,
        }
    }

    // For now, this will handle the x-ms-version issue
    pub fn finalize_request(mut request: Request) -> Request {
        request.insert_header("x-ms-version", "2023-11-03");
        request
    }

    fn build_pipeline(policies: Vec<Arc<dyn Policy>>) -> Pipeline {
        Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            ClientOptions::default(),
            policies,
            Vec::new(),
        )
    }
}
