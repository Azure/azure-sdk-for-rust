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
    pub fn grants_client(&self) -> grants::Client {
        grants::Client(self.clone())
    }
    pub fn join_requests_client(&self) -> join_requests::Client {
        join_requests::Client(self.clone())
    }
    pub fn labs_client(&self) -> labs::Client {
        labs::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn student_labs_client(&self) -> student_labs::Client {
        student_labs::Client(self.clone())
    }
    pub fn students_client(&self) -> students::Client {
        students::Client(self.clone())
    }
}
pub mod operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the available Microsoft.Education API operations."]
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
                        let url = azure_core::Url::parse(&format!("{}/providers/Microsoft.Education/operations", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
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
pub mod grants {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of grants that Microsoft has provided."]
        pub fn list_all(&self) -> list_all::Builder {
            list_all::Builder {
                client: self.0.clone(),
                include_allocated_budget: None,
            }
        }
        #[doc = "Get details for a specific grant linked to the provided billing account and billing profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_name`: Billing account name."]
        #[doc = "* `billing_profile_name`: Billing profile name."]
        pub fn list(&self, billing_account_name: impl Into<String>, billing_profile_name: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                billing_account_name: billing_account_name.into(),
                billing_profile_name: billing_profile_name.into(),
                include_allocated_budget: None,
            }
        }
        #[doc = "Get details for a specific grant linked to the provided billing account and billing profile."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_name`: Billing account name."]
        #[doc = "* `billing_profile_name`: Billing profile name."]
        pub fn get(&self, billing_account_name: impl Into<String>, billing_profile_name: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                billing_account_name: billing_account_name.into(),
                billing_profile_name: billing_profile_name.into(),
                include_allocated_budget: None,
            }
        }
    }
    pub mod list_all {
        use super::models;
        type Response = models::GrantListResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) include_allocated_budget: Option<bool>,
        }
        impl Builder {
            #[doc = "May be used to include information about budget that has been allocated."]
            pub fn include_allocated_budget(mut self, include_allocated_budget: bool) -> Self {
                self.include_allocated_budget = Some(include_allocated_budget);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!("{}/providers/Microsoft.Education/grants", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
                                if let Some(include_allocated_budget) = &this.include_allocated_budget {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("includeAllocatedBudget", &include_allocated_budget.to_string());
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
                                let rsp_value: models::GrantListResponse = serde_json::from_slice(&rsp_body)?;
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
    pub mod list {
        use super::models;
        type Response = models::GrantListResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_name: String,
            pub(crate) billing_profile_name: String,
            pub(crate) include_allocated_budget: Option<bool>,
        }
        impl Builder {
            #[doc = "May be used to include information about budget that has been allocated."]
            pub fn include_allocated_budget(mut self, include_allocated_budget: bool) -> Self {
                self.include_allocated_budget = Some(include_allocated_budget);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Billing/billingAccounts/{}/billingProfiles/{}/providers/Microsoft.Education/grants",
                            this.client.endpoint(),
                            &this.billing_account_name,
                            &this.billing_profile_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
                                if let Some(include_allocated_budget) = &this.include_allocated_budget {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("includeAllocatedBudget", &include_allocated_budget.to_string());
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
                                let rsp_value: models::GrantListResponse = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::GrantDetails;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_name: String,
            pub(crate) billing_profile_name: String,
            pub(crate) include_allocated_budget: Option<bool>,
        }
        impl Builder {
            #[doc = "May be used to include information about budget that has been allocated."]
            pub fn include_allocated_budget(mut self, include_allocated_budget: bool) -> Self {
                self.include_allocated_budget = Some(include_allocated_budget);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/providers/Microsoft.Billing/billingAccounts/{}/billingProfiles/{}/providers/Microsoft.Education/grants/default" , this . client . endpoint () , & this . billing_account_name , & this . billing_profile_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
                        if let Some(include_allocated_budget) = &this.include_allocated_budget {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeAllocatedBudget", &include_allocated_budget.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::GrantDetails = serde_json::from_slice(&rsp_body)?;
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
pub mod labs {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of labs associated with the provided billing account name and billing profile name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_name`: Billing account name."]
        #[doc = "* `billing_profile_name`: Billing profile name."]
        pub fn list_all(&self, billing_account_name: impl Into<String>, billing_profile_name: impl Into<String>) -> list_all::Builder {
            list_all::Builder {
                client: self.0.clone(),
                billing_account_name: billing_account_name.into(),
                billing_profile_name: billing_profile_name.into(),
                include_budget: None,
                include_deleted: None,
            }
        }
        #[doc = "Get the details for a specific lab associated with the provided billing account name, billing profile name, and invoice section name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_name`: Billing account name."]
        #[doc = "* `billing_profile_name`: Billing profile name."]
        #[doc = "* `invoice_section_name`: Invoice section name."]
        pub fn list(
            &self,
            billing_account_name: impl Into<String>,
            billing_profile_name: impl Into<String>,
            invoice_section_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                billing_account_name: billing_account_name.into(),
                billing_profile_name: billing_profile_name.into(),
                invoice_section_name: invoice_section_name.into(),
                include_budget: None,
            }
        }
        #[doc = "Get the details for a specific lab associated with the provided billing account name, billing profile name, and invoice section name."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_name`: Billing account name."]
        #[doc = "* `billing_profile_name`: Billing profile name."]
        #[doc = "* `invoice_section_name`: Invoice section name."]
        pub fn get(
            &self,
            billing_account_name: impl Into<String>,
            billing_profile_name: impl Into<String>,
            invoice_section_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                billing_account_name: billing_account_name.into(),
                billing_profile_name: billing_profile_name.into(),
                invoice_section_name: invoice_section_name.into(),
                include_budget: None,
            }
        }
        #[doc = "Create a new lab or update a previously created lab."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_name`: Billing account name."]
        #[doc = "* `billing_profile_name`: Billing profile name."]
        #[doc = "* `invoice_section_name`: Invoice section name."]
        #[doc = "* `parameters`: Request parameters that are provided to create lab resource."]
        pub fn create_or_update(
            &self,
            billing_account_name: impl Into<String>,
            billing_profile_name: impl Into<String>,
            invoice_section_name: impl Into<String>,
            parameters: impl Into<models::LabDetails>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                billing_account_name: billing_account_name.into(),
                billing_profile_name: billing_profile_name.into(),
                invoice_section_name: invoice_section_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Delete a specific lab associated with the provided billing account name, billing profile name, and invoice section name. Note all students must be removed from the lab in order to delete the lab."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_name`: Billing account name."]
        #[doc = "* `billing_profile_name`: Billing profile name."]
        #[doc = "* `invoice_section_name`: Invoice section name."]
        pub fn delete(
            &self,
            billing_account_name: impl Into<String>,
            billing_profile_name: impl Into<String>,
            invoice_section_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                billing_account_name: billing_account_name.into(),
                billing_profile_name: billing_profile_name.into(),
                invoice_section_name: invoice_section_name.into(),
            }
        }
        #[doc = "Generate invite code for a lab"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_name`: Billing account name."]
        #[doc = "* `billing_profile_name`: Billing profile name."]
        #[doc = "* `invoice_section_name`: Invoice section name."]
        #[doc = "* `parameters`: Request parameters that are provided to generate invite code."]
        pub fn generate_invite_code(
            &self,
            billing_account_name: impl Into<String>,
            billing_profile_name: impl Into<String>,
            invoice_section_name: impl Into<String>,
            parameters: impl Into<models::InviteCodeGenerateRequest>,
        ) -> generate_invite_code::Builder {
            generate_invite_code::Builder {
                client: self.0.clone(),
                billing_account_name: billing_account_name.into(),
                billing_profile_name: billing_profile_name.into(),
                invoice_section_name: invoice_section_name.into(),
                parameters: parameters.into(),
                only_update_student_count_parameter: None,
            }
        }
    }
    pub mod list_all {
        use super::models;
        type Response = models::LabListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_name: String,
            pub(crate) billing_profile_name: String,
            pub(crate) include_budget: Option<bool>,
            pub(crate) include_deleted: Option<bool>,
        }
        impl Builder {
            #[doc = "May be used to include budget information."]
            pub fn include_budget(mut self, include_budget: bool) -> Self {
                self.include_budget = Some(include_budget);
                self
            }
            #[doc = "May be used to show deleted items."]
            pub fn include_deleted(mut self, include_deleted: bool) -> Self {
                self.include_deleted = Some(include_deleted);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Billing/billingAccounts/{}/billingProfiles/{}/providers/Microsoft.Education/labs",
                            this.client.endpoint(),
                            &this.billing_account_name,
                            &this.billing_profile_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
                                if let Some(include_budget) = &this.include_budget {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("includeBudget", &include_budget.to_string());
                                }
                                if let Some(include_deleted) = &this.include_deleted {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("includeDeleted", &include_deleted.to_string());
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
                                let rsp_value: models::LabListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod list {
        use super::models;
        type Response = models::LabListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_name: String,
            pub(crate) billing_profile_name: String,
            pub(crate) invoice_section_name: String,
            pub(crate) include_budget: Option<bool>,
        }
        impl Builder {
            #[doc = "May be used to include budget information."]
            pub fn include_budget(mut self, include_budget: bool) -> Self {
                self.include_budget = Some(include_budget);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/providers/Microsoft.Billing/billingAccounts/{}/billingProfiles/{}/invoiceSections/{}/providers/Microsoft.Education/labs" , this . client . endpoint () , & this . billing_account_name , & this . billing_profile_name , & this . invoice_section_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
                                if let Some(include_budget) = &this.include_budget {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("includeBudget", &include_budget.to_string());
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
                                let rsp_value: models::LabListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::LabDetails;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_name: String,
            pub(crate) billing_profile_name: String,
            pub(crate) invoice_section_name: String,
            pub(crate) include_budget: Option<bool>,
        }
        impl Builder {
            #[doc = "May be used to include budget information."]
            pub fn include_budget(mut self, include_budget: bool) -> Self {
                self.include_budget = Some(include_budget);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/providers/Microsoft.Billing/billingAccounts/{}/billingProfiles/{}/invoiceSections/{}/providers/Microsoft.Education/labs/default" , this . client . endpoint () , & this . billing_account_name , & this . billing_profile_name , & this . invoice_section_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
                        if let Some(include_budget) = &this.include_budget {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("includeBudget", &include_budget.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::LabDetails = serde_json::from_slice(&rsp_body)?;
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
            Created201(models::LabDetails),
            Ok200(models::LabDetails),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_name: String,
            pub(crate) billing_profile_name: String,
            pub(crate) invoice_section_name: String,
            pub(crate) parameters: models::LabDetails,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/providers/Microsoft.Billing/billingAccounts/{}/billingProfiles/{}/invoiceSections/{}/providers/Microsoft.Education/labs/default" , this . client . endpoint () , & this . billing_account_name , & this . billing_profile_name , & this . invoice_section_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::LabDetails = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::LabDetails = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
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
            pub(crate) billing_account_name: String,
            pub(crate) billing_profile_name: String,
            pub(crate) invoice_section_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/providers/Microsoft.Billing/billingAccounts/{}/billingProfiles/{}/invoiceSections/{}/providers/Microsoft.Education/labs/default" , this . client . endpoint () , & this . billing_account_name , & this . billing_profile_name , & this . invoice_section_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
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
    pub mod generate_invite_code {
        use super::models;
        type Response = models::LabDetails;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_name: String,
            pub(crate) billing_profile_name: String,
            pub(crate) invoice_section_name: String,
            pub(crate) parameters: models::InviteCodeGenerateRequest,
            pub(crate) only_update_student_count_parameter: Option<bool>,
        }
        impl Builder {
            #[doc = "set this flag to true if you want to update student count without generating a new invite code"]
            pub fn only_update_student_count_parameter(mut self, only_update_student_count_parameter: bool) -> Self {
                self.only_update_student_count_parameter = Some(only_update_student_count_parameter);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/providers/Microsoft.Billing/billingAccounts/{}/billingProfiles/{}/invoiceSections/{}/providers/Microsoft.Education/labs/default/generateInviteCode" , this . client . endpoint () , & this . billing_account_name , & this . billing_profile_name , & this . invoice_section_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
                        if let Some(only_update_student_count_parameter) = &this.only_update_student_count_parameter {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("onlyUpdateStudentCountParameter", &only_update_student_count_parameter.to_string());
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::LabDetails = serde_json::from_slice(&rsp_body)?;
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
pub mod join_requests {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "get student join requests"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_name`: Billing account name."]
        #[doc = "* `billing_profile_name`: Billing profile name."]
        #[doc = "* `invoice_section_name`: Invoice section name."]
        pub fn list(
            &self,
            billing_account_name: impl Into<String>,
            billing_profile_name: impl Into<String>,
            invoice_section_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                billing_account_name: billing_account_name.into(),
                billing_profile_name: billing_profile_name.into(),
                invoice_section_name: invoice_section_name.into(),
                include_denied: None,
            }
        }
        #[doc = "get student join requests"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_name`: Billing account name."]
        #[doc = "* `billing_profile_name`: Billing profile name."]
        #[doc = "* `invoice_section_name`: Invoice section name."]
        #[doc = "* `join_request_name`: Join name"]
        pub fn get(
            &self,
            billing_account_name: impl Into<String>,
            billing_profile_name: impl Into<String>,
            invoice_section_name: impl Into<String>,
            join_request_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                billing_account_name: billing_account_name.into(),
                billing_profile_name: billing_profile_name.into(),
                invoice_section_name: invoice_section_name.into(),
                join_request_name: join_request_name.into(),
            }
        }
        #[doc = "Approve student joining the redeemable lab"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_name`: Billing account name."]
        #[doc = "* `billing_profile_name`: Billing profile name."]
        #[doc = "* `invoice_section_name`: Invoice section name."]
        #[doc = "* `join_request_name`: Join name"]
        pub fn approve(
            &self,
            billing_account_name: impl Into<String>,
            billing_profile_name: impl Into<String>,
            invoice_section_name: impl Into<String>,
            join_request_name: impl Into<String>,
        ) -> approve::Builder {
            approve::Builder {
                client: self.0.clone(),
                billing_account_name: billing_account_name.into(),
                billing_profile_name: billing_profile_name.into(),
                invoice_section_name: invoice_section_name.into(),
                join_request_name: join_request_name.into(),
            }
        }
        #[doc = "Deny student joining the redeemable lab"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_name`: Billing account name."]
        #[doc = "* `billing_profile_name`: Billing profile name."]
        #[doc = "* `invoice_section_name`: Invoice section name."]
        #[doc = "* `join_request_name`: Join name"]
        pub fn deny(
            &self,
            billing_account_name: impl Into<String>,
            billing_profile_name: impl Into<String>,
            invoice_section_name: impl Into<String>,
            join_request_name: impl Into<String>,
        ) -> deny::Builder {
            deny::Builder {
                client: self.0.clone(),
                billing_account_name: billing_account_name.into(),
                billing_profile_name: billing_profile_name.into(),
                invoice_section_name: invoice_section_name.into(),
                join_request_name: join_request_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::JoinRequestList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_name: String,
            pub(crate) billing_profile_name: String,
            pub(crate) invoice_section_name: String,
            pub(crate) include_denied: Option<bool>,
        }
        impl Builder {
            #[doc = "Include denied"]
            pub fn include_denied(mut self, include_denied: bool) -> Self {
                self.include_denied = Some(include_denied);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/providers/Microsoft.Billing/billingAccounts/{}/billingProfiles/{}/invoiceSections/{}/providers/Microsoft.Education/labs/default/joinRequests" , this . client . endpoint () , & this . billing_account_name , & this . billing_profile_name , & this . invoice_section_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
                                if let Some(include_denied) = &this.include_denied {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("includeDenied", &include_denied.to_string());
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
                                let rsp_value: models::JoinRequestList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::JoinRequestDetails;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_name: String,
            pub(crate) billing_profile_name: String,
            pub(crate) invoice_section_name: String,
            pub(crate) join_request_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/providers/Microsoft.Billing/billingAccounts/{}/billingProfiles/{}/invoiceSections/{}/providers/Microsoft.Education/labs/default/joinRequests/{}" , this . client . endpoint () , & this . billing_account_name , & this . billing_profile_name , & this . invoice_section_name , & this . join_request_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::JoinRequestDetails = serde_json::from_slice(&rsp_body)?;
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
    pub mod approve {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_name: String,
            pub(crate) billing_profile_name: String,
            pub(crate) invoice_section_name: String,
            pub(crate) join_request_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/providers/Microsoft.Billing/billingAccounts/{}/billingProfiles/{}/invoiceSections/{}/providers/Microsoft.Education/labs/default/joinRequests/{}/approve" , this . client . endpoint () , & this . billing_account_name , & this . billing_profile_name , & this . invoice_section_name , & this . join_request_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
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
    pub mod deny {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_name: String,
            pub(crate) billing_profile_name: String,
            pub(crate) invoice_section_name: String,
            pub(crate) join_request_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/providers/Microsoft.Billing/billingAccounts/{}/billingProfiles/{}/invoiceSections/{}/providers/Microsoft.Education/labs/default/joinRequests/{}/deny" , this . client . endpoint () , & this . billing_account_name , & this . billing_profile_name , & this . invoice_section_name , & this . join_request_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
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
    #[doc = "Redeem invite code to join a redeemable lab"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `parameters`: Request parameters to provide redeem code."]
    pub fn redeem_invitation_code(&self, parameters: impl Into<models::RedeemRequest>) -> redeem_invitation_code::Builder {
        redeem_invitation_code::Builder {
            client: self.clone(),
            parameters: parameters.into(),
        }
    }
}
pub mod redeem_invitation_code {
    use super::models;
    type Response = ();
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) parameters: models::RedeemRequest,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/providers/Microsoft.Education/redeemInvitationCode",
                        this.client.endpoint(),
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
                        .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
                    req.insert_header("content-type", "application/json");
                    let req_body = azure_core::to_json(&this.parameters)?;
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
pub mod students {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of details about students that are associated with the specified lab."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_name`: Billing account name."]
        #[doc = "* `billing_profile_name`: Billing profile name."]
        #[doc = "* `invoice_section_name`: Invoice section name."]
        pub fn list(
            &self,
            billing_account_name: impl Into<String>,
            billing_profile_name: impl Into<String>,
            invoice_section_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                billing_account_name: billing_account_name.into(),
                billing_profile_name: billing_profile_name.into(),
                invoice_section_name: invoice_section_name.into(),
                include_deleted: None,
            }
        }
        #[doc = "Get the details for a specific student in the specified lab by student alias"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_name`: Billing account name."]
        #[doc = "* `billing_profile_name`: Billing profile name."]
        #[doc = "* `invoice_section_name`: Invoice section name."]
        #[doc = "* `student_alias`: Student alias."]
        pub fn get(
            &self,
            billing_account_name: impl Into<String>,
            billing_profile_name: impl Into<String>,
            invoice_section_name: impl Into<String>,
            student_alias: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                billing_account_name: billing_account_name.into(),
                billing_profile_name: billing_profile_name.into(),
                invoice_section_name: invoice_section_name.into(),
                student_alias: student_alias.into(),
            }
        }
        #[doc = "Create and add a new student to the specified lab or update the details of an existing student in a lab. Note the student must have a valid tenant to accept the lab after they have been added to lab."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_name`: Billing account name."]
        #[doc = "* `billing_profile_name`: Billing profile name."]
        #[doc = "* `invoice_section_name`: Invoice section name."]
        #[doc = "* `student_alias`: Student alias."]
        #[doc = "* `parameters`: Request parameters that are provided to update student properties."]
        pub fn create_or_update(
            &self,
            billing_account_name: impl Into<String>,
            billing_profile_name: impl Into<String>,
            invoice_section_name: impl Into<String>,
            student_alias: impl Into<String>,
            parameters: impl Into<models::StudentDetails>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                billing_account_name: billing_account_name.into(),
                billing_profile_name: billing_profile_name.into(),
                invoice_section_name: invoice_section_name.into(),
                student_alias: student_alias.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Delete the specified student based on the student alias."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `billing_account_name`: Billing account name."]
        #[doc = "* `billing_profile_name`: Billing profile name."]
        #[doc = "* `invoice_section_name`: Invoice section name."]
        #[doc = "* `student_alias`: Student alias."]
        pub fn delete(
            &self,
            billing_account_name: impl Into<String>,
            billing_profile_name: impl Into<String>,
            invoice_section_name: impl Into<String>,
            student_alias: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                billing_account_name: billing_account_name.into(),
                billing_profile_name: billing_profile_name.into(),
                invoice_section_name: invoice_section_name.into(),
                student_alias: student_alias.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::StudentListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_name: String,
            pub(crate) billing_profile_name: String,
            pub(crate) invoice_section_name: String,
            pub(crate) include_deleted: Option<bool>,
        }
        impl Builder {
            #[doc = "May be used to show deleted items."]
            pub fn include_deleted(mut self, include_deleted: bool) -> Self {
                self.include_deleted = Some(include_deleted);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/providers/Microsoft.Billing/billingAccounts/{}/billingProfiles/{}/invoiceSections/{}/providers/Microsoft.Education/labs/default/students" , this . client . endpoint () , & this . billing_account_name , & this . billing_profile_name , & this . invoice_section_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
                                if let Some(include_deleted) = &this.include_deleted {
                                    req.url_mut()
                                        .query_pairs_mut()
                                        .append_pair("includeDeleted", &include_deleted.to_string());
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
                                let rsp_value: models::StudentListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::StudentDetails;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_name: String,
            pub(crate) billing_profile_name: String,
            pub(crate) invoice_section_name: String,
            pub(crate) student_alias: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/providers/Microsoft.Billing/billingAccounts/{}/billingProfiles/{}/invoiceSections/{}/providers/Microsoft.Education/labs/default/students/{}" , this . client . endpoint () , & this . billing_account_name , & this . billing_profile_name , & this . invoice_section_name , & this . student_alias)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::StudentDetails = serde_json::from_slice(&rsp_body)?;
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
            Created201(models::StudentDetails),
            Ok200(models::StudentDetails),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) billing_account_name: String,
            pub(crate) billing_profile_name: String,
            pub(crate) invoice_section_name: String,
            pub(crate) student_alias: String,
            pub(crate) parameters: models::StudentDetails,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/providers/Microsoft.Billing/billingAccounts/{}/billingProfiles/{}/invoiceSections/{}/providers/Microsoft.Education/labs/default/students/{}" , this . client . endpoint () , & this . billing_account_name , & this . billing_profile_name , & this . invoice_section_name , & this . student_alias)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::StudentDetails = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::StudentDetails = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
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
            pub(crate) billing_account_name: String,
            pub(crate) billing_profile_name: String,
            pub(crate) invoice_section_name: String,
            pub(crate) student_alias: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/providers/Microsoft.Billing/billingAccounts/{}/billingProfiles/{}/invoiceSections/{}/providers/Microsoft.Education/labs/default/students/{}" , this . client . endpoint () , & this . billing_account_name , & this . billing_profile_name , & this . invoice_section_name , & this . student_alias)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
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
pub mod student_labs {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get a list of all labs associated with the caller of the API."]
        pub fn list_all(&self) -> list_all::Builder {
            list_all::Builder { client: self.0.clone() }
        }
        #[doc = "Get the details for a specified lab associated with the student lab."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `student_lab_name`: Student lab name."]
        pub fn get(&self, student_lab_name: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                student_lab_name: student_lab_name.into(),
            }
        }
    }
    pub mod list_all {
        use super::models;
        type Response = models::StudentLabListResult;
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
                            azure_core::Url::parse(&format!("{}/providers/Microsoft.Education/studentLabs", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::StudentLabListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::StudentLabDetails;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) student_lab_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.Education/studentLabs/{}",
                            this.client.endpoint(),
                            &this.student_lab_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-12-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::StudentLabDetails = serde_json::from_slice(&rsp_body)?;
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
