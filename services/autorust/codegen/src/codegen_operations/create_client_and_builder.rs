use crate::{identifier::SnakeCaseIdent, Result};
use proc_macro2::TokenStream;
use quote::quote;

pub fn create_client(modules: &[String], endpoint: Option<&str>) -> Result<TokenStream> {
    let mut clients = TokenStream::new();
    for md in modules {
        let client = format!("{md}_client").to_snake_case_ident()?;
        let md = md.to_snake_case_ident()?;
        clients.extend(quote! {
            pub fn #client(&self) -> #md::Client {
                #md::Client(self.clone())
            }
        });
    }

    let public_cloud = quote! {
        pub use azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD as DEFAULT_ENDPOINT;
    };
    let default_endpoint_code = if let Some(endpoint) = endpoint {
        if endpoint == "https://management.azure.com" {
            public_cloud
        } else {
            quote! {
                azure_core::static_url!(DEFAULT_ENDPOINT, #endpoint);
            }
        }
    } else {
        public_cloud
    };

    let mut code = TokenStream::new();
    code.extend(quote! {

        #[derive(Clone)]
        pub struct Client {
            endpoint: azure_core::Url,
            credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
            scopes: Vec<String>,
            pipeline: azure_core::Pipeline,
        }

        #[derive(Clone)]
        pub struct ClientBuilder {
            credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
            endpoint: Option<azure_core::Url>,
            scopes: Option<Vec<String>>,
            options: azure_core::ClientOptions,
        }

        #default_endpoint_code

        impl ClientBuilder {
            #[doc = "Create a new instance of `ClientBuilder`."]
            #[must_use]
            pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
                Self {
                    credential,
                    endpoint: None,
                    scopes: None,
                    options: azure_core::ClientOptions::default(),
                }
            }

            #[doc = "Set the endpoint."]
            #[must_use]
            pub fn endpoint(mut self, endpoint: impl Into<azure_core::Url>) -> Self {
                self.endpoint = Some(endpoint.into());
                self
            }

            #[doc = "Set the scopes."]
            #[must_use]
            pub fn scopes(mut self, scopes: &[&str]) -> Self {
                self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
                self
            }

            #[doc = "Set the retry options."]
            #[must_use]
            pub fn retry(mut self, retry: impl Into<azure_core::RetryOptions>) -> Self {
                self.options = self.options.retry(retry);
                self
            }

            #[doc = "Set the transport options."]
            #[must_use]
            pub fn transport(mut self, transport: impl Into<azure_core::TransportOptions>) -> Self {
                self.options = self.options.transport(transport);
                self
            }

            #[doc = "Convert the builder into a `Client` instance."]
            pub fn build(self) -> azure_core::Result<Client> {
                let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
                let scopes = if let Some(scopes) = self.scopes {
                    scopes
                } else {
                    vec![endpoint.join(azure_core::auth::DEFAULT_SCOPE_SUFFIX)?.to_string()]
                };
                Ok(Client::new(endpoint, self.credential, scopes, self.options))
            }
        }

        impl Client {
            pub(crate) async fn bearer_token(&self) -> azure_core::Result<azure_core::auth::Secret> {
                let credential = self.token_credential();
                let response = credential.get_token(&self.scopes()).await?;
                Ok(response.token)
            }

            pub(crate) fn endpoint(&self) -> &azure_core::Url {
                &self.endpoint
            }
            pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
                self.credential.as_ref()
            }
            pub(crate) fn scopes(&self) -> Vec<&str> {
                self.scopes.iter().map(String::as_str).collect()
            }
            pub(crate) async fn send(&self, request: &mut azure_core::Request) -> azure_core::Result<azure_core::Response> {
                let context = azure_core::Context::default();
                self.pipeline.send(&context, request).await
            }

            #[doc = "Create a new `ClientBuilder`."]
            #[must_use]
            pub fn builder(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> ClientBuilder {
                ClientBuilder::new(credential)
            }

            #[doc = "Create a new `Client`."]
            #[must_use]
            pub fn new(endpoint: impl Into<azure_core::Url>, credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>, scopes: Vec<String>, options: azure_core::ClientOptions) -> Self {
                let endpoint = endpoint.into();
                let pipeline = azure_core::Pipeline::new(
                    option_env!("CARGO_PKG_NAME"),
                    option_env!("CARGO_PKG_VERSION"),
                    options,
                    Vec::new(),
                    Vec::new(),
                );
                Self {
                    endpoint,
                    credential,
                    scopes,
                    pipeline,
                }
            }

            #clients
        }
    });
    Ok(code)
}
