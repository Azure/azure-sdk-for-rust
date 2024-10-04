#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::redundant_clone)]
pub mod models;
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
pub use azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD as DEFAULT_ENDPOINT;
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
    pub fn new(
        endpoint: impl Into<azure_core::Url>,
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
}
impl Client {
    #[doc = "Gets a list of keys."]
    pub fn get_keys(&self) -> get_keys::RequestBuilder {
        get_keys::RequestBuilder {
            client: self.clone(),
            name: None,
            sync_token: None,
            after: None,
            accept_datetime: None,
        }
    }
    #[doc = "Requests the headers and status of the given resource."]
    pub fn check_keys(&self) -> check_keys::RequestBuilder {
        check_keys::RequestBuilder {
            client: self.clone(),
            name: None,
            sync_token: None,
            after: None,
            accept_datetime: None,
        }
    }
    #[doc = "Gets a list of key-values."]
    pub fn get_key_values(&self) -> get_key_values::RequestBuilder {
        get_key_values::RequestBuilder {
            client: self.clone(),
            key: None,
            label: None,
            sync_token: None,
            after: None,
            accept_datetime: None,
            select: Vec::new(),
        }
    }
    #[doc = "Requests the headers and status of the given resource."]
    pub fn check_key_values(&self) -> check_key_values::RequestBuilder {
        check_key_values::RequestBuilder {
            client: self.clone(),
            key: None,
            label: None,
            sync_token: None,
            after: None,
            accept_datetime: None,
            select: Vec::new(),
        }
    }
    #[doc = "Gets a single key-value."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `key`: The key of the key-value to retrieve."]
    pub fn get_key_value(&self, key: impl Into<String>) -> get_key_value::RequestBuilder {
        get_key_value::RequestBuilder {
            client: self.clone(),
            key: key.into(),
            label: None,
            sync_token: None,
            accept_datetime: None,
            if_match: None,
            if_none_match: None,
            select: Vec::new(),
        }
    }
    #[doc = "Creates a key-value."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `key`: The key of the key-value to create."]
    pub fn put_key_value(&self, key: impl Into<String>) -> put_key_value::RequestBuilder {
        put_key_value::RequestBuilder {
            client: self.clone(),
            key: key.into(),
            label: None,
            entity: None,
            sync_token: None,
            if_match: None,
            if_none_match: None,
        }
    }
    #[doc = "Deletes a key-value."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `key`: The key of the key-value to delete."]
    pub fn delete_key_value(&self, key: impl Into<String>) -> delete_key_value::RequestBuilder {
        delete_key_value::RequestBuilder {
            client: self.clone(),
            key: key.into(),
            label: None,
            sync_token: None,
            if_match: None,
        }
    }
    #[doc = "Requests the headers and status of the given resource."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `key`: The key of the key-value to retrieve."]
    pub fn check_key_value(&self, key: impl Into<String>) -> check_key_value::RequestBuilder {
        check_key_value::RequestBuilder {
            client: self.clone(),
            key: key.into(),
            label: None,
            sync_token: None,
            accept_datetime: None,
            if_match: None,
            if_none_match: None,
            select: Vec::new(),
        }
    }
    #[doc = "Gets a list of labels."]
    pub fn get_labels(&self) -> get_labels::RequestBuilder {
        get_labels::RequestBuilder {
            client: self.clone(),
            name: None,
            sync_token: None,
            after: None,
            accept_datetime: None,
            select: Vec::new(),
        }
    }
    #[doc = "Requests the headers and status of the given resource."]
    pub fn check_labels(&self) -> check_labels::RequestBuilder {
        check_labels::RequestBuilder {
            client: self.clone(),
            name: None,
            sync_token: None,
            after: None,
            accept_datetime: None,
            select: Vec::new(),
        }
    }
    #[doc = "Locks a key-value."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `key`: The key of the key-value to lock."]
    pub fn put_lock(&self, key: impl Into<String>) -> put_lock::RequestBuilder {
        put_lock::RequestBuilder {
            client: self.clone(),
            key: key.into(),
            label: None,
            sync_token: None,
            if_match: None,
            if_none_match: None,
        }
    }
    #[doc = "Unlocks a key-value."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `key`: The key of the key-value to unlock."]
    pub fn delete_lock(&self, key: impl Into<String>) -> delete_lock::RequestBuilder {
        delete_lock::RequestBuilder {
            client: self.clone(),
            key: key.into(),
            label: None,
            sync_token: None,
            if_match: None,
            if_none_match: None,
        }
    }
    #[doc = "Gets a list of key-value revisions."]
    pub fn get_revisions(&self) -> get_revisions::RequestBuilder {
        get_revisions::RequestBuilder {
            client: self.clone(),
            key: None,
            label: None,
            sync_token: None,
            after: None,
            accept_datetime: None,
            select: Vec::new(),
        }
    }
    #[doc = "Requests the headers and status of the given resource."]
    pub fn check_revisions(&self) -> check_revisions::RequestBuilder {
        check_revisions::RequestBuilder {
            client: self.clone(),
            key: None,
            label: None,
            sync_token: None,
            after: None,
            accept_datetime: None,
            select: Vec::new(),
        }
    }
}
pub mod get_keys {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    #[derive(Debug)]
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::KeyListResult> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::KeyListResult = serde_json::from_slice(&bytes)?;
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
        #[doc = "Enables real-time consistency between requests by providing the returned value in the next request made to the server."]
        pub fn sync_token(&self) -> azure_core::Result<&str> {
            self.0.get_str(&azure_core::headers::HeaderName::from_static("sync-token"))
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
        pub(crate) client: super::Client,
        pub(crate) name: Option<String>,
        pub(crate) sync_token: Option<String>,
        pub(crate) after: Option<String>,
        pub(crate) accept_datetime: Option<String>,
    }
    impl RequestBuilder {
        #[doc = "A filter for the name of the returned keys."]
        pub fn name(mut self, name: impl Into<String>) -> Self {
            self.name = Some(name.into());
            self
        }
        #[doc = "Used to guarantee real-time consistency between requests."]
        pub fn sync_token(mut self, sync_token: impl Into<String>) -> Self {
            self.sync_token = Some(sync_token.into());
            self
        }
        #[doc = "Instructs the server to return elements that appear after the element referred to by the specified token."]
        pub fn after(mut self, after: impl Into<String>) -> Self {
            self.after = Some(after.into());
            self
        }
        #[doc = "Requests the server to respond with the state of the resource at the specified time."]
        pub fn accept_datetime(mut self, accept_datetime: impl Into<String>) -> Self {
            self.accept_datetime = Some(accept_datetime.into());
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<models::KeyListResult, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = this.url()?;
                    let rsp = match continuation {
                        Some(value) => {
                            url.set_path("");
                            url = url.join(&value)?;
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let bearer_token = this.client.bearer_token().await?;
                            req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                            let has_api_version_already =
                                req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                            if !has_api_version_already {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "1.0");
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                        None => {
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let bearer_token = this.client.bearer_token().await?;
                            req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                            if let Some(name) = &this.name {
                                req.url_mut().query_pairs_mut().append_pair("name", name);
                            }
                            if let Some(sync_token) = &this.sync_token {
                                req.insert_header("sync-token", sync_token);
                            }
                            if let Some(after) = &this.after {
                                req.url_mut().query_pairs_mut().append_pair("After", after);
                            }
                            if let Some(accept_datetime) = &this.accept_datetime {
                                req.insert_header("accept-datetime", accept_datetime);
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
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let mut url = self.client.endpoint().clone();
            url.set_path(&format!("/keys",));
            let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
            if !has_api_version_already {
                url.query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, "1.0");
            }
            Ok(url)
        }
    }
}
pub mod check_keys {
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
        #[doc = "Enables real-time consistency between requests by providing the returned value in the next request made to the server."]
        pub fn sync_token(&self) -> azure_core::Result<&str> {
            self.0.get_str(&azure_core::headers::HeaderName::from_static("sync-token"))
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
        pub(crate) client: super::Client,
        pub(crate) name: Option<String>,
        pub(crate) sync_token: Option<String>,
        pub(crate) after: Option<String>,
        pub(crate) accept_datetime: Option<String>,
    }
    impl RequestBuilder {
        #[doc = "A filter for the name of the returned keys."]
        pub fn name(mut self, name: impl Into<String>) -> Self {
            self.name = Some(name.into());
            self
        }
        #[doc = "Used to guarantee real-time consistency between requests."]
        pub fn sync_token(mut self, sync_token: impl Into<String>) -> Self {
            self.sync_token = Some(sync_token.into());
            self
        }
        #[doc = "Instructs the server to return elements that appear after the element referred to by the specified token."]
        pub fn after(mut self, after: impl Into<String>) -> Self {
            self.after = Some(after.into());
            self
        }
        #[doc = "Requests the server to respond with the state of the resource at the specified time."]
        pub fn accept_datetime(mut self, accept_datetime: impl Into<String>) -> Self {
            self.accept_datetime = Some(accept_datetime.into());
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
                    let bearer_token = this.client.bearer_token().await?;
                    req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                    if let Some(name) = &this.name {
                        req.url_mut().query_pairs_mut().append_pair("name", name);
                    }
                    if let Some(sync_token) = &this.sync_token {
                        req.insert_header("sync-token", sync_token);
                    }
                    if let Some(after) = &this.after {
                        req.url_mut().query_pairs_mut().append_pair("After", after);
                    }
                    if let Some(accept_datetime) = &this.accept_datetime {
                        req.insert_header("accept-datetime", accept_datetime);
                    }
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let mut url = self.client.endpoint().clone();
            url.set_path(&format!("/keys",));
            let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
            if !has_api_version_already {
                url.query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, "1.0");
            }
            Ok(url)
        }
    }
}
pub mod get_key_values {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    #[derive(Debug)]
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::KeyValueListResult> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::KeyValueListResult = serde_json::from_slice(&bytes)?;
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
        #[doc = "Enables real-time consistency between requests by providing the returned value in the next request made to the server."]
        pub fn sync_token(&self) -> azure_core::Result<&str> {
            self.0.get_str(&azure_core::headers::HeaderName::from_static("sync-token"))
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
        pub(crate) client: super::Client,
        pub(crate) key: Option<String>,
        pub(crate) label: Option<String>,
        pub(crate) sync_token: Option<String>,
        pub(crate) after: Option<String>,
        pub(crate) accept_datetime: Option<String>,
        pub(crate) select: Vec<String>,
    }
    impl RequestBuilder {
        #[doc = "A filter used to match keys."]
        pub fn key(mut self, key: impl Into<String>) -> Self {
            self.key = Some(key.into());
            self
        }
        #[doc = "A filter used to match labels"]
        pub fn label(mut self, label: impl Into<String>) -> Self {
            self.label = Some(label.into());
            self
        }
        #[doc = "Used to guarantee real-time consistency between requests."]
        pub fn sync_token(mut self, sync_token: impl Into<String>) -> Self {
            self.sync_token = Some(sync_token.into());
            self
        }
        #[doc = "Instructs the server to return elements that appear after the element referred to by the specified token."]
        pub fn after(mut self, after: impl Into<String>) -> Self {
            self.after = Some(after.into());
            self
        }
        #[doc = "Requests the server to respond with the state of the resource at the specified time."]
        pub fn accept_datetime(mut self, accept_datetime: impl Into<String>) -> Self {
            self.accept_datetime = Some(accept_datetime.into());
            self
        }
        #[doc = "Used to select what fields are present in the returned resource(s)."]
        pub fn select(mut self, select: Vec<String>) -> Self {
            self.select = select;
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<models::KeyValueListResult, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = this.url()?;
                    let rsp = match continuation {
                        Some(value) => {
                            url.set_path("");
                            url = url.join(&value)?;
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let bearer_token = this.client.bearer_token().await?;
                            req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                            let has_api_version_already =
                                req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                            if !has_api_version_already {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "1.0");
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                        None => {
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let bearer_token = this.client.bearer_token().await?;
                            req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                            if let Some(key) = &this.key {
                                req.url_mut().query_pairs_mut().append_pair("key", key);
                            }
                            if let Some(label) = &this.label {
                                req.url_mut().query_pairs_mut().append_pair("label", label);
                            }
                            if let Some(sync_token) = &this.sync_token {
                                req.insert_header("sync-token", sync_token);
                            }
                            if let Some(after) = &this.after {
                                req.url_mut().query_pairs_mut().append_pair("After", after);
                            }
                            if let Some(accept_datetime) = &this.accept_datetime {
                                req.insert_header("accept-datetime", accept_datetime);
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
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let mut url = self.client.endpoint().clone();
            url.set_path(&format!("/kv",));
            let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
            if !has_api_version_already {
                url.query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, "1.0");
            }
            Ok(url)
        }
    }
}
pub mod check_key_values {
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
        #[doc = "Enables real-time consistency between requests by providing the returned value in the next request made to the server."]
        pub fn sync_token(&self) -> azure_core::Result<&str> {
            self.0.get_str(&azure_core::headers::HeaderName::from_static("sync-token"))
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
        pub(crate) client: super::Client,
        pub(crate) key: Option<String>,
        pub(crate) label: Option<String>,
        pub(crate) sync_token: Option<String>,
        pub(crate) after: Option<String>,
        pub(crate) accept_datetime: Option<String>,
        pub(crate) select: Vec<String>,
    }
    impl RequestBuilder {
        #[doc = "A filter used to match keys."]
        pub fn key(mut self, key: impl Into<String>) -> Self {
            self.key = Some(key.into());
            self
        }
        #[doc = "A filter used to match labels"]
        pub fn label(mut self, label: impl Into<String>) -> Self {
            self.label = Some(label.into());
            self
        }
        #[doc = "Used to guarantee real-time consistency between requests."]
        pub fn sync_token(mut self, sync_token: impl Into<String>) -> Self {
            self.sync_token = Some(sync_token.into());
            self
        }
        #[doc = "Instructs the server to return elements that appear after the element referred to by the specified token."]
        pub fn after(mut self, after: impl Into<String>) -> Self {
            self.after = Some(after.into());
            self
        }
        #[doc = "Requests the server to respond with the state of the resource at the specified time."]
        pub fn accept_datetime(mut self, accept_datetime: impl Into<String>) -> Self {
            self.accept_datetime = Some(accept_datetime.into());
            self
        }
        #[doc = "Used to select what fields are present in the returned resource(s)."]
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
                    let url = this.url()?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Head);
                    let bearer_token = this.client.bearer_token().await?;
                    req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                    if let Some(key) = &this.key {
                        req.url_mut().query_pairs_mut().append_pair("key", key);
                    }
                    if let Some(label) = &this.label {
                        req.url_mut().query_pairs_mut().append_pair("label", label);
                    }
                    if let Some(sync_token) = &this.sync_token {
                        req.insert_header("sync-token", sync_token);
                    }
                    if let Some(after) = &this.after {
                        req.url_mut().query_pairs_mut().append_pair("After", after);
                    }
                    if let Some(accept_datetime) = &this.accept_datetime {
                        req.insert_header("accept-datetime", accept_datetime);
                    }
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let mut url = self.client.endpoint().clone();
            url.set_path(&format!("/kv",));
            let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
            if !has_api_version_already {
                url.query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, "1.0");
            }
            Ok(url)
        }
    }
}
pub mod get_key_value {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    #[derive(Debug)]
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::KeyValue> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::KeyValue = serde_json::from_slice(&bytes)?;
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
        #[doc = "Enables real-time consistency between requests by providing the returned value in the next request made to the server."]
        pub fn sync_token(&self) -> azure_core::Result<&str> {
            self.0.get_str(&azure_core::headers::HeaderName::from_static("sync-token"))
        }
        #[doc = "An identifier representing the returned state of the resource."]
        pub fn e_tag(&self) -> azure_core::Result<&str> {
            self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
        }
        #[doc = "A UTC datetime that specifies the last time the resource was modified."]
        pub fn last_modified(&self) -> azure_core::Result<&str> {
            self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))
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
        pub(crate) client: super::Client,
        pub(crate) key: String,
        pub(crate) label: Option<String>,
        pub(crate) sync_token: Option<String>,
        pub(crate) accept_datetime: Option<String>,
        pub(crate) if_match: Option<String>,
        pub(crate) if_none_match: Option<String>,
        pub(crate) select: Vec<String>,
    }
    impl RequestBuilder {
        #[doc = "The label of the key-value to retrieve."]
        pub fn label(mut self, label: impl Into<String>) -> Self {
            self.label = Some(label.into());
            self
        }
        #[doc = "Used to guarantee real-time consistency between requests."]
        pub fn sync_token(mut self, sync_token: impl Into<String>) -> Self {
            self.sync_token = Some(sync_token.into());
            self
        }
        #[doc = "Requests the server to respond with the state of the resource at the specified time."]
        pub fn accept_datetime(mut self, accept_datetime: impl Into<String>) -> Self {
            self.accept_datetime = Some(accept_datetime.into());
            self
        }
        #[doc = "Used to perform an operation only if the targeted resource's etag matches the value provided."]
        pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
            self.if_match = Some(if_match.into());
            self
        }
        #[doc = "Used to perform an operation only if the targeted resource's etag does not match the value provided."]
        pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
            self.if_none_match = Some(if_none_match.into());
            self
        }
        #[doc = "Used to select what fields are present in the returned resource(s)."]
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
                    let url = this.url()?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let bearer_token = this.client.bearer_token().await?;
                    req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                    if let Some(label) = &this.label {
                        req.url_mut().query_pairs_mut().append_pair("label", label);
                    }
                    if let Some(sync_token) = &this.sync_token {
                        req.insert_header("sync-token", sync_token);
                    }
                    if let Some(accept_datetime) = &this.accept_datetime {
                        req.insert_header("accept-datetime", accept_datetime);
                    }
                    if let Some(if_match) = &this.if_match {
                        req.insert_header("if-match", if_match);
                    }
                    if let Some(if_none_match) = &this.if_none_match {
                        req.insert_header("if-none-match", if_none_match);
                    }
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let mut url = self.client.endpoint().clone();
            url.set_path(&format!("/kv/{}", &self.key));
            let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
            if !has_api_version_already {
                url.query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, "1.0");
            }
            Ok(url)
        }
    }
    impl std::future::IntoFuture for RequestBuilder {
        type Output = azure_core::Result<models::KeyValue>;
        type IntoFuture = BoxFuture<'static, azure_core::Result<models::KeyValue>>;
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
pub mod put_key_value {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    #[derive(Debug)]
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::KeyValue> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::KeyValue = serde_json::from_slice(&bytes)?;
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
        #[doc = "Enables real-time consistency between requests by providing the returned value in the next request made to the server."]
        pub fn sync_token(&self) -> azure_core::Result<&str> {
            self.0.get_str(&azure_core::headers::HeaderName::from_static("sync-token"))
        }
        #[doc = "An identifier representing the returned state of the resource."]
        pub fn e_tag(&self) -> azure_core::Result<&str> {
            self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
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
        pub(crate) client: super::Client,
        pub(crate) key: String,
        pub(crate) label: Option<String>,
        pub(crate) entity: Option<models::KeyValue>,
        pub(crate) sync_token: Option<String>,
        pub(crate) if_match: Option<String>,
        pub(crate) if_none_match: Option<String>,
    }
    impl RequestBuilder {
        #[doc = "The label of the key-value to create."]
        pub fn label(mut self, label: impl Into<String>) -> Self {
            self.label = Some(label.into());
            self
        }
        #[doc = "The key-value to create."]
        pub fn entity(mut self, entity: impl Into<models::KeyValue>) -> Self {
            self.entity = Some(entity.into());
            self
        }
        #[doc = "Used to guarantee real-time consistency between requests."]
        pub fn sync_token(mut self, sync_token: impl Into<String>) -> Self {
            self.sync_token = Some(sync_token.into());
            self
        }
        #[doc = "Used to perform an operation only if the targeted resource's etag matches the value provided."]
        pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
            self.if_match = Some(if_match.into());
            self
        }
        #[doc = "Used to perform an operation only if the targeted resource's etag does not match the value provided."]
        pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
            self.if_none_match = Some(if_none_match.into());
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
                    let bearer_token = this.client.bearer_token().await?;
                    req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                    if let Some(label) = &this.label {
                        req.url_mut().query_pairs_mut().append_pair("label", label);
                    }
                    let req_body = if let Some(entity) = &this.entity {
                        req.insert_header("content-type", "application/json");
                        azure_core::to_json(entity)?
                    } else {
                        azure_core::EMPTY_BODY
                    };
                    if let Some(sync_token) = &this.sync_token {
                        req.insert_header("sync-token", sync_token);
                    }
                    if let Some(if_match) = &this.if_match {
                        req.insert_header("if-match", if_match);
                    }
                    if let Some(if_none_match) = &this.if_none_match {
                        req.insert_header("if-none-match", if_none_match);
                    }
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let mut url = self.client.endpoint().clone();
            url.set_path(&format!("/kv/{}", &self.key));
            let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
            if !has_api_version_already {
                url.query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, "1.0");
            }
            Ok(url)
        }
    }
    impl std::future::IntoFuture for RequestBuilder {
        type Output = azure_core::Result<models::KeyValue>;
        type IntoFuture = BoxFuture<'static, azure_core::Result<models::KeyValue>>;
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
pub mod delete_key_value {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    #[derive(Debug)]
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::KeyValue> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::KeyValue = serde_json::from_slice(&bytes)?;
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
        #[doc = "Enables real-time consistency between requests by providing the returned value in the next request made to the server."]
        pub fn sync_token(&self) -> azure_core::Result<&str> {
            self.0.get_str(&azure_core::headers::HeaderName::from_static("sync-token"))
        }
        #[doc = "An identifier representing the returned state of the resource."]
        pub fn e_tag(&self) -> azure_core::Result<&str> {
            self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
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
        pub(crate) client: super::Client,
        pub(crate) key: String,
        pub(crate) label: Option<String>,
        pub(crate) sync_token: Option<String>,
        pub(crate) if_match: Option<String>,
    }
    impl RequestBuilder {
        #[doc = "The label of the key-value to delete."]
        pub fn label(mut self, label: impl Into<String>) -> Self {
            self.label = Some(label.into());
            self
        }
        #[doc = "Used to guarantee real-time consistency between requests."]
        pub fn sync_token(mut self, sync_token: impl Into<String>) -> Self {
            self.sync_token = Some(sync_token.into());
            self
        }
        #[doc = "Used to perform an operation only if the targeted resource's etag matches the value provided."]
        pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
            self.if_match = Some(if_match.into());
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
                    let bearer_token = this.client.bearer_token().await?;
                    req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                    if let Some(label) = &this.label {
                        req.url_mut().query_pairs_mut().append_pair("label", label);
                    }
                    if let Some(sync_token) = &this.sync_token {
                        req.insert_header("sync-token", sync_token);
                    }
                    if let Some(if_match) = &this.if_match {
                        req.insert_header("if-match", if_match);
                    }
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let mut url = self.client.endpoint().clone();
            url.set_path(&format!("/kv/{}", &self.key));
            let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
            if !has_api_version_already {
                url.query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, "1.0");
            }
            Ok(url)
        }
    }
    impl std::future::IntoFuture for RequestBuilder {
        type Output = azure_core::Result<models::KeyValue>;
        type IntoFuture = BoxFuture<'static, azure_core::Result<models::KeyValue>>;
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
pub mod check_key_value {
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
        #[doc = "Enables real-time consistency between requests by providing the returned value in the next request made to the server."]
        pub fn sync_token(&self) -> azure_core::Result<&str> {
            self.0.get_str(&azure_core::headers::HeaderName::from_static("sync-token"))
        }
        #[doc = "An identifier representing the returned state of the resource."]
        pub fn e_tag(&self) -> azure_core::Result<&str> {
            self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
        }
        #[doc = "A UTC datetime that specifies the last time the resource was modified."]
        pub fn last_modified(&self) -> azure_core::Result<&str> {
            self.0.get_str(&azure_core::headers::HeaderName::from_static("last-modified"))
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
        pub(crate) client: super::Client,
        pub(crate) key: String,
        pub(crate) label: Option<String>,
        pub(crate) sync_token: Option<String>,
        pub(crate) accept_datetime: Option<String>,
        pub(crate) if_match: Option<String>,
        pub(crate) if_none_match: Option<String>,
        pub(crate) select: Vec<String>,
    }
    impl RequestBuilder {
        #[doc = "The label of the key-value to retrieve."]
        pub fn label(mut self, label: impl Into<String>) -> Self {
            self.label = Some(label.into());
            self
        }
        #[doc = "Used to guarantee real-time consistency between requests."]
        pub fn sync_token(mut self, sync_token: impl Into<String>) -> Self {
            self.sync_token = Some(sync_token.into());
            self
        }
        #[doc = "Requests the server to respond with the state of the resource at the specified time."]
        pub fn accept_datetime(mut self, accept_datetime: impl Into<String>) -> Self {
            self.accept_datetime = Some(accept_datetime.into());
            self
        }
        #[doc = "Used to perform an operation only if the targeted resource's etag matches the value provided."]
        pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
            self.if_match = Some(if_match.into());
            self
        }
        #[doc = "Used to perform an operation only if the targeted resource's etag does not match the value provided."]
        pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
            self.if_none_match = Some(if_none_match.into());
            self
        }
        #[doc = "Used to select what fields are present in the returned resource(s)."]
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
                    let url = this.url()?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Head);
                    let bearer_token = this.client.bearer_token().await?;
                    req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                    if let Some(label) = &this.label {
                        req.url_mut().query_pairs_mut().append_pair("label", label);
                    }
                    if let Some(sync_token) = &this.sync_token {
                        req.insert_header("sync-token", sync_token);
                    }
                    if let Some(accept_datetime) = &this.accept_datetime {
                        req.insert_header("accept-datetime", accept_datetime);
                    }
                    if let Some(if_match) = &this.if_match {
                        req.insert_header("if-match", if_match);
                    }
                    if let Some(if_none_match) = &this.if_none_match {
                        req.insert_header("if-none-match", if_none_match);
                    }
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let mut url = self.client.endpoint().clone();
            url.set_path(&format!("/kv/{}", &self.key));
            let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
            if !has_api_version_already {
                url.query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, "1.0");
            }
            Ok(url)
        }
    }
}
pub mod get_labels {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    #[derive(Debug)]
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::LabelListResult> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::LabelListResult = serde_json::from_slice(&bytes)?;
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
        #[doc = "Enables real-time consistency between requests by providing the returned value in the next request made to the server."]
        pub fn sync_token(&self) -> azure_core::Result<&str> {
            self.0.get_str(&azure_core::headers::HeaderName::from_static("sync-token"))
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
        pub(crate) client: super::Client,
        pub(crate) name: Option<String>,
        pub(crate) sync_token: Option<String>,
        pub(crate) after: Option<String>,
        pub(crate) accept_datetime: Option<String>,
        pub(crate) select: Vec<String>,
    }
    impl RequestBuilder {
        #[doc = "A filter for the name of the returned labels."]
        pub fn name(mut self, name: impl Into<String>) -> Self {
            self.name = Some(name.into());
            self
        }
        #[doc = "Used to guarantee real-time consistency between requests."]
        pub fn sync_token(mut self, sync_token: impl Into<String>) -> Self {
            self.sync_token = Some(sync_token.into());
            self
        }
        #[doc = "Instructs the server to return elements that appear after the element referred to by the specified token."]
        pub fn after(mut self, after: impl Into<String>) -> Self {
            self.after = Some(after.into());
            self
        }
        #[doc = "Requests the server to respond with the state of the resource at the specified time."]
        pub fn accept_datetime(mut self, accept_datetime: impl Into<String>) -> Self {
            self.accept_datetime = Some(accept_datetime.into());
            self
        }
        #[doc = "Used to select what fields are present in the returned resource(s)."]
        pub fn select(mut self, select: Vec<String>) -> Self {
            self.select = select;
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<models::LabelListResult, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = this.url()?;
                    let rsp = match continuation {
                        Some(value) => {
                            url.set_path("");
                            url = url.join(&value)?;
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let bearer_token = this.client.bearer_token().await?;
                            req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                            let has_api_version_already =
                                req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                            if !has_api_version_already {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "1.0");
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                        None => {
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let bearer_token = this.client.bearer_token().await?;
                            req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                            if let Some(name) = &this.name {
                                req.url_mut().query_pairs_mut().append_pair("name", name);
                            }
                            if let Some(sync_token) = &this.sync_token {
                                req.insert_header("sync-token", sync_token);
                            }
                            if let Some(after) = &this.after {
                                req.url_mut().query_pairs_mut().append_pair("After", after);
                            }
                            if let Some(accept_datetime) = &this.accept_datetime {
                                req.insert_header("accept-datetime", accept_datetime);
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
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let mut url = self.client.endpoint().clone();
            url.set_path(&format!("/labels",));
            let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
            if !has_api_version_already {
                url.query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, "1.0");
            }
            Ok(url)
        }
    }
}
pub mod check_labels {
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
        #[doc = "Enables real-time consistency between requests by providing the returned value in the next request made to the server."]
        pub fn sync_token(&self) -> azure_core::Result<&str> {
            self.0.get_str(&azure_core::headers::HeaderName::from_static("sync-token"))
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
        pub(crate) client: super::Client,
        pub(crate) name: Option<String>,
        pub(crate) sync_token: Option<String>,
        pub(crate) after: Option<String>,
        pub(crate) accept_datetime: Option<String>,
        pub(crate) select: Vec<String>,
    }
    impl RequestBuilder {
        #[doc = "A filter for the name of the returned labels."]
        pub fn name(mut self, name: impl Into<String>) -> Self {
            self.name = Some(name.into());
            self
        }
        #[doc = "Used to guarantee real-time consistency between requests."]
        pub fn sync_token(mut self, sync_token: impl Into<String>) -> Self {
            self.sync_token = Some(sync_token.into());
            self
        }
        #[doc = "Instructs the server to return elements that appear after the element referred to by the specified token."]
        pub fn after(mut self, after: impl Into<String>) -> Self {
            self.after = Some(after.into());
            self
        }
        #[doc = "Requests the server to respond with the state of the resource at the specified time."]
        pub fn accept_datetime(mut self, accept_datetime: impl Into<String>) -> Self {
            self.accept_datetime = Some(accept_datetime.into());
            self
        }
        #[doc = "Used to select what fields are present in the returned resource(s)."]
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
                    let url = this.url()?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Head);
                    let bearer_token = this.client.bearer_token().await?;
                    req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                    if let Some(name) = &this.name {
                        req.url_mut().query_pairs_mut().append_pair("name", name);
                    }
                    if let Some(sync_token) = &this.sync_token {
                        req.insert_header("sync-token", sync_token);
                    }
                    if let Some(after) = &this.after {
                        req.url_mut().query_pairs_mut().append_pair("After", after);
                    }
                    if let Some(accept_datetime) = &this.accept_datetime {
                        req.insert_header("accept-datetime", accept_datetime);
                    }
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let mut url = self.client.endpoint().clone();
            url.set_path(&format!("/labels",));
            let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
            if !has_api_version_already {
                url.query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, "1.0");
            }
            Ok(url)
        }
    }
}
pub mod put_lock {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    #[derive(Debug)]
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::KeyValue> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::KeyValue = serde_json::from_slice(&bytes)?;
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
        #[doc = "Enables real-time consistency between requests by providing the returned value in the next request made to the server."]
        pub fn sync_token(&self) -> azure_core::Result<&str> {
            self.0.get_str(&azure_core::headers::HeaderName::from_static("sync-token"))
        }
        #[doc = "An identifier representing the returned state of the resource."]
        pub fn e_tag(&self) -> azure_core::Result<&str> {
            self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
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
        pub(crate) client: super::Client,
        pub(crate) key: String,
        pub(crate) label: Option<String>,
        pub(crate) sync_token: Option<String>,
        pub(crate) if_match: Option<String>,
        pub(crate) if_none_match: Option<String>,
    }
    impl RequestBuilder {
        #[doc = "The label, if any, of the key-value to lock."]
        pub fn label(mut self, label: impl Into<String>) -> Self {
            self.label = Some(label.into());
            self
        }
        #[doc = "Used to guarantee real-time consistency between requests."]
        pub fn sync_token(mut self, sync_token: impl Into<String>) -> Self {
            self.sync_token = Some(sync_token.into());
            self
        }
        #[doc = "Used to perform an operation only if the targeted resource's etag matches the value provided."]
        pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
            self.if_match = Some(if_match.into());
            self
        }
        #[doc = "Used to perform an operation only if the targeted resource's etag does not match the value provided."]
        pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
            self.if_none_match = Some(if_none_match.into());
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
                    let bearer_token = this.client.bearer_token().await?;
                    req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                    if let Some(label) = &this.label {
                        req.url_mut().query_pairs_mut().append_pair("label", label);
                    }
                    if let Some(sync_token) = &this.sync_token {
                        req.insert_header("sync-token", sync_token);
                    }
                    if let Some(if_match) = &this.if_match {
                        req.insert_header("if-match", if_match);
                    }
                    if let Some(if_none_match) = &this.if_none_match {
                        req.insert_header("if-none-match", if_none_match);
                    }
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let mut url = self.client.endpoint().clone();
            url.set_path(&format!("/locks/{}", &self.key));
            let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
            if !has_api_version_already {
                url.query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, "1.0");
            }
            Ok(url)
        }
    }
    impl std::future::IntoFuture for RequestBuilder {
        type Output = azure_core::Result<models::KeyValue>;
        type IntoFuture = BoxFuture<'static, azure_core::Result<models::KeyValue>>;
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
pub mod delete_lock {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    #[derive(Debug)]
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::KeyValue> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::KeyValue = serde_json::from_slice(&bytes)?;
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
        #[doc = "Enables real-time consistency between requests by providing the returned value in the next request made to the server."]
        pub fn sync_token(&self) -> azure_core::Result<&str> {
            self.0.get_str(&azure_core::headers::HeaderName::from_static("sync-token"))
        }
        #[doc = "An identifier representing the returned state of the resource."]
        pub fn e_tag(&self) -> azure_core::Result<&str> {
            self.0.get_str(&azure_core::headers::HeaderName::from_static("etag"))
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
        pub(crate) client: super::Client,
        pub(crate) key: String,
        pub(crate) label: Option<String>,
        pub(crate) sync_token: Option<String>,
        pub(crate) if_match: Option<String>,
        pub(crate) if_none_match: Option<String>,
    }
    impl RequestBuilder {
        #[doc = "The label, if any, of the key-value to unlock."]
        pub fn label(mut self, label: impl Into<String>) -> Self {
            self.label = Some(label.into());
            self
        }
        #[doc = "Used to guarantee real-time consistency between requests."]
        pub fn sync_token(mut self, sync_token: impl Into<String>) -> Self {
            self.sync_token = Some(sync_token.into());
            self
        }
        #[doc = "Used to perform an operation only if the targeted resource's etag matches the value provided."]
        pub fn if_match(mut self, if_match: impl Into<String>) -> Self {
            self.if_match = Some(if_match.into());
            self
        }
        #[doc = "Used to perform an operation only if the targeted resource's etag does not match the value provided."]
        pub fn if_none_match(mut self, if_none_match: impl Into<String>) -> Self {
            self.if_none_match = Some(if_none_match.into());
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
                    let bearer_token = this.client.bearer_token().await?;
                    req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                    if let Some(label) = &this.label {
                        req.url_mut().query_pairs_mut().append_pair("label", label);
                    }
                    if let Some(sync_token) = &this.sync_token {
                        req.insert_header("sync-token", sync_token);
                    }
                    if let Some(if_match) = &this.if_match {
                        req.insert_header("if-match", if_match);
                    }
                    if let Some(if_none_match) = &this.if_none_match {
                        req.insert_header("if-none-match", if_none_match);
                    }
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let mut url = self.client.endpoint().clone();
            url.set_path(&format!("/locks/{}", &self.key));
            let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
            if !has_api_version_already {
                url.query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, "1.0");
            }
            Ok(url)
        }
    }
    impl std::future::IntoFuture for RequestBuilder {
        type Output = azure_core::Result<models::KeyValue>;
        type IntoFuture = BoxFuture<'static, azure_core::Result<models::KeyValue>>;
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
pub mod get_revisions {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    #[derive(Debug)]
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::KeyValueListResult> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::KeyValueListResult = serde_json::from_slice(&bytes)?;
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
        #[doc = "Enables real-time consistency between requests by providing the returned value in the next request made to the server."]
        pub fn sync_token(&self) -> azure_core::Result<&str> {
            self.0.get_str(&azure_core::headers::HeaderName::from_static("sync-token"))
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
        pub(crate) client: super::Client,
        pub(crate) key: Option<String>,
        pub(crate) label: Option<String>,
        pub(crate) sync_token: Option<String>,
        pub(crate) after: Option<String>,
        pub(crate) accept_datetime: Option<String>,
        pub(crate) select: Vec<String>,
    }
    impl RequestBuilder {
        #[doc = "A filter used to match keys."]
        pub fn key(mut self, key: impl Into<String>) -> Self {
            self.key = Some(key.into());
            self
        }
        #[doc = "A filter used to match labels"]
        pub fn label(mut self, label: impl Into<String>) -> Self {
            self.label = Some(label.into());
            self
        }
        #[doc = "Used to guarantee real-time consistency between requests."]
        pub fn sync_token(mut self, sync_token: impl Into<String>) -> Self {
            self.sync_token = Some(sync_token.into());
            self
        }
        #[doc = "Instructs the server to return elements that appear after the element referred to by the specified token."]
        pub fn after(mut self, after: impl Into<String>) -> Self {
            self.after = Some(after.into());
            self
        }
        #[doc = "Requests the server to respond with the state of the resource at the specified time."]
        pub fn accept_datetime(mut self, accept_datetime: impl Into<String>) -> Self {
            self.accept_datetime = Some(accept_datetime.into());
            self
        }
        #[doc = "Used to select what fields are present in the returned resource(s)."]
        pub fn select(mut self, select: Vec<String>) -> Self {
            self.select = select;
            self
        }
        pub fn into_stream(self) -> azure_core::Pageable<models::KeyValueListResult, azure_core::error::Error> {
            let make_request = move |continuation: Option<String>| {
                let this = self.clone();
                async move {
                    let mut url = this.url()?;
                    let rsp = match continuation {
                        Some(value) => {
                            url.set_path("");
                            url = url.join(&value)?;
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let bearer_token = this.client.bearer_token().await?;
                            req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                            let has_api_version_already =
                                req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                            if !has_api_version_already {
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "1.0");
                            }
                            let req_body = azure_core::EMPTY_BODY;
                            req.set_body(req_body);
                            this.client.send(&mut req).await?
                        }
                        None => {
                            let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                            let bearer_token = this.client.bearer_token().await?;
                            req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                            if let Some(key) = &this.key {
                                req.url_mut().query_pairs_mut().append_pair("key", key);
                            }
                            if let Some(label) = &this.label {
                                req.url_mut().query_pairs_mut().append_pair("label", label);
                            }
                            if let Some(sync_token) = &this.sync_token {
                                req.insert_header("sync-token", sync_token);
                            }
                            if let Some(after) = &this.after {
                                req.url_mut().query_pairs_mut().append_pair("After", after);
                            }
                            if let Some(accept_datetime) = &this.accept_datetime {
                                req.insert_header("accept-datetime", accept_datetime);
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
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let mut url = self.client.endpoint().clone();
            url.set_path(&format!("/revisions",));
            let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
            if !has_api_version_already {
                url.query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, "1.0");
            }
            Ok(url)
        }
    }
}
pub mod check_revisions {
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
        #[doc = "Enables real-time consistency between requests by providing the returned value in the next request made to the server."]
        pub fn sync_token(&self) -> azure_core::Result<&str> {
            self.0.get_str(&azure_core::headers::HeaderName::from_static("sync-token"))
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
        pub(crate) client: super::Client,
        pub(crate) key: Option<String>,
        pub(crate) label: Option<String>,
        pub(crate) sync_token: Option<String>,
        pub(crate) after: Option<String>,
        pub(crate) accept_datetime: Option<String>,
        pub(crate) select: Vec<String>,
    }
    impl RequestBuilder {
        #[doc = "A filter used to match keys."]
        pub fn key(mut self, key: impl Into<String>) -> Self {
            self.key = Some(key.into());
            self
        }
        #[doc = "A filter used to match labels"]
        pub fn label(mut self, label: impl Into<String>) -> Self {
            self.label = Some(label.into());
            self
        }
        #[doc = "Used to guarantee real-time consistency between requests."]
        pub fn sync_token(mut self, sync_token: impl Into<String>) -> Self {
            self.sync_token = Some(sync_token.into());
            self
        }
        #[doc = "Instructs the server to return elements that appear after the element referred to by the specified token."]
        pub fn after(mut self, after: impl Into<String>) -> Self {
            self.after = Some(after.into());
            self
        }
        #[doc = "Requests the server to respond with the state of the resource at the specified time."]
        pub fn accept_datetime(mut self, accept_datetime: impl Into<String>) -> Self {
            self.accept_datetime = Some(accept_datetime.into());
            self
        }
        #[doc = "Used to select what fields are present in the returned resource(s)."]
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
                    let url = this.url()?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Head);
                    let bearer_token = this.client.bearer_token().await?;
                    req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                    if let Some(key) = &this.key {
                        req.url_mut().query_pairs_mut().append_pair("key", key);
                    }
                    if let Some(label) = &this.label {
                        req.url_mut().query_pairs_mut().append_pair("label", label);
                    }
                    if let Some(sync_token) = &this.sync_token {
                        req.insert_header("sync-token", sync_token);
                    }
                    if let Some(after) = &this.after {
                        req.url_mut().query_pairs_mut().append_pair("After", after);
                    }
                    if let Some(accept_datetime) = &this.accept_datetime {
                        req.insert_header("accept-datetime", accept_datetime);
                    }
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let mut url = self.client.endpoint().clone();
            url.set_path(&format!("/revisions",));
            let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
            if !has_api_version_already {
                url.query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, "1.0");
            }
            Ok(url)
        }
    }
}
