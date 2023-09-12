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
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{endpoint}/")]);
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
    pub fn facets_client(&self) -> facets::Client {
        facets::Client(self.clone())
    }
    pub fn public_products_client(&self) -> public_products::Client {
        public_products::Client(self.clone())
    }
    pub fn search_client(&self) -> search::Client {
        search::Client(self.clone())
    }
    pub fn single_public_product_client(&self) -> single_public_product::Client {
        single_public_product::Client(self.clone())
    }
    pub fn suggestions_client(&self) -> suggestions::Client {
        suggestions::Client(self.clone())
    }
}
pub mod single_public_product {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Single Public Product API provides an unauthenticated endpoint for consuming product attributes of a single commercial Marketplace public product. Each API request must include an API key allocated to you. Send an email to MKPL_Platform_API_DL@microsoft.com including the following details to get the API Key:\n- Customer name\n- Service/Product name\n- AD Registered App ID (if available)\n- Focal point email\n- Use case/scenario\n- Expected traffic volume, including peak requests per second (daily)"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The unique product Id"]
        #[doc = "* `market`: Product market value  (the response will include only products that can be sold in the specified market)\nPossible values can be found at https://docs.microsoft.com/en-us/azure/marketplace/marketplace-geo-availability-currencies. Example: 'US'"]
        #[doc = "* `x_api_key`: Use Api key provided by Marketplace Catalog Team as a parameter provided in the header"]
        pub fn get(&self, id: impl Into<String>, market: impl Into<String>, x_api_key: impl Into<String>) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                market: market.into(),
                x_api_key: x_api_key.into(),
                language: None,
                include_stop_sold_plans: None,
                hide_keys: Vec::new(),
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProductDetails> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProductDetails = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) id: String,
            pub(crate) market: String,
            pub(crate) x_api_key: String,
            pub(crate) language: Option<String>,
            pub(crate) include_stop_sold_plans: Option<bool>,
            pub(crate) hide_keys: Vec<String>,
        }
        impl RequestBuilder {
            #[doc = "Language to search, ISO 639-1 two-letter code, possible values - 'en,cs,de,es,fr,hu,it,ja,ko,nl,pl,pt-br,pt-pt,ru,sv,tr,zh-hans,zh-hant'. Default is \"en\""]
            pub fn language(mut self, language: impl Into<String>) -> Self {
                self.language = Some(language.into());
                self
            }
            #[doc = "Denotes whether to include stop sold plans"]
            pub fn include_stop_sold_plans(mut self, include_stop_sold_plans: bool) -> Self {
                self.include_stop_sold_plans = Some(include_stop_sold_plans);
                self
            }
            #[doc = "Add hide key to retrieve preview items. Hidekeys are provided in 'multi' format. Example: HideKey[]=first"]
            pub fn hide_keys(mut self, hide_keys: Vec<String>) -> Self {
                self.hide_keys = hide_keys;
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/products/{}", this.client.endpoint(), &this.id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2023-01-01-preview");
                        let market = &this.market;
                        req.url_mut().query_pairs_mut().append_pair("market", market);
                        if let Some(language) = &this.language {
                            req.url_mut().query_pairs_mut().append_pair("language", language);
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
                        req.insert_header("x-api-key", &this.x_api_key);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ProductDetails>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ProductDetails>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod public_products {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Public Products API provides an unauthenticated endpoint for consuming commercial Marketplace public products. In addition, it enables filtering on selected product\u{202f}properties, search, and getting starting price information per product. Each API request must include an API key allocated to you (see X-API-Key description).Send an email to MKPL_Platform_API_DL@microsoft.com including the following details to get the API Key:\n- Customer name\n- Service/Product name\n- AD Registered App ID (if available)\n- Focal point email\n- Use case/scenario\n- Expected traffic volume, including peak requests per second (daily)"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `x_api_key`: Use Api key provided by Marketplace Catalog Team as a parameter provided in the header"]
        pub fn list(&self, x_api_key: impl Into<String>) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                x_api_key: x_api_key.into(),
                storefront: None,
                language: None,
                market: None,
                hide_keys: Vec::new(),
                select: Vec::new(),
                filter: None,
                expand: Vec::new(),
                orderby: None,
            }
        }
    }
    pub mod list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::CatalogApiResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::CatalogApiResponse = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) x_api_key: String,
            pub(crate) storefront: Option<String>,
            pub(crate) language: Option<String>,
            pub(crate) market: Option<String>,
            pub(crate) hide_keys: Vec<String>,
            pub(crate) select: Vec<String>,
            pub(crate) filter: Option<String>,
            pub(crate) expand: Vec<String>,
            pub(crate) orderby: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Relevant storefront, possible values - 'azure, amp, appsource'. Default='azure'"]
            pub fn storefront(mut self, storefront: impl Into<String>) -> Self {
                self.storefront = Some(storefront.into());
                self
            }
            #[doc = "Language to search, ISO 639-1 two-letter code, possible values - 'en,cs,de,es,fr,hu,it,ja,ko,nl,pl,pt-br,pt-pt,ru,sv,tr,zh-hans,zh-hant'. Default is \"en\""]
            pub fn language(mut self, language: impl Into<String>) -> Self {
                self.language = Some(language.into());
                self
            }
            #[doc = "Product market value  (the response will include only products that can be sold in the specified market)\nPossible values can be found at https://docs.microsoft.com/en-us/azure/marketplace/marketplace-geo-availability-currencies. Example: 'US'"]
            pub fn market(mut self, market: impl Into<String>) -> Self {
                self.market = Some(market.into());
                self
            }
            #[doc = "Add hide key to retrieve preview items. Hidekeys are provided in 'multi' format. Example: HideKey[]=first"]
            pub fn hide_keys(mut self, hide_keys: Vec<String>) -> Self {
                self.hide_keys = hide_keys;
                self
            }
            #[doc = "Selects which properties to include in the results. Example: 'select=displayName'"]
            pub fn select(mut self, select: Vec<String>) -> Self {
                self.select = select;
                self
            }
            #[doc = "Filters the results, based on a Boolean condition. Example: 'filter=productType eq \"VirtualMachine\"'. Fields that can be filtered by are:\n- `displayName`\n- `popularity`\n- `categoryIds`\n- `industryIds`\n- `publisherId`\n- `uniqueProductId`\n- `productType`\n- `operatingSystems`\n- `pricingTypes`\n- `publisherDisplayName`\n- `longSummary`\n- `summary`\n- `linkedAddinsTypes`\n- `description`\n- `supportedProducts`\n- `applicableProducts`\n- `lastModifiedDateTime`\n- `plan.planId`\n- `plan.displayName`\n- `plan.cspState`\n- `plan.altStackReference`\n- `plan.stackType`\n- `plan.categoryIds`\n- `plan.hasProtectedArtifacts`\n- `plan.pricingTypes`\n- `plan.summary`\n- `plan.description`\n- `plan.skuId`\n- `plan.displayRank`\n- `plan.isPrivate`"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "Expands related entities inline. Example: 'expand=startingPrice'"]
            pub fn expand(mut self, expand: Vec<String>) -> Self {
                self.expand = expand;
                self
            }
            #[doc = "Ordering expression for the results using OData notation. Avoid using orderby unless essential as this may impact the latency of your request.  Example: 'orderby=displayName desc'.This API only supports ordering by a single field. Fields that can be ordered by are:\n- `lastModifiedDateTime`\n- `uniqueProductId`\n- `productType`\n- `displayName`\n- `publisherId`"]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/products", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2023-01-01-preview");
                        if let Some(storefront) = &this.storefront {
                            req.url_mut().query_pairs_mut().append_pair("storefront", storefront);
                        }
                        if let Some(language) = &this.language {
                            req.url_mut().query_pairs_mut().append_pair("language", language);
                        }
                        if let Some(market) = &this.market {
                            req.url_mut().query_pairs_mut().append_pair("market", market);
                        }
                        let hide_keys = &this.hide_keys;
                        for value in &this.hide_keys {
                            req.url_mut().query_pairs_mut().append_pair("hideKeys", &value.to_string());
                        }
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("filter", filter);
                        }
                        if let Some(orderby) = &this.orderby {
                            req.url_mut().query_pairs_mut().append_pair("orderby", orderby);
                        }
                        req.insert_header("x-api-key", &this.x_api_key);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::CatalogApiResponse>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::CatalogApiResponse>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod facets {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Public Facets API provides an unauthenticated endpoint for consuming list of Marketplace public products counts per requested properties. In addition, it enables filtering on selected product\u{202f}properties, search, and getting product details. Each API request must include an API key allocated to you (see X-API-Key description). Send an email to MKPL_Platform_API_DL@microsoft.com including the following details to get the API Key:\n- Customer name\n- Service/Product name\n- AD Registered App ID (if available)\n- Focal point email\n- Use case/scenario\n- Expected traffic volume, including peak requests per second (daily)"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `language`: Language to search, ISO 639-1 two-letter code, possible values - 'en,cs,de,es,fr,hu,it,ja,ko,nl,pl,pt-br,pt-pt,ru,sv,tr,zh-hans,zh-hant'"]
        #[doc = "* `market`: Product sold market, Possible values - https://docs.microsoft.com/en-us/azure/marketplace/marketplace-geo-availability-currencies or 'All' for all markets. Such as 'US'"]
        #[doc = "* `facets`: Array facets to facet by, if none provided then no facets will return. Such as facets=PricingTypes,operatingSystems. \n- `SupportedProducts`: Supported Products. \n- `PublisherType`: Publisher Type.\n- `AzureBenefit`: Azure Benefit.\n- `ProductType`: Product Type.\n- `OperatingSystems`: Operating Systems.\n- `PricingTypes`: Pricing Types.\n- `VmImageGenerations`: Vm Image Generations.\n- `VmSecurityTypes`: Vm Security Types.\n- `VmArchitectureTypes`: Vm Architecture Types.\n- `RatingBuckets`: Rating Buckets."]
        pub fn get(&self, language: impl Into<String>, market: impl Into<String>, facets: Vec<String>) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                language: language.into(),
                market: market.into(),
                facets,
                search_query: None,
                publisher_display_name: None,
                azure_benefit: None,
                publisher_types: Vec::new(),
                badges: Vec::new(),
                industry_cloud: None,
                product_types: Vec::new(),
                pricing_types: Vec::new(),
                operating_systems: Vec::new(),
                hide_keys: Vec::new(),
                supported_products: Vec::new(),
                publisher_ids: Vec::new(),
                rating_buckets: Vec::new(),
                vm_architecture_types: Vec::new(),
                vm_security_types: Vec::new(),
                publishing_stage: None,
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::FacetsResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::FacetsResponse = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) language: String,
            pub(crate) market: String,
            pub(crate) facets: Vec<String>,
            pub(crate) search_query: Option<String>,
            pub(crate) publisher_display_name: Option<String>,
            pub(crate) azure_benefit: Option<String>,
            pub(crate) publisher_types: Vec<String>,
            pub(crate) badges: Vec<String>,
            pub(crate) industry_cloud: Option<String>,
            pub(crate) product_types: Vec<String>,
            pub(crate) pricing_types: Vec<String>,
            pub(crate) operating_systems: Vec<String>,
            pub(crate) hide_keys: Vec<String>,
            pub(crate) supported_products: Vec<String>,
            pub(crate) publisher_ids: Vec<String>,
            pub(crate) rating_buckets: Vec<String>,
            pub(crate) vm_architecture_types: Vec<String>,
            pub(crate) vm_security_types: Vec<String>,
            pub(crate) publishing_stage: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The search text, if no value provided, this filter is ignored. Such as searchQuery=red hat"]
            pub fn search_query(mut self, search_query: impl Into<String>) -> Self {
                self.search_query = Some(search_query.into());
                self
            }
            #[doc = "The search publisher name. Such as publisherDisplayName=red hat. If no value provided, this filter is ignored."]
            pub fn publisher_display_name(mut self, publisher_display_name: impl Into<String>) -> Self {
                self.publisher_display_name = Some(publisher_display_name.into());
                self
            }
            #[doc = "Define the search for only azure benefit eligible offers, if no value provided, this filter is ignored. Default: null. \n- `Eligible`: Eligible.\n- `NotEligible`: Not Eligible."]
            pub fn azure_benefit(mut self, azure_benefit: impl Into<String>) -> Self {
                self.azure_benefit = Some(azure_benefit.into());
                self
            }
            #[doc = "Products that contains any of the given publisher types, If no value provided, this filter is ignored. Possible values: Microsoft, ThirdParty. Default: Microsoft. \n- `Microsoft`: Microsoft.\n- `ThirdParty`: Third Party."]
            pub fn publisher_types(mut self, publisher_types: Vec<String>) -> Self {
                self.publisher_types = publisher_types;
                self
            }
            #[doc = "Products that contains any of the given badges. If no value provided, this filter is ignored. The following product badges are available: \n- `PreferredSolution`: Preferred Solution.\n- `PowerBICertified`: power BI Certified.\n- `AdditionalPurchaseRequirement`: Additional Purchase Requirement."]
            pub fn badges(mut self, badges: Vec<String>) -> Self {
                self.badges = badges;
                self
            }
            #[doc = "Define the search for IndustryCloud offers. If no value provided, this filter is ignored. Default: NotApplicable. \n- `NotApplicable`: Not Applicable.\n- `True`: True.\n- `False`: False."]
            pub fn industry_cloud(mut self, industry_cloud: impl Into<String>) -> Self {
                self.industry_cloud = Some(industry_cloud.into());
                self
            }
            #[doc = "Products that contains any of the given product types, Such as 'VirtualMachine'. If no value provided, this filter is ignored.\n- `AADApps`: Azure Active Directory Apps.\n- `AzureApplication`: Azure Application.\n- `AzureServices`: Azure Services.\n- `ConsultingServices`: Consulting Services.\n- `Container`: Container.\n- `ContainerApps`: Container Apps.\n- `CoreVirtualMachine`: Core Virtual Machine.\n- `CosellOnly`: Cosell Only.\n- `DevService`: Dev Service.\n- `DynamicsBC`: Dynamics BC.\n- `DynamicsCE`: Dynamics CE.\n- `DynamicsOps`: Dynamics Ops.\n- `IotEdgeModules`: Iot Edge Modules.\n- `ManagedApplication`: Managed Application.\n- `ManagedServices`: Managed Services.\n- `Office365`: Office365.\n- `PowerBI`: PowerBI.\n- `PowerBIVisuals`: PowerBI Visuals.\n- `SaaS`: SaaS.\n- `SolutionTemplate`: Solution Template.\n- `VirtualMachine`: Virtual Machine.\n- `VisualStudioExtension`: Visual Studio Extension.\n- `AppService`: App Service.\n- `LogAnalytics`: Log Analytics."]
            pub fn product_types(mut self, product_types: Vec<String>) -> Self {
                self.product_types = product_types;
                self
            }
            #[doc = "Products that contains any of the given pricing types. If no value provided, this filter is ignored. \n- `Byol`: The product has at least one plan that is bring your own license.\n- `Free`: The product has at least one plan that is free of charge.\n- `FreeTrial`: The product has at least one plan that is free trial.\n- `Payg`: The product has at least one plan that is Pay as you go, usage based billing model.\n- `RI`: The product has at least one plan that is Reserved Instance billing model."]
            pub fn pricing_types(mut self, pricing_types: Vec<String>) -> Self {
                self.pricing_types = pricing_types;
                self
            }
            #[doc = "Products that contains any of the given operating systems, Such as operatingSystems=windows,linux. If no value provided, this filter is ignored. This filter is relevant for Virtual Machine product type only.\n- `windows.windowsserver2019`: Windows Server 2019.\n- `windows.windowsserver2019`: Windows Server 2019.\n- `windows.windowsserver2016`: Windows Server 2016.\n- `windows.windowsserver2012r2`: Windows Server 2012 R2.\n- `windows.windowsserver2012`: Windows Server 2012.\n- `windows.windowsserver2008r2`: Windows Server 2008 R2.\n- `windows.others`: Others (windows).\n- `windows.windowsserver2022`: Windows Server 2022.\n- `linux.centos`: Cent OS.\n- `linux.debian`: Debian.\n- `linux.redhat`: Red Hat.\n- `linux.suse`: SUSE.\n- `linux.ubuntu`: Ubuntu.\n- `linux.others`: Others (Linux)."]
            pub fn operating_systems(mut self, operating_systems: Vec<String>) -> Self {
                self.operating_systems = operating_systems;
                self
            }
            #[doc = "Products that contains any of the given hideKeys for preview. Such as '22c6b3ae-1111-1111-1111-e7cbdc8569dd'. If no value provided, this filter is ignored. This filter is applied only when PublishingStage filter is set to Preview."]
            pub fn hide_keys(mut self, hide_keys: Vec<String>) -> Self {
                self.hide_keys = hide_keys;
                self
            }
            #[doc = "Products that contains any of the given supported products. Such as 'CRM'. If no value provided, this filter is ignored."]
            pub fn supported_products(mut self, supported_products: Vec<String>) -> Self {
                self.supported_products = supported_products;
                self
            }
            #[doc = "Products that contains any of the given publisher ids. Such as 'Microsoft'. If no value provided, this filter is ignored."]
            pub fn publisher_ids(mut self, publisher_ids: Vec<String>) -> Self {
                self.publisher_ids = publisher_ids;
                self
            }
            #[doc = "Products that contains any of the given rating buckets. Such as 'Above1'. If no value provided, this filter is ignored. \n- `AboveOne`: Above One.\n- `AboveTwo`: Above Two.\n- `AboveThree`: Above Three.\n- `AboveFour`: Above Four"]
            pub fn rating_buckets(mut self, rating_buckets: Vec<String>) -> Self {
                self.rating_buckets = rating_buckets;
                self
            }
            #[doc = "Array of Virtual Machine image architecture types to search by, If no value provided, this filter is ignored. see https://docs.microsoft.com/en-us/azure/virtual-machines/generation-2: \n - `X64Gen1`: X64 Generation 1.\n - `X64Gen2`: X64 Generation 2.\n - `Arm64`: Arm64 image architecture."]
            pub fn vm_architecture_types(mut self, vm_architecture_types: Vec<String>) -> Self {
                self.vm_architecture_types = vm_architecture_types;
                self
            }
            #[doc = "Products that contains any of the given vm security types. Such as 'Trusted'. If no value provided, this filter is ignored. \n- `None`: None.\n- `Trusted`: Trusted.\n- `Confidential`: Confidential"]
            pub fn vm_security_types(mut self, vm_security_types: Vec<String>) -> Self {
                self.vm_security_types = vm_security_types;
                self
            }
            #[doc = "Audience. Default: Public.\n- `Preview`: Preview.\n- `Public`: Public"]
            pub fn publishing_stage(mut self, publishing_stage: impl Into<String>) -> Self {
                self.publishing_stage = Some(publishing_stage.into());
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/facets", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2023-01-01-preview");
                        if let Some(search_query) = &this.search_query {
                            req.url_mut().query_pairs_mut().append_pair("searchQuery", search_query);
                        }
                        if let Some(publisher_display_name) = &this.publisher_display_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("publisherDisplayName", publisher_display_name);
                        }
                        if let Some(azure_benefit) = &this.azure_benefit {
                            req.url_mut().query_pairs_mut().append_pair("azureBenefit", azure_benefit);
                        }
                        if let Some(industry_cloud) = &this.industry_cloud {
                            req.url_mut().query_pairs_mut().append_pair("industryCloud", industry_cloud);
                        }
                        let language = &this.language;
                        req.url_mut().query_pairs_mut().append_pair("language", language);
                        let market = &this.market;
                        req.url_mut().query_pairs_mut().append_pair("market", market);
                        if let Some(publishing_stage) = &this.publishing_stage {
                            req.url_mut().query_pairs_mut().append_pair("publishingStage", publishing_stage);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::FacetsResponse>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::FacetsResponse>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod search {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Public Search API provides an unauthenticated endpoint for consuming list of Marketplace public products, total count of products returned, and facets per requested properties. In addition, it enables filtering on selected product\u{202f}properties, search, and getting product details. Each API request must include an API key allocated to you (see X-API-Key description). Send an email to MKPL_Platform_API_DL@microsoft.com including the following details to get the API Key:\n- Customer name\n- Service/Product name\n- AD Registered App ID (if available)\n- Focal point email\n- Use case/scenario\n- Expected traffic volume, including peak requests per second (daily)"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `language`: Language to search, ISO 639-1 two-letter code, possible values - 'en,cs,de,es,fr,hu,it,ja,ko,nl,pl,pt-br,pt-pt,ru,sv,tr,zh-hans,zh-hant'"]
        #[doc = "* `select`: Array of fields to return. Such as select=pricingTypes. \n- `All`: All fields.\n- `SupportedProducts`: Supported Products.\n- `PublisherId`: Publisher Id.\n- `DisplayName`: Display Name.\n- `AzureBenefit`: Azure Benefit.\n- `Badges`: Badges.\n- `SmallIconUri`: Small Icon Uri.\n- `PublisherType`: Publisher Type.\n- `PublishingState`: Publishing State.\n- `UniqueProductId`: Unique Product Id.\n- `ProductType`: Product Type.\n- `Plans`: Plans.\n- `OperatingSystems`: Operating Systems.\n- `PricingTypes`: Pricing Types.\n- `PublisherDisplayName`: Publisher Display Name.\n- `Summary`: Summary.\n- `Description`: Description.\n- `RatingBuckets`: Rating Buckets.\n- `RatingAverage`: Rating Average.\n- `LastModifiedDateTime`: Last Modified time.\n- `RatingCount`: Rating Count.\n- `LongSummary`: Long Summary."]
        #[doc = "* `market`: Product sold market, Possible values - https://docs.microsoft.com/en-us/azure/marketplace/marketplace-geo-availability-currencies or 'All' for all markets. Such as 'US'"]
        pub fn get(&self, language: impl Into<String>, select: Vec<String>, market: impl Into<String>) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                language: language.into(),
                select,
                market: market.into(),
                search_query: None,
                publisher_display_name: None,
                azure_benefit: None,
                publisher_types: Vec::new(),
                badges: Vec::new(),
                industry_cloud: None,
                order_by: Vec::new(),
                product_types: Vec::new(),
                pricing_types: Vec::new(),
                operating_systems: Vec::new(),
                hide_keys: Vec::new(),
                supported_products: Vec::new(),
                publisher_ids: Vec::new(),
                rating_buckets: Vec::new(),
                vm_architecture_types: Vec::new(),
                vm_security_types: Vec::new(),
                publishing_stage: None,
                facets: Vec::new(),
                skip: None,
                top: None,
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SearchResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SearchResponse = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) language: String,
            pub(crate) select: Vec<String>,
            pub(crate) market: String,
            pub(crate) search_query: Option<String>,
            pub(crate) publisher_display_name: Option<String>,
            pub(crate) azure_benefit: Option<String>,
            pub(crate) publisher_types: Vec<String>,
            pub(crate) badges: Vec<String>,
            pub(crate) industry_cloud: Option<String>,
            pub(crate) order_by: Vec<String>,
            pub(crate) product_types: Vec<String>,
            pub(crate) pricing_types: Vec<String>,
            pub(crate) operating_systems: Vec<String>,
            pub(crate) hide_keys: Vec<String>,
            pub(crate) supported_products: Vec<String>,
            pub(crate) publisher_ids: Vec<String>,
            pub(crate) rating_buckets: Vec<String>,
            pub(crate) vm_architecture_types: Vec<String>,
            pub(crate) vm_security_types: Vec<String>,
            pub(crate) publishing_stage: Option<String>,
            pub(crate) facets: Vec<String>,
            pub(crate) skip: Option<i32>,
            pub(crate) top: Option<i32>,
        }
        impl RequestBuilder {
            #[doc = "The search text, if no value provided, this filter is ignored. Such as searchQuery=red hat"]
            pub fn search_query(mut self, search_query: impl Into<String>) -> Self {
                self.search_query = Some(search_query.into());
                self
            }
            #[doc = "The search publisher name. Such as publisherDisplayName=red hat. If no value provided, this filter is ignored."]
            pub fn publisher_display_name(mut self, publisher_display_name: impl Into<String>) -> Self {
                self.publisher_display_name = Some(publisher_display_name.into());
                self
            }
            #[doc = "Define the search for only azure benefit eligible offers, if no value provided, this filter is ignored. Default: null. \n- `Eligible`: Eligible.\n- `NotEligible`: Not Eligible."]
            pub fn azure_benefit(mut self, azure_benefit: impl Into<String>) -> Self {
                self.azure_benefit = Some(azure_benefit.into());
                self
            }
            #[doc = "Products that contains any of the given publisher types, If no value provided, this filter is ignored. Possible values: Microsoft, ThirdParty. Default: Microsoft. \n- `Microsoft`: Microsoft.\n- `ThirdParty`: Third Party."]
            pub fn publisher_types(mut self, publisher_types: Vec<String>) -> Self {
                self.publisher_types = publisher_types;
                self
            }
            #[doc = "Products that contains any of the given badges. If no value provided, this filter is ignored. The following product badges are available: \n- `PreferredSolution`: Preferred Solution.\n- `PowerBICertified`: power BI Certified.\n- `AdditionalPurchaseRequirement`: Additional Purchase Requirement."]
            pub fn badges(mut self, badges: Vec<String>) -> Self {
                self.badges = badges;
                self
            }
            #[doc = "Define the search for IndustryCloud offers. If no value provided, this filter is ignored. Default: NotApplicable. \n- `NotApplicable`: Not Applicable.\n- `True`: True.\n- `False`: False."]
            pub fn industry_cloud(mut self, industry_cloud: impl Into<String>) -> Self {
                self.industry_cloud = Some(industry_cloud.into());
                self
            }
            #[doc = "Array of sort by fields to order by and ordering type (asc\\desc), If no value provided, ordered by search score. Default ordering type: asc. For example, orderby=RatingCount desc. \n- `RatingCount`: ratingCount (asc/desc).\n- `LastModifiedDateTime`: lastModifiedDateTime (asc/desc).\n- `RatingAverage`: RatingAverage (asc/desc)."]
            pub fn order_by(mut self, order_by: Vec<String>) -> Self {
                self.order_by = order_by;
                self
            }
            #[doc = "Products that contains any of the given product types, Such as 'VirtualMachine'. If no value provided, this filter is ignored.\n- `AADApps`: Azure Active Directory Apps.\n- `AzureApplication`: Azure Application.\n- `AzureServices`: Azure Services.\n- `ConsultingServices`: Consulting Services.\n- `Container`: Container.\n- `ContainerApps`: Container Apps.\n- `CoreVirtualMachine`: Core Virtual Machine.\n- `CosellOnly`: Cosell Only.\n- `DevService`: Dev Service.\n- `DynamicsBC`: Dynamics BC.\n- `DynamicsCE`: Dynamics CE.\n- `DynamicsOps`: Dynamics Ops.\n- `IotEdgeModules`: Iot Edge Modules.\n- `ManagedApplication`: Managed Application.\n- `ManagedServices`: Managed Services.\n- `Office365`: Office365.\n- `PowerBI`: PowerBI.\n- `PowerBIVisuals`: PowerBI Visuals.\n- `SaaS`: SaaS.\n- `SolutionTemplate`: Solution Template.\n- `VirtualMachine`: Virtual Machine.\n- `VisualStudioExtension`: Visual Studio Extension.\n- `AppService`: App Service.\n- `LogAnalytics`: Log Analytics."]
            pub fn product_types(mut self, product_types: Vec<String>) -> Self {
                self.product_types = product_types;
                self
            }
            #[doc = "Products that contains any of the given pricing types. If no value provided, this filter is ignored. \n- `Byol`: The product has at least one plan that is bring your own license.\n- `Free`: The product has at least one plan that is free of charge.\n- `FreeTrial`: The product has at least one plan that is free trial.\n- `Payg`: The product has at least one plan that is Pay as you go, usage based billing model.\n- `RI`: The product has at least one plan that is Reserved Instance billing model."]
            pub fn pricing_types(mut self, pricing_types: Vec<String>) -> Self {
                self.pricing_types = pricing_types;
                self
            }
            #[doc = "Products that contains any of the given operating systems, Such as operatingSystems=windows,linux. If no value provided, this filter is ignored. This filter is relevant for Virtual Machine product type only.\n- `windows.windowsserver2019`: Windows Server 2019.\n- `windows.windowsserver2019`: Windows Server 2019.\n- `windows.windowsserver2016`: Windows Server 2016.\n- `windows.windowsserver2012r2`: Windows Server 2012 R2.\n- `windows.windowsserver2012`: Windows Server 2012.\n- `windows.windowsserver2008r2`: Windows Server 2008 R2.\n- `windows.others`: Others (windows).\n- `windows.windowsserver2022`: Windows Server 2022.\n- `linux.centos`: Cent OS.\n- `linux.debian`: Debian.\n- `linux.redhat`: Red Hat.\n- `linux.suse`: SUSE.\n- `linux.ubuntu`: Ubuntu.\n- `linux.others`: Others (Linux)."]
            pub fn operating_systems(mut self, operating_systems: Vec<String>) -> Self {
                self.operating_systems = operating_systems;
                self
            }
            #[doc = "Products that contains any of the given hideKeys for preview. Such as '22c6b3ae-1111-1111-1111-e7cbdc8569dd'. If no value provided, this filter is ignored. This filter is applied only when PublishingStage filter is set to Preview."]
            pub fn hide_keys(mut self, hide_keys: Vec<String>) -> Self {
                self.hide_keys = hide_keys;
                self
            }
            #[doc = "Products that contains any of the given supported products. Such as 'CRM'. If no value provided, this filter is ignored."]
            pub fn supported_products(mut self, supported_products: Vec<String>) -> Self {
                self.supported_products = supported_products;
                self
            }
            #[doc = "Products that contains any of the given publisher ids. Such as 'Microsoft'. If no value provided, this filter is ignored."]
            pub fn publisher_ids(mut self, publisher_ids: Vec<String>) -> Self {
                self.publisher_ids = publisher_ids;
                self
            }
            #[doc = "Products that contains any of the given rating buckets. Such as 'Above1'. If no value provided, this filter is ignored. \n- `AboveOne`: Above One.\n- `AboveTwo`: Above Two.\n- `AboveThree`: Above Three.\n- `AboveFour`: Above Four"]
            pub fn rating_buckets(mut self, rating_buckets: Vec<String>) -> Self {
                self.rating_buckets = rating_buckets;
                self
            }
            #[doc = "Array of Virtual Machine image architecture types to search by, If no value provided, this filter is ignored. see https://docs.microsoft.com/en-us/azure/virtual-machines/generation-2: \n - `X64Gen1`: X64 Generation 1.\n - `X64Gen2`: X64 Generation 2.\n - `Arm64`: Arm64 image architecture."]
            pub fn vm_architecture_types(mut self, vm_architecture_types: Vec<String>) -> Self {
                self.vm_architecture_types = vm_architecture_types;
                self
            }
            #[doc = "Products that contains any of the given vm security types. Such as 'Trusted'. If no value provided, this filter is ignored. \n- `None`: None.\n- `Trusted`: Trusted.\n- `Confidential`: Confidential"]
            pub fn vm_security_types(mut self, vm_security_types: Vec<String>) -> Self {
                self.vm_security_types = vm_security_types;
                self
            }
            #[doc = "Audience. Default: Public.\n- `Preview`: Preview.\n- `Public`: Public"]
            pub fn publishing_stage(mut self, publishing_stage: impl Into<String>) -> Self {
                self.publishing_stage = Some(publishing_stage.into());
                self
            }
            #[doc = "Array facets to facet by, if none provided then no facets will return. Such as facets=PricingTypes,operatingSystems. \n- `SupportedProducts`: Supported Products. \n- `PublisherType`: Publisher Type.\n- `AzureBenefit`: Azure Benefit.\n- `ProductType`: Product Type.\n- `OperatingSystems`: Operating Systems.\n- `PricingTypes`: Pricing Types.\n- `VmImageGenerations`: Vm Image Generations.\n- `VmSecurityTypes`: Vm Security Types.\n- `VmArchitectureTypes`: Vm Architecture Types.\n- `RatingBuckets`: Rating Buckets."]
            pub fn facets(mut self, facets: Vec<String>) -> Self {
                self.facets = facets;
                self
            }
            #[doc = "Number of items to skip. Minimum: 0, Maximum: 100000, Default : 0."]
            pub fn skip(mut self, skip: i32) -> Self {
                self.skip = Some(skip);
                self
            }
            #[doc = "Number of items to return. Minimum: 0, Maximum: 1000, Default: 20."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/search", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2023-01-01-preview");
                        if let Some(search_query) = &this.search_query {
                            req.url_mut().query_pairs_mut().append_pair("searchQuery", search_query);
                        }
                        if let Some(publisher_display_name) = &this.publisher_display_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("publisherDisplayName", publisher_display_name);
                        }
                        if let Some(azure_benefit) = &this.azure_benefit {
                            req.url_mut().query_pairs_mut().append_pair("azureBenefit", azure_benefit);
                        }
                        if let Some(industry_cloud) = &this.industry_cloud {
                            req.url_mut().query_pairs_mut().append_pair("industryCloud", industry_cloud);
                        }
                        let language = &this.language;
                        req.url_mut().query_pairs_mut().append_pair("language", language);
                        let market = &this.market;
                        req.url_mut().query_pairs_mut().append_pair("market", market);
                        if let Some(publishing_stage) = &this.publishing_stage {
                            req.url_mut().query_pairs_mut().append_pair("publishingStage", publishing_stage);
                        }
                        if let Some(skip) = &this.skip {
                            req.url_mut().query_pairs_mut().append_pair("skip", &skip.to_string());
                        }
                        if let Some(top) = &this.top {
                            req.url_mut().query_pairs_mut().append_pair("top", &top.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SearchResponse>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SearchResponse>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
pub mod suggestions {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Public Suggestions API provides an unauthenticated endpoint for consuming list of Marketplace public products suggestions per requested properties. In addition, it enables filtering on selected product\u{202f}properties, and getting product details. Each API request must include an API key allocated to you (see X-API-Key description). Send an email to MKPL_Platform_API_DL@microsoft.com including the following details to get the API Key:\n- Customer name\n- Service/Product name\n- AD Registered App ID (if available)\n- Focal point email\n- Use case/scenario\n- Expected traffic volume, including peak requests per second (daily)"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `search_query`: The search text, if no value provided, this filter is ignored. Such as searchQuery=red hat"]
        #[doc = "* `language`: Language to search, ISO 639-1 two-letter code, possible values - 'en,cs,de,es,fr,hu,it,ja,ko,nl,pl,pt-br,pt-pt,ru,sv,tr,zh-hans,zh-hant'"]
        #[doc = "* `suggestion_types`: Suggestion types. \n- `WordSearch`: WordSearch.\n- `Entity`: Entity."]
        pub fn get_products(
            &self,
            search_query: impl Into<String>,
            language: impl Into<String>,
            suggestion_types: Vec<String>,
        ) -> get_products::RequestBuilder {
            get_products::RequestBuilder {
                client: self.0.clone(),
                search_query: search_query.into(),
                language: language.into(),
                suggestion_types,
                publisher_display_name: None,
                azure_benefit: None,
                publisher_types: Vec::new(),
                badges: Vec::new(),
                industry_cloud: None,
                product_types: Vec::new(),
                pricing_types: Vec::new(),
                operating_systems: Vec::new(),
                supported_products: Vec::new(),
                publisher_ids: Vec::new(),
                rating_buckets: Vec::new(),
                vm_architecture_types: Vec::new(),
                vm_security_types: Vec::new(),
                top: None,
                select: Vec::new(),
            }
        }
    }
    pub mod get_products {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SuggestionsResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SuggestionsResponse = serde_json::from_slice(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
        }
        impl From<Response> for azure_core::Response {
            fn from(rsp: Response) -> Self {
                rsp.into_raw_response()
            }
        }
        impl AsRef<azure_core::Response> for Response {
            fn as_ref(&self) -> &azure_core::Response {
                self.as_raw_response()
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" The building of a request is typically finalized by invoking `.await` on"]
        #[doc = r" `RequestBuilder`. This implicitly invokes the [`IntoFuture::into_future()`](#method.into_future)"]
        #[doc = r" method, which converts `RequestBuilder` into a future that executes the request"]
        #[doc = r" operation and returns a `Result` with the parsed response."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details (e.g. to inspect"]
        #[doc = r" response headers or raw body data) then you can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future that resolves to a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) search_query: String,
            pub(crate) language: String,
            pub(crate) suggestion_types: Vec<String>,
            pub(crate) publisher_display_name: Option<String>,
            pub(crate) azure_benefit: Option<String>,
            pub(crate) publisher_types: Vec<String>,
            pub(crate) badges: Vec<String>,
            pub(crate) industry_cloud: Option<String>,
            pub(crate) product_types: Vec<String>,
            pub(crate) pricing_types: Vec<String>,
            pub(crate) operating_systems: Vec<String>,
            pub(crate) supported_products: Vec<String>,
            pub(crate) publisher_ids: Vec<String>,
            pub(crate) rating_buckets: Vec<String>,
            pub(crate) vm_architecture_types: Vec<String>,
            pub(crate) vm_security_types: Vec<String>,
            pub(crate) top: Option<i32>,
            pub(crate) select: Vec<String>,
        }
        impl RequestBuilder {
            #[doc = "The search publisher name. Such as publisherDisplayName=red hat. If no value provided, this filter is ignored."]
            pub fn publisher_display_name(mut self, publisher_display_name: impl Into<String>) -> Self {
                self.publisher_display_name = Some(publisher_display_name.into());
                self
            }
            #[doc = "Define the search for only azure benefit eligible offers, if no value provided, this filter is ignored. Default: null. \n- `Eligible`: Eligible.\n- `NotEligible`: Not Eligible."]
            pub fn azure_benefit(mut self, azure_benefit: impl Into<String>) -> Self {
                self.azure_benefit = Some(azure_benefit.into());
                self
            }
            #[doc = "Products that contains any of the given publisher types, If no value provided, this filter is ignored. Possible values: Microsoft, ThirdParty. Default: Microsoft. \n- `Microsoft`: Microsoft.\n- `ThirdParty`: Third Party."]
            pub fn publisher_types(mut self, publisher_types: Vec<String>) -> Self {
                self.publisher_types = publisher_types;
                self
            }
            #[doc = "Products that contains any of the given badges. If no value provided, this filter is ignored. The following product badges are available: \n- `PreferredSolution`: Preferred Solution.\n- `PowerBICertified`: power BI Certified.\n- `AdditionalPurchaseRequirement`: Additional Purchase Requirement."]
            pub fn badges(mut self, badges: Vec<String>) -> Self {
                self.badges = badges;
                self
            }
            #[doc = "Define the search for IndustryCloud offers. If no value provided, this filter is ignored. Default: NotApplicable. \n- `NotApplicable`: Not Applicable.\n- `True`: True.\n- `False`: False."]
            pub fn industry_cloud(mut self, industry_cloud: impl Into<String>) -> Self {
                self.industry_cloud = Some(industry_cloud.into());
                self
            }
            #[doc = "Products that contains any of the given product types, Such as 'VirtualMachine'. If no value provided, this filter is ignored.\n- `AADApps`: Azure Active Directory Apps.\n- `AzureApplication`: Azure Application.\n- `AzureServices`: Azure Services.\n- `ConsultingServices`: Consulting Services.\n- `Container`: Container.\n- `ContainerApps`: Container Apps.\n- `CoreVirtualMachine`: Core Virtual Machine.\n- `CosellOnly`: Cosell Only.\n- `DevService`: Dev Service.\n- `DynamicsBC`: Dynamics BC.\n- `DynamicsCE`: Dynamics CE.\n- `DynamicsOps`: Dynamics Ops.\n- `IotEdgeModules`: Iot Edge Modules.\n- `ManagedApplication`: Managed Application.\n- `ManagedServices`: Managed Services.\n- `Office365`: Office365.\n- `PowerBI`: PowerBI.\n- `PowerBIVisuals`: PowerBI Visuals.\n- `SaaS`: SaaS.\n- `SolutionTemplate`: Solution Template.\n- `VirtualMachine`: Virtual Machine.\n- `VisualStudioExtension`: Visual Studio Extension.\n- `AppService`: App Service.\n- `LogAnalytics`: Log Analytics."]
            pub fn product_types(mut self, product_types: Vec<String>) -> Self {
                self.product_types = product_types;
                self
            }
            #[doc = "Products that contains any of the given pricing types. If no value provided, this filter is ignored. \n- `Byol`: The product has at least one plan that is bring your own license.\n- `Free`: The product has at least one plan that is free of charge.\n- `FreeTrial`: The product has at least one plan that is free trial.\n- `Payg`: The product has at least one plan that is Pay as you go, usage based billing model.\n- `RI`: The product has at least one plan that is Reserved Instance billing model."]
            pub fn pricing_types(mut self, pricing_types: Vec<String>) -> Self {
                self.pricing_types = pricing_types;
                self
            }
            #[doc = "Products that contains any of the given operating systems, Such as operatingSystems=windows,linux. If no value provided, this filter is ignored. This filter is relevant for Virtual Machine product type only.\n- `windows.windowsserver2019`: Windows Server 2019.\n- `windows.windowsserver2019`: Windows Server 2019.\n- `windows.windowsserver2016`: Windows Server 2016.\n- `windows.windowsserver2012r2`: Windows Server 2012 R2.\n- `windows.windowsserver2012`: Windows Server 2012.\n- `windows.windowsserver2008r2`: Windows Server 2008 R2.\n- `windows.others`: Others (windows).\n- `windows.windowsserver2022`: Windows Server 2022.\n- `linux.centos`: Cent OS.\n- `linux.debian`: Debian.\n- `linux.redhat`: Red Hat.\n- `linux.suse`: SUSE.\n- `linux.ubuntu`: Ubuntu.\n- `linux.others`: Others (Linux)."]
            pub fn operating_systems(mut self, operating_systems: Vec<String>) -> Self {
                self.operating_systems = operating_systems;
                self
            }
            #[doc = "Products that contains any of the given supported products. Such as 'CRM'. If no value provided, this filter is ignored."]
            pub fn supported_products(mut self, supported_products: Vec<String>) -> Self {
                self.supported_products = supported_products;
                self
            }
            #[doc = "Products that contains any of the given publisher ids. Such as 'Microsoft'. If no value provided, this filter is ignored."]
            pub fn publisher_ids(mut self, publisher_ids: Vec<String>) -> Self {
                self.publisher_ids = publisher_ids;
                self
            }
            #[doc = "Products that contains any of the given rating buckets. Such as 'Above1'. If no value provided, this filter is ignored. \n- `AboveOne`: Above One.\n- `AboveTwo`: Above Two.\n- `AboveThree`: Above Three.\n- `AboveFour`: Above Four"]
            pub fn rating_buckets(mut self, rating_buckets: Vec<String>) -> Self {
                self.rating_buckets = rating_buckets;
                self
            }
            #[doc = "Array of Virtual Machine image architecture types to search by, If no value provided, this filter is ignored. see https://docs.microsoft.com/en-us/azure/virtual-machines/generation-2: \n - `X64Gen1`: X64 Generation 1.\n - `X64Gen2`: X64 Generation 2.\n - `Arm64`: Arm64 image architecture."]
            pub fn vm_architecture_types(mut self, vm_architecture_types: Vec<String>) -> Self {
                self.vm_architecture_types = vm_architecture_types;
                self
            }
            #[doc = "Products that contains any of the given vm security types. Such as 'Trusted'. If no value provided, this filter is ignored. \n- `None`: None.\n- `Trusted`: Trusted.\n- `Confidential`: Confidential"]
            pub fn vm_security_types(mut self, vm_security_types: Vec<String>) -> Self {
                self.vm_security_types = vm_security_types;
                self
            }
            #[doc = "Number of items per SuggestionType to return. Minimum: 1, Maximum: 40, Default: 20."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Array of fields to return. Such as 'displayText'. If no value provided, return fields as follows. if suggestionsType=Entity, return: suggestionType, displayText, id. if suggestionType=WordSearch, return suggestionType and displayText. \n- `DisplayText`: Display text.\n- `Id`: Id.\n- `IconUrl`: Icon Url.\n- `ProductType`: Product type.\n- `LinkedAddInsTypes`: Linked AddIns types."]
            pub fn select(mut self, select: Vec<String>) -> Self {
                self.select = select;
                self
            }
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/suggestions/products", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2023-01-01-preview");
                        let search_query = &this.search_query;
                        req.url_mut().query_pairs_mut().append_pair("searchQuery", search_query);
                        if let Some(publisher_display_name) = &this.publisher_display_name {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("publisherDisplayName", publisher_display_name);
                        }
                        if let Some(azure_benefit) = &this.azure_benefit {
                            req.url_mut().query_pairs_mut().append_pair("azureBenefit", azure_benefit);
                        }
                        if let Some(industry_cloud) = &this.industry_cloud {
                            req.url_mut().query_pairs_mut().append_pair("industryCloud", industry_cloud);
                        }
                        let language = &this.language;
                        req.url_mut().query_pairs_mut().append_pair("language", language);
                        if let Some(top) = &this.top {
                            req.url_mut().query_pairs_mut().append_pair("top", &top.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SuggestionsResponse>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SuggestionsResponse>>;
            #[doc = "Returns a future that sends the request and returns the parsed response body."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move { self.send().await?.into_body().await })
            }
        }
    }
}
