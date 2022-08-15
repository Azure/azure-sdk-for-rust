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
    pub fn assessed_machines_client(&self) -> assessed_machines::Client {
        assessed_machines::Client(self.clone())
    }
    pub fn assessment_options_client(&self) -> assessment_options::Client {
        assessment_options::Client(self.clone())
    }
    pub fn assessments_client(&self) -> assessments::Client {
        assessments::Client(self.clone())
    }
    pub fn groups_client(&self) -> groups::Client {
        groups::Client(self.clone())
    }
    pub fn location_client(&self) -> location::Client {
        location::Client(self.clone())
    }
    pub fn machines_client(&self) -> machines::Client {
        machines::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn projects_client(&self) -> projects::Client {
        projects::Client(self.clone())
    }
}
pub mod location {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Checks whether the project name is available in the specified region."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `location_name`: The desired region for the name check."]
        #[doc = "* `subscription_id`: Azure Subscription Id in which project was created."]
        #[doc = "* `parameters`: Properties needed to check the availability of a name."]
        pub fn check_name_availability(
            &self,
            location_name: impl Into<String>,
            subscription_id: impl Into<String>,
            parameters: impl Into<models::CheckNameAvailabilityParameters>,
        ) -> check_name_availability::Builder {
            check_name_availability::Builder {
                client: self.0.clone(),
                location_name: location_name.into(),
                subscription_id: subscription_id.into(),
                parameters: parameters.into(),
            }
        }
    }
    pub mod check_name_availability {
        use super::models;
        type Response = models::CheckNameAvailabilityResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) location_name: String,
            pub(crate) subscription_id: String,
            pub(crate) parameters: models::CheckNameAvailabilityParameters,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.Migrate/locations/{}/checkNameAvailability",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.location_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-02-02");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CheckNameAvailabilityResult = serde_json::from_slice(&rsp_body)?;
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
pub mod assessment_options {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get the assessment options."]
        #[doc = "Get the available options for the properties of an assessment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which project was created."]
        #[doc = "* `location_name`: Azure region in which the project is created."]
        pub fn get(&self, subscription_id: impl Into<String>, location_name: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                location_name: location_name.into(),
                accept_language: None,
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::AssessmentOptionsResultList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) location_name: String,
            pub(crate) accept_language: Option<String>,
        }
        impl Builder {
            #[doc = "Standard request header. Used by service to respond to client in appropriate language."]
            pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
                self.accept_language = Some(accept_language.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.Migrate/locations/{}/assessmentOptions",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.location_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-02-02");
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AssessmentOptionsResultList = serde_json::from_slice(&rsp_body)?;
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
pub mod projects {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get all projects."]
        #[doc = "Get all the projects in the subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which project was created."]
        pub fn list_by_subscription(&self, subscription_id: impl Into<String>) -> list_by_subscription::Builder {
            list_by_subscription::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                accept_language: None,
            }
        }
        #[doc = "Get all projects."]
        #[doc = "Get all the projects in the resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that project is part of."]
        pub fn list_by_resource_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
        ) -> list_by_resource_group::Builder {
            list_by_resource_group::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                accept_language: None,
            }
        }
        #[doc = "Get the specified project."]
        #[doc = "Get the project with the specified name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that project is part of."]
        #[doc = "* `project_name`: Name of the Azure Migrate project."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            project_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                project_name: project_name.into(),
                accept_language: None,
            }
        }
        #[doc = "Create or update project."]
        #[doc = "Create a project with specified name. If a project already exists, update it."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that project is part of."]
        #[doc = "* `project_name`: Name of the Azure Migrate project."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            project_name: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                project_name: project_name.into(),
                accept_language: None,
                project: None,
            }
        }
        #[doc = "Update project."]
        #[doc = "Update a project with specified name. Supports partial updates, for example only tags can be provided."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that project is part of."]
        #[doc = "* `project_name`: Name of the Azure Migrate project."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            project_name: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                project_name: project_name.into(),
                accept_language: None,
                project: None,
            }
        }
        #[doc = "Delete the project"]
        #[doc = "Delete the project. Deleting non-existent project is a no-operation."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that project is part of."]
        #[doc = "* `project_name`: Name of the Azure Migrate project."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            project_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                project_name: project_name.into(),
                accept_language: None,
            }
        }
        #[doc = "Get shared keys for the project."]
        #[doc = "Gets the Log Analytics Workspace ID and Primary Key for the specified project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that project is part of."]
        #[doc = "* `project_name`: Name of the Azure Migrate project."]
        pub fn get_keys(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            project_name: impl Into<String>,
        ) -> get_keys::Builder {
            get_keys::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                project_name: project_name.into(),
                accept_language: None,
            }
        }
    }
    pub mod list_by_subscription {
        use super::models;
        type Response = models::ProjectResultList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) accept_language: Option<String>,
        }
        impl Builder {
            #[doc = "Standard request header. Used by service to respond to client in appropriate language."]
            pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
                self.accept_language = Some(accept_language.into());
                self
            }
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.Migrate/projects",
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-02-02");
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProjectResultList = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_by_resource_group {
        use super::models;
        type Response = models::ProjectResultList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) accept_language: Option<String>,
        }
        impl Builder {
            #[doc = "Standard request header. Used by service to respond to client in appropriate language."]
            pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
                self.accept_language = Some(accept_language.into());
                self
            }
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourcegroups/{}/providers/Microsoft.Migrate/projects",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-02-02");
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProjectResultList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Project;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) project_name: String,
            pub(crate) accept_language: Option<String>,
        }
        impl Builder {
            #[doc = "Standard request header. Used by service to respond to client in appropriate language."]
            pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
                self.accept_language = Some(accept_language.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourcegroups/{}/providers/Microsoft.Migrate/projects/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.project_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-02-02");
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Project = serde_json::from_slice(&rsp_body)?;
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
    pub mod create {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Project),
            Created201(models::Project),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) project_name: String,
            pub(crate) accept_language: Option<String>,
            pub(crate) project: Option<models::Project>,
        }
        impl Builder {
            #[doc = "Standard request header. Used by service to respond to client in appropriate language."]
            pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
                self.accept_language = Some(accept_language.into());
                self
            }
            #[doc = "New or Updated project object."]
            pub fn project(mut self, project: impl Into<models::Project>) -> Self {
                self.project = Some(project.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourcegroups/{}/providers/Microsoft.Migrate/projects/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.project_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-02-02");
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        let req_body = if let Some(project) = &this.project {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(project)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Project = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Project = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Project;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) project_name: String,
            pub(crate) accept_language: Option<String>,
            pub(crate) project: Option<models::Project>,
        }
        impl Builder {
            #[doc = "Standard request header. Used by service to respond to client in appropriate language."]
            pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
                self.accept_language = Some(accept_language.into());
                self
            }
            #[doc = "Updated project object."]
            pub fn project(mut self, project: impl Into<models::Project>) -> Self {
                self.project = Some(project.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourcegroups/{}/providers/Microsoft.Migrate/projects/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.project_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-02-02");
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        let req_body = if let Some(project) = &this.project {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(project)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Project = serde_json::from_slice(&rsp_body)?;
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
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) project_name: String,
            pub(crate) accept_language: Option<String>,
        }
        impl Builder {
            #[doc = "Standard request header. Used by service to respond to client in appropriate language."]
            pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
                self.accept_language = Some(accept_language.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourcegroups/{}/providers/Microsoft.Migrate/projects/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.project_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-02-02");
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        let req_body = azure_core::EMPTY_BODY;
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
    pub mod get_keys {
        use super::models;
        type Response = models::ProjectKey;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) project_name: String,
            pub(crate) accept_language: Option<String>,
        }
        impl Builder {
            #[doc = "Standard request header. Used by service to respond to client in appropriate language."]
            pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
                self.accept_language = Some(accept_language.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourcegroups/{}/providers/Microsoft.Migrate/projects/{}/keys",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.project_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-02-02");
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProjectKey = serde_json::from_slice(&rsp_body)?;
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
pub mod machines {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get all machines in the project"]
        #[doc = "Get data of all the machines available in the project. Returns a json array of objects of type 'machine' defined in Models section."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that project is part of."]
        #[doc = "* `project_name`: Name of the Azure Migrate project."]
        pub fn list_by_project(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            project_name: impl Into<String>,
        ) -> list_by_project::Builder {
            list_by_project::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                project_name: project_name.into(),
                accept_language: None,
            }
        }
        #[doc = "Get a specific machine."]
        #[doc = "Get the machine with the specified name. Returns a json object of type 'machine' defined in Models section."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that project is part of."]
        #[doc = "* `project_name`: Name of the Azure Migrate project."]
        #[doc = "* `machine_name`: Unique name of a machine in private datacenter."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            project_name: impl Into<String>,
            machine_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                project_name: project_name.into(),
                machine_name: machine_name.into(),
                accept_language: None,
            }
        }
    }
    pub mod list_by_project {
        use super::models;
        type Response = models::MachineResultList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) project_name: String,
            pub(crate) accept_language: Option<String>,
        }
        impl Builder {
            #[doc = "Standard request header. Used by service to respond to client in appropriate language."]
            pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
                self.accept_language = Some(accept_language.into());
                self
            }
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/projects/{}/machines",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.project_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-02-02");
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MachineResultList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Machine;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) project_name: String,
            pub(crate) machine_name: String,
            pub(crate) accept_language: Option<String>,
        }
        impl Builder {
            #[doc = "Standard request header. Used by service to respond to client in appropriate language."]
            pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
                self.accept_language = Some(accept_language.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/projects/{}/machines/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.project_name,
                            &this.machine_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-02-02");
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Machine = serde_json::from_slice(&rsp_body)?;
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
pub mod groups {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get all groups"]
        #[doc = "Get all groups created in the project. Returns a json array of objects of type 'group' as specified in the Models section."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that project is part of."]
        #[doc = "* `project_name`: Name of the Azure Migrate project."]
        pub fn list_by_project(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            project_name: impl Into<String>,
        ) -> list_by_project::Builder {
            list_by_project::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                project_name: project_name.into(),
                accept_language: None,
            }
        }
        #[doc = "Get a specific group."]
        #[doc = "Get information related to a specific group in the project. Returns a json object of type 'group' as specified in the models section."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that project is part of."]
        #[doc = "* `project_name`: Name of the Azure Migrate project."]
        #[doc = "* `group_name`: Unique name of a group within a project."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            project_name: impl Into<String>,
            group_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                project_name: project_name.into(),
                group_name: group_name.into(),
                accept_language: None,
            }
        }
        #[doc = "Create a new group with specified settings. If group with the name provided already exists, then the existing group is updated."]
        #[doc = "Create a new group by sending a json object of type 'group' as given in Models section as part of the Request Body. The group name in a project is unique. Labels can be applied on a group as part of creation.\n\nIf a group with the groupName specified in the URL already exists, then this call acts as an update.\n\nThis operation is Idempotent.\n"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that project is part of."]
        #[doc = "* `project_name`: Name of the Azure Migrate project."]
        #[doc = "* `group_name`: Unique name of a group within a project."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            project_name: impl Into<String>,
            group_name: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                project_name: project_name.into(),
                group_name: group_name.into(),
                accept_language: None,
                group: None,
            }
        }
        #[doc = "Delete the group"]
        #[doc = "Delete the group from the project. The machines remain in the project. Deleting a non-existent group results in a no-operation.\n\nA group is an aggregation mechanism for machines in a project. Therefore, deleting group does not delete machines in it.\n"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that project is part of."]
        #[doc = "* `project_name`: Name of the Azure Migrate project."]
        #[doc = "* `group_name`: Unique name of a group within a project."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            project_name: impl Into<String>,
            group_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                project_name: project_name.into(),
                group_name: group_name.into(),
                accept_language: None,
            }
        }
    }
    pub mod list_by_project {
        use super::models;
        type Response = models::GroupResultList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) project_name: String,
            pub(crate) accept_language: Option<String>,
        }
        impl Builder {
            #[doc = "Standard request header. Used by service to respond to client in appropriate language."]
            pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
                self.accept_language = Some(accept_language.into());
                self
            }
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/projects/{}/groups",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.project_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-02-02");
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::GroupResultList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Group;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) project_name: String,
            pub(crate) group_name: String,
            pub(crate) accept_language: Option<String>,
        }
        impl Builder {
            #[doc = "Standard request header. Used by service to respond to client in appropriate language."]
            pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
                self.accept_language = Some(accept_language.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/projects/{}/groups/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.project_name,
                            &this.group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-02-02");
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Group = serde_json::from_slice(&rsp_body)?;
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
    pub mod create {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Group),
            Created201(models::Group),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) project_name: String,
            pub(crate) group_name: String,
            pub(crate) accept_language: Option<String>,
            pub(crate) group: Option<models::Group>,
        }
        impl Builder {
            #[doc = "Standard request header. Used by service to respond to client in appropriate language."]
            pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
                self.accept_language = Some(accept_language.into());
                self
            }
            #[doc = "New or Updated Group object."]
            pub fn group(mut self, group: impl Into<models::Group>) -> Self {
                self.group = Some(group.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/projects/{}/groups/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.project_name,
                            &this.group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-02-02");
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        let req_body = if let Some(group) = &this.group {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(group)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Group = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Group = serde_json::from_slice(&rsp_body)?;
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
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) project_name: String,
            pub(crate) group_name: String,
            pub(crate) accept_language: Option<String>,
        }
        impl Builder {
            #[doc = "Standard request header. Used by service to respond to client in appropriate language."]
            pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
                self.accept_language = Some(accept_language.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/projects/{}/groups/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.project_name,
                            &this.group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-02-02");
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        let req_body = azure_core::EMPTY_BODY;
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
pub mod assessments {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get all assessments created for the specified group."]
        #[doc = "Get all assessments created for the specified group.\n\nReturns a json array of objects of type 'assessment' as specified in Models section.\n"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that project is part of."]
        #[doc = "* `project_name`: Name of the Azure Migrate project."]
        #[doc = "* `group_name`: Unique name of a group within a project."]
        pub fn list_by_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            project_name: impl Into<String>,
            group_name: impl Into<String>,
        ) -> list_by_group::Builder {
            list_by_group::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                project_name: project_name.into(),
                group_name: group_name.into(),
                accept_language: None,
            }
        }
        #[doc = "Get all assessments created in the project."]
        #[doc = "Get all assessments created in the project.\n\nReturns a json array of objects of type 'assessment' as specified in Models section.\n"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that project is part of."]
        #[doc = "* `project_name`: Name of the Azure Migrate project."]
        pub fn list_by_project(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            project_name: impl Into<String>,
        ) -> list_by_project::Builder {
            list_by_project::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                project_name: project_name.into(),
                accept_language: None,
            }
        }
        #[doc = "Get an assessment."]
        #[doc = "Get an existing assessment with the specified name. Returns a json object of type 'assessment' as specified in Models section."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that project is part of."]
        #[doc = "* `project_name`: Name of the Azure Migrate project."]
        #[doc = "* `group_name`: Unique name of a group within a project."]
        #[doc = "* `assessment_name`: Unique name of an assessment within a project."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            project_name: impl Into<String>,
            group_name: impl Into<String>,
            assessment_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                project_name: project_name.into(),
                group_name: group_name.into(),
                assessment_name: assessment_name.into(),
                accept_language: None,
            }
        }
        #[doc = "Create or Update assessment."]
        #[doc = "Create a new assessment with the given name and the specified settings. Since name of an assessment in a project is a unique identifier, if an assessment with the name provided already exists, then the existing assessment is updated.\n\nAny PUT operation, resulting in either create or update on an assessment, will cause the assessment to go in a \"InProgress\" state. This will be indicated by the field 'computationState' on the Assessment object. During this time no other PUT operation will be allowed on that assessment object, nor will a Delete operation. Once the computation for the assessment is complete, the field 'computationState' will be updated to 'Ready', and then other PUT or DELETE operations can happen on the assessment.\n\nWhen assessment is under computation, any PUT will lead to a 400 - Bad Request error.\n"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that project is part of."]
        #[doc = "* `project_name`: Name of the Azure Migrate project."]
        #[doc = "* `group_name`: Unique name of a group within a project."]
        #[doc = "* `assessment_name`: Unique name of an assessment within a project."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            project_name: impl Into<String>,
            group_name: impl Into<String>,
            assessment_name: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                project_name: project_name.into(),
                group_name: group_name.into(),
                assessment_name: assessment_name.into(),
                accept_language: None,
                assessment: None,
            }
        }
        #[doc = "Deletes an assessment from the project."]
        #[doc = "Delete an assessment from the project. The machines remain in the assessment. Deleting a non-existent assessment results in a no-operation.\n\nWhen an assessment is under computation, as indicated by the 'computationState' field, it cannot be deleted. Any such attempt will return a 400 - Bad Request.\n"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that project is part of."]
        #[doc = "* `project_name`: Name of the Azure Migrate project."]
        #[doc = "* `group_name`: Unique name of a group within a project."]
        #[doc = "* `assessment_name`: Unique name of an assessment within a project."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            project_name: impl Into<String>,
            group_name: impl Into<String>,
            assessment_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                project_name: project_name.into(),
                group_name: group_name.into(),
                assessment_name: assessment_name.into(),
                accept_language: None,
            }
        }
        #[doc = "Get download URL for the assessment report."]
        #[doc = "Get the URL for downloading the assessment in a report format."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that project is part of."]
        #[doc = "* `project_name`: Name of the Azure Migrate project."]
        #[doc = "* `group_name`: Unique name of a group within a project."]
        #[doc = "* `assessment_name`: Unique name of an assessment within a project."]
        pub fn get_report_download_url(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            project_name: impl Into<String>,
            group_name: impl Into<String>,
            assessment_name: impl Into<String>,
        ) -> get_report_download_url::Builder {
            get_report_download_url::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                project_name: project_name.into(),
                group_name: group_name.into(),
                assessment_name: assessment_name.into(),
                accept_language: None,
            }
        }
    }
    pub mod list_by_group {
        use super::models;
        type Response = models::AssessmentResultList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) project_name: String,
            pub(crate) group_name: String,
            pub(crate) accept_language: Option<String>,
        }
        impl Builder {
            #[doc = "Standard request header. Used by service to respond to client in appropriate language."]
            pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
                self.accept_language = Some(accept_language.into());
                self
            }
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/projects/{}/groups/{}/assessments",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.project_name,
                            &this.group_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-02-02");
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AssessmentResultList = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_by_project {
        use super::models;
        type Response = models::AssessmentResultList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) project_name: String,
            pub(crate) accept_language: Option<String>,
        }
        impl Builder {
            #[doc = "Standard request header. Used by service to respond to client in appropriate language."]
            pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
                self.accept_language = Some(accept_language.into());
                self
            }
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/projects/{}/assessments",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.project_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-02-02");
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AssessmentResultList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Assessment;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) project_name: String,
            pub(crate) group_name: String,
            pub(crate) assessment_name: String,
            pub(crate) accept_language: Option<String>,
        }
        impl Builder {
            #[doc = "Standard request header. Used by service to respond to client in appropriate language."]
            pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
                self.accept_language = Some(accept_language.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/projects/{}/groups/{}/assessments/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.project_name,
                            &this.group_name,
                            &this.assessment_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-02-02");
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Assessment = serde_json::from_slice(&rsp_body)?;
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
    pub mod create {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Assessment),
            Created201(models::Assessment),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) project_name: String,
            pub(crate) group_name: String,
            pub(crate) assessment_name: String,
            pub(crate) accept_language: Option<String>,
            pub(crate) assessment: Option<models::Assessment>,
        }
        impl Builder {
            #[doc = "Standard request header. Used by service to respond to client in appropriate language."]
            pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
                self.accept_language = Some(accept_language.into());
                self
            }
            #[doc = "New or Updated Assessment object."]
            pub fn assessment(mut self, assessment: impl Into<models::Assessment>) -> Self {
                self.assessment = Some(assessment.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/projects/{}/groups/{}/assessments/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.project_name,
                            &this.group_name,
                            &this.assessment_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-02-02");
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        let req_body = if let Some(assessment) = &this.assessment {
                            req.insert_header("content-type", "application/json");
                            azure_core::to_json(assessment)?
                        } else {
                            azure_core::EMPTY_BODY
                        };
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Assessment = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Assessment = serde_json::from_slice(&rsp_body)?;
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
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) project_name: String,
            pub(crate) group_name: String,
            pub(crate) assessment_name: String,
            pub(crate) accept_language: Option<String>,
        }
        impl Builder {
            #[doc = "Standard request header. Used by service to respond to client in appropriate language."]
            pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
                self.accept_language = Some(accept_language.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/projects/{}/groups/{}/assessments/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.project_name,
                            &this.group_name,
                            &this.assessment_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-02-02");
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        let req_body = azure_core::EMPTY_BODY;
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
    pub mod get_report_download_url {
        use super::models;
        type Response = models::DownloadUrl;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) project_name: String,
            pub(crate) group_name: String,
            pub(crate) assessment_name: String,
            pub(crate) accept_language: Option<String>,
        }
        impl Builder {
            #[doc = "Standard request header. Used by service to respond to client in appropriate language."]
            pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
                self.accept_language = Some(accept_language.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/projects/{}/groups/{}/assessments/{}/downloadUrl" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . project_name , & this . group_name , & this . assessment_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2018-02-02");
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DownloadUrl = serde_json::from_slice(&rsp_body)?;
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
pub mod assessed_machines {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get assessed machines for assessment."]
        #[doc = "Get list of machines that assessed as part of the specified assessment. Returns a json array of objects of type 'assessedMachine' as specified in the Models section.\n\nWhenever an assessment is created or updated, it goes under computation. During this phase, the 'status' field of Assessment object reports 'Computing'.\nDuring the period when the assessment is under computation, the list of assessed machines is empty and no assessed machines are returned by this call.\n"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that project is part of."]
        #[doc = "* `project_name`: Name of the Azure Migrate project."]
        #[doc = "* `group_name`: Unique name of a group within a project."]
        #[doc = "* `assessment_name`: Unique name of an assessment within a project."]
        pub fn list_by_assessment(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            project_name: impl Into<String>,
            group_name: impl Into<String>,
            assessment_name: impl Into<String>,
        ) -> list_by_assessment::Builder {
            list_by_assessment::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                project_name: project_name.into(),
                group_name: group_name.into(),
                assessment_name: assessment_name.into(),
                accept_language: None,
            }
        }
        #[doc = "Get an assessed machine."]
        #[doc = "Get an assessed machine with its size & cost estimate that was evaluated in the specified assessment."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that project is part of."]
        #[doc = "* `project_name`: Name of the Azure Migrate project."]
        #[doc = "* `group_name`: Unique name of a group within a project."]
        #[doc = "* `assessment_name`: Unique name of an assessment within a project."]
        #[doc = "* `assessed_machine_name`: Unique name of an assessed machine evaluated as part of an assessment."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            project_name: impl Into<String>,
            group_name: impl Into<String>,
            assessment_name: impl Into<String>,
            assessed_machine_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                project_name: project_name.into(),
                group_name: group_name.into(),
                assessment_name: assessment_name.into(),
                assessed_machine_name: assessed_machine_name.into(),
                accept_language: None,
            }
        }
    }
    pub mod list_by_assessment {
        use super::models;
        type Response = models::AssessedMachineResultList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) project_name: String,
            pub(crate) group_name: String,
            pub(crate) assessment_name: String,
            pub(crate) accept_language: Option<String>,
        }
        impl Builder {
            #[doc = "Standard request header. Used by service to respond to client in appropriate language."]
            pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
                self.accept_language = Some(accept_language.into());
                self
            }
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/projects/{}/groups/{}/assessments/{}/assessedMachines" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . project_name , & this . group_name , & this . assessment_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2018-02-02");
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AssessedMachineResultList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::AssessedMachine;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) project_name: String,
            pub(crate) group_name: String,
            pub(crate) assessment_name: String,
            pub(crate) assessed_machine_name: String,
            pub(crate) accept_language: Option<String>,
        }
        impl Builder {
            #[doc = "Standard request header. Used by service to respond to client in appropriate language."]
            pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
                self.accept_language = Some(accept_language.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/projects/{}/groups/{}/assessments/{}/assessedMachines/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . project_name , & this . group_name , & this . assessment_name , & this . assessed_machine_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2018-02-02");
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AssessedMachine = serde_json::from_slice(&rsp_body)?;
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
pub mod operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get list of operations supported in the API."]
        #[doc = "Get a list of REST API supported by Microsoft.Migrate provider."]
        pub fn list(&self) -> list::Builder {
            list::Builder { client: self.0.clone() }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::OperationResultList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/providers/Microsoft.Migrate/operations", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::OperationResultList = serde_json::from_slice(&rsp_body)?;
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
