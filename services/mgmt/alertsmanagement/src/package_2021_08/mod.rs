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
    pub fn alert_processing_rules_client(&self) -> alert_processing_rules::Client {
        alert_processing_rules::Client(self.clone())
    }
    pub fn alerts_client(&self) -> alerts::Client {
        alerts::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn smart_groups_client(&self) -> smart_groups::Client {
        smart_groups::Client(self.clone())
    }
}
pub mod alert_processing_rules {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List all alert processing rules in a subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        pub fn list_by_subscription(&self, subscription_id: impl Into<String>) -> list_by_subscription::Builder {
            list_by_subscription::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "List all alert processing rules in a resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: Resource group name where the resource is created."]
        pub fn list_by_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
        ) -> list_by_resource_group::Builder {
            list_by_resource_group::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
            }
        }
        #[doc = "Get an alert processing rule by name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: Resource group name where the resource is created."]
        #[doc = "* `alert_processing_rule_name`: The name of the alert processing rule that needs to be fetched."]
        pub fn get_by_name(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            alert_processing_rule_name: impl Into<String>,
        ) -> get_by_name::Builder {
            get_by_name::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                alert_processing_rule_name: alert_processing_rule_name.into(),
            }
        }
        #[doc = "Create or update an alert processing rule."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: Resource group name where the resource is created."]
        #[doc = "* `alert_processing_rule_name`: The name of the alert processing rule that needs to be created/updated."]
        #[doc = "* `alert_processing_rule`: Alert processing rule to be created/updated."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            alert_processing_rule_name: impl Into<String>,
            alert_processing_rule: impl Into<models::AlertProcessingRule>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                alert_processing_rule_name: alert_processing_rule_name.into(),
                alert_processing_rule: alert_processing_rule.into(),
            }
        }
        #[doc = "Enable, disable, or update tags for an alert processing rule."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: Resource group name where the resource is created."]
        #[doc = "* `alert_processing_rule_name`: The name that needs to be updated."]
        #[doc = "* `alert_processing_rule_patch`: Parameters supplied to the operation."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            alert_processing_rule_name: impl Into<String>,
            alert_processing_rule_patch: impl Into<models::PatchObject>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                alert_processing_rule_name: alert_processing_rule_name.into(),
                alert_processing_rule_patch: alert_processing_rule_patch.into(),
            }
        }
        #[doc = "Delete an alert processing rule."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: Resource group name where the resource is created."]
        #[doc = "* `alert_processing_rule_name`: The name of the alert processing rule that needs to be deleted."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            alert_processing_rule_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                alert_processing_rule_name: alert_processing_rule_name.into(),
            }
        }
    }
    pub mod list_by_subscription {
        use super::models;
        type Response = models::AlertProcessingRulesList;
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
                            "{}/subscriptions/{}/providers/Microsoft.AlertsManagement/actionRules",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-08-08");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-08-08");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AlertProcessingRulesList = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_by_resource_group {
        use super::models;
        type Response = models::AlertProcessingRulesList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AlertsManagement/actionRules",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-08-08");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-08-08");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AlertProcessingRulesList = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_by_name {
        use super::models;
        type Response = models::AlertProcessingRule;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) alert_processing_rule_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AlertsManagement/actionRules/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.alert_processing_rule_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-08-08");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AlertProcessingRule = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::AlertProcessingRule),
            Created201(models::AlertProcessingRule),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) alert_processing_rule_name: String,
            pub(crate) alert_processing_rule: models::AlertProcessingRule,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AlertsManagement/actionRules/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.alert_processing_rule_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-08-08");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.alert_processing_rule)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AlertProcessingRule = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AlertProcessingRule = serde_json::from_slice(&rsp_body)?;
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
    pub mod update {
        use super::models;
        type Response = models::AlertProcessingRule;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) alert_processing_rule_name: String,
            pub(crate) alert_processing_rule_patch: models::PatchObject,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AlertsManagement/actionRules/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.alert_processing_rule_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-08-08");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.alert_processing_rule_patch)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AlertProcessingRule = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) resource_group_name: String,
            pub(crate) alert_processing_rule_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AlertsManagement/actionRules/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.alert_processing_rule_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-08-08");
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
        #[doc = "List all operations available through Azure Alerts Management Resource Provider."]
        pub fn list(&self) -> list::Builder {
            list::Builder { client: self.0.clone() }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::OperationsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.AlertsManagement/operations",
                            this.client.endpoint(),
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-05-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-05-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::OperationsList = serde_json::from_slice(&rsp_body)?;
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
pub mod alerts {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List alerts meta data information based on value of identifier parameter."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `identifier`: Identification of the information to be retrieved by API call."]
        pub fn meta_data(&self, identifier: impl Into<String>) -> meta_data::Builder {
            meta_data::Builder {
                client: self.0.clone(),
                identifier: identifier.into(),
            }
        }
        #[doc = "List all existing alerts, where the results can be filtered on the basis of multiple parameters (e.g. time range). The results can then be sorted on the basis specific fields, with the default being lastModifiedDateTime. "]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        pub fn get_all(&self, subscription_id: impl Into<String>) -> get_all::Builder {
            get_all::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                target_resource: None,
                target_resource_type: None,
                target_resource_group: None,
                monitor_service: None,
                monitor_condition: None,
                severity: None,
                alert_state: None,
                alert_rule: None,
                smart_group_id: None,
                include_context: None,
                include_egress_config: None,
                page_count: None,
                sort_by: None,
                sort_order: None,
                select: None,
                time_range: None,
                custom_time_range: None,
            }
        }
        #[doc = "Get a specific alert."]
        #[doc = "Get information related to a specific alert"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `alert_id`: Unique ID of an alert instance."]
        pub fn get_by_id(&self, subscription_id: impl Into<String>, alert_id: impl Into<String>) -> get_by_id::Builder {
            get_by_id::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                alert_id: alert_id.into(),
            }
        }
        #[doc = "Change the state of an alert."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `alert_id`: Unique ID of an alert instance."]
        #[doc = "* `new_state`: New state of the alert."]
        pub fn change_state(
            &self,
            subscription_id: impl Into<String>,
            alert_id: impl Into<String>,
            new_state: impl Into<String>,
        ) -> change_state::Builder {
            change_state::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                alert_id: alert_id.into(),
                new_state: new_state.into(),
                comment: None,
            }
        }
        #[doc = "Get the history of an alert, which captures any monitor condition changes (Fired/Resolved) and alert state changes (New/Acknowledged/Closed)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `alert_id`: Unique ID of an alert instance."]
        pub fn get_history(&self, subscription_id: impl Into<String>, alert_id: impl Into<String>) -> get_history::Builder {
            get_history::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                alert_id: alert_id.into(),
            }
        }
        #[doc = "Get a summarized count of your alerts grouped by various parameters (e.g. grouping by 'Severity' returns the count of alerts for each severity)."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `groupby`: This parameter allows the result set to be grouped by input fields (Maximum 2 comma separated fields supported). For example, groupby=severity or groupby=severity,alertstate."]
        pub fn get_summary(&self, subscription_id: impl Into<String>, groupby: impl Into<String>) -> get_summary::Builder {
            get_summary::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                groupby: groupby.into(),
                include_smart_groups_count: None,
                target_resource: None,
                target_resource_type: None,
                target_resource_group: None,
                monitor_service: None,
                monitor_condition: None,
                severity: None,
                alert_state: None,
                alert_rule: None,
                time_range: None,
                custom_time_range: None,
            }
        }
    }
    pub mod meta_data {
        use super::models;
        type Response = models::AlertsMetaData;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) identifier: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.AlertsManagement/alertsMetaData",
                            this.client.endpoint(),
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-05-preview");
                        let identifier = &this.identifier;
                        req.url_mut().query_pairs_mut().append_pair("identifier", identifier);
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AlertsMetaData = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_all {
        use super::models;
        type Response = models::AlertsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) target_resource: Option<String>,
            pub(crate) target_resource_type: Option<String>,
            pub(crate) target_resource_group: Option<String>,
            pub(crate) monitor_service: Option<String>,
            pub(crate) monitor_condition: Option<String>,
            pub(crate) severity: Option<String>,
            pub(crate) alert_state: Option<String>,
            pub(crate) alert_rule: Option<String>,
            pub(crate) smart_group_id: Option<String>,
            pub(crate) include_context: Option<bool>,
            pub(crate) include_egress_config: Option<bool>,
            pub(crate) page_count: Option<i64>,
            pub(crate) sort_by: Option<String>,
            pub(crate) sort_order: Option<String>,
            pub(crate) select: Option<String>,
            pub(crate) time_range: Option<String>,
            pub(crate) custom_time_range: Option<String>,
        }
        impl Builder {
            #[doc = "Filter by target resource( which is full ARM ID) Default value is select all."]
            pub fn target_resource(mut self, target_resource: impl Into<String>) -> Self {
                self.target_resource = Some(target_resource.into());
                self
            }
            #[doc = "Filter by target resource type. Default value is select all."]
            pub fn target_resource_type(mut self, target_resource_type: impl Into<String>) -> Self {
                self.target_resource_type = Some(target_resource_type.into());
                self
            }
            #[doc = "Filter by target resource group name. Default value is select all."]
            pub fn target_resource_group(mut self, target_resource_group: impl Into<String>) -> Self {
                self.target_resource_group = Some(target_resource_group.into());
                self
            }
            #[doc = "Filter by monitor service which generates the alert instance. Default value is select all."]
            pub fn monitor_service(mut self, monitor_service: impl Into<String>) -> Self {
                self.monitor_service = Some(monitor_service.into());
                self
            }
            #[doc = "Filter by monitor condition which is either 'Fired' or 'Resolved'. Default value is to select all."]
            pub fn monitor_condition(mut self, monitor_condition: impl Into<String>) -> Self {
                self.monitor_condition = Some(monitor_condition.into());
                self
            }
            #[doc = "Filter by severity.  Default value is select all."]
            pub fn severity(mut self, severity: impl Into<String>) -> Self {
                self.severity = Some(severity.into());
                self
            }
            #[doc = "Filter by state of the alert instance. Default value is to select all."]
            pub fn alert_state(mut self, alert_state: impl Into<String>) -> Self {
                self.alert_state = Some(alert_state.into());
                self
            }
            #[doc = "Filter by specific alert rule.  Default value is to select all."]
            pub fn alert_rule(mut self, alert_rule: impl Into<String>) -> Self {
                self.alert_rule = Some(alert_rule.into());
                self
            }
            #[doc = "Filter the alerts list by the Smart Group Id. Default value is none."]
            pub fn smart_group_id(mut self, smart_group_id: impl Into<String>) -> Self {
                self.smart_group_id = Some(smart_group_id.into());
                self
            }
            #[doc = "Include context which has contextual data specific to the monitor service. Default value is false'"]
            pub fn include_context(mut self, include_context: bool) -> Self {
                self.include_context = Some(include_context);
                self
            }
            #[doc = "Include egress config which would be used for displaying the content in portal.  Default value is 'false'."]
            pub fn include_egress_config(mut self, include_egress_config: bool) -> Self {
                self.include_egress_config = Some(include_egress_config);
                self
            }
            #[doc = "Determines number of alerts returned per page in response. Permissible value is between 1 to 250. When the \"includeContent\"  filter is selected, maximum value allowed is 25. Default value is 25."]
            pub fn page_count(mut self, page_count: i64) -> Self {
                self.page_count = Some(page_count);
                self
            }
            #[doc = "Sort the query results by input field,  Default value is 'lastModifiedDateTime'."]
            pub fn sort_by(mut self, sort_by: impl Into<String>) -> Self {
                self.sort_by = Some(sort_by.into());
                self
            }
            #[doc = "Sort the query results order in either ascending or descending.  Default value is 'desc' for time fields and 'asc' for others."]
            pub fn sort_order(mut self, sort_order: impl Into<String>) -> Self {
                self.sort_order = Some(sort_order.into());
                self
            }
            #[doc = "This filter allows to selection of the fields(comma separated) which would  be part of the essential section. This would allow to project only the  required fields rather than getting entire content.  Default is to fetch all the fields in the essentials section."]
            pub fn select(mut self, select: impl Into<String>) -> Self {
                self.select = Some(select.into());
                self
            }
            #[doc = "Filter by time range by below listed values. Default value is 1 day."]
            pub fn time_range(mut self, time_range: impl Into<String>) -> Self {
                self.time_range = Some(time_range.into());
                self
            }
            #[doc = "Filter by custom time range in the format <start-time>/<end-time>  where time is in (ISO-8601 format)'. Permissible values is within 30 days from  query time. Either timeRange or customTimeRange could be used but not both. Default is none."]
            pub fn custom_time_range(mut self, custom_time_range: impl Into<String>) -> Self {
                self.custom_time_range = Some(custom_time_range.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.AlertsManagement/alerts",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-05-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-05-preview");
                                if let Some(target_resource) = &this.target_resource {
                                    req.url_mut().query_pairs_mut().append_pair("targetResource", target_resource);
                                }
                                if let Some(target_resource_type) = &this.target_resource_type {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("targetResourceType", target_resource_type);
                                }
                                if let Some(target_resource_group) = &this.target_resource_group {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("targetResourceGroup", target_resource_group);
                                }
                                if let Some(monitor_service) = &this.monitor_service {
                                    req.url_mut().query_pairs_mut().append_pair("monitorService", monitor_service);
                                }
                                if let Some(monitor_condition) = &this.monitor_condition {
                                    req.url_mut().query_pairs_mut().append_pair("monitorCondition", monitor_condition);
                                }
                                if let Some(severity) = &this.severity {
                                    req.url_mut().query_pairs_mut().append_pair("severity", severity);
                                }
                                if let Some(alert_state) = &this.alert_state {
                                    req.url_mut().query_pairs_mut().append_pair("alertState", alert_state);
                                }
                                if let Some(alert_rule) = &this.alert_rule {
                                    req.url_mut().query_pairs_mut().append_pair("alertRule", alert_rule);
                                }
                                if let Some(smart_group_id) = &this.smart_group_id {
                                    req.url_mut().query_pairs_mut().append_pair("smartGroupId", smart_group_id);
                                }
                                if let Some(include_context) = &this.include_context {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("includeContext", &include_context.to_string());
                                }
                                if let Some(include_egress_config) = &this.include_egress_config {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("includeEgressConfig", &include_egress_config.to_string());
                                }
                                if let Some(page_count) = &this.page_count {
                                    req.url_mut().query_pairs_mut().append_pair("pageCount", &page_count.to_string());
                                }
                                if let Some(sort_by) = &this.sort_by {
                                    req.url_mut().query_pairs_mut().append_pair("sortBy", sort_by);
                                }
                                if let Some(sort_order) = &this.sort_order {
                                    req.url_mut().query_pairs_mut().append_pair("sortOrder", sort_order);
                                }
                                if let Some(select) = &this.select {
                                    req.url_mut().query_pairs_mut().append_pair("select", select);
                                }
                                if let Some(time_range) = &this.time_range {
                                    req.url_mut().query_pairs_mut().append_pair("timeRange", time_range);
                                }
                                if let Some(custom_time_range) = &this.custom_time_range {
                                    req.url_mut().query_pairs_mut().append_pair("customTimeRange", custom_time_range);
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
                                let rsp_value: models::AlertsList = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_by_id {
        use super::models;
        type Response = models::Alert;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) alert_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.AlertsManagement/alerts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.alert_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-05-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Alert = serde_json::from_slice(&rsp_body)?;
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
    pub mod change_state {
        use super::models;
        type Response = models::Alert;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) alert_id: String,
            pub(crate) new_state: String,
            pub(crate) comment: Option<String>,
        }
        impl Builder {
            #[doc = "reason of change alert state"]
            pub fn comment(mut self, comment: impl Into<String>) -> Self {
                self.comment = Some(comment.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.AlertsManagement/alerts/{}/changestate",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.alert_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-05-preview");
                        let new_state = &this.new_state;
                        req.url_mut().query_pairs_mut().append_pair("newState", new_state);
                        let req_body = if let Some(comment) = &this.comment {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(comment)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Alert = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_history {
        use super::models;
        type Response = models::AlertModification;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) alert_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.AlertsManagement/alerts/{}/history",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.alert_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-05-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AlertModification = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_summary {
        use super::models;
        type Response = models::AlertsSummary;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) groupby: String,
            pub(crate) include_smart_groups_count: Option<bool>,
            pub(crate) target_resource: Option<String>,
            pub(crate) target_resource_type: Option<String>,
            pub(crate) target_resource_group: Option<String>,
            pub(crate) monitor_service: Option<String>,
            pub(crate) monitor_condition: Option<String>,
            pub(crate) severity: Option<String>,
            pub(crate) alert_state: Option<String>,
            pub(crate) alert_rule: Option<String>,
            pub(crate) time_range: Option<String>,
            pub(crate) custom_time_range: Option<String>,
        }
        impl Builder {
            #[doc = "Include count of the SmartGroups as part of the summary. Default value is 'false'."]
            pub fn include_smart_groups_count(mut self, include_smart_groups_count: bool) -> Self {
                self.include_smart_groups_count = Some(include_smart_groups_count);
                self
            }
            #[doc = "Filter by target resource( which is full ARM ID) Default value is select all."]
            pub fn target_resource(mut self, target_resource: impl Into<String>) -> Self {
                self.target_resource = Some(target_resource.into());
                self
            }
            #[doc = "Filter by target resource type. Default value is select all."]
            pub fn target_resource_type(mut self, target_resource_type: impl Into<String>) -> Self {
                self.target_resource_type = Some(target_resource_type.into());
                self
            }
            #[doc = "Filter by target resource group name. Default value is select all."]
            pub fn target_resource_group(mut self, target_resource_group: impl Into<String>) -> Self {
                self.target_resource_group = Some(target_resource_group.into());
                self
            }
            #[doc = "Filter by monitor service which generates the alert instance. Default value is select all."]
            pub fn monitor_service(mut self, monitor_service: impl Into<String>) -> Self {
                self.monitor_service = Some(monitor_service.into());
                self
            }
            #[doc = "Filter by monitor condition which is either 'Fired' or 'Resolved'. Default value is to select all."]
            pub fn monitor_condition(mut self, monitor_condition: impl Into<String>) -> Self {
                self.monitor_condition = Some(monitor_condition.into());
                self
            }
            #[doc = "Filter by severity.  Default value is select all."]
            pub fn severity(mut self, severity: impl Into<String>) -> Self {
                self.severity = Some(severity.into());
                self
            }
            #[doc = "Filter by state of the alert instance. Default value is to select all."]
            pub fn alert_state(mut self, alert_state: impl Into<String>) -> Self {
                self.alert_state = Some(alert_state.into());
                self
            }
            #[doc = "Filter by specific alert rule.  Default value is to select all."]
            pub fn alert_rule(mut self, alert_rule: impl Into<String>) -> Self {
                self.alert_rule = Some(alert_rule.into());
                self
            }
            #[doc = "Filter by time range by below listed values. Default value is 1 day."]
            pub fn time_range(mut self, time_range: impl Into<String>) -> Self {
                self.time_range = Some(time_range.into());
                self
            }
            #[doc = "Filter by custom time range in the format <start-time>/<end-time>  where time is in (ISO-8601 format)'. Permissible values is within 30 days from  query time. Either timeRange or customTimeRange could be used but not both. Default is none."]
            pub fn custom_time_range(mut self, custom_time_range: impl Into<String>) -> Self {
                self.custom_time_range = Some(custom_time_range.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.AlertsManagement/alertsSummary",
                            this.client.endpoint(),
                            &this.subscription_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-05-preview");
                        let groupby = &this.groupby;
                        req.url_mut().query_pairs_mut().append_pair("groupby", groupby);
                        if let Some(include_smart_groups_count) = &this.include_smart_groups_count {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeSmartGroupsCount", &include_smart_groups_count.to_string());
                        }
                        if let Some(target_resource) = &this.target_resource {
                            req.url_mut().query_pairs_mut().append_pair("targetResource", target_resource);
                        }
                        if let Some(target_resource_type) = &this.target_resource_type {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("targetResourceType", target_resource_type);
                        }
                        if let Some(target_resource_group) = &this.target_resource_group {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("targetResourceGroup", target_resource_group);
                        }
                        if let Some(monitor_service) = &this.monitor_service {
                            req.url_mut().query_pairs_mut().append_pair("monitorService", monitor_service);
                        }
                        if let Some(monitor_condition) = &this.monitor_condition {
                            req.url_mut().query_pairs_mut().append_pair("monitorCondition", monitor_condition);
                        }
                        if let Some(severity) = &this.severity {
                            req.url_mut().query_pairs_mut().append_pair("severity", severity);
                        }
                        if let Some(alert_state) = &this.alert_state {
                            req.url_mut().query_pairs_mut().append_pair("alertState", alert_state);
                        }
                        if let Some(alert_rule) = &this.alert_rule {
                            req.url_mut().query_pairs_mut().append_pair("alertRule", alert_rule);
                        }
                        if let Some(time_range) = &this.time_range {
                            req.url_mut().query_pairs_mut().append_pair("timeRange", time_range);
                        }
                        if let Some(custom_time_range) = &this.custom_time_range {
                            req.url_mut().query_pairs_mut().append_pair("customTimeRange", custom_time_range);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AlertsSummary = serde_json::from_slice(&rsp_body)?;
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
pub mod smart_groups {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get all Smart Groups within a specified subscription"]
        #[doc = "List all the Smart Groups within a specified subscription. "]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        pub fn get_all(&self, subscription_id: impl Into<String>) -> get_all::Builder {
            get_all::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                target_resource: None,
                target_resource_group: None,
                target_resource_type: None,
                monitor_service: None,
                monitor_condition: None,
                severity: None,
                smart_group_state: None,
                time_range: None,
                page_count: None,
                sort_by: None,
                sort_order: None,
            }
        }
        #[doc = "Get information related to a specific Smart Group."]
        #[doc = "Get information related to a specific Smart Group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `smart_group_id`: Smart group unique id. "]
        pub fn get_by_id(&self, subscription_id: impl Into<String>, smart_group_id: impl Into<String>) -> get_by_id::Builder {
            get_by_id::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                smart_group_id: smart_group_id.into(),
            }
        }
        #[doc = "Change the state of a Smart Group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `smart_group_id`: Smart group unique id. "]
        #[doc = "* `new_state`: New state of the alert."]
        pub fn change_state(
            &self,
            subscription_id: impl Into<String>,
            smart_group_id: impl Into<String>,
            new_state: impl Into<String>,
        ) -> change_state::Builder {
            change_state::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                smart_group_id: smart_group_id.into(),
                new_state: new_state.into(),
            }
        }
        #[doc = "Get the history a smart group, which captures any Smart Group state changes (New/Acknowledged/Closed) ."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `smart_group_id`: Smart group unique id. "]
        pub fn get_history(&self, subscription_id: impl Into<String>, smart_group_id: impl Into<String>) -> get_history::Builder {
            get_history::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                smart_group_id: smart_group_id.into(),
            }
        }
    }
    pub mod get_all {
        use super::models;
        type Response = models::SmartGroupsList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) target_resource: Option<String>,
            pub(crate) target_resource_group: Option<String>,
            pub(crate) target_resource_type: Option<String>,
            pub(crate) monitor_service: Option<String>,
            pub(crate) monitor_condition: Option<String>,
            pub(crate) severity: Option<String>,
            pub(crate) smart_group_state: Option<String>,
            pub(crate) time_range: Option<String>,
            pub(crate) page_count: Option<i64>,
            pub(crate) sort_by: Option<String>,
            pub(crate) sort_order: Option<String>,
        }
        impl Builder {
            #[doc = "Filter by target resource( which is full ARM ID) Default value is select all."]
            pub fn target_resource(mut self, target_resource: impl Into<String>) -> Self {
                self.target_resource = Some(target_resource.into());
                self
            }
            #[doc = "Filter by target resource group name. Default value is select all."]
            pub fn target_resource_group(mut self, target_resource_group: impl Into<String>) -> Self {
                self.target_resource_group = Some(target_resource_group.into());
                self
            }
            #[doc = "Filter by target resource type. Default value is select all."]
            pub fn target_resource_type(mut self, target_resource_type: impl Into<String>) -> Self {
                self.target_resource_type = Some(target_resource_type.into());
                self
            }
            #[doc = "Filter by monitor service which generates the alert instance. Default value is select all."]
            pub fn monitor_service(mut self, monitor_service: impl Into<String>) -> Self {
                self.monitor_service = Some(monitor_service.into());
                self
            }
            #[doc = "Filter by monitor condition which is either 'Fired' or 'Resolved'. Default value is to select all."]
            pub fn monitor_condition(mut self, monitor_condition: impl Into<String>) -> Self {
                self.monitor_condition = Some(monitor_condition.into());
                self
            }
            #[doc = "Filter by severity.  Default value is select all."]
            pub fn severity(mut self, severity: impl Into<String>) -> Self {
                self.severity = Some(severity.into());
                self
            }
            #[doc = "Filter by state of the smart group. Default value is to select all."]
            pub fn smart_group_state(mut self, smart_group_state: impl Into<String>) -> Self {
                self.smart_group_state = Some(smart_group_state.into());
                self
            }
            #[doc = "Filter by time range by below listed values. Default value is 1 day."]
            pub fn time_range(mut self, time_range: impl Into<String>) -> Self {
                self.time_range = Some(time_range.into());
                self
            }
            #[doc = "Determines number of alerts returned per page in response. Permissible value is between 1 to 250. When the \"includeContent\"  filter is selected, maximum value allowed is 25. Default value is 25."]
            pub fn page_count(mut self, page_count: i64) -> Self {
                self.page_count = Some(page_count);
                self
            }
            #[doc = "Sort the query results by input field. Default value is sort by 'lastModifiedDateTime'."]
            pub fn sort_by(mut self, sort_by: impl Into<String>) -> Self {
                self.sort_by = Some(sort_by.into());
                self
            }
            #[doc = "Sort the query results order in either ascending or descending.  Default value is 'desc' for time fields and 'asc' for others."]
            pub fn sort_order(mut self, sort_order: impl Into<String>) -> Self {
                self.sort_order = Some(sort_order.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.AlertsManagement/smartGroups",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-05-05-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-05-05-preview");
                                if let Some(target_resource) = &this.target_resource {
                                    req.url_mut().query_pairs_mut().append_pair("targetResource", target_resource);
                                }
                                if let Some(target_resource_group) = &this.target_resource_group {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("targetResourceGroup", target_resource_group);
                                }
                                if let Some(target_resource_type) = &this.target_resource_type {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("targetResourceType", target_resource_type);
                                }
                                if let Some(monitor_service) = &this.monitor_service {
                                    req.url_mut().query_pairs_mut().append_pair("monitorService", monitor_service);
                                }
                                if let Some(monitor_condition) = &this.monitor_condition {
                                    req.url_mut().query_pairs_mut().append_pair("monitorCondition", monitor_condition);
                                }
                                if let Some(severity) = &this.severity {
                                    req.url_mut().query_pairs_mut().append_pair("severity", severity);
                                }
                                if let Some(smart_group_state) = &this.smart_group_state {
                                    req.url_mut().query_pairs_mut().append_pair("smartGroupState", smart_group_state);
                                }
                                if let Some(time_range) = &this.time_range {
                                    req.url_mut().query_pairs_mut().append_pair("timeRange", time_range);
                                }
                                if let Some(page_count) = &this.page_count {
                                    req.url_mut().query_pairs_mut().append_pair("pageCount", &page_count.to_string());
                                }
                                if let Some(sort_by) = &this.sort_by {
                                    req.url_mut().query_pairs_mut().append_pair("sortBy", sort_by);
                                }
                                if let Some(sort_order) = &this.sort_order {
                                    req.url_mut().query_pairs_mut().append_pair("sortOrder", sort_order);
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
                                let rsp_value: models::SmartGroupsList = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_by_id {
        use super::models;
        type Response = models::SmartGroup;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) smart_group_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.AlertsManagement/smartGroups/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.smart_group_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-05-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SmartGroup = serde_json::from_slice(&rsp_body)?;
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
    pub mod change_state {
        use super::models;
        type Response = models::SmartGroup;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) smart_group_id: String,
            pub(crate) new_state: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.AlertsManagement/smartGroups/{}/changeState",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.smart_group_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-05-preview");
                        let new_state = &this.new_state;
                        req.url_mut().query_pairs_mut().append_pair("newState", new_state);
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SmartGroup = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_history {
        use super::models;
        type Response = models::SmartGroupModification;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) smart_group_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.AlertsManagement/smartGroups/{}/history",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.smart_group_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-05-05-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SmartGroupModification = serde_json::from_slice(&rsp_body)?;
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
