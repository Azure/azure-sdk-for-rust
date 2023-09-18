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
    pub fn file_system_client(&self) -> file_system::Client {
        file_system::Client(self.clone())
    }
    pub fn path_client(&self) -> path::Client {
        path::Client(self.clone())
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
        #[doc = "List FileSystems"]
        #[doc = "List filesystems and their properties in given account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource`: The value must be \"account\" for all account operations."]
        pub fn list_file_systems(&self, resource: impl Into<String>) -> list_file_systems::RequestBuilder {
            list_file_systems::RequestBuilder {
                client: self.0.clone(),
                resource: resource.into(),
                prefix: None,
                continuation: None,
                max_results: None,
                x_ms_client_request_id: None,
                timeout: None,
            }
        }
    }
    pub mod list_file_systems {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::FileSystemList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::FileSystemList = serde_json::from_slice(&bytes)?;
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
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "A server-generated UUID recorded in the analytics logs for troubleshooting and correlation."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "The version of the REST protocol used to process the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "If the number of filesystems to be listed exceeds the maxResults limit, a continuation token is returned in this response header.  When a continuation token is returned in the response, it must be specified in a subsequent invocation of the list operation to continue listing the filesystems."]
            pub fn x_ms_continuation(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-continuation"))
            }
            #[doc = "The content type of list filesystem response. The default content type is application/json."]
            pub fn content_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-type"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) resource: String,
            pub(crate) prefix: Option<String>,
            pub(crate) continuation: Option<String>,
            pub(crate) max_results: Option<i32>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) timeout: Option<i64>,
        }
        impl RequestBuilder {
            #[doc = "Filters results to filesystems within the specified prefix."]
            pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
                self.prefix = Some(prefix.into());
                self
            }
            #[doc = "Optional.  When deleting a directory, the number of paths that are deleted with each invocation is limited.  If the number of paths to be deleted exceeds this limit, a continuation token is returned in this response header.  When a continuation token is returned in the response, it must be specified in a subsequent invocation of the delete operation to continue deleting the directory."]
            pub fn continuation(mut self, continuation: impl Into<String>) -> Self {
                self.continuation = Some(continuation.into());
                self
            }
            #[doc = "An optional value that specifies the maximum number of items to return. If omitted or greater than 5,000, the response will include up to 5,000 items."]
            pub fn max_results(mut self, max_results: i32) -> Self {
                self.max_results = Some(max_results);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Only the first response will be fetched as the continuation token is not part of the response schema"]
            #[doc = ""]
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-06-08");
                        let resource = &this.resource;
                        req.url_mut().query_pairs_mut().append_pair("resource", resource);
                        if let Some(prefix) = &this.prefix {
                            req.url_mut().query_pairs_mut().append_pair("prefix", prefix);
                        }
                        if let Some(continuation) = &this.continuation {
                            req.url_mut().query_pairs_mut().append_pair("continuation", continuation);
                        }
                        if let Some(max_results) = &this.max_results {
                            req.url_mut().query_pairs_mut().append_pair("maxResults", &max_results.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core::Url::parse(&format!("{}/", self.client.endpoint(),))?;
                Ok(url)
            }
        }
    }
}
pub mod file_system {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Create FileSystem"]
        #[doc = "Create a FileSystem rooted at the specified location. If the FileSystem already exists, the operation fails.  This operation does not support conditional HTTP requests."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `filesystem`: The filesystem identifier."]
        #[doc = "* `resource`: The value must be \"filesystem\" for all filesystem operations."]
        pub fn create(&self, filesystem: impl Into<String>, resource: impl Into<String>) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                filesystem: filesystem.into(),
                resource: resource.into(),
                x_ms_client_request_id: None,
                timeout: None,
                x_ms_properties: None,
            }
        }
        #[doc = "Set FileSystem Properties"]
        #[doc = "Set properties for the FileSystem.  This operation supports conditional HTTP requests.  For more information, see [Specifying Conditional Headers for Blob Service Operations](https://docs.microsoft.com/en-us/rest/api/storageservices/specifying-conditional-headers-for-blob-service-operations)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `filesystem`: The filesystem identifier."]
        #[doc = "* `resource`: The value must be \"filesystem\" for all filesystem operations."]
        pub fn set_properties(&self, filesystem: impl Into<String>, resource: impl Into<String>) -> set_properties::RequestBuilder {
            set_properties::RequestBuilder {
                client: self.0.clone(),
                filesystem: filesystem.into(),
                resource: resource.into(),
                x_ms_client_request_id: None,
                timeout: None,
                x_ms_properties: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Delete FileSystem"]
        #[doc = "Marks the FileSystem for deletion.  When a FileSystem is deleted, a FileSystem with the same identifier cannot be created for at least 30 seconds. While the filesystem is being deleted, attempts to create a filesystem with the same identifier will fail with status code 409 (Conflict), with the service returning additional error information indicating that the filesystem is being deleted. All other operations, including operations on any files or directories within the filesystem, will fail with status code 404 (Not Found) while the filesystem is being deleted. This operation supports conditional HTTP requests.  For more information, see [Specifying Conditional Headers for Blob Service Operations](https://docs.microsoft.com/en-us/rest/api/storageservices/specifying-conditional-headers-for-blob-service-operations)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `filesystem`: The filesystem identifier."]
        #[doc = "* `resource`: The value must be \"filesystem\" for all filesystem operations."]
        pub fn delete(&self, filesystem: impl Into<String>, resource: impl Into<String>) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                filesystem: filesystem.into(),
                resource: resource.into(),
                x_ms_client_request_id: None,
                timeout: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Get FileSystem Properties."]
        #[doc = "All system and user-defined filesystem properties are specified in the response headers."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `filesystem`: The filesystem identifier."]
        #[doc = "* `resource`: The value must be \"filesystem\" for all filesystem operations."]
        pub fn get_properties(&self, filesystem: impl Into<String>, resource: impl Into<String>) -> get_properties::RequestBuilder {
            get_properties::RequestBuilder {
                client: self.0.clone(),
                filesystem: filesystem.into(),
                resource: resource.into(),
                x_ms_client_request_id: None,
                timeout: None,
            }
        }
        #[doc = "List Paths"]
        #[doc = "List FileSystem paths and their properties."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `filesystem`: The filesystem identifier."]
        #[doc = "* `recursive`: Required"]
        pub fn list_paths(&self, filesystem: impl Into<String>, recursive: bool) -> list_paths::RequestBuilder {
            list_paths::RequestBuilder {
                client: self.0.clone(),
                filesystem: filesystem.into(),
                recursive,
                x_ms_client_request_id: None,
                timeout: None,
                continuation: None,
                directory: None,
                max_results: None,
                upn: None,
            }
        }
        #[doc = "The List Blobs operation returns a list of the blobs under the specified container"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `filesystem`: The filesystem identifier."]
        pub fn list_blob_hierarchy_segment(&self, filesystem: impl Into<String>) -> list_blob_hierarchy_segment::RequestBuilder {
            list_blob_hierarchy_segment::RequestBuilder {
                client: self.0.clone(),
                filesystem: filesystem.into(),
                prefix: None,
                delimiter: None,
                marker: None,
                max_results: None,
                include: Vec::new(),
                showonly: None,
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
        #[derive(Debug)]
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
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "An HTTP entity tag associated with the FileSystem."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "The data and time the filesystem was last modified.  Operations on files and directories do not affect the last modified time."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "A server-generated UUID recorded in the analytics logs for troubleshooting and correlation."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "The version of the REST protocol used to process the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "A bool string indicates whether the namespace feature is enabled. If \"true\", the namespace is enabled for the filesystem."]
            pub fn x_ms_namespace_enabled(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-namespace-enabled"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) filesystem: String,
            pub(crate) resource: String,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_properties: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional. User-defined properties to be stored with the filesystem, in the format of a comma-separated list of name and value pairs \"n1=v1, n2=v2, ...\", where each value is a base64 encoded string. Note that the string may only contain ASCII characters in the ISO-8859-1 character set.  If the filesystem exists, any properties not included in the list will be removed.  All properties are removed if the header is omitted.  To merge new and existing properties, first get all existing properties and the current E-Tag, then make a conditional request with the E-Tag and include values for all properties."]
            pub fn x_ms_properties(mut self, x_ms_properties: impl Into<String>) -> Self {
                self.x_ms_properties = Some(x_ms_properties.into());
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
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-06-08");
                        let resource = &this.resource;
                        req.url_mut().query_pairs_mut().append_pair("resource", resource);
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_properties) = &this.x_ms_properties {
                            req.insert_header("x-ms-properties", x_ms_properties);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core::Url::parse(&format!("{}/{}", self.client.endpoint(), &self.filesystem))?;
                Ok(url)
            }
        }
    }
    pub mod set_properties {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
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
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "An HTTP entity tag associated with the filesystem.  Changes to filesystem properties affect the entity tag, but operations on files and directories do not."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "The data and time the filesystem was last modified.  Changes to filesystem properties update the last modified time, but operations on files and directories do not."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "A server-generated UUID recorded in the analytics logs for troubleshooting and correlation."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "The version of the REST protocol used to process the request."]
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
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) filesystem: String,
            pub(crate) resource: String,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_properties: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl RequestBuilder {
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional. User-defined properties to be stored with the filesystem, in the format of a comma-separated list of name and value pairs \"n1=v1, n2=v2, ...\", where each value is a base64 encoded string. Note that the string may only contain ASCII characters in the ISO-8859-1 character set.  If the filesystem exists, any properties not included in the list will be removed.  All properties are removed if the header is omitted.  To merge new and existing properties, first get all existing properties and the current E-Tag, then make a conditional request with the E-Tag and include values for all properties."]
            pub fn x_ms_properties(mut self, x_ms_properties: impl Into<String>) -> Self {
                self.x_ms_properties = Some(x_ms_properties.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
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
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-06-08");
                        let resource = &this.resource;
                        req.url_mut().query_pairs_mut().append_pair("resource", resource);
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_properties) = &this.x_ms_properties {
                            req.insert_header("x-ms-properties", x_ms_properties);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core::Url::parse(&format!("{}/{}", self.client.endpoint(), &self.filesystem))?;
                Ok(url)
            }
        }
    }
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
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
            #[doc = "A server-generated UUID recorded in the analytics logs for troubleshooting and correlation."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "The version of the REST protocol used to process the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
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
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) filesystem: String,
            pub(crate) resource: String,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl RequestBuilder {
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
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
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-06-08");
                        let resource = &this.resource;
                        req.url_mut().query_pairs_mut().append_pair("resource", resource);
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core::Url::parse(&format!("{}/{}", self.client.endpoint(), &self.filesystem))?;
                Ok(url)
            }
        }
    }
    pub mod get_properties {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
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
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "An HTTP entity tag associated with the filesystem.  Changes to filesystem properties affect the entity tag, but operations on files and directories do not."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "The data and time the filesystem was last modified.  Changes to filesystem properties update the last modified time, but operations on files and directories do not."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "A server-generated UUID recorded in the analytics logs for troubleshooting and correlation."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "The version of the REST protocol used to process the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "The user-defined properties associated with the filesystem.  A comma-separated list of name and value pairs in the format \"n1=v1, n2=v2, ...\", where each value is a base64 encoded string. Note that the string may only contain ASCII characters in the ISO-8859-1 character set."]
            pub fn x_ms_properties(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-properties"))
            }
            #[doc = "A bool string indicates whether the namespace feature is enabled. If \"true\", the namespace is enabled for the filesystem."]
            pub fn x_ms_namespace_enabled(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-namespace-enabled"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) filesystem: String,
            pub(crate) resource: String,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) timeout: Option<i64>,
        }
        impl RequestBuilder {
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
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
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Head);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-06-08");
                        let resource = &this.resource;
                        req.url_mut().query_pairs_mut().append_pair("resource", resource);
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core::Url::parse(&format!("{}/{}", self.client.endpoint(), &self.filesystem))?;
                Ok(url)
            }
        }
    }
    pub mod list_paths {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::PathList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::PathList = serde_json::from_slice(&bytes)?;
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
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "An HTTP entity tag associated with the filesystem.  Changes to filesystem properties affect the entity tag, but operations on files and directories do not."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "The data and time the filesystem was last modified.  Changes to filesystem properties update the last modified time, but operations on files and directories do not."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "A server-generated UUID recorded in the analytics logs for troubleshooting and correlation."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "The version of the REST protocol used to process the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "If the number of paths to be listed exceeds the maxResults limit, a continuation token is returned in this response header.  When a continuation token is returned in the response, it must be specified in a subsequent invocation of the list operation to continue listing the paths."]
            pub fn x_ms_continuation(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-continuation"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) filesystem: String,
            pub(crate) recursive: bool,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) continuation: Option<String>,
            pub(crate) directory: Option<String>,
            pub(crate) max_results: Option<i32>,
            pub(crate) upn: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional.  When deleting a directory, the number of paths that are deleted with each invocation is limited.  If the number of paths to be deleted exceeds this limit, a continuation token is returned in this response header.  When a continuation token is returned in the response, it must be specified in a subsequent invocation of the delete operation to continue deleting the directory."]
            pub fn continuation(mut self, continuation: impl Into<String>) -> Self {
                self.continuation = Some(continuation.into());
                self
            }
            #[doc = "Optional.  Filters results to paths within the specified directory. An error occurs if the directory does not exist."]
            pub fn directory(mut self, directory: impl Into<String>) -> Self {
                self.directory = Some(directory.into());
                self
            }
            #[doc = "An optional value that specifies the maximum number of items to return. If omitted or greater than 5,000, the response will include up to 5,000 items."]
            pub fn max_results(mut self, max_results: i32) -> Self {
                self.max_results = Some(max_results);
                self
            }
            #[doc = "Optional. Valid only when Hierarchical Namespace is enabled for the account. If \"true\", the user identity values returned in the x-ms-owner, x-ms-group, and x-ms-acl response headers will be transformed from Azure Active Directory Object IDs to User Principal Names.  If \"false\", the values will be returned as Azure Active Directory Object IDs. The default value is false. Note that group and application Object IDs are not translated because they do not have unique friendly names."]
            pub fn upn(mut self, upn: bool) -> Self {
                self.upn = Some(upn);
                self
            }
            #[doc = "Only the first response will be fetched as the continuation token is not part of the response schema"]
            #[doc = ""]
            #[doc = "Returns a future that sends the request and returns a [`Response`] object that provides low-level access to full response details."]
            #[doc = ""]
            #[doc = "You should typically use `.await` (which implicitly calls `IntoFuture::into_future()`) to finalize and send requests rather than `send()`."]
            #[doc = "However, this function can provide more flexibility when required."]
            pub fn send(self) -> BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-06-08");
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(continuation) = &this.continuation {
                            req.url_mut().query_pairs_mut().append_pair("continuation", continuation);
                        }
                        if let Some(directory) = &this.directory {
                            req.url_mut().query_pairs_mut().append_pair("directory", directory);
                        }
                        let recursive = &this.recursive;
                        req.url_mut().query_pairs_mut().append_pair("recursive", &recursive.to_string());
                        if let Some(max_results) = &this.max_results {
                            req.url_mut().query_pairs_mut().append_pair("maxResults", &max_results.to_string());
                        }
                        if let Some(upn) = &this.upn {
                            req.url_mut().query_pairs_mut().append_pair("upn", &upn.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core::Url::parse(&format!("{}/{}?resource=filesystem", self.client.endpoint(), &self.filesystem))?;
                Ok(url)
            }
        }
    }
    pub mod list_blob_hierarchy_segment {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ListBlobsHierarchySegmentResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ListBlobsHierarchySegmentResponse = azure_core::xml::read_xml(&bytes)?;
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
            #[doc = "The media type of the body of the response. For List Blobs this is 'application/xml'"]
            pub fn content_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-type"))
            }
            #[doc = "If a client request id header is sent in the request, this header will be present in the response with the same value."]
            pub fn x_ms_client_request_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-client-request-id"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Blob service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
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
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) filesystem: String,
            pub(crate) prefix: Option<String>,
            pub(crate) delimiter: Option<String>,
            pub(crate) marker: Option<String>,
            pub(crate) max_results: Option<i32>,
            pub(crate) include: Vec<String>,
            pub(crate) showonly: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Filters results to filesystems within the specified prefix."]
            pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
                self.prefix = Some(prefix.into());
                self
            }
            #[doc = "When the request includes this parameter, the operation returns a BlobPrefix element in the response body that acts as a placeholder for all blobs whose names begin with the same substring up to the appearance of the delimiter character. The delimiter may be a single character or a string."]
            pub fn delimiter(mut self, delimiter: impl Into<String>) -> Self {
                self.delimiter = Some(delimiter.into());
                self
            }
            #[doc = "A string value that identifies the portion of the list of containers to be returned with the next listing operation. The operation returns the NextMarker value within the response body if the listing operation did not return all containers remaining to be listed with the current page. The NextMarker value can be used as the value for the marker parameter in a subsequent call to request the next page of list items. The marker value is opaque to the client."]
            pub fn marker(mut self, marker: impl Into<String>) -> Self {
                self.marker = Some(marker.into());
                self
            }
            #[doc = "An optional value that specifies the maximum number of items to return. If omitted or greater than 5,000, the response will include up to 5,000 items."]
            pub fn max_results(mut self, max_results: i32) -> Self {
                self.max_results = Some(max_results);
                self
            }
            #[doc = "Include this parameter to specify one or more datasets to include in the response."]
            pub fn include(mut self, include: Vec<String>) -> Self {
                self.include = include;
                self
            }
            #[doc = "Include this parameter to specify one or more datasets to include in the response."]
            pub fn showonly(mut self, showonly: impl Into<String>) -> Self {
                self.showonly = Some(showonly.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::ListBlobsHierarchySegmentResponse, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-06-08");
                        if let Some(prefix) = &this.prefix {
                            req.url_mut().query_pairs_mut().append_pair("prefix", prefix);
                        }
                        if let Some(delimiter) = &this.delimiter {
                            req.url_mut().query_pairs_mut().append_pair("delimiter", delimiter);
                        }
                        if let Some(marker) = &this.marker {
                            req.url_mut().query_pairs_mut().append_pair("marker", marker);
                        }
                        if let Some(max_results) = &this.max_results {
                            req.url_mut().query_pairs_mut().append_pair("maxResults", &max_results.to_string());
                        }
                        if let Some(showonly) = &this.showonly {
                            req.url_mut().query_pairs_mut().append_pair("showonly", showonly);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        if let Some(value) = continuation.as_ref() {
                            req.url_mut().query_pairs_mut().append_pair("marker", value);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core::Url::parse(&format!(
                    "{}/{}?restype=container&comp=list&hierarchy",
                    self.client.endpoint(),
                    &self.filesystem
                ))?;
                Ok(url)
            }
        }
    }
}
pub mod path {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Read File"]
        #[doc = "Read the contents of a file.  For read operations, range requests are supported. This operation supports conditional HTTP requests.  For more information, see [Specifying Conditional Headers for Blob Service Operations](https://docs.microsoft.com/en-us/rest/api/storageservices/specifying-conditional-headers-for-blob-service-operations)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `filesystem`: The filesystem identifier."]
        #[doc = "* `path`: The file or directory path."]
        pub fn read(&self, filesystem: impl Into<String>, path: impl Into<String>) -> read::RequestBuilder {
            read::RequestBuilder {
                client: self.0.clone(),
                filesystem: filesystem.into(),
                path: path.into(),
                x_ms_client_request_id: None,
                timeout: None,
                range: None,
                x_ms_lease_id: None,
                x_ms_range_get_content_md5: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
                x_ms_encryption_key: None,
                x_ms_encryption_key_sha256: None,
                x_ms_encryption_algorithm: None,
            }
        }
        #[doc = "Lease Path"]
        #[doc = "Create and manage a lease to restrict write and delete access to the path. This operation supports conditional HTTP requests.  For more information, see [Specifying Conditional Headers for Blob Service Operations](https://docs.microsoft.com/en-us/rest/api/storageservices/specifying-conditional-headers-for-blob-service-operations)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `filesystem`: The filesystem identifier."]
        #[doc = "* `path`: The file or directory path."]
        #[doc = "* `x_ms_lease_action`: There are five lease actions: \"acquire\", \"break\", \"change\", \"renew\", and \"release\". Use \"acquire\" and specify the \"x-ms-proposed-lease-id\" and \"x-ms-lease-duration\" to acquire a new lease. Use \"break\" to break an existing lease. When a lease is broken, the lease break period is allowed to elapse, during which time no lease operation except break and release can be performed on the file. When a lease is successfully broken, the response indicates the interval in seconds until a new lease can be acquired. Use \"change\" and specify the current lease ID in \"x-ms-lease-id\" and the new lease ID in \"x-ms-proposed-lease-id\" to change the lease ID of an active lease. Use \"renew\" and specify the \"x-ms-lease-id\" to renew an existing lease. Use \"release\" and specify the \"x-ms-lease-id\" to release a lease."]
        pub fn lease(
            &self,
            filesystem: impl Into<String>,
            path: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
        ) -> lease::RequestBuilder {
            lease::RequestBuilder {
                client: self.0.clone(),
                filesystem: filesystem.into(),
                path: path.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                x_ms_client_request_id: None,
                timeout: None,
                x_ms_lease_duration: None,
                x_ms_lease_break_period: None,
                x_ms_lease_id: None,
                x_ms_proposed_lease_id: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Create File | Create Directory | Rename File | Rename Directory"]
        #[doc = "Create or rename a file or directory.    By default, the destination is overwritten and if the destination already exists and has a lease the lease is broken.  This operation supports conditional HTTP requests.  For more information, see [Specifying Conditional Headers for Blob Service Operations](https://docs.microsoft.com/en-us/rest/api/storageservices/specifying-conditional-headers-for-blob-service-operations).  To fail if the destination already exists, use a conditional request with If-None-Match: \"*\"."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `filesystem`: The filesystem identifier."]
        #[doc = "* `path`: The file or directory path."]
        pub fn create(&self, filesystem: impl Into<String>, path: impl Into<String>) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                filesystem: filesystem.into(),
                path: path.into(),
                x_ms_client_request_id: None,
                timeout: None,
                resource: None,
                continuation: None,
                mode: None,
                x_ms_cache_control: None,
                x_ms_content_encoding: None,
                x_ms_content_language: None,
                x_ms_content_disposition: None,
                x_ms_content_type: None,
                x_ms_rename_source: None,
                x_ms_lease_id: None,
                x_ms_source_lease_id: None,
                x_ms_properties: None,
                x_ms_permissions: None,
                x_ms_umask: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
                x_ms_source_if_match: None,
                x_ms_source_if_none_match: None,
                x_ms_source_if_modified_since: None,
                x_ms_source_if_unmodified_since: None,
                x_ms_encryption_key: None,
                x_ms_encryption_key_sha256: None,
                x_ms_encryption_algorithm: None,
            }
        }
        #[doc = "Append Data | Flush Data | Set Properties | Set Access Control"]
        #[doc = "Uploads data to be appended to a file, flushes (writes) previously uploaded data to a file, sets properties for a file or directory, or sets access control for a file or directory. Data can only be appended to a file. Concurrent writes to the same file using multiple clients are not supported. This operation supports conditional HTTP requests. For more information, see [Specifying Conditional Headers for Blob Service Operations](https://docs.microsoft.com/en-us/rest/api/storageservices/specifying-conditional-headers-for-blob-service-operations)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `filesystem`: The filesystem identifier."]
        #[doc = "* `path`: The file or directory path."]
        #[doc = "* `action`: The action must be \"append\" to upload data to be appended to a file, \"flush\" to flush previously uploaded data to a file, \"setProperties\" to set the properties of a file or directory, \"setAccessControl\" to set the owner, group, permissions, or access control list for a file or directory, or  \"setAccessControlRecursive\" to set the access control list for a directory recursively. Note that Hierarchical Namespace must be enabled for the account in order to use access control.  Also note that the Access Control List (ACL) includes permissions for the owner, owning group, and others, so the x-ms-permissions and x-ms-acl request headers are mutually exclusive."]
        #[doc = "* `mode`: Mode \"set\" sets POSIX access control rights on files and directories, \"modify\" modifies one or more POSIX access control rights  that pre-exist on files and directories, \"remove\" removes one or more POSIX access control rights  that were present earlier on files and directories"]
        #[doc = "* `body`: Initial data"]
        pub fn update(
            &self,
            filesystem: impl Into<String>,
            path: impl Into<String>,
            action: impl Into<String>,
            mode: impl Into<String>,
            body: impl Into<serde_json::Value>,
        ) -> update::RequestBuilder {
            update::RequestBuilder {
                client: self.0.clone(),
                filesystem: filesystem.into(),
                path: path.into(),
                action: action.into(),
                mode: mode.into(),
                body: body.into(),
                x_ms_client_request_id: None,
                timeout: None,
                max_records: None,
                continuation: None,
                force_flag: None,
                position: None,
                retain_uncommitted_data: None,
                close: None,
                content_length: None,
                x_ms_content_md5: None,
                x_ms_lease_id: None,
                x_ms_cache_control: None,
                x_ms_content_type: None,
                x_ms_content_disposition: None,
                x_ms_content_encoding: None,
                x_ms_content_language: None,
                x_ms_properties: None,
                x_ms_owner: None,
                x_ms_group: None,
                x_ms_permissions: None,
                x_ms_acl: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Delete File | Delete Directory"]
        #[doc = "Delete the file or directory. This operation supports conditional HTTP requests.  For more information, see [Specifying Conditional Headers for Blob Service Operations](https://docs.microsoft.com/en-us/rest/api/storageservices/specifying-conditional-headers-for-blob-service-operations)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `filesystem`: The filesystem identifier."]
        #[doc = "* `path`: The file or directory path."]
        pub fn delete(&self, filesystem: impl Into<String>, path: impl Into<String>) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                filesystem: filesystem.into(),
                path: path.into(),
                x_ms_client_request_id: None,
                timeout: None,
                recursive: None,
                continuation: None,
                x_ms_lease_id: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Get Properties | Get Status | Get Access Control List"]
        #[doc = "Get Properties returns all system and user defined properties for a path. Get Status returns all system defined properties for a path. Get Access Control List returns the access control list for a path. This operation supports conditional HTTP requests.  For more information, see [Specifying Conditional Headers for Blob Service Operations](https://docs.microsoft.com/en-us/rest/api/storageservices/specifying-conditional-headers-for-blob-service-operations)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `filesystem`: The filesystem identifier."]
        #[doc = "* `path`: The file or directory path."]
        pub fn get_properties(&self, filesystem: impl Into<String>, path: impl Into<String>) -> get_properties::RequestBuilder {
            get_properties::RequestBuilder {
                client: self.0.clone(),
                filesystem: filesystem.into(),
                path: path.into(),
                x_ms_client_request_id: None,
                timeout: None,
                action: None,
                upn: None,
                x_ms_lease_id: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
            }
        }
        #[doc = "Set the owner, group, permissions, or access control list for a path."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `filesystem`: The filesystem identifier."]
        #[doc = "* `path`: The file or directory path."]
        pub fn set_access_control(&self, filesystem: impl Into<String>, path: impl Into<String>) -> set_access_control::RequestBuilder {
            set_access_control::RequestBuilder {
                client: self.0.clone(),
                filesystem: filesystem.into(),
                path: path.into(),
                timeout: None,
                x_ms_lease_id: None,
                x_ms_owner: None,
                x_ms_group: None,
                x_ms_permissions: None,
                x_ms_acl: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Set the access control list for a path and sub-paths."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `filesystem`: The filesystem identifier."]
        #[doc = "* `path`: The file or directory path."]
        #[doc = "* `mode`: Mode \"set\" sets POSIX access control rights on files and directories, \"modify\" modifies one or more POSIX access control rights  that pre-exist on files and directories, \"remove\" removes one or more POSIX access control rights  that were present earlier on files and directories"]
        pub fn set_access_control_recursive(
            &self,
            filesystem: impl Into<String>,
            path: impl Into<String>,
            mode: impl Into<String>,
        ) -> set_access_control_recursive::RequestBuilder {
            set_access_control_recursive::RequestBuilder {
                client: self.0.clone(),
                filesystem: filesystem.into(),
                path: path.into(),
                mode: mode.into(),
                timeout: None,
                continuation: None,
                force_flag: None,
                max_records: None,
                x_ms_acl: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "Set the owner, group, permissions, or access control list for a path."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `filesystem`: The filesystem identifier."]
        #[doc = "* `path`: The file or directory path."]
        pub fn flush_data(&self, filesystem: impl Into<String>, path: impl Into<String>) -> flush_data::RequestBuilder {
            flush_data::RequestBuilder {
                client: self.0.clone(),
                filesystem: filesystem.into(),
                path: path.into(),
                timeout: None,
                position: None,
                retain_uncommitted_data: None,
                close: None,
                content_length: None,
                x_ms_content_md5: None,
                x_ms_lease_id: None,
                x_ms_cache_control: None,
                x_ms_content_type: None,
                x_ms_content_disposition: None,
                x_ms_content_encoding: None,
                x_ms_content_language: None,
                if_match: None,
                if_none_match: None,
                if_modified_since: None,
                if_unmodified_since: None,
                x_ms_client_request_id: None,
                x_ms_encryption_key: None,
                x_ms_encryption_key_sha256: None,
                x_ms_encryption_algorithm: None,
            }
        }
        #[doc = "Append data to the file."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `filesystem`: The filesystem identifier."]
        #[doc = "* `path`: The file or directory path."]
        #[doc = "* `body`: Initial data"]
        pub fn append_data(
            &self,
            filesystem: impl Into<String>,
            path: impl Into<String>,
            body: impl Into<serde_json::Value>,
        ) -> append_data::RequestBuilder {
            append_data::RequestBuilder {
                client: self.0.clone(),
                filesystem: filesystem.into(),
                path: path.into(),
                body: body.into(),
                position: None,
                timeout: None,
                content_length: None,
                content_md5: None,
                x_ms_content_crc64: None,
                x_ms_lease_id: None,
                x_ms_client_request_id: None,
                x_ms_encryption_key: None,
                x_ms_encryption_key_sha256: None,
                x_ms_encryption_algorithm: None,
            }
        }
        #[doc = "Sets the time a blob will expire and be deleted."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `filesystem`: The filesystem identifier."]
        #[doc = "* `path`: The file or directory path."]
        #[doc = "* `x_ms_expiry_option`: Required. Indicates mode of the expiry time"]
        pub fn set_expiry(
            &self,
            filesystem: impl Into<String>,
            path: impl Into<String>,
            x_ms_expiry_option: impl Into<String>,
        ) -> set_expiry::RequestBuilder {
            set_expiry::RequestBuilder {
                client: self.0.clone(),
                filesystem: filesystem.into(),
                path: path.into(),
                x_ms_expiry_option: x_ms_expiry_option.into(),
                timeout: None,
                x_ms_client_request_id: None,
                x_ms_expiry_time: None,
            }
        }
        #[doc = "Undelete a path that was previously soft deleted"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `filesystem`: The filesystem identifier."]
        #[doc = "* `path`: The file or directory path."]
        pub fn undelete(&self, filesystem: impl Into<String>, path: impl Into<String>) -> undelete::RequestBuilder {
            undelete::RequestBuilder {
                client: self.0.clone(),
                filesystem: filesystem.into(),
                path: path.into(),
                timeout: None,
                x_ms_undelete_source: None,
                x_ms_client_request_id: None,
            }
        }
    }
    pub mod read {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<serde_json::Value> {
                let bytes = self.0.into_body().collect().await?;
                let body: serde_json::Value = serde_json::from_slice(&bytes)?;
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
            #[doc = "Indicates that the service supports requests for partial file content."]
            pub fn accept_ranges(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("accept-ranges"))
            }
            #[doc = "If the Cache-Control request header has previously been set for the resource, that value is returned in this header."]
            pub fn cache_control(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("cache-control"))
            }
            #[doc = "If the Content-Disposition request header has previously been set for the resource, that value is returned in this header."]
            pub fn content_disposition(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-disposition"))
            }
            #[doc = "If the Content-Encoding request header has previously been set for the resource, that value is returned in this header."]
            pub fn content_encoding(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-encoding"))
            }
            #[doc = "If the Content-Language request header has previously been set for the resource, that value is returned in this header."]
            pub fn content_language(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-language"))
            }
            #[doc = "The size of the resource in bytes."]
            pub fn content_length(&self) -> azure_core::Result<i64> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("content-length"))
            }
            #[doc = "Indicates the range of bytes returned in the event that the client requested a subset of the file by setting the Range request header."]
            pub fn content_range(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-range"))
            }
            #[doc = "The content type specified for the resource. If no content type was specified, the default content type is application/octet-stream."]
            pub fn content_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-type"))
            }
            #[doc = "The MD5 hash of read range. If the request is to read a specified range and the \"x-ms-range-get-content-md5\" is set to true, then the request returns an MD5 hash for the range, as long as the range size is less than or equal to 4 MB."]
            pub fn content_md5(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-md5"))
            }
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "An HTTP entity tag associated with the file or directory."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "The data and time the file or directory was last modified.  Write operations on the file or directory update the last modified time."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "A server-generated UUID recorded in the analytics logs for troubleshooting and correlation."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "The version of the REST protocol used to process the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "The type of the resource.  The value may be \"file\" or \"directory\".  If not set, the value is \"file\"."]
            pub fn x_ms_resource_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-resource-type"))
            }
            #[doc = "The user-defined properties associated with the file or directory, in the format of a comma-separated list of name and value pairs \"n1=v1, n2=v2, ...\", where each value is a base64 encoded string. Note that the string may only contain ASCII characters in the ISO-8859-1 character set."]
            pub fn x_ms_properties(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-properties"))
            }
            #[doc = "When a resource is leased, specifies whether the lease is of infinite or fixed duration."]
            pub fn x_ms_lease_duration(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-duration"))
            }
            #[doc = "Lease state of the resource. "]
            pub fn x_ms_lease_state(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-state"))
            }
            #[doc = "The lease status of the resource."]
            pub fn x_ms_lease_status(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-status"))
            }
            #[doc = "The value of this header is set to true if the contents of the request are successfully encrypted using the specified algorithm, and false otherwise."]
            pub fn x_ms_request_server_encrypted(&self) -> azure_core::Result<bool> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-request-server-encrypted"))
            }
            #[doc = "The SHA-256 hash of the encryption key used to encrypt the blob. This header is only returned when the blob was encrypted with a customer-provided key."]
            pub fn x_ms_encryption_key_sha256(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-encryption-key-sha256"))
            }
            #[doc = "The MD5 hash of complete file stored in storage. If the file has a MD5 hash, and if request contains range header (Range or x-ms-range), this response header is returned with the value of the complete file's MD5 value. This value may or may not be equal to the value returned in Content-MD5 header, with the latter calculated from the requested range."]
            pub fn x_ms_content_md5(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-content-md5"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) filesystem: String,
            pub(crate) path: String,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) range: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_range_get_content_md5: Option<bool>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_encryption_key: Option<String>,
            pub(crate) x_ms_encryption_key_sha256: Option<String>,
            pub(crate) x_ms_encryption_algorithm: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The HTTP Range request header specifies one or more byte ranges of the resource to be retrieved."]
            pub fn range(mut self, range: impl Into<String>) -> Self {
                self.range = Some(range.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Optional. When this header is set to \"true\" and specified together with the Range header, the service returns the MD5 hash for the range, as long as the range is less than or equal to 4MB in size. If this header is specified without the Range header, the service returns status code 400 (Bad Request). If this header is set to true when the range exceeds 4 MB in size, the service returns status code 400 (Bad Request)."]
            pub fn x_ms_range_get_content_md5(mut self, x_ms_range_get_content_md5: bool) -> Self {
                self.x_ms_range_get_content_md5 = Some(x_ms_range_get_content_md5);
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Optional. Specifies the encryption key to use to encrypt the data provided in the request. If not specified, encryption is performed with the root account encryption key.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_key(mut self, x_ms_encryption_key: impl Into<String>) -> Self {
                self.x_ms_encryption_key = Some(x_ms_encryption_key.into());
                self
            }
            #[doc = "The SHA-256 hash of the provided encryption key. Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_key_sha256(mut self, x_ms_encryption_key_sha256: impl Into<String>) -> Self {
                self.x_ms_encryption_key_sha256 = Some(x_ms_encryption_key_sha256.into());
                self
            }
            #[doc = "The algorithm used to produce the encryption key hash. Currently, the only accepted value is \"AES256\". Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_algorithm(mut self, x_ms_encryption_algorithm: impl Into<String>) -> Self {
                self.x_ms_encryption_algorithm = Some(x_ms_encryption_algorithm.into());
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
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-06-08");
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(range) = &this.range {
                            req.insert_header("range", range);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_range_get_content_md5) = &this.x_ms_range_get_content_md5 {
                            req.insert_header("x-ms-range-get-content-md5", &x_ms_range_get_content_md5.to_string());
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(x_ms_encryption_key) = &this.x_ms_encryption_key {
                            req.insert_header("x-ms-encryption-key", x_ms_encryption_key);
                        }
                        if let Some(x_ms_encryption_key_sha256) = &this.x_ms_encryption_key_sha256 {
                            req.insert_header("x-ms-encryption-key-sha256", x_ms_encryption_key_sha256);
                        }
                        if let Some(x_ms_encryption_algorithm) = &this.x_ms_encryption_algorithm {
                            req.insert_header("x-ms-encryption-algorithm", x_ms_encryption_algorithm);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core::Url::parse(&format!("{}/{}/{}", self.client.endpoint(), &self.filesystem, &self.path))?;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<serde_json::Value>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<serde_json::Value>>;
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
    pub mod lease {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
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
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "An HTTP entity tag associated with the file or directory."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "The data and time the file or directory was last modified.  Write operations on the file or directory update the last modified time."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "A server-generated UUID recorded in the analytics logs for troubleshooting and correlation."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "The version of the REST protocol used to process the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "A successful \"acquire\" action returns the lease ID."]
            pub fn x_ms_lease_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-id"))
            }
            #[doc = "The time remaining in the lease period in seconds."]
            pub fn x_ms_lease_time(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-time"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) filesystem: String,
            pub(crate) path: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_duration: Option<i32>,
            pub(crate) x_ms_lease_break_period: Option<i32>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_proposed_lease_id: Option<String>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl RequestBuilder {
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The lease duration is required to acquire a lease, and specifies the duration of the lease in seconds.  The lease duration must be between 15 and 60 seconds or -1 for infinite lease."]
            pub fn x_ms_lease_duration(mut self, x_ms_lease_duration: i32) -> Self {
                self.x_ms_lease_duration = Some(x_ms_lease_duration);
                self
            }
            #[doc = "The lease break period duration is optional to break a lease, and  specifies the break period of the lease in seconds.  The lease break  duration must be between 0 and 60 seconds."]
            pub fn x_ms_lease_break_period(mut self, x_ms_lease_break_period: i32) -> Self {
                self.x_ms_lease_break_period = Some(x_ms_lease_break_period);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Proposed lease ID, in a GUID string format. The Blob service returns 400 (Invalid request) if the proposed lease ID is not in the correct format. See Guid Constructor (String) for a list of valid GUID string formats."]
            pub fn x_ms_proposed_lease_id(mut self, x_ms_proposed_lease_id: impl Into<String>) -> Self {
                self.x_ms_proposed_lease_id = Some(x_ms_proposed_lease_id.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
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
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-06-08");
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(x_ms_lease_duration) = &this.x_ms_lease_duration {
                            req.insert_header("x-ms-lease-duration", &x_ms_lease_duration.to_string());
                        }
                        if let Some(x_ms_lease_break_period) = &this.x_ms_lease_break_period {
                            req.insert_header("x-ms-lease-break-period", &x_ms_lease_break_period.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_proposed_lease_id) = &this.x_ms_proposed_lease_id {
                            req.insert_header("x-ms-proposed-lease-id", x_ms_proposed_lease_id);
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core::Url::parse(&format!("{}/{}/{}", self.client.endpoint(), &self.filesystem, &self.path))?;
                Ok(url)
            }
        }
    }
    pub mod create {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
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
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "An HTTP entity tag associated with the file or directory."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "The data and time the file or directory was last modified.  Write operations on the file or directory update the last modified time."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "A server-generated UUID recorded in the analytics logs for troubleshooting and correlation."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "The version of the REST protocol used to process the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "When renaming a directory, the number of paths that are renamed with each invocation is limited.  If the number of paths to be renamed exceeds this limit, a continuation token is returned in this response header.  When a continuation token is returned in the response, it must be specified in a subsequent invocation of the rename operation to continue renaming the directory."]
            pub fn x_ms_continuation(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-continuation"))
            }
            #[doc = "The size of the resource in bytes."]
            pub fn content_length(&self) -> azure_core::Result<i64> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("content-length"))
            }
            #[doc = "The value of this header is set to true if the contents of the request are successfully encrypted using the specified algorithm, and false otherwise."]
            pub fn x_ms_request_server_encrypted(&self) -> azure_core::Result<bool> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-request-server-encrypted"))
            }
            #[doc = "The SHA-256 hash of the encryption key used to encrypt the blob. This header is only returned when the blob was encrypted with a customer-provided key."]
            pub fn x_ms_encryption_key_sha256(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-encryption-key-sha256"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) filesystem: String,
            pub(crate) path: String,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) resource: Option<String>,
            pub(crate) continuation: Option<String>,
            pub(crate) mode: Option<String>,
            pub(crate) x_ms_cache_control: Option<String>,
            pub(crate) x_ms_content_encoding: Option<String>,
            pub(crate) x_ms_content_language: Option<String>,
            pub(crate) x_ms_content_disposition: Option<String>,
            pub(crate) x_ms_content_type: Option<String>,
            pub(crate) x_ms_rename_source: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_source_lease_id: Option<String>,
            pub(crate) x_ms_properties: Option<String>,
            pub(crate) x_ms_permissions: Option<String>,
            pub(crate) x_ms_umask: Option<String>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_source_if_match: Option<String>,
            pub(crate) x_ms_source_if_none_match: Option<String>,
            pub(crate) x_ms_source_if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_source_if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_encryption_key: Option<String>,
            pub(crate) x_ms_encryption_key_sha256: Option<String>,
            pub(crate) x_ms_encryption_algorithm: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Required only for Create File and Create Directory. The value must be \"file\" or \"directory\"."]
            pub fn resource(mut self, resource: impl Into<String>) -> Self {
                self.resource = Some(resource.into());
                self
            }
            #[doc = "Optional.  When deleting a directory, the number of paths that are deleted with each invocation is limited.  If the number of paths to be deleted exceeds this limit, a continuation token is returned in this response header.  When a continuation token is returned in the response, it must be specified in a subsequent invocation of the delete operation to continue deleting the directory."]
            pub fn continuation(mut self, continuation: impl Into<String>) -> Self {
                self.continuation = Some(continuation.into());
                self
            }
            #[doc = "Optional. Valid only when namespace is enabled. This parameter determines the behavior of the rename operation. The value must be \"legacy\" or \"posix\", and the default value will be \"posix\"."]
            pub fn mode(mut self, mode: impl Into<String>) -> Self {
                self.mode = Some(mode.into());
                self
            }
            #[doc = "Optional. Sets the blob's cache control. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_cache_control(mut self, x_ms_cache_control: impl Into<String>) -> Self {
                self.x_ms_cache_control = Some(x_ms_cache_control.into());
                self
            }
            #[doc = "Optional. Sets the blob's content encoding. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_content_encoding(mut self, x_ms_content_encoding: impl Into<String>) -> Self {
                self.x_ms_content_encoding = Some(x_ms_content_encoding.into());
                self
            }
            #[doc = "Optional. Set the blob's content language. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_content_language(mut self, x_ms_content_language: impl Into<String>) -> Self {
                self.x_ms_content_language = Some(x_ms_content_language.into());
                self
            }
            #[doc = "Optional. Sets the blob's Content-Disposition header."]
            pub fn x_ms_content_disposition(mut self, x_ms_content_disposition: impl Into<String>) -> Self {
                self.x_ms_content_disposition = Some(x_ms_content_disposition.into());
                self
            }
            #[doc = "Optional. Sets the blob's content type. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_content_type(mut self, x_ms_content_type: impl Into<String>) -> Self {
                self.x_ms_content_type = Some(x_ms_content_type.into());
                self
            }
            #[doc = "An optional file or directory to be renamed.  The value must have the following format: \"/{filesystem}/{path}\".  If \"x-ms-properties\" is specified, the properties will overwrite the existing properties; otherwise, the existing properties will be preserved. This value must be a URL percent-encoded string. Note that the string may only contain ASCII characters in the ISO-8859-1 character set."]
            pub fn x_ms_rename_source(mut self, x_ms_rename_source: impl Into<String>) -> Self {
                self.x_ms_rename_source = Some(x_ms_rename_source.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "A lease ID for the source path. If specified, the source path must have an active lease and the lease ID must match."]
            pub fn x_ms_source_lease_id(mut self, x_ms_source_lease_id: impl Into<String>) -> Self {
                self.x_ms_source_lease_id = Some(x_ms_source_lease_id.into());
                self
            }
            #[doc = "Optional. User-defined properties to be stored with the filesystem, in the format of a comma-separated list of name and value pairs \"n1=v1, n2=v2, ...\", where each value is a base64 encoded string. Note that the string may only contain ASCII characters in the ISO-8859-1 character set.  If the filesystem exists, any properties not included in the list will be removed.  All properties are removed if the header is omitted.  To merge new and existing properties, first get all existing properties and the current E-Tag, then make a conditional request with the E-Tag and include values for all properties."]
            pub fn x_ms_properties(mut self, x_ms_properties: impl Into<String>) -> Self {
                self.x_ms_properties = Some(x_ms_properties.into());
                self
            }
            #[doc = "Optional and only valid if Hierarchical Namespace is enabled for the account. Sets POSIX access permissions for the file owner, the file owning group, and others. Each class may be granted read, write, or execute permission.  The sticky bit is also supported.  Both symbolic (rwxrw-rw-) and 4-digit octal notation (e.g. 0766) are supported."]
            pub fn x_ms_permissions(mut self, x_ms_permissions: impl Into<String>) -> Self {
                self.x_ms_permissions = Some(x_ms_permissions.into());
                self
            }
            #[doc = "Optional and only valid if Hierarchical Namespace is enabled for the account. When creating a file or directory and the parent folder does not have a default ACL, the umask restricts the permissions of the file or directory to be created.  The resulting permission is given by p bitwise and not u, where p is the permission and u is the umask.  For example, if p is 0777 and u is 0057, then the resulting permission is 0720.  The default permission is 0777 for a directory and 0666 for a file.  The default umask is 0027.  The umask must be specified in 4-digit octal notation (e.g. 0766)."]
            pub fn x_ms_umask(mut self, x_ms_umask: impl Into<String>) -> Self {
                self.x_ms_umask = Some(x_ms_umask.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn x_ms_source_if_match(mut self, x_ms_source_if_match: impl Into<String>) -> Self {
                self.x_ms_source_if_match = Some(x_ms_source_if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn x_ms_source_if_none_match(mut self, x_ms_source_if_none_match: impl Into<String>) -> Self {
                self.x_ms_source_if_none_match = Some(x_ms_source_if_none_match.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn x_ms_source_if_modified_since(mut self, x_ms_source_if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_source_if_modified_since = Some(x_ms_source_if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn x_ms_source_if_unmodified_since(mut self, x_ms_source_if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_source_if_unmodified_since = Some(x_ms_source_if_unmodified_since.into());
                self
            }
            #[doc = "Optional. Specifies the encryption key to use to encrypt the data provided in the request. If not specified, encryption is performed with the root account encryption key.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_key(mut self, x_ms_encryption_key: impl Into<String>) -> Self {
                self.x_ms_encryption_key = Some(x_ms_encryption_key.into());
                self
            }
            #[doc = "The SHA-256 hash of the provided encryption key. Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_key_sha256(mut self, x_ms_encryption_key_sha256: impl Into<String>) -> Self {
                self.x_ms_encryption_key_sha256 = Some(x_ms_encryption_key_sha256.into());
                self
            }
            #[doc = "The algorithm used to produce the encryption key hash. Currently, the only accepted value is \"AES256\". Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_algorithm(mut self, x_ms_encryption_algorithm: impl Into<String>) -> Self {
                self.x_ms_encryption_algorithm = Some(x_ms_encryption_algorithm.into());
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
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-06-08");
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(resource) = &this.resource {
                            req.url_mut().query_pairs_mut().append_pair("resource", resource);
                        }
                        if let Some(continuation) = &this.continuation {
                            req.url_mut().query_pairs_mut().append_pair("continuation", continuation);
                        }
                        if let Some(mode) = &this.mode {
                            req.url_mut().query_pairs_mut().append_pair("mode", mode);
                        }
                        if let Some(x_ms_cache_control) = &this.x_ms_cache_control {
                            req.insert_header("x-ms-cache-control", x_ms_cache_control);
                        }
                        if let Some(x_ms_content_encoding) = &this.x_ms_content_encoding {
                            req.insert_header("x-ms-content-encoding", x_ms_content_encoding);
                        }
                        if let Some(x_ms_content_language) = &this.x_ms_content_language {
                            req.insert_header("x-ms-content-language", x_ms_content_language);
                        }
                        if let Some(x_ms_content_disposition) = &this.x_ms_content_disposition {
                            req.insert_header("x-ms-content-disposition", x_ms_content_disposition);
                        }
                        if let Some(x_ms_content_type) = &this.x_ms_content_type {
                            req.insert_header("x-ms-content-type", x_ms_content_type);
                        }
                        if let Some(x_ms_rename_source) = &this.x_ms_rename_source {
                            req.insert_header("x-ms-rename-source", x_ms_rename_source);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_source_lease_id) = &this.x_ms_source_lease_id {
                            req.insert_header("x-ms-source-lease-id", x_ms_source_lease_id);
                        }
                        if let Some(x_ms_properties) = &this.x_ms_properties {
                            req.insert_header("x-ms-properties", x_ms_properties);
                        }
                        if let Some(x_ms_permissions) = &this.x_ms_permissions {
                            req.insert_header("x-ms-permissions", x_ms_permissions);
                        }
                        if let Some(x_ms_umask) = &this.x_ms_umask {
                            req.insert_header("x-ms-umask", x_ms_umask);
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(x_ms_source_if_match) = &this.x_ms_source_if_match {
                            req.insert_header("x-ms-source-if-match", x_ms_source_if_match);
                        }
                        if let Some(x_ms_source_if_none_match) = &this.x_ms_source_if_none_match {
                            req.insert_header("x-ms-source-if-none-match", x_ms_source_if_none_match);
                        }
                        if let Some(x_ms_source_if_modified_since) = &this.x_ms_source_if_modified_since {
                            req.insert_header("x-ms-source-if-modified-since", &x_ms_source_if_modified_since.to_string());
                        }
                        if let Some(x_ms_source_if_unmodified_since) = &this.x_ms_source_if_unmodified_since {
                            req.insert_header("x-ms-source-if-unmodified-since", &x_ms_source_if_unmodified_since.to_string());
                        }
                        if let Some(x_ms_encryption_key) = &this.x_ms_encryption_key {
                            req.insert_header("x-ms-encryption-key", x_ms_encryption_key);
                        }
                        if let Some(x_ms_encryption_key_sha256) = &this.x_ms_encryption_key_sha256 {
                            req.insert_header("x-ms-encryption-key-sha256", x_ms_encryption_key_sha256);
                        }
                        if let Some(x_ms_encryption_algorithm) = &this.x_ms_encryption_algorithm {
                            req.insert_header("x-ms-encryption-algorithm", x_ms_encryption_algorithm);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core::Url::parse(&format!("{}/{}/{}", self.client.endpoint(), &self.filesystem, &self.path))?;
                Ok(url)
            }
        }
    }
    pub mod update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SetAccessControlRecursiveResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SetAccessControlRecursiveResponse = serde_json::from_slice(&bytes)?;
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
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "An HTTP entity tag associated with the file or directory."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "The data and time the file or directory was last modified.  Write operations on the file or directory update the last modified time."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "Indicates that the service supports requests for partial file content."]
            pub fn accept_ranges(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("accept-ranges"))
            }
            #[doc = "If the Cache-Control request header has previously been set for the resource, that value is returned in this header."]
            pub fn cache_control(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("cache-control"))
            }
            #[doc = "If the Content-Disposition request header has previously been set for the resource, that value is returned in this header."]
            pub fn content_disposition(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-disposition"))
            }
            #[doc = "If the Content-Encoding request header has previously been set for the resource, that value is returned in this header."]
            pub fn content_encoding(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-encoding"))
            }
            #[doc = "If the Content-Language request header has previously been set for the resource, that value is returned in this header."]
            pub fn content_language(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-language"))
            }
            #[doc = "The size of the resource in bytes."]
            pub fn content_length(&self) -> azure_core::Result<i64> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("content-length"))
            }
            #[doc = "Indicates the range of bytes returned in the event that the client requested a subset of the file by setting the Range request header."]
            pub fn content_range(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-range"))
            }
            #[doc = "The content type specified for the resource. If no content type was specified, the default content type is application/octet-stream."]
            pub fn content_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-type"))
            }
            #[doc = "An MD5 hash of the request content. This header is only returned for \"Append\" operation. This header is returned so that the client can check for message content integrity. The value of this header is computed by the service; it is not necessarily the same value specified in the request headers."]
            pub fn content_md5(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-md5"))
            }
            #[doc = "User-defined properties associated with the file or directory, in the format of a comma-separated list of name and value pairs \"n1=v1, n2=v2, ...\", where each value is a base64 encoded string. Note that the string may only contain ASCII characters in the ISO-8859-1 character set."]
            pub fn x_ms_properties(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-properties"))
            }
            #[doc = "When performing setAccessControlRecursive on a directory, the number of paths that are processed with each invocation is limited.  If the number of paths to be processed exceeds this limit, a continuation token is returned in this response header.  When a continuation token is returned in the response, it must be specified in a subsequent invocation of the setAccessControlRecursive operation to continue the setAccessControlRecursive operation on the directory."]
            pub fn x_ms_continuation(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-continuation"))
            }
            #[doc = "A server-generated UUID recorded in the analytics logs for troubleshooting and correlation."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "The version of the REST protocol used to process the request."]
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
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) filesystem: String,
            pub(crate) path: String,
            pub(crate) action: String,
            pub(crate) mode: String,
            pub(crate) body: serde_json::Value,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) max_records: Option<i32>,
            pub(crate) continuation: Option<String>,
            pub(crate) force_flag: Option<bool>,
            pub(crate) position: Option<i64>,
            pub(crate) retain_uncommitted_data: Option<bool>,
            pub(crate) close: Option<bool>,
            pub(crate) content_length: Option<i64>,
            pub(crate) x_ms_content_md5: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_cache_control: Option<String>,
            pub(crate) x_ms_content_type: Option<String>,
            pub(crate) x_ms_content_disposition: Option<String>,
            pub(crate) x_ms_content_encoding: Option<String>,
            pub(crate) x_ms_content_language: Option<String>,
            pub(crate) x_ms_properties: Option<String>,
            pub(crate) x_ms_owner: Option<String>,
            pub(crate) x_ms_group: Option<String>,
            pub(crate) x_ms_permissions: Option<String>,
            pub(crate) x_ms_acl: Option<String>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl RequestBuilder {
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional. Valid for \"SetAccessControlRecursive\" operation. It specifies the maximum number of files or directories on which the acl change will be applied. If omitted or greater than 2,000, the request will process up to 2,000 items"]
            pub fn max_records(mut self, max_records: i32) -> Self {
                self.max_records = Some(max_records);
                self
            }
            #[doc = "Optional. The number of paths processed with each invocation is limited. If the number of paths to be processed exceeds this limit, a continuation token is returned in the response header x-ms-continuation. When a continuation token is  returned in the response, it must be percent-encoded and specified in a subsequent invocation of setAccessControlRecursive operation."]
            pub fn continuation(mut self, continuation: impl Into<String>) -> Self {
                self.continuation = Some(continuation.into());
                self
            }
            #[doc = "Optional. Valid for \"SetAccessControlRecursive\" operation. If set to false, the operation will terminate quickly on encountering user errors (4XX). If true, the operation will ignore user errors and proceed with the operation on other sub-entities of the directory. Continuation token will only be returned when forceFlag is true in case of user errors. If not set the default value is false for this."]
            pub fn force_flag(mut self, force_flag: bool) -> Self {
                self.force_flag = Some(force_flag);
                self
            }
            #[doc = "This parameter allows the caller to upload data in parallel and control the order in which it is appended to the file.  It is required when uploading data to be appended to the file and when flushing previously uploaded data to the file.  The value must be the position where the data is to be appended.  Uploaded data is not immediately flushed, or written, to the file.  To flush, the previously uploaded data must be contiguous, the position parameter must be specified and equal to the length of the file after all data has been written, and there must not be a request entity body included with the request."]
            pub fn position(mut self, position: i64) -> Self {
                self.position = Some(position);
                self
            }
            #[doc = "Valid only for flush operations.  If \"true\", uncommitted data is retained after the flush operation completes; otherwise, the uncommitted data is deleted after the flush operation.  The default is false.  Data at offsets less than the specified position are written to the file when flush succeeds, but this optional parameter allows data after the flush position to be retained for a future flush operation."]
            pub fn retain_uncommitted_data(mut self, retain_uncommitted_data: bool) -> Self {
                self.retain_uncommitted_data = Some(retain_uncommitted_data);
                self
            }
            #[doc = "Azure Storage Events allow applications to receive notifications when files change. When Azure Storage Events are enabled, a file changed event is raised. This event has a property indicating whether this is the final change to distinguish the difference between an intermediate flush to a file stream and the final close of a file stream. The close query parameter is valid only when the action is \"flush\" and change notifications are enabled. If the value of close is \"true\" and the flush operation completes successfully, the service raises a file change notification with a property indicating that this is the final update (the file stream has been closed). If \"false\" a change notification is raised indicating the file has changed. The default is false. This query parameter is set to true by the Hadoop ABFS driver to indicate that the file stream has been closed.\""]
            pub fn close(mut self, close: bool) -> Self {
                self.close = Some(close);
                self
            }
            #[doc = "Required for \"Append Data\" and \"Flush Data\".  Must be 0 for \"Flush Data\".  Must be the length of the request content in bytes for \"Append Data\"."]
            pub fn content_length(mut self, content_length: i64) -> Self {
                self.content_length = Some(content_length);
                self
            }
            #[doc = "Specify the transactional md5 for the body, to be validated by the service."]
            pub fn x_ms_content_md5(mut self, x_ms_content_md5: impl Into<String>) -> Self {
                self.x_ms_content_md5 = Some(x_ms_content_md5.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Optional. Sets the blob's cache control. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_cache_control(mut self, x_ms_cache_control: impl Into<String>) -> Self {
                self.x_ms_cache_control = Some(x_ms_cache_control.into());
                self
            }
            #[doc = "Optional. Sets the blob's content type. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_content_type(mut self, x_ms_content_type: impl Into<String>) -> Self {
                self.x_ms_content_type = Some(x_ms_content_type.into());
                self
            }
            #[doc = "Optional. Sets the blob's Content-Disposition header."]
            pub fn x_ms_content_disposition(mut self, x_ms_content_disposition: impl Into<String>) -> Self {
                self.x_ms_content_disposition = Some(x_ms_content_disposition.into());
                self
            }
            #[doc = "Optional. Sets the blob's content encoding. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_content_encoding(mut self, x_ms_content_encoding: impl Into<String>) -> Self {
                self.x_ms_content_encoding = Some(x_ms_content_encoding.into());
                self
            }
            #[doc = "Optional. Set the blob's content language. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_content_language(mut self, x_ms_content_language: impl Into<String>) -> Self {
                self.x_ms_content_language = Some(x_ms_content_language.into());
                self
            }
            #[doc = "Optional. User-defined properties to be stored with the filesystem, in the format of a comma-separated list of name and value pairs \"n1=v1, n2=v2, ...\", where each value is a base64 encoded string. Note that the string may only contain ASCII characters in the ISO-8859-1 character set.  If the filesystem exists, any properties not included in the list will be removed.  All properties are removed if the header is omitted.  To merge new and existing properties, first get all existing properties and the current E-Tag, then make a conditional request with the E-Tag and include values for all properties."]
            pub fn x_ms_properties(mut self, x_ms_properties: impl Into<String>) -> Self {
                self.x_ms_properties = Some(x_ms_properties.into());
                self
            }
            #[doc = "Optional. The owner of the blob or directory."]
            pub fn x_ms_owner(mut self, x_ms_owner: impl Into<String>) -> Self {
                self.x_ms_owner = Some(x_ms_owner.into());
                self
            }
            #[doc = "Optional. The owning group of the blob or directory."]
            pub fn x_ms_group(mut self, x_ms_group: impl Into<String>) -> Self {
                self.x_ms_group = Some(x_ms_group.into());
                self
            }
            #[doc = "Optional and only valid if Hierarchical Namespace is enabled for the account. Sets POSIX access permissions for the file owner, the file owning group, and others. Each class may be granted read, write, or execute permission.  The sticky bit is also supported.  Both symbolic (rwxrw-rw-) and 4-digit octal notation (e.g. 0766) are supported."]
            pub fn x_ms_permissions(mut self, x_ms_permissions: impl Into<String>) -> Self {
                self.x_ms_permissions = Some(x_ms_permissions.into());
                self
            }
            #[doc = "Sets POSIX access control rights on files and directories. The value is a comma-separated list of access control entries. Each access control entry (ACE) consists of a scope, a type, a user or group identifier, and permissions in the format \"[scope:][type]:[id]:[permissions]\"."]
            pub fn x_ms_acl(mut self, x_ms_acl: impl Into<String>) -> Self {
                self.x_ms_acl = Some(x_ms_acl.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
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
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-06-08");
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        let action = &this.action;
                        req.url_mut().query_pairs_mut().append_pair("action", action);
                        if let Some(max_records) = &this.max_records {
                            req.url_mut().query_pairs_mut().append_pair("maxRecords", &max_records.to_string());
                        }
                        if let Some(continuation) = &this.continuation {
                            req.url_mut().query_pairs_mut().append_pair("continuation", continuation);
                        }
                        let mode = &this.mode;
                        req.url_mut().query_pairs_mut().append_pair("mode", mode);
                        if let Some(force_flag) = &this.force_flag {
                            req.url_mut().query_pairs_mut().append_pair("forceFlag", &force_flag.to_string());
                        }
                        if let Some(position) = &this.position {
                            req.url_mut().query_pairs_mut().append_pair("position", &position.to_string());
                        }
                        if let Some(retain_uncommitted_data) = &this.retain_uncommitted_data {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("retainUncommittedData", &retain_uncommitted_data.to_string());
                        }
                        if let Some(close) = &this.close {
                            req.url_mut().query_pairs_mut().append_pair("close", &close.to_string());
                        }
                        if let Some(content_length) = &this.content_length {
                            req.insert_header("content-length", &content_length.to_string());
                        }
                        if let Some(x_ms_content_md5) = &this.x_ms_content_md5 {
                            req.insert_header("x-ms-content-md5", x_ms_content_md5);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_cache_control) = &this.x_ms_cache_control {
                            req.insert_header("x-ms-cache-control", x_ms_cache_control);
                        }
                        if let Some(x_ms_content_type) = &this.x_ms_content_type {
                            req.insert_header("x-ms-content-type", x_ms_content_type);
                        }
                        if let Some(x_ms_content_disposition) = &this.x_ms_content_disposition {
                            req.insert_header("x-ms-content-disposition", x_ms_content_disposition);
                        }
                        if let Some(x_ms_content_encoding) = &this.x_ms_content_encoding {
                            req.insert_header("x-ms-content-encoding", x_ms_content_encoding);
                        }
                        if let Some(x_ms_content_language) = &this.x_ms_content_language {
                            req.insert_header("x-ms-content-language", x_ms_content_language);
                        }
                        if let Some(x_ms_properties) = &this.x_ms_properties {
                            req.insert_header("x-ms-properties", x_ms_properties);
                        }
                        if let Some(x_ms_owner) = &this.x_ms_owner {
                            req.insert_header("x-ms-owner", x_ms_owner);
                        }
                        if let Some(x_ms_group) = &this.x_ms_group {
                            req.insert_header("x-ms-group", x_ms_group);
                        }
                        if let Some(x_ms_permissions) = &this.x_ms_permissions {
                            req.insert_header("x-ms-permissions", x_ms_permissions);
                        }
                        if let Some(x_ms_acl) = &this.x_ms_acl {
                            req.insert_header("x-ms-acl", x_ms_acl);
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        req.insert_header("content-type", "application/octet-stream");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core::Url::parse(&format!("{}/{}/{}", self.client.endpoint(), &self.filesystem, &self.path))?;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SetAccessControlRecursiveResponse>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SetAccessControlRecursiveResponse>>;
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
    pub mod delete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
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
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "A server-generated UUID recorded in the analytics logs for troubleshooting and correlation."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "The version of the REST protocol used to process the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "When deleting a directory, the number of paths that are deleted with each invocation is limited.  If the number of paths to be deleted exceeds this limit, a continuation token is returned in this response header.  When a continuation token is returned in the response, it must be specified in a subsequent invocation of the delete operation to continue deleting the directory."]
            pub fn x_ms_continuation(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-continuation"))
            }
            #[doc = "Returned only for hierarchical namespace space enabled accounts when soft delete is enabled. A unique identifier for the entity that can be used to restore it. See the Undelete REST API for more information."]
            pub fn x_ms_deletion_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-deletion-id"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) filesystem: String,
            pub(crate) path: String,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) recursive: Option<bool>,
            pub(crate) continuation: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl RequestBuilder {
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Required"]
            pub fn recursive(mut self, recursive: bool) -> Self {
                self.recursive = Some(recursive);
                self
            }
            #[doc = "Optional.  When deleting a directory, the number of paths that are deleted with each invocation is limited.  If the number of paths to be deleted exceeds this limit, a continuation token is returned in this response header.  When a continuation token is returned in the response, it must be specified in a subsequent invocation of the delete operation to continue deleting the directory."]
            pub fn continuation(mut self, continuation: impl Into<String>) -> Self {
                self.continuation = Some(continuation.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
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
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-06-08");
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(recursive) = &this.recursive {
                            req.url_mut().query_pairs_mut().append_pair("recursive", &recursive.to_string());
                        }
                        if let Some(continuation) = &this.continuation {
                            req.url_mut().query_pairs_mut().append_pair("continuation", continuation);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core::Url::parse(&format!("{}/{}/{}", self.client.endpoint(), &self.filesystem, &self.path))?;
                Ok(url)
            }
        }
    }
    pub mod get_properties {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
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
            #[doc = "Indicates that the service supports requests for partial file content."]
            pub fn accept_ranges(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("accept-ranges"))
            }
            #[doc = "If the Cache-Control request header has previously been set for the resource, that value is returned in this header."]
            pub fn cache_control(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("cache-control"))
            }
            #[doc = "If the Content-Disposition request header has previously been set for the resource, that value is returned in this header."]
            pub fn content_disposition(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-disposition"))
            }
            #[doc = "If the Content-Encoding request header has previously been set for the resource, that value is returned in this header."]
            pub fn content_encoding(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-encoding"))
            }
            #[doc = "If the Content-Language request header has previously been set for the resource, that value is returned in this header."]
            pub fn content_language(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-language"))
            }
            #[doc = "The size of the resource in bytes."]
            pub fn content_length(&self) -> azure_core::Result<i64> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("content-length"))
            }
            #[doc = "Indicates the range of bytes returned in the event that the client requested a subset of the file by setting the Range request header."]
            pub fn content_range(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-range"))
            }
            #[doc = "The content type specified for the resource. If no content type was specified, the default content type is application/octet-stream."]
            pub fn content_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-type"))
            }
            #[doc = "The MD5 hash of complete file stored in storage. This header is returned only for \"GetProperties\" operation. If the Content-MD5 header has been set for the file, this response header is returned for GetProperties call so that the client can check for message content integrity."]
            pub fn content_md5(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-md5"))
            }
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "An HTTP entity tag associated with the file or directory."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "The data and time the file or directory was last modified.  Write operations on the file or directory update the last modified time."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "A server-generated UUID recorded in the analytics logs for troubleshooting and correlation."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "The version of the REST protocol used to process the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "The type of the resource.  The value may be \"file\" or \"directory\".  If not set, the value is \"file\"."]
            pub fn x_ms_resource_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-resource-type"))
            }
            #[doc = "The user-defined properties associated with the file or directory, in the format of a comma-separated list of name and value pairs \"n1=v1, n2=v2, ...\", where each value is a base64 encoded string. Note that the string may only contain ASCII characters in the ISO-8859-1 character set."]
            pub fn x_ms_properties(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-properties"))
            }
            #[doc = "The owner of the file or directory. Included in the response if Hierarchical Namespace is enabled for the account."]
            pub fn x_ms_owner(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-owner"))
            }
            #[doc = "The owning group of the file or directory. Included in the response if Hierarchical Namespace is enabled for the account."]
            pub fn x_ms_group(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-group"))
            }
            #[doc = "The POSIX access permissions for the file owner, the file owning group, and others. Included in the response if Hierarchical Namespace is enabled for the account."]
            pub fn x_ms_permissions(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-permissions"))
            }
            #[doc = "The POSIX access control list for the file or directory.  Included in the response only if the action is \"getAccessControl\" and Hierarchical Namespace is enabled for the account."]
            pub fn x_ms_acl(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-acl"))
            }
            #[doc = "When a resource is leased, specifies whether the lease is of infinite or fixed duration."]
            pub fn x_ms_lease_duration(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-duration"))
            }
            #[doc = "Lease state of the resource."]
            pub fn x_ms_lease_state(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-state"))
            }
            #[doc = "The lease status of the resource."]
            pub fn x_ms_lease_status(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-status"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) filesystem: String,
            pub(crate) path: String,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) action: Option<String>,
            pub(crate) upn: Option<bool>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
        }
        impl RequestBuilder {
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional. If the value is \"getStatus\" only the system defined properties for the path are returned. If the value is \"getAccessControl\" the access control list is returned in the response headers (Hierarchical Namespace must be enabled for the account), otherwise the properties are returned."]
            pub fn action(mut self, action: impl Into<String>) -> Self {
                self.action = Some(action.into());
                self
            }
            #[doc = "Optional. Valid only when Hierarchical Namespace is enabled for the account. If \"true\", the user identity values returned in the x-ms-owner, x-ms-group, and x-ms-acl response headers will be transformed from Azure Active Directory Object IDs to User Principal Names.  If \"false\", the values will be returned as Azure Active Directory Object IDs. The default value is false. Note that group and application Object IDs are not translated because they do not have unique friendly names."]
            pub fn upn(mut self, upn: bool) -> Self {
                self.upn = Some(upn);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
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
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Head);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-06-08");
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(action) = &this.action {
                            req.url_mut().query_pairs_mut().append_pair("action", action);
                        }
                        if let Some(upn) = &this.upn {
                            req.url_mut().query_pairs_mut().append_pair("upn", &upn.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core::Url::parse(&format!("{}/{}/{}", self.client.endpoint(), &self.filesystem, &self.path))?;
                Ok(url)
            }
        }
    }
    pub mod set_access_control {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
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
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "An HTTP entity tag associated with the file or directory."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "The data and time the file or directory was last modified. Write operations on the file or directory update the last modified time."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "If a client request id header is sent in the request, this header will be present in the response with the same value."]
            pub fn x_ms_client_request_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-client-request-id"))
            }
            #[doc = "A server-generated UUID recorded in the analytics logs for troubleshooting and correlation."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "The version of the REST protocol used to process the request."]
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
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) filesystem: String,
            pub(crate) path: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_owner: Option<String>,
            pub(crate) x_ms_group: Option<String>,
            pub(crate) x_ms_permissions: Option<String>,
            pub(crate) x_ms_acl: Option<String>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Optional. The owner of the blob or directory."]
            pub fn x_ms_owner(mut self, x_ms_owner: impl Into<String>) -> Self {
                self.x_ms_owner = Some(x_ms_owner.into());
                self
            }
            #[doc = "Optional. The owning group of the blob or directory."]
            pub fn x_ms_group(mut self, x_ms_group: impl Into<String>) -> Self {
                self.x_ms_group = Some(x_ms_group.into());
                self
            }
            #[doc = "Optional and only valid if Hierarchical Namespace is enabled for the account. Sets POSIX access permissions for the file owner, the file owning group, and others. Each class may be granted read, write, or execute permission.  The sticky bit is also supported.  Both symbolic (rwxrw-rw-) and 4-digit octal notation (e.g. 0766) are supported."]
            pub fn x_ms_permissions(mut self, x_ms_permissions: impl Into<String>) -> Self {
                self.x_ms_permissions = Some(x_ms_permissions.into());
                self
            }
            #[doc = "Sets POSIX access control rights on files and directories. The value is a comma-separated list of access control entries. Each access control entry (ACE) consists of a scope, a type, a user or group identifier, and permissions in the format \"[scope:][type]:[id]:[permissions]\"."]
            pub fn x_ms_acl(mut self, x_ms_acl: impl Into<String>) -> Self {
                self.x_ms_acl = Some(x_ms_acl.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
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
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-06-08");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_owner) = &this.x_ms_owner {
                            req.insert_header("x-ms-owner", x_ms_owner);
                        }
                        if let Some(x_ms_group) = &this.x_ms_group {
                            req.insert_header("x-ms-group", x_ms_group);
                        }
                        if let Some(x_ms_permissions) = &this.x_ms_permissions {
                            req.insert_header("x-ms-permissions", x_ms_permissions);
                        }
                        if let Some(x_ms_acl) = &this.x_ms_acl {
                            req.insert_header("x-ms-acl", x_ms_acl);
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core::Url::parse(&format!(
                    "{}/{}/{}?action=setAccessControl",
                    self.client.endpoint(),
                    &self.filesystem,
                    &self.path
                ))?;
                Ok(url)
            }
        }
    }
    pub mod set_access_control_recursive {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SetAccessControlRecursiveResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SetAccessControlRecursiveResponse = serde_json::from_slice(&bytes)?;
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
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "If a client request id header is sent in the request, this header will be present in the response with the same value."]
            pub fn x_ms_client_request_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-client-request-id"))
            }
            #[doc = "When performing setAccessControlRecursive on a directory, the number of paths that are processed with each invocation is limited.  If the number of paths to be processed exceeds this limit, a continuation token is returned in this response header.  When a continuation token is returned in the response, it must be specified in a subsequent invocation of the setAccessControlRecursive operation to continue the setAccessControlRecursive operation on the directory."]
            pub fn x_ms_continuation(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-continuation"))
            }
            #[doc = "A server-generated UUID recorded in the analytics logs for troubleshooting and correlation."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "The version of the REST protocol used to process the request."]
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
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) filesystem: String,
            pub(crate) path: String,
            pub(crate) mode: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) continuation: Option<String>,
            pub(crate) force_flag: Option<bool>,
            pub(crate) max_records: Option<i32>,
            pub(crate) x_ms_acl: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional.  When deleting a directory, the number of paths that are deleted with each invocation is limited.  If the number of paths to be deleted exceeds this limit, a continuation token is returned in this response header.  When a continuation token is returned in the response, it must be specified in a subsequent invocation of the delete operation to continue deleting the directory."]
            pub fn continuation(mut self, continuation: impl Into<String>) -> Self {
                self.continuation = Some(continuation.into());
                self
            }
            #[doc = "Optional. Valid for \"SetAccessControlRecursive\" operation. If set to false, the operation will terminate quickly on encountering user errors (4XX). If true, the operation will ignore user errors and proceed with the operation on other sub-entities of the directory. Continuation token will only be returned when forceFlag is true in case of user errors. If not set the default value is false for this."]
            pub fn force_flag(mut self, force_flag: bool) -> Self {
                self.force_flag = Some(force_flag);
                self
            }
            #[doc = "Optional. It specifies the maximum number of files or directories on which the acl change will be applied. If omitted or greater than 2,000, the request will process up to 2,000 items"]
            pub fn max_records(mut self, max_records: i32) -> Self {
                self.max_records = Some(max_records);
                self
            }
            #[doc = "Sets POSIX access control rights on files and directories. The value is a comma-separated list of access control entries. Each access control entry (ACE) consists of a scope, a type, a user or group identifier, and permissions in the format \"[scope:][type]:[id]:[permissions]\"."]
            pub fn x_ms_acl(mut self, x_ms_acl: impl Into<String>) -> Self {
                self.x_ms_acl = Some(x_ms_acl.into());
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
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-06-08");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(continuation) = &this.continuation {
                            req.url_mut().query_pairs_mut().append_pair("continuation", continuation);
                        }
                        let mode = &this.mode;
                        req.url_mut().query_pairs_mut().append_pair("mode", mode);
                        if let Some(force_flag) = &this.force_flag {
                            req.url_mut().query_pairs_mut().append_pair("forceFlag", &force_flag.to_string());
                        }
                        if let Some(max_records) = &this.max_records {
                            req.url_mut().query_pairs_mut().append_pair("maxRecords", &max_records.to_string());
                        }
                        if let Some(x_ms_acl) = &this.x_ms_acl {
                            req.insert_header("x-ms-acl", x_ms_acl);
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core::Url::parse(&format!(
                    "{}/{}/{}?action=setAccessControlRecursive",
                    self.client.endpoint(),
                    &self.filesystem,
                    &self.path
                ))?;
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SetAccessControlRecursiveResponse>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SetAccessControlRecursiveResponse>>;
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
    pub mod flush_data {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
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
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "An HTTP entity tag associated with the file or directory."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "The data and time the file or directory was last modified.  Write operations on the file or directory update the last modified time."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "The size of the resource in bytes."]
            pub fn content_length(&self) -> azure_core::Result<i64> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("content-length"))
            }
            #[doc = "If a client request id header is sent in the request, this header will be present in the response with the same value."]
            pub fn x_ms_client_request_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-client-request-id"))
            }
            #[doc = "A server-generated UUID recorded in the analytics logs for troubleshooting and correlation."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "The version of the REST protocol used to process the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "The value of this header is set to true if the contents of the request are successfully encrypted using the specified algorithm, and false otherwise."]
            pub fn x_ms_request_server_encrypted(&self) -> azure_core::Result<bool> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-request-server-encrypted"))
            }
            #[doc = "The SHA-256 hash of the encryption key used to encrypt the blob. This header is only returned when the blob was encrypted with a customer-provided key."]
            pub fn x_ms_encryption_key_sha256(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-encryption-key-sha256"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) filesystem: String,
            pub(crate) path: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) position: Option<i64>,
            pub(crate) retain_uncommitted_data: Option<bool>,
            pub(crate) close: Option<bool>,
            pub(crate) content_length: Option<i64>,
            pub(crate) x_ms_content_md5: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_cache_control: Option<String>,
            pub(crate) x_ms_content_type: Option<String>,
            pub(crate) x_ms_content_disposition: Option<String>,
            pub(crate) x_ms_content_encoding: Option<String>,
            pub(crate) x_ms_content_language: Option<String>,
            pub(crate) if_match: Option<String>,
            pub(crate) if_none_match: Option<String>,
            pub(crate) if_modified_since: Option<time::OffsetDateTime>,
            pub(crate) if_unmodified_since: Option<time::OffsetDateTime>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_encryption_key: Option<String>,
            pub(crate) x_ms_encryption_key_sha256: Option<String>,
            pub(crate) x_ms_encryption_algorithm: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "This parameter allows the caller to upload data in parallel and control the order in which it is appended to the file.  It is required when uploading data to be appended to the file and when flushing previously uploaded data to the file.  The value must be the position where the data is to be appended.  Uploaded data is not immediately flushed, or written, to the file.  To flush, the previously uploaded data must be contiguous, the position parameter must be specified and equal to the length of the file after all data has been written, and there must not be a request entity body included with the request."]
            pub fn position(mut self, position: i64) -> Self {
                self.position = Some(position);
                self
            }
            #[doc = "Valid only for flush operations.  If \"true\", uncommitted data is retained after the flush operation completes; otherwise, the uncommitted data is deleted after the flush operation.  The default is false.  Data at offsets less than the specified position are written to the file when flush succeeds, but this optional parameter allows data after the flush position to be retained for a future flush operation."]
            pub fn retain_uncommitted_data(mut self, retain_uncommitted_data: bool) -> Self {
                self.retain_uncommitted_data = Some(retain_uncommitted_data);
                self
            }
            #[doc = "Azure Storage Events allow applications to receive notifications when files change. When Azure Storage Events are enabled, a file changed event is raised. This event has a property indicating whether this is the final change to distinguish the difference between an intermediate flush to a file stream and the final close of a file stream. The close query parameter is valid only when the action is \"flush\" and change notifications are enabled. If the value of close is \"true\" and the flush operation completes successfully, the service raises a file change notification with a property indicating that this is the final update (the file stream has been closed). If \"false\" a change notification is raised indicating the file has changed. The default is false. This query parameter is set to true by the Hadoop ABFS driver to indicate that the file stream has been closed.\""]
            pub fn close(mut self, close: bool) -> Self {
                self.close = Some(close);
                self
            }
            #[doc = "Required for \"Append Data\" and \"Flush Data\".  Must be 0 for \"Flush Data\".  Must be the length of the request content in bytes for \"Append Data\"."]
            pub fn content_length(mut self, content_length: i64) -> Self {
                self.content_length = Some(content_length);
                self
            }
            #[doc = "Specify the transactional md5 for the body, to be validated by the service."]
            pub fn x_ms_content_md5(mut self, x_ms_content_md5: impl Into<String>) -> Self {
                self.x_ms_content_md5 = Some(x_ms_content_md5.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Optional. Sets the blob's cache control. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_cache_control(mut self, x_ms_cache_control: impl Into<String>) -> Self {
                self.x_ms_cache_control = Some(x_ms_cache_control.into());
                self
            }
            #[doc = "Optional. Sets the blob's content type. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_content_type(mut self, x_ms_content_type: impl Into<String>) -> Self {
                self.x_ms_content_type = Some(x_ms_content_type.into());
                self
            }
            #[doc = "Optional. Sets the blob's Content-Disposition header."]
            pub fn x_ms_content_disposition(mut self, x_ms_content_disposition: impl Into<String>) -> Self {
                self.x_ms_content_disposition = Some(x_ms_content_disposition.into());
                self
            }
            #[doc = "Optional. Sets the blob's content encoding. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_content_encoding(mut self, x_ms_content_encoding: impl Into<String>) -> Self {
                self.x_ms_content_encoding = Some(x_ms_content_encoding.into());
                self
            }
            #[doc = "Optional. Set the blob's content language. If specified, this property is stored with the blob and returned with a read request."]
            pub fn x_ms_content_language(mut self, x_ms_content_language: impl Into<String>) -> Self {
                self.x_ms_content_language = Some(x_ms_content_language.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs with a matching value."]
            pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
                self.if_match = Some(if_match.into());
                self
            }
            #[doc = "Specify an ETag value to operate only on blobs without a matching value."]
            pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
                self.if_none_match = Some(if_none_match.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has been modified since the specified date/time."]
            pub fn if_modified_since(mut self, if_modified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_modified_since = Some(if_modified_since.into());
                self
            }
            #[doc = "Specify this header value to operate only on a blob if it has not been modified since the specified date/time."]
            pub fn if_unmodified_since(mut self, if_unmodified_since: impl Into<time::OffsetDateTime>) -> Self {
                self.if_unmodified_since = Some(if_unmodified_since.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Optional. Specifies the encryption key to use to encrypt the data provided in the request. If not specified, encryption is performed with the root account encryption key.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_key(mut self, x_ms_encryption_key: impl Into<String>) -> Self {
                self.x_ms_encryption_key = Some(x_ms_encryption_key.into());
                self
            }
            #[doc = "The SHA-256 hash of the provided encryption key. Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_key_sha256(mut self, x_ms_encryption_key_sha256: impl Into<String>) -> Self {
                self.x_ms_encryption_key_sha256 = Some(x_ms_encryption_key_sha256.into());
                self
            }
            #[doc = "The algorithm used to produce the encryption key hash. Currently, the only accepted value is \"AES256\". Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_algorithm(mut self, x_ms_encryption_algorithm: impl Into<String>) -> Self {
                self.x_ms_encryption_algorithm = Some(x_ms_encryption_algorithm.into());
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
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-06-08");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(position) = &this.position {
                            req.url_mut().query_pairs_mut().append_pair("position", &position.to_string());
                        }
                        if let Some(retain_uncommitted_data) = &this.retain_uncommitted_data {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("retainUncommittedData", &retain_uncommitted_data.to_string());
                        }
                        if let Some(close) = &this.close {
                            req.url_mut().query_pairs_mut().append_pair("close", &close.to_string());
                        }
                        if let Some(content_length) = &this.content_length {
                            req.insert_header("content-length", &content_length.to_string());
                        }
                        if let Some(x_ms_content_md5) = &this.x_ms_content_md5 {
                            req.insert_header("x-ms-content-md5", x_ms_content_md5);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_cache_control) = &this.x_ms_cache_control {
                            req.insert_header("x-ms-cache-control", x_ms_cache_control);
                        }
                        if let Some(x_ms_content_type) = &this.x_ms_content_type {
                            req.insert_header("x-ms-content-type", x_ms_content_type);
                        }
                        if let Some(x_ms_content_disposition) = &this.x_ms_content_disposition {
                            req.insert_header("x-ms-content-disposition", x_ms_content_disposition);
                        }
                        if let Some(x_ms_content_encoding) = &this.x_ms_content_encoding {
                            req.insert_header("x-ms-content-encoding", x_ms_content_encoding);
                        }
                        if let Some(x_ms_content_language) = &this.x_ms_content_language {
                            req.insert_header("x-ms-content-language", x_ms_content_language);
                        }
                        if let Some(if_match) = &this.if_match {
                            req.insert_header("if-match", if_match);
                        }
                        if let Some(if_none_match) = &this.if_none_match {
                            req.insert_header("if-none-match", if_none_match);
                        }
                        if let Some(if_modified_since) = &this.if_modified_since {
                            req.insert_header("if-modified-since", &if_modified_since.to_string());
                        }
                        if let Some(if_unmodified_since) = &this.if_unmodified_since {
                            req.insert_header("if-unmodified-since", &if_unmodified_since.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_encryption_key) = &this.x_ms_encryption_key {
                            req.insert_header("x-ms-encryption-key", x_ms_encryption_key);
                        }
                        if let Some(x_ms_encryption_key_sha256) = &this.x_ms_encryption_key_sha256 {
                            req.insert_header("x-ms-encryption-key-sha256", x_ms_encryption_key_sha256);
                        }
                        if let Some(x_ms_encryption_algorithm) = &this.x_ms_encryption_algorithm {
                            req.insert_header("x-ms-encryption-algorithm", x_ms_encryption_algorithm);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core::Url::parse(&format!(
                    "{}/{}/{}?action=flush",
                    self.client.endpoint(),
                    &self.filesystem,
                    &self.path
                ))?;
                Ok(url)
            }
        }
    }
    pub mod append_data {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
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
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "A server-generated UUID recorded in the analytics logs for troubleshooting and correlation."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "If a client request id header is sent in the request, this header will be present in the response with the same value."]
            pub fn x_ms_client_request_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-client-request-id"))
            }
            #[doc = "The version of the REST protocol used to process the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "An HTTP entity tag associated with the file or directory."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "If the blob has an MD5 hash and this operation is to read the full blob, this response header is returned so that the client can check for message content integrity."]
            pub fn content_md5(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-md5"))
            }
            #[doc = "This header is returned so that the client can check for message content integrity. The value of this header is computed by the Blob service; it is not necessarily the same value specified in the request headers."]
            pub fn x_ms_content_crc64(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-content-crc64"))
            }
            #[doc = "The value of this header is set to true if the contents of the request are successfully encrypted using the specified algorithm, and false otherwise."]
            pub fn x_ms_request_server_encrypted(&self) -> azure_core::Result<bool> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-request-server-encrypted"))
            }
            #[doc = "The SHA-256 hash of the encryption key used to encrypt the blob. This header is only returned when the blob was encrypted with a customer-provided key."]
            pub fn x_ms_encryption_key_sha256(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-encryption-key-sha256"))
            }
        }
        #[derive(Clone)]
        #[doc = r" `RequestBuilder` provides a mechanism for setting optional parameters on a request."]
        #[doc = r""]
        #[doc = r" Each `RequestBuilder` parameter method call returns `Self`, so setting of multiple"]
        #[doc = r" parameters can be chained."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) filesystem: String,
            pub(crate) path: String,
            pub(crate) body: serde_json::Value,
            pub(crate) position: Option<i64>,
            pub(crate) timeout: Option<i64>,
            pub(crate) content_length: Option<i64>,
            pub(crate) content_md5: Option<String>,
            pub(crate) x_ms_content_crc64: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_encryption_key: Option<String>,
            pub(crate) x_ms_encryption_key_sha256: Option<String>,
            pub(crate) x_ms_encryption_algorithm: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "This parameter allows the caller to upload data in parallel and control the order in which it is appended to the file.  It is required when uploading data to be appended to the file and when flushing previously uploaded data to the file.  The value must be the position where the data is to be appended.  Uploaded data is not immediately flushed, or written, to the file.  To flush, the previously uploaded data must be contiguous, the position parameter must be specified and equal to the length of the file after all data has been written, and there must not be a request entity body included with the request."]
            pub fn position(mut self, position: i64) -> Self {
                self.position = Some(position);
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Required for \"Append Data\" and \"Flush Data\".  Must be 0 for \"Flush Data\".  Must be the length of the request content in bytes for \"Append Data\"."]
            pub fn content_length(mut self, content_length: i64) -> Self {
                self.content_length = Some(content_length);
                self
            }
            #[doc = "Specify the transactional md5 for the body, to be validated by the service."]
            pub fn content_md5(mut self, content_md5: impl Into<String>) -> Self {
                self.content_md5 = Some(content_md5.into());
                self
            }
            #[doc = "Specify the transactional crc64 for the body, to be validated by the service."]
            pub fn x_ms_content_crc64(mut self, x_ms_content_crc64: impl Into<String>) -> Self {
                self.x_ms_content_crc64 = Some(x_ms_content_crc64.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Optional. Specifies the encryption key to use to encrypt the data provided in the request. If not specified, encryption is performed with the root account encryption key.  For more information, see Encryption at Rest for Azure Storage Services."]
            pub fn x_ms_encryption_key(mut self, x_ms_encryption_key: impl Into<String>) -> Self {
                self.x_ms_encryption_key = Some(x_ms_encryption_key.into());
                self
            }
            #[doc = "The SHA-256 hash of the provided encryption key. Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_key_sha256(mut self, x_ms_encryption_key_sha256: impl Into<String>) -> Self {
                self.x_ms_encryption_key_sha256 = Some(x_ms_encryption_key_sha256.into());
                self
            }
            #[doc = "The algorithm used to produce the encryption key hash. Currently, the only accepted value is \"AES256\". Must be provided if the x-ms-encryption-key header is provided."]
            pub fn x_ms_encryption_algorithm(mut self, x_ms_encryption_algorithm: impl Into<String>) -> Self {
                self.x_ms_encryption_algorithm = Some(x_ms_encryption_algorithm.into());
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
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-06-08");
                        if let Some(position) = &this.position {
                            req.url_mut().query_pairs_mut().append_pair("position", &position.to_string());
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(content_length) = &this.content_length {
                            req.insert_header("content-length", &content_length.to_string());
                        }
                        if let Some(content_md5) = &this.content_md5 {
                            req.insert_header("content-md5", content_md5);
                        }
                        if let Some(x_ms_content_crc64) = &this.x_ms_content_crc64 {
                            req.insert_header("x-ms-content-crc64", x_ms_content_crc64);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_encryption_key) = &this.x_ms_encryption_key {
                            req.insert_header("x-ms-encryption-key", x_ms_encryption_key);
                        }
                        if let Some(x_ms_encryption_key_sha256) = &this.x_ms_encryption_key_sha256 {
                            req.insert_header("x-ms-encryption-key-sha256", x_ms_encryption_key_sha256);
                        }
                        if let Some(x_ms_encryption_algorithm) = &this.x_ms_encryption_algorithm {
                            req.insert_header("x-ms-encryption-algorithm", x_ms_encryption_algorithm);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core::Url::parse(&format!(
                    "{}/{}/{}?action=append",
                    self.client.endpoint(),
                    &self.filesystem,
                    &self.path
                ))?;
                Ok(url)
            }
        }
    }
    pub mod set_expiry {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
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
            #[doc = "The ETag contains a value that you can use to perform operations conditionally. If the request version is 2011-08-18 or newer, the ETag value will be in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the container was last modified. Any operation that modifies the blob, including an update of the blob's metadata or properties, changes the last-modified time of the blob."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "If a client request id header is sent in the request, this header will be present in the response with the same value."]
            pub fn x_ms_client_request_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-client-request-id"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the Blob service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated."]
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
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) filesystem: String,
            pub(crate) path: String,
            pub(crate) x_ms_expiry_option: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_expiry_time: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "The time to set the blob to expiry"]
            pub fn x_ms_expiry_time(mut self, x_ms_expiry_time: impl Into<String>) -> Self {
                self.x_ms_expiry_time = Some(x_ms_expiry_time.into());
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
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-06-08");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        req.insert_header("x-ms-expiry-option", &this.x_ms_expiry_option);
                        if let Some(x_ms_expiry_time) = &this.x_ms_expiry_time {
                            req.insert_header("x-ms-expiry-time", x_ms_expiry_time);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core::Url::parse(&format!(
                    "{}/{}/{}?comp=expiry",
                    self.client.endpoint(),
                    &self.filesystem,
                    &self.path
                ))?;
                Ok(url)
            }
        }
    }
    pub mod undelete {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
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
            #[doc = "If a client request id header is sent in the request, this header will be present in the response with the same value."]
            pub fn x_ms_client_request_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-client-request-id"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "The type of the resource.  The value may be \"file\" or \"directory\".  If not set, the value is \"file\"."]
            pub fn x_ms_resource_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-resource-type"))
            }
            #[doc = "Indicates the version of the Blob service used to execute the request. This header is returned for requests made against version 2009-09-19 and above."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "UTC date/time value generated by the service that indicates the time at which the response was initiated."]
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
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the [`RequestBuilder`] into a future"]
        #[doc = r" executes the request and returns a `Result` with the parsed"]
        #[doc = r" response."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use `.send().await` instead."]
        #[doc = r""]
        #[doc = r" If you need lower-level access to the raw response details"]
        #[doc = r" (e.g. to inspect response headers or raw body data) then you"]
        #[doc = r" can finalize the request using the"]
        #[doc = r" [`RequestBuilder::send()`] method which returns a future"]
        #[doc = r" that resolves to a lower-level [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) filesystem: String,
            pub(crate) path: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_undelete_source: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations\">Setting Timeouts for Blob Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Only for hierarchical namespace enabled accounts. Optional. The path of the soft deleted blob to undelete."]
            pub fn x_ms_undelete_source(mut self, x_ms_undelete_source: impl Into<String>) -> Self {
                self.x_ms_undelete_source = Some(x_ms_undelete_source.into());
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
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2021-06-08");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_undelete_source) = &this.x_ms_undelete_source {
                            req.insert_header("x-ms-undelete-source", x_ms_undelete_source);
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
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let url = azure_core::Url::parse(&format!(
                    "{}/{}/{}?comp=undelete",
                    self.client.endpoint(),
                    &self.filesystem,
                    &self.path
                ))?;
                Ok(url)
            }
        }
    }
}
