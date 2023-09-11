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
pub const DEFAULT_ENDPOINT: &str = azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD;
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
    pub fn metrics_client(&self) -> metrics::Client {
        metrics::Client(self.clone())
    }
}
pub mod metrics {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists the metric values for multiple resources."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The subscription identifier for the resources in this batch."]
        #[doc = "* `metricnamespace`: Metric namespace that contains the requested metric names."]
        #[doc = "* `metricnames`: The names of the metrics (comma separated) to retrieve."]
        #[doc = "* `resource_ids`: The comma separated list of resource IDs to query metrics for."]
        pub fn batch(
            &self,
            subscription_id: impl Into<String>,
            metricnamespace: impl Into<String>,
            metricnames: Vec<String>,
            resource_ids: impl Into<models::ResourceIdList>,
        ) -> batch::RequestBuilder {
            batch::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                metricnamespace: metricnamespace.into(),
                metricnames,
                resource_ids: resource_ids.into(),
                starttime: None,
                endtime: None,
                interval: None,
                aggregation: None,
                top: None,
                orderby: None,
                filter: None,
            }
        }
    }
    pub mod batch {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::MetricResultsResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::MetricResultsResponse = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) metricnamespace: String,
            pub(crate) metricnames: Vec<String>,
            pub(crate) resource_ids: models::ResourceIdList,
            pub(crate) starttime: Option<String>,
            pub(crate) endtime: Option<String>,
            pub(crate) interval: Option<String>,
            pub(crate) aggregation: Option<String>,
            pub(crate) top: Option<i32>,
            pub(crate) orderby: Option<String>,
            pub(crate) filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The start time of the query. It is a string in the format 'yyyy-MM-ddTHH:mm:ss.fffZ'. If you have specified the endtime parameter, then this parameter is required.\nIf only starttime is specified, then endtime defaults to the current time.\nIf no time interval is specified, the default is 1 hour."]
            pub fn starttime(mut self, starttime: impl Into<String>) -> Self {
                self.starttime = Some(starttime.into());
                self
            }
            #[doc = "The end time of the query. It is a string in the format 'yyyy-MM-ddTHH:mm:ss.fffZ'."]
            pub fn endtime(mut self, endtime: impl Into<String>) -> Self {
                self.endtime = Some(endtime.into());
                self
            }
            #[doc = "The interval (i.e. timegrain) of the query.\n*Examples: PT15M, PT1H, P1D*"]
            pub fn interval(mut self, interval: impl Into<String>) -> Self {
                self.interval = Some(interval.into());
                self
            }
            #[doc = "The list of aggregation types (comma separated) to retrieve.\n*Examples: average, minimum, maximum*"]
            pub fn aggregation(mut self, aggregation: impl Into<String>) -> Self {
                self.aggregation = Some(aggregation.into());
                self
            }
            #[doc = "The maximum number of records to retrieve per resource ID in the request.\nValid only if filter is specified.\nDefaults to 10."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "The aggregation to use for sorting results and the direction of the sort.\nOnly one order can be specified.\n*Examples: sum asc*"]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "The filter is used to reduce the set of metric data returned.<br>Example:<br>Metric contains metadata A, B and C.<br>- Return all time series of C where A = a1 and B = b1 or b2<br>**filter=A eq ‘a1’ and B eq ‘b1’ or B eq ‘b2’ and C eq ‘*’**<br>- Invalid variant:<br>**filter=A eq ‘a1’ and B eq ‘b1’ and C eq ‘*’ or B = ‘b2’**<br>This is invalid because the logical or operator cannot separate two different metadata names.<br>- Return all time series where A = a1, B = b1 and C = c1:<br>**filter=A eq ‘a1’ and B eq ‘b1’ and C eq ‘c1’**<br>- Return all time series where A = a1<br>**filter=A eq ‘a1’ and B eq ‘*’ and C eq ‘*’**."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
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
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/metrics:getBatch",
                            this.client.endpoint(),
                            &this.subscription_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2023-05-01-preview");
                        if let Some(starttime) = &this.starttime {
                            req.url_mut().query_pairs_mut().append_pair("starttime", starttime);
                        }
                        if let Some(endtime) = &this.endtime {
                            req.url_mut().query_pairs_mut().append_pair("endtime", endtime);
                        }
                        if let Some(interval) = &this.interval {
                            req.url_mut().query_pairs_mut().append_pair("interval", interval);
                        }
                        let metricnamespace = &this.metricnamespace;
                        req.url_mut().query_pairs_mut().append_pair("metricnamespace", metricnamespace);
                        if let Some(aggregation) = &this.aggregation {
                            req.url_mut().query_pairs_mut().append_pair("aggregation", aggregation);
                        }
                        if let Some(top) = &this.top {
                            req.url_mut().query_pairs_mut().append_pair("top", &top.to_string());
                        }
                        if let Some(orderby) = &this.orderby {
                            req.url_mut().query_pairs_mut().append_pair("orderby", orderby);
                        }
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("filter", filter);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.resource_ids)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::MetricResultsResponse>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::MetricResultsResponse>>;
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
