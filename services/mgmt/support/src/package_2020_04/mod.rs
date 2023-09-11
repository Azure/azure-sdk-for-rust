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
    pub fn communications_client(&self) -> communications::Client {
        communications::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn problem_classifications_client(&self) -> problem_classifications::Client {
        problem_classifications::Client(self.clone())
    }
    pub fn services_client(&self) -> services::Client {
        services::Client(self.clone())
    }
    pub fn support_tickets_client(&self) -> support_tickets::Client {
        support_tickets::Client(self.clone())
    }
}
pub mod operations {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "This lists all the available Microsoft Support REST API operations."]
        pub fn list(&self) -> list::RequestBuilder {
            list::RequestBuilder { client: self.0.clone() }
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
            pub async fn into_body(self) -> azure_core::Result<models::OperationsListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::OperationsListResult = serde_json::from_slice(&bytes)?;
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
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/providers/Microsoft.Support/operations", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-04-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod services {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all the Azure services available for support ticket creation. For **Technical** issues, select the Service Id that maps to the Azure service/product as displayed in the **Services** drop-down list on the Azure portal's [New support request](https://portal.azure.com/#blade/Microsoft_Azure_Support/HelpAndSupportBlade/overview) page. Always use the service and its corresponding problem classification(s) obtained programmatically for support ticket creation. This practice ensures that you always have the most recent set of service and problem classification Ids."]
        pub fn list(&self) -> list::RequestBuilder {
            list::RequestBuilder { client: self.0.clone() }
        }
        #[doc = "Gets a specific Azure service for support ticket creation."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: Name of the Azure service."]
        pub fn get(&self, service_name: impl Into<String>) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                service_name: service_name.into(),
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
            pub async fn into_body(self) -> azure_core::Result<models::ServicesListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ServicesListResult = serde_json::from_slice(&bytes)?;
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
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/providers/Microsoft.Support/services", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-04-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
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
            pub async fn into_body(self) -> azure_core::Result<models::Service> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::Service = serde_json::from_slice(&bytes)?;
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
            pub(crate) service_name: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Support/services/{}",
                            this.client.endpoint(),
                            &this.service_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-04-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::Service>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::Service>>;
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
pub mod problem_classifications {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all the problem classifications (categories) available for a specific Azure service. Always use the service and problem classifications obtained programmatically. This practice ensures that you always have the most recent set of service and problem classification Ids."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: Name of the Azure service for which the problem classifications need to be retrieved."]
        pub fn list(&self, service_name: impl Into<String>) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                service_name: service_name.into(),
            }
        }
        #[doc = "Get problem classification details for a specific Azure service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `service_name`: Name of the Azure service available for support."]
        #[doc = "* `problem_classification_name`: Name of problem classification."]
        pub fn get(&self, service_name: impl Into<String>, problem_classification_name: impl Into<String>) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                service_name: service_name.into(),
                problem_classification_name: problem_classification_name.into(),
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
            pub async fn into_body(self) -> azure_core::Result<models::ProblemClassificationsListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProblemClassificationsListResult = serde_json::from_slice(&bytes)?;
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
            pub(crate) service_name: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Support/services/{}/problemClassifications",
                            this.client.endpoint(),
                            &this.service_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-04-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
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
            pub async fn into_body(self) -> azure_core::Result<models::ProblemClassification> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProblemClassification = serde_json::from_slice(&bytes)?;
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
            pub(crate) service_name: String,
            pub(crate) problem_classification_name: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Support/services/{}/problemClassifications/{}",
                            this.client.endpoint(),
                            &this.service_name,
                            &this.problem_classification_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-04-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ProblemClassification>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ProblemClassification>>;
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
pub mod support_tickets {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Check the availability of a resource name. This API should be used to check the uniqueness of the name for support ticket creation for the selected subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `check_name_availability_input`: Input to check."]
        #[doc = "* `subscription_id`: Azure subscription Id."]
        pub fn check_name_availability(
            &self,
            check_name_availability_input: impl Into<models::CheckNameAvailabilityInput>,
            subscription_id: impl Into<String>,
        ) -> check_name_availability::RequestBuilder {
            check_name_availability::RequestBuilder {
                client: self.0.clone(),
                check_name_availability_input: check_name_availability_input.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Lists all the support tickets for an Azure subscription. You can also filter the support tickets by _Status_, _CreatedDate_, _ServiceId_, and _ProblemClassificationId_ using the $filter parameter. Output will be a paged result with _nextLink_, using which you can retrieve the next set of support tickets. <br/><br/>Support ticket data is available for 18 months after ticket creation. If a ticket was created more than 18 months ago, a request for data might cause an error."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure subscription Id."]
        pub fn list(&self, subscription_id: impl Into<String>) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                top: None,
                filter: None,
            }
        }
        #[doc = "Get ticket details for an Azure subscription. Support ticket data is available for 18 months after ticket creation. If a ticket was created more than 18 months ago, a request for data might cause an error."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `support_ticket_name`: Support ticket name."]
        #[doc = "* `subscription_id`: Azure subscription Id."]
        pub fn get(&self, support_ticket_name: impl Into<String>, subscription_id: impl Into<String>) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                support_ticket_name: support_ticket_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates a new support ticket for Subscription and Service limits (Quota), Technical, Billing, and Subscription Management issues for the specified subscription. Learn the [prerequisites](https://aka.ms/supportAPI) required to create a support ticket.<br/><br/>Always call the Services and ProblemClassifications API to get the most recent set of services and problem categories required for support ticket creation.<br/><br/>Adding attachments is not currently supported via the API. To add a file to an existing support ticket, visit the [Manage support ticket](https://portal.azure.com/#blade/Microsoft_Azure_Support/HelpAndSupportBlade/managesupportrequest) page in the Azure portal, select the support ticket, and use the file upload control to add a new file.<br/><br/>Providing consent to share diagnostic information with Azure support is currently not supported via the API. The Azure support engineer working on your ticket will reach out to you for consent if your issue requires gathering diagnostic information from your Azure resources.<br/><br/>**Creating a support ticket for on-behalf-of**: Include _x-ms-authorization-auxiliary_ header to provide an auxiliary token as per [documentation](https://docs.microsoft.com/azure/azure-resource-manager/management/authenticate-multi-tenant). The primary token will be from the tenant for whom a support ticket is being raised against the subscription, i.e. Cloud solution provider (CSP) customer tenant. The auxiliary token will be from the Cloud solution provider (CSP) partner tenant."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `support_ticket_name`: Support ticket name."]
        #[doc = "* `create_support_ticket_parameters`: Support ticket request payload."]
        #[doc = "* `subscription_id`: Azure subscription Id."]
        pub fn create(
            &self,
            support_ticket_name: impl Into<String>,
            create_support_ticket_parameters: impl Into<models::SupportTicketDetails>,
            subscription_id: impl Into<String>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                support_ticket_name: support_ticket_name.into(),
                create_support_ticket_parameters: create_support_ticket_parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "This API allows you to update the severity level, ticket status, and your contact information in the support ticket.<br/><br/>Note: The severity levels cannot be changed if a support ticket is actively being worked upon by an Azure support engineer. In such a case, contact your support engineer to request severity update by adding a new communication using the Communications API.<br/><br/>Changing the ticket status to _closed_ is allowed only on an unassigned case. When an engineer is actively working on the ticket, send your ticket closure request by sending a note to your engineer."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `support_ticket_name`: Support ticket name."]
        #[doc = "* `update_support_ticket`: UpdateSupportTicket object."]
        #[doc = "* `subscription_id`: Azure subscription Id."]
        pub fn update(
            &self,
            support_ticket_name: impl Into<String>,
            update_support_ticket: impl Into<models::UpdateSupportTicket>,
            subscription_id: impl Into<String>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                support_ticket_name: support_ticket_name.into(),
                update_support_ticket: update_support_ticket.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod check_name_availability {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::CheckNameAvailabilityOutput> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::CheckNameAvailabilityOutput = serde_json::from_slice(&bytes)?;
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
            pub(crate) check_name_availability_input: models::CheckNameAvailabilityInput,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.Support/checkNameAvailability",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-04-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.check_name_availability_input)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::CheckNameAvailabilityOutput>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::CheckNameAvailabilityOutput>>;
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
            pub async fn into_body(self) -> azure_core::Result<models::SupportTicketsListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SupportTicketsListResult = serde_json::from_slice(&bytes)?;
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
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The number of values to return in the collection. Default is 25 and max is 100."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "The filter to apply on the operation. We support 'odata v4.0' filter semantics. [Learn more](https://docs.microsoft.com/odata/concepts/queryoptions-overview). _Status_, _ServiceId_, and _ProblemClassificationId_ filters can only be used with Equals ('eq') operator. For _CreatedDate_ filter, the supported operators are Greater Than ('gt') and Greater Than or Equals ('ge'). When using both filters, combine them using the logical 'AND'."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::SupportTicketsListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.Support/supportTickets",
                            this.client.endpoint(),
                            &this.subscription_id
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-04-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-04-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
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
            pub async fn into_body(self) -> azure_core::Result<models::SupportTicketDetails> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SupportTicketDetails = serde_json::from_slice(&bytes)?;
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
            pub(crate) support_ticket_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.Support/supportTickets/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.support_ticket_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-04-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SupportTicketDetails>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SupportTicketDetails>>;
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
    pub mod create {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SupportTicketDetails> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SupportTicketDetails = serde_json::from_slice(&bytes)?;
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
            pub(crate) support_ticket_name: String,
            pub(crate) create_support_ticket_parameters: models::SupportTicketDetails,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.Support/supportTickets/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.support_ticket_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-04-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.create_support_ticket_parameters)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SupportTicketDetails>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SupportTicketDetails>>;
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
    pub mod update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SupportTicketDetails> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SupportTicketDetails = serde_json::from_slice(&bytes)?;
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
            pub(crate) support_ticket_name: String,
            pub(crate) update_support_ticket: models::UpdateSupportTicket,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.Support/supportTickets/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.support_ticket_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-04-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.update_support_ticket)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SupportTicketDetails>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SupportTicketDetails>>;
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
pub mod communications {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Check the availability of a resource name. This API should be used to check the uniqueness of the name for adding a new communication to the support ticket."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `support_ticket_name`: Support ticket name."]
        #[doc = "* `check_name_availability_input`: Input to check."]
        #[doc = "* `subscription_id`: Azure subscription Id."]
        pub fn check_name_availability(
            &self,
            support_ticket_name: impl Into<String>,
            check_name_availability_input: impl Into<models::CheckNameAvailabilityInput>,
            subscription_id: impl Into<String>,
        ) -> check_name_availability::RequestBuilder {
            check_name_availability::RequestBuilder {
                client: self.0.clone(),
                support_ticket_name: support_ticket_name.into(),
                check_name_availability_input: check_name_availability_input.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Lists all communications (attachments not included) for a support ticket. <br/></br> You can also filter support ticket communications by _CreatedDate_ or _CommunicationType_ using the $filter parameter. The only type of communication supported today is _Web_. Output will be a paged result with _nextLink_, using which you can retrieve the next set of Communication results. <br/><br/>Support ticket data is available for 18 months after ticket creation. If a ticket was created more than 18 months ago, a request for data might cause an error."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `support_ticket_name`: Support ticket name."]
        #[doc = "* `subscription_id`: Azure subscription Id."]
        pub fn list(&self, support_ticket_name: impl Into<String>, subscription_id: impl Into<String>) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                support_ticket_name: support_ticket_name.into(),
                subscription_id: subscription_id.into(),
                top: None,
                filter: None,
            }
        }
        #[doc = "Returns communication details for a support ticket."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `support_ticket_name`: Support ticket name."]
        #[doc = "* `communication_name`: Communication name."]
        #[doc = "* `subscription_id`: Azure subscription Id."]
        pub fn get(
            &self,
            support_ticket_name: impl Into<String>,
            communication_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                support_ticket_name: support_ticket_name.into(),
                communication_name: communication_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Adds a new customer communication to an Azure support ticket."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `support_ticket_name`: Support ticket name."]
        #[doc = "* `communication_name`: Communication name."]
        #[doc = "* `create_communication_parameters`: Communication object."]
        #[doc = "* `subscription_id`: Azure subscription Id."]
        pub fn create(
            &self,
            support_ticket_name: impl Into<String>,
            communication_name: impl Into<String>,
            create_communication_parameters: impl Into<models::CommunicationDetails>,
            subscription_id: impl Into<String>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                support_ticket_name: support_ticket_name.into(),
                communication_name: communication_name.into(),
                create_communication_parameters: create_communication_parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod check_name_availability {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::CheckNameAvailabilityOutput> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::CheckNameAvailabilityOutput = serde_json::from_slice(&bytes)?;
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
            pub(crate) support_ticket_name: String,
            pub(crate) check_name_availability_input: models::CheckNameAvailabilityInput,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.Support/supportTickets/{}/checkNameAvailability",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.support_ticket_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2020-04-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.check_name_availability_input)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::CheckNameAvailabilityOutput>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::CheckNameAvailabilityOutput>>;
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
            pub async fn into_body(self) -> azure_core::Result<models::CommunicationsListResult> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::CommunicationsListResult = serde_json::from_slice(&bytes)?;
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
            pub(crate) support_ticket_name: String,
            pub(crate) subscription_id: String,
            pub(crate) top: Option<i32>,
            pub(crate) filter: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The number of values to return in the collection. Default is 10 and max is 10."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            #[doc = "The filter to apply on the operation. You can filter by communicationType and createdDate properties. CommunicationType supports Equals ('eq') operator and createdDate supports Greater Than ('gt') and Greater Than or Equals ('ge') operators. You may combine the CommunicationType and CreatedDate filters by Logical And ('and') operator."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::CommunicationsListResult, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.Support/supportTickets/{}/communications",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.support_ticket_name
                        ))?;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                let has_api_version_already =
                                    req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                                if !has_api_version_already {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair(azure_core::query_param::API_VERSION, "2020-04-01");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2020-04-01");
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
                                }
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let rsp = match rsp.status() {
                            azure_core::StatusCode::Ok => Ok(Response(rsp)),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        };
                        rsp?.into_body().await
                    }
                };
                azure_core::Pageable::new(make_request)
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
            pub async fn into_body(self) -> azure_core::Result<models::CommunicationDetails> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::CommunicationDetails = serde_json::from_slice(&bytes)?;
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
            pub(crate) support_ticket_name: String,
            pub(crate) communication_name: String,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.Support/supportTickets/{}/communications/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.support_ticket_name,
                            &this.communication_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-04-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::CommunicationDetails>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::CommunicationDetails>>;
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
    pub mod create {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::CommunicationDetails> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::CommunicationDetails = serde_json::from_slice(&bytes)?;
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
            pub(crate) support_ticket_name: String,
            pub(crate) communication_name: String,
            pub(crate) create_communication_parameters: models::CommunicationDetails,
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.Support/supportTickets/{}/communications/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.support_ticket_name,
                            &this.communication_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2020-04-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.create_communication_parameters)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::CommunicationDetails>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::CommunicationDetails>>;
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
