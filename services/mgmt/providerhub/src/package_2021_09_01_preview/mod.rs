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
    pub fn custom_rollouts_client(&self) -> custom_rollouts::Client {
        custom_rollouts::Client(self.clone())
    }
    pub fn default_rollouts_client(&self) -> default_rollouts::Client {
        default_rollouts::Client(self.clone())
    }
    pub fn notification_registrations_client(&self) -> notification_registrations::Client {
        notification_registrations::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn provider_registrations_client(&self) -> provider_registrations::Client {
        provider_registrations::Client(self.clone())
    }
    pub fn resource_actions_client(&self) -> resource_actions::Client {
        resource_actions::Client(self.clone())
    }
    pub fn resource_type_registrations_client(&self) -> resource_type_registrations::Client {
        resource_type_registrations::Client(self.clone())
    }
    pub fn skus_client(&self) -> skus::Client {
        skus::Client(self.clone())
    }
}
pub mod custom_rollouts {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the custom rollout details."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `rollout_name`: The rollout name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            rollout_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                rollout_name: rollout_name.into(),
            }
        }
        #[doc = "Creates or updates the rollout details."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `rollout_name`: The rollout name."]
        #[doc = "* `properties`: The custom rollout properties supplied to the CreateOrUpdate operation."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            rollout_name: impl Into<String>,
            properties: impl Into<models::CustomRollout>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                rollout_name: rollout_name.into(),
                properties: properties.into(),
            }
        }
        #[doc = "Gets the list of the custom rollouts for the given provider."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        pub fn list_by_provider_registration(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
        ) -> list_by_provider_registration::RequestBuilder {
            list_by_provider_registration::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::CustomRollout> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::CustomRollout = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) rollout_name: String,
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
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/customRollouts/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace,
                    &self.rollout_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::CustomRollout>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::CustomRollout>>;
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
    pub mod create_or_update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::CustomRollout> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::CustomRollout = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) rollout_name: String,
            pub(crate) properties: models::CustomRollout,
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
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.properties)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/customRollouts/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace,
                    &self.rollout_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::CustomRollout>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::CustomRollout>>;
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
    pub mod list_by_provider_registration {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::CustomRolloutArrayResponseWithContinuation> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::CustomRolloutArrayResponseWithContinuation = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::CustomRolloutArrayResponseWithContinuation, azure_core::error::Error> {
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
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
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/customRollouts",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
    }
}
pub mod default_rollouts {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the default rollout details."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `rollout_name`: The rollout name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            rollout_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                rollout_name: rollout_name.into(),
            }
        }
        #[doc = "Creates or updates the rollout details."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `rollout_name`: The rollout name."]
        #[doc = "* `properties`: The Default rollout properties supplied to the CreateOrUpdate operation."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            rollout_name: impl Into<String>,
            properties: impl Into<models::DefaultRollout>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                rollout_name: rollout_name.into(),
                properties: properties.into(),
            }
        }
        #[doc = "Deletes the rollout resource. Rollout must be in terminal state."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `rollout_name`: The rollout name."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            rollout_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                rollout_name: rollout_name.into(),
            }
        }
        #[doc = "Gets the list of the rollouts for the given provider."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        pub fn list_by_provider_registration(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
        ) -> list_by_provider_registration::RequestBuilder {
            list_by_provider_registration::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
            }
        }
        #[doc = "Stops or cancels the rollout, if in progress."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `rollout_name`: The rollout name."]
        pub fn stop(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            rollout_name: impl Into<String>,
        ) -> stop::RequestBuilder {
            stop::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                rollout_name: rollout_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DefaultRollout> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DefaultRollout = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) rollout_name: String,
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
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/defaultRollouts/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace,
                    &self.rollout_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DefaultRollout>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::DefaultRollout>>;
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
    pub mod create_or_update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DefaultRollout> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DefaultRollout = serde_json::from_slice(&bytes)?;
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
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) rollout_name: String,
            pub(crate) properties: models::DefaultRollout,
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
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.properties)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/defaultRollouts/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace,
                    &self.rollout_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::DefaultRollout>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::DefaultRollout>>;
            #[doc = "Returns a future that polls the long running operation, returning once the operation completes."]
            #[doc = ""]
            #[doc = "To only submit the request but not monitor the status of the operation until completion, use `send()` instead."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move {
                    use azure_core::{
                        error::{Error, ErrorKind},
                        lro::{
                            get_retry_after,
                            location::{get_location, get_provisioning_state, FinalState},
                            LroStatus,
                        },
                        sleep::sleep,
                    };
                    use std::time::Duration;
                    let this = self.clone();
                    let response = this.send().await?;
                    let headers = response.as_raw_response().headers();
                    let location = get_location(headers, FinalState::AzureAsyncOperation)?;
                    if let Some(url) = location {
                        loop {
                            let mut req = azure_core::Request::new(url.clone(), azure_core::Method::Get);
                            let bearer_token = self.client.bearer_token().await?;
                            req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                            let response = self.client.send(&mut req).await?;
                            let headers = response.headers();
                            let retry_after = get_retry_after(headers);
                            let bytes = response.into_body().collect().await?;
                            let provisioning_state = get_provisioning_state(&bytes).ok_or_else(|| {
                                Error::message(
                                    ErrorKind::Other,
                                    "Long running operation failed (missing provisioning state)".to_string(),
                                )
                            })?;
                            log::trace!("current provisioning_state: {provisioning_state:?}");
                            match provisioning_state {
                                LroStatus::Succeeded => {
                                    let mut req = azure_core::Request::new(self.url()?, azure_core::Method::Get);
                                    let bearer_token = self.client.bearer_token().await?;
                                    req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                    let response = self.client.send(&mut req).await?;
                                    return Response(response).into_body().await;
                                }
                                LroStatus::Failed => {
                                    return Err(Error::message(ErrorKind::Other, "Long running operation failed".to_string()))
                                }
                                LroStatus::Canceled => {
                                    return Err(Error::message(ErrorKind::Other, "Long running operation canceled".to_string()))
                                }
                                _ => {
                                    sleep(retry_after).await;
                                }
                            }
                        }
                    } else {
                        response.into_body().await
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
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) rollout_name: String,
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
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
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
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/defaultRollouts/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace,
                    &self.rollout_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
    }
    pub mod list_by_provider_registration {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::DefaultRolloutArrayResponseWithContinuation> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::DefaultRolloutArrayResponseWithContinuation = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
        }
        impl RequestBuilder {
            pub fn into_stream(
                self,
            ) -> azure_core::Pageable<models::DefaultRolloutArrayResponseWithContinuation, azure_core::error::Error> {
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
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
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/defaultRollouts",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
    }
    pub mod stop {
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) rollout_name: String,
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/defaultRollouts/{}/stop",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace,
                    &self.rollout_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
    }
}
impl Client {
    #[doc = "Generates the manifest for the given provider."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `subscription_id`: The ID of the target subscription."]
    #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
    pub fn generate_manifest(
        &self,
        subscription_id: impl Into<String>,
        provider_namespace: impl Into<String>,
    ) -> generate_manifest::RequestBuilder {
        generate_manifest::RequestBuilder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            provider_namespace: provider_namespace.into(),
        }
    }
    #[doc = "Checkin the manifest."]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `subscription_id`: The ID of the target subscription."]
    #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
    #[doc = "* `checkin_manifest_params`: The required body parameters supplied to the checkin manifest operation."]
    pub fn checkin_manifest(
        &self,
        subscription_id: impl Into<String>,
        provider_namespace: impl Into<String>,
        checkin_manifest_params: impl Into<models::CheckinManifestParams>,
    ) -> checkin_manifest::RequestBuilder {
        checkin_manifest::RequestBuilder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            provider_namespace: provider_namespace.into(),
            checkin_manifest_params: checkin_manifest_params.into(),
        }
    }
}
pub mod generate_manifest {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    #[derive(Debug)]
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::ResourceProviderManifest> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::ResourceProviderManifest = serde_json::from_slice(&bytes)?;
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
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) provider_namespace: String,
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
                    let req_body = azure_core::EMPTY_BODY;
                    req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let mut url = azure_core::Url::parse(&format!(
                "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/generateManifest",
                self.client.endpoint(),
                &self.subscription_id,
                &self.provider_namespace
            ))?;
            let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
            if !has_api_version_already {
                url.query_pairs_mut()
                    .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
            }
            Ok(url)
        }
    }
    impl std::future::IntoFuture for RequestBuilder {
        type Output = azure_core::Result<models::ResourceProviderManifest>;
        type IntoFuture = BoxFuture<'static, azure_core::Result<models::ResourceProviderManifest>>;
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
pub mod checkin_manifest {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    #[derive(Debug)]
    pub struct Response(azure_core::Response);
    impl Response {
        pub async fn into_body(self) -> azure_core::Result<models::CheckinManifestInfo> {
            let bytes = self.0.into_body().collect().await?;
            let body: models::CheckinManifestInfo = serde_json::from_slice(&bytes)?;
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
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) provider_namespace: String,
        pub(crate) checkin_manifest_params: models::CheckinManifestParams,
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
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.checkin_manifest_params)?;
                    req.set_body(req_body);
                    Ok(Response(this.client.send(&mut req).await?))
                }
            })
        }
        fn url(&self) -> azure_core::Result<azure_core::Url> {
            let mut url = azure_core::Url::parse(&format!(
                "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/checkinManifest",
                self.client.endpoint(),
                &self.subscription_id,
                &self.provider_namespace
            ))?;
            let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
            if !has_api_version_already {
                url.query_pairs_mut()
                    .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
            }
            Ok(url)
        }
    }
    impl std::future::IntoFuture for RequestBuilder {
        type Output = azure_core::Result<models::CheckinManifestInfo>;
        type IntoFuture = BoxFuture<'static, azure_core::Result<models::CheckinManifestInfo>>;
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
pub mod notification_registrations {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the notification registration details."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `notification_registration_name`: The notification registration."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            notification_registration_name: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                notification_registration_name: notification_registration_name.into(),
            }
        }
        #[doc = "Creates or updates a notification registration."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `notification_registration_name`: The notification registration."]
        #[doc = "* `properties`: The required body parameters supplied to the notification registration operation."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            notification_registration_name: impl Into<String>,
            properties: impl Into<models::NotificationRegistration>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                notification_registration_name: notification_registration_name.into(),
                properties: properties.into(),
            }
        }
        #[doc = "Deletes a notification registration."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `notification_registration_name`: The notification registration."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            notification_registration_name: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                notification_registration_name: notification_registration_name.into(),
            }
        }
        #[doc = "Gets the list of the notification registrations for the given provider."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        pub fn list_by_provider_registration(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
        ) -> list_by_provider_registration::RequestBuilder {
            list_by_provider_registration::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::NotificationRegistration> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::NotificationRegistration = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) notification_registration_name: String,
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
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/notificationRegistrations/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace,
                    &self.notification_registration_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::NotificationRegistration>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::NotificationRegistration>>;
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
    pub mod create_or_update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::NotificationRegistration> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::NotificationRegistration = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) notification_registration_name: String,
            pub(crate) properties: models::NotificationRegistration,
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
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.properties)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/notificationRegistrations/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace,
                    &self.notification_registration_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::NotificationRegistration>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::NotificationRegistration>>;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) notification_registration_name: String,
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
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
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
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/notificationRegistrations/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace,
                    &self.notification_registration_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
    }
    pub mod list_by_provider_registration {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::NotificationRegistrationArrayResponseWithContinuation> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::NotificationRegistrationArrayResponseWithContinuation = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
        }
        impl RequestBuilder {
            pub fn into_stream(
                self,
            ) -> azure_core::Pageable<models::NotificationRegistrationArrayResponseWithContinuation, azure_core::error::Error> {
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
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
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/notificationRegistrations",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
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
        #[doc = "Lists all the operations supported by Microsoft.ProviderHub."]
        pub fn list(&self) -> list::RequestBuilder {
            list::RequestBuilder { client: self.0.clone() }
        }
        #[doc = "Gets the operations supported by the given provider."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        pub fn list_by_provider_registration(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
        ) -> list_by_provider_registration::RequestBuilder {
            list_by_provider_registration::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
            }
        }
        #[doc = "Creates or updates the operation supported by the given provider."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `operations_put_content`: The operations content properties supplied to the CreateOrUpdate operation."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            operations_put_content: impl Into<models::OperationsPutContent>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                operations_put_content: operations_put_content.into(),
            }
        }
        #[doc = "Deletes an operation."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        pub fn delete(&self, subscription_id: impl Into<String>, provider_namespace: impl Into<String>) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
            }
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
            pub async fn into_body(self) -> azure_core::Result<models::OperationsDefinitionArrayResponseWithContinuation> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::OperationsDefinitionArrayResponseWithContinuation = serde_json::from_slice(&bytes)?;
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
            pub fn into_stream(
                self,
            ) -> azure_core::Pageable<models::OperationsDefinitionArrayResponseWithContinuation, azure_core::error::Error> {
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
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
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
                let url = azure_core::Url::parse(&format!("{}/providers/Microsoft.ProviderHub/operations", self.client.endpoint(),))?;
                Ok(url)
            }
        }
    }
    pub mod list_by_provider_registration {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<Vec<models::OperationsDefinition>> {
                let bytes = self.0.into_body().collect().await?;
                let body: Vec<models::OperationsDefinition> = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
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
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/operations/default",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<Vec<models::OperationsDefinition>>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<Vec<models::OperationsDefinition>>>;
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
    pub mod create_or_update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::OperationsPutContent> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::OperationsPutContent = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) operations_put_content: models::OperationsPutContent,
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
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.operations_put_content)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/operations/default",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::OperationsPutContent>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::OperationsPutContent>>;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
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
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
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
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/operations/default",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
    }
}
pub mod provider_registrations {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the provider registration details."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        pub fn get(&self, subscription_id: impl Into<String>, provider_namespace: impl Into<String>) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
            }
        }
        #[doc = "Creates or updates the provider registration."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `properties`: The provider registration properties supplied to the CreateOrUpdate operation."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            properties: impl Into<models::ProviderRegistration>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                properties: properties.into(),
            }
        }
        #[doc = "Deletes a provider registration."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        pub fn delete(&self, subscription_id: impl Into<String>, provider_namespace: impl Into<String>) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
            }
        }
        #[doc = "Gets the list of the provider registrations in the subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        pub fn list(&self, subscription_id: impl Into<String>) -> list::RequestBuilder {
            list::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Generates the operations api for the given provider."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        pub fn generate_operations(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
        ) -> generate_operations::RequestBuilder {
            generate_operations::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProviderRegistration> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProviderRegistration = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
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
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ProviderRegistration>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ProviderRegistration>>;
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
    pub mod create_or_update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ProviderRegistration> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProviderRegistration = serde_json::from_slice(&bytes)?;
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
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) properties: models::ProviderRegistration,
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
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.properties)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ProviderRegistration>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ProviderRegistration>>;
            #[doc = "Returns a future that polls the long running operation, returning once the operation completes."]
            #[doc = ""]
            #[doc = "To only submit the request but not monitor the status of the operation until completion, use `send()` instead."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move {
                    use azure_core::{
                        error::{Error, ErrorKind},
                        lro::{
                            get_retry_after,
                            location::{get_location, get_provisioning_state, FinalState},
                            LroStatus,
                        },
                        sleep::sleep,
                    };
                    use std::time::Duration;
                    let this = self.clone();
                    let response = this.send().await?;
                    let headers = response.as_raw_response().headers();
                    let location = get_location(headers, FinalState::AzureAsyncOperation)?;
                    if let Some(url) = location {
                        loop {
                            let mut req = azure_core::Request::new(url.clone(), azure_core::Method::Get);
                            let bearer_token = self.client.bearer_token().await?;
                            req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                            let response = self.client.send(&mut req).await?;
                            let headers = response.headers();
                            let retry_after = get_retry_after(headers);
                            let bytes = response.into_body().collect().await?;
                            let provisioning_state = get_provisioning_state(&bytes).ok_or_else(|| {
                                Error::message(
                                    ErrorKind::Other,
                                    "Long running operation failed (missing provisioning state)".to_string(),
                                )
                            })?;
                            log::trace!("current provisioning_state: {provisioning_state:?}");
                            match provisioning_state {
                                LroStatus::Succeeded => {
                                    let mut req = azure_core::Request::new(self.url()?, azure_core::Method::Get);
                                    let bearer_token = self.client.bearer_token().await?;
                                    req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                    let response = self.client.send(&mut req).await?;
                                    return Response(response).into_body().await;
                                }
                                LroStatus::Failed => {
                                    return Err(Error::message(ErrorKind::Other, "Long running operation failed".to_string()))
                                }
                                LroStatus::Canceled => {
                                    return Err(Error::message(ErrorKind::Other, "Long running operation canceled".to_string()))
                                }
                                _ => {
                                    sleep(retry_after).await;
                                }
                            }
                        }
                    } else {
                        response.into_body().await
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
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
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
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
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
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
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
            pub async fn into_body(self) -> azure_core::Result<models::ProviderRegistrationArrayResponseWithContinuation> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ProviderRegistrationArrayResponseWithContinuation = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
        }
        impl RequestBuilder {
            pub fn into_stream(
                self,
            ) -> azure_core::Pageable<models::ProviderRegistrationArrayResponseWithContinuation, azure_core::error::Error> {
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
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
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations",
                    self.client.endpoint(),
                    &self.subscription_id
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
    }
    pub mod generate_operations {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<Vec<models::OperationsDefinition>> {
                let bytes = self.0.into_body().collect().await?;
                let body: Vec<models::OperationsDefinition> = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/generateOperations",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<Vec<models::OperationsDefinition>>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<Vec<models::OperationsDefinition>>>;
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
pub mod resource_type_registrations {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a resource type details in the given subscription and provider."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `resource_type`: The resource type."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            resource_type: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                resource_type: resource_type.into(),
            }
        }
        #[doc = "Creates or updates a resource type."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `resource_type`: The resource type."]
        #[doc = "* `properties`: The required request body parameters supplied to the resource type registration CreateOrUpdate operation."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            properties: impl Into<models::ResourceTypeRegistration>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                resource_type: resource_type.into(),
                properties: properties.into(),
            }
        }
        #[doc = "Deletes a resource type"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `resource_type`: The resource type."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            resource_type: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                resource_type: resource_type.into(),
            }
        }
        #[doc = "Gets the list of the resource types for the given provider."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        pub fn list_by_provider_registration(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
        ) -> list_by_provider_registration::RequestBuilder {
            list_by_provider_registration::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ResourceTypeRegistration> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ResourceTypeRegistration = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
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
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace,
                    &self.resource_type
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ResourceTypeRegistration>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ResourceTypeRegistration>>;
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
    pub mod create_or_update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ResourceTypeRegistration> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ResourceTypeRegistration = serde_json::from_slice(&bytes)?;
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
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) properties: models::ResourceTypeRegistration,
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
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.properties)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace,
                    &self.resource_type
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ResourceTypeRegistration>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ResourceTypeRegistration>>;
            #[doc = "Returns a future that polls the long running operation, returning once the operation completes."]
            #[doc = ""]
            #[doc = "To only submit the request but not monitor the status of the operation until completion, use `send()` instead."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move {
                    use azure_core::{
                        error::{Error, ErrorKind},
                        lro::{
                            get_retry_after,
                            location::{get_location, get_provisioning_state, FinalState},
                            LroStatus,
                        },
                        sleep::sleep,
                    };
                    use std::time::Duration;
                    let this = self.clone();
                    let response = this.send().await?;
                    let headers = response.as_raw_response().headers();
                    let location = get_location(headers, FinalState::AzureAsyncOperation)?;
                    if let Some(url) = location {
                        loop {
                            let mut req = azure_core::Request::new(url.clone(), azure_core::Method::Get);
                            let bearer_token = self.client.bearer_token().await?;
                            req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                            let response = self.client.send(&mut req).await?;
                            let headers = response.headers();
                            let retry_after = get_retry_after(headers);
                            let bytes = response.into_body().collect().await?;
                            let provisioning_state = get_provisioning_state(&bytes).ok_or_else(|| {
                                Error::message(
                                    ErrorKind::Other,
                                    "Long running operation failed (missing provisioning state)".to_string(),
                                )
                            })?;
                            log::trace!("current provisioning_state: {provisioning_state:?}");
                            match provisioning_state {
                                LroStatus::Succeeded => {
                                    let mut req = azure_core::Request::new(self.url()?, azure_core::Method::Get);
                                    let bearer_token = self.client.bearer_token().await?;
                                    req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                    let response = self.client.send(&mut req).await?;
                                    return Response(response).into_body().await;
                                }
                                LroStatus::Failed => {
                                    return Err(Error::message(ErrorKind::Other, "Long running operation failed".to_string()))
                                }
                                LroStatus::Canceled => {
                                    return Err(Error::message(ErrorKind::Other, "Long running operation canceled".to_string()))
                                }
                                _ => {
                                    sleep(retry_after).await;
                                }
                            }
                        }
                    } else {
                        response.into_body().await
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
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
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
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
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
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace,
                    &self.resource_type
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
    }
    pub mod list_by_provider_registration {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ResourceTypeRegistrationArrayResponseWithContinuation> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ResourceTypeRegistrationArrayResponseWithContinuation = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
        }
        impl RequestBuilder {
            pub fn into_stream(
                self,
            ) -> azure_core::Pageable<models::ResourceTypeRegistrationArrayResponseWithContinuation, azure_core::error::Error> {
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
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
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
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
        #[doc = "Gets the sku details for the given resource type and sku name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `resource_type`: The resource type."]
        #[doc = "* `sku`: The SKU."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            sku: impl Into<String>,
        ) -> get::RequestBuilder {
            get::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                resource_type: resource_type.into(),
                sku: sku.into(),
            }
        }
        #[doc = "Creates or updates the resource type skus in the given resource type."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `resource_type`: The resource type."]
        #[doc = "* `sku`: The SKU."]
        #[doc = "* `properties`: The required body parameters supplied to the resource sku operation."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            sku: impl Into<String>,
            properties: impl Into<models::SkuResource>,
        ) -> create_or_update::RequestBuilder {
            create_or_update::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                resource_type: resource_type.into(),
                sku: sku.into(),
                properties: properties.into(),
            }
        }
        #[doc = "Deletes a resource type sku."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `resource_type`: The resource type."]
        #[doc = "* `sku`: The SKU."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            sku: impl Into<String>,
        ) -> delete::RequestBuilder {
            delete::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                resource_type: resource_type.into(),
                sku: sku.into(),
            }
        }
        #[doc = "Gets the sku details for the given resource type and sku name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `resource_type`: The resource type."]
        #[doc = "* `nested_resource_type_first`: The first child resource type."]
        #[doc = "* `sku`: The SKU."]
        pub fn get_nested_resource_type_first(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            nested_resource_type_first: impl Into<String>,
            sku: impl Into<String>,
        ) -> get_nested_resource_type_first::RequestBuilder {
            get_nested_resource_type_first::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                resource_type: resource_type.into(),
                nested_resource_type_first: nested_resource_type_first.into(),
                sku: sku.into(),
            }
        }
        #[doc = "Creates or updates the resource type skus in the given resource type."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `resource_type`: The resource type."]
        #[doc = "* `nested_resource_type_first`: The first child resource type."]
        #[doc = "* `sku`: The SKU."]
        #[doc = "* `properties`: The required body parameters supplied to the resource sku operation."]
        pub fn create_or_update_nested_resource_type_first(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            nested_resource_type_first: impl Into<String>,
            sku: impl Into<String>,
            properties: impl Into<models::SkuResource>,
        ) -> create_or_update_nested_resource_type_first::RequestBuilder {
            create_or_update_nested_resource_type_first::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                resource_type: resource_type.into(),
                nested_resource_type_first: nested_resource_type_first.into(),
                sku: sku.into(),
                properties: properties.into(),
            }
        }
        #[doc = "Deletes a resource type sku."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `resource_type`: The resource type."]
        #[doc = "* `nested_resource_type_first`: The first child resource type."]
        #[doc = "* `sku`: The SKU."]
        pub fn delete_nested_resource_type_first(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            nested_resource_type_first: impl Into<String>,
            sku: impl Into<String>,
        ) -> delete_nested_resource_type_first::RequestBuilder {
            delete_nested_resource_type_first::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                resource_type: resource_type.into(),
                nested_resource_type_first: nested_resource_type_first.into(),
                sku: sku.into(),
            }
        }
        #[doc = "Gets the sku details for the given resource type and sku name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `resource_type`: The resource type."]
        #[doc = "* `nested_resource_type_first`: The first child resource type."]
        #[doc = "* `nested_resource_type_second`: The second child resource type."]
        #[doc = "* `sku`: The SKU."]
        pub fn get_nested_resource_type_second(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            nested_resource_type_first: impl Into<String>,
            nested_resource_type_second: impl Into<String>,
            sku: impl Into<String>,
        ) -> get_nested_resource_type_second::RequestBuilder {
            get_nested_resource_type_second::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                resource_type: resource_type.into(),
                nested_resource_type_first: nested_resource_type_first.into(),
                nested_resource_type_second: nested_resource_type_second.into(),
                sku: sku.into(),
            }
        }
        #[doc = "Creates or updates the resource type skus in the given resource type."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `resource_type`: The resource type."]
        #[doc = "* `nested_resource_type_first`: The first child resource type."]
        #[doc = "* `nested_resource_type_second`: The second child resource type."]
        #[doc = "* `sku`: The SKU."]
        #[doc = "* `properties`: The required body parameters supplied to the resource sku operation."]
        pub fn create_or_update_nested_resource_type_second(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            nested_resource_type_first: impl Into<String>,
            nested_resource_type_second: impl Into<String>,
            sku: impl Into<String>,
            properties: impl Into<models::SkuResource>,
        ) -> create_or_update_nested_resource_type_second::RequestBuilder {
            create_or_update_nested_resource_type_second::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                resource_type: resource_type.into(),
                nested_resource_type_first: nested_resource_type_first.into(),
                nested_resource_type_second: nested_resource_type_second.into(),
                sku: sku.into(),
                properties: properties.into(),
            }
        }
        #[doc = "Deletes a resource type sku."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `resource_type`: The resource type."]
        #[doc = "* `nested_resource_type_first`: The first child resource type."]
        #[doc = "* `nested_resource_type_second`: The second child resource type."]
        #[doc = "* `sku`: The SKU."]
        pub fn delete_nested_resource_type_second(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            nested_resource_type_first: impl Into<String>,
            nested_resource_type_second: impl Into<String>,
            sku: impl Into<String>,
        ) -> delete_nested_resource_type_second::RequestBuilder {
            delete_nested_resource_type_second::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                resource_type: resource_type.into(),
                nested_resource_type_first: nested_resource_type_first.into(),
                nested_resource_type_second: nested_resource_type_second.into(),
                sku: sku.into(),
            }
        }
        #[doc = "Gets the sku details for the given resource type and sku name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `resource_type`: The resource type."]
        #[doc = "* `nested_resource_type_first`: The first child resource type."]
        #[doc = "* `nested_resource_type_second`: The second child resource type."]
        #[doc = "* `nested_resource_type_third`: The third child resource type."]
        #[doc = "* `sku`: The SKU."]
        pub fn get_nested_resource_type_third(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            nested_resource_type_first: impl Into<String>,
            nested_resource_type_second: impl Into<String>,
            nested_resource_type_third: impl Into<String>,
            sku: impl Into<String>,
        ) -> get_nested_resource_type_third::RequestBuilder {
            get_nested_resource_type_third::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                resource_type: resource_type.into(),
                nested_resource_type_first: nested_resource_type_first.into(),
                nested_resource_type_second: nested_resource_type_second.into(),
                nested_resource_type_third: nested_resource_type_third.into(),
                sku: sku.into(),
            }
        }
        #[doc = "Creates or updates the resource type skus in the given resource type."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `resource_type`: The resource type."]
        #[doc = "* `nested_resource_type_first`: The first child resource type."]
        #[doc = "* `nested_resource_type_second`: The second child resource type."]
        #[doc = "* `nested_resource_type_third`: The third child resource type."]
        #[doc = "* `sku`: The SKU."]
        #[doc = "* `properties`: The required body parameters supplied to the resource sku operation."]
        pub fn create_or_update_nested_resource_type_third(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            nested_resource_type_first: impl Into<String>,
            nested_resource_type_second: impl Into<String>,
            nested_resource_type_third: impl Into<String>,
            sku: impl Into<String>,
            properties: impl Into<models::SkuResource>,
        ) -> create_or_update_nested_resource_type_third::RequestBuilder {
            create_or_update_nested_resource_type_third::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                resource_type: resource_type.into(),
                nested_resource_type_first: nested_resource_type_first.into(),
                nested_resource_type_second: nested_resource_type_second.into(),
                nested_resource_type_third: nested_resource_type_third.into(),
                sku: sku.into(),
                properties: properties.into(),
            }
        }
        #[doc = "Deletes a resource type sku."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `resource_type`: The resource type."]
        #[doc = "* `nested_resource_type_first`: The first child resource type."]
        #[doc = "* `nested_resource_type_second`: The second child resource type."]
        #[doc = "* `nested_resource_type_third`: The third child resource type."]
        #[doc = "* `sku`: The SKU."]
        pub fn delete_nested_resource_type_third(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            nested_resource_type_first: impl Into<String>,
            nested_resource_type_second: impl Into<String>,
            nested_resource_type_third: impl Into<String>,
            sku: impl Into<String>,
        ) -> delete_nested_resource_type_third::RequestBuilder {
            delete_nested_resource_type_third::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                resource_type: resource_type.into(),
                nested_resource_type_first: nested_resource_type_first.into(),
                nested_resource_type_second: nested_resource_type_second.into(),
                nested_resource_type_third: nested_resource_type_third.into(),
                sku: sku.into(),
            }
        }
        #[doc = "Gets the list of skus for the given resource type."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `resource_type`: The resource type."]
        pub fn list_by_resource_type_registrations(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            resource_type: impl Into<String>,
        ) -> list_by_resource_type_registrations::RequestBuilder {
            list_by_resource_type_registrations::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                resource_type: resource_type.into(),
            }
        }
        #[doc = "Gets the list of skus for the given resource type."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `resource_type`: The resource type."]
        #[doc = "* `nested_resource_type_first`: The first child resource type."]
        pub fn list_by_resource_type_registrations_nested_resource_type_first(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            nested_resource_type_first: impl Into<String>,
        ) -> list_by_resource_type_registrations_nested_resource_type_first::RequestBuilder {
            list_by_resource_type_registrations_nested_resource_type_first::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                resource_type: resource_type.into(),
                nested_resource_type_first: nested_resource_type_first.into(),
            }
        }
        #[doc = "Gets the list of skus for the given resource type."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `resource_type`: The resource type."]
        #[doc = "* `nested_resource_type_first`: The first child resource type."]
        #[doc = "* `nested_resource_type_second`: The second child resource type."]
        pub fn list_by_resource_type_registrations_nested_resource_type_second(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            nested_resource_type_first: impl Into<String>,
            nested_resource_type_second: impl Into<String>,
        ) -> list_by_resource_type_registrations_nested_resource_type_second::RequestBuilder {
            list_by_resource_type_registrations_nested_resource_type_second::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                resource_type: resource_type.into(),
                nested_resource_type_first: nested_resource_type_first.into(),
                nested_resource_type_second: nested_resource_type_second.into(),
            }
        }
        #[doc = "Gets the list of skus for the given resource type."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `resource_type`: The resource type."]
        #[doc = "* `nested_resource_type_first`: The first child resource type."]
        #[doc = "* `nested_resource_type_second`: The second child resource type."]
        #[doc = "* `nested_resource_type_third`: The third child resource type."]
        pub fn list_by_resource_type_registrations_nested_resource_type_third(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            resource_type: impl Into<String>,
            nested_resource_type_first: impl Into<String>,
            nested_resource_type_second: impl Into<String>,
            nested_resource_type_third: impl Into<String>,
        ) -> list_by_resource_type_registrations_nested_resource_type_third::RequestBuilder {
            list_by_resource_type_registrations_nested_resource_type_third::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                resource_type: resource_type.into(),
                nested_resource_type_first: nested_resource_type_first.into(),
                nested_resource_type_second: nested_resource_type_second.into(),
                nested_resource_type_third: nested_resource_type_third.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SkuResource> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SkuResource = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) sku: String,
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
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/skus/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace,
                    &self.resource_type,
                    &self.sku
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SkuResource>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SkuResource>>;
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
    pub mod create_or_update {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SkuResource> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SkuResource = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) sku: String,
            pub(crate) properties: models::SkuResource,
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
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.properties)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/skus/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace,
                    &self.resource_type,
                    &self.sku
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SkuResource>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SkuResource>>;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) sku: String,
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
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
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
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/skus/{}",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace,
                    &self.resource_type,
                    &self.sku
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
    }
    pub mod get_nested_resource_type_first {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SkuResource> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SkuResource = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) nested_resource_type_first: String,
            pub(crate) sku: String,
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
                let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus/{}" , self . client . endpoint () , & self . subscription_id , & self . provider_namespace , & self . resource_type , & self . nested_resource_type_first , & self . sku)) ? ;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SkuResource>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SkuResource>>;
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
    pub mod create_or_update_nested_resource_type_first {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SkuResource> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SkuResource = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) nested_resource_type_first: String,
            pub(crate) sku: String,
            pub(crate) properties: models::SkuResource,
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
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.properties)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus/{}" , self . client . endpoint () , & self . subscription_id , & self . provider_namespace , & self . resource_type , & self . nested_resource_type_first , & self . sku)) ? ;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SkuResource>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SkuResource>>;
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
    pub mod delete_nested_resource_type_first {
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) nested_resource_type_first: String,
            pub(crate) sku: String,
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
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus/{}" , self . client . endpoint () , & self . subscription_id , & self . provider_namespace , & self . resource_type , & self . nested_resource_type_first , & self . sku)) ? ;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
    }
    pub mod get_nested_resource_type_second {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SkuResource> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SkuResource = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) nested_resource_type_first: String,
            pub(crate) nested_resource_type_second: String,
            pub(crate) sku: String,
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
                let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus/{}" , self . client . endpoint () , & self . subscription_id , & self . provider_namespace , & self . resource_type , & self . nested_resource_type_first , & self . nested_resource_type_second , & self . sku)) ? ;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SkuResource>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SkuResource>>;
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
    pub mod create_or_update_nested_resource_type_second {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SkuResource> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SkuResource = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) nested_resource_type_first: String,
            pub(crate) nested_resource_type_second: String,
            pub(crate) sku: String,
            pub(crate) properties: models::SkuResource,
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
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.properties)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus/{}" , self . client . endpoint () , & self . subscription_id , & self . provider_namespace , & self . resource_type , & self . nested_resource_type_first , & self . nested_resource_type_second , & self . sku)) ? ;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SkuResource>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SkuResource>>;
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
    pub mod delete_nested_resource_type_second {
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) nested_resource_type_first: String,
            pub(crate) nested_resource_type_second: String,
            pub(crate) sku: String,
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
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus/{}" , self . client . endpoint () , & self . subscription_id , & self . provider_namespace , & self . resource_type , & self . nested_resource_type_first , & self . nested_resource_type_second , & self . sku)) ? ;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
    }
    pub mod get_nested_resource_type_third {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SkuResource> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SkuResource = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) nested_resource_type_first: String,
            pub(crate) nested_resource_type_second: String,
            pub(crate) nested_resource_type_third: String,
            pub(crate) sku: String,
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
                let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus/{}" , self . client . endpoint () , & self . subscription_id , & self . provider_namespace , & self . resource_type , & self . nested_resource_type_first , & self . nested_resource_type_second , & self . nested_resource_type_third , & self . sku)) ? ;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SkuResource>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SkuResource>>;
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
    pub mod create_or_update_nested_resource_type_third {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SkuResource> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SkuResource = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) nested_resource_type_first: String,
            pub(crate) nested_resource_type_second: String,
            pub(crate) nested_resource_type_third: String,
            pub(crate) sku: String,
            pub(crate) properties: models::SkuResource,
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
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.properties)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus/{}" , self . client . endpoint () , & self . subscription_id , & self . provider_namespace , & self . resource_type , & self . nested_resource_type_first , & self . nested_resource_type_second , & self . nested_resource_type_third , & self . sku)) ? ;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::SkuResource>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::SkuResource>>;
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
    pub mod delete_nested_resource_type_third {
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) nested_resource_type_first: String,
            pub(crate) nested_resource_type_second: String,
            pub(crate) nested_resource_type_third: String,
            pub(crate) sku: String,
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
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let bearer_token = this.client.bearer_token().await?;
                        req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus/{}" , self . client . endpoint () , & self . subscription_id , & self . provider_namespace , & self . resource_type , & self . nested_resource_type_first , & self . nested_resource_type_second , & self . nested_resource_type_third , & self . sku)) ? ;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
    }
    pub mod list_by_resource_type_registrations {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SkuResourceArrayResponseWithContinuation> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SkuResourceArrayResponseWithContinuation = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::SkuResourceArrayResponseWithContinuation, azure_core::error::Error> {
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
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
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/skus",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace,
                    &self.resource_type
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
    }
    pub mod list_by_resource_type_registrations_nested_resource_type_first {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SkuResourceArrayResponseWithContinuation> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SkuResourceArrayResponseWithContinuation = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) nested_resource_type_first: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::SkuResourceArrayResponseWithContinuation, azure_core::error::Error> {
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
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
                let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus" , self . client . endpoint () , & self . subscription_id , & self . provider_namespace , & self . resource_type , & self . nested_resource_type_first)) ? ;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
    }
    pub mod list_by_resource_type_registrations_nested_resource_type_second {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SkuResourceArrayResponseWithContinuation> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SkuResourceArrayResponseWithContinuation = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) nested_resource_type_first: String,
            pub(crate) nested_resource_type_second: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::SkuResourceArrayResponseWithContinuation, azure_core::error::Error> {
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
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
                let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus" , self . client . endpoint () , & self . subscription_id , & self . provider_namespace , & self . resource_type , & self . nested_resource_type_first , & self . nested_resource_type_second)) ? ;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
    }
    pub mod list_by_resource_type_registrations_nested_resource_type_third {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::SkuResourceArrayResponseWithContinuation> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::SkuResourceArrayResponseWithContinuation = serde_json::from_slice(&bytes)?;
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
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) nested_resource_type_first: String,
            pub(crate) nested_resource_type_second: String,
            pub(crate) nested_resource_type_third: String,
        }
        impl RequestBuilder {
            pub fn into_stream(self) -> azure_core::Pageable<models::SkuResourceArrayResponseWithContinuation, azure_core::error::Error> {
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                                let bearer_token = this.client.bearer_token().await?;
                                req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
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
                let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus" , self . client . endpoint () , & self . subscription_id , & self . provider_namespace , & self . resource_type , & self . nested_resource_type_first , & self . nested_resource_type_second , & self . nested_resource_type_third)) ? ;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
    }
}
pub mod resource_actions {
    use super::models;
    #[cfg(not(target_arch = "wasm32"))]
    use futures::future::BoxFuture;
    #[cfg(target_arch = "wasm32")]
    use futures::future::LocalBoxFuture as BoxFuture;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Deletes resources."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        #[doc = "* `resource_action_name`: The resource action name."]
        #[doc = "* `properties`: The properties supplied to the DeleteResources operation."]
        pub fn delete_resources(
            &self,
            subscription_id: impl Into<String>,
            provider_namespace: impl Into<String>,
            resource_action_name: impl Into<String>,
            properties: impl Into<models::ResourceManagementAction>,
        ) -> delete_resources::RequestBuilder {
            delete_resources::RequestBuilder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                resource_action_name: resource_action_name.into(),
                properties: properties.into(),
            }
        }
    }
    pub mod delete_resources {
        use super::models;
        #[cfg(not(target_arch = "wasm32"))]
        use futures::future::BoxFuture;
        #[cfg(target_arch = "wasm32")]
        use futures::future::LocalBoxFuture as BoxFuture;
        #[derive(Debug)]
        pub struct Response(azure_core::Response);
        impl Response {
            pub async fn into_body(self) -> azure_core::Result<models::ResourceManagementAction> {
                let bytes = self.0.into_body().collect().await?;
                let body: models::ResourceManagementAction = serde_json::from_slice(&bytes)?;
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
        #[doc = r" This `RequestBuilder` implements a Long Running Operation"]
        #[doc = r" (LRO)."]
        #[doc = r""]
        #[doc = r" To finalize and submit the request, invoke `.await`, which"]
        #[doc = r" which will convert the `RequestBuilder` into a future"]
        #[doc = r" executes the request and polls the service until the"]
        #[doc = r" operation completes."]
        #[doc = r""]
        #[doc = r" In order to execute the request without polling the service"]
        #[doc = r" until the operation completes, use"]
        #[doc = r" [`RequestBuilder::send()`], which will return a lower-level"]
        #[doc = r" [`Response`] value."]
        pub struct RequestBuilder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_action_name: String,
            pub(crate) properties: models::ResourceManagementAction,
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
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.properties)?;
                        req.set_body(req_body);
                        Ok(Response(this.client.send(&mut req).await?))
                    }
                })
            }
            fn url(&self) -> azure_core::Result<azure_core::Url> {
                let mut url = azure_core::Url::parse(&format!(
                    "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourceActions/{}/deleteResources",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.provider_namespace,
                    &self.resource_action_name
                ))?;
                let has_api_version_already = url.query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                if !has_api_version_already {
                    url.query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                }
                Ok(url)
            }
        }
        impl std::future::IntoFuture for RequestBuilder {
            type Output = azure_core::Result<models::ResourceManagementAction>;
            type IntoFuture = BoxFuture<'static, azure_core::Result<models::ResourceManagementAction>>;
            #[doc = "Returns a future that polls the long running operation, returning once the operation completes."]
            #[doc = ""]
            #[doc = "To only submit the request but not monitor the status of the operation until completion, use `send()` instead."]
            #[doc = ""]
            #[doc = "You should not normally call this method directly, simply invoke `.await` which implicitly calls `IntoFuture::into_future`."]
            #[doc = ""]
            #[doc = "See [IntoFuture documentation](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) for more details."]
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(async move {
                    use azure_core::{
                        error::{Error, ErrorKind},
                        lro::{
                            get_retry_after,
                            location::{get_location, get_provisioning_state, FinalState},
                            LroStatus,
                        },
                        sleep::sleep,
                    };
                    use std::time::Duration;
                    let this = self.clone();
                    let response = this.send().await?;
                    let headers = response.as_raw_response().headers();
                    let location = get_location(headers, FinalState::AzureAsyncOperation)?;
                    if let Some(url) = location {
                        loop {
                            let mut req = azure_core::Request::new(url.clone(), azure_core::Method::Get);
                            let bearer_token = self.client.bearer_token().await?;
                            req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                            let response = self.client.send(&mut req).await?;
                            let headers = response.headers();
                            let retry_after = get_retry_after(headers);
                            let bytes = response.into_body().collect().await?;
                            let provisioning_state = get_provisioning_state(&bytes).ok_or_else(|| {
                                Error::message(
                                    ErrorKind::Other,
                                    "Long running operation failed (missing provisioning state)".to_string(),
                                )
                            })?;
                            log::trace!("current provisioning_state: {provisioning_state:?}");
                            match provisioning_state {
                                LroStatus::Succeeded => {
                                    let mut req = azure_core::Request::new(self.url()?, azure_core::Method::Get);
                                    let bearer_token = self.client.bearer_token().await?;
                                    req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", bearer_token.secret()));
                                    let response = self.client.send(&mut req).await?;
                                    return Response(response).into_body().await;
                                }
                                LroStatus::Failed => {
                                    return Err(Error::message(ErrorKind::Other, "Long running operation failed".to_string()))
                                }
                                LroStatus::Canceled => {
                                    return Err(Error::message(ErrorKind::Other, "Long running operation canceled".to_string()))
                                }
                                _ => {
                                    sleep(retry_after).await;
                                }
                            }
                        }
                    } else {
                        response.into_body().await
                    }
                })
            }
        }
    }
}
