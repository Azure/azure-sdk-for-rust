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
    pub fn agent_pools_client(&self) -> agent_pools::Client {
        agent_pools::Client(self.clone())
    }
    pub fn connected_registries_client(&self) -> connected_registries::Client {
        connected_registries::Client(self.clone())
    }
    pub fn export_pipelines_client(&self) -> export_pipelines::Client {
        export_pipelines::Client(self.clone())
    }
    pub fn import_pipelines_client(&self) -> import_pipelines::Client {
        import_pipelines::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn pipeline_runs_client(&self) -> pipeline_runs::Client {
        pipeline_runs::Client(self.clone())
    }
    pub fn private_endpoint_connections_client(&self) -> private_endpoint_connections::Client {
        private_endpoint_connections::Client(self.clone())
    }
    pub fn registries_client(&self) -> registries::Client {
        registries::Client(self.clone())
    }
    pub fn replications_client(&self) -> replications::Client {
        replications::Client(self.clone())
    }
    pub fn runs_client(&self) -> runs::Client {
        runs::Client(self.clone())
    }
    pub fn scope_maps_client(&self) -> scope_maps::Client {
        scope_maps::Client(self.clone())
    }
    pub fn task_runs_client(&self) -> task_runs::Client {
        task_runs::Client(self.clone())
    }
    pub fn tasks_client(&self) -> tasks::Client {
        tasks::Client(self.clone())
    }
    pub fn tokens_client(&self) -> tokens::Client {
        tokens::Client(self.clone())
    }
    pub fn webhooks_client(&self) -> webhooks::Client {
        webhooks::Client(self.clone())
    }
}
pub mod connected_registries {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the properties of the connected registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `connected_registry_name`: The name of the connected registry."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            connected_registry_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                connected_registry_name: connected_registry_name.into(),
            }
        }
        #[doc = "Creates a connected registry for a container registry with the specified parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `connected_registry_name`: The name of the connected registry."]
        #[doc = "* `connected_registry_create_parameters`: The parameters for creating a connectedRegistry."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            connected_registry_name: impl Into<String>,
            connected_registry_create_parameters: impl Into<models::ConnectedRegistry>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                connected_registry_name: connected_registry_name.into(),
                connected_registry_create_parameters: connected_registry_create_parameters.into(),
            }
        }
        #[doc = "Updates a connected registry with the specified parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `connected_registry_name`: The name of the connected registry."]
        #[doc = "* `connected_registry_update_parameters`: The parameters for updating a connectedRegistry."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            connected_registry_name: impl Into<String>,
            connected_registry_update_parameters: impl Into<models::ConnectedRegistryUpdateParameters>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                connected_registry_name: connected_registry_name.into(),
                connected_registry_update_parameters: connected_registry_update_parameters.into(),
            }
        }
        #[doc = "Deletes a connected registry from a container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `connected_registry_name`: The name of the connected registry."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            connected_registry_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                connected_registry_name: connected_registry_name.into(),
            }
        }
        #[doc = "Lists all connected registries for the specified container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                filter: None,
            }
        }
        #[doc = "Deactivates the connected registry instance."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `connected_registry_name`: The name of the connected registry."]
        pub fn deactivate(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            connected_registry_name: impl Into<String>,
        ) -> deactivate::Builder {
            deactivate::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                connected_registry_name: connected_registry_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::ConnectedRegistry;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) connected_registry_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/connectedRegistries/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . registry_name , & this . connected_registry_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ConnectedRegistry = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::ConnectedRegistry),
            Created201(models::ConnectedRegistry),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) connected_registry_name: String,
            pub(crate) connected_registry_create_parameters: models::ConnectedRegistry,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/connectedRegistries/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . registry_name , & this . connected_registry_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.connected_registry_create_parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ConnectedRegistry = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ConnectedRegistry = serde_json::from_slice(&rsp_body)?;
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ConnectedRegistry),
            Created201(models::ConnectedRegistry),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) connected_registry_name: String,
            pub(crate) connected_registry_update_parameters: models::ConnectedRegistryUpdateParameters,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/connectedRegistries/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . registry_name , & this . connected_registry_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Patch);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.connected_registry_update_parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ConnectedRegistry = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ConnectedRegistry = serde_json::from_slice(&rsp_body)?;
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
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) connected_registry_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/connectedRegistries/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . registry_name , & this . connected_registry_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
    pub mod list {
        use super::models;
        type Response = models::ConnectedRegistryListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) filter: Option<String>,
        }
        impl Builder {
            #[doc = "An OData filter expression that describes a subset of connectedRegistries to return. The parameters that can be filtered are parent.id (the resource id of the connectedRegistry parent), mode, and connectionState. The supported operator is eq."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/connectedRegistries",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
                                let rsp_value: models::ConnectedRegistryListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod deactivate {
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
            pub(crate) registry_name: String,
            pub(crate) connected_registry_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/connectedRegistries/{}/deactivate" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . registry_name , & this . connected_registry_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
pub mod export_pipelines {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the properties of the export pipeline."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `export_pipeline_name`: The name of the export pipeline."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            export_pipeline_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                export_pipeline_name: export_pipeline_name.into(),
            }
        }
        #[doc = "Creates an export pipeline for a container registry with the specified parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `export_pipeline_name`: The name of the export pipeline."]
        #[doc = "* `export_pipeline_create_parameters`: The parameters for creating an export pipeline."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            export_pipeline_name: impl Into<String>,
            export_pipeline_create_parameters: impl Into<models::ExportPipeline>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                export_pipeline_name: export_pipeline_name.into(),
                export_pipeline_create_parameters: export_pipeline_create_parameters.into(),
            }
        }
        #[doc = "Deletes an export pipeline from a container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `export_pipeline_name`: The name of the export pipeline."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            export_pipeline_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                export_pipeline_name: export_pipeline_name.into(),
            }
        }
        #[doc = "Lists all export pipelines for the specified container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::ExportPipeline;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) export_pipeline_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/exportPipelines/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.export_pipeline_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ExportPipeline = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::ExportPipeline),
            Created201(models::ExportPipeline),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) export_pipeline_name: String,
            pub(crate) export_pipeline_create_parameters: models::ExportPipeline,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/exportPipelines/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.export_pipeline_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.export_pipeline_create_parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ExportPipeline = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ExportPipeline = serde_json::from_slice(&rsp_body)?;
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
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) export_pipeline_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/exportPipelines/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.export_pipeline_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
    pub mod list {
        use super::models;
        type Response = models::ExportPipelineListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/exportPipelines",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ExportPipelineListResult = serde_json::from_slice(&rsp_body)?;
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
pub mod registries {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Copies an image to this container registry from the specified container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `parameters`: The parameters specifying the image to copy and the source container registry."]
        pub fn import_image(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            parameters: impl Into<models::ImportImageParameters>,
        ) -> import_image::Builder {
            import_image::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Checks whether the container registry name is available for use. The name must contain only alphanumeric characters, be globally unique, and between 5 and 50 characters in length."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `registry_name_check_request`: The object containing information for the availability request."]
        pub fn check_name_availability(
            &self,
            subscription_id: impl Into<String>,
            registry_name_check_request: impl Into<models::RegistryNameCheckRequest>,
        ) -> check_name_availability::Builder {
            check_name_availability::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                registry_name_check_request: registry_name_check_request.into(),
            }
        }
        #[doc = "Gets the properties of the specified container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
            }
        }
        #[doc = "Creates a container registry with the specified parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `registry`: The parameters for creating a container registry."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            registry: impl Into<models::Registry>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                registry: registry.into(),
            }
        }
        #[doc = "Updates a container registry with the specified parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `registry_update_parameters`: The parameters for updating a container registry."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            registry_update_parameters: impl Into<models::RegistryUpdateParameters>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                registry_update_parameters: registry_update_parameters.into(),
            }
        }
        #[doc = "Deletes a container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
            }
        }
        #[doc = "Lists all the container registries under the specified resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
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
        #[doc = "Lists all the container registries under the specified subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        pub fn list(&self, subscription_id: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Lists the login credentials for the specified container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        pub fn list_credentials(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
        ) -> list_credentials::Builder {
            list_credentials::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
            }
        }
        #[doc = "Regenerates one of the login credentials for the specified container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `regenerate_credential_parameters`: Specifies name of the password which should be regenerated -- password or password2."]
        pub fn regenerate_credential(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            regenerate_credential_parameters: impl Into<models::RegenerateCredentialParameters>,
        ) -> regenerate_credential::Builder {
            regenerate_credential::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                regenerate_credential_parameters: regenerate_credential_parameters.into(),
            }
        }
        #[doc = "Gets the quota usages for the specified container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        pub fn list_usages(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
        ) -> list_usages::Builder {
            list_usages::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
            }
        }
        #[doc = "Lists the private link resources for a container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        pub fn list_private_link_resources(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
        ) -> list_private_link_resources::Builder {
            list_private_link_resources::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
            }
        }
        #[doc = "Generate keys for a token of a specified container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `generate_credentials_parameters`: The parameters for generating credentials."]
        pub fn generate_credentials(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            generate_credentials_parameters: impl Into<models::GenerateCredentialsParameters>,
        ) -> generate_credentials::Builder {
            generate_credentials::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                generate_credentials_parameters: generate_credentials_parameters.into(),
            }
        }
        #[doc = "Schedules a new run based on the request parameters and add it to the run queue."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `run_request`: The parameters of a run that needs to scheduled."]
        pub fn schedule_run(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            run_request: impl Into<models::RunRequest>,
        ) -> schedule_run::Builder {
            schedule_run::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                run_request: run_request.into(),
            }
        }
        #[doc = "Get the upload location for the user to be able to upload the source."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        pub fn get_build_source_upload_url(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
        ) -> get_build_source_upload_url::Builder {
            get_build_source_upload_url::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
            }
        }
    }
    pub mod import_image {
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
            pub(crate) registry_name: String,
            pub(crate) parameters: models::ImportImageParameters,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/importImage",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
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
    pub mod check_name_availability {
        use super::models;
        type Response = models::RegistryNameStatus;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) registry_name_check_request: models::RegistryNameCheckRequest,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.ContainerRegistry/checkNameAvailability",
                            this.client.endpoint(),
                            &this.subscription_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.registry_name_check_request)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RegistryNameStatus = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Registry;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Registry = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::Registry),
            Created201(models::Registry),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) registry: models::Registry,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.registry)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Registry = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Registry = serde_json::from_slice(&rsp_body)?;
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Registry),
            Created201(models::Registry),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) registry_update_parameters: models::RegistryUpdateParameters,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.registry_update_parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Registry = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Registry = serde_json::from_slice(&rsp_body)?;
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
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
    pub mod list_by_resource_group {
        use super::models;
        type Response = models::RegistryListResult;
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RegistryListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::RegistryListResult;
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
                            "{}/subscriptions/{}/providers/Microsoft.ContainerRegistry/registries",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RegistryListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_credentials {
        use super::models;
        type Response = models::RegistryListCredentialsResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/listCredentials",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RegistryListCredentialsResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod regenerate_credential {
        use super::models;
        type Response = models::RegistryListCredentialsResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) regenerate_credential_parameters: models::RegenerateCredentialParameters,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/regenerateCredential" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . registry_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.regenerate_credential_parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RegistryListCredentialsResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_usages {
        use super::models;
        type Response = models::RegistryUsageListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/listUsages",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RegistryUsageListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_private_link_resources {
        use super::models;
        type Response = models::PrivateLinkResourceListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/privateLinkResources" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . registry_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PrivateLinkResourceListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod generate_credentials {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::GenerateCredentialsResult),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) generate_credentials_parameters: models::GenerateCredentialsParameters,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/generateCredentials",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.generate_credentials_parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::GenerateCredentialsResult = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
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
    pub mod schedule_run {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Run),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) run_request: models::RunRequest,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/scheduleRun",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.run_request)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Run = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
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
    pub mod get_build_source_upload_url {
        use super::models;
        type Response = models::SourceUploadDefinition;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/listBuildSourceUploadUrl" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . registry_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SourceUploadDefinition = serde_json::from_slice(&rsp_body)?;
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
pub mod import_pipelines {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the properties of the import pipeline."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `import_pipeline_name`: The name of the import pipeline."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            import_pipeline_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                import_pipeline_name: import_pipeline_name.into(),
            }
        }
        #[doc = "Creates an import pipeline for a container registry with the specified parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `import_pipeline_name`: The name of the import pipeline."]
        #[doc = "* `import_pipeline_create_parameters`: The parameters for creating an import pipeline."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            import_pipeline_name: impl Into<String>,
            import_pipeline_create_parameters: impl Into<models::ImportPipeline>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                import_pipeline_name: import_pipeline_name.into(),
                import_pipeline_create_parameters: import_pipeline_create_parameters.into(),
            }
        }
        #[doc = "Deletes an import pipeline from a container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `import_pipeline_name`: The name of the import pipeline."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            import_pipeline_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                import_pipeline_name: import_pipeline_name.into(),
            }
        }
        #[doc = "Lists all import pipelines for the specified container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::ImportPipeline;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) import_pipeline_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/importPipelines/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.import_pipeline_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ImportPipeline = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::ImportPipeline),
            Created201(models::ImportPipeline),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) import_pipeline_name: String,
            pub(crate) import_pipeline_create_parameters: models::ImportPipeline,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/importPipelines/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.import_pipeline_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.import_pipeline_create_parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ImportPipeline = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ImportPipeline = serde_json::from_slice(&rsp_body)?;
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
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) import_pipeline_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/importPipelines/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.import_pipeline_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
    pub mod list {
        use super::models;
        type Response = models::ImportPipelineListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/importPipelines",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ImportPipelineListResult = serde_json::from_slice(&rsp_body)?;
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
        #[doc = "Lists all of the available Azure Container Registry REST API operations."]
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
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/providers/Microsoft.ContainerRegistry/operations",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
pub mod pipeline_runs {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the detailed information for a given pipeline run."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `pipeline_run_name`: The name of the pipeline run."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            pipeline_run_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                pipeline_run_name: pipeline_run_name.into(),
            }
        }
        #[doc = "Creates a pipeline run for a container registry with the specified parameters"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `pipeline_run_name`: The name of the pipeline run."]
        #[doc = "* `pipeline_run_create_parameters`: The parameters for creating a pipeline run."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            pipeline_run_name: impl Into<String>,
            pipeline_run_create_parameters: impl Into<models::PipelineRun>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                pipeline_run_name: pipeline_run_name.into(),
                pipeline_run_create_parameters: pipeline_run_create_parameters.into(),
            }
        }
        #[doc = "Deletes a pipeline run from a container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `pipeline_run_name`: The name of the pipeline run."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            pipeline_run_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                pipeline_run_name: pipeline_run_name.into(),
            }
        }
        #[doc = "Lists all the pipeline runs for the specified container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::PipelineRun;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) pipeline_run_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/pipelineRuns/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.pipeline_run_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PipelineRun = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::PipelineRun),
            Created201(models::PipelineRun),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) pipeline_run_name: String,
            pub(crate) pipeline_run_create_parameters: models::PipelineRun,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/pipelineRuns/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.pipeline_run_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.pipeline_run_create_parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PipelineRun = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PipelineRun = serde_json::from_slice(&rsp_body)?;
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
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) pipeline_run_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/pipelineRuns/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.pipeline_run_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
    pub mod list {
        use super::models;
        type Response = models::PipelineRunListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/pipelineRuns",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PipelineRunListResult = serde_json::from_slice(&rsp_body)?;
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
pub mod private_endpoint_connections {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get the specified private endpoint connection associated with the container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `private_endpoint_connection_name`: The name of the private endpoint connection."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            private_endpoint_connection_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                private_endpoint_connection_name: private_endpoint_connection_name.into(),
            }
        }
        #[doc = "Update the state of specified private endpoint connection associated with the container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `private_endpoint_connection_name`: The name of the private endpoint connection."]
        #[doc = "* `private_endpoint_connection`: The parameters for creating a private endpoint connection."]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            private_endpoint_connection_name: impl Into<String>,
            private_endpoint_connection: impl Into<models::PrivateEndpointConnection>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                private_endpoint_connection_name: private_endpoint_connection_name.into(),
                private_endpoint_connection: private_endpoint_connection.into(),
            }
        }
        #[doc = "Deletes the specified private endpoint connection associated with the container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `private_endpoint_connection_name`: The name of the private endpoint connection."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            private_endpoint_connection_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                private_endpoint_connection_name: private_endpoint_connection_name.into(),
            }
        }
        #[doc = "List all private endpoint connections in a container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::PrivateEndpointConnection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) private_endpoint_connection_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/privateEndpointConnections/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . registry_name , & this . private_endpoint_connection_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PrivateEndpointConnection = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::PrivateEndpointConnection),
            Created201(models::PrivateEndpointConnection),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) private_endpoint_connection_name: String,
            pub(crate) private_endpoint_connection: models::PrivateEndpointConnection,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/privateEndpointConnections/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . registry_name , & this . private_endpoint_connection_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.private_endpoint_connection)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PrivateEndpointConnection = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PrivateEndpointConnection = serde_json::from_slice(&rsp_body)?;
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
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) private_endpoint_connection_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/privateEndpointConnections/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . registry_name , & this . private_endpoint_connection_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Delete);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
    pub mod list {
        use super::models;
        type Response = models::PrivateEndpointConnectionListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/privateEndpointConnections" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . registry_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::PrivateEndpointConnectionListResult = serde_json::from_slice(&rsp_body)?;
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
pub mod replications {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the properties of the specified replication."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `replication_name`: The name of the replication."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            replication_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                replication_name: replication_name.into(),
            }
        }
        #[doc = "Creates a replication for a container registry with the specified parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `replication_name`: The name of the replication."]
        #[doc = "* `replication`: The parameters for creating a replication."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            replication_name: impl Into<String>,
            replication: impl Into<models::Replication>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                replication_name: replication_name.into(),
                replication: replication.into(),
            }
        }
        #[doc = "Updates a replication for a container registry with the specified parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `replication_name`: The name of the replication."]
        #[doc = "* `replication_update_parameters`: The parameters for updating a replication."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            replication_name: impl Into<String>,
            replication_update_parameters: impl Into<models::ReplicationUpdateParameters>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                replication_name: replication_name.into(),
                replication_update_parameters: replication_update_parameters.into(),
            }
        }
        #[doc = "Deletes a replication from a container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `replication_name`: The name of the replication."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            replication_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                replication_name: replication_name.into(),
            }
        }
        #[doc = "Lists all the replications for the specified container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::Replication;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) replication_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/replications/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.replication_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Replication = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::Replication),
            Created201(models::Replication),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) replication_name: String,
            pub(crate) replication: models::Replication,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/replications/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.replication_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.replication)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Replication = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Replication = serde_json::from_slice(&rsp_body)?;
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Replication),
            Created201(models::Replication),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) replication_name: String,
            pub(crate) replication_update_parameters: models::ReplicationUpdateParameters,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/replications/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.replication_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.replication_update_parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Replication = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Replication = serde_json::from_slice(&rsp_body)?;
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
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) replication_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/replications/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.replication_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
    pub mod list {
        use super::models;
        type Response = models::ReplicationListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/replications",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ReplicationListResult = serde_json::from_slice(&rsp_body)?;
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
pub mod scope_maps {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the properties of the specified scope map."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `scope_map_name`: The name of the scope map."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            scope_map_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                scope_map_name: scope_map_name.into(),
            }
        }
        #[doc = "Creates a scope map for a container registry with the specified parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `scope_map_name`: The name of the scope map."]
        #[doc = "* `scope_map_create_parameters`: The parameters for creating a scope map."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            scope_map_name: impl Into<String>,
            scope_map_create_parameters: impl Into<models::ScopeMap>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                scope_map_name: scope_map_name.into(),
                scope_map_create_parameters: scope_map_create_parameters.into(),
            }
        }
        #[doc = "Updates a scope map with the specified parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `scope_map_name`: The name of the scope map."]
        #[doc = "* `scope_map_update_parameters`: The parameters for updating a scope map."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            scope_map_name: impl Into<String>,
            scope_map_update_parameters: impl Into<models::ScopeMapUpdateParameters>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                scope_map_name: scope_map_name.into(),
                scope_map_update_parameters: scope_map_update_parameters.into(),
            }
        }
        #[doc = "Deletes a scope map from a container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `scope_map_name`: The name of the scope map."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            scope_map_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                scope_map_name: scope_map_name.into(),
            }
        }
        #[doc = "Lists all the scope maps for the specified container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::ScopeMap;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) scope_map_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/scopeMaps/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.scope_map_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ScopeMap = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::ScopeMap),
            Created201(models::ScopeMap),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) scope_map_name: String,
            pub(crate) scope_map_create_parameters: models::ScopeMap,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/scopeMaps/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.scope_map_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.scope_map_create_parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ScopeMap = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ScopeMap = serde_json::from_slice(&rsp_body)?;
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::ScopeMap),
            Created201(models::ScopeMap),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) scope_map_name: String,
            pub(crate) scope_map_update_parameters: models::ScopeMapUpdateParameters,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/scopeMaps/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.scope_map_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.scope_map_update_parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ScopeMap = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ScopeMap = serde_json::from_slice(&rsp_body)?;
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
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) scope_map_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/scopeMaps/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.scope_map_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
    pub mod list {
        use super::models;
        type Response = models::ScopeMapListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/scopeMaps",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ScopeMapListResult = serde_json::from_slice(&rsp_body)?;
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
pub mod tokens {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the properties of the specified token."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `token_name`: The name of the token."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            token_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                token_name: token_name.into(),
            }
        }
        #[doc = "Creates a token for a container registry with the specified parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `token_name`: The name of the token."]
        #[doc = "* `token_create_parameters`: The parameters for creating a token."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            token_name: impl Into<String>,
            token_create_parameters: impl Into<models::Token>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                token_name: token_name.into(),
                token_create_parameters: token_create_parameters.into(),
            }
        }
        #[doc = "Updates a token with the specified parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `token_name`: The name of the token."]
        #[doc = "* `token_update_parameters`: The parameters for updating a token."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            token_name: impl Into<String>,
            token_update_parameters: impl Into<models::TokenUpdateParameters>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                token_name: token_name.into(),
                token_update_parameters: token_update_parameters.into(),
            }
        }
        #[doc = "Deletes a token from a container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `token_name`: The name of the token."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            token_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                token_name: token_name.into(),
            }
        }
        #[doc = "Lists all the tokens for the specified container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::Token;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) token_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/tokens/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.token_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Token = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::Token),
            Created201(models::Token),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) token_name: String,
            pub(crate) token_create_parameters: models::Token,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/tokens/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.token_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.token_create_parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Token = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Token = serde_json::from_slice(&rsp_body)?;
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Token),
            Created201(models::Token),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) token_name: String,
            pub(crate) token_update_parameters: models::TokenUpdateParameters,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/tokens/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.token_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.token_update_parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Token = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Token = serde_json::from_slice(&rsp_body)?;
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
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) token_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/tokens/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.token_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
    pub mod list {
        use super::models;
        type Response = models::TokenListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/tokens",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TokenListResult = serde_json::from_slice(&rsp_body)?;
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
pub mod webhooks {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the properties of the specified webhook."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `webhook_name`: The name of the webhook."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            webhook_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                webhook_name: webhook_name.into(),
            }
        }
        #[doc = "Creates a webhook for a container registry with the specified parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `webhook_name`: The name of the webhook."]
        #[doc = "* `webhook_create_parameters`: The parameters for creating a webhook."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            webhook_name: impl Into<String>,
            webhook_create_parameters: impl Into<models::WebhookCreateParameters>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                webhook_name: webhook_name.into(),
                webhook_create_parameters: webhook_create_parameters.into(),
            }
        }
        #[doc = "Updates a webhook with the specified parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `webhook_name`: The name of the webhook."]
        #[doc = "* `webhook_update_parameters`: The parameters for updating a webhook."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            webhook_name: impl Into<String>,
            webhook_update_parameters: impl Into<models::WebhookUpdateParameters>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                webhook_name: webhook_name.into(),
                webhook_update_parameters: webhook_update_parameters.into(),
            }
        }
        #[doc = "Deletes a webhook from a container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `webhook_name`: The name of the webhook."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            webhook_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                webhook_name: webhook_name.into(),
            }
        }
        #[doc = "Lists all the webhooks for the specified container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
            }
        }
        #[doc = "Triggers a ping event to be sent to the webhook."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `webhook_name`: The name of the webhook."]
        pub fn ping(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            webhook_name: impl Into<String>,
        ) -> ping::Builder {
            ping::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                webhook_name: webhook_name.into(),
            }
        }
        #[doc = "Gets the configuration of service URI and custom headers for the webhook."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `webhook_name`: The name of the webhook."]
        pub fn get_callback_config(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            webhook_name: impl Into<String>,
        ) -> get_callback_config::Builder {
            get_callback_config::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                webhook_name: webhook_name.into(),
            }
        }
        #[doc = "Lists recent events for the specified webhook."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `webhook_name`: The name of the webhook."]
        pub fn list_events(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            webhook_name: impl Into<String>,
        ) -> list_events::Builder {
            list_events::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                webhook_name: webhook_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::Webhook;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) webhook_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/webhooks/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.webhook_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Webhook = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::Webhook),
            Created201(models::Webhook),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) webhook_name: String,
            pub(crate) webhook_create_parameters: models::WebhookCreateParameters,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/webhooks/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.webhook_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.webhook_create_parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Webhook = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Webhook = serde_json::from_slice(&rsp_body)?;
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Webhook),
            Created201(models::Webhook),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) webhook_name: String,
            pub(crate) webhook_update_parameters: models::WebhookUpdateParameters,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/webhooks/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.webhook_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.webhook_update_parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Webhook = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Webhook = serde_json::from_slice(&rsp_body)?;
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
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) webhook_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/webhooks/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.webhook_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
    pub mod list {
        use super::models;
        type Response = models::WebhookListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/webhooks",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::WebhookListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod ping {
        use super::models;
        type Response = models::EventInfo;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) webhook_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/webhooks/{}/ping",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.webhook_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EventInfo = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_callback_config {
        use super::models;
        type Response = models::CallbackConfig;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) webhook_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/webhooks/{}/getCallbackConfig" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . registry_name , & this . webhook_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CallbackConfig = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_events {
        use super::models;
        type Response = models::EventListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) webhook_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/webhooks/{}/listEvents" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . registry_name , & this . webhook_name)) ? ;
                        let rsp = match continuation {
                            Some(value) => {
                                url.set_path("");
                                url = url.join(&value)?;
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                                }
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                            None => {
                                let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                                let credential = this.client.token_credential();
                                let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                                req.insert_header(
                                    azure_core::headers::AUTHORIZATION,
                                    format!("Bearer {}", token_response.token.secret()),
                                );
                                req.url_mut()
                                    .query_pairs_mut()
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-06-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::EventListResult = serde_json::from_slice(&rsp_body)?;
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
pub mod agent_pools {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the detailed information for a given agent pool."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `agent_pool_name`: The name of the agent pool."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            agent_pool_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                agent_pool_name: agent_pool_name.into(),
            }
        }
        #[doc = "Creates an agent pool for a container registry with the specified parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `agent_pool_name`: The name of the agent pool."]
        #[doc = "* `agent_pool`: The parameters of an agent pool that needs to scheduled."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            agent_pool_name: impl Into<String>,
            agent_pool: impl Into<models::AgentPool>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                agent_pool_name: agent_pool_name.into(),
                agent_pool: agent_pool.into(),
            }
        }
        #[doc = "Updates an agent pool with the specified parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `agent_pool_name`: The name of the agent pool."]
        #[doc = "* `update_parameters`: The parameters for updating an agent pool."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            agent_pool_name: impl Into<String>,
            update_parameters: impl Into<models::AgentPoolUpdateParameters>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                agent_pool_name: agent_pool_name.into(),
                update_parameters: update_parameters.into(),
            }
        }
        #[doc = "Deletes a specified agent pool resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `agent_pool_name`: The name of the agent pool."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            agent_pool_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                agent_pool_name: agent_pool_name.into(),
            }
        }
        #[doc = "Lists all the agent pools for a specified container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
            }
        }
        #[doc = "Gets the count of queued runs for a given agent pool."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `agent_pool_name`: The name of the agent pool."]
        pub fn get_queue_status(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            agent_pool_name: impl Into<String>,
        ) -> get_queue_status::Builder {
            get_queue_status::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                agent_pool_name: agent_pool_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::AgentPool;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) agent_pool_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/agentPools/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.agent_pool_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AgentPool = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::AgentPool),
            Created201(models::AgentPool),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) agent_pool_name: String,
            pub(crate) agent_pool: models::AgentPool,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/agentPools/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.agent_pool_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.agent_pool)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AgentPool = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AgentPool = serde_json::from_slice(&rsp_body)?;
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::AgentPool),
            Created201(models::AgentPool),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) agent_pool_name: String,
            pub(crate) update_parameters: models::AgentPoolUpdateParameters,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/agentPools/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.agent_pool_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.update_parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AgentPool = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AgentPool = serde_json::from_slice(&rsp_body)?;
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
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) agent_pool_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/agentPools/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.agent_pool_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
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
    pub mod list {
        use super::models;
        type Response = models::AgentPoolListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/agentPools",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AgentPoolListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_queue_status {
        use super::models;
        type Response = models::AgentPoolQueueStatus;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) agent_pool_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/agentPools/{}/listQueueStatus" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . registry_name , & this . agent_pool_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AgentPoolQueueStatus = serde_json::from_slice(&rsp_body)?;
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
pub mod runs {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets all the runs for a registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                filter: None,
                top: None,
            }
        }
        #[doc = "Gets the detailed information for a given run."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `run_id`: The run ID."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            run_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                run_id: run_id.into(),
            }
        }
        #[doc = "Patch the run properties."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `run_id`: The run ID."]
        #[doc = "* `run_update_parameters`: The run update properties."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            run_id: impl Into<String>,
            run_update_parameters: impl Into<models::RunUpdateParameters>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                run_id: run_id.into(),
                run_update_parameters: run_update_parameters.into(),
            }
        }
        #[doc = "Gets a link to download the run logs."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `run_id`: The run ID."]
        pub fn get_log_sas_url(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            run_id: impl Into<String>,
        ) -> get_log_sas_url::Builder {
            get_log_sas_url::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                run_id: run_id.into(),
            }
        }
        #[doc = "Cancel an existing run."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `run_id`: The run ID."]
        pub fn cancel(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            run_id: impl Into<String>,
        ) -> cancel::Builder {
            cancel::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                run_id: run_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::RunListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) filter: Option<String>,
            pub(crate) top: Option<i32>,
        }
        impl Builder {
            #[doc = "The runs filter to apply on the operation. Arithmetic operators are not supported. The allowed string function is 'contains'. All logical operators except 'Not', 'Has', 'All' are allowed."]
            pub fn filter(mut self, filter: impl Into<String>) -> Self {
                self.filter = Some(filter.into());
                self
            }
            #[doc = "$top is supported for get list of runs, which limits the maximum number of runs to return."]
            pub fn top(mut self, top: i32) -> Self {
                self.top = Some(top);
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/runs",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
                                if let Some(filter) = &this.filter {
                                    req.url_mut().query_pairs_mut().append_pair("$filter", filter);
                                }
                                if let Some(top) = &this.top {
                                    req.url_mut().query_pairs_mut().append_pair("$top", &top.to_string());
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
                                let rsp_value: models::RunListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Run;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) run_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/runs/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.run_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Run = serde_json::from_slice(&rsp_body)?;
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Run),
            Created201(models::Run),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) run_id: String,
            pub(crate) run_update_parameters: models::RunUpdateParameters,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/runs/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.run_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.run_update_parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Run = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Run = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_log_sas_url {
        use super::models;
        type Response = models::RunGetLogResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) run_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/runs/{}/listLogSasUrl" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . registry_name , & this . run_id)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RunGetLogResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod cancel {
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
            pub(crate) registry_name: String,
            pub(crate) run_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/runs/{}/cancel",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.run_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
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
pub mod task_runs {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets the detailed information for a given task run."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `task_run_name`: The name of the task run."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            task_run_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                task_run_name: task_run_name.into(),
            }
        }
        #[doc = "Creates a task run for a container registry with the specified parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `task_run_name`: The name of the task run."]
        #[doc = "* `task_run`: The parameters of a run that needs to scheduled."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            task_run_name: impl Into<String>,
            task_run: impl Into<models::TaskRun>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                task_run_name: task_run_name.into(),
                task_run: task_run.into(),
            }
        }
        #[doc = "Updates a task run with the specified parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `task_run_name`: The name of the task run."]
        #[doc = "* `update_parameters`: The parameters for updating a task run."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            task_run_name: impl Into<String>,
            update_parameters: impl Into<models::TaskRunUpdateParameters>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                task_run_name: task_run_name.into(),
                update_parameters: update_parameters.into(),
            }
        }
        #[doc = "Deletes a specified task run resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `task_run_name`: The name of the task run."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            task_run_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                task_run_name: task_run_name.into(),
            }
        }
        #[doc = "Gets the detailed information for a given task run that includes all secrets."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `task_run_name`: The name of the task run."]
        pub fn get_details(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            task_run_name: impl Into<String>,
        ) -> get_details::Builder {
            get_details::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                task_run_name: task_run_name.into(),
            }
        }
        #[doc = "Lists all the task runs for a specified container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::TaskRun;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) task_run_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/taskRuns/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.task_run_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TaskRun = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::TaskRun),
            Created201(models::TaskRun),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) task_run_name: String,
            pub(crate) task_run: models::TaskRun,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/taskRuns/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.task_run_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.task_run)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TaskRun = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TaskRun = serde_json::from_slice(&rsp_body)?;
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::TaskRun),
            Created201(models::TaskRun),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) task_run_name: String,
            pub(crate) update_parameters: models::TaskRunUpdateParameters,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/taskRuns/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.task_run_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.update_parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TaskRun = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TaskRun = serde_json::from_slice(&rsp_body)?;
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
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) task_run_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/taskRuns/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.task_run_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
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
    pub mod get_details {
        use super::models;
        type Response = models::TaskRun;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) task_run_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/taskRuns/{}/listDetails" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . registry_name , & this . task_run_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TaskRun = serde_json::from_slice(&rsp_body)?;
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
    pub mod list {
        use super::models;
        type Response = models::TaskRunListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/taskRuns",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TaskRunListResult = serde_json::from_slice(&rsp_body)?;
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
pub mod tasks {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all the tasks for a specified container registry."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
            }
        }
        #[doc = "Get the properties of a specified task."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `task_name`: The name of the container registry task."]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            task_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                task_name: task_name.into(),
            }
        }
        #[doc = "Creates a task for a container registry with the specified parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `task_name`: The name of the container registry task."]
        #[doc = "* `task_create_parameters`: The parameters for creating a task."]
        pub fn create(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            task_name: impl Into<String>,
            task_create_parameters: impl Into<models::Task>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                task_name: task_name.into(),
                task_create_parameters: task_create_parameters.into(),
            }
        }
        #[doc = "Updates a task with the specified parameters."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `task_name`: The name of the container registry task."]
        #[doc = "* `task_update_parameters`: The parameters for updating a task."]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            task_name: impl Into<String>,
            task_update_parameters: impl Into<models::TaskUpdateParameters>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                task_name: task_name.into(),
                task_update_parameters: task_update_parameters.into(),
            }
        }
        #[doc = "Deletes a specified task."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `task_name`: The name of the container registry task."]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            task_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                task_name: task_name.into(),
            }
        }
        #[doc = "Returns a task with extended information that includes all secrets."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: The Microsoft Azure subscription ID."]
        #[doc = "* `resource_group_name`: The name of the resource group to which the container registry belongs."]
        #[doc = "* `registry_name`: The name of the container registry."]
        #[doc = "* `task_name`: The name of the container registry task."]
        pub fn get_details(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            registry_name: impl Into<String>,
            task_name: impl Into<String>,
        ) -> get_details::Builder {
            get_details::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                registry_name: registry_name.into(),
                task_name: task_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::TaskListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/tasks",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::TaskListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Task;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) task_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/tasks/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.task_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Task = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::Task),
            Created201(models::Task),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) task_name: String,
            pub(crate) task_create_parameters: models::Task,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/tasks/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.task_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.task_create_parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Task = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Task = serde_json::from_slice(&rsp_body)?;
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
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Task),
            Created201(models::Task),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) task_name: String,
            pub(crate) task_update_parameters: models::TaskUpdateParameters,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/tasks/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.task_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.task_update_parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Task = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Task = serde_json::from_slice(&rsp_body)?;
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
            Accepted202,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) task_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/tasks/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.registry_name,
                            &this.task_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
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
    pub mod get_details {
        use super::models;
        type Response = models::Task;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) registry_name: String,
            pub(crate) task_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/tasks/{}/listDetails" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . registry_name , & this . task_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2019-06-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Task = serde_json::from_slice(&rsp_body)?;
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
