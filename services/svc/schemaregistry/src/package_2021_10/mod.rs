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
    pub fn schema_client(&self) -> schema::Client {
        schema::Client(self.clone())
    }
    pub fn schema_groups_client(&self) -> schema_groups::Client {
        schema_groups::Client(self.clone())
    }
}
pub mod schema_groups {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get list of schema groups."]
        #[doc = "Gets the list of schema groups user is authorized to access."]
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
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SchemaGroups> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SchemaGroups = serde_json::from_slice(&bytes)?;
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
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/$schemaGroups", self.client.endpoint(),))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, "2021-10");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SchemaGroups>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SchemaGroups>>;
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
pub mod schema {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a registered schema by its unique ID reference."]
        #[doc = "Gets a registered schema by its unique ID.  Azure Schema Registry guarantees that ID is unique within a namespace. Operation response type is based on serialization of schema requested."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `id`: References specific schema in registry namespace."]
        pub fn get_by_id(&self, id: impl Into<String>) -> get_by_id::RequestBuilder {
            get_by_id::RequestBuilder {
                client: self.0.clone(),
                id: id.into(),
            }
        }
        #[doc = "Get list schema versions."]
        #[doc = "Gets the list of all versions of one schema."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `group_name`: Schema group under which schema is registered.  Group's serialization type should match the serialization type specified in the request."]
        #[doc = "* `schema_name`: Name of schema."]
        pub fn get_versions(&self, group_name: impl Into<String>, schema_name: impl Into<String>) -> get_versions::RequestBuilder {
            get_versions::RequestBuilder {
                client: self.0.clone(),
                group_name: group_name.into(),
                schema_name: schema_name.into(),
            }
        }
        #[doc = "Get specific schema versions."]
        #[doc = "Gets one specific version of one schema."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `group_name`: Schema group under which schema is registered.  Group's serialization type should match the serialization type specified in the request."]
        #[doc = "* `schema_name`: Name of schema."]
        #[doc = "* `schema_version`: Version number of specific schema."]
        pub fn get_schema_version(
            &self,
            group_name: impl Into<String>,
            schema_name: impl Into<String>,
            schema_version: i32,
        ) -> get_schema_version::RequestBuilder {
            get_schema_version::RequestBuilder {
                client: self.0.clone(),
                group_name: group_name.into(),
                schema_name: schema_name.into(),
                schema_version,
            }
        }
        #[doc = "Get ID for existing schema."]
        #[doc = "Gets the ID referencing an existing schema within the specified schema group, as matched by schema content comparison."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `group_name`: Schema group under which schema is registered.  Group's serialization type should match the serialization type specified in the request."]
        #[doc = "* `schema_name`: Name of schema."]
        #[doc = "* `schema_content`: String representation (UTF-8) of the registered schema."]
        pub fn query_id_by_content(
            &self,
            group_name: impl Into<String>,
            schema_name: impl Into<String>,
            schema_content: impl Into<String>,
        ) -> query_id_by_content::RequestBuilder {
            query_id_by_content::RequestBuilder {
                client: self.0.clone(),
                group_name: group_name.into(),
                schema_name: schema_name.into(),
                schema_content: schema_content.into(),
            }
        }
        #[doc = "Register new schema"]
        #[doc = "Register new schema. If schema of specified name does not exist in specified group, schema is created at version 1. If schema of specified name exists already in specified group, schema is created at latest version + 1.\n"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `group_name`: Schema group under which schema should be registered.  Group's serialization type should match the serialization type specified in the request."]
        #[doc = "* `schema_name`: Name of schema."]
        #[doc = "* `schema_content`: String representation (UTF-8) of the schema being registered."]
        pub fn register(
            &self,
            group_name: impl Into<String>,
            schema_name: impl Into<String>,
            schema_content: impl Into<String>,
        ) -> register::RequestBuilder {
            register::RequestBuilder {
                client: self.0.clone(),
                group_name: group_name.into(),
                schema_name: schema_name.into(),
                schema_content: schema_content.into(),
            }
        }
    }
    pub mod get_by_id {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
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
            #[doc = "URL location of schema, identified by schema group, schema name, and version."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("location"))
            }
            #[doc = "The content type for given schema. Each schema type has an associated content-type."]
            pub fn content_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-type"))
            }
            #[doc = "References specific schema in registry namespace."]
            pub fn schema_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("schema-id"))
            }
            #[doc = "URL location of schema, identified by schema ID."]
            pub fn schema_id_location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("schema-id-location"))
            }
            #[doc = "References schema group."]
            pub fn schema_group_name(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("schema-group-name"))
            }
            #[doc = "References schema name."]
            pub fn schema_name(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("schema-name"))
            }
            #[doc = "Version of the returned schema."]
            pub fn schema_version(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("schema-version"))
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
            pub(crate) id: String,
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
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!("{}/$schemaGroups/$schemas/{}", self.client.endpoint(), &self.id))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, "2021-10");
                }
                Ok(url)
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
    pub mod get_versions {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SchemaVersions> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SchemaVersions = serde_json::from_slice(&bytes)?;
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
            pub(crate) group_name: String,
            pub(crate) schema_name: String,
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
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/$schemaGroups/{}/schemas/{}/versions",
                    self.client.endpoint(),
                    &self.group_name,
                    &self.schema_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, "2021-10");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SchemaVersions>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SchemaVersions>>;
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
    pub mod get_schema_version {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
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
            #[doc = "URL location of schema, identified by schema group, schema name, and version."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("location"))
            }
            #[doc = "The content type for given schema. Each schema type has an associated content-type."]
            pub fn content_type(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("content-type"))
            }
            #[doc = "References specific schema in registry namespace."]
            pub fn schema_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("schema-id"))
            }
            #[doc = "URL location of schema, identified by schema ID."]
            pub fn schema_id_location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("schema-id-location"))
            }
            #[doc = "References schema group."]
            pub fn schema_group_name(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("schema-group-name"))
            }
            #[doc = "References schema name."]
            pub fn schema_name(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("schema-name"))
            }
            #[doc = "Version of the returned schema."]
            pub fn schema_version(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("schema-version"))
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
            pub(crate) group_name: String,
            pub(crate) schema_name: String,
            pub(crate) schema_version: i32,
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
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/$schemaGroups/{}/schemas/{}/versions/{}",
                    self.client.endpoint(),
                    &self.group_name,
                    &self.schema_name,
                    &self.schema_version
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, "2021-10");
                }
                Ok(url)
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
    pub mod query_id_by_content {
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
            #[doc = "URL location of schema, identified by schema group, schema name, and version."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("location"))
            }
            #[doc = "References specific schema in registry namespace."]
            pub fn schema_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("schema-id"))
            }
            #[doc = "URL location of schema, identified by schema ID."]
            pub fn schema_id_location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("schema-id-location"))
            }
            #[doc = "References schema group."]
            pub fn schema_group_name(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("schema-group-name"))
            }
            #[doc = "References schema name."]
            pub fn schema_name(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("schema-name"))
            }
            #[doc = "Version of the returned schema."]
            pub fn schema_version(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("schema-version"))
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
            pub(crate) group_name: String,
            pub(crate) schema_name: String,
            pub(crate) schema_content: String,
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
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        req.insert_header("content-type", "application/json; serialization=Avro");
                        let req_body = azure_core::to_json(&this.schema_content)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/$schemaGroups/{}/schemas/{}:get-id",
                    self.client.endpoint(),
                    &self.group_name,
                    &self.schema_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, "2021-10");
                }
                Ok(url)
            }
        }
    }
    pub mod register {
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
            #[doc = "URL location of schema, identified by schema group, schema name, and version."]
            pub fn location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("location"))
            }
            #[doc = "References specific schema in registry namespace."]
            pub fn schema_id(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("schema-id"))
            }
            #[doc = "URL location of schema, identified by schema ID."]
            pub fn schema_id_location(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("schema-id-location"))
            }
            #[doc = "References schema group."]
            pub fn schema_group_name(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("schema-group-name"))
            }
            #[doc = "References schema name."]
            pub fn schema_name(&self) -> azure_core::Result<&str> {
                self.0.get_str(&azure_core::headers::HeaderName::from_static("schema-name"))
            }
            #[doc = "Version of the returned schema."]
            pub fn schema_version(&self) -> azure_core::Result<i32> {
                self.0.get_as(&azure_core::headers::HeaderName::from_static("schema-version"))
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
            pub(crate) group_name: String,
            pub(crate) schema_name: String,
            pub(crate) schema_content: String,
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
                        let url = this.url()?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        req.insert_header("content-type", "application/json; serialization=Avro");
                        let req_body = azure_core::to_json(&this.schema_content)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/$schemaGroups/{}/schemas/{}",
                    self.client.endpoint(),
                    &self.group_name,
                    &self.schema_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, "2021-10");
                }
                Ok(url)
            }
        }
    }
}
