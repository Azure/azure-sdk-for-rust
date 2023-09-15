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
    pub fn directory_client(&self) -> directory::Client {
        directory::Client(self.clone())
    }
    pub fn file_client(&self) -> file::Client {
        file::Client(self.clone())
    }
    pub fn service_client(&self) -> service::Client {
        service::Client(self.clone())
    }
    pub fn share_client(&self) -> share::Client {
        share::Client(self.clone())
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
        #[doc = "Gets the properties of a storage account's File service, including properties for Storage Analytics metrics and CORS (Cross-Origin Resource Sharing) rules."]
        pub fn get_properties(&self) -> get_properties::RequestBuilder {
            get_properties::RequestBuilder {
                client: self.0.clone(),
                timeout: None,
            }
        }
        #[doc = "Sets properties for a storage account's File service endpoint, including properties for Storage Analytics metrics and CORS (Cross-Origin Resource Sharing) rules."]
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
            }
        }
        #[doc = "The List Shares Segment operation returns a list of the shares and share snapshots under the specified account."]
        pub fn list_shares_segment(&self) -> list_shares_segment::RequestBuilder {
            list_shares_segment::RequestBuilder {
                client: self.0.clone(),
                prefix: None,
                marker: None,
                maxresults: None,
                include: Vec::new(),
                timeout: None,
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
            #[doc = "Indicates the version of the File service used to execute the request."]
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
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
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
                        let url = azure_core::Url::parse(&format!("{}/?restype=service&comp=properties", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
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
            #[doc = "Indicates the version of the File service used to execute the request."]
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
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
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
                        let url = azure_core::Url::parse(&format!("{}/?restype=service&comp=properties", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        req.insert_header("content-type", "application/xml");
                        let req_body = azure_core::to_json(&this.storage_service_properties)?;
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_shares_segment {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ListSharesResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ListSharesResponse = azure_core::xml::read_xml(&bytes)?;
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
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) prefix: Option<String>,
            pub(crate) marker: Option<String>,
            pub(crate) maxresults: Option<i64>,
            pub(crate) include: Vec<String>,
            pub(crate) timeout: Option<i64>,
        }
        impl RequestBuilder {
            #[doc = "Filters the results to return only entries whose name begins with the specified prefix."]
            pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
                self.prefix = Some(prefix.into());
                self
            }
            #[doc = "A string value that identifies the portion of the list to be returned with the next list operation. The operation returns a marker value within the response body if the list returned was not complete. The marker value may then be used in a subsequent call to request the next set of list items. The marker value is opaque to the client."]
            pub fn marker(mut self, marker: impl Into<String>) -> Self {
                self.marker = Some(marker.into());
                self
            }
            #[doc = "Specifies the maximum number of entries to return. If the request does not specify maxresults, or specifies a value greater than 5,000, the server will return up to 5,000 items."]
            pub fn maxresults(mut self, maxresults: i64) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "Include this parameter to specify one or more datasets to include in the response."]
            pub fn include(mut self, include: Vec<String>) -> Self {
                self.include = include;
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::ListSharesResponse, azure_core::error::Error> {
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
                                req.insert_header(azure_core::headers::VERSION, "2023-08-03");
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
pub mod share {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns all user-defined metadata and system properties for the specified share or share snapshot. The data returned does not include the share's list of files."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        pub fn get_properties(&self, share_name: impl Into<String>) -> get_properties::RequestBuilder {
            get_properties::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                sharesnapshot: None,
                timeout: None,
                x_ms_lease_id: None,
            }
        }
        #[doc = "Creates a new share under the specified account. If the share with the same name already exists, the operation fails."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        pub fn create(&self, share_name: impl Into<String>) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                timeout: None,
                x_ms_meta: None,
                x_ms_share_quota: None,
                x_ms_access_tier: None,
                x_ms_enabled_protocols: None,
                x_ms_root_squash: None,
            }
        }
        #[doc = "Operation marks the specified share or share snapshot for deletion. The share or share snapshot and any files contained within it are later deleted during garbage collection."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        pub fn delete(&self, share_name: impl Into<String>) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                sharesnapshot: None,
                timeout: None,
                x_ms_delete_snapshots: None,
                x_ms_lease_id: None,
            }
        }
        #[doc = "The Lease Share operation establishes and manages a lock on a share, or the specified snapshot for set and delete share operations."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        pub fn acquire_lease(&self, share_name: impl Into<String>, x_ms_lease_action: impl Into<String>) -> acquire_lease::RequestBuilder {
            acquire_lease::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                timeout: None,
                x_ms_lease_duration: None,
                x_ms_proposed_lease_id: None,
                sharesnapshot: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Lease Share operation establishes and manages a lock on a share, or the specified snapshot for set and delete share operations."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        #[doc = "* `x_ms_lease_id`: Specifies the current lease ID on the resource."]
        pub fn release_lease(
            &self,
            share_name: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
            x_ms_lease_id: impl Into<String>,
        ) -> release_lease::RequestBuilder {
            release_lease::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                x_ms_lease_id: x_ms_lease_id.into(),
                timeout: None,
                sharesnapshot: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Lease Share operation establishes and manages a lock on a share, or the specified snapshot for set and delete share operations."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        #[doc = "* `x_ms_lease_id`: Specifies the current lease ID on the resource."]
        pub fn change_lease(
            &self,
            share_name: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
            x_ms_lease_id: impl Into<String>,
        ) -> change_lease::RequestBuilder {
            change_lease::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                x_ms_lease_id: x_ms_lease_id.into(),
                timeout: None,
                x_ms_proposed_lease_id: None,
                sharesnapshot: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Lease Share operation establishes and manages a lock on a share, or the specified snapshot for set and delete share operations."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        #[doc = "* `x_ms_lease_id`: Specifies the current lease ID on the resource."]
        pub fn renew_lease(
            &self,
            share_name: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
            x_ms_lease_id: impl Into<String>,
        ) -> renew_lease::RequestBuilder {
            renew_lease::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                x_ms_lease_id: x_ms_lease_id.into(),
                timeout: None,
                sharesnapshot: None,
                x_ms_client_request_id: None,
            }
        }
        #[doc = "The Lease Share operation establishes and manages a lock on a share, or the specified snapshot for set and delete share operations."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        pub fn break_lease(&self, share_name: impl Into<String>, x_ms_lease_action: impl Into<String>) -> break_lease::RequestBuilder {
            break_lease::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                timeout: None,
                x_ms_lease_break_period: None,
                x_ms_lease_id: None,
                x_ms_client_request_id: None,
                sharesnapshot: None,
            }
        }
        #[doc = "Creates a read-only snapshot of a share."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        pub fn create_snapshot(&self, share_name: impl Into<String>) -> create_snapshot::RequestBuilder {
            create_snapshot::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                timeout: None,
                x_ms_meta: None,
            }
        }
        #[doc = "Returns the permission (security descriptor) for a given key"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `x_ms_file_permission_key`: Key of the permission to be set for the directory/file."]
        pub fn get_permission(
            &self,
            share_name: impl Into<String>,
            x_ms_file_permission_key: impl Into<String>,
        ) -> get_permission::RequestBuilder {
            get_permission::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                x_ms_file_permission_key: x_ms_file_permission_key.into(),
                timeout: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "Create a permission (a security descriptor)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `share_permission`: A permission (a security descriptor) at the share level."]
        pub fn create_permission(
            &self,
            share_name: impl Into<String>,
            share_permission: impl Into<models::SharePermission>,
        ) -> create_permission::RequestBuilder {
            create_permission::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                share_permission: share_permission.into(),
                timeout: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "Sets properties for the specified share."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        pub fn set_properties(&self, share_name: impl Into<String>) -> set_properties::RequestBuilder {
            set_properties::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                timeout: None,
                x_ms_share_quota: None,
                x_ms_access_tier: None,
                x_ms_lease_id: None,
                x_ms_root_squash: None,
            }
        }
        #[doc = "Sets one or more user-defined name-value pairs for the specified share."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        pub fn set_metadata(&self, share_name: impl Into<String>) -> set_metadata::RequestBuilder {
            set_metadata::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                timeout: None,
                x_ms_meta: None,
                x_ms_lease_id: None,
            }
        }
        #[doc = "Returns information about stored access policies specified on the share."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        pub fn get_access_policy(&self, share_name: impl Into<String>) -> get_access_policy::RequestBuilder {
            get_access_policy::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                timeout: None,
                x_ms_lease_id: None,
            }
        }
        #[doc = "Sets a stored access policy for use with shared access signatures."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        pub fn set_access_policy(&self, share_name: impl Into<String>) -> set_access_policy::RequestBuilder {
            set_access_policy::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                share_acl: None,
                timeout: None,
                x_ms_lease_id: None,
            }
        }
        #[doc = "Retrieves statistics related to the share."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        pub fn get_statistics(&self, share_name: impl Into<String>) -> get_statistics::RequestBuilder {
            get_statistics::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                timeout: None,
                x_ms_lease_id: None,
            }
        }
        #[doc = "Restores a previously deleted Share."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        pub fn restore(&self, share_name: impl Into<String>) -> restore::RequestBuilder {
            restore::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                timeout: None,
                x_ms_client_request_id: None,
                x_ms_deleted_share_name: None,
                x_ms_deleted_share_version: None,
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
            #[doc = "A set of name-value pairs that contain the user-defined metadata of the share."]
            pub fn x_ms_meta(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-meta"))
            }
            #[doc = "The ETag contains a value that you can use to perform operations conditionally, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the share was last modified. Any operation that modifies the share or its properties updates the last modified time. Operations on files do not affect the last modified time of the share."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "Returns the current share quota in GB."]
            pub fn x_ms_share_quota(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("x-ms-share-quota"))
            }
            #[doc = "Returns the current share provisioned ipos."]
            pub fn x_ms_share_provisioned_iops(&self) -> azure_core::Result<i32> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-share-provisioned-iops"))
            }
            #[doc = "Returns the current share provisioned ingress in megabytes per second."]
            pub fn x_ms_share_provisioned_ingress_mbps(&self) -> azure_core::Result<i32> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-share-provisioned-ingress-mbps"))
            }
            #[doc = "Returns the current share provisioned egress in megabytes per second."]
            pub fn x_ms_share_provisioned_egress_mbps(&self) -> azure_core::Result<i32> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-share-provisioned-egress-mbps"))
            }
            #[doc = "Returns the current share next allowed quota downgrade time."]
            pub fn x_ms_share_next_allowed_quota_downgrade_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static(
                    "x-ms-share-next-allowed-quota-downgrade-time",
                ))?)
            }
            #[doc = "Returns the current share provisioned bandwidth in megabits per second."]
            pub fn x_ms_share_provisioned_bandwidth_mibps(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static(
                    "x-ms-share-provisioned-bandwidth-mibps",
                ))
            }
            #[doc = "When a share is leased, specifies whether the lease is of infinite or fixed duration."]
            pub fn x_ms_lease_duration(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-duration"))
            }
            #[doc = "Lease state of the share."]
            pub fn x_ms_lease_state(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-state"))
            }
            #[doc = "The current lease status of the share."]
            pub fn x_ms_lease_status(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-status"))
            }
            #[doc = "Returns the access tier set on the share."]
            pub fn x_ms_access_tier(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-access-tier"))
            }
            #[doc = "Returns the last modified time (in UTC) of the access tier of the share."]
            pub fn x_ms_access_tier_change_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-access-tier-change-time"))?,
                )
            }
            #[doc = "Returns the transition state between access tiers, when present."]
            pub fn x_ms_access_tier_transition_state(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-access-tier-transition-state"))
            }
            #[doc = "The protocols that have been enabled on the share."]
            pub fn x_ms_enabled_protocols(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-enabled-protocols"))
            }
            #[doc = "Valid for NFS shares only."]
            pub fn x_ms_root_squash(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-root-squash"))
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
            pub(crate) share_name: String,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
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
                        let url = azure_core::Url::parse(&format!("{}/{}?restype=share", this.client.endpoint(), &this.share_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
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
            #[doc = "The ETag contains a value which represents the version of the share, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the share was last modified. Any operation that modifies the share or its properties or metadata updates the last modified time. Operations on files do not affect the last modified time of the share."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_share_quota: Option<i64>,
            pub(crate) x_ms_access_tier: Option<String>,
            pub(crate) x_ms_enabled_protocols: Option<String>,
            pub(crate) x_ms_root_squash: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "A name-value pair to associate with a file storage object."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "Specifies the maximum size of the share, in gigabytes."]
            pub fn x_ms_share_quota(mut self, x_ms_share_quota: i64) -> Self {
                self.x_ms_share_quota = Some(x_ms_share_quota);
                self
            }
            #[doc = "Specifies the access tier of the share."]
            pub fn x_ms_access_tier(mut self, x_ms_access_tier: impl Into<String>) -> Self {
                self.x_ms_access_tier = Some(x_ms_access_tier.into());
                self
            }
            #[doc = "Protocols to enable on the share."]
            pub fn x_ms_enabled_protocols(mut self, x_ms_enabled_protocols: impl Into<String>) -> Self {
                self.x_ms_enabled_protocols = Some(x_ms_enabled_protocols.into());
                self
            }
            #[doc = "Root squash to set on the share.  Only valid for NFS shares."]
            pub fn x_ms_root_squash(mut self, x_ms_root_squash: impl Into<String>) -> Self {
                self.x_ms_root_squash = Some(x_ms_root_squash.into());
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
                        let url = azure_core::Url::parse(&format!("{}/{}?restype=share", this.client.endpoint(), &this.share_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(x_ms_share_quota) = &this.x_ms_share_quota {
                            req.insert_header("x-ms-share-quota", &x_ms_share_quota.to_string());
                        }
                        if let Some(x_ms_access_tier) = &this.x_ms_access_tier {
                            req.insert_header("x-ms-access-tier", x_ms_access_tier);
                        }
                        if let Some(x_ms_enabled_protocols) = &this.x_ms_enabled_protocols {
                            req.insert_header("x-ms-enabled-protocols", x_ms_enabled_protocols);
                        }
                        if let Some(x_ms_root_squash) = &this.x_ms_root_squash {
                            req.insert_header("x-ms-root-squash", x_ms_root_squash);
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
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_delete_snapshots: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specifies the option include to delete the base share and all of its snapshots."]
            pub fn x_ms_delete_snapshots(mut self, x_ms_delete_snapshots: impl Into<String>) -> Self {
                self.x_ms_delete_snapshots = Some(x_ms_delete_snapshots.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
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
                        let url = azure_core::Url::parse(&format!("{}/{}?restype=share", this.client.endpoint(), &this.share_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_delete_snapshots) = &this.x_ms_delete_snapshots {
                            req.insert_header("x-ms-delete-snapshots", x_ms_delete_snapshots);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod acquire_lease {
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
            #[doc = "The ETag contains a value that you can use to perform operations conditionally, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the share was last modified. Any operation that modifies the share or its properties updates the last modified time. Operations on files do not affect the last modified time of the share."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "Uniquely identifies a share's lease"]
            pub fn x_ms_lease_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-id"))
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
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_duration: Option<i64>,
            pub(crate) x_ms_proposed_lease_id: Option<String>,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specifies the duration of the lease, in seconds, or negative one (-1) for a lease that never expires. A non-infinite lease can be between 15 and 60 seconds. A lease duration cannot be changed using renew or change."]
            pub fn x_ms_lease_duration(mut self, x_ms_lease_duration: i64) -> Self {
                self.x_ms_lease_duration = Some(x_ms_lease_duration);
                self
            }
            #[doc = "Proposed lease ID, in a GUID string format. The File service returns 400 (Invalid request) if the proposed lease ID is not in the correct format. See Guid Constructor (String) for a list of valid GUID string formats."]
            pub fn x_ms_proposed_lease_id(mut self, x_ms_proposed_lease_id: impl Into<String>) -> Self {
                self.x_ms_proposed_lease_id = Some(x_ms_proposed_lease_id.into());
                self
            }
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
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
                            "{}/{}?restype=share&comp=lease&acquire",
                            this.client.endpoint(),
                            &this.share_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_duration) = &this.x_ms_lease_duration {
                            req.insert_header("x-ms-lease-duration", &x_ms_lease_duration.to_string());
                        }
                        if let Some(x_ms_proposed_lease_id) = &this.x_ms_proposed_lease_id {
                            req.insert_header("x-ms-proposed-lease-id", x_ms_proposed_lease_id);
                        }
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
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
    pub mod release_lease {
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
            #[doc = "The ETag contains a value that you can use to perform operations conditionally, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the share was last modified. Any operation that modifies the share or its properties updates the last modified time. Operations on files do not affect the last modified time of the share."]
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
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) x_ms_lease_id: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
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
                            "{}/{}?restype=share&comp=lease&release",
                            this.client.endpoint(),
                            &this.share_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-lease-id", &this.x_ms_lease_id);
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
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
    pub mod change_lease {
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
            #[doc = "The ETag contains a value that you can use to perform operations conditionally, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the share was last modified. Any operation that modifies the share or its properties updates the last modified time. Operations on files do not affect the last modified time of the share."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "Uniquely identifies a share's lease"]
            pub fn x_ms_lease_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-id"))
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
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) x_ms_lease_id: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_proposed_lease_id: Option<String>,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Proposed lease ID, in a GUID string format. The File service returns 400 (Invalid request) if the proposed lease ID is not in the correct format. See Guid Constructor (String) for a list of valid GUID string formats."]
            pub fn x_ms_proposed_lease_id(mut self, x_ms_proposed_lease_id: impl Into<String>) -> Self {
                self.x_ms_proposed_lease_id = Some(x_ms_proposed_lease_id.into());
                self
            }
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
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
                            "{}/{}?restype=share&comp=lease&change",
                            this.client.endpoint(),
                            &this.share_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-lease-id", &this.x_ms_lease_id);
                        if let Some(x_ms_proposed_lease_id) = &this.x_ms_proposed_lease_id {
                            req.insert_header("x-ms-proposed-lease-id", x_ms_proposed_lease_id);
                        }
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
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
    pub mod renew_lease {
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
            #[doc = "The ETag contains a value that you can use to perform operations conditionally, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the share was last modified. Any operation that modifies the share or its properties updates the last modified time. Operations on files do not affect the last modified time of the share."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "Uniquely identifies a share's lease"]
            pub fn x_ms_lease_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-id"))
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
            #[doc = "Indicates the version of the File service used to execute the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "Returns the current share next allowed quota downgrade time."]
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
            pub(crate) share_name: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) x_ms_lease_id: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
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
                            "{}/{}?restype=share&comp=lease&renew",
                            this.client.endpoint(),
                            &this.share_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-lease-id", &this.x_ms_lease_id);
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
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
    pub mod break_lease {
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
            #[doc = "The ETag contains a value that you can use to perform operations conditionally, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the share was last modified. Any operation that modifies the share or its properties updates the last modified time. Operations on files do not affect the last modified time of the share."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "Approximate time remaining in the lease period, in seconds."]
            pub fn x_ms_lease_time(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("x-ms-lease-time"))
            }
            #[doc = "Uniquely identifies a share's lease"]
            pub fn x_ms_lease_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-id"))
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
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_break_period: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) sharesnapshot: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "For a break operation, proposed duration the lease should continue before it is broken, in seconds, between 0 and 60. This break period is only used if it is shorter than the time remaining on the lease. If longer, the time remaining on the lease is used. A new lease will not be available before the break period has expired, but the lease may be held for longer than the break period. If this header does not appear with a break operation, a fixed-duration lease breaks after the remaining lease period elapses, and an infinite lease breaks immediately."]
            pub fn x_ms_lease_break_period(mut self, x_ms_lease_break_period: i64) -> Self {
                self.x_ms_lease_break_period = Some(x_ms_lease_break_period);
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
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
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
                            "{}/{}?restype=share&comp=lease&break",
                            this.client.endpoint(),
                            &this.share_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_break_period) = &this.x_ms_lease_break_period {
                            req.insert_header("x-ms-lease-break-period", &x_ms_lease_break_period.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod create_snapshot {
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
            #[doc = "This header is a DateTime value that uniquely identifies the share snapshot. The value of this header may be used in subsequent requests to access the share snapshot. This value is opaque."]
            pub fn x_ms_snapshot(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-snapshot"))
            }
            #[doc = "The ETag contains a value which represents the version of the share snapshot, in quotes. A share snapshot cannot be modified, so the ETag of a given share snapshot never changes. However, if new metadata was supplied with the Snapshot Share request then the ETag of the share snapshot differs from that of the base share. If no metadata was specified with the request, the ETag of the share snapshot is identical to that of the base share at the time the share snapshot was taken."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the share was last modified. A share snapshot cannot be modified, so the last modified time of a given share snapshot never changes. However, if new metadata was supplied with the Snapshot Share request then the last modified time of the share snapshot differs from that of the base share. If no metadata was specified with the request, the last modified time of the share snapshot is identical to that of the base share at the time the share snapshot was taken."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_meta: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "A name-value pair to associate with a file storage object."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
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
                            "{}/{}?restype=share&comp=snapshot",
                            this.client.endpoint(),
                            &this.share_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_permission {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SharePermission> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SharePermission = serde_json::from_slice(&bytes)?;
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
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) x_ms_file_permission_key: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}?restype=share&comp=filepermission",
                            this.client.endpoint(),
                            &this.share_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        req.insert_header("x-ms-file-permission-key", &this.x_ms_file_permission_key);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SharePermission>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SharePermission>>;
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
    pub mod create_permission {
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
            #[doc = "Indicates the version of the File service used to execute the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "Key of the permission set for the directory/file."]
            pub fn x_ms_file_permission_key(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-permission-key"))
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
            pub(crate) share_name: String,
            pub(crate) share_permission: models::SharePermission,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}?restype=share&comp=filepermission",
                            this.client.endpoint(),
                            &this.share_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.share_permission)?;
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
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
            #[doc = "The ETag contains a value that you can use to perform operations conditionally, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the share was last modified. Any operation that modifies the share or its properties updates the last modified time. Operations on files do not affect the last modified time of the share."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_share_quota: Option<i64>,
            pub(crate) x_ms_access_tier: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_root_squash: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specifies the maximum size of the share, in gigabytes."]
            pub fn x_ms_share_quota(mut self, x_ms_share_quota: i64) -> Self {
                self.x_ms_share_quota = Some(x_ms_share_quota);
                self
            }
            #[doc = "Specifies the access tier of the share."]
            pub fn x_ms_access_tier(mut self, x_ms_access_tier: impl Into<String>) -> Self {
                self.x_ms_access_tier = Some(x_ms_access_tier.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Root squash to set on the share.  Only valid for NFS shares."]
            pub fn x_ms_root_squash(mut self, x_ms_root_squash: impl Into<String>) -> Self {
                self.x_ms_root_squash = Some(x_ms_root_squash.into());
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
                            "{}/{}?restype=share&comp=properties",
                            this.client.endpoint(),
                            &this.share_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_share_quota) = &this.x_ms_share_quota {
                            req.insert_header("x-ms-share-quota", &x_ms_share_quota.to_string());
                        }
                        if let Some(x_ms_access_tier) = &this.x_ms_access_tier {
                            req.insert_header("x-ms-access-tier", x_ms_access_tier);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_root_squash) = &this.x_ms_root_squash {
                            req.insert_header("x-ms-root-squash", x_ms_root_squash);
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
            #[doc = "The ETag contains a value that you can use to perform operations conditionally, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the share was last modified. Any operation that modifies the share or its properties updates the last modified time. Operations on files do not affect the last modified time of the share."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "A name-value pair to associate with a file storage object."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
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
                            "{}/{}?restype=share&comp=metadata",
                            this.client.endpoint(),
                            &this.share_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
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
            #[doc = "The ETag contains a value that you can use to perform operations conditionally, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the share was last modified. Any operation that modifies the share or its properties updates the last modified time. Operations on files do not affect the last modified time of the share."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
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
                            azure_core::Url::parse(&format!("{}/{}?restype=share&comp=acl", this.client.endpoint(), &this.share_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
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
            #[doc = "The ETag contains a value that you can use to perform operations conditionally, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the share was last modified. Any operation that modifies the share or its properties updates the last modified time. Operations on files do not affect the last modified time of the share."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) share_acl: Option<models::SignedIdentifiers>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The ACL for the share."]
            pub fn share_acl(mut self, share_acl: impl Into<models::SignedIdentifiers>) -> Self {
                self.share_acl = Some(share_acl.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
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
                            azure_core::Url::parse(&format!("{}/{}?restype=share&comp=acl", this.client.endpoint(), &this.share_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        let req_body = if let Some(share_acl) = &this.share_acl {
                            req.insert_header("content-type", "application/xml");
                            azure_core::to_json(share_acl)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
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
            pub async fn into_body(self) -> azure_core::Result<models::ShareStats> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ShareStats = azure_core::xml::read_xml(&bytes)?;
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
            #[doc = "The ETag contains a value that you can use to perform operations conditionally, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the share was last modified. Any operation that modifies the share or its properties updates the last modified time. Operations on files do not affect the last modified time of the share."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
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
                            azure_core::Url::parse(&format!("{}/{}?restype=share&comp=stats", this.client.endpoint(), &this.share_name))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ShareStats>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ShareStats>>;
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
    pub mod restore {
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
            #[doc = "The ETag contains a value that you can use to perform operations conditionally, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the share was last modified. Any operation that modifies the share or its properties updates the last modified time. Operations on files do not affect the last modified time of the share."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "If a client request id header is sent in the request, this header will be present in the response with the same value."]
            pub fn x_ms_client_request_id(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-client-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_deleted_share_name: Option<String>,
            pub(crate) x_ms_deleted_share_version: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "Specifies the name of the previously-deleted share."]
            pub fn x_ms_deleted_share_name(mut self, x_ms_deleted_share_name: impl Into<String>) -> Self {
                self.x_ms_deleted_share_name = Some(x_ms_deleted_share_name.into());
                self
            }
            #[doc = "Specifies the version of the previously-deleted share."]
            pub fn x_ms_deleted_share_version(mut self, x_ms_deleted_share_version: impl Into<String>) -> Self {
                self.x_ms_deleted_share_version = Some(x_ms_deleted_share_version.into());
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
                            "{}/{}?restype=share&comp=undelete",
                            this.client.endpoint(),
                            &this.share_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_deleted_share_name) = &this.x_ms_deleted_share_name {
                            req.insert_header("x-ms-deleted-share-name", x_ms_deleted_share_name);
                        }
                        if let Some(x_ms_deleted_share_version) = &this.x_ms_deleted_share_version {
                            req.insert_header("x-ms-deleted-share-version", x_ms_deleted_share_version);
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
pub mod directory {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Returns all system properties for the specified directory, and can also be used to check the existence of a directory. The data returned does not include the files in the directory or any subdirectories."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        pub fn get_properties(&self, share_name: impl Into<String>, directory: impl Into<String>) -> get_properties::RequestBuilder {
            get_properties::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                x_ms_allow_trailing_dot: None,
                sharesnapshot: None,
                timeout: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "Creates a new directory under the specified share or parent directory."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `x_ms_file_attributes`: If specified, the provided file attributes shall be set. Default value: ‘Archive’ for file and ‘Directory’ for directory. ‘None’ can also be specified as default."]
        pub fn create(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            x_ms_file_attributes: impl Into<String>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                x_ms_file_attributes: x_ms_file_attributes.into(),
                x_ms_allow_trailing_dot: None,
                timeout: None,
                x_ms_meta: None,
                x_ms_file_permission: None,
                x_ms_file_permission_key: None,
                x_ms_file_creation_time: None,
                x_ms_file_last_write_time: None,
                x_ms_file_change_time: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "Removes the specified empty directory. Note that the directory must be empty before it can be deleted."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        pub fn delete(&self, share_name: impl Into<String>, directory: impl Into<String>) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                x_ms_allow_trailing_dot: None,
                timeout: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "Sets properties on the directory."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `x_ms_file_attributes`: If specified, the provided file attributes shall be set. Default value: ‘Archive’ for file and ‘Directory’ for directory. ‘None’ can also be specified as default."]
        pub fn set_properties(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            x_ms_file_attributes: impl Into<String>,
        ) -> set_properties::RequestBuilder {
            set_properties::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                x_ms_file_attributes: x_ms_file_attributes.into(),
                timeout: None,
                x_ms_file_permission: None,
                x_ms_file_permission_key: None,
                x_ms_file_creation_time: None,
                x_ms_file_last_write_time: None,
                x_ms_file_change_time: None,
                x_ms_allow_trailing_dot: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "Updates user defined metadata for the specified directory."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        pub fn set_metadata(&self, share_name: impl Into<String>, directory: impl Into<String>) -> set_metadata::RequestBuilder {
            set_metadata::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                timeout: None,
                x_ms_meta: None,
                x_ms_allow_trailing_dot: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "Returns a list of files or directories under the specified share or directory. It lists the contents only for a single level of the directory hierarchy."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        pub fn list_files_and_directories_segment(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
        ) -> list_files_and_directories_segment::RequestBuilder {
            list_files_and_directories_segment::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                prefix: None,
                sharesnapshot: None,
                marker: None,
                maxresults: None,
                timeout: None,
                include: Vec::new(),
                x_ms_file_extended_info: None,
                x_ms_allow_trailing_dot: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "Lists handles for directory."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        pub fn list_handles(&self, share_name: impl Into<String>, directory: impl Into<String>) -> list_handles::RequestBuilder {
            list_handles::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                marker: None,
                maxresults: None,
                timeout: None,
                sharesnapshot: None,
                x_ms_recursive: None,
                x_ms_allow_trailing_dot: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "Closes all handles open for given directory."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `x_ms_handle_id`: Specifies handle ID opened on the file or directory to be closed. Asterisk (‘*’) is a wildcard that specifies all handles."]
        pub fn force_close_handles(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            x_ms_handle_id: impl Into<String>,
        ) -> force_close_handles::RequestBuilder {
            force_close_handles::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                x_ms_handle_id: x_ms_handle_id.into(),
                timeout: None,
                marker: None,
                sharesnapshot: None,
                x_ms_recursive: None,
                x_ms_allow_trailing_dot: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "Renames a directory"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `x_ms_file_rename_source`: Required. Specifies the URI-style path of the source file, up to 2 KB in length."]
        pub fn rename(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            x_ms_file_rename_source: impl Into<String>,
        ) -> rename::RequestBuilder {
            rename::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                x_ms_file_rename_source: x_ms_file_rename_source.into(),
                timeout: None,
                x_ms_file_rename_replace_if_exists: None,
                x_ms_file_rename_ignore_readonly: None,
                x_ms_source_lease_id: None,
                x_ms_destination_lease_id: None,
                x_ms_file_attributes: None,
                x_ms_file_creation_time: None,
                x_ms_file_last_write_time: None,
                x_ms_file_change_time: None,
                x_ms_file_permission: None,
                x_ms_file_permission_key: None,
                x_ms_meta: None,
                x_ms_allow_trailing_dot: None,
                x_ms_source_allow_trailing_dot: None,
                x_ms_file_request_intent: None,
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
            #[doc = "A set of name-value pairs that contain metadata for the directory."]
            pub fn x_ms_meta(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-meta"))
            }
            #[doc = "The ETag contains a value that you can use to perform operations conditionally, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the Directory was last modified. Operations on files within the directory do not affect the last modified time of the directory."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "The value of this header is set to true if the directory metadata is completely encrypted using the specified algorithm. Otherwise, the value is set to false."]
            pub fn x_ms_server_encrypted(&self) -> azure_core::Result<bool> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-server-encrypted"))
            }
            #[doc = "Attributes set for the directory."]
            pub fn x_ms_file_attributes(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-attributes"))
            }
            #[doc = "Creation time for the directory."]
            pub fn x_ms_file_creation_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-creation-time"))?,
                )
            }
            #[doc = "Last write time for the directory."]
            pub fn x_ms_file_last_write_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-last-write-time"))?,
                )
            }
            #[doc = "Change time for the directory."]
            pub fn x_ms_file_change_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-change-time"))?,
                )
            }
            #[doc = "Key of the permission set for the directory."]
            pub fn x_ms_file_permission_key(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-permission-key"))
            }
            #[doc = "The fileId of the directory."]
            pub fn x_ms_file_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-id"))
            }
            #[doc = "The parent fileId of the directory."]
            pub fn x_ms_file_parent_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-parent-id"))
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}?restype=directory",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
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
            #[doc = "The ETag contains a value which represents the version of the directory, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the share was last modified. Any operation that modifies the directory or its properties updates the last modified time. Operations on files do not affect the last modified time of the directory."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "The value of this header is set to true if the contents of the request are successfully encrypted using the specified algorithm, and false otherwise."]
            pub fn x_ms_request_server_encrypted(&self) -> azure_core::Result<bool> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-request-server-encrypted"))
            }
            #[doc = "Key of the permission set for the directory."]
            pub fn x_ms_file_permission_key(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-permission-key"))
            }
            #[doc = "Attributes set for the directory."]
            pub fn x_ms_file_attributes(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-attributes"))
            }
            #[doc = "Creation time for the directory."]
            pub fn x_ms_file_creation_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-creation-time"))?,
                )
            }
            #[doc = "Last write time for the directory."]
            pub fn x_ms_file_last_write_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-last-write-time"))?,
                )
            }
            #[doc = "Change time for the directory."]
            pub fn x_ms_file_change_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-change-time"))?,
                )
            }
            #[doc = "The fileId of the directory."]
            pub fn x_ms_file_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-id"))
            }
            #[doc = "The parent fileId of the directory."]
            pub fn x_ms_file_parent_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-parent-id"))
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) x_ms_file_attributes: String,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_file_permission: Option<String>,
            pub(crate) x_ms_file_permission_key: Option<String>,
            pub(crate) x_ms_file_creation_time: Option<time::OffsetDateTime>,
            pub(crate) x_ms_file_last_write_time: Option<time::OffsetDateTime>,
            pub(crate) x_ms_file_change_time: Option<time::OffsetDateTime>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "A name-value pair to associate with a file storage object."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "If specified the permission (security descriptor) shall be set for the directory/file. This header can be used if Permission size is <= 8KB, else x-ms-file-permission-key header shall be used. Default value: Inherit. If SDDL is specified as input, it must have owner, group and dacl. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission(mut self, x_ms_file_permission: impl Into<String>) -> Self {
                self.x_ms_file_permission = Some(x_ms_file_permission.into());
                self
            }
            #[doc = "Key of the permission to be set for the directory/file. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission_key(mut self, x_ms_file_permission_key: impl Into<String>) -> Self {
                self.x_ms_file_permission_key = Some(x_ms_file_permission_key.into());
                self
            }
            #[doc = "Creation time for the file/directory. Default value: Now."]
            pub fn x_ms_file_creation_time(mut self, x_ms_file_creation_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_creation_time = Some(x_ms_file_creation_time.into());
                self
            }
            #[doc = "Last write time for the file/directory. Default value: Now."]
            pub fn x_ms_file_last_write_time(mut self, x_ms_file_last_write_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_last_write_time = Some(x_ms_file_last_write_time.into());
                self
            }
            #[doc = "Change time for the file/directory. Default value: Now."]
            pub fn x_ms_file_change_time(mut self, x_ms_file_change_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_change_time = Some(x_ms_file_change_time.into());
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}?restype=directory",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(x_ms_file_permission) = &this.x_ms_file_permission {
                            req.insert_header("x-ms-file-permission", x_ms_file_permission);
                        }
                        if let Some(x_ms_file_permission_key) = &this.x_ms_file_permission_key {
                            req.insert_header("x-ms-file-permission-key", x_ms_file_permission_key);
                        }
                        req.insert_header("x-ms-file-attributes", &this.x_ms_file_attributes);
                        if let Some(x_ms_file_creation_time) = &this.x_ms_file_creation_time {
                            req.insert_header("x-ms-file-creation-time", &x_ms_file_creation_time.to_string());
                        }
                        if let Some(x_ms_file_last_write_time) = &this.x_ms_file_last_write_time {
                            req.insert_header("x-ms-file-last-write-time", &x_ms_file_last_write_time.to_string());
                        }
                        if let Some(x_ms_file_change_time) = &this.x_ms_file_change_time {
                            req.insert_header("x-ms-file-change-time", &x_ms_file_change_time.to_string());
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
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
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}?restype=directory",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
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
            #[doc = "The ETag contains a value which represents the version of the file, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Returns the date and time the directory was last modified. Any operation that modifies the directory or its properties updates the last modified time. Operations on files do not affect the last modified time of the directory."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "The value of this header is set to true if the contents of the request are successfully encrypted using the specified algorithm, and false otherwise."]
            pub fn x_ms_request_server_encrypted(&self) -> azure_core::Result<bool> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-request-server-encrypted"))
            }
            #[doc = "Key of the permission set for the directory."]
            pub fn x_ms_file_permission_key(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-permission-key"))
            }
            #[doc = "Attributes set for the directory."]
            pub fn x_ms_file_attributes(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-attributes"))
            }
            #[doc = "Creation time for the directory."]
            pub fn x_ms_file_creation_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-creation-time"))?,
                )
            }
            #[doc = "Last write time for the directory."]
            pub fn x_ms_file_last_write_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-last-write-time"))?,
                )
            }
            #[doc = "Change time for the directory."]
            pub fn x_ms_file_change_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-change-time"))?,
                )
            }
            #[doc = "The fileId of the directory."]
            pub fn x_ms_file_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-id"))
            }
            #[doc = "The parent fileId of the directory."]
            pub fn x_ms_file_parent_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-parent-id"))
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) x_ms_file_attributes: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_file_permission: Option<String>,
            pub(crate) x_ms_file_permission_key: Option<String>,
            pub(crate) x_ms_file_creation_time: Option<time::OffsetDateTime>,
            pub(crate) x_ms_file_last_write_time: Option<time::OffsetDateTime>,
            pub(crate) x_ms_file_change_time: Option<time::OffsetDateTime>,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified the permission (security descriptor) shall be set for the directory/file. This header can be used if Permission size is <= 8KB, else x-ms-file-permission-key header shall be used. Default value: Inherit. If SDDL is specified as input, it must have owner, group and dacl. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission(mut self, x_ms_file_permission: impl Into<String>) -> Self {
                self.x_ms_file_permission = Some(x_ms_file_permission.into());
                self
            }
            #[doc = "Key of the permission to be set for the directory/file. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission_key(mut self, x_ms_file_permission_key: impl Into<String>) -> Self {
                self.x_ms_file_permission_key = Some(x_ms_file_permission_key.into());
                self
            }
            #[doc = "Creation time for the file/directory. Default value: Now."]
            pub fn x_ms_file_creation_time(mut self, x_ms_file_creation_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_creation_time = Some(x_ms_file_creation_time.into());
                self
            }
            #[doc = "Last write time for the file/directory. Default value: Now."]
            pub fn x_ms_file_last_write_time(mut self, x_ms_file_last_write_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_last_write_time = Some(x_ms_file_last_write_time.into());
                self
            }
            #[doc = "Change time for the file/directory. Default value: Now."]
            pub fn x_ms_file_change_time(mut self, x_ms_file_change_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_change_time = Some(x_ms_file_change_time.into());
                self
            }
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}?restype=directory&comp=properties",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_file_permission) = &this.x_ms_file_permission {
                            req.insert_header("x-ms-file-permission", x_ms_file_permission);
                        }
                        if let Some(x_ms_file_permission_key) = &this.x_ms_file_permission_key {
                            req.insert_header("x-ms-file-permission-key", x_ms_file_permission_key);
                        }
                        req.insert_header("x-ms-file-attributes", &this.x_ms_file_attributes);
                        if let Some(x_ms_file_creation_time) = &this.x_ms_file_creation_time {
                            req.insert_header("x-ms-file-creation-time", &x_ms_file_creation_time.to_string());
                        }
                        if let Some(x_ms_file_last_write_time) = &this.x_ms_file_last_write_time {
                            req.insert_header("x-ms-file-last-write-time", &x_ms_file_last_write_time.to_string());
                        }
                        if let Some(x_ms_file_change_time) = &this.x_ms_file_change_time {
                            req.insert_header("x-ms-file-change-time", &x_ms_file_change_time.to_string());
                        }
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
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
            #[doc = "The ETag contains a value which represents the version of the directory, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "The value of this header is set to true if the contents of the request are successfully encrypted using the specified algorithm, and false otherwise."]
            pub fn x_ms_request_server_encrypted(&self) -> azure_core::Result<bool> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-request-server-encrypted"))
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "A name-value pair to associate with a file storage object."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}?restype=directory&comp=metadata",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_files_and_directories_segment {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ListFilesAndDirectoriesSegmentResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ListFilesAndDirectoriesSegmentResponse = azure_core::xml::read_xml(&bytes)?;
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
            #[doc = "Specifies the format in which the results are returned. Currently this value is 'application/xml'."]
            pub fn content_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-type"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) prefix: Option<String>,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) marker: Option<String>,
            pub(crate) maxresults: Option<i64>,
            pub(crate) timeout: Option<i64>,
            pub(crate) include: Vec<String>,
            pub(crate) x_ms_file_extended_info: Option<bool>,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Filters the results to return only entries whose name begins with the specified prefix."]
            pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
                self.prefix = Some(prefix.into());
                self
            }
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            #[doc = "A string value that identifies the portion of the list to be returned with the next list operation. The operation returns a marker value within the response body if the list returned was not complete. The marker value may then be used in a subsequent call to request the next set of list items. The marker value is opaque to the client."]
            pub fn marker(mut self, marker: impl Into<String>) -> Self {
                self.marker = Some(marker.into());
                self
            }
            #[doc = "Specifies the maximum number of entries to return. If the request does not specify maxresults, or specifies a value greater than 5,000, the server will return up to 5,000 items."]
            pub fn maxresults(mut self, maxresults: i64) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Include this parameter to specify one or more datasets to include in the response."]
            pub fn include(mut self, include: Vec<String>) -> Self {
                self.include = include;
                self
            }
            #[doc = "Include extended information."]
            pub fn x_ms_file_extended_info(mut self, x_ms_file_extended_info: bool) -> Self {
                self.x_ms_file_extended_info = Some(x_ms_file_extended_info);
                self
            }
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<models::ListFilesAndDirectoriesSegmentResponse, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/{}/{}?restype=directory&comp=list",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory
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
                                req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                                if let Some(prefix) = &this.prefix {
                                    req.url_mut().query_pairs_mut().append_pair("prefix", prefix);
                                }
                                if let Some(sharesnapshot) = &this.sharesnapshot {
                                    req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
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
                                if let Some(x_ms_file_extended_info) = &this.x_ms_file_extended_info {
                                    req.insert_header("x-ms-file-extended-info", &x_ms_file_extended_info.to_string());
                                }
                                if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                                    req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                                }
                                if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                                    req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
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
    pub mod list_handles {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ListHandlesResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ListHandlesResponse = azure_core::xml::read_xml(&bytes)?;
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
            #[doc = "Specifies the format in which the results are returned. Currently this value is 'application/xml'."]
            pub fn content_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-type"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) marker: Option<String>,
            pub(crate) maxresults: Option<i64>,
            pub(crate) timeout: Option<i64>,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) x_ms_recursive: Option<bool>,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "A string value that identifies the portion of the list to be returned with the next list operation. The operation returns a marker value within the response body if the list returned was not complete. The marker value may then be used in a subsequent call to request the next set of list items. The marker value is opaque to the client."]
            pub fn marker(mut self, marker: impl Into<String>) -> Self {
                self.marker = Some(marker.into());
                self
            }
            #[doc = "Specifies the maximum number of entries to return. If the request does not specify maxresults, or specifies a value greater than 5,000, the server will return up to 5,000 items."]
            pub fn maxresults(mut self, maxresults: i64) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            #[doc = "Specifies operation should apply to the directory specified in the URI, its files, its subdirectories and their files."]
            pub fn x_ms_recursive(mut self, x_ms_recursive: bool) -> Self {
                self.x_ms_recursive = Some(x_ms_recursive);
                self
            }
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}?comp=listhandles",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(marker) = &this.marker {
                            req.url_mut().query_pairs_mut().append_pair("marker", marker);
                        }
                        if let Some(maxresults) = &this.maxresults {
                            req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                        }
                        if let Some(x_ms_recursive) = &this.x_ms_recursive {
                            req.insert_header("x-ms-recursive", &x_ms_recursive.to_string());
                        }
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ListHandlesResponse>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ListHandlesResponse>>;
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
    pub mod force_close_handles {
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
            #[doc = "Indicates the version of the File service used to execute the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "A string describing next handle to be closed. It is returned when more handles need to be closed to complete the request."]
            pub fn x_ms_marker(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-marker"))
            }
            #[doc = "Contains count of number of handles closed."]
            pub fn x_ms_number_of_handles_closed(&self) -> azure_core::Result<i32> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-number-of-handles-closed"))
            }
            #[doc = "Contains count of number of handles that failed to close."]
            pub fn x_ms_number_of_handles_failed(&self) -> azure_core::Result<i32> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-number-of-handles-failed"))
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) x_ms_handle_id: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) marker: Option<String>,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) x_ms_recursive: Option<bool>,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "A string value that identifies the portion of the list to be returned with the next list operation. The operation returns a marker value within the response body if the list returned was not complete. The marker value may then be used in a subsequent call to request the next set of list items. The marker value is opaque to the client."]
            pub fn marker(mut self, marker: impl Into<String>) -> Self {
                self.marker = Some(marker.into());
                self
            }
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            #[doc = "Specifies operation should apply to the directory specified in the URI, its files, its subdirectories and their files."]
            pub fn x_ms_recursive(mut self, x_ms_recursive: bool) -> Self {
                self.x_ms_recursive = Some(x_ms_recursive);
                self
            }
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}?comp=forceclosehandles",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(marker) = &this.marker {
                            req.url_mut().query_pairs_mut().append_pair("marker", marker);
                        }
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                        }
                        req.insert_header("x-ms-handle-id", &this.x_ms_handle_id);
                        if let Some(x_ms_recursive) = &this.x_ms_recursive {
                            req.insert_header("x-ms-recursive", &x_ms_recursive.to_string());
                        }
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod rename {
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
            #[doc = "The ETag contains a value which represents the version of the file, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the share was last modified. Any operation that modifies the directory or its properties updates the last modified time. Operations on files do not affect the last modified time of the directory."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "The value of this header is set to true if the contents of the request are successfully encrypted using the specified algorithm, and false otherwise."]
            pub fn x_ms_request_server_encrypted(&self) -> azure_core::Result<bool> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-request-server-encrypted"))
            }
            #[doc = "Key of the permission set for the file."]
            pub fn x_ms_file_permission_key(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-permission-key"))
            }
            #[doc = "Attributes set for the file."]
            pub fn x_ms_file_attributes(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-attributes"))
            }
            #[doc = "Creation time for the file."]
            pub fn x_ms_file_creation_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-creation-time"))?,
                )
            }
            #[doc = "Last write time for the file."]
            pub fn x_ms_file_last_write_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-last-write-time"))?,
                )
            }
            #[doc = "Change time for the file."]
            pub fn x_ms_file_change_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-change-time"))?,
                )
            }
            #[doc = "The fileId of the file."]
            pub fn x_ms_file_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-id"))
            }
            #[doc = "The parent fileId of the directory."]
            pub fn x_ms_file_parent_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-parent-id"))
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) x_ms_file_rename_source: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_file_rename_replace_if_exists: Option<bool>,
            pub(crate) x_ms_file_rename_ignore_readonly: Option<bool>,
            pub(crate) x_ms_source_lease_id: Option<String>,
            pub(crate) x_ms_destination_lease_id: Option<String>,
            pub(crate) x_ms_file_attributes: Option<String>,
            pub(crate) x_ms_file_creation_time: Option<String>,
            pub(crate) x_ms_file_last_write_time: Option<String>,
            pub(crate) x_ms_file_change_time: Option<String>,
            pub(crate) x_ms_file_permission: Option<String>,
            pub(crate) x_ms_file_permission_key: Option<String>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) x_ms_source_allow_trailing_dot: Option<bool>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional. A boolean value for if the destination file already exists, whether this request will overwrite the file or not. If true, the rename will succeed and will overwrite the destination file. If not provided or if false and the destination file does exist, the request will not overwrite the destination file. If provided and the destination file doesn’t exist, the rename will succeed. Note: This value does not override the x-ms-file-copy-ignore-read-only header value."]
            pub fn x_ms_file_rename_replace_if_exists(mut self, x_ms_file_rename_replace_if_exists: bool) -> Self {
                self.x_ms_file_rename_replace_if_exists = Some(x_ms_file_rename_replace_if_exists);
                self
            }
            #[doc = "Optional. A boolean value that specifies whether the ReadOnly attribute on a preexisting destination file should be respected. If true, the rename will succeed, otherwise, a previous file at the destination with the ReadOnly attribute set will cause the rename to fail."]
            pub fn x_ms_file_rename_ignore_readonly(mut self, x_ms_file_rename_ignore_readonly: bool) -> Self {
                self.x_ms_file_rename_ignore_readonly = Some(x_ms_file_rename_ignore_readonly);
                self
            }
            #[doc = "Required if the source file has an active infinite lease."]
            pub fn x_ms_source_lease_id(mut self, x_ms_source_lease_id: impl Into<String>) -> Self {
                self.x_ms_source_lease_id = Some(x_ms_source_lease_id.into());
                self
            }
            #[doc = "Required if the destination file has an active infinite lease. The lease ID specified for this header must match the lease ID of the destination file. If the request does not include the lease ID or it is not valid, the operation fails with status code 412 (Precondition Failed). If this header is specified and the destination file does not currently have an active lease, the operation will also fail with status code 412 (Precondition Failed)."]
            pub fn x_ms_destination_lease_id(mut self, x_ms_destination_lease_id: impl Into<String>) -> Self {
                self.x_ms_destination_lease_id = Some(x_ms_destination_lease_id.into());
                self
            }
            #[doc = "Specifies either the option to copy file attributes from a source file(source) to a target file or a list of attributes to set on a target file."]
            pub fn x_ms_file_attributes(mut self, x_ms_file_attributes: impl Into<String>) -> Self {
                self.x_ms_file_attributes = Some(x_ms_file_attributes.into());
                self
            }
            #[doc = "Specifies either the option to copy file creation time from a source file(source) to a target file or a time value in ISO 8601 format to set as creation time on a target file."]
            pub fn x_ms_file_creation_time(mut self, x_ms_file_creation_time: impl Into<String>) -> Self {
                self.x_ms_file_creation_time = Some(x_ms_file_creation_time.into());
                self
            }
            #[doc = "Specifies either the option to copy file last write time from a source file(source) to a target file or a time value in ISO 8601 format to set as last write time on a target file."]
            pub fn x_ms_file_last_write_time(mut self, x_ms_file_last_write_time: impl Into<String>) -> Self {
                self.x_ms_file_last_write_time = Some(x_ms_file_last_write_time.into());
                self
            }
            #[doc = "Specifies either the option to copy file last write time from a source file(source) to a target file or a time value in ISO 8601 format to set as last write time on a target file."]
            pub fn x_ms_file_change_time(mut self, x_ms_file_change_time: impl Into<String>) -> Self {
                self.x_ms_file_change_time = Some(x_ms_file_change_time.into());
                self
            }
            #[doc = "If specified the permission (security descriptor) shall be set for the directory/file. This header can be used if Permission size is <= 8KB, else x-ms-file-permission-key header shall be used. Default value: Inherit. If SDDL is specified as input, it must have owner, group and dacl. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission(mut self, x_ms_file_permission: impl Into<String>) -> Self {
                self.x_ms_file_permission = Some(x_ms_file_permission.into());
                self
            }
            #[doc = "Key of the permission to be set for the directory/file. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission_key(mut self, x_ms_file_permission_key: impl Into<String>) -> Self {
                self.x_ms_file_permission_key = Some(x_ms_file_permission_key.into());
                self
            }
            #[doc = "A name-value pair to associate with a file storage object."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "If true, the trailing dot will not be trimmed from the source URI."]
            pub fn x_ms_source_allow_trailing_dot(mut self, x_ms_source_allow_trailing_dot: bool) -> Self {
                self.x_ms_source_allow_trailing_dot = Some(x_ms_source_allow_trailing_dot);
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}?restype=directory&comp=rename",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-file-rename-source", &this.x_ms_file_rename_source);
                        if let Some(x_ms_file_rename_replace_if_exists) = &this.x_ms_file_rename_replace_if_exists {
                            req.insert_header(
                                "x-ms-file-rename-replace-if-exists",
                                &x_ms_file_rename_replace_if_exists.to_string(),
                            );
                        }
                        if let Some(x_ms_file_rename_ignore_readonly) = &this.x_ms_file_rename_ignore_readonly {
                            req.insert_header("x-ms-file-rename-ignore-readonly", &x_ms_file_rename_ignore_readonly.to_string());
                        }
                        if let Some(x_ms_source_lease_id) = &this.x_ms_source_lease_id {
                            req.insert_header("x-ms-source-lease-id", x_ms_source_lease_id);
                        }
                        if let Some(x_ms_destination_lease_id) = &this.x_ms_destination_lease_id {
                            req.insert_header("x-ms-destination-lease-id", x_ms_destination_lease_id);
                        }
                        if let Some(x_ms_file_attributes) = &this.x_ms_file_attributes {
                            req.insert_header("x-ms-file-attributes", x_ms_file_attributes);
                        }
                        if let Some(x_ms_file_creation_time) = &this.x_ms_file_creation_time {
                            req.insert_header("x-ms-file-creation-time", x_ms_file_creation_time);
                        }
                        if let Some(x_ms_file_last_write_time) = &this.x_ms_file_last_write_time {
                            req.insert_header("x-ms-file-last-write-time", x_ms_file_last_write_time);
                        }
                        if let Some(x_ms_file_change_time) = &this.x_ms_file_change_time {
                            req.insert_header("x-ms-file-change-time", x_ms_file_change_time);
                        }
                        if let Some(x_ms_file_permission) = &this.x_ms_file_permission {
                            req.insert_header("x-ms-file-permission", x_ms_file_permission);
                        }
                        if let Some(x_ms_file_permission_key) = &this.x_ms_file_permission_key {
                            req.insert_header("x-ms-file-permission-key", x_ms_file_permission_key);
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(x_ms_source_allow_trailing_dot) = &this.x_ms_source_allow_trailing_dot {
                            req.insert_header("x-ms-source-allow-trailing-dot", &x_ms_source_allow_trailing_dot.to_string());
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
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
pub mod file {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Reads or downloads a file from the system, including its metadata and properties."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        pub fn download(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
        ) -> download::RequestBuilder {
            download::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_allow_trailing_dot: None,
                timeout: None,
                x_ms_range: None,
                x_ms_range_get_content_md5: None,
                x_ms_lease_id: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "Creates a new file or replaces a file. Note it only initializes the file with no content."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_content_length`: Specifies the maximum size for the file, up to 4 TB."]
        #[doc = "* `x_ms_type`: Dummy constant parameter, file type can only be file."]
        #[doc = "* `x_ms_file_attributes`: If specified, the provided file attributes shall be set. Default value: ‘Archive’ for file and ‘Directory’ for directory. ‘None’ can also be specified as default."]
        pub fn create(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_content_length: i64,
            x_ms_type: impl Into<String>,
            x_ms_file_attributes: impl Into<String>,
        ) -> create::RequestBuilder {
            create::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_content_length,
                x_ms_type: x_ms_type.into(),
                x_ms_file_attributes: x_ms_file_attributes.into(),
                x_ms_allow_trailing_dot: None,
                timeout: None,
                x_ms_content_type: None,
                x_ms_content_encoding: None,
                x_ms_content_language: None,
                x_ms_cache_control: None,
                x_ms_content_md5: None,
                x_ms_content_disposition: None,
                x_ms_meta: None,
                x_ms_file_permission: None,
                x_ms_file_permission_key: None,
                x_ms_file_creation_time: None,
                x_ms_file_last_write_time: None,
                x_ms_file_change_time: None,
                x_ms_lease_id: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "removes the file from the storage account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        pub fn delete(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_allow_trailing_dot: None,
                timeout: None,
                x_ms_lease_id: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "Returns all user-defined metadata, standard HTTP properties, and system properties for the file. It does not return the content of the file."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        pub fn get_properties(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
        ) -> get_properties::RequestBuilder {
            get_properties::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_allow_trailing_dot: None,
                sharesnapshot: None,
                timeout: None,
                x_ms_lease_id: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "Sets HTTP headers on the file."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_file_attributes`: If specified, the provided file attributes shall be set. Default value: ‘Archive’ for file and ‘Directory’ for directory. ‘None’ can also be specified as default."]
        pub fn set_http_headers(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_file_attributes: impl Into<String>,
        ) -> set_http_headers::RequestBuilder {
            set_http_headers::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_file_attributes: x_ms_file_attributes.into(),
                timeout: None,
                x_ms_content_length: None,
                x_ms_content_type: None,
                x_ms_content_encoding: None,
                x_ms_content_language: None,
                x_ms_cache_control: None,
                x_ms_content_md5: None,
                x_ms_content_disposition: None,
                x_ms_file_permission: None,
                x_ms_file_permission_key: None,
                x_ms_file_creation_time: None,
                x_ms_file_last_write_time: None,
                x_ms_file_change_time: None,
                x_ms_lease_id: None,
                x_ms_allow_trailing_dot: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "Updates user-defined metadata for the specified file."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        pub fn set_metadata(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
        ) -> set_metadata::RequestBuilder {
            set_metadata::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                timeout: None,
                x_ms_meta: None,
                x_ms_lease_id: None,
                x_ms_allow_trailing_dot: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "[Update] The Lease File operation establishes and manages a lock on a file for write and delete operations"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        pub fn acquire_lease(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
        ) -> acquire_lease::RequestBuilder {
            acquire_lease::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                timeout: None,
                x_ms_lease_duration: None,
                x_ms_proposed_lease_id: None,
                x_ms_client_request_id: None,
                x_ms_allow_trailing_dot: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "[Update] The Lease File operation establishes and manages a lock on a file for write and delete operations"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        #[doc = "* `x_ms_lease_id`: Specifies the current lease ID on the resource."]
        pub fn release_lease(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
            x_ms_lease_id: impl Into<String>,
        ) -> release_lease::RequestBuilder {
            release_lease::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                x_ms_lease_id: x_ms_lease_id.into(),
                timeout: None,
                x_ms_client_request_id: None,
                x_ms_allow_trailing_dot: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "[Update] The Lease File operation establishes and manages a lock on a file for write and delete operations"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        #[doc = "* `x_ms_lease_id`: Specifies the current lease ID on the resource."]
        pub fn change_lease(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
            x_ms_lease_id: impl Into<String>,
        ) -> change_lease::RequestBuilder {
            change_lease::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                x_ms_lease_id: x_ms_lease_id.into(),
                timeout: None,
                x_ms_proposed_lease_id: None,
                x_ms_client_request_id: None,
                x_ms_allow_trailing_dot: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "[Update] The Lease File operation establishes and manages a lock on a file for write and delete operations"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_lease_action`: Describes what lease action to take."]
        pub fn break_lease(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_lease_action: impl Into<String>,
        ) -> break_lease::RequestBuilder {
            break_lease::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_lease_action: x_ms_lease_action.into(),
                timeout: None,
                x_ms_lease_id: None,
                x_ms_client_request_id: None,
                x_ms_allow_trailing_dot: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "Upload a range of bytes to a file."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_range`: Specifies the range of bytes to be written. Both the start and end of the range must be specified. For an update operation, the range can be up to 4 MB in size. For a clear operation, the range can be up to the value of the file's full size. The File service accepts only a single byte range for the Range and 'x-ms-range' headers, and the byte range must be specified in the following format: bytes=startByte-endByte."]
        #[doc = "* `x_ms_write`: Specify one of the following options: - Update: Writes the bytes specified by the request body into the specified range. The Range and Content-Length headers must match to perform the update. - Clear: Clears the specified range and releases the space used in storage for that range. To clear a range, set the Content-Length header to zero, and set the Range header to a value that indicates the range to clear, up to maximum file size."]
        #[doc = "* `content_length`: Specifies the number of bytes being transmitted in the request body. When the x-ms-write header is set to clear, the value of this header must be set to zero."]
        pub fn upload_range(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_range: impl Into<String>,
            x_ms_write: impl Into<String>,
            content_length: i64,
        ) -> upload_range::RequestBuilder {
            upload_range::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_range: x_ms_range.into(),
                x_ms_write: x_ms_write.into(),
                content_length,
                optionalbody: None,
                timeout: None,
                content_md5: None,
                x_ms_lease_id: None,
                x_ms_file_last_write_time: None,
                x_ms_allow_trailing_dot: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "Upload a range of bytes to a file where the contents are read from a URL."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_range`: Writes data to the specified byte range in the file."]
        #[doc = "* `x_ms_copy_source`: Specifies the URL of the source file or blob, up to 2 KB in length. To copy a file to another file within the same storage account, you may use Shared Key to authenticate the source file. If you are copying a file from another storage account, or if you are copying a blob from the same storage account or another storage account, then you must authenticate the source file or blob using a shared access signature. If the source is a public blob, no authentication is required to perform the copy operation. A file in a share snapshot can also be specified as a copy source."]
        #[doc = "* `x_ms_write`: Only update is supported: - Update: Writes the bytes downloaded from the source url into the specified range."]
        #[doc = "* `content_length`: Specifies the number of bytes being transmitted in the request body. When the x-ms-write header is set to clear, the value of this header must be set to zero."]
        pub fn upload_range_from_url(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_range: impl Into<String>,
            x_ms_copy_source: impl Into<String>,
            x_ms_write: impl Into<String>,
            content_length: i64,
        ) -> upload_range_from_url::RequestBuilder {
            upload_range_from_url::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_range: x_ms_range.into(),
                x_ms_copy_source: x_ms_copy_source.into(),
                x_ms_write: x_ms_write.into(),
                content_length,
                timeout: None,
                x_ms_source_range: None,
                x_ms_source_content_crc64: None,
                x_ms_source_if_match_crc64: None,
                x_ms_source_if_none_match_crc64: None,
                x_ms_lease_id: None,
                x_ms_copy_source_authorization: None,
                x_ms_file_last_write_time: None,
                x_ms_allow_trailing_dot: None,
                x_ms_source_allow_trailing_dot: None,
            }
        }
        #[doc = "Returns the list of valid ranges for a file."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        pub fn get_range_list(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
        ) -> get_range_list::RequestBuilder {
            get_range_list::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                sharesnapshot: None,
                prevsharesnapshot: None,
                timeout: None,
                x_ms_range: None,
                x_ms_lease_id: None,
                x_ms_allow_trailing_dot: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "Copies a blob or file to a destination file within the storage account."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_copy_source`: Specifies the URL of the source file or blob, up to 2 KB in length. To copy a file to another file within the same storage account, you may use Shared Key to authenticate the source file. If you are copying a file from another storage account, or if you are copying a blob from the same storage account or another storage account, then you must authenticate the source file or blob using a shared access signature. If the source is a public blob, no authentication is required to perform the copy operation. A file in a share snapshot can also be specified as a copy source."]
        pub fn start_copy(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_copy_source: impl Into<String>,
        ) -> start_copy::RequestBuilder {
            start_copy::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_copy_source: x_ms_copy_source.into(),
                timeout: None,
                x_ms_meta: None,
                x_ms_file_permission: None,
                x_ms_file_permission_key: None,
                x_ms_file_permission_copy_mode: None,
                x_ms_file_copy_ignore_readonly: None,
                x_ms_file_attributes: None,
                x_ms_file_creation_time: None,
                x_ms_file_last_write_time: None,
                x_ms_file_change_time: None,
                x_ms_file_copy_set_archive: None,
                x_ms_lease_id: None,
                x_ms_allow_trailing_dot: None,
                x_ms_source_allow_trailing_dot: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "Aborts a pending Copy File operation, and leaves a destination file with zero length and full metadata."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_copy_action`: Abort."]
        pub fn abort_copy(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_copy_action: impl Into<String>,
        ) -> abort_copy::RequestBuilder {
            abort_copy::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_copy_action: x_ms_copy_action.into(),
                timeout: None,
                x_ms_lease_id: None,
                x_ms_allow_trailing_dot: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "Lists handles for file"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        pub fn list_handles(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
        ) -> list_handles::RequestBuilder {
            list_handles::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                marker: None,
                maxresults: None,
                timeout: None,
                sharesnapshot: None,
                x_ms_allow_trailing_dot: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "Closes all handles open for given file"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_handle_id`: Specifies handle ID opened on the file or directory to be closed. Asterisk (‘*’) is a wildcard that specifies all handles."]
        pub fn force_close_handles(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_handle_id: impl Into<String>,
        ) -> force_close_handles::RequestBuilder {
            force_close_handles::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_handle_id: x_ms_handle_id.into(),
                timeout: None,
                marker: None,
                sharesnapshot: None,
                x_ms_allow_trailing_dot: None,
                x_ms_file_request_intent: None,
            }
        }
        #[doc = "Renames a file"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `share_name`: The name of the target share."]
        #[doc = "* `directory`: The path of the target directory."]
        #[doc = "* `file_name`: The path of the target file."]
        #[doc = "* `x_ms_file_rename_source`: Required. Specifies the URI-style path of the source file, up to 2 KB in length."]
        pub fn rename(
            &self,
            share_name: impl Into<String>,
            directory: impl Into<String>,
            file_name: impl Into<String>,
            x_ms_file_rename_source: impl Into<String>,
        ) -> rename::RequestBuilder {
            rename::RequestBuilder {
                client: self.0.clone(),
                share_name: share_name.into(),
                directory: directory.into(),
                file_name: file_name.into(),
                x_ms_file_rename_source: x_ms_file_rename_source.into(),
                timeout: None,
                x_ms_file_rename_replace_if_exists: None,
                x_ms_file_rename_ignore_readonly: None,
                x_ms_source_lease_id: None,
                x_ms_destination_lease_id: None,
                x_ms_file_attributes: None,
                x_ms_file_creation_time: None,
                x_ms_file_last_write_time: None,
                x_ms_file_change_time: None,
                x_ms_file_permission: None,
                x_ms_file_permission_key: None,
                x_ms_meta: None,
                x_ms_content_type: None,
                x_ms_allow_trailing_dot: None,
                x_ms_source_allow_trailing_dot: None,
                x_ms_file_request_intent: None,
            }
        }
    }
    pub mod download {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<bytes::Bytes> {
                let bytes = self.0.into_body().collect().await?;
                let body = bytes;
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
            #[doc = "Returns the date and time the file was last modified. Any operation that modifies the file or its properties updates the last modified time."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "A set of name-value pairs associated with this file as user-defined metadata."]
            pub fn x_ms_meta(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-meta"))
            }
            #[doc = "The number of bytes present in the response body."]
            pub fn content_length(&self) -> azure_core::Result<i64> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("content-length"))
            }
            #[doc = "The content type specified for the file. The default content type is 'application/octet-stream'"]
            pub fn content_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-type"))
            }
            #[doc = "Indicates the range of bytes returned if the client requested a subset of the file by setting the Range request header."]
            pub fn content_range(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-range"))
            }
            #[doc = "The ETag contains a value that you can use to perform operations conditionally, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "If the file has an MD5 hash and the request is to read the full file, this response header is returned so that the client can check for message content integrity. If the request is to read a specified range and the 'x-ms-range-get-content-md5' is set to true, then the request returns an MD5 hash for the range, as long as the range size is less than or equal to 4 MB. If neither of these sets of conditions is true, then no value is returned for the 'Content-MD5' header."]
            pub fn content_md5(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-md5"))
            }
            #[doc = "Returns the value that was specified for the Content-Encoding request header."]
            pub fn content_encoding(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-encoding"))
            }
            #[doc = "Returned if it was previously specified for the file."]
            pub fn cache_control(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("cache-control"))
            }
            #[doc = "Returns the value that was specified for the 'x-ms-content-disposition' header and specifies how to process the response."]
            pub fn content_disposition(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-disposition"))
            }
            #[doc = "Returns the value that was specified for the Content-Language request header."]
            pub fn content_language(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-language"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "Indicates that the service supports requests for partial file content."]
            pub fn accept_ranges(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("accept-ranges"))
            }
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "Conclusion time of the last attempted Copy File operation where this file was the destination file. This value can specify the time of a completed, aborted, or failed copy attempt."]
            pub fn x_ms_copy_completion_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-completion-time"))?,
                )
            }
            #[doc = "Only appears when x-ms-copy-status is failed or pending. Describes cause of fatal or non-fatal copy operation failure."]
            pub fn x_ms_copy_status_description(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-status-description"))
            }
            #[doc = "String identifier for the last attempted Copy File operation where this file was the destination file."]
            pub fn x_ms_copy_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-id"))
            }
            #[doc = "Contains the number of bytes copied and the total bytes in the source in the last attempted Copy File operation where this file was the destination file. Can show between 0 and Content-Length bytes copied."]
            pub fn x_ms_copy_progress(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-progress"))
            }
            #[doc = "URL up to 2KB in length that specifies the source file used in the last attempted Copy File operation where this file was the destination file."]
            pub fn x_ms_copy_source(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-source"))
            }
            #[doc = "State of the copy operation identified by 'x-ms-copy-id'."]
            pub fn x_ms_copy_status(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-status"))
            }
            #[doc = "If the file has a MD5 hash, and if request contains range header (Range or x-ms-range), this response header is returned with the value of the whole file's MD5 value. This value may or may not be equal to the value returned in Content-MD5 header, with the latter calculated from the requested range."]
            pub fn x_ms_content_md5(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-content-md5"))
            }
            #[doc = "The value of this header is set to true if the file data and application metadata are completely encrypted using the specified algorithm. Otherwise, the value is set to false (when the file is unencrypted, or if only parts of the file/application metadata are encrypted)."]
            pub fn x_ms_server_encrypted(&self) -> azure_core::Result<bool> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-server-encrypted"))
            }
            #[doc = "Attributes set for the file."]
            pub fn x_ms_file_attributes(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-attributes"))
            }
            #[doc = "Creation time for the file."]
            pub fn x_ms_file_creation_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-creation-time"))?,
                )
            }
            #[doc = "Last write time for the file."]
            pub fn x_ms_file_last_write_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-last-write-time"))?,
                )
            }
            #[doc = "Change time for the file."]
            pub fn x_ms_file_change_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-change-time"))?,
                )
            }
            #[doc = "Key of the permission set for the file."]
            pub fn x_ms_file_permission_key(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-permission-key"))
            }
            #[doc = "The fileId of the file."]
            pub fn x_ms_file_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-id"))
            }
            #[doc = "The parent fileId of the file."]
            pub fn x_ms_file_parent_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-parent-id"))
            }
            #[doc = "When a file is leased, specifies whether the lease is of infinite or fixed duration."]
            pub fn x_ms_lease_duration(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-duration"))
            }
            #[doc = "Lease state of the file."]
            pub fn x_ms_lease_state(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-state"))
            }
            #[doc = "The current lease status of the file."]
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_range: Option<String>,
            pub(crate) x_ms_range_get_content_md5: Option<bool>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Return file data only from the specified byte range."]
            pub fn x_ms_range(mut self, x_ms_range: impl Into<String>) -> Self {
                self.x_ms_range = Some(x_ms_range.into());
                self
            }
            #[doc = "When this header is set to true and specified together with the Range header, the service returns the MD5 hash for the range, as long as the range is less than or equal to 4 MB in size."]
            pub fn x_ms_range_get_content_md5(mut self, x_ms_range_get_content_md5: bool) -> Self {
                self.x_ms_range_get_content_md5 = Some(x_ms_range_get_content_md5);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}/{}",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_range) = &this.x_ms_range {
                            req.insert_header("x-ms-range", x_ms_range);
                        }
                        if let Some(x_ms_range_get_content_md5) = &this.x_ms_range_get_content_md5 {
                            req.insert_header("x-ms-range-get-content-md5", &x_ms_range_get_content_md5.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<bytes::Bytes>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<bytes::Bytes>>;
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
            #[doc = "The ETag contains a value which represents the version of the file, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the share was last modified. Any operation that modifies the directory or its properties updates the last modified time. Operations on files do not affect the last modified time of the directory."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "The value of this header is set to true if the contents of the request are successfully encrypted using the specified algorithm, and false otherwise."]
            pub fn x_ms_request_server_encrypted(&self) -> azure_core::Result<bool> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-request-server-encrypted"))
            }
            #[doc = "Key of the permission set for the file."]
            pub fn x_ms_file_permission_key(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-permission-key"))
            }
            #[doc = "Attributes set for the file."]
            pub fn x_ms_file_attributes(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-attributes"))
            }
            #[doc = "Creation time for the file."]
            pub fn x_ms_file_creation_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-creation-time"))?,
                )
            }
            #[doc = "Last write time for the file."]
            pub fn x_ms_file_last_write_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-last-write-time"))?,
                )
            }
            #[doc = "Change time for the file."]
            pub fn x_ms_file_change_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-change-time"))?,
                )
            }
            #[doc = "The fileId of the file."]
            pub fn x_ms_file_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-id"))
            }
            #[doc = "The parent fileId of the file."]
            pub fn x_ms_file_parent_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-parent-id"))
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_content_length: i64,
            pub(crate) x_ms_type: String,
            pub(crate) x_ms_file_attributes: String,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_content_type: Option<String>,
            pub(crate) x_ms_content_encoding: Option<String>,
            pub(crate) x_ms_content_language: Option<String>,
            pub(crate) x_ms_cache_control: Option<String>,
            pub(crate) x_ms_content_md5: Option<String>,
            pub(crate) x_ms_content_disposition: Option<String>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_file_permission: Option<String>,
            pub(crate) x_ms_file_permission_key: Option<String>,
            pub(crate) x_ms_file_creation_time: Option<time::OffsetDateTime>,
            pub(crate) x_ms_file_last_write_time: Option<time::OffsetDateTime>,
            pub(crate) x_ms_file_change_time: Option<time::OffsetDateTime>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Sets the MIME content type of the file. The default type is 'application/octet-stream'."]
            pub fn x_ms_content_type(mut self, x_ms_content_type: impl Into<String>) -> Self {
                self.x_ms_content_type = Some(x_ms_content_type.into());
                self
            }
            #[doc = "Specifies which content encodings have been applied to the file."]
            pub fn x_ms_content_encoding(mut self, x_ms_content_encoding: impl Into<String>) -> Self {
                self.x_ms_content_encoding = Some(x_ms_content_encoding.into());
                self
            }
            #[doc = "Specifies the natural languages used by this resource."]
            pub fn x_ms_content_language(mut self, x_ms_content_language: impl Into<String>) -> Self {
                self.x_ms_content_language = Some(x_ms_content_language.into());
                self
            }
            #[doc = "Sets the file's cache control. The File service stores this value but does not use or modify it."]
            pub fn x_ms_cache_control(mut self, x_ms_cache_control: impl Into<String>) -> Self {
                self.x_ms_cache_control = Some(x_ms_cache_control.into());
                self
            }
            #[doc = "Sets the file's MD5 hash."]
            pub fn x_ms_content_md5(mut self, x_ms_content_md5: impl Into<String>) -> Self {
                self.x_ms_content_md5 = Some(x_ms_content_md5.into());
                self
            }
            #[doc = "Sets the file's Content-Disposition header."]
            pub fn x_ms_content_disposition(mut self, x_ms_content_disposition: impl Into<String>) -> Self {
                self.x_ms_content_disposition = Some(x_ms_content_disposition.into());
                self
            }
            #[doc = "A name-value pair to associate with a file storage object."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "If specified the permission (security descriptor) shall be set for the directory/file. This header can be used if Permission size is <= 8KB, else x-ms-file-permission-key header shall be used. Default value: Inherit. If SDDL is specified as input, it must have owner, group and dacl. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission(mut self, x_ms_file_permission: impl Into<String>) -> Self {
                self.x_ms_file_permission = Some(x_ms_file_permission.into());
                self
            }
            #[doc = "Key of the permission to be set for the directory/file. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission_key(mut self, x_ms_file_permission_key: impl Into<String>) -> Self {
                self.x_ms_file_permission_key = Some(x_ms_file_permission_key.into());
                self
            }
            #[doc = "Creation time for the file/directory. Default value: Now."]
            pub fn x_ms_file_creation_time(mut self, x_ms_file_creation_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_creation_time = Some(x_ms_file_creation_time.into());
                self
            }
            #[doc = "Last write time for the file/directory. Default value: Now."]
            pub fn x_ms_file_last_write_time(mut self, x_ms_file_last_write_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_last_write_time = Some(x_ms_file_last_write_time.into());
                self
            }
            #[doc = "Change time for the file/directory. Default value: Now."]
            pub fn x_ms_file_change_time(mut self, x_ms_file_change_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_change_time = Some(x_ms_file_change_time.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}/{}",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-content-length", &this.x_ms_content_length.to_string());
                        req.insert_header("x-ms-type", &this.x_ms_type);
                        if let Some(x_ms_content_type) = &this.x_ms_content_type {
                            req.insert_header("x-ms-content-type", x_ms_content_type);
                        }
                        if let Some(x_ms_content_encoding) = &this.x_ms_content_encoding {
                            req.insert_header("x-ms-content-encoding", x_ms_content_encoding);
                        }
                        if let Some(x_ms_content_language) = &this.x_ms_content_language {
                            req.insert_header("x-ms-content-language", x_ms_content_language);
                        }
                        if let Some(x_ms_cache_control) = &this.x_ms_cache_control {
                            req.insert_header("x-ms-cache-control", x_ms_cache_control);
                        }
                        if let Some(x_ms_content_md5) = &this.x_ms_content_md5 {
                            req.insert_header("x-ms-content-md5", x_ms_content_md5);
                        }
                        if let Some(x_ms_content_disposition) = &this.x_ms_content_disposition {
                            req.insert_header("x-ms-content-disposition", x_ms_content_disposition);
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(x_ms_file_permission) = &this.x_ms_file_permission {
                            req.insert_header("x-ms-file-permission", x_ms_file_permission);
                        }
                        if let Some(x_ms_file_permission_key) = &this.x_ms_file_permission_key {
                            req.insert_header("x-ms-file-permission-key", x_ms_file_permission_key);
                        }
                        req.insert_header("x-ms-file-attributes", &this.x_ms_file_attributes);
                        if let Some(x_ms_file_creation_time) = &this.x_ms_file_creation_time {
                            req.insert_header("x-ms-file-creation-time", &x_ms_file_creation_time.to_string());
                        }
                        if let Some(x_ms_file_last_write_time) = &this.x_ms_file_last_write_time {
                            req.insert_header("x-ms-file-last-write-time", &x_ms_file_last_write_time.to_string());
                        }
                        if let Some(x_ms_file_change_time) = &this.x_ms_file_change_time {
                            req.insert_header("x-ms-file-change-time", &x_ms_file_change_time.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
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
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}/{}",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
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
            #[doc = "Returns the date and time the file was last modified. The date format follows RFC 1123. Any operation that modifies the file or its properties updates the last modified time."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "A set of name-value pairs associated with this file as user-defined metadata."]
            pub fn x_ms_meta(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-meta"))
            }
            #[doc = "Returns the type File. Reserved for future use."]
            pub fn x_ms_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-type"))
            }
            #[doc = "The size of the file in bytes. This header returns the value of the 'x-ms-content-length' header that is stored with the file."]
            pub fn content_length(&self) -> azure_core::Result<i64> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("content-length"))
            }
            #[doc = "The content type specified for the file. The default content type is 'application/octet-stream'"]
            pub fn content_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-type"))
            }
            #[doc = "The ETag contains a value that you can use to perform operations conditionally, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "If the Content-MD5 header has been set for the file, the Content-MD5 response header is returned so that the client can check for message content integrity."]
            pub fn content_md5(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-md5"))
            }
            #[doc = "If the Content-Encoding request header has previously been set for the file, the Content-Encoding value is returned in this header."]
            pub fn content_encoding(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-encoding"))
            }
            #[doc = "If the Cache-Control request header has previously been set for the file, the Cache-Control value is returned in this header."]
            pub fn cache_control(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("cache-control"))
            }
            #[doc = "Returns the value that was specified for the 'x-ms-content-disposition' header and specifies how to process the response."]
            pub fn content_disposition(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-disposition"))
            }
            #[doc = "Returns the value that was specified for the Content-Language request header."]
            pub fn content_language(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-language"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "Conclusion time of the last attempted Copy File operation where this file was the destination file. This value can specify the time of a completed, aborted, or failed copy attempt."]
            pub fn x_ms_copy_completion_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-completion-time"))?,
                )
            }
            #[doc = "Only appears when x-ms-copy-status is failed or pending. Describes cause of fatal or non-fatal copy operation failure."]
            pub fn x_ms_copy_status_description(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-status-description"))
            }
            #[doc = "String identifier for the last attempted Copy File operation where this file was the destination file."]
            pub fn x_ms_copy_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-id"))
            }
            #[doc = "Contains the number of bytes copied and the total bytes in the source in the last attempted Copy File operation where this file was the destination file. Can show between 0 and Content-Length bytes copied."]
            pub fn x_ms_copy_progress(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-progress"))
            }
            #[doc = "URL up to 2KB in length that specifies the source file used in the last attempted Copy File operation where this file was the destination file."]
            pub fn x_ms_copy_source(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-source"))
            }
            #[doc = "State of the copy operation identified by 'x-ms-copy-id'."]
            pub fn x_ms_copy_status(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-status"))
            }
            #[doc = "The value of this header is set to true if the file data and application metadata are completely encrypted using the specified algorithm. Otherwise, the value is set to false (when the file is unencrypted, or if only parts of the file/application metadata are encrypted)."]
            pub fn x_ms_server_encrypted(&self) -> azure_core::Result<bool> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-server-encrypted"))
            }
            #[doc = "Attributes set for the file."]
            pub fn x_ms_file_attributes(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-attributes"))
            }
            #[doc = "Creation time for the file."]
            pub fn x_ms_file_creation_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-creation-time"))?,
                )
            }
            #[doc = "Last write time for the file."]
            pub fn x_ms_file_last_write_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-last-write-time"))?,
                )
            }
            #[doc = "Change time for the file."]
            pub fn x_ms_file_change_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-change-time"))?,
                )
            }
            #[doc = "Key of the permission set for the file."]
            pub fn x_ms_file_permission_key(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-permission-key"))
            }
            #[doc = "The fileId of the file."]
            pub fn x_ms_file_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-id"))
            }
            #[doc = "The parent fileId of the file."]
            pub fn x_ms_file_parent_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-parent-id"))
            }
            #[doc = "When a file is leased, specifies whether the lease is of infinite or fixed duration."]
            pub fn x_ms_lease_duration(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-duration"))
            }
            #[doc = "Lease state of the file."]
            pub fn x_ms_lease_state(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-state"))
            }
            #[doc = "The current lease status of the file."]
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}/{}",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Head);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod set_http_headers {
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
            #[doc = "The ETag contains a value which represents the version of the file, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the directory was last modified. Any operation that modifies the directory or its properties updates the last modified time. Operations on files do not affect the last modified time of the directory."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "The value of this header is set to true if the contents of the request are successfully encrypted using the specified algorithm, and false otherwise."]
            pub fn x_ms_request_server_encrypted(&self) -> azure_core::Result<bool> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-request-server-encrypted"))
            }
            #[doc = "Key of the permission set for the file."]
            pub fn x_ms_file_permission_key(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-permission-key"))
            }
            #[doc = "Attributes set for the file."]
            pub fn x_ms_file_attributes(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-attributes"))
            }
            #[doc = "Creation time for the file."]
            pub fn x_ms_file_creation_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-creation-time"))?,
                )
            }
            #[doc = "Last write time for the file."]
            pub fn x_ms_file_last_write_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-last-write-time"))?,
                )
            }
            #[doc = "Change time for the file."]
            pub fn x_ms_file_change_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-change-time"))?,
                )
            }
            #[doc = "The fileId of the directory."]
            pub fn x_ms_file_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-id"))
            }
            #[doc = "The parent fileId of the directory."]
            pub fn x_ms_file_parent_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-parent-id"))
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_file_attributes: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_content_length: Option<i64>,
            pub(crate) x_ms_content_type: Option<String>,
            pub(crate) x_ms_content_encoding: Option<String>,
            pub(crate) x_ms_content_language: Option<String>,
            pub(crate) x_ms_cache_control: Option<String>,
            pub(crate) x_ms_content_md5: Option<String>,
            pub(crate) x_ms_content_disposition: Option<String>,
            pub(crate) x_ms_file_permission: Option<String>,
            pub(crate) x_ms_file_permission_key: Option<String>,
            pub(crate) x_ms_file_creation_time: Option<time::OffsetDateTime>,
            pub(crate) x_ms_file_last_write_time: Option<time::OffsetDateTime>,
            pub(crate) x_ms_file_change_time: Option<time::OffsetDateTime>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Resizes a file to the specified size. If the specified byte value is less than the current size of the file, then all ranges above the specified byte value are cleared."]
            pub fn x_ms_content_length(mut self, x_ms_content_length: i64) -> Self {
                self.x_ms_content_length = Some(x_ms_content_length);
                self
            }
            #[doc = "Sets the MIME content type of the file. The default type is 'application/octet-stream'."]
            pub fn x_ms_content_type(mut self, x_ms_content_type: impl Into<String>) -> Self {
                self.x_ms_content_type = Some(x_ms_content_type.into());
                self
            }
            #[doc = "Specifies which content encodings have been applied to the file."]
            pub fn x_ms_content_encoding(mut self, x_ms_content_encoding: impl Into<String>) -> Self {
                self.x_ms_content_encoding = Some(x_ms_content_encoding.into());
                self
            }
            #[doc = "Specifies the natural languages used by this resource."]
            pub fn x_ms_content_language(mut self, x_ms_content_language: impl Into<String>) -> Self {
                self.x_ms_content_language = Some(x_ms_content_language.into());
                self
            }
            #[doc = "Sets the file's cache control. The File service stores this value but does not use or modify it."]
            pub fn x_ms_cache_control(mut self, x_ms_cache_control: impl Into<String>) -> Self {
                self.x_ms_cache_control = Some(x_ms_cache_control.into());
                self
            }
            #[doc = "Sets the file's MD5 hash."]
            pub fn x_ms_content_md5(mut self, x_ms_content_md5: impl Into<String>) -> Self {
                self.x_ms_content_md5 = Some(x_ms_content_md5.into());
                self
            }
            #[doc = "Sets the file's Content-Disposition header."]
            pub fn x_ms_content_disposition(mut self, x_ms_content_disposition: impl Into<String>) -> Self {
                self.x_ms_content_disposition = Some(x_ms_content_disposition.into());
                self
            }
            #[doc = "If specified the permission (security descriptor) shall be set for the directory/file. This header can be used if Permission size is <= 8KB, else x-ms-file-permission-key header shall be used. Default value: Inherit. If SDDL is specified as input, it must have owner, group and dacl. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission(mut self, x_ms_file_permission: impl Into<String>) -> Self {
                self.x_ms_file_permission = Some(x_ms_file_permission.into());
                self
            }
            #[doc = "Key of the permission to be set for the directory/file. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission_key(mut self, x_ms_file_permission_key: impl Into<String>) -> Self {
                self.x_ms_file_permission_key = Some(x_ms_file_permission_key.into());
                self
            }
            #[doc = "Creation time for the file/directory. Default value: Now."]
            pub fn x_ms_file_creation_time(mut self, x_ms_file_creation_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_creation_time = Some(x_ms_file_creation_time.into());
                self
            }
            #[doc = "Last write time for the file/directory. Default value: Now."]
            pub fn x_ms_file_last_write_time(mut self, x_ms_file_last_write_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_last_write_time = Some(x_ms_file_last_write_time.into());
                self
            }
            #[doc = "Change time for the file/directory. Default value: Now."]
            pub fn x_ms_file_change_time(mut self, x_ms_file_change_time: impl Into<time::OffsetDateTime>) -> Self {
                self.x_ms_file_change_time = Some(x_ms_file_change_time.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}/{}?comp=properties",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_content_length) = &this.x_ms_content_length {
                            req.insert_header("x-ms-content-length", &x_ms_content_length.to_string());
                        }
                        if let Some(x_ms_content_type) = &this.x_ms_content_type {
                            req.insert_header("x-ms-content-type", x_ms_content_type);
                        }
                        if let Some(x_ms_content_encoding) = &this.x_ms_content_encoding {
                            req.insert_header("x-ms-content-encoding", x_ms_content_encoding);
                        }
                        if let Some(x_ms_content_language) = &this.x_ms_content_language {
                            req.insert_header("x-ms-content-language", x_ms_content_language);
                        }
                        if let Some(x_ms_cache_control) = &this.x_ms_cache_control {
                            req.insert_header("x-ms-cache-control", x_ms_cache_control);
                        }
                        if let Some(x_ms_content_md5) = &this.x_ms_content_md5 {
                            req.insert_header("x-ms-content-md5", x_ms_content_md5);
                        }
                        if let Some(x_ms_content_disposition) = &this.x_ms_content_disposition {
                            req.insert_header("x-ms-content-disposition", x_ms_content_disposition);
                        }
                        if let Some(x_ms_file_permission) = &this.x_ms_file_permission {
                            req.insert_header("x-ms-file-permission", x_ms_file_permission);
                        }
                        if let Some(x_ms_file_permission_key) = &this.x_ms_file_permission_key {
                            req.insert_header("x-ms-file-permission-key", x_ms_file_permission_key);
                        }
                        req.insert_header("x-ms-file-attributes", &this.x_ms_file_attributes);
                        if let Some(x_ms_file_creation_time) = &this.x_ms_file_creation_time {
                            req.insert_header("x-ms-file-creation-time", &x_ms_file_creation_time.to_string());
                        }
                        if let Some(x_ms_file_last_write_time) = &this.x_ms_file_last_write_time {
                            req.insert_header("x-ms-file-last-write-time", &x_ms_file_last_write_time.to_string());
                        }
                        if let Some(x_ms_file_change_time) = &this.x_ms_file_change_time {
                            req.insert_header("x-ms-file-change-time", &x_ms_file_change_time.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
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
            #[doc = "The ETag contains a value which represents the version of the file, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "The value of this header is set to true if the contents of the request are successfully encrypted using the specified algorithm, and false otherwise."]
            pub fn x_ms_request_server_encrypted(&self) -> azure_core::Result<bool> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-request-server-encrypted"))
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "A name-value pair to associate with a file storage object."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}/{}?comp=metadata",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod acquire_lease {
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
            #[doc = "The ETag contains a value that you can use to perform operations conditionally. If the request version is 2011-08-18 or newer, the ETag value will be in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the file was last modified. Any operation that modifies the file, including an update of the file's metadata or properties, changes the last-modified time of the file."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "Uniquely identifies a file's lease"]
            pub fn x_ms_lease_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-id"))
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
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_duration: Option<i64>,
            pub(crate) x_ms_proposed_lease_id: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specifies the duration of the lease, in seconds, or negative one (-1) for a lease that never expires. A non-infinite lease can be between 15 and 60 seconds. A lease duration cannot be changed using renew or change."]
            pub fn x_ms_lease_duration(mut self, x_ms_lease_duration: i64) -> Self {
                self.x_ms_lease_duration = Some(x_ms_lease_duration);
                self
            }
            #[doc = "Proposed lease ID, in a GUID string format. The File service returns 400 (Invalid request) if the proposed lease ID is not in the correct format. See Guid Constructor (String) for a list of valid GUID string formats."]
            pub fn x_ms_proposed_lease_id(mut self, x_ms_proposed_lease_id: impl Into<String>) -> Self {
                self.x_ms_proposed_lease_id = Some(x_ms_proposed_lease_id.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}/{}?comp=lease&acquire",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_duration) = &this.x_ms_lease_duration {
                            req.insert_header("x-ms-lease-duration", &x_ms_lease_duration.to_string());
                        }
                        if let Some(x_ms_proposed_lease_id) = &this.x_ms_proposed_lease_id {
                            req.insert_header("x-ms-proposed-lease-id", x_ms_proposed_lease_id);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod release_lease {
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
            #[doc = "The ETag contains a value that you can use to perform operations conditionally. If the request version is 2011-08-18 or newer, the ETag value will be in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the file was last modified. Any operation that modifies the file, including an update of the file's metadata or properties, changes the last-modified time of the file."]
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
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) x_ms_lease_id: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}/{}?comp=lease&release",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-lease-id", &this.x_ms_lease_id);
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod change_lease {
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
            #[doc = "The ETag contains a value that you can use to perform operations conditionally. If the request version is 2011-08-18 or newer, the ETag value will be in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the file was last modified. Any operation that modifies the file, including an update of the file's metadata or properties, changes the last-modified time of the file."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "Uniquely identifies a file's lease"]
            pub fn x_ms_lease_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-id"))
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
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) x_ms_lease_id: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_proposed_lease_id: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Proposed lease ID, in a GUID string format. The File service returns 400 (Invalid request) if the proposed lease ID is not in the correct format. See Guid Constructor (String) for a list of valid GUID string formats."]
            pub fn x_ms_proposed_lease_id(mut self, x_ms_proposed_lease_id: impl Into<String>) -> Self {
                self.x_ms_proposed_lease_id = Some(x_ms_proposed_lease_id.into());
                self
            }
            #[doc = "Provides a client-generated, opaque value with a 1 KB character limit that is recorded in the analytics logs when storage analytics logging is enabled."]
            pub fn x_ms_client_request_id(mut self, x_ms_client_request_id: impl Into<String>) -> Self {
                self.x_ms_client_request_id = Some(x_ms_client_request_id.into());
                self
            }
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}/{}?comp=lease&change",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-lease-id", &this.x_ms_lease_id);
                        if let Some(x_ms_proposed_lease_id) = &this.x_ms_proposed_lease_id {
                            req.insert_header("x-ms-proposed-lease-id", x_ms_proposed_lease_id);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod break_lease {
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
            #[doc = "The ETag contains a value that you can use to perform operations conditionally. If the request version is 2011-08-18 or newer, the ETag value will be in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the file was last modified. Any operation that modifies the file, including an update of the file's metadata or properties, changes the last-modified time of the file."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "Uniquely identifies a file's lease"]
            pub fn x_ms_lease_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-lease-id"))
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
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_lease_action: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_client_request_id: Option<String>,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
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
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}/{}?comp=lease&break",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        req.insert_header("x-ms-lease-action", &this.x_ms_lease_action);
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_client_request_id) = &this.x_ms_client_request_id {
                            req.insert_header("x-ms-client-request-id", x_ms_client_request_id);
                        }
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod upload_range {
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
            #[doc = "The ETag contains a value which represents the version of the file, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the directory was last modified. Any operation that modifies the share or its properties or metadata updates the last modified time. Operations on files do not affect the last modified time of the share."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "This header is returned so that the client can check for message content integrity. The value of this header is computed by the File service; it is not necessarily the same value as may have been specified in the request headers."]
            pub fn content_md5(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-md5"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "The value of this header is set to true if the contents of the request are successfully encrypted using the specified algorithm, and false otherwise."]
            pub fn x_ms_request_server_encrypted(&self) -> azure_core::Result<bool> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-request-server-encrypted"))
            }
            #[doc = "Last write time for the file."]
            pub fn x_ms_file_last_write_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-last-write-time"))?,
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_range: String,
            pub(crate) x_ms_write: String,
            pub(crate) content_length: i64,
            pub(crate) optionalbody: Option<serde_json::Value>,
            pub(crate) timeout: Option<i64>,
            pub(crate) content_md5: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_file_last_write_time: Option<String>,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "Initial data."]
            pub fn optionalbody(mut self, optionalbody: impl Into<serde_json::Value>) -> Self {
                self.optionalbody = Some(optionalbody.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "An MD5 hash of the content. This hash is used to verify the integrity of the data during transport. When the Content-MD5 header is specified, the File service compares the hash of the content that has arrived with the header value that was sent. If the two hashes do not match, the operation will fail with error code 400 (Bad Request)."]
            pub fn content_md5(mut self, content_md5: impl Into<String>) -> Self {
                self.content_md5 = Some(content_md5.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "If the file last write time should be preserved or overwritten"]
            pub fn x_ms_file_last_write_time(mut self, x_ms_file_last_write_time: impl Into<String>) -> Self {
                self.x_ms_file_last_write_time = Some(x_ms_file_last_write_time.into());
                self
            }
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}/{}?comp=range",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        let req_body = if let Some(optionalbody) = &this.optionalbody {
                            req.insert_header("content-type", "application/octet-stream");
                            azure_core::to_json(optionalbody)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-range", &this.x_ms_range);
                        req.insert_header("x-ms-write", &this.x_ms_write);
                        req.insert_header("content-length", &this.content_length.to_string());
                        if let Some(content_md5) = &this.content_md5 {
                            req.insert_header("content-md5", content_md5);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_file_last_write_time) = &this.x_ms_file_last_write_time {
                            req.insert_header("x-ms-file-last-write-time", x_ms_file_last_write_time);
                        }
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
                        }
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod upload_range_from_url {
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
            #[doc = "The ETag contains a value which represents the version of the file, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the directory was last modified. Any operation that modifies the share or its properties or metadata updates the last modified time. Operations on files do not affect the last modified time of the share."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "This header is returned so that the client can check for message content integrity. The value of this header is computed by the File service; it is not necessarily the same value as may have been specified in the request headers."]
            pub fn x_ms_content_crc64(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-content-crc64"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "The value of this header is set to true if the contents of the request are successfully encrypted using the specified algorithm, and false otherwise."]
            pub fn x_ms_request_server_encrypted(&self) -> azure_core::Result<bool> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-request-server-encrypted"))
            }
            #[doc = "Last write time for the file."]
            pub fn x_ms_file_last_write_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-last-write-time"))?,
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_range: String,
            pub(crate) x_ms_copy_source: String,
            pub(crate) x_ms_write: String,
            pub(crate) content_length: i64,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_source_range: Option<String>,
            pub(crate) x_ms_source_content_crc64: Option<String>,
            pub(crate) x_ms_source_if_match_crc64: Option<String>,
            pub(crate) x_ms_source_if_none_match_crc64: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_copy_source_authorization: Option<String>,
            pub(crate) x_ms_file_last_write_time: Option<String>,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) x_ms_source_allow_trailing_dot: Option<bool>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Bytes of source data in the specified range."]
            pub fn x_ms_source_range(mut self, x_ms_source_range: impl Into<String>) -> Self {
                self.x_ms_source_range = Some(x_ms_source_range.into());
                self
            }
            #[doc = "Specify the crc64 calculated for the range of bytes that must be read from the copy source."]
            pub fn x_ms_source_content_crc64(mut self, x_ms_source_content_crc64: impl Into<String>) -> Self {
                self.x_ms_source_content_crc64 = Some(x_ms_source_content_crc64.into());
                self
            }
            #[doc = "Specify the crc64 value to operate only on range with a matching crc64 checksum."]
            pub fn x_ms_source_if_match_crc64(mut self, x_ms_source_if_match_crc64: impl Into<String>) -> Self {
                self.x_ms_source_if_match_crc64 = Some(x_ms_source_if_match_crc64.into());
                self
            }
            #[doc = "Specify the crc64 value to operate only on range without a matching crc64 checksum."]
            pub fn x_ms_source_if_none_match_crc64(mut self, x_ms_source_if_none_match_crc64: impl Into<String>) -> Self {
                self.x_ms_source_if_none_match_crc64 = Some(x_ms_source_if_none_match_crc64.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "Only Bearer type is supported. Credentials should be a valid OAuth access token to copy source."]
            pub fn x_ms_copy_source_authorization(mut self, x_ms_copy_source_authorization: impl Into<String>) -> Self {
                self.x_ms_copy_source_authorization = Some(x_ms_copy_source_authorization.into());
                self
            }
            #[doc = "If the file last write time should be preserved or overwritten"]
            pub fn x_ms_file_last_write_time(mut self, x_ms_file_last_write_time: impl Into<String>) -> Self {
                self.x_ms_file_last_write_time = Some(x_ms_file_last_write_time.into());
                self
            }
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "If true, the trailing dot will not be trimmed from the source URI."]
            pub fn x_ms_source_allow_trailing_dot(mut self, x_ms_source_allow_trailing_dot: bool) -> Self {
                self.x_ms_source_allow_trailing_dot = Some(x_ms_source_allow_trailing_dot);
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
                            "{}/{}/{}/{}?comp=range&fromURL",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-range", &this.x_ms_range);
                        req.insert_header("x-ms-copy-source", &this.x_ms_copy_source);
                        if let Some(x_ms_source_range) = &this.x_ms_source_range {
                            req.insert_header("x-ms-source-range", x_ms_source_range);
                        }
                        req.insert_header("x-ms-write", &this.x_ms_write);
                        req.insert_header("content-length", &this.content_length.to_string());
                        if let Some(x_ms_source_content_crc64) = &this.x_ms_source_content_crc64 {
                            req.insert_header("x-ms-source-content-crc64", x_ms_source_content_crc64);
                        }
                        if let Some(x_ms_source_if_match_crc64) = &this.x_ms_source_if_match_crc64 {
                            req.insert_header("x-ms-source-if-match-crc64", x_ms_source_if_match_crc64);
                        }
                        if let Some(x_ms_source_if_none_match_crc64) = &this.x_ms_source_if_none_match_crc64 {
                            req.insert_header("x-ms-source-if-none-match-crc64", x_ms_source_if_none_match_crc64);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_copy_source_authorization) = &this.x_ms_copy_source_authorization {
                            req.insert_header("x-ms-copy-source-authorization", x_ms_copy_source_authorization);
                        }
                        if let Some(x_ms_file_last_write_time) = &this.x_ms_file_last_write_time {
                            req.insert_header("x-ms-file-last-write-time", x_ms_file_last_write_time);
                        }
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(x_ms_source_allow_trailing_dot) = &this.x_ms_source_allow_trailing_dot {
                            req.insert_header("x-ms-source-allow-trailing-dot", &x_ms_source_allow_trailing_dot.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod get_range_list {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ShareFileRangeList> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ShareFileRangeList = azure_core::xml::read_xml(&bytes)?;
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
            #[doc = "The date/time that the file was last modified. Any operation that modifies the file, including an update of the file's metadata or properties, changes the file's last modified time."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "The ETag contains a value which represents the version of the file, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "The size of the file in bytes."]
            pub fn x_ms_content_length(&self) -> azure_core::Result<i64> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("x-ms-content-length"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) prevsharesnapshot: Option<String>,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_range: Option<String>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            #[doc = "The previous snapshot parameter is an opaque DateTime value that, when present, specifies the previous snapshot."]
            pub fn prevsharesnapshot(mut self, prevsharesnapshot: impl Into<String>) -> Self {
                self.prevsharesnapshot = Some(prevsharesnapshot.into());
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Specifies the range of bytes over which to list ranges, inclusively."]
            pub fn x_ms_range(mut self, x_ms_range: impl Into<String>) -> Self {
                self.x_ms_range = Some(x_ms_range.into());
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}/{}?comp=rangelist",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                        }
                        if let Some(prevsharesnapshot) = &this.prevsharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("prevsharesnapshot", prevsharesnapshot);
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_range) = &this.x_ms_range {
                            req.insert_header("x-ms-range", x_ms_range);
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ShareFileRangeList>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ShareFileRangeList>>;
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
    pub mod start_copy {
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
            #[doc = "If the copy is completed, contains the ETag of the destination file. If the copy is not complete, contains the ETag of the empty file created at the start of the copy."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date/time that the copy operation to the destination file completed."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "String identifier for this copy operation. Use with Get File or Get File Properties to check the status of this copy operation, or pass to Abort Copy File to abort a pending copy."]
            pub fn x_ms_copy_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-id"))
            }
            #[doc = "State of the copy operation identified by x-ms-copy-id."]
            pub fn x_ms_copy_status(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-copy-status"))
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_copy_source: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_file_permission: Option<String>,
            pub(crate) x_ms_file_permission_key: Option<String>,
            pub(crate) x_ms_file_permission_copy_mode: Option<String>,
            pub(crate) x_ms_file_copy_ignore_readonly: Option<bool>,
            pub(crate) x_ms_file_attributes: Option<String>,
            pub(crate) x_ms_file_creation_time: Option<String>,
            pub(crate) x_ms_file_last_write_time: Option<String>,
            pub(crate) x_ms_file_change_time: Option<String>,
            pub(crate) x_ms_file_copy_set_archive: Option<bool>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) x_ms_source_allow_trailing_dot: Option<bool>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "A name-value pair to associate with a file storage object."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "If specified the permission (security descriptor) shall be set for the directory/file. This header can be used if Permission size is <= 8KB, else x-ms-file-permission-key header shall be used. Default value: Inherit. If SDDL is specified as input, it must have owner, group and dacl. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission(mut self, x_ms_file_permission: impl Into<String>) -> Self {
                self.x_ms_file_permission = Some(x_ms_file_permission.into());
                self
            }
            #[doc = "Key of the permission to be set for the directory/file. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission_key(mut self, x_ms_file_permission_key: impl Into<String>) -> Self {
                self.x_ms_file_permission_key = Some(x_ms_file_permission_key.into());
                self
            }
            #[doc = "Specifies the option to copy file security descriptor from source file or to set it using the value which is defined by the header value of x-ms-file-permission or x-ms-file-permission-key."]
            pub fn x_ms_file_permission_copy_mode(mut self, x_ms_file_permission_copy_mode: impl Into<String>) -> Self {
                self.x_ms_file_permission_copy_mode = Some(x_ms_file_permission_copy_mode.into());
                self
            }
            #[doc = "Specifies the option to overwrite the target file if it already exists and has read-only attribute set."]
            pub fn x_ms_file_copy_ignore_readonly(mut self, x_ms_file_copy_ignore_readonly: bool) -> Self {
                self.x_ms_file_copy_ignore_readonly = Some(x_ms_file_copy_ignore_readonly);
                self
            }
            #[doc = "Specifies either the option to copy file attributes from a source file(source) to a target file or a list of attributes to set on a target file."]
            pub fn x_ms_file_attributes(mut self, x_ms_file_attributes: impl Into<String>) -> Self {
                self.x_ms_file_attributes = Some(x_ms_file_attributes.into());
                self
            }
            #[doc = "Specifies either the option to copy file creation time from a source file(source) to a target file or a time value in ISO 8601 format to set as creation time on a target file."]
            pub fn x_ms_file_creation_time(mut self, x_ms_file_creation_time: impl Into<String>) -> Self {
                self.x_ms_file_creation_time = Some(x_ms_file_creation_time.into());
                self
            }
            #[doc = "Specifies either the option to copy file last write time from a source file(source) to a target file or a time value in ISO 8601 format to set as last write time on a target file."]
            pub fn x_ms_file_last_write_time(mut self, x_ms_file_last_write_time: impl Into<String>) -> Self {
                self.x_ms_file_last_write_time = Some(x_ms_file_last_write_time.into());
                self
            }
            #[doc = "Specifies either the option to copy file last write time from a source file(source) to a target file or a time value in ISO 8601 format to set as last write time on a target file."]
            pub fn x_ms_file_change_time(mut self, x_ms_file_change_time: impl Into<String>) -> Self {
                self.x_ms_file_change_time = Some(x_ms_file_change_time.into());
                self
            }
            #[doc = "Specifies the option to set archive attribute on a target file. True means archive attribute will be set on a target file despite attribute overrides or a source file state."]
            pub fn x_ms_file_copy_set_archive(mut self, x_ms_file_copy_set_archive: bool) -> Self {
                self.x_ms_file_copy_set_archive = Some(x_ms_file_copy_set_archive);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "If true, the trailing dot will not be trimmed from the source URI."]
            pub fn x_ms_source_allow_trailing_dot(mut self, x_ms_source_allow_trailing_dot: bool) -> Self {
                self.x_ms_source_allow_trailing_dot = Some(x_ms_source_allow_trailing_dot);
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}/{}?comp=copy",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        req.insert_header("x-ms-copy-source", &this.x_ms_copy_source);
                        if let Some(x_ms_file_permission) = &this.x_ms_file_permission {
                            req.insert_header("x-ms-file-permission", x_ms_file_permission);
                        }
                        if let Some(x_ms_file_permission_key) = &this.x_ms_file_permission_key {
                            req.insert_header("x-ms-file-permission-key", x_ms_file_permission_key);
                        }
                        if let Some(x_ms_file_permission_copy_mode) = &this.x_ms_file_permission_copy_mode {
                            req.insert_header("x-ms-file-permission-copy-mode", x_ms_file_permission_copy_mode);
                        }
                        if let Some(x_ms_file_copy_ignore_readonly) = &this.x_ms_file_copy_ignore_readonly {
                            req.insert_header("x-ms-file-copy-ignore-readonly", &x_ms_file_copy_ignore_readonly.to_string());
                        }
                        if let Some(x_ms_file_attributes) = &this.x_ms_file_attributes {
                            req.insert_header("x-ms-file-attributes", x_ms_file_attributes);
                        }
                        if let Some(x_ms_file_creation_time) = &this.x_ms_file_creation_time {
                            req.insert_header("x-ms-file-creation-time", x_ms_file_creation_time);
                        }
                        if let Some(x_ms_file_last_write_time) = &this.x_ms_file_last_write_time {
                            req.insert_header("x-ms-file-last-write-time", x_ms_file_last_write_time);
                        }
                        if let Some(x_ms_file_change_time) = &this.x_ms_file_change_time {
                            req.insert_header("x-ms-file-change-time", x_ms_file_change_time);
                        }
                        if let Some(x_ms_file_copy_set_archive) = &this.x_ms_file_copy_set_archive {
                            req.insert_header("x-ms-file-copy-set-archive", &x_ms_file_copy_set_archive.to_string());
                        }
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(x_ms_source_allow_trailing_dot) = &this.x_ms_source_allow_trailing_dot {
                            req.insert_header("x-ms-source-allow-trailing-dot", &x_ms_source_allow_trailing_dot.to_string());
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod abort_copy {
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
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_copy_action: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_lease_id: Option<String>,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "If specified, the operation only succeeds if the resource's lease is active and matches this ID."]
            pub fn x_ms_lease_id(mut self, x_ms_lease_id: impl Into<String>) -> Self {
                self.x_ms_lease_id = Some(x_ms_lease_id.into());
                self
            }
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}/{}?comp=copy&copyid",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-copy-action", &this.x_ms_copy_action);
                        if let Some(x_ms_lease_id) = &this.x_ms_lease_id {
                            req.insert_header("x-ms-lease-id", x_ms_lease_id);
                        }
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod list_handles {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ListHandlesResponse> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ListHandlesResponse = azure_core::xml::read_xml(&bytes)?;
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
            #[doc = "Specifies the format in which the results are returned. Currently this value is 'application/xml'."]
            pub fn content_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-type"))
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) marker: Option<String>,
            pub(crate) maxresults: Option<i64>,
            pub(crate) timeout: Option<i64>,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "A string value that identifies the portion of the list to be returned with the next list operation. The operation returns a marker value within the response body if the list returned was not complete. The marker value may then be used in a subsequent call to request the next set of list items. The marker value is opaque to the client."]
            pub fn marker(mut self, marker: impl Into<String>) -> Self {
                self.marker = Some(marker.into());
                self
            }
            #[doc = "Specifies the maximum number of entries to return. If the request does not specify maxresults, or specifies a value greater than 5,000, the server will return up to 5,000 items."]
            pub fn maxresults(mut self, maxresults: i64) -> Self {
                self.maxresults = Some(maxresults);
                self
            }
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}/{}?comp=listhandles",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(marker) = &this.marker {
                            req.url_mut().query_pairs_mut().append_pair("marker", marker);
                        }
                        if let Some(maxresults) = &this.maxresults {
                            req.url_mut().query_pairs_mut().append_pair("maxresults", &maxresults.to_string());
                        }
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                        }
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ListHandlesResponse>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ListHandlesResponse>>;
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
    pub mod force_close_handles {
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
            #[doc = "Indicates the version of the File service used to execute the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "A string describing next handle to be closed. It is returned when more handles need to be closed to complete the request."]
            pub fn x_ms_marker(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-marker"))
            }
            #[doc = "Contains count of number of handles closed."]
            pub fn x_ms_number_of_handles_closed(&self) -> azure_core::Result<i32> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-number-of-handles-closed"))
            }
            #[doc = "Contains count of number of handles that failed to close."]
            pub fn x_ms_number_of_handles_failed(&self) -> azure_core::Result<i32> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-number-of-handles-failed"))
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_handle_id: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) marker: Option<String>,
            pub(crate) sharesnapshot: Option<String>,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "A string value that identifies the portion of the list to be returned with the next list operation. The operation returns a marker value within the response body if the list returned was not complete. The marker value may then be used in a subsequent call to request the next set of list items. The marker value is opaque to the client."]
            pub fn marker(mut self, marker: impl Into<String>) -> Self {
                self.marker = Some(marker.into());
                self
            }
            #[doc = "The snapshot parameter is an opaque DateTime value that, when present, specifies the share snapshot to query."]
            pub fn sharesnapshot(mut self, sharesnapshot: impl Into<String>) -> Self {
                self.sharesnapshot = Some(sharesnapshot.into());
                self
            }
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}/{}?comp=forceclosehandles",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        if let Some(marker) = &this.marker {
                            req.url_mut().query_pairs_mut().append_pair("marker", marker);
                        }
                        if let Some(sharesnapshot) = &this.sharesnapshot {
                            req.url_mut().query_pairs_mut().append_pair("sharesnapshot", sharesnapshot);
                        }
                        req.insert_header("x-ms-handle-id", &this.x_ms_handle_id);
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
        }
    }
    pub mod rename {
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
            #[doc = "The ETag contains a value which represents the version of the file, in quotes."]
            pub fn e_tag(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
            }
            #[doc = "Returns the date and time the share was last modified. Any operation that modifies the directory or its properties updates the last modified time. Operations on files do not affect the last modified time of the directory."]
            pub fn last_modified(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))?)
            }
            #[doc = "This header uniquely identifies the request that was made and can be used for troubleshooting the request."]
            pub fn x_ms_request_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-request-id"))
            }
            #[doc = "Indicates the version of the File service used to execute the request."]
            pub fn x_ms_version(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-version"))
            }
            #[doc = "A UTC date/time value generated by the service that indicates the time at which the response was initiated."]
            pub fn date(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(self.0.get_str(&azure_core::headers::HeaderName::from_static("date"))?)
            }
            #[doc = "The value of this header is set to true if the contents of the request are successfully encrypted using the specified algorithm, and false otherwise."]
            pub fn x_ms_request_server_encrypted(&self) -> azure_core::Result<bool> {
                self.0
                    .get_as(&azure_core::headers::HeaderName::from_static("x-ms-request-server-encrypted"))
            }
            #[doc = "Key of the permission set for the file."]
            pub fn x_ms_file_permission_key(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-permission-key"))
            }
            #[doc = "Attributes set for the file."]
            pub fn x_ms_file_attributes(&self) -> azure_core::Result<&str> {
                self.0
                    .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-attributes"))
            }
            #[doc = "Creation time for the file."]
            pub fn x_ms_file_creation_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-creation-time"))?,
                )
            }
            #[doc = "Last write time for the file."]
            pub fn x_ms_file_last_write_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-last-write-time"))?,
                )
            }
            #[doc = "Change time for the file."]
            pub fn x_ms_file_change_time(&self) -> azure_core::Result<time::OffsetDateTime> {
                azure_core::date::parse_rfc1123(
                    self.0
                        .get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-change-time"))?,
                )
            }
            #[doc = "The fileId of the file."]
            pub fn x_ms_file_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-id"))
            }
            #[doc = "The parent fileId of the directory."]
            pub fn x_ms_file_parent_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("x-ms-file-parent-id"))
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
            pub(crate) share_name: String,
            pub(crate) directory: String,
            pub(crate) file_name: String,
            pub(crate) x_ms_file_rename_source: String,
            pub(crate) timeout: Option<i64>,
            pub(crate) x_ms_file_rename_replace_if_exists: Option<bool>,
            pub(crate) x_ms_file_rename_ignore_readonly: Option<bool>,
            pub(crate) x_ms_source_lease_id: Option<String>,
            pub(crate) x_ms_destination_lease_id: Option<String>,
            pub(crate) x_ms_file_attributes: Option<String>,
            pub(crate) x_ms_file_creation_time: Option<String>,
            pub(crate) x_ms_file_last_write_time: Option<String>,
            pub(crate) x_ms_file_change_time: Option<String>,
            pub(crate) x_ms_file_permission: Option<String>,
            pub(crate) x_ms_file_permission_key: Option<String>,
            pub(crate) x_ms_meta: Option<String>,
            pub(crate) x_ms_content_type: Option<String>,
            pub(crate) x_ms_allow_trailing_dot: Option<bool>,
            pub(crate) x_ms_source_allow_trailing_dot: Option<bool>,
            pub(crate) x_ms_file_request_intent: Option<String>,
        }
        impl RequestBuilder {
            #[doc = "The timeout parameter is expressed in seconds. For more information, see <a href=\"https://docs.microsoft.com/en-us/rest/api/storageservices/Setting-Timeouts-for-File-Service-Operations?redirectedfrom=MSDN\">Setting Timeouts for File Service Operations.</a>"]
            pub fn timeout(mut self, timeout: i64) -> Self {
                self.timeout = Some(timeout);
                self
            }
            #[doc = "Optional. A boolean value for if the destination file already exists, whether this request will overwrite the file or not. If true, the rename will succeed and will overwrite the destination file. If not provided or if false and the destination file does exist, the request will not overwrite the destination file. If provided and the destination file doesn’t exist, the rename will succeed. Note: This value does not override the x-ms-file-copy-ignore-read-only header value."]
            pub fn x_ms_file_rename_replace_if_exists(mut self, x_ms_file_rename_replace_if_exists: bool) -> Self {
                self.x_ms_file_rename_replace_if_exists = Some(x_ms_file_rename_replace_if_exists);
                self
            }
            #[doc = "Optional. A boolean value that specifies whether the ReadOnly attribute on a preexisting destination file should be respected. If true, the rename will succeed, otherwise, a previous file at the destination with the ReadOnly attribute set will cause the rename to fail."]
            pub fn x_ms_file_rename_ignore_readonly(mut self, x_ms_file_rename_ignore_readonly: bool) -> Self {
                self.x_ms_file_rename_ignore_readonly = Some(x_ms_file_rename_ignore_readonly);
                self
            }
            #[doc = "Required if the source file has an active infinite lease."]
            pub fn x_ms_source_lease_id(mut self, x_ms_source_lease_id: impl Into<String>) -> Self {
                self.x_ms_source_lease_id = Some(x_ms_source_lease_id.into());
                self
            }
            #[doc = "Required if the destination file has an active infinite lease. The lease ID specified for this header must match the lease ID of the destination file. If the request does not include the lease ID or it is not valid, the operation fails with status code 412 (Precondition Failed). If this header is specified and the destination file does not currently have an active lease, the operation will also fail with status code 412 (Precondition Failed)."]
            pub fn x_ms_destination_lease_id(mut self, x_ms_destination_lease_id: impl Into<String>) -> Self {
                self.x_ms_destination_lease_id = Some(x_ms_destination_lease_id.into());
                self
            }
            #[doc = "Specifies either the option to copy file attributes from a source file(source) to a target file or a list of attributes to set on a target file."]
            pub fn x_ms_file_attributes(mut self, x_ms_file_attributes: impl Into<String>) -> Self {
                self.x_ms_file_attributes = Some(x_ms_file_attributes.into());
                self
            }
            #[doc = "Specifies either the option to copy file creation time from a source file(source) to a target file or a time value in ISO 8601 format to set as creation time on a target file."]
            pub fn x_ms_file_creation_time(mut self, x_ms_file_creation_time: impl Into<String>) -> Self {
                self.x_ms_file_creation_time = Some(x_ms_file_creation_time.into());
                self
            }
            #[doc = "Specifies either the option to copy file last write time from a source file(source) to a target file or a time value in ISO 8601 format to set as last write time on a target file."]
            pub fn x_ms_file_last_write_time(mut self, x_ms_file_last_write_time: impl Into<String>) -> Self {
                self.x_ms_file_last_write_time = Some(x_ms_file_last_write_time.into());
                self
            }
            #[doc = "Specifies either the option to copy file last write time from a source file(source) to a target file or a time value in ISO 8601 format to set as last write time on a target file."]
            pub fn x_ms_file_change_time(mut self, x_ms_file_change_time: impl Into<String>) -> Self {
                self.x_ms_file_change_time = Some(x_ms_file_change_time.into());
                self
            }
            #[doc = "If specified the permission (security descriptor) shall be set for the directory/file. This header can be used if Permission size is <= 8KB, else x-ms-file-permission-key header shall be used. Default value: Inherit. If SDDL is specified as input, it must have owner, group and dacl. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission(mut self, x_ms_file_permission: impl Into<String>) -> Self {
                self.x_ms_file_permission = Some(x_ms_file_permission.into());
                self
            }
            #[doc = "Key of the permission to be set for the directory/file. Note: Only one of the x-ms-file-permission or x-ms-file-permission-key should be specified."]
            pub fn x_ms_file_permission_key(mut self, x_ms_file_permission_key: impl Into<String>) -> Self {
                self.x_ms_file_permission_key = Some(x_ms_file_permission_key.into());
                self
            }
            #[doc = "A name-value pair to associate with a file storage object."]
            pub fn x_ms_meta(mut self, x_ms_meta: impl Into<String>) -> Self {
                self.x_ms_meta = Some(x_ms_meta.into());
                self
            }
            #[doc = "Sets the MIME content type of the file. The default type is 'application/octet-stream'."]
            pub fn x_ms_content_type(mut self, x_ms_content_type: impl Into<String>) -> Self {
                self.x_ms_content_type = Some(x_ms_content_type.into());
                self
            }
            #[doc = "If true, the trailing dot will not be trimmed from the target URI."]
            pub fn x_ms_allow_trailing_dot(mut self, x_ms_allow_trailing_dot: bool) -> Self {
                self.x_ms_allow_trailing_dot = Some(x_ms_allow_trailing_dot);
                self
            }
            #[doc = "If true, the trailing dot will not be trimmed from the source URI."]
            pub fn x_ms_source_allow_trailing_dot(mut self, x_ms_source_allow_trailing_dot: bool) -> Self {
                self.x_ms_source_allow_trailing_dot = Some(x_ms_source_allow_trailing_dot);
                self
            }
            #[doc = "Valid value is backup"]
            pub fn x_ms_file_request_intent(mut self, x_ms_file_request_intent: impl Into<String>) -> Self {
                self.x_ms_file_request_intent = Some(x_ms_file_request_intent.into());
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
                            "{}/{}/{}/{}?comp=rename",
                            this.client.endpoint(),
                            &this.share_name,
                            &this.directory,
                            &this.file_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.insert_header(azure_core::headers::VERSION, "2023-08-03");
                        if let Some(timeout) = &this.timeout {
                            req.url_mut().query_pairs_mut().append_pair("timeout", &timeout.to_string());
                        }
                        req.insert_header("x-ms-file-rename-source", &this.x_ms_file_rename_source);
                        if let Some(x_ms_file_rename_replace_if_exists) = &this.x_ms_file_rename_replace_if_exists {
                            req.insert_header(
                                "x-ms-file-rename-replace-if-exists",
                                &x_ms_file_rename_replace_if_exists.to_string(),
                            );
                        }
                        if let Some(x_ms_file_rename_ignore_readonly) = &this.x_ms_file_rename_ignore_readonly {
                            req.insert_header("x-ms-file-rename-ignore-readonly", &x_ms_file_rename_ignore_readonly.to_string());
                        }
                        if let Some(x_ms_source_lease_id) = &this.x_ms_source_lease_id {
                            req.insert_header("x-ms-source-lease-id", x_ms_source_lease_id);
                        }
                        if let Some(x_ms_destination_lease_id) = &this.x_ms_destination_lease_id {
                            req.insert_header("x-ms-destination-lease-id", x_ms_destination_lease_id);
                        }
                        if let Some(x_ms_file_attributes) = &this.x_ms_file_attributes {
                            req.insert_header("x-ms-file-attributes", x_ms_file_attributes);
                        }
                        if let Some(x_ms_file_creation_time) = &this.x_ms_file_creation_time {
                            req.insert_header("x-ms-file-creation-time", x_ms_file_creation_time);
                        }
                        if let Some(x_ms_file_last_write_time) = &this.x_ms_file_last_write_time {
                            req.insert_header("x-ms-file-last-write-time", x_ms_file_last_write_time);
                        }
                        if let Some(x_ms_file_change_time) = &this.x_ms_file_change_time {
                            req.insert_header("x-ms-file-change-time", x_ms_file_change_time);
                        }
                        if let Some(x_ms_file_permission) = &this.x_ms_file_permission {
                            req.insert_header("x-ms-file-permission", x_ms_file_permission);
                        }
                        if let Some(x_ms_file_permission_key) = &this.x_ms_file_permission_key {
                            req.insert_header("x-ms-file-permission-key", x_ms_file_permission_key);
                        }
                        if let Some(x_ms_meta) = &this.x_ms_meta {
                            req.insert_header("x-ms-meta", x_ms_meta);
                        }
                        if let Some(x_ms_content_type) = &this.x_ms_content_type {
                            req.insert_header("x-ms-content-type", x_ms_content_type);
                        }
                        if let Some(x_ms_allow_trailing_dot) = &this.x_ms_allow_trailing_dot {
                            req.insert_header("x-ms-allow-trailing-dot", &x_ms_allow_trailing_dot.to_string());
                        }
                        if let Some(x_ms_source_allow_trailing_dot) = &this.x_ms_source_allow_trailing_dot {
                            req.insert_header("x-ms-source-allow-trailing-dot", &x_ms_source_allow_trailing_dot.to_string());
                        }
                        if let Some(x_ms_file_request_intent) = &this.x_ms_file_request_intent {
                            req.insert_header("x-ms-file-request-intent", x_ms_file_request_intent);
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
