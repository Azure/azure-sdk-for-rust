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
    pub fn products_client(&self) -> products::Client {
        products::Client(self.clone())
    }
    pub fn skus_client(&self) -> skus::Client {
        skus::Client(self.clone())
    }
}
pub mod products {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Single Public Product API provides an unauthenticated endpoint for consuming product attributes of a single commercial 3rd party Marketplace public product or 1st party Azure product. Each API request must include an API key allocated to you. Send an email to MKPL_Platform_API_DL@microsoft.com including the following details to get the API Key:\n- Customer name\n- Service/Product name\n- AD Registered App ID (if available)\n- Focal point email\n- Use case/scenario\n- Expected traffic volume, including peak requests per second (daily)"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: The unique product Id or product id"]
        #[doc = "* `market`: Product market value  (the response will include only products that can be sold in the specified market)\nPossible values can be found at https://docs.microsoft.com/en-us/azure/marketplace/marketplace-geo-availability-currencies. Example: 'US'"]
        #[doc = "* `x_api_key`: Use Api key provided by Marketplace Catalog Team as a parameter provided in the header"]
        pub fn get(&self, id: impl Into<String>, market: impl Into<String>, x_api_key: impl Into<String>) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
                market: market.into(),
                x_api_key: x_api_key.into(),
                language: None,
                locations: Vec::new(),
                include_stop_sold_plans: None,
                hide_keys: Vec::new(),
                exclude_sku_details: None,
                include_future_availabilities: None,
                sku_id: None,
                x_ms_pricing_audience: None,
            }
        }
        #[doc = "Public Products API provides an unauthenticated endpoint for consuming commercial 3rd party Marketplace public products and 1st party Azure products. In addition, it enables Odata filtering on selected product properties.  Each API request must include an API key allocated to you (see X-API-Key description). Send an email to MKPL_Platform_API_DL@microsoft.com including the following details to get the API Key:\n- Customer name\n- Service/Product name\n- AD Registered App ID (if available)\n- Focal point email\n- Use case/scenario\n- Expected traffic volume, including peak requests per second (daily)"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `x_api_key`: Use Api key provided by Marketplace Catalog Team as a parameter provided in the header"]
        pub fn list(&self, x_api_key: impl Into<String>) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                x_api_key: x_api_key.into(),
                storefront: None,
                language: None,
                locations: Vec::new(),
                market: None,
                hide_keys: Vec::new(),
                select: Vec::new(),
                filter: None,
                expand: Vec::new(),
                orderby: None,
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
            pub(crate) locations: Vec<String>,
            pub(crate) include_stop_sold_plans: Option<bool>,
            pub(crate) hide_keys: Vec<String>,
            pub(crate) exclude_sku_details: Option<bool>,
            pub(crate) include_future_availabilities: Option<bool>,
            pub(crate) sku_id: Option<String>,
            pub(crate) x_ms_pricing_audience: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Language to search, ISO 639-1 two-letter code, possible values - 'en,cs,de,es,fr,hu,it,ja,ko,nl,pl,pt-br,pt-pt,ru,sv,tr,zh-hans,zh-hant'. Default is \"en\""]
            pub fn language(mut self, language: impl Into<String>) -> Self {
                self.language = Some(language.into());
                self
            }
            #[doc = "Return products available in selected location. Enumeration of the Azure datacenter regions. See https://azure.microsoft.com/regions/"]
            pub fn locations(mut self, locations: Vec<String>) -> Self {
                self.locations = locations;
                self
            }
            #[doc = "Indicates whether to include in the response the product's plans/SKUs that are no longer available for purchase. By default, includeStopSoldPlans is set to FALSE."]
            pub fn include_stop_sold_plans(mut self, include_stop_sold_plans: bool) -> Self {
                self.include_stop_sold_plans = Some(include_stop_sold_plans);
                self
            }
            #[doc = "This key is utilized to retrieve preview products and is generated within the Partner Center during the offer publishing process."]
            pub fn hide_keys(mut self, hide_keys: Vec<String>) -> Self {
                self.hide_keys = hide_keys;
                self
            }
            #[doc = "By default SKU details are included (excludeSkuDetails = FALSE ), to exclude SKU details in the response, set excludeSkuDetails to TRUE."]
            pub fn exclude_sku_details(mut self, exclude_sku_details: bool) -> Self {
                self.exclude_sku_details = Some(exclude_sku_details);
                self
            }
            #[doc = "Indicates whether to include the product's plans/SKU availabilities with future dates in the response. By default, includeFutureAvailabilities is set to FALSE."]
            pub fn include_future_availabilities(mut self, include_future_availabilities: bool) -> Self {
                self.include_future_availabilities = Some(include_future_availabilities);
                self
            }
            #[doc = "Apply SKU ID filtering to the results and only return products that include SKUs matching the selected SKU ID."]
            pub fn sku_id(mut self, sku_id: impl Into<String>) -> Self {
                self.sku_id = Some(sku_id.into());
                self
            }
            #[doc = "DirectCommercial or PartnerCommercial, DirectCommercial is the default"]
            pub fn x_ms_pricing_audience(mut self, x_ms_pricing_audience: impl Into<String>) -> Self {
                self.x_ms_pricing_audience = Some(x_ms_pricing_audience.into());
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
                            .append_pair(azure_core::query_param::API_VERSION, "2023-05-01-preview");
                        let market = &this.market;
                        req.url_mut().query_pairs_mut().append_pair("market", market);
                        if let Some(language) = &this.language {
                            req.url_mut().query_pairs_mut().append_pair("language", language);
                        }
                        let locations = &this.locations;
                        for value in &this.locations {
                            req.url_mut().query_pairs_mut().append_pair("locations", &value.to_string());
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
                        if let Some(exclude_sku_details) = &this.exclude_sku_details {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("excludeSkuDetails", &exclude_sku_details.to_string());
                        }
                        if let Some(include_future_availabilities) = &this.include_future_availabilities {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeFutureAvailabilities", &include_future_availabilities.to_string());
                        }
                        if let Some(sku_id) = &this.sku_id {
                            req.url_mut().query_pairs_mut().append_pair("skuId", sku_id);
                        }
                        if let Some(x_ms_pricing_audience) = &this.x_ms_pricing_audience {
                            req.insert_header("x-ms-pricing-audience", x_ms_pricing_audience);
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
            pub(crate) locations: Vec<String>,
            pub(crate) market: Option<String>,
            pub(crate) hide_keys: Vec<String>,
            pub(crate) select: Vec<String>,
            pub(crate) filter: Option<String>,
            pub(crate) expand: Vec<String>,
            pub(crate) orderby: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Return products available in Azure Portal Marketplace, AppSource or Azure Marketplace portal. The possible values are: \n- `azure` - Azure Portal Marketplace\n- `amp` - Azure Marketplace portal\n- `appsource` - AppSource portal\nThe default value is 'azure'"]
            pub fn storefront(mut self, storefront: impl Into<String>) -> Self {
                self.storefront = Some(storefront.into());
                self
            }
            #[doc = "Language to search, ISO 639-1 two-letter code, possible values - 'en,cs,de,es,fr,hu,it,ja,ko,nl,pl,pt-br,pt-pt,ru,sv,tr,zh-hans,zh-hant'. Default is \"en\""]
            pub fn language(mut self, language: impl Into<String>) -> Self {
                self.language = Some(language.into());
                self
            }
            #[doc = "Return products available in selected location. Enumeration of the Azure datacenter regions. See https://azure.microsoft.com/regions/"]
            pub fn locations(mut self, locations: Vec<String>) -> Self {
                self.locations = locations;
                self
            }
            #[doc = "Product market value  (the response will include only products that can be sold in the specified market)\nPossible values can be found at https://docs.microsoft.com/en-us/azure/marketplace/marketplace-geo-availability-currencies. Example: 'US'"]
            pub fn market(mut self, market: impl Into<String>) -> Self {
                self.market = Some(market.into());
                self
            }
            #[doc = "This key is utilized to retrieve preview products and is generated within the Partner Center during the offer publishing process."]
            pub fn hide_keys(mut self, hide_keys: Vec<String>) -> Self {
                self.hide_keys = hide_keys;
                self
            }
            #[doc = "Selects which properties to include in the results. Example: 'select=displayName'"]
            pub fn select(mut self, select: Vec<String>) -> Self {
                self.select = select;
                self
            }
            #[doc = "Filters the results, based on a Boolean condition. Example: 'filter=productType eq \"VirtualMachine\"'. Fields that can be filtered by are:\n- `displayName`\n- `productId`\n- `popularity`\n- `categoryIds`\n- `industryIds`\n- `publisherId`\n- `uniqueProductId`\n- `productType`\n- `operatingSystems`\n- `pricingTypes`\n- `publisherDisplayName`\n- `longSummary`\n- `summary`\n- `linkedAddinsTypes`\n- `description`\n- `supportedProducts`\n- `applicableProducts`\n- `lastModifiedDateTime`\n- `plan.planId`\n- `plan.displayName`\n- `plan.cspState`\n- `plan.altStackReference`\n- `plan.stackType`\n- `plan.categoryIds`\n- `plan.hasProtectedArtifacts`\n- `plan.pricingTypes`\n- `plan.summary`\n- `plan.description`\n- `plan.skuId`\n- `plan.displayRank`\n- `plan.isPrivate`"]
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
                            .append_pair(azure_core::query_param::API_VERSION, "2023-05-01-preview");
                        if let Some(storefront) = &this.storefront {
                            req.url_mut().query_pairs_mut().append_pair("storefront", storefront);
                        }
                        if let Some(language) = &this.language {
                            req.url_mut().query_pairs_mut().append_pair("language", language);
                        }
                        let locations = &this.locations;
                        for value in &this.locations {
                            req.url_mut().query_pairs_mut().append_pair("locations", &value.to_string());
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
pub mod skus {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Public Skus API provides an unauthenticated endpoint for consuming commercial 3rd party Marketplace public skus and 1st party Azure skus. In addition, it enables Odata filtering on selected sku properties or attributes. Each API request must include an API key allocated to you (see X-API-Key description). Send an email to MKPL_Platform_API_DL@microsoft.com including the following details to get the API Key:\n- Customer name\n- Service/Product name\n- AD Registered App ID (if available)\n- Focal point email\n- Use case/scenario\n- Expected traffic volume, including peak requests per second (daily)"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_family`: The service family of the product, such as 'Compute'"]
        #[doc = "* `service`: The service name of the product, such as 'Virtual Machines'"]
        #[doc = "* `x_api_key`: Use Api key provided by Marketplace Catalog Team as a parameter provided in the header"]
        pub fn list(
            &self,
            service_family: impl Into<String>,
            service: impl Into<String>,
            x_api_key: impl Into<String>,
        ) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                service_family: service_family.into(),
                service: service.into(),
                x_api_key: x_api_key.into(),
                language: None,
                locations: Vec::new(),
                market: None,
                select: Vec::new(),
                filter: None,
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
            pub async fn into_body(self) -> azure_core::Result<models::SkuApiResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SkuApiResponse = serde_json::from_slice(&bytes)?;
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
            pub(crate) service_family: String,
            pub(crate) service: String,
            pub(crate) x_api_key: String,
            pub(crate) language: Option<String>,
            pub(crate) locations: Vec<String>,
            pub(crate) market: Option<String>,
            pub(crate) select: Vec<String>,
            pub(crate) filter: Option<String>,
            pub(crate) orderby: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Language to search, ISO 639-1 two-letter code, possible values - 'en,cs,de,es,fr,hu,it,ja,ko,nl,pl,pt-br,pt-pt,ru,sv,tr,zh-hans,zh-hant'. Default is \"en\""]
            pub fn language(mut self, language: impl Into<String>) -> Self {
                self.language = Some(language.into());
                self
            }
            #[doc = "Return skus available in selected location. Enumeration of the Azure datacenter regions. See https://azure.microsoft.com/regions/"]
            pub fn locations(mut self, locations: Vec<String>) -> Self {
                self.locations = locations;
                self
            }
            #[doc = "Product market value  (the response will include only skus that can be sold in the specified market)\nPossible values can be found at https://docs.microsoft.com/en-us/azure/marketplace/marketplace-geo-availability-currencies. Example: 'US'"]
            pub fn market(mut self, market: impl Into<String>) -> Self {
                self.market = Some(market.into());
                self
            }
            #[doc = "Selects which properties to include in the results. Example: 'select=skuTitle'"]
            pub fn select(mut self, select: Vec<String>) -> Self {
                self.select = select;
                self
            }
            #[doc = "Filters the results, based on a Boolean condition. Example: 'filter=productType eq \"VirtualMachine\"'. Fields that can be filtered by are:\n- `productId`\n- `productDisplayName`\n- `productDescription`\n- `publisherId`\n- `publisherType`\n- `productType`\n- `productSubType`\n- `productAttributes`\n- `skuName`\n- `skuId`\n- `skuType`\n- `skuDescription`\n- `skuTitle`\n- `armRegionName`\n- `skuGroupId`\n- `cloud`\n- `locationType`\n- `region`\n- `lastModifiedDateTime`\n- `zone`\n- `serviceType`\n- `skuAttributes`\n- `skuProperties`\n- `offeringProperties`"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "Ordering expression for the results using OData notation. Avoid using orderby unless essential as this may impact the latency of your request.  Example: 'orderby=skuName desc'.This API only supports ordering by a single field. Fields that can be ordered by are:\n- `lastModifiedDateTime`\n- `publisherId`\n- `productType`\n- `skuName`\n- `skuType`"]
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
                        let url = azure_core::Url::parse(&format!("{}/skus", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2023-05-01-preview");
                        let service_family = &this.service_family;
                        req.url_mut().query_pairs_mut().append_pair("serviceFamily", service_family);
                        let service = &this.service;
                        req.url_mut().query_pairs_mut().append_pair("service", service);
                        if let Some(language) = &this.language {
                            req.url_mut().query_pairs_mut().append_pair("language", language);
                        }
                        let locations = &this.locations;
                        for value in &this.locations {
                            req.url_mut().query_pairs_mut().append_pair("locations", &value.to_string());
                        }
                        if let Some(market) = &this.market {
                            req.url_mut().query_pairs_mut().append_pair("market", market);
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
            type Output = azure_core::Result<models::SkuApiResponse>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SkuApiResponse>>;
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
