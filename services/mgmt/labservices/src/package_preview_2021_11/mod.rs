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
    pub fn images_client(&self) -> images::Client {
        images::Client(self.clone())
    }
    pub fn lab_plans_client(&self) -> lab_plans::Client {
        lab_plans::Client(self.clone())
    }
    pub fn labs_client(&self) -> labs::Client {
        labs::Client(self.clone())
    }
    pub fn operation_results_client(&self) -> operation_results::Client {
        operation_results::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn schedules_client(&self) -> schedules::Client {
        schedules::Client(self.clone())
    }
    pub fn skus_client(&self) -> skus::Client {
        skus::Client(self.clone())
    }
    pub fn usages_client(&self) -> usages::Client {
        usages::Client(self.clone())
    }
    pub fn users_client(&self) -> users::Client {
        users::Client(self.clone())
    }
    pub fn virtual_machines_client(&self) -> virtual_machines::Client {
        virtual_machines::Client(self.clone())
    }
}
pub mod images {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets all images."]
        #[doc = "Gets all images from galleries attached to a lab plan."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_plan_name`: The name of the lab plan that uniquely identifies it within containing resource group. Used in resource URIs and in UI."]
        pub fn list_by_lab_plan(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_plan_name: impl Into<String>,
        ) -> list_by_lab_plan::Builder {
            list_by_lab_plan::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_plan_name: lab_plan_name.into(),
                filter: None,
            }
        }
        #[doc = "Gets an image."]
        #[doc = "Gets an image resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_plan_name`: The name of the lab plan that uniquely identifies it within containing resource group. Used in resource URIs and in UI."]
        #[doc = "* `image_name`: The image name."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_plan_name: impl Into<String>,
            image_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_plan_name: lab_plan_name.into(),
                image_name: image_name.into(),
            }
        }
        #[doc = "Updates an image via PUT."]
        #[doc = "Updates an image resource via PUT. Creating new resources via PUT will not function."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_plan_name`: The name of the lab plan that uniquely identifies it within containing resource group. Used in resource URIs and in UI."]
        #[doc = "* `image_name`: The image name."]
        #[doc = "* `body`: The request body."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_plan_name: impl Into<String>,
            image_name: impl Into<String>,
            body: impl Into<models::Image>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_plan_name: lab_plan_name.into(),
                image_name: image_name.into(),
                body: body.into(),
            }
        }
        #[doc = "Updates an image."]
        #[doc = "Updates an image resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_plan_name`: The name of the lab plan that uniquely identifies it within containing resource group. Used in resource URIs and in UI."]
        #[doc = "* `image_name`: The image name."]
        #[doc = "* `body`: The request body."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_plan_name: impl Into<String>,
            image_name: impl Into<String>,
            body: impl Into<models::ImageUpdate>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_plan_name: lab_plan_name.into(),
                image_name: image_name.into(),
                body: body.into(),
            }
        }
    }
    pub mod list_by_lab_plan {
        use super::models;
        type Response = models::PagedImages;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_plan_name: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "The filter to apply to the operation."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labPlans/{}/images",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_plan_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
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
                                let rsp_value: models::PagedImages = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Image;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_plan_name: String,
            pub(crate) image_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labPlans/{}/images/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_plan_name,
                            &this.image_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Image = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Image;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_plan_name: String,
            pub(crate) image_name: String,
            pub(crate) body: models::Image,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labPlans/{}/images/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_plan_name,
                            &this.image_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Image = serde_json::from_slice(&rsp_body)?;
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
    pub mod update {
        use super::models;
        type Response = models::Image;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_plan_name: String,
            pub(crate) image_name: String,
            pub(crate) body: models::ImageUpdate,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labPlans/{}/images/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_plan_name,
                            &this.image_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Image = serde_json::from_slice(&rsp_body)?;
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
pub mod lab_plans {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get all lab plans for a subscription."]
        #[doc = "Returns a list of all lab plans within a subscription"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        pub fn list_by_subscription(&self, subscription_id: impl Into<String>) -> list_by_subscription::Builder {
            list_by_subscription::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                filter: None,
            }
        }
        #[doc = "Get all lab plans for a subscription and resource group."]
        #[doc = "Returns a list of all lab plans for a subscription and resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
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
        #[doc = "Retrieves a Lab Plan resource."]
        #[doc = "Retrieves the properties of a Lab Plan."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_plan_name`: The name of the lab plan that uniquely identifies it within containing resource group. Used in resource URIs and in UI."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_plan_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_plan_name: lab_plan_name.into(),
            }
        }
        #[doc = "Updates or creates a Lab Plan resource."]
        #[doc = "Operation to create or update a Lab Plan resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_plan_name`: The name of the lab plan that uniquely identifies it within containing resource group. Used in resource URIs and in UI."]
        #[doc = "* `body`: The request body."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_plan_name: impl Into<String>,
            body: impl Into<models::LabPlan>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_plan_name: lab_plan_name.into(),
                body: body.into(),
            }
        }
        #[doc = "Updates a Lab Plan resource."]
        #[doc = "Operation to update a Lab Plan resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_plan_name`: The name of the lab plan that uniquely identifies it within containing resource group. Used in resource URIs and in UI."]
        #[doc = "* `body`: The request body."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_plan_name: impl Into<String>,
            body: impl Into<models::LabPlanUpdate>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_plan_name: lab_plan_name.into(),
                body: body.into(),
            }
        }
        #[doc = "Deletes a Lab Plan resource."]
        #[doc = "Operation to delete a Lab Plan resource. Deleting a lab plan does not delete labs associated with a lab plan, nor does it delete shared images added to a gallery via the lab plan permission container."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_plan_name`: The name of the lab plan that uniquely identifies it within containing resource group. Used in resource URIs and in UI."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_plan_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_plan_name: lab_plan_name.into(),
            }
        }
        #[doc = "Save an image from a lab VM to the attached shared image gallery."]
        #[doc = "Saves an image from a lab VM to the attached shared image gallery."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_plan_name`: The name of the lab plan that uniquely identifies it within containing resource group. Used in resource URIs and in UI."]
        #[doc = "* `body`: The request body."]
        pub fn save_image(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_plan_name: impl Into<String>,
            body: impl Into<models::SaveImageBody>,
        ) -> save_image::Builder {
            save_image::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_plan_name: lab_plan_name.into(),
                body: body.into(),
            }
        }
    }
    pub mod list_by_subscription {
        use super::models;
        type Response = models::PagedLabPlans;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "The filter to apply to the operation."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.LabServices/labPlans",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
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
                                let rsp_value: models::PagedLabPlans = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::PagedLabPlans;
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labPlans",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PagedLabPlans = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::LabPlan;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_plan_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labPlans/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_plan_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::LabPlan = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::LabPlan),
            Created201(models::LabPlan),
            Accepted202(models::LabPlan),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_plan_name: String,
            pub(crate) body: models::LabPlan,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labPlans/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_plan_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::LabPlan = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::LabPlan = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::LabPlan = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::LabPlan),
            Accepted202(models::LabPlan),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_plan_name: String,
            pub(crate) body: models::LabPlanUpdate,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labPlans/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_plan_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::LabPlan = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::LabPlan = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_plan_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labPlans/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_plan_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::Accepted => Ok(Response::Accepted202),
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
    pub mod save_image {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_plan_name: String,
            pub(crate) body: models::SaveImageBody,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labPlans/{}/saveImage",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_plan_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::Accepted => Ok(Response::Accepted202),
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
        #[doc = "Get all operations"]
        #[doc = "Returns a list of all operations."]
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
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url =
                            azure_core::Url::parse(&format!("{}/providers/Microsoft.LabServices/operations", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
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
                };
                azure_core::Pageable::new(make_request)
            }
        }
    }
}
pub mod labs {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get all labs for a subscription."]
        #[doc = "Returns a list of all labs for a subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        pub fn list_by_subscription(&self, subscription_id: impl Into<String>) -> list_by_subscription::Builder {
            list_by_subscription::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                filter: None,
            }
        }
        #[doc = "Get all labs for a subscription and resource group."]
        #[doc = "Returns a list of all labs in a resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
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
        #[doc = "Get a lab resource."]
        #[doc = "Returns the properties of a lab resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
            }
        }
        #[doc = "Create or update a lab resource."]
        #[doc = "Operation to create or update a lab resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        #[doc = "* `body`: The request body."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
            body: impl Into<models::Lab>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
                body: body.into(),
            }
        }
        #[doc = "Update a lab resource."]
        #[doc = "Operation to update a lab resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        #[doc = "* `body`: The request body."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
            body: impl Into<models::LabUpdate>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
                body: body.into(),
            }
        }
        #[doc = "Deletes a lab resource."]
        #[doc = "Operation to delete a lab resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
            }
        }
        #[doc = "Publish or re-publish a lab."]
        #[doc = "Publish or re-publish a lab. This will create or update all lab resources, such as virtual machines."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        pub fn publish(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
        ) -> publish::Builder {
            publish::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
            }
        }
        #[doc = "Manually sync the lab group."]
        #[doc = "Action used to manually kick off an AAD group sync job."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        pub fn sync_group(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
        ) -> sync_group::Builder {
            sync_group::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
            }
        }
    }
    pub mod list_by_subscription {
        use super::models;
        type Response = models::PagedLabs;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "The filter to apply to the operation."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.LabServices/labs",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
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
                                let rsp_value: models::PagedLabs = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::PagedLabs;
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PagedLabs = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Lab;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Lab = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::Lab),
            Created201(models::Lab),
            Accepted202(models::Lab),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
            pub(crate) body: models::Lab,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Lab = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Lab = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Lab = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Lab),
            Accepted202(models::Lab),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
            pub(crate) body: models::LabUpdate,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Lab = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Lab = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::Accepted => Ok(Response::Accepted202),
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
    pub mod publish {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}/publish",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::Accepted => Ok(Response::Accepted202),
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
    pub mod sync_group {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}/syncGroup",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::Accepted => Ok(Response::Accepted202),
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
pub mod operation_results {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get an azure operation result."]
        #[doc = "Returns an azure operation result."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `operation_result_id`: The operation result ID / name."]
        pub fn get(&self, subscription_id: impl Into<String>, operation_result_id: impl Into<String>) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                operation_result_id: operation_result_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::OperationResult),
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) operation_result_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.LabServices/operationResults/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.operation_result_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::OperationResult = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
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
pub mod schedules {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get all schedules for a lab."]
        #[doc = "Returns a list of all schedules for a lab."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        pub fn list_by_lab(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
        ) -> list_by_lab::Builder {
            list_by_lab::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
                filter: None,
            }
        }
        #[doc = "Get a lab Schedule."]
        #[doc = "Returns the properties of a lab Schedule."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        #[doc = "* `schedule_name`: The name of the schedule that uniquely identifies it within containing lab. Used in resource URIs."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
            schedule_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
                schedule_name: schedule_name.into(),
            }
        }
        #[doc = "Create or update a lab schedule."]
        #[doc = "Operation to create or update a lab schedule."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        #[doc = "* `schedule_name`: The name of the schedule that uniquely identifies it within containing lab. Used in resource URIs."]
        #[doc = "* `body`: The request body."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
            schedule_name: impl Into<String>,
            body: impl Into<models::Schedule>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
                schedule_name: schedule_name.into(),
                body: body.into(),
            }
        }
        #[doc = "Update a lab schedule."]
        #[doc = "Operation to update a lab schedule."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        #[doc = "* `schedule_name`: The name of the schedule that uniquely identifies it within containing lab. Used in resource URIs."]
        #[doc = "* `body`: The request body."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
            schedule_name: impl Into<String>,
            body: impl Into<models::ScheduleUpdate>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
                schedule_name: schedule_name.into(),
                body: body.into(),
            }
        }
        #[doc = "Deletes a schedule resource."]
        #[doc = "Operation to delete a schedule resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        #[doc = "* `schedule_name`: The name of the schedule that uniquely identifies it within containing lab. Used in resource URIs."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
            schedule_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
                schedule_name: schedule_name.into(),
            }
        }
    }
    pub mod list_by_lab {
        use super::models;
        type Response = models::PagedSchedules;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "The filter to apply to the operation."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}/schedules",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
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
                                let rsp_value: models::PagedSchedules = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Schedule;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
            pub(crate) schedule_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}/schedules/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_name,
                            &this.schedule_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Schedule = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::Schedule),
            Created201(models::Schedule),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
            pub(crate) schedule_name: String,
            pub(crate) body: models::Schedule,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}/schedules/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_name,
                            &this.schedule_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Schedule = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Schedule = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Schedule;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
            pub(crate) schedule_name: String,
            pub(crate) body: models::ScheduleUpdate,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}/schedules/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_name,
                            &this.schedule_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Schedule = serde_json::from_slice(&rsp_body)?;
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
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
            pub(crate) schedule_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}/schedules/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_name,
                            &this.schedule_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::Accepted => Ok(Response::Accepted202),
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
pub mod users {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get all users for a lab."]
        #[doc = "Returns a list of all users for a lab."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        pub fn list_by_lab(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
        ) -> list_by_lab::Builder {
            list_by_lab::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
                filter: None,
            }
        }
        #[doc = "Get a lab user."]
        #[doc = "Returns the properties of a lab user."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        #[doc = "* `user_name`: The name of the user that uniquely identifies it within containing lab. Used in resource URIs."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
            user_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
                user_name: user_name.into(),
            }
        }
        #[doc = "Create or update a lab user."]
        #[doc = "Operation to create or update a lab user."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        #[doc = "* `user_name`: The name of the user that uniquely identifies it within containing lab. Used in resource URIs."]
        #[doc = "* `body`: The request body."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
            user_name: impl Into<String>,
            body: impl Into<models::User>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
                user_name: user_name.into(),
                body: body.into(),
            }
        }
        #[doc = "Update a lab user."]
        #[doc = "Operation to update a lab user."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        #[doc = "* `user_name`: The name of the user that uniquely identifies it within containing lab. Used in resource URIs."]
        #[doc = "* `body`: The request body."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
            user_name: impl Into<String>,
            body: impl Into<models::UserUpdate>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
                user_name: user_name.into(),
                body: body.into(),
            }
        }
        #[doc = "Deletes a user resource."]
        #[doc = "Operation to delete a user resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        #[doc = "* `user_name`: The name of the user that uniquely identifies it within containing lab. Used in resource URIs."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
            user_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
                user_name: user_name.into(),
            }
        }
        #[doc = "Invite a user to a lab."]
        #[doc = "Operation to invite a user to a lab."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        #[doc = "* `user_name`: The name of the user that uniquely identifies it within containing lab. Used in resource URIs."]
        #[doc = "* `body`: The request body."]
        pub fn invite(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
            user_name: impl Into<String>,
            body: impl Into<models::InviteBody>,
        ) -> invite::Builder {
            invite::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
                user_name: user_name.into(),
                body: body.into(),
            }
        }
    }
    pub mod list_by_lab {
        use super::models;
        type Response = models::PagedUsers;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "The filter to apply to the operation."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}/users",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
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
                                let rsp_value: models::PagedUsers = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::User;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
            pub(crate) user_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}/users/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_name,
                            &this.user_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::User = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::User),
            Created201(models::User),
            Accepted202(models::User),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
            pub(crate) user_name: String,
            pub(crate) body: models::User,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}/users/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_name,
                            &this.user_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::User = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::User = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::User = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::User),
            Accepted202(models::User),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
            pub(crate) user_name: String,
            pub(crate) body: models::UserUpdate,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}/users/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_name,
                            &this.user_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::User = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Accepted => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::User = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Accepted202(rsp_value))
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
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
            pub(crate) user_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}/users/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_name,
                            &this.user_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::Accepted => Ok(Response::Accepted202),
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
    pub mod invite {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
            pub(crate) user_name: String,
            pub(crate) body: models::InviteBody,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}/users/{}/invite",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_name,
                            &this.user_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::Accepted => Ok(Response::Accepted202),
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
pub mod virtual_machines {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get all virtual machines for a lab."]
        #[doc = "Returns a list of all virtual machines for a lab."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        pub fn list_by_lab(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
        ) -> list_by_lab::Builder {
            list_by_lab::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
                filter: None,
            }
        }
        #[doc = "Get a lab virtual machine."]
        #[doc = "Returns the properties for a lab virtual machine."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        #[doc = "* `virtual_machine_name`: The ID of the virtual machine that uniquely identifies it within the containing lab. Used in resource URIs."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
            virtual_machine_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
                virtual_machine_name: virtual_machine_name.into(),
            }
        }
        #[doc = "Start a lab virtual machine."]
        #[doc = "Action to start a lab virtual machine."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        #[doc = "* `virtual_machine_name`: The ID of the virtual machine that uniquely identifies it within the containing lab. Used in resource URIs."]
        pub fn start(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
            virtual_machine_name: impl Into<String>,
        ) -> start::Builder {
            start::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
                virtual_machine_name: virtual_machine_name.into(),
            }
        }
        #[doc = "Stop a lab virtual machine."]
        #[doc = "Action to stop a lab virtual machine."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        #[doc = "* `virtual_machine_name`: The ID of the virtual machine that uniquely identifies it within the containing lab. Used in resource URIs."]
        pub fn stop(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
            virtual_machine_name: impl Into<String>,
        ) -> stop::Builder {
            stop::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
                virtual_machine_name: virtual_machine_name.into(),
            }
        }
        #[doc = "Re-image a lab virtual machine."]
        #[doc = "Re-image a lab virtual machine. The virtual machine will be deleted and recreated using the latest published snapshot of the reference environment of the lab."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        #[doc = "* `virtual_machine_name`: The ID of the virtual machine that uniquely identifies it within the containing lab. Used in resource URIs."]
        pub fn reimage(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
            virtual_machine_name: impl Into<String>,
        ) -> reimage::Builder {
            reimage::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
                virtual_machine_name: virtual_machine_name.into(),
            }
        }
        #[doc = "Redeploy a lab virtual machine to a different compute node. For troubleshooting connectivity."]
        #[doc = "Action to redeploy a lab virtual machine to a different compute node. For troubleshooting connectivity."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        #[doc = "* `virtual_machine_name`: The ID of the virtual machine that uniquely identifies it within the containing lab. Used in resource URIs."]
        pub fn redeploy(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
            virtual_machine_name: impl Into<String>,
        ) -> redeploy::Builder {
            redeploy::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
                virtual_machine_name: virtual_machine_name.into(),
            }
        }
        #[doc = "Reset a lab virtual machine password."]
        #[doc = "Resets a lab virtual machine password."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `resource_group_name`: The name of the resource group. The name is case insensitive."]
        #[doc = "* `lab_name`: The name of the lab that uniquely identifies it within containing lab account. Used in resource URIs."]
        #[doc = "* `virtual_machine_name`: The ID of the virtual machine that uniquely identifies it within the containing lab. Used in resource URIs."]
        #[doc = "* `body`: The request body."]
        pub fn reset_password(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            lab_name: impl Into<String>,
            virtual_machine_name: impl Into<String>,
            body: impl Into<models::ResetPasswordBody>,
        ) -> reset_password::Builder {
            reset_password::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                lab_name: lab_name.into(),
                virtual_machine_name: virtual_machine_name.into(),
                body: body.into(),
            }
        }
    }
    pub mod list_by_lab {
        use super::models;
        type Response = models::PagedVirtualMachines;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "The filter to apply to the operation."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}/virtualMachines",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
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
                                let rsp_value: models::PagedVirtualMachines = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::VirtualMachine;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
            pub(crate) virtual_machine_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}/virtualMachines/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_name,
                            &this.virtual_machine_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::VirtualMachine = serde_json::from_slice(&rsp_body)?;
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
    pub mod start {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
            pub(crate) virtual_machine_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}/virtualMachines/{}/start",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_name,
                            &this.virtual_machine_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::Accepted => Ok(Response::Accepted202),
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
    pub mod stop {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
            pub(crate) virtual_machine_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}/virtualMachines/{}/stop",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_name,
                            &this.virtual_machine_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::Accepted => Ok(Response::Accepted202),
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
    pub mod reimage {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
            pub(crate) virtual_machine_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}/virtualMachines/{}/reimage",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_name,
                            &this.virtual_machine_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::Accepted => Ok(Response::Accepted202),
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
    pub mod redeploy {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
            pub(crate) virtual_machine_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}/virtualMachines/{}/redeploy",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.lab_name,
                            &this.virtual_machine_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::Accepted => Ok(Response::Accepted202),
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
    pub mod reset_password {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) lab_name: String,
            pub(crate) virtual_machine_name: String,
            pub(crate) body: models::ResetPasswordBody,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.LabServices/labs/{}/virtualMachines/{}/resetPassword" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . lab_name , & this . virtual_machine_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => Ok(Response::Ok200),
                            azure_core::StatusCode::Accepted => Ok(Response::Accepted202),
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
pub mod usages {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the list of usages."]
        #[doc = "Returns list of usage per SKU family for the specified subscription in the specified region."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        #[doc = "* `location`: The location name."]
        pub fn list_by_location(&self, subscription_id: impl Into<String>, location: impl Into<String>) -> list_by_location::Builder {
            list_by_location::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                location: location.into(),
                filter: None,
            }
        }
    }
    pub mod list_by_location {
        use super::models;
        type Response = models::ListUsagesResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) location: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "The filter to apply to the operation."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.LabServices/locations/{}/usages",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.location
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
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
                                let rsp_value: models::ListUsagesResult = serde_json::from_slice(&rsp_body)?;
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
        #[doc = "Gets all the Azure Lab Services resource SKUs."]
        #[doc = "Returns a list of all the Azure Lab Services resource SKUs."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The ID of the target subscription."]
        pub fn list(&self, subscription_id: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                filter: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::PagedLabServicesSkus;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "The filter to apply to the operation."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.LabServices/skus",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-11-15-preview");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
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
                                let rsp_value: models::PagedLabServicesSkus = serde_json::from_slice(&rsp_body)?;
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
