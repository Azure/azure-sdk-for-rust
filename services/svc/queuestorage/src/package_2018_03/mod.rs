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
    pub fn message_id_client(&self) -> message_id::Client {
        message_id::Client(self.clone())
    }
    pub fn messages_client(&self) -> messages::Client {
        messages::Client(self.clone())
    }
    pub fn queue_client(&self) -> queue::Client {
        queue::Client(self.clone())
    }
    pub fn service_client(&self) -> service::Client {
        service::Client(self.clone())
    }
}
pub mod service {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "gets the properties of a storage account's Queue service, including properties for Storage Analytics and CORS (Cross-Origin Resource Sharing) rules."]
        pub fn get_properties(&self) -> get_properties::RequestBuilder {
            get_properties::RequestBuilder {
                client: self.0.clone(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Sets properties for a storage account's Queue service endpoint, including properties for Storage Analytics and CORS (Cross-Origin Resource Sharing) rules"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `storage_service_properties`: The StorageService properties."]
        pub fn set_properties(
            &self,
            storage_service_properties: impl Into<models::StorageServiceProperties>,
        ) -> set_properties::RequestBuilder {
            set_properties::RequestBuilder {
                client: self.0.clone(),
                storage_service_properties: storage_service_properties.into(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Retrieves statistics related to replication for the Queue service. It is only available on the secondary location endpoint when read-access geo-redundant replication is enabled for the storage account."]
        pub fn get_statistics(&self) -> get_statistics::RequestBuilder {
            get_statistics::RequestBuilder {
                client: self.0.clone(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The List Queues Segment operation returns a list of the queues under the specified account"]
        pub fn list_queues_segment(&self) -> list_queues_segment::RequestBuilder {
            list_queues_segment::RequestBuilder {
                client: self.0.clone(),
                prefix: None,
                marker: None,
                maxresults: None,
                include: Vec::new(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
    }
    pub mod get_properties {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::StorageServiceProperties> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::StorageServiceProperties = azure_core::xml::read_xml(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Queue service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
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
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/setting-timeouts-for-queue-service-operations>Setting Timeouts for Queue Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
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
                        let url = azure_core::Url::parse(&format!("{}/?restype=service&comp=properties", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2018-03-28");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::StorageServiceProperties>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::StorageServiceProperties>>;
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
    pub mod set_properties {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Queue service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
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
            pub(crate) storage_service_properties: models::StorageServiceProperties,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/setting-timeouts-for-queue-service-operations>Setting Timeouts for Queue Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
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
                        let url = azure_core::Url::parse(&format!("{}/?restype=service&comp=properties", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2018-03-28");
                        req.insert_header("content-type", "application/xml");
                        let req_body = azure_core::to_json(&this.storage_service_properties)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_statistics {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::StorageServiceStats> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::StorageServiceStats = azure_core::xml::read_xml(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Queue service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
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
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/setting-timeouts-for-queue-service-operations>Setting Timeouts for Queue Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
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
                        let url = azure_core::Url::parse(&format!("{}/?restype=service&comp=stats", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2018-03-28");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::StorageServiceStats>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::StorageServiceStats>>;
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
    pub mod list_queues_segment {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ListQueuesSegmentResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ListQueuesSegmentResponse = azure_core::xml::read_xml(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Queue service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
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
            pub(crate) prefix: Option<String>,
            pub(crate) marker: Option<String>,
            pub(crate) maxresults: Option<i64>,
            pub(crate) include: Vec<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Filters the results to return only queues whose name begins with the specified prefix."]
            pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
                self.prefix = Some(prefix.into());
                self
            }
            #[doc = "A string value that identifies the portion of the list of queues to be returned with the next listing operation. The operation returns the NextMarker value within the response body if the listing operation did not return all queues remaining to be listed with the current page. The NextMarker value can be used as the value for the marker parameter in a subsequent call to request the next page of list items. The marker value is opaque to the client."]
            pub fn marker(mut self, marker: impl Into<String>) -> Self {
                self.marker = Some(marker.into());
                self
            }
            #[doc = "Specifies the maximum number of queues to return. If the request does not specify maxresults, or specifies a value greater than 5000, the server will return up to 5000 items. Note that if the listing operation crosses a partition boundary, then the service will return a continuation token for retrieving the remainder of the results. For this reason, it is possible that the service will return fewer results than specified by maxresults, or than the default of 5000."]
            pub fn maxresults(mut self, maxresults: i64) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "Include this parameter to specify that the queues' metadata be returned as part of the response body."]
            pub fn include(mut self, include: Vec<String>) -> Self {
                self.include = include;
                self
            }
            #[doc = "The The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/setting-timeouts-for-queue-service-operations>Setting Timeouts for Queue Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::ListQueuesSegmentResponse, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!("{}/?comp=list", this.client.endpoint(),))?;
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
                                req.insert_header(azure_core::headers::VERSION, "2018-03-28");
                                if let Some(prefix) = &this.prefix {
                                    req.url_mut().query_pairs_mut().append_pair("prefix", prefix);
                                }
                                if let Some(marker) = &this.marker {
                                    req.url_mut().query_pairs_mut().append_pair("marker", marker);
                                }
                                if let Some(maxresults) = &this.maxresults {
                                    req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                                }
                                if let Some(timeout) = &this.timeout {
                                    req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                                }
                                if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                                    req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
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
}
pub mod queue {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "creates a new queue under the given account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `queue_name`: The queue name."]
        pub fn create(&self, queue_name: impl Into<String>) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                queue_name: queue_name.into(),
                timeout: None,
                x_ms_meta: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "operation permanently deletes the specified queue"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `queue_name`: The queue name."]
        pub fn delete(&self, queue_name: impl Into<String>) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                queue_name: queue_name.into(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Retrieves user-defined metadata and queue properties on the specified queue. Metadata is associated with the queue as name-values pairs."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `queue_name`: The queue name."]
        pub fn get_properties(&self, queue_name: impl Into<String>) -> get_properties::RequestBuilder {
            get_properties::RequestBuilder {
                client: self.0.clone(),
                queue_name: queue_name.into(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "sets user-defined metadata on the specified queue. Metadata is associated with the queue as name-value pairs."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `queue_name`: The queue name."]
        pub fn set_metadata(&self, queue_name: impl Into<String>) -> set_metadata::RequestBuilder {
            set_metadata::RequestBuilder {
                client: self.0.clone(),
                queue_name: queue_name.into(),
                timeout: None,
                x_ms_meta: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "returns details about any stored access policies specified on the queue that may be used with Shared Access Signatures."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `queue_name`: The queue name."]
        pub fn get_access_policy(&self, queue_name: impl Into<String>) -> get_access_policy::RequestBuilder {
            get_access_policy::RequestBuilder {
                client: self.0.clone(),
                queue_name: queue_name.into(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "sets stored access policies for the queue that may be used with Shared Access Signatures"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `queue_name`: The queue name."]
        pub fn set_access_policy(&self, queue_name: impl Into<String>) -> set_access_policy::RequestBuilder {
            set_access_policy::RequestBuilder {
                client: self.0.clone(),
                queue_name: queue_name.into(),
                queue_acl: None,
                timeout: None,
                x_ms_client_request_id: None,
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
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Queue service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
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
            pub(crate) queue_name: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/setting-timeouts-for-queue-service-operations>Setting Timeouts for Queue Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional. Include this parameter to specify that the queue's metadata be returned as part of the response body. Note that metadata requested with this parameter must be stored in accordance with the naming restrictions imposed by the 2009-09-19 version of the Queue service. Beginning with this version, all metadata names must adhere to the naming conventions for C# identifiers."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
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
                        let url = azure_core::Url::parse(&format!("{}/{}", this.client.endpoint(), &this.queue_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2018-03-28");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Queue service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
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
            pub(crate) queue_name: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/setting-timeouts-for-queue-service-operations>Setting Timeouts for Queue Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
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
                        let url = azure_core::Url::parse(&format!("{}/{}", this.client.endpoint(), &this.queue_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2018-03-28");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_properties {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            pub fn x_ms_meta(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-meta"))
            }
            #[doc = "The approximate number of messages in the queue. This number is not lower than the actual number of messages in the queue, but could be higher."]
            pub fn x_ms_approximate_messages_count(&self) -> azure_core::Result<i32> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-approximate-messages-count"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Queue service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
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
            pub(crate) queue_name: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/setting-timeouts-for-queue-service-operations>Setting Timeouts for Queue Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
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
                        let url = azure_core::Url::parse(&format!("{}/{}?comp=metadata", this.client.endpoint(), &this.queue_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2018-03-28");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod set_metadata {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Queue service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
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
            pub(crate) queue_name: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/setting-timeouts-for-queue-service-operations>Setting Timeouts for Queue Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional. Include this parameter to specify that the queue's metadata be returned as part of the response body. Note that metadata requested with this parameter must be stored in accordance with the naming restrictions imposed by the 2009-09-19 version of the Queue service. Beginning with this version, all metadata names must adhere to the naming conventions for C# identifiers."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
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
                        let url = azure_core::Url::parse(&format!("{}/{}?comp=metadata", this.client.endpoint(), &this.queue_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2018-03-28");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_access_policy {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SignedIdentifiers> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SignedIdentifiers = azure_core::xml::read_xml(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Queue service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
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
            pub(crate) queue_name: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/setting-timeouts-for-queue-service-operations>Setting Timeouts for Queue Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
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
                        let url = azure_core::Url::parse(&format!("{}/{}?comp=acl", this.client.endpoint(), &this.queue_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2018-03-28");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SignedIdentifiers>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SignedIdentifiers>>;
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
    pub mod set_access_policy {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Queue service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
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
            pub(crate) queue_name: String,
            pub(crate) queue_acl: Option<models::SignedIdentifiers>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "the acls for the queue"]
            pub fn queue_acl(mut self, queue_acl: impl Into<models::SignedIdentifiers>) -> Self {
                self.queue_acl = Some(queue_acl.into());
                self
            }
            #[doc = "The The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/setting-timeouts-for-queue-service-operations>Setting Timeouts for Queue Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
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
                        let url = azure_core::Url::parse(&format!("{}/{}?comp=acl", this.client.endpoint(), &this.queue_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2018-03-28");
                        let req_body = if let Some(queue_acl) = &this.queue_acl {
                            req.insert_header("content-type", "application/xml");
                            azure_core::to_json(queue_acl)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
pub mod messages {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "The Dequeue operation retrieves one or more messages from the front of the queue."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `queue_name`: The queue name."]
        pub fn dequeue(&self, queue_name: impl Into<String>) -> dequeue::RequestBuilder {
            dequeue::RequestBuilder {
                client: self.0.clone(),
                queue_name: queue_name.into(),
                numofmessages: None,
                visibilitytimeout: None,
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Enqueue operation adds a new message to the back of the message queue. A visibility timeout can also be specified to make the message invisible until the visibility timeout expires. A message must be in a format that can be included in an XML request with UTF-8 encoding. The encoded message can be up to 64 KB in size for versions 2011-08-18 and newer, or 8 KB in size for previous versions."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `queue_name`: The queue name."]
        #[doc = "* `queue_message`: A Message object which can be stored in a Queue"]
        pub fn enqueue(&self, queue_name: impl Into<String>, queue_message: impl Into<models::QueueMessage>) -> enqueue::RequestBuilder {
            enqueue::RequestBuilder {
                client: self.0.clone(),
                queue_name: queue_name.into(),
                queue_message: queue_message.into(),
                visibilitytimeout: None,
                messagettl: None,
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Clear operation deletes all messages from the specified queue."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `queue_name`: The queue name."]
        pub fn clear(&self, queue_name: impl Into<String>) -> clear::RequestBuilder {
            clear::RequestBuilder {
                client: self.0.clone(),
                queue_name: queue_name.into(),
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Peek operation retrieves one or more messages from the front of the queue, but does not alter the visibility of the message."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `queue_name`: The queue name."]
        pub fn peek(&self, queue_name: impl Into<String>) -> peek::RequestBuilder {
            peek::RequestBuilder {
                client: self.0.clone(),
                queue_name: queue_name.into(),
                numofmessages: None,
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
    }
    pub mod dequeue {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DequeuedMessagesList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DequeuedMessagesList = azure_core::xml::read_xml(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Queue service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
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
            pub(crate) queue_name: String,
            pub(crate) numofmessages: Option<i64>,
            pub(crate) visibilitytimeout: Option<i64>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Optional. A nonzero integer value that specifies the number of messages to retrieve from the queue, up to a maximum of 32. If fewer are visible, the visible messages are returned. By default, a single message is retrieved from the queue with this operation."]
            pub fn numofmessages(mut self, numofmessages: i64) -> Self {
                self.numofmessages = Some(numofmessages);
                self
            }
            #[doc = "Optional. Specifies the new visibility timeout value, in seconds, relative to server time. The default value is 30 seconds. A specified value must be larger than or equal to 1 second, and cannot be larger than 7 days, or larger than 2 hours on REST protocol versions prior to version 2011-08-18. The visibility timeout of a message can be set to a value later than the expiry time."]
            pub fn visibilitytimeout(mut self, visibilitytimeout: i64) -> Self {
                self.visibilitytimeout = Some(visibilitytimeout);
                self
            }
            #[doc = "The The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/setting-timeouts-for-queue-service-operations>Setting Timeouts for Queue Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
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
                        let url = azure_core::Url::parse(&format!("{}/{}/messages", this.client.endpoint(), &this.queue_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2018-03-28");
                        if let Some(numofmessages) = &this.numofmessages {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("numofmessages", &numofmessages.to_string());
                        }
                        if let Some(visibilitytimeout) = &this.visibilitytimeout {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("visibilitytimeout", &visibilitytimeout.to_string());
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DequeuedMessagesList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::DequeuedMessagesList>>;
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
    pub mod enqueue {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::EnqueuedMessageList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::EnqueuedMessageList = azure_core::xml::read_xml(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Queue service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
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
            pub(crate) queue_name: String,
            pub(crate) queue_message: models::QueueMessage,
            pub(crate) visibilitytimeout: Option<i64>,
            pub(crate) messagettl: Option<i64>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Optional. If specified, the request must be made using an x-ms-version of 2011-08-18 or later. If not specified, the default value is 0. Specifies the new visibility timeout value, in seconds, relative to server time. The new value must be larger than or equal to 0, and cannot be larger than 7 days. The visibility timeout of a message cannot be set to a value later than the expiry time. visibilitytimeout should be set to a value smaller than the time-to-live value."]
            pub fn visibilitytimeout(mut self, visibilitytimeout: i64) -> Self {
                self.visibilitytimeout = Some(visibilitytimeout);
                self
            }
            #[doc = "Optional. Specifies the time-to-live interval for the message, in seconds. Prior to version 2017-07-29, the maximum time-to-live allowed is 7 days. For version 2017-07-29 or later, the maximum time-to-live can be any positive number, as well as -1 indicating that the message does not expire. If this parameter is omitted, the default time-to-live is 7 days."]
            pub fn messagettl(mut self, messagettl: i64) -> Self {
                self.messagettl = Some(messagettl);
                self
            }
            #[doc = "The The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/setting-timeouts-for-queue-service-operations>Setting Timeouts for Queue Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
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
                        let url = azure_core::Url::parse(&format!("{}/{}/messages", this.client.endpoint(), &this.queue_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2018-03-28");
                        req.insert_header("content-type", "application/xml");
                        let req_body = azure_core::to_json(&this.queue_message)?;
                        if let Some(visibilitytimeout) = &this.visibilitytimeout {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("visibilitytimeout", &visibilitytimeout.to_string());
                        }
                        if let Some(messagettl) = &this.messagettl {
                            req.url_mut().query_pairs_mut().append_pair("messagettl", &messagettl.to_string());
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::EnqueuedMessageList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::EnqueuedMessageList>>;
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
    pub mod clear {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Queue service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
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
            pub(crate) queue_name: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/setting-timeouts-for-queue-service-operations>Setting Timeouts for Queue Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
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
                        let url = azure_core::Url::parse(&format!("{}/{}/messages", this.client.endpoint(), &this.queue_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2018-03-28");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod peek {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PeekedMessagesList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PeekedMessagesList = azure_core::xml::read_xml(&bytes)?;
                Ok(body)
            }
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Queue service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
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
            pub(crate) queue_name: String,
            pub(crate) numofmessages: Option<i64>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Optional. A nonzero integer value that specifies the number of messages to retrieve from the queue, up to a maximum of 32. If fewer are visible, the visible messages are returned. By default, a single message is retrieved from the queue with this operation."]
            pub fn numofmessages(mut self, numofmessages: i64) -> Self {
                self.numofmessages = Some(numofmessages);
                self
            }
            #[doc = "The The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/setting-timeouts-for-queue-service-operations>Setting Timeouts for Queue Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
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
                        let url =
                            azure_core::Url::parse(&format!("{}/{}/messages?peekonly=true", this.client.endpoint(), &this.queue_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2018-03-28");
                        if let Some(numofmessages) = &this.numofmessages {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("numofmessages", &numofmessages.to_string());
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::PeekedMessagesList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::PeekedMessagesList>>;
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
pub mod message_id {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "The Update operation was introduced with version 2011-08-18 of the Queue service API. The Update Message operation updates the visibility timeout of a message. You can also use this operation to update the contents of a message. A message must be in a format that can be included in an XML request with UTF-8 encoding, and the encoded message can be up to 64KB in size."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `queue_name`: The queue name."]
        #[doc = "* `messageid`: The container name."]
        #[doc = "* `popreceipt`: Required. Specifies the valid pop receipt value returned from an earlier call to the Get Messages or Update Message operation."]
        #[doc = "* `visibilitytimeout`: Optional. Specifies the new visibility timeout value, in seconds, relative to server time. The default value is 30 seconds. A specified value must be larger than or equal to 1 second, and cannot be larger than 7 days, or larger than 2 hours on REST protocol versions prior to version 2011-08-18. The visibility timeout of a message can be set to a value later than the expiry time."]
        pub fn update(
            &self,
            queue_name: impl Into<String>,
            messageid: impl Into<String>,
            popreceipt: impl Into<String>,
            visibilitytimeout: i64,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                queue_name: queue_name.into(),
                messageid: messageid.into(),
                popreceipt: popreceipt.into(),
                visibilitytimeout,
                queue_message: None,
                timeout: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Delete operation deletes the specified message."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `queue_name`: The queue name."]
        #[doc = "* `messageid`: The container name."]
        #[doc = "* `popreceipt`: Required. Specifies the valid pop receipt value returned from an earlier call to the Get Messages or Update Message operation."]
        pub fn delete(
            &self,
            queue_name: impl Into<String>,
            messageid: impl Into<String>,
            popreceipt: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                queue_name: queue_name.into(),
                messageid: messageid.into(),
                popreceipt: popreceipt.into(),
                timeout: None,
                x_ms_client_request_id: None,
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
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Queue service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "The pop receipt of the queue message."]
            pub fn x_ms_popreceipt(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-popreceipt"))
            }
            #[doc = "A UTC date/time value that represents when the message will be visible on the queue."]
            pub fn x_ms_time_next_visible(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-time-next-visible"))?,
                )
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
            pub(crate) queue_name: String,
            pub(crate) messageid: String,
            pub(crate) popreceipt: String,
            pub(crate) visibilitytimeout: i64,
            pub(crate) queue_message: Option<models::QueueMessage>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "A Message object which can be stored in a Queue"]
            pub fn queue_message(mut self, queue_message: impl Into<models::QueueMessage>) -> Self {
                self.queue_message = Some(queue_message.into());
                self
            }
            #[doc = "The The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/setting-timeouts-for-queue-service-operations>Setting Timeouts for Queue Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
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
                            "{}/{}/messages/{}",
                            this.client.endpoint(),
                            &this.queue_name,
                            &this.messageid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2018-03-28");
                        let req_body = if let Some(queue_message) = &this.queue_message {
                            req.insert_header("content-type", "application/xml");
                            azure_core::to_json(queue_message)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        let popreceipt = &this.popreceipt;
                        req.url_mut().query_pairs_mut().append_pair("popreceipt", popreceipt);
                        let visibilitytimeout = &this.visibilitytimeout;
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair("visibilitytimeout", &visibilitytimeout.to_string());
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub fn into_raw_response(self) -> azure_core::Response {
                self.0
            }
            pub fn as_raw_response(&self) -> &azure_core::Response {
                &self.0
            }
            pub fn headers(&self) -> Headers {
                Headers(self.0.headers())
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
        pub struct Headers<'a>(&'a azure_core::headers::Headers);
        impl<'a> Headers<'a> {
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Queue service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated"]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
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
            pub(crate) queue_name: String,
            pub(crate) messageid: String,
            pub(crate) popreceipt: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/setting-timeouts-for-queue-service-operations>Setting Timeouts for Queue Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
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
                            "{}/{}/messages/{}",
                            this.client.endpoint(),
                            &this.queue_name,
                            &this.messageid
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2018-03-28");
                        let popreceipt = &this.popreceipt;
                        req.url_mut().query_pairs_mut().append_pair("popreceipt", popreceipt);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
}
