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
pub const DEFAULT_ENDPOINT: &str = "https://api.applicationinsights.io/v1";
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
    pub fn events_client(&self) -> events::Client {
        events::Client(self.clone())
    }
    pub fn metadata_client(&self) -> metadata::Client {
        metadata::Client(self.clone())
    }
    pub fn metrics_client(&self) -> metrics::Client {
        metrics::Client(self.clone())
    }
    pub fn query_client(&self) -> query::Client {
        query::Client(self.clone())
    }
}
pub mod metrics {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieve metric data"]
        #[doc = "Gets metric values for a single metric"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `app_id`: ID of the application. This is Application ID from the API Access settings blade in the Azure portal."]
        #[doc = "* `metric_id`: ID of the metric. This is either a standard AI metric, or an application-specific custom metric."]
        pub fn get(&self, app_id: impl Into<String>, metric_id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                app_id: app_id.into(),
                metric_id: metric_id.into(),
                timespan: None,
                interval: None,
                aggregation: Vec::new(),
                segment: Vec::new(),
                top: None,
                orderby: None,
                filter: None,
            }
        }
        #[doc = "Retrieve metric data"]
        #[doc = "Gets metric values for multiple metrics"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `app_id`: ID of the application. This is Application ID from the API Access settings blade in the Azure portal."]
        #[doc = "* `body`: The batched metrics query."]
        pub fn get_multiple(&self, app_id: impl Into<String>, body: impl Into<models::MetricsPostBody>) -> get_multiple::Builder {
            get_multiple::Builder {
                client: self.0.clone(),
                app_id: app_id.into(),
                body: body.into(),
            }
        }
        #[doc = "Retrieve metric metadata"]
        #[doc = "Gets metadata describing the available metrics"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `app_id`: ID of the application. This is Application ID from the API Access settings blade in the Azure portal."]
        pub fn get_metadata(&self, app_id: impl Into<String>) -> get_metadata::Builder {
            get_metadata::Builder {
                client: self.0.clone(),
                app_id: app_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::MetricsResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) app_id: String,
            pub(crate) metric_id: String,
            pub(crate) timespan: Option<String>,
            pub(crate) interval: Option<String>,
            pub(crate) aggregation: Vec<String>,
            pub(crate) segment: Vec<String>,
            pub(crate) top: Option<i32>,
            pub(crate) orderby: Option<String>,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "The timespan over which to retrieve metric values. This is an ISO8601 time period value. If timespan is omitted, a default time range of `PT12H` (\"last 12 hours\") is used. The actual timespan that is queried may be adjusted by the server based. In all cases, the actual time span used for the query is included in the response."]
            pub fn timespan(mut self, timespan: impl Into<String>) -> Self {
                self.timespan = Some(timespan.into());
                self
            }
            #[doc = "The time interval to use when retrieving metric values. This is an ISO8601 duration. If interval is omitted, the metric value is aggregated across the entire timespan. If interval is supplied, the server may adjust the interval to a more appropriate size based on the timespan used for the query. In all cases, the actual interval used for the query is included in the response."]
            pub fn interval(mut self, interval: impl Into<String>) -> Self {
                self.interval = Some(interval.into());
                self
            }
            #[doc = "The aggregation to use when computing the metric values. To retrieve more than one aggregation at a time, separate them with a comma. If no aggregation is specified, then the default aggregation for the metric is used."]
            pub fn aggregation(mut self, aggregation: Vec<String>) -> Self {
                self.aggregation = aggregation;
                self
            }
            #[doc = "The name of the dimension to segment the metric values by. This dimension must be applicable to the metric you are retrieving. To segment by more than one dimension at a time, separate them with a comma (,). In this case, the metric data will be segmented in the order the dimensions are listed in the parameter."]
            pub fn segment(mut self, segment: Vec<String>) -> Self {
                self.segment = segment;
                self
            }
            #[doc = "The number of segments to return.  This value is only valid when segment is specified."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "The aggregation function and direction to sort the segments by.  This value is only valid when segment is specified."]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "An expression used to filter the results.  This value should be a valid OData filter expression where the keys of each clause should be applicable dimensions for the metric you are retrieving."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/apps/{}/metrics/{}",
                            this.client.endpoint(),
                            &this.app_id,
                            &this.metric_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timespan) = &this.timespan {
                            req.url_mut().query_pairs_mut().append_pair("timespan", timespan);
                        }
                        if let Some(interval) = &this.interval {
                            req.url_mut().query_pairs_mut().append_pair("interval", interval);
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MetricsResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_multiple {
        use super::models;
        type Response = models::MetricsResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) app_id: String,
            pub(crate) body: models::MetricsPostBody,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/apps/{}/metrics", this.client.endpoint(), &this.app_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MetricsResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_metadata {
        use super::models;
        type Response = serde_json::Value;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) app_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/apps/{}/metrics/metadata", this.client.endpoint(), &this.app_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: serde_json::Value = serde_json::from_slice(&rsp_body)?;
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
pub mod events {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Execute OData query"]
        #[doc = "Executes an OData query for events"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `app_id`: ID of the application. This is Application ID from the API Access settings blade in the Azure portal."]
        #[doc = "* `event_type`: The type of events to query; either a standard event type (`traces`, `customEvents`, `pageViews`, `requests`, `dependencies`, `exceptions`, `availabilityResults`) or `$all` to query across all event types."]
        pub fn get_by_type(&self, app_id: impl Into<String>, event_type: impl Into<String>) -> get_by_type::Builder {
            get_by_type::Builder {
                client: self.0.clone(),
                app_id: app_id.into(),
                event_type: event_type.into(),
                timespan: None,
                filter: None,
                search: None,
                orderby: None,
                select: None,
                skip: None,
                top: None,
                format: None,
                count: None,
                apply: None,
            }
        }
        #[doc = "Get an event"]
        #[doc = "Gets the data for a single event"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `app_id`: ID of the application. This is Application ID from the API Access settings blade in the Azure portal."]
        #[doc = "* `event_type`: The type of events to query; either a standard event type (`traces`, `customEvents`, `pageViews`, `requests`, `dependencies`, `exceptions`, `availabilityResults`) or `$all` to query across all event types."]
        #[doc = "* `event_id`: ID of event."]
        pub fn get(&self, app_id: impl Into<String>, event_type: impl Into<String>, event_id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                app_id: app_id.into(),
                event_type: event_type.into(),
                event_id: event_id.into(),
                timespan: None,
            }
        }
        #[doc = "Get OData metadata"]
        #[doc = "Gets OData EDMX metadata describing the event data model"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `app_id`: ID of the application. This is Application ID from the API Access settings blade in the Azure portal."]
        pub fn get_odata_metadata(&self, app_id: impl Into<String>) -> get_odata_metadata::Builder {
            get_odata_metadata::Builder {
                client: self.0.clone(),
                app_id: app_id.into(),
            }
        }
    }
    pub mod get_by_type {
        use super::models;
        type Response = models::EventsResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) app_id: String,
            pub(crate) event_type: String,
            pub(crate) timespan: Option<String>,
            pub(crate) filter: Option<String>,
            pub(crate) search: Option<String>,
            pub(crate) orderby: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) skip: Option<i32>,
            pub(crate) top: Option<i32>,
            pub(crate) format: Option<String>,
            pub(crate) count: Option<bool>,
            pub(crate) apply: Option<String>,
        }
        impl Builder {
            #[doc = "Optional. The timespan over which to retrieve events. This is an ISO8601 time period value.  This timespan is applied in addition to any that are specified in the Odata expression."]
            pub fn timespan(mut self, timespan: impl Into<String>) -> Self {
                self.timespan = Some(timespan.into());
                self
            }
            #[doc = "An expression used to filter the returned events"]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "A free-text search expression to match for whether a particular event should be returned"]
            pub fn search(mut self, search: impl Into<String>) -> Self {
                self.search = Some(search.into());
                self
            }
            #[doc = "A comma-separated list of properties with \\\"asc\\\" (the default) or \\\"desc\\\" to control the order of returned events"]
            pub fn orderby(mut self, orderby: impl Into<String>) -> Self {
                self.orderby = Some(orderby.into());
                self
            }
            #[doc = "Limits the properties to just those requested on each returned event"]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "The number of items to skip over before returning events"]
            pub fn skip(mut self, skip: i32) -> Self {
                self.skip = Some(skip);
                self
            }
            #[doc = "The number of events to return"]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "Format for the returned events"]
            pub fn format(mut self, format: impl Into<String>) -> Self {
                self.format = Some(format.into());
                self
            }
            #[doc = "Request a count of matching items included with the returned events"]
            pub fn count(mut self, count: bool) -> Self {
                self.count = Some(count);
                self
            }
            #[doc = "An expression used for aggregation over returned events"]
            pub fn apply(mut self, apply: impl Into<String>) -> Self {
                self.apply = Some(apply.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/apps/{}/events/{}",
                            this.client.endpoint(),
                            &this.app_id,
                            &this.event_type
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timespan) = &this.timespan {
                            req.url_mut().query_pairs_mut().append_pair("timespan", timespan);
                        }
                        if let Some(filter) = &this.filter {
                            req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                        }
                        if let Some(search) = &this.search {
                            req.url_mut().query_pairs_mut().append_pair("$search", search);
                        }
                        if let Some(orderby) = &this.orderby {
                            req.url_mut().query_pairs_mut().append_pair("$orderby", orderby);
                        }
                        if let Some(select) = &this.select {
                            req.url_mut().query_pairs_mut().append_pair("$select", select);
                        }
                        if let Some(skip) = &this.skip {
                            req.url_mut().query_pairs_mut().append_pair("$skip", &skip.to_string());
                        }
                        if let Some(top) = &this.top {
                            req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                        }
                        if let Some(format) = &this.format {
                            req.url_mut().query_pairs_mut().append_pair("$format", format);
                        }
                        if let Some(count) = &this.count {
                            req.url_mut().query_pairs_mut().append_pair("$count", &count.to_string());
                        }
                        if let Some(apply) = &this.apply {
                            req.url_mut().query_pairs_mut().append_pair("$apply", apply);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EventsResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod get {
        use super::models;
        type Response = models::EventsResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) app_id: String,
            pub(crate) event_type: String,
            pub(crate) event_id: String,
            pub(crate) timespan: Option<String>,
        }
        impl Builder {
            #[doc = "Optional. The timespan over which to retrieve events. This is an ISO8601 time period value.  This timespan is applied in addition to any that are specified in the Odata expression."]
            pub fn timespan(mut self, timespan: impl Into<String>) -> Self {
                self.timespan = Some(timespan.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/apps/{}/events/{}/{}",
                            this.client.endpoint(),
                            &this.app_id,
                            &this.event_type,
                            &this.event_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        if let Some(timespan) = &this.timespan {
                            req.url_mut().query_pairs_mut().append_pair("timespan", timespan);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EventsResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_odata_metadata {
        use super::models;
        type Response = serde_json::Value;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) app_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/apps/{}/events/$metadata", this.client.endpoint(), &this.app_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: serde_json::Value = serde_json::from_slice(&rsp_body)?;
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
pub mod query {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Execute an Analytics query"]
        #[doc = "Executes an Analytics query for data"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `app_id`: ID of the application. This is Application ID from the API Access settings blade in the Azure portal."]
        #[doc = "* `query`: The Analytics query. Learn more about the [Analytics query syntax](https://azure.microsoft.com/documentation/articles/app-insights-analytics-reference/)"]
        pub fn get(&self, app_id: impl Into<String>, query: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                app_id: app_id.into(),
                query: query.into(),
                timespan: None,
            }
        }
        #[doc = "Execute an Analytics query"]
        #[doc = "Executes an Analytics query for data. [Here](https://dev.applicationinsights.io/documentation/Using-the-API/Query) is an example for using POST with an Analytics query."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `app_id`: ID of the application. This is Application ID from the API Access settings blade in the Azure portal."]
        #[doc = "* `body`: The Analytics query. Learn more about the [Analytics query syntax](https://azure.microsoft.com/documentation/articles/app-insights-analytics-reference/)"]
        pub fn execute(&self, app_id: impl Into<String>, body: impl Into<models::QueryBody>) -> execute::Builder {
            execute::Builder {
                client: self.0.clone(),
                app_id: app_id.into(),
                body: body.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::QueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) app_id: String,
            pub(crate) query: String,
            pub(crate) timespan: Option<String>,
        }
        impl Builder {
            #[doc = "Optional. The timespan over which to query data. This is an ISO8601 time period value.  This timespan is applied in addition to any that are specified in the query expression."]
            pub fn timespan(mut self, timespan: impl Into<String>) -> Self {
                self.timespan = Some(timespan.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/apps/{}/query", this.client.endpoint(), &this.app_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let query = &this.query;
                        req.url_mut().query_pairs_mut().append_pair("query", query);
                        if let Some(timespan) = &this.timespan {
                            req.url_mut().query_pairs_mut().append_pair("timespan", timespan);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::QueryResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod execute {
        use super::models;
        type Response = models::QueryResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) app_id: String,
            pub(crate) body: models::QueryBody,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/apps/{}/query", this.client.endpoint(), &this.app_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::QueryResults = serde_json::from_slice(&rsp_body)?;
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
pub mod metadata {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets metadata information"]
        #[doc = "Retrieve the metadata information for the app, including its schema, etc."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `app_id`: ID of the application. This is Application ID from the API Access settings blade in the Azure portal."]
        pub fn get(&self, app_id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                app_id: app_id.into(),
            }
        }
        #[doc = "Gets metadata information"]
        #[doc = "Retrieve the metadata information for the app, including its schema, etc."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `app_id`: ID of the application. This is Application ID from the API Access settings blade in the Azure portal."]
        pub fn post(&self, app_id: impl Into<String>) -> post::Builder {
            post::Builder {
                client: self.0.clone(),
                app_id: app_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::MetadataResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) app_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/apps/{}/metadata", this.client.endpoint(), &this.app_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MetadataResults = serde_json::from_slice(&rsp_body)?;
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
    pub mod post {
        use super::models;
        type Response = models::MetadataResults;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) app_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/apps/{}/metadata", this.client.endpoint(), &this.app_id))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MetadataResults = serde_json::from_slice(&rsp_body)?;
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
