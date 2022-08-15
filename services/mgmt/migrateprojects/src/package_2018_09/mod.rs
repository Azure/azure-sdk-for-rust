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
    pub fn database_instances_client(&self) -> database_instances::Client {
        database_instances::Client(self.clone())
    }
    pub fn databases_client(&self) -> databases::Client {
        databases::Client(self.clone())
    }
    pub fn events_client(&self) -> events::Client {
        events::Client(self.clone())
    }
    pub fn machines_client(&self) -> machines::Client {
        machines::Client(self.clone())
    }
    pub fn migrate_projects_client(&self) -> migrate_projects::Client {
        migrate_projects::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn solutions_client(&self) -> solutions::Client {
        solutions::Client(self.clone())
    }
}
pub mod database_instances {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a list of database instances in the migrate project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which migrate project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that migrate project is part of."]
        #[doc = "* `migrate_project_name`: Name of the Azure Migrate project."]
        pub fn enumerate_database_instances(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            migrate_project_name: impl Into<String>,
        ) -> enumerate_database_instances::Builder {
            enumerate_database_instances::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                migrate_project_name: migrate_project_name.into(),
                continuation_token: None,
                page_size: None,
                accept_language: None,
            }
        }
        #[doc = "Gets a database instance in the migrate project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which migrate project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that migrate project is part of."]
        #[doc = "* `migrate_project_name`: Name of the Azure Migrate project."]
        #[doc = "* `database_instance_name`: Unique name of a database instance in Azure migration hub."]
        pub fn get_database_instance(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            migrate_project_name: impl Into<String>,
            database_instance_name: impl Into<String>,
        ) -> get_database_instance::Builder {
            get_database_instance::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                migrate_project_name: migrate_project_name.into(),
                database_instance_name: database_instance_name.into(),
                accept_language: None,
            }
        }
    }
    pub mod enumerate_database_instances {
        use super::models;
        type Response = models::DatabaseInstanceCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) migrate_project_name: String,
            pub(crate) continuation_token: Option<String>,
            pub(crate) page_size: Option<i64>,
            pub(crate) accept_language: Option<String>,
        }
        impl Builder {
            #[doc = "The continuation token."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "The number of items to be returned in a single page. This value is honored only if it is less than the 100."]
            pub fn page_size(mut self, page_size: i64) -> Self {
                self.page_size = Some(page_size);
                self
            }
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/migrateProjects/{}/databaseInstances",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.migrate_project_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-09-01-preview");
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut().query_pairs_mut().append_pair("continuationToken", continuation_token);
                        }
                        if let Some(page_size) = &this.page_size {
                            req.url_mut().query_pairs_mut().append_pair("pageSize", &page_size.to_string());
                        }
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
                                let rsp_value: models::DatabaseInstanceCollection = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_database_instance {
        use super::models;
        type Response = models::DatabaseInstance;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) migrate_project_name: String,
            pub(crate) database_instance_name: String,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/migrateProjects/{}/databaseInstances/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.migrate_project_name,
                            &this.database_instance_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-09-01-preview");
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
                                let rsp_value: models::DatabaseInstance = serde_json::from_slice(&rsp_body)?;
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
pub mod databases {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a list of databases in the migrate project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which migrate project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that migrate project is part of."]
        #[doc = "* `migrate_project_name`: Name of the Azure Migrate project."]
        pub fn enumerate_databases(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            migrate_project_name: impl Into<String>,
        ) -> enumerate_databases::Builder {
            enumerate_databases::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                migrate_project_name: migrate_project_name.into(),
                continuation_token: None,
                page_size: None,
                accept_language: None,
            }
        }
        #[doc = "Gets a database in the migrate project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which migrate project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that migrate project is part of."]
        #[doc = "* `migrate_project_name`: Name of the Azure Migrate project."]
        #[doc = "* `database_name`: Unique name of a database in Azure migration hub."]
        pub fn get_database(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            migrate_project_name: impl Into<String>,
            database_name: impl Into<String>,
        ) -> get_database::Builder {
            get_database::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                migrate_project_name: migrate_project_name.into(),
                database_name: database_name.into(),
                accept_language: None,
            }
        }
    }
    pub mod enumerate_databases {
        use super::models;
        type Response = models::DatabaseCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) migrate_project_name: String,
            pub(crate) continuation_token: Option<String>,
            pub(crate) page_size: Option<i64>,
            pub(crate) accept_language: Option<String>,
        }
        impl Builder {
            #[doc = "The continuation token."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "The number of items to be returned in a single page. This value is honored only if it is less than the 100."]
            pub fn page_size(mut self, page_size: i64) -> Self {
                self.page_size = Some(page_size);
                self
            }
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/migrateProjects/{}/databases",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.migrate_project_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-09-01-preview");
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut().query_pairs_mut().append_pair("continuationToken", continuation_token);
                        }
                        if let Some(page_size) = &this.page_size {
                            req.url_mut().query_pairs_mut().append_pair("pageSize", &page_size.to_string());
                        }
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
                                let rsp_value: models::DatabaseCollection = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_database {
        use super::models;
        type Response = models::Database;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) migrate_project_name: String,
            pub(crate) database_name: String,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/migrateProjects/{}/databases/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.migrate_project_name,
                            &this.database_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-09-01-preview");
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
                                let rsp_value: models::Database = serde_json::from_slice(&rsp_body)?;
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
pub mod events {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a list of events in the migrate project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which migrate project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that migrate project is part of."]
        #[doc = "* `migrate_project_name`: Name of the Azure Migrate project."]
        pub fn enumerate_events(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            migrate_project_name: impl Into<String>,
        ) -> enumerate_events::Builder {
            enumerate_events::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                migrate_project_name: migrate_project_name.into(),
                continuation_token: None,
                page_size: None,
                accept_language: None,
            }
        }
        #[doc = "Gets an event in the migrate project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which migrate project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that migrate project is part of."]
        #[doc = "* `migrate_project_name`: Name of the Azure Migrate project."]
        #[doc = "* `event_name`: Unique name of an event within a migrate project."]
        pub fn get_event(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            migrate_project_name: impl Into<String>,
            event_name: impl Into<String>,
        ) -> get_event::Builder {
            get_event::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                migrate_project_name: migrate_project_name.into(),
                event_name: event_name.into(),
            }
        }
        #[doc = "Delete the migrate event"]
        #[doc = "Delete the migrate event. Deleting non-existent migrate event is a no-operation."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which migrate project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that migrate project is part of."]
        #[doc = "* `migrate_project_name`: Name of the Azure Migrate project."]
        #[doc = "* `event_name`: Unique name of an event within a migrate project."]
        pub fn delete_event(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            migrate_project_name: impl Into<String>,
            event_name: impl Into<String>,
        ) -> delete_event::Builder {
            delete_event::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                migrate_project_name: migrate_project_name.into(),
                event_name: event_name.into(),
            }
        }
    }
    pub mod enumerate_events {
        use super::models;
        type Response = models::EventCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) migrate_project_name: String,
            pub(crate) continuation_token: Option<String>,
            pub(crate) page_size: Option<i64>,
            pub(crate) accept_language: Option<String>,
        }
        impl Builder {
            #[doc = "The continuation token."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "The number of items to be returned in a single page. This value is honored only if it is less than the 100."]
            pub fn page_size(mut self, page_size: i64) -> Self {
                self.page_size = Some(page_size);
                self
            }
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/migrateProjects/{}/migrateEvents",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.migrate_project_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-09-01-preview");
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut().query_pairs_mut().append_pair("continuationToken", continuation_token);
                        }
                        if let Some(page_size) = &this.page_size {
                            req.url_mut().query_pairs_mut().append_pair("pageSize", &page_size.to_string());
                        }
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
                                let rsp_value: models::EventCollection = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_event {
        use super::models;
        type Response = models::MigrateEvent;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) migrate_project_name: String,
            pub(crate) event_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/migrateProjects/{}/migrateEvents/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.migrate_project_name,
                            &this.event_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MigrateEvent = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete_event {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) migrate_project_name: String,
            pub(crate) event_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/migrateProjects/{}/migrateEvents/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.migrate_project_name,
                            &this.event_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-09-01-preview");
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
pub mod machines {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a list of machines in the migrate project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which migrate project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that migrate project is part of."]
        #[doc = "* `migrate_project_name`: Name of the Azure Migrate project."]
        pub fn enumerate_machines(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            migrate_project_name: impl Into<String>,
        ) -> enumerate_machines::Builder {
            enumerate_machines::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                migrate_project_name: migrate_project_name.into(),
                continuation_token: None,
                page_size: None,
            }
        }
        #[doc = "Gets a machine in the migrate project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which migrate project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that migrate project is part of."]
        #[doc = "* `migrate_project_name`: Name of the Azure Migrate project."]
        #[doc = "* `machine_name`: Unique name of a machine in Azure migration hub."]
        pub fn get_machine(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            migrate_project_name: impl Into<String>,
            machine_name: impl Into<String>,
        ) -> get_machine::Builder {
            get_machine::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                migrate_project_name: migrate_project_name.into(),
                machine_name: machine_name.into(),
            }
        }
    }
    pub mod enumerate_machines {
        use super::models;
        type Response = models::MachineCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) migrate_project_name: String,
            pub(crate) continuation_token: Option<String>,
            pub(crate) page_size: Option<i64>,
        }
        impl Builder {
            #[doc = "The continuation token."]
            pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
                self.continuation_token = Some(continuation_token.into());
                self
            }
            #[doc = "The number of items to be returned in a single page. This value is honored only if it is less than the 100."]
            pub fn page_size(mut self, page_size: i64) -> Self {
                self.page_size = Some(page_size);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/migrateProjects/{}/machines",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.migrate_project_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-09-01-preview");
                        if let Some(continuation_token) = &this.continuation_token {
                            req.url_mut().query_pairs_mut().append_pair("continuationToken", continuation_token);
                        }
                        if let Some(page_size) = &this.page_size {
                            req.url_mut().query_pairs_mut().append_pair("pageSize", &page_size.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MachineCollection = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_machine {
        use super::models;
        type Response = models::Machine;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) migrate_project_name: String,
            pub(crate) machine_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/migrateProjects/{}/machines/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.migrate_project_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-09-01-preview");
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
pub mod migrate_projects {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Method to get a migrate project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which migrate project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that migrate project is part of."]
        #[doc = "* `migrate_project_name`: Name of the Azure Migrate project."]
        pub fn get_migrate_project(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            migrate_project_name: impl Into<String>,
        ) -> get_migrate_project::Builder {
            get_migrate_project::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                migrate_project_name: migrate_project_name.into(),
            }
        }
        #[doc = "Method to create or update a migrate project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which migrate project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that migrate project is part of."]
        #[doc = "* `migrate_project_name`: Name of the Azure Migrate project."]
        #[doc = "* `body`: Body with migrate project details."]
        pub fn put_migrate_project(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            migrate_project_name: impl Into<String>,
            body: impl Into<models::MigrateProject>,
        ) -> put_migrate_project::Builder {
            put_migrate_project::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                migrate_project_name: migrate_project_name.into(),
                body: body.into(),
                accept_language: None,
            }
        }
        #[doc = "Update migrate project."]
        #[doc = "Update a migrate project with specified name. Supports partial updates, for example only tags can be provided."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which migrate project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that migrate project is part of."]
        #[doc = "* `migrate_project_name`: Name of the Azure Migrate project."]
        #[doc = "* `body`: Body with migrate project details."]
        pub fn patch_migrate_project(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            migrate_project_name: impl Into<String>,
            body: impl Into<models::MigrateProject>,
        ) -> patch_migrate_project::Builder {
            patch_migrate_project::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                migrate_project_name: migrate_project_name.into(),
                body: body.into(),
                accept_language: None,
            }
        }
        #[doc = "Delete the migrate project"]
        #[doc = "Delete the migrate project. Deleting non-existent project is a no-operation."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which migrate project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that migrate project is part of."]
        #[doc = "* `migrate_project_name`: Name of the Azure Migrate project."]
        pub fn delete_migrate_project(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            migrate_project_name: impl Into<String>,
        ) -> delete_migrate_project::Builder {
            delete_migrate_project::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                migrate_project_name: migrate_project_name.into(),
                accept_language: None,
            }
        }
        #[doc = "Registers a tool with the migrate project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which migrate project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that migrate project is part of."]
        #[doc = "* `migrate_project_name`: Name of the Azure Migrate project."]
        #[doc = "* `input`: Input containing the name of the tool to be registered."]
        pub fn register_tool(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            migrate_project_name: impl Into<String>,
            input: impl Into<models::RegisterToolInput>,
        ) -> register_tool::Builder {
            register_tool::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                migrate_project_name: migrate_project_name.into(),
                input: input.into(),
                accept_language: None,
            }
        }
        #[doc = "Refresh the summary of the migrate project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which migrate project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that migrate project is part of."]
        #[doc = "* `migrate_project_name`: Name of the Azure Migrate project."]
        #[doc = "* `input`: The goal input which needs to be refreshed."]
        pub fn refresh_migrate_project_summary(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            migrate_project_name: impl Into<String>,
            input: impl Into<models::RefreshSummaryInput>,
        ) -> refresh_migrate_project_summary::Builder {
            refresh_migrate_project_summary::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                migrate_project_name: migrate_project_name.into(),
                input: input.into(),
            }
        }
    }
    pub mod get_migrate_project {
        use super::models;
        type Response = models::MigrateProject;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) migrate_project_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/migrateProjects/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.migrate_project_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MigrateProject = serde_json::from_slice(&rsp_body)?;
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
    pub mod put_migrate_project {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::MigrateProject),
            Created201(models::MigrateProject),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) migrate_project_name: String,
            pub(crate) body: models::MigrateProject,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/migrateProjects/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.migrate_project_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MigrateProject = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MigrateProject = serde_json::from_slice(&rsp_body)?;
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
    pub mod patch_migrate_project {
        use super::models;
        type Response = models::MigrateProject;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) migrate_project_name: String,
            pub(crate) body: models::MigrateProject,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/migrateProjects/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.migrate_project_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.body)?;
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::MigrateProject = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete_migrate_project {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) migrate_project_name: String,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/migrateProjects/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.migrate_project_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-09-01-preview");
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
    pub mod register_tool {
        use super::models;
        type Response = models::RegistrationResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) migrate_project_name: String,
            pub(crate) input: models::RegisterToolInput,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/migrateProjects/{}/registerTool",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.migrate_project_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.input)?;
                        if let Some(accept_language) = &this.accept_language {
                            req.insert_header("accept-language", accept_language);
                        }
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RegistrationResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod refresh_migrate_project_summary {
        use super::models;
        type Response = models::RefreshSummaryResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) migrate_project_name: String,
            pub(crate) input: models::RefreshSummaryInput,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/migrateProjects/{}/refreshSummary",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.migrate_project_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RefreshSummaryResult = serde_json::from_slice(&rsp_body)?;
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
pub mod solutions {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Gets a solution in the migrate project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which migrate project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that migrate project is part of."]
        #[doc = "* `migrate_project_name`: Name of the Azure Migrate project."]
        #[doc = "* `solution_name`: Unique name of a migration solution within a migrate project."]
        pub fn get_solution(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            migrate_project_name: impl Into<String>,
            solution_name: impl Into<String>,
        ) -> get_solution::Builder {
            get_solution::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                migrate_project_name: migrate_project_name.into(),
                solution_name: solution_name.into(),
            }
        }
        #[doc = "Creates a solution in the migrate project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which migrate project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that migrate project is part of."]
        #[doc = "* `migrate_project_name`: Name of the Azure Migrate project."]
        #[doc = "* `solution_name`: Unique name of a migration solution within a migrate project."]
        #[doc = "* `solution_input`: The input for the solution."]
        pub fn put_solution(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            migrate_project_name: impl Into<String>,
            solution_name: impl Into<String>,
            solution_input: impl Into<models::Solution>,
        ) -> put_solution::Builder {
            put_solution::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                migrate_project_name: migrate_project_name.into(),
                solution_name: solution_name.into(),
                solution_input: solution_input.into(),
            }
        }
        #[doc = "Update solution."]
        #[doc = "Update a solution with specified name. Supports partial updates, for example only tags can be provided."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which migrate project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that migrate project is part of."]
        #[doc = "* `migrate_project_name`: Name of the Azure Migrate project."]
        #[doc = "* `solution_name`: Unique name of a migration solution within a migrate project."]
        #[doc = "* `solution_input`: The input for the solution."]
        pub fn patch_solution(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            migrate_project_name: impl Into<String>,
            solution_name: impl Into<String>,
            solution_input: impl Into<models::Solution>,
        ) -> patch_solution::Builder {
            patch_solution::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                migrate_project_name: migrate_project_name.into(),
                solution_name: solution_name.into(),
                solution_input: solution_input.into(),
            }
        }
        #[doc = "Delete the solution"]
        #[doc = "Delete the solution. Deleting non-existent project is a no-operation."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which migrate project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that migrate project is part of."]
        #[doc = "* `migrate_project_name`: Name of the Azure Migrate project."]
        #[doc = "* `solution_name`: Unique name of a migration solution within a migrate project."]
        pub fn delete_solution(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            migrate_project_name: impl Into<String>,
            solution_name: impl Into<String>,
        ) -> delete_solution::Builder {
            delete_solution::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                migrate_project_name: migrate_project_name.into(),
                solution_name: solution_name.into(),
                accept_language: None,
            }
        }
        #[doc = "Gets the list of solutions in the migrate project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which migrate project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that migrate project is part of."]
        #[doc = "* `migrate_project_name`: Name of the Azure Migrate project."]
        pub fn enumerate_solutions(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            migrate_project_name: impl Into<String>,
        ) -> enumerate_solutions::Builder {
            enumerate_solutions::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                migrate_project_name: migrate_project_name.into(),
            }
        }
        #[doc = "Gets the config for the solution in the migrate project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which migrate project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that migrate project is part of."]
        #[doc = "* `migrate_project_name`: Name of the Azure Migrate project."]
        #[doc = "* `solution_name`: Unique name of a migration solution within a migrate project."]
        pub fn get_config(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            migrate_project_name: impl Into<String>,
            solution_name: impl Into<String>,
        ) -> get_config::Builder {
            get_config::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                migrate_project_name: migrate_project_name.into(),
                solution_name: solution_name.into(),
            }
        }
        #[doc = "Cleanup the solution data in the migrate project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Azure Subscription Id in which migrate project was created."]
        #[doc = "* `resource_group_name`: Name of the Azure Resource Group that migrate project is part of."]
        #[doc = "* `migrate_project_name`: Name of the Azure Migrate project."]
        #[doc = "* `solution_name`: Unique name of a migration solution within a migrate project."]
        pub fn cleanup_solution_data(
            &self,
            subscription_id: impl Into<String>,
            resource_group_name: impl Into<String>,
            migrate_project_name: impl Into<String>,
            solution_name: impl Into<String>,
        ) -> cleanup_solution_data::Builder {
            cleanup_solution_data::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                resource_group_name: resource_group_name.into(),
                migrate_project_name: migrate_project_name.into(),
                solution_name: solution_name.into(),
            }
        }
    }
    pub mod get_solution {
        use super::models;
        type Response = models::Solution;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) migrate_project_name: String,
            pub(crate) solution_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/migrateProjects/{}/solutions/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.migrate_project_name,
                            &this.solution_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Solution = serde_json::from_slice(&rsp_body)?;
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
    pub mod put_solution {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::Solution),
            Created201(models::Solution),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) migrate_project_name: String,
            pub(crate) solution_name: String,
            pub(crate) solution_input: models::Solution,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/migrateProjects/{}/solutions/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.migrate_project_name,
                            &this.solution_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.solution_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Solution = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Solution = serde_json::from_slice(&rsp_body)?;
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
    pub mod patch_solution {
        use super::models;
        type Response = models::Solution;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) migrate_project_name: String,
            pub(crate) solution_name: String,
            pub(crate) solution_input: models::Solution,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/migrateProjects/{}/solutions/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.migrate_project_name,
                            &this.solution_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-09-01-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.solution_input)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::Solution = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete_solution {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) migrate_project_name: String,
            pub(crate) solution_name: String,
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
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/migrateProjects/{}/solutions/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.migrate_project_name,
                            &this.solution_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-09-01-preview");
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
    pub mod enumerate_solutions {
        use super::models;
        type Response = models::SolutionsCollection;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) migrate_project_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/migrateProjects/{}/solutions",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.migrate_project_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SolutionsCollection = serde_json::from_slice(&rsp_body)?;
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
    pub mod get_config {
        use super::models;
        type Response = models::SolutionConfig;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) migrate_project_name: String,
            pub(crate) solution_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/migrateProjects/{}/solutions/{}/getConfig",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.migrate_project_name,
                            &this.solution_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-09-01-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SolutionConfig = serde_json::from_slice(&rsp_body)?;
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
    pub mod cleanup_solution_data {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) resource_group_name: String,
            pub(crate) migrate_project_name: String,
            pub(crate) solution_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Migrate/migrateProjects/{}/solutions/{}/cleanupData",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.migrate_project_name,
                            &this.solution_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-09-01-preview");
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
