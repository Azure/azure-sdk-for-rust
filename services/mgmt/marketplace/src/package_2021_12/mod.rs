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
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn private_store_client(&self) -> private_store::Client {
        private_store::Client(self.clone())
    }
    pub fn private_store_collection_client(&self) -> private_store_collection::Client {
        private_store_collection::Client(self.clone())
    }
    pub fn private_store_collection_offer_client(&self) -> private_store_collection_offer::Client {
        private_store_collection_offer::Client(self.clone())
    }
}
pub mod private_store {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of available private stores."]
        pub fn list(&self) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                use_cache: None,
            }
        }
        #[doc = "Get information about the private store"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        pub fn get(&self, private_store_id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
            }
        }
        #[doc = "Changes private store properties"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        pub fn create_or_update(&self, private_store_id: impl Into<String>) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
                payload: None,
            }
        }
        #[doc = "Deletes the private store. All that is not saved will be lost."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        pub fn delete(&self, private_store_id: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
            }
        }
        #[doc = "List of offers, regardless the collections"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        pub fn query_offers(&self, private_store_id: impl Into<String>) -> query_offers::Builder {
            query_offers::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
            }
        }
        #[doc = "Tenant billing accounts names"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        pub fn billing_accounts(&self, private_store_id: impl Into<String>) -> billing_accounts::Builder {
            billing_accounts::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
            }
        }
        #[doc = "For a given subscriptions list, the API will return a map of collections and the related subscriptions from the supplied list."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        pub fn collections_to_subscriptions_mapping(
            &self,
            private_store_id: impl Into<String>,
        ) -> collections_to_subscriptions_mapping::Builder {
            collections_to_subscriptions_mapping::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
                payload: None,
            }
        }
        #[doc = "Get map of plans and related approved subscriptions."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        pub fn query_approved_plans(&self, private_store_id: impl Into<String>) -> query_approved_plans::Builder {
            query_approved_plans::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
                payload: None,
            }
        }
        #[doc = "Perform an action on bulk collections"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        pub fn bulk_collections_action(&self, private_store_id: impl Into<String>) -> bulk_collections_action::Builder {
            bulk_collections_action::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
                payload: None,
            }
        }
        #[doc = "Get all open approval requests of current user"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        pub fn get_approval_requests_list(&self, private_store_id: impl Into<String>) -> get_approval_requests_list::Builder {
            get_approval_requests_list::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
            }
        }
        #[doc = "Get open request approval details"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        #[doc = "* `request_approval_id`: The request approval ID to get create or update"]
        pub fn get_request_approval(
            &self,
            private_store_id: impl Into<String>,
            request_approval_id: impl Into<String>,
        ) -> get_request_approval::Builder {
            get_request_approval::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
                request_approval_id: request_approval_id.into(),
            }
        }
        #[doc = "Create approval request"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        #[doc = "* `request_approval_id`: The request approval ID to get create or update"]
        pub fn create_approval_request(
            &self,
            private_store_id: impl Into<String>,
            request_approval_id: impl Into<String>,
        ) -> create_approval_request::Builder {
            create_approval_request::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
                request_approval_id: request_approval_id.into(),
                payload: None,
            }
        }
        #[doc = "Get request statuses foreach plan, this api is used as a complex GET action."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        #[doc = "* `request_approval_id`: The request approval ID to get create or update"]
        pub fn query_request_approval(
            &self,
            private_store_id: impl Into<String>,
            request_approval_id: impl Into<String>,
        ) -> query_request_approval::Builder {
            query_request_approval::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
                request_approval_id: request_approval_id.into(),
                payload: None,
            }
        }
        #[doc = "Get list of admin request approvals"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        pub fn admin_request_approvals_list(&self, private_store_id: impl Into<String>) -> admin_request_approvals_list::Builder {
            admin_request_approvals_list::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
            }
        }
        #[doc = "Get open approval requests"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        #[doc = "* `admin_request_approval_id`: The admin request approval ID to get create or update"]
        #[doc = "* `publisher_id`: The publisher id of this offer."]
        pub fn get_admin_request_approval(
            &self,
            private_store_id: impl Into<String>,
            admin_request_approval_id: impl Into<String>,
            publisher_id: impl Into<String>,
        ) -> get_admin_request_approval::Builder {
            get_admin_request_approval::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
                admin_request_approval_id: admin_request_approval_id.into(),
                publisher_id: publisher_id.into(),
            }
        }
        #[doc = "Update the admin action, weather the request is approved or rejected and the approved plans"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        #[doc = "* `admin_request_approval_id`: The admin request approval ID to get create or update"]
        pub fn update_admin_request_approval(
            &self,
            private_store_id: impl Into<String>,
            admin_request_approval_id: impl Into<String>,
        ) -> update_admin_request_approval::Builder {
            update_admin_request_approval::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
                admin_request_approval_id: admin_request_approval_id.into(),
                payload: None,
            }
        }
        #[doc = "Get private store notifications state"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        pub fn query_notifications_state(&self, private_store_id: impl Into<String>) -> query_notifications_state::Builder {
            query_notifications_state::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
            }
        }
        #[doc = "Acknowledge notification for offer"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        #[doc = "* `offer_id`: The offer ID to update or delete"]
        pub fn acknowledge_offer_notification(
            &self,
            private_store_id: impl Into<String>,
            offer_id: impl Into<String>,
        ) -> acknowledge_offer_notification::Builder {
            acknowledge_offer_notification::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
                offer_id: offer_id.into(),
                payload: None,
            }
        }
        #[doc = "Withdraw a user request approval on specific plan"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        #[doc = "* `request_approval_id`: The request approval ID to get create or update"]
        pub fn withdraw_plan(&self, private_store_id: impl Into<String>, request_approval_id: impl Into<String>) -> withdraw_plan::Builder {
            withdraw_plan::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
                request_approval_id: request_approval_id.into(),
                payload: None,
            }
        }
        #[doc = "Fetch all subscriptions in tenant, only for marketplace admin"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        pub fn fetch_all_subscriptions_in_tenant(&self, private_store_id: impl Into<String>) -> fetch_all_subscriptions_in_tenant::Builder {
            fetch_all_subscriptions_in_tenant::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
                next_page_token: None,
            }
        }
        #[doc = "List new plans notifications"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        pub fn list_new_plans_notifications(&self, private_store_id: impl Into<String>) -> list_new_plans_notifications::Builder {
            list_new_plans_notifications::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
            }
        }
        #[doc = "List stop sell notifications for both stop sell offers and stop sell plans"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        pub fn list_stop_sell_offers_plans_notifications(
            &self,
            private_store_id: impl Into<String>,
        ) -> list_stop_sell_offers_plans_notifications::Builder {
            list_stop_sell_offers_plans_notifications::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
                stop_sell_subscriptions: None,
            }
        }
        #[doc = "List all the subscriptions in the private store context"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        pub fn list_subscriptions_context(&self, private_store_id: impl Into<String>) -> list_subscriptions_context::Builder {
            list_subscriptions_context::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::PrivateStoreList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) use_cache: Option<String>,
        }
        impl Builder {
            #[doc = "Determines if to use cache or DB for serving this request"]
            pub fn use_cache(mut self, use_cache: impl Into<String>) -> Self {
                self.use_cache = Some(use_cache.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url =
                            azure_core::Url::parse(&format!("{}/providers/Microsoft.Marketplace/privateStores", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                                if let Some(use_cache) = &this.use_cache {
                                    req.url_mut().query_pairs_mut().append_pair("use-cache", use_cache);
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PrivateStoreList = serde_json::from_slice(&rsp_body)?;
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
    pub mod get {
        use super::models;
        type Response = models::PrivateStore;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}",
                            this.client.endpoint(),
                            &this.private_store_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PrivateStore = serde_json::from_slice(&rsp_body)?;
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
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
            pub(crate) payload: Option<models::PrivateStore>,
        }
        impl Builder {
            pub fn payload(mut self, payload: impl Into<models::PrivateStore>) -> Self {
                self.payload = Some(payload.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}",
                            this.client.endpoint(),
                            &this.private_store_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = if let Some(payload) = &this.payload {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(payload)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
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
            pub(crate) private_store_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}",
                            this.client.endpoint(),
                            &this.private_store_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
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
    pub mod query_offers {
        use super::models;
        type Response = models::QueryOffers;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/queryOffers",
                            this.client.endpoint(),
                            &this.private_store_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::QueryOffers = serde_json::from_slice(&rsp_body)?;
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
    pub mod billing_accounts {
        use super::models;
        type Response = models::BillingAccountsResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/billingAccounts",
                            this.client.endpoint(),
                            &this.private_store_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BillingAccountsResponse = serde_json::from_slice(&rsp_body)?;
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
    pub mod collections_to_subscriptions_mapping {
        use super::models;
        type Response = models::CollectionsToSubscriptionsMappingResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
            pub(crate) payload: Option<models::CollectionsToSubscriptionsMappingPayload>,
        }
        impl Builder {
            pub fn payload(mut self, payload: impl Into<models::CollectionsToSubscriptionsMappingPayload>) -> Self {
                self.payload = Some(payload.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/collectionsToSubscriptionsMapping",
                            this.client.endpoint(),
                            &this.private_store_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = if let Some(payload) = &this.payload {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(payload)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CollectionsToSubscriptionsMappingResponse = serde_json::from_slice(&rsp_body)?;
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
    pub mod query_approved_plans {
        use super::models;
        type Response = models::QueryApprovedPlansResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
            pub(crate) payload: Option<models::QueryApprovedPlansPayload>,
        }
        impl Builder {
            pub fn payload(mut self, payload: impl Into<models::QueryApprovedPlansPayload>) -> Self {
                self.payload = Some(payload.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/queryApprovedPlans",
                            this.client.endpoint(),
                            &this.private_store_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = if let Some(payload) = &this.payload {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(payload)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::QueryApprovedPlansResponse = serde_json::from_slice(&rsp_body)?;
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
    pub mod bulk_collections_action {
        use super::models;
        type Response = models::BulkCollectionsResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
            pub(crate) payload: Option<models::BulkCollectionsPayload>,
        }
        impl Builder {
            pub fn payload(mut self, payload: impl Into<models::BulkCollectionsPayload>) -> Self {
                self.payload = Some(payload.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/bulkCollectionsAction",
                            this.client.endpoint(),
                            &this.private_store_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = if let Some(payload) = &this.payload {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(payload)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::BulkCollectionsResponse = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_approval_requests_list {
        use super::models;
        type Response = models::RequestApprovalsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/requestApprovals",
                            this.client.endpoint(),
                            &this.private_store_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RequestApprovalsList = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_request_approval {
        use super::models;
        type Response = models::RequestApprovalResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
            pub(crate) request_approval_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/requestApprovals/{}",
                            this.client.endpoint(),
                            &this.private_store_id,
                            &this.request_approval_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RequestApprovalResource = serde_json::from_slice(&rsp_body)?;
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
    pub mod create_approval_request {
        use super::models;
        type Response = models::RequestApprovalResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
            pub(crate) request_approval_id: String,
            pub(crate) payload: Option<models::RequestApprovalResource>,
        }
        impl Builder {
            pub fn payload(mut self, payload: impl Into<models::RequestApprovalResource>) -> Self {
                self.payload = Some(payload.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/requestApprovals/{}",
                            this.client.endpoint(),
                            &this.private_store_id,
                            &this.request_approval_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = if let Some(payload) = &this.payload {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(payload)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RequestApprovalResource = serde_json::from_slice(&rsp_body)?;
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
    pub mod query_request_approval {
        use super::models;
        type Response = models::QueryRequestApproval;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
            pub(crate) request_approval_id: String,
            pub(crate) payload: Option<models::QueryRequestApprovalProperties>,
        }
        impl Builder {
            pub fn payload(mut self, payload: impl Into<models::QueryRequestApprovalProperties>) -> Self {
                self.payload = Some(payload.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/requestApprovals/{}/query",
                            this.client.endpoint(),
                            &this.private_store_id,
                            &this.request_approval_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = if let Some(payload) = &this.payload {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(payload)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::QueryRequestApproval = serde_json::from_slice(&rsp_body)?;
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
    pub mod admin_request_approvals_list {
        use super::models;
        type Response = models::AdminRequestApprovalsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/adminRequestApprovals",
                            this.client.endpoint(),
                            &this.private_store_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AdminRequestApprovalsList = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_admin_request_approval {
        use super::models;
        type Response = models::AdminRequestApprovalsResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
            pub(crate) admin_request_approval_id: String,
            pub(crate) publisher_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/adminRequestApprovals/{}",
                            this.client.endpoint(),
                            &this.private_store_id,
                            &this.admin_request_approval_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let publisher_id = &this.publisher_id;
                        req.url_mut().query_pairs_mut().append_pair("publisherId", publisher_id);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AdminRequestApprovalsResource = serde_json::from_slice(&rsp_body)?;
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
    pub mod update_admin_request_approval {
        use super::models;
        type Response = models::AdminRequestApprovalsResource;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
            pub(crate) admin_request_approval_id: String,
            pub(crate) payload: Option<models::AdminRequestApprovalsResource>,
        }
        impl Builder {
            pub fn payload(mut self, payload: impl Into<models::AdminRequestApprovalsResource>) -> Self {
                self.payload = Some(payload.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/adminRequestApprovals/{}",
                            this.client.endpoint(),
                            &this.private_store_id,
                            &this.admin_request_approval_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = if let Some(payload) = &this.payload {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(payload)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AdminRequestApprovalsResource = serde_json::from_slice(&rsp_body)?;
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
    pub mod query_notifications_state {
        use super::models;
        type Response = models::PrivateStoreNotificationsState;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/queryNotificationsState",
                            this.client.endpoint(),
                            &this.private_store_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PrivateStoreNotificationsState = serde_json::from_slice(&rsp_body)?;
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
    pub mod acknowledge_offer_notification {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
            pub(crate) offer_id: String,
            pub(crate) payload: Option<models::AcknowledgeOfferNotificationProperties>,
        }
        impl Builder {
            pub fn payload(mut self, payload: impl Into<models::AcknowledgeOfferNotificationProperties>) -> Self {
                self.payload = Some(payload.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/offers/{}/acknowledgeNotification",
                            this.client.endpoint(),
                            &this.private_store_id,
                            &this.offer_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = if let Some(payload) = &this.payload {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(payload)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
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
    pub mod withdraw_plan {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
            pub(crate) request_approval_id: String,
            pub(crate) payload: Option<models::WithdrawProperties>,
        }
        impl Builder {
            pub fn payload(mut self, payload: impl Into<models::WithdrawProperties>) -> Self {
                self.payload = Some(payload.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/requestApprovals/{}/withdrawPlan",
                            this.client.endpoint(),
                            &this.private_store_id,
                            &this.request_approval_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = if let Some(payload) = &this.payload {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(payload)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
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
    pub mod fetch_all_subscriptions_in_tenant {
        use super::models;
        type Response = models::SubscriptionsResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
            pub(crate) next_page_token: Option<String>,
        }
        impl Builder {
            #[doc = "The skip token to get the next page."]
            pub fn next_page_token(mut self, next_page_token: impl Into<String>) -> Self {
                self.next_page_token = Some(next_page_token.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/fetchAllSubscriptionsInTenant",
                            this.client.endpoint(),
                            &this.private_store_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        if let Some(next_page_token) = &this.next_page_token {
                            req.insert_header("next-page-token", next_page_token);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SubscriptionsResponse = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_new_plans_notifications {
        use super::models;
        type Response = models::NewPlansNotificationsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/listNewPlansNotifications",
                            this.client.endpoint(),
                            &this.private_store_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NewPlansNotificationsList = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_stop_sell_offers_plans_notifications {
        use super::models;
        type Response = models::StopSellOffersPlansNotificationsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
            pub(crate) stop_sell_subscriptions: Option<models::StopSellSubscriptions>,
        }
        impl Builder {
            pub fn stop_sell_subscriptions(mut self, stop_sell_subscriptions: impl Into<models::StopSellSubscriptions>) -> Self {
                self.stop_sell_subscriptions = Some(stop_sell_subscriptions.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/listStopSellOffersPlansNotifications",
                            this.client.endpoint(),
                            &this.private_store_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = if let Some(stop_sell_subscriptions) = &this.stop_sell_subscriptions {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(stop_sell_subscriptions)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::StopSellOffersPlansNotificationsList = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_subscriptions_context {
        use super::models;
        type Response = models::SubscriptionsContextList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/listSubscriptionsContext",
                            this.client.endpoint(),
                            &this.private_store_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SubscriptionsContextList = serde_json::from_slice(&rsp_body)?;
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
pub mod private_store_collection {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets private store collections list"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        pub fn list(&self, private_store_id: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
            }
        }
        #[doc = "Gets private store collection"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        #[doc = "* `collection_id`: The collection ID"]
        pub fn get(&self, private_store_id: impl Into<String>, collection_id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
                collection_id: collection_id.into(),
            }
        }
        #[doc = "Delete Private store collection. This is a workaround."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        #[doc = "* `collection_id`: The collection ID"]
        pub fn post(&self, private_store_id: impl Into<String>, collection_id: impl Into<String>) -> post::Builder {
            post::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
                collection_id: collection_id.into(),
                payload: None,
            }
        }
        #[doc = "Create or update private store collection"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        #[doc = "* `collection_id`: The collection ID"]
        pub fn create_or_update(&self, private_store_id: impl Into<String>, collection_id: impl Into<String>) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
                collection_id: collection_id.into(),
                payload: None,
            }
        }
        #[doc = "Delete a collection from the given private store."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        #[doc = "* `collection_id`: The collection ID"]
        pub fn delete(&self, private_store_id: impl Into<String>, collection_id: impl Into<String>) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
                collection_id: collection_id.into(),
            }
        }
        #[doc = "transferring offers (copy or move) from source collection to target collection(s)"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        #[doc = "* `collection_id`: The collection ID"]
        pub fn transfer_offers(&self, private_store_id: impl Into<String>, collection_id: impl Into<String>) -> transfer_offers::Builder {
            transfer_offers::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
                collection_id: collection_id.into(),
                payload: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::CollectionsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/collections",
                            this.client.endpoint(),
                            &this.private_store_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CollectionsList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Collection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
            pub(crate) collection_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/collections/{}",
                            this.client.endpoint(),
                            &this.private_store_id,
                            &this.collection_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Collection = serde_json::from_slice(&rsp_body)?;
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
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
            pub(crate) collection_id: String,
            pub(crate) payload: Option<models::PrivateStoreOperation>,
        }
        impl Builder {
            pub fn payload(mut self, payload: impl Into<models::PrivateStoreOperation>) -> Self {
                self.payload = Some(payload.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/collections/{}",
                            this.client.endpoint(),
                            &this.private_store_id,
                            &this.collection_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = if let Some(payload) = &this.payload {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(payload)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
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
    pub mod create_or_update {
        use super::models;
        type Response = models::Collection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
            pub(crate) collection_id: String,
            pub(crate) payload: Option<models::Collection>,
        }
        impl Builder {
            pub fn payload(mut self, payload: impl Into<models::Collection>) -> Self {
                self.payload = Some(payload.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/collections/{}",
                            this.client.endpoint(),
                            &this.private_store_id,
                            &this.collection_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = if let Some(payload) = &this.payload {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(payload)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Collection = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) private_store_id: String,
            pub(crate) collection_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/collections/{}",
                            this.client.endpoint(),
                            &this.private_store_id,
                            &this.collection_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
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
    pub mod transfer_offers {
        use super::models;
        type Response = models::TransferOffersResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
            pub(crate) collection_id: String,
            pub(crate) payload: Option<models::TransferOffersProperties>,
        }
        impl Builder {
            pub fn payload(mut self, payload: impl Into<models::TransferOffersProperties>) -> Self {
                self.payload = Some(payload.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/collections/{}/transferOffers",
                            this.client.endpoint(),
                            &this.private_store_id,
                            &this.collection_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = if let Some(payload) = &this.payload {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(payload)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TransferOffersResponse = serde_json::from_slice(&rsp_body)?;
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
pub mod private_store_collection_offer {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of all private offers in the given private store and collection"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        #[doc = "* `collection_id`: The collection ID"]
        pub fn list(&self, private_store_id: impl Into<String>, collection_id: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
                collection_id: collection_id.into(),
            }
        }
        #[doc = "Gets information about a specific offer."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        #[doc = "* `offer_id`: The offer ID to update or delete"]
        #[doc = "* `collection_id`: The collection ID"]
        pub fn get(
            &self,
            private_store_id: impl Into<String>,
            offer_id: impl Into<String>,
            collection_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
                offer_id: offer_id.into(),
                collection_id: collection_id.into(),
            }
        }
        #[doc = "Delete Private store offer. This is a workaround."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        #[doc = "* `offer_id`: The offer ID to update or delete"]
        #[doc = "* `collection_id`: The collection ID"]
        pub fn post(
            &self,
            private_store_id: impl Into<String>,
            offer_id: impl Into<String>,
            collection_id: impl Into<String>,
        ) -> post::Builder {
            post::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
                offer_id: offer_id.into(),
                collection_id: collection_id.into(),
                payload: None,
            }
        }
        #[doc = "Update or add an offer to a specific collection of the private store."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        #[doc = "* `offer_id`: The offer ID to update or delete"]
        #[doc = "* `collection_id`: The collection ID"]
        pub fn create_or_update(
            &self,
            private_store_id: impl Into<String>,
            offer_id: impl Into<String>,
            collection_id: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
                offer_id: offer_id.into(),
                collection_id: collection_id.into(),
                payload: None,
            }
        }
        #[doc = "Deletes an offer from the given collection of private store."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `private_store_id`: The store ID - must use the tenant ID"]
        #[doc = "* `offer_id`: The offer ID to update or delete"]
        #[doc = "* `collection_id`: The collection ID"]
        pub fn delete(
            &self,
            private_store_id: impl Into<String>,
            offer_id: impl Into<String>,
            collection_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                private_store_id: private_store_id.into(),
                offer_id: offer_id.into(),
                collection_id: collection_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::OfferListResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
            pub(crate) collection_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/collections/{}/offers",
                            this.client.endpoint(),
                            &this.private_store_id,
                            &this.collection_id
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::OfferListResponse = serde_json::from_slice(&rsp_body)?;
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
    pub mod get {
        use super::models;
        type Response = models::Offer;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
            pub(crate) offer_id: String,
            pub(crate) collection_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/collections/{}/offers/{}",
                            this.client.endpoint(),
                            &this.private_store_id,
                            &this.collection_id,
                            &this.offer_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Offer = serde_json::from_slice(&rsp_body)?;
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
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
            pub(crate) offer_id: String,
            pub(crate) collection_id: String,
            pub(crate) payload: Option<models::PrivateStoreOperation>,
        }
        impl Builder {
            pub fn payload(mut self, payload: impl Into<models::PrivateStoreOperation>) -> Self {
                self.payload = Some(payload.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/collections/{}/offers/{}",
                            this.client.endpoint(),
                            &this.private_store_id,
                            &this.collection_id,
                            &this.offer_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = if let Some(payload) = &this.payload {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(payload)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
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
    pub mod create_or_update {
        use super::models;
        type Response = models::Offer;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) private_store_id: String,
            pub(crate) offer_id: String,
            pub(crate) collection_id: String,
            pub(crate) payload: Option<models::Offer>,
        }
        impl Builder {
            pub fn payload(mut self, payload: impl Into<models::Offer>) -> Self {
                self.payload = Some(payload.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/collections/{}/offers/{}",
                            this.client.endpoint(),
                            &this.private_store_id,
                            &this.collection_id,
                            &this.offer_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = if let Some(payload) = &this.payload {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(payload)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Offer = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) private_store_id: String,
            pub(crate) offer_id: String,
            pub(crate) collection_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Marketplace/privateStores/{}/collections/{}/offers/{}",
                            this.client.endpoint(),
                            &this.private_store_id,
                            &this.collection_id,
                            &this.offer_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
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
pub mod operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the available Microsoft.Marketplace REST API operations."]
        pub fn list(&self) -> list::Builder {
            list::Builder { client: self.0.clone() }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::OperationListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/providers/Microsoft.Marketplace/operations", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::OperationListResult = serde_json::from_slice(&rsp_body)?;
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
