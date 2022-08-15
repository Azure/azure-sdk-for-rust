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
        ) -> get::Builder {
            get::Builder {
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
        ) -> create_or_update::Builder {
            create_or_update::Builder {
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
        ) -> list_by_provider_registration::Builder {
            list_by_provider_registration::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::CustomRollout;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) rollout_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/customRollouts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.provider_namespace,
                            &this.rollout_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CustomRollout = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update {
        use super::models;
        type Response = models::CustomRollout;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) rollout_name: String,
            pub(crate) properties: models::CustomRollout,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/customRollouts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.provider_namespace,
                            &this.rollout_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.properties)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CustomRollout = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_by_provider_registration {
        use super::models;
        type Response = models::CustomRolloutArrayResponseWithContinuation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/customRollouts",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.provider_namespace
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CustomRolloutArrayResponseWithContinuation = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
}
pub mod default_rollouts {
    use super::models;
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
        ) -> get::Builder {
            get::Builder {
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
        ) -> create_or_update::Builder {
            create_or_update::Builder {
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
        ) -> delete::Builder {
            delete::Builder {
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
        ) -> list_by_provider_registration::Builder {
            list_by_provider_registration::Builder {
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
        ) -> stop::Builder {
            stop::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
                rollout_name: rollout_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::DefaultRollout;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) rollout_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/defaultRollouts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.provider_namespace,
                            &this.rollout_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DefaultRollout = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::DefaultRollout),
            Created201(models::DefaultRollout),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) rollout_name: String,
            pub(crate) properties: models::DefaultRollout,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/defaultRollouts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.provider_namespace,
                            &this.rollout_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.properties)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DefaultRollout = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DefaultRollout = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
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
    pub mod delete {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) rollout_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/defaultRollouts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.provider_namespace,
                            &this.rollout_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::NoContent => Ok(Response::NoContent204),
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
    pub mod list_by_provider_registration {
        use super::models;
        type Response = models::DefaultRolloutArrayResponseWithContinuation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/defaultRollouts",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.provider_namespace
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DefaultRolloutArrayResponseWithContinuation = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod stop {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) rollout_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/defaultRollouts/{}/stop",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.provider_namespace,
                            &this.rollout_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(()),
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
    ) -> generate_manifest::Builder {
        generate_manifest::Builder {
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
    ) -> checkin_manifest::Builder {
        checkin_manifest::Builder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            provider_namespace: provider_namespace.into(),
            checkin_manifest_params: checkin_manifest_params.into(),
        }
    }
}
pub mod generate_manifest {
    use super::models;
    type Response = models::ResourceProviderManifest;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) provider_namespace: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/generateManifest",
                        this.client.endpoint(),
                        &this.subscription_id,
                        &this.provider_namespace
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
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::ResourceProviderManifest = serde_json::from_slice(&rsp_body)?;
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
pub mod checkin_manifest {
    use super::models;
    type Response = models::CheckinManifestInfo;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) provider_namespace: String,
        pub(crate) checkin_manifest_params: models::CheckinManifestParams,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/checkinManifest",
                        this.client.endpoint(),
                        &this.subscription_id,
                        &this.provider_namespace
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
                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.checkin_manifest_params)?;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::CheckinManifestInfo = serde_json::from_slice(&rsp_body)?;
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
pub mod notification_registrations {
    use super::models;
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
        ) -> get::Builder {
            get::Builder {
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
        ) -> create_or_update::Builder {
            create_or_update::Builder {
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
        ) -> delete::Builder {
            delete::Builder {
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
        ) -> list_by_provider_registration::Builder {
            list_by_provider_registration::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::NotificationRegistration;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) notification_registration_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/notificationRegistrations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.provider_namespace,
                            &this.notification_registration_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NotificationRegistration = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update {
        use super::models;
        type Response = models::NotificationRegistration;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) notification_registration_name: String,
            pub(crate) properties: models::NotificationRegistration,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/notificationRegistrations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.provider_namespace,
                            &this.notification_registration_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.properties)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NotificationRegistration = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) notification_registration_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/notificationRegistrations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.provider_namespace,
                            &this.notification_registration_name
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::NoContent => Ok(Response::NoContent204),
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
    pub mod list_by_provider_registration {
        use super::models;
        type Response = models::NotificationRegistrationArrayResponseWithContinuation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/notificationRegistrations",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.provider_namespace
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NotificationRegistrationArrayResponseWithContinuation =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
}
pub mod operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all the operations supported by Microsoft.ProviderHub."]
        pub fn list(&self) -> list::Builder {
            list::Builder { client: self.0.clone() }
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
        ) -> list_by_provider_registration::Builder {
            list_by_provider_registration::Builder {
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
        ) -> create_or_update::Builder {
            create_or_update::Builder {
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
        pub fn delete(&self, subscription_id: impl Into<String>, provider_namespace: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::OperationsDefinitionArrayResponseWithContinuation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url =
                            azure_core::Url::parse(&format!("{}/providers/Microsoft.ProviderHub/operations", this.client.endpoint(),))?;
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
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::OperationsDefinitionArrayResponseWithContinuation =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod list_by_provider_registration {
        use super::models;
        type Response = Vec<models::OperationsDefinition>;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/operations/default",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.provider_namespace
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: Vec<models::OperationsDefinition> = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update {
        use super::models;
        type Response = models::OperationsPutContent;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) operations_put_content: models::OperationsPutContent,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/operations/default",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.provider_namespace
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.operations_put_content)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::OperationsPutContent = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/operations/default",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.provider_namespace
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::NoContent => Ok(Response::NoContent204),
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
pub mod provider_registrations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the provider registration details."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `provider_namespace`: The name of the resource provider hosted within ProviderHub."]
        pub fn get(&self, subscription_id: impl Into<String>, provider_namespace: impl Into<String>) -> get::Builder {
            get::Builder {
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
        ) -> create_or_update::Builder {
            create_or_update::Builder {
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
        pub fn delete(&self, subscription_id: impl Into<String>, provider_namespace: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
            }
        }
        #[doc = "Gets the list of the provider registrations in the subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        pub fn list(&self, subscription_id: impl Into<String>) -> list::Builder {
            list::Builder {
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
        ) -> generate_operations::Builder {
            generate_operations::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::ProviderRegistration;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.provider_namespace
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProviderRegistration = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ProviderRegistration),
            Created201,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) properties: models::ProviderRegistration,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.provider_namespace
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.properties)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProviderRegistration = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => Ok(Response::Created201),
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
    pub mod delete {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.provider_namespace
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::NoContent => Ok(Response::NoContent204),
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
    pub mod list {
        use super::models;
        type Response = models::ProviderRegistrationArrayResponseWithContinuation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProviderRegistrationArrayResponseWithContinuation =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod generate_operations {
        use super::models;
        type Response = Vec<models::OperationsDefinition>;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/generateOperations",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.provider_namespace
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: Vec<models::OperationsDefinition> = serde_json::from_slice(&rsp_body)?;
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
pub mod resource_type_registrations {
    use super::models;
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
        ) -> get::Builder {
            get::Builder {
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
        ) -> create_or_update::Builder {
            create_or_update::Builder {
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
        ) -> delete::Builder {
            delete::Builder {
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
        ) -> list_by_provider_registration::Builder {
            list_by_provider_registration::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                provider_namespace: provider_namespace.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::ResourceTypeRegistration;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.provider_namespace,
                            &this.resource_type
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ResourceTypeRegistration = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ResourceTypeRegistration),
            Created201(models::ResourceTypeRegistration),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) properties: models::ResourceTypeRegistration,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.provider_namespace,
                            &this.resource_type
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.properties)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ResourceTypeRegistration = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ResourceTypeRegistration = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
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
    pub mod delete {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.provider_namespace,
                            &this.resource_type
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::NoContent => Ok(Response::NoContent204),
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
    pub mod list_by_provider_registration {
        use super::models;
        type Response = models::ResourceTypeRegistrationArrayResponseWithContinuation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.provider_namespace
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ResourceTypeRegistrationArrayResponseWithContinuation =
                                    serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
}
pub mod skus {
    use super::models;
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
        ) -> get::Builder {
            get::Builder {
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
        ) -> create_or_update::Builder {
            create_or_update::Builder {
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
        ) -> delete::Builder {
            delete::Builder {
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
        ) -> get_nested_resource_type_first::Builder {
            get_nested_resource_type_first::Builder {
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
        ) -> create_or_update_nested_resource_type_first::Builder {
            create_or_update_nested_resource_type_first::Builder {
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
        ) -> delete_nested_resource_type_first::Builder {
            delete_nested_resource_type_first::Builder {
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
        ) -> get_nested_resource_type_second::Builder {
            get_nested_resource_type_second::Builder {
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
        ) -> create_or_update_nested_resource_type_second::Builder {
            create_or_update_nested_resource_type_second::Builder {
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
        ) -> delete_nested_resource_type_second::Builder {
            delete_nested_resource_type_second::Builder {
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
        ) -> get_nested_resource_type_third::Builder {
            get_nested_resource_type_third::Builder {
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
        ) -> create_or_update_nested_resource_type_third::Builder {
            create_or_update_nested_resource_type_third::Builder {
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
        ) -> delete_nested_resource_type_third::Builder {
            delete_nested_resource_type_third::Builder {
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
        ) -> list_by_resource_type_registrations::Builder {
            list_by_resource_type_registrations::Builder {
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
        ) -> list_by_resource_type_registrations_nested_resource_type_first::Builder {
            list_by_resource_type_registrations_nested_resource_type_first::Builder {
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
        ) -> list_by_resource_type_registrations_nested_resource_type_second::Builder {
            list_by_resource_type_registrations_nested_resource_type_second::Builder {
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
        ) -> list_by_resource_type_registrations_nested_resource_type_third::Builder {
            list_by_resource_type_registrations_nested_resource_type_third::Builder {
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
        type Response = models::SkuResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) sku: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/skus/{}" , this . client . endpoint () , & this . subscription_id , & this . provider_namespace , & this . resource_type , & this . sku)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SkuResource = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update {
        use super::models;
        type Response = models::SkuResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) sku: String,
            pub(crate) properties: models::SkuResource,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/skus/{}" , this . client . endpoint () , & this . subscription_id , & this . provider_namespace , & this . resource_type , & this . sku)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.properties)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SkuResource = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) sku: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/skus/{}" , this . client . endpoint () , & this . subscription_id , & this . provider_namespace , & this . resource_type , & this . sku)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::NoContent => Ok(Response::NoContent204),
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
    pub mod get_nested_resource_type_first {
        use super::models;
        type Response = models::SkuResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) nested_resource_type_first: String,
            pub(crate) sku: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus/{}" , this . client . endpoint () , & this . subscription_id , & this . provider_namespace , & this . resource_type , & this . nested_resource_type_first , & this . sku)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SkuResource = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update_nested_resource_type_first {
        use super::models;
        type Response = models::SkuResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) nested_resource_type_first: String,
            pub(crate) sku: String,
            pub(crate) properties: models::SkuResource,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus/{}" , this . client . endpoint () , & this . subscription_id , & this . provider_namespace , & this . resource_type , & this . nested_resource_type_first , & this . sku)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.properties)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SkuResource = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete_nested_resource_type_first {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) nested_resource_type_first: String,
            pub(crate) sku: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus/{}" , this . client . endpoint () , & this . subscription_id , & this . provider_namespace , & this . resource_type , & this . nested_resource_type_first , & this . sku)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::NoContent => Ok(Response::NoContent204),
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
    pub mod get_nested_resource_type_second {
        use super::models;
        type Response = models::SkuResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) nested_resource_type_first: String,
            pub(crate) nested_resource_type_second: String,
            pub(crate) sku: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus/{}" , this . client . endpoint () , & this . subscription_id , & this . provider_namespace , & this . resource_type , & this . nested_resource_type_first , & this . nested_resource_type_second , & this . sku)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SkuResource = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update_nested_resource_type_second {
        use super::models;
        type Response = models::SkuResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) nested_resource_type_first: String,
            pub(crate) nested_resource_type_second: String,
            pub(crate) sku: String,
            pub(crate) properties: models::SkuResource,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus/{}" , this . client . endpoint () , & this . subscription_id , & this . provider_namespace , & this . resource_type , & this . nested_resource_type_first , & this . nested_resource_type_second , & this . sku)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.properties)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SkuResource = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete_nested_resource_type_second {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) nested_resource_type_first: String,
            pub(crate) nested_resource_type_second: String,
            pub(crate) sku: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus/{}" , this . client . endpoint () , & this . subscription_id , & this . provider_namespace , & this . resource_type , & this . nested_resource_type_first , & this . nested_resource_type_second , & this . sku)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::NoContent => Ok(Response::NoContent204),
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
    pub mod get_nested_resource_type_third {
        use super::models;
        type Response = models::SkuResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) nested_resource_type_first: String,
            pub(crate) nested_resource_type_second: String,
            pub(crate) nested_resource_type_third: String,
            pub(crate) sku: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus/{}" , this . client . endpoint () , & this . subscription_id , & this . provider_namespace , & this . resource_type , & this . nested_resource_type_first , & this . nested_resource_type_second , & this . nested_resource_type_third , & this . sku)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SkuResource = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_or_update_nested_resource_type_third {
        use super::models;
        type Response = models::SkuResource;
        #[derive(Clone)]
        pub struct Builder {
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
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus/{}" , this . client . endpoint () , & this . subscription_id , & this . provider_namespace , & this . resource_type , & this . nested_resource_type_first , & this . nested_resource_type_second , & this . nested_resource_type_third , & this . sku)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.properties)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SkuResource = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete_nested_resource_type_third {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) nested_resource_type_first: String,
            pub(crate) nested_resource_type_second: String,
            pub(crate) nested_resource_type_third: String,
            pub(crate) sku: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus/{}" , this . client . endpoint () , & this . subscription_id , & this . provider_namespace , & this . resource_type , & this . nested_resource_type_first , & this . nested_resource_type_second , & this . nested_resource_type_third , & this . sku)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::NoContent => Ok(Response::NoContent204),
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
    pub mod list_by_resource_type_registrations {
        use super::models;
        type Response = models::SkuResourceArrayResponseWithContinuation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/skus" , this . client . endpoint () , & this . subscription_id , & this . provider_namespace , & this . resource_type)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SkuResourceArrayResponseWithContinuation = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod list_by_resource_type_registrations_nested_resource_type_first {
        use super::models;
        type Response = models::SkuResourceArrayResponseWithContinuation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) nested_resource_type_first: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus" , this . client . endpoint () , & this . subscription_id , & this . provider_namespace , & this . resource_type , & this . nested_resource_type_first)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SkuResourceArrayResponseWithContinuation = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod list_by_resource_type_registrations_nested_resource_type_second {
        use super::models;
        type Response = models::SkuResourceArrayResponseWithContinuation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) nested_resource_type_first: String,
            pub(crate) nested_resource_type_second: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus" , this . client . endpoint () , & this . subscription_id , & this . provider_namespace , & this . resource_type , & this . nested_resource_type_first , & this . nested_resource_type_second)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SkuResourceArrayResponseWithContinuation = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
    pub mod list_by_resource_type_registrations_nested_resource_type_third {
        use super::models;
        type Response = models::SkuResourceArrayResponseWithContinuation;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_type: String,
            pub(crate) nested_resource_type_first: String,
            pub(crate) nested_resource_type_second: String,
            pub(crate) nested_resource_type_third: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/resourcetypeRegistrations/{}/skus" , this . client . endpoint () , & this . subscription_id , & this . provider_namespace , & this . resource_type , & this . nested_resource_type_first , & this . nested_resource_type_second , & this . nested_resource_type_third)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SkuResourceArrayResponseWithContinuation = serde_json::from_slice(&rsp_body)?;
                                Ok(rsp_value)
                            }
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
}
pub mod resource_actions {
    use super::models;
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
        ) -> delete_resources::Builder {
            delete_resources::Builder {
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
        type Response = models::ResourceManagementAction;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) provider_namespace: String,
            pub(crate) resource_action_name: String,
            pub(crate) properties: models::ResourceManagementAction,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/providers/Microsoft.ProviderHub/providerRegistrations/{}/resourceActions/{}/deleteResources" , this . client . endpoint () , & this . subscription_id , & this . provider_namespace , & this . resource_action_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.properties)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ResourceManagementAction = serde_json::from_slice(&rsp_body)?;
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
