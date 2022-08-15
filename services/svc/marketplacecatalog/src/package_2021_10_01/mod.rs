#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
pub mod models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
    options: azure_core::ClientOptions,
}
pub const DEFAULT_ENDPOINT: &str = "https://catalogapi.azure.com";
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
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
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
    #[must_use]
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes, self.options)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: &mut azure_core::Request) -> azure_core::Result<azure_core::Response> {
        let mut context = azure_core::Context::default();
        self.pipeline.send(&mut context, request).await
    }
    #[doc = "Create a new `ClientBuilder`."]
    #[must_use]
    pub fn builder(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> ClientBuilder {
        ClientBuilder::new(credential)
    }
    #[doc = "Create a new `Client`."]
    #[must_use]
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
        options: azure_core::ClientOptions,
    ) -> Self {
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
    pub fn public_offers_client(&self) -> public_offers::Client {
        public_offers::Client(self.clone())
    }
}
pub mod public_offers {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a public offer by id"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: Offer id"]
        pub fn get(&self, id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                id: id.into(),
                language: None,
                market: None,
                include_stop_sold_plans: None,
                hide_keys: Vec::new(),
            }
        }
        #[doc = "Get a list of public available offers"]
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                language: None,
                hide_keys: Vec::new(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::CatalogItem;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) language: Option<String>,
            pub(crate) market: Option<String>,
            pub(crate) include_stop_sold_plans: Option<bool>,
            pub(crate) hide_keys: Vec<String>,
        }
        impl Builder {
            #[doc = "Offer language"]
            pub fn language(mut self, language: impl Into<String>) -> Self {
                self.language = Some(language.into());
                self
            }
            #[doc = "Offer market"]
            pub fn market(mut self, market: impl Into<String>) -> Self {
                self.market = Some(market.into());
                self
            }
            #[doc = "To include stop sold or hidden plans"]
            pub fn include_stop_sold_plans(mut self, include_stop_sold_plans: bool) -> Self {
                self.include_stop_sold_plans = Some(include_stop_sold_plans);
                self
            }
            #[doc = "Add hide key to retrieve preview items"]
            pub fn hide_keys(mut self, hide_keys: Vec<String>) -> Self {
                self.hide_keys = hide_keys;
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/offers/{}", this.client.endpoint(), &this.id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        if let Some(language) = &this.language {
                            req.url_mut().query_pairs_mut().append_pair("language", language);
                        }
                        if let Some(market) = &this.market {
                            req.url_mut().query_pairs_mut().append_pair("market", market);
                        }
                        if let Some(include_stop_sold_plans) = &this.include_stop_sold_plans {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeStopSoldPlans", &include_stop_sold_plans.to_string());
                        }
                        let hide_keys = &this.hide_keys;
                        for value in &this.hide_keys {
                            req.url_mut().query_pairs_mut().append_pair("hideKeys", &value.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CatalogItem = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::PageResultOfCatalogItem;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) language: Option<String>,
            pub(crate) hide_keys: Vec<String>,
        }
        impl Builder {
            #[doc = "Offer language"]
            pub fn language(mut self, language: impl Into<String>) -> Self {
                self.language = Some(language.into());
                self
            }
            #[doc = "Add hide key to retrieve preview items"]
            pub fn hide_keys(mut self, hide_keys: Vec<String>) -> Self {
                self.hide_keys = hide_keys;
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/offers", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        if let Some(language) = &this.language {
                            req.url_mut().query_pairs_mut().append_pair("language", language);
                        }
                        let hide_keys = &this.hide_keys;
                        for value in &this.hide_keys {
                            req.url_mut().query_pairs_mut().append_pair("hideKeys", &value.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PageResultOfCatalogItem = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
}
