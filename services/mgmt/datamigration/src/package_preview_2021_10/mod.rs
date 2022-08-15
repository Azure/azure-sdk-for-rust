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
    pub fn database_migrations_sql_mi_client(&self) -> database_migrations_sql_mi::Client {
        database_migrations_sql_mi::Client(self.clone())
    }
    pub fn database_migrations_sql_vm_client(&self) -> database_migrations_sql_vm::Client {
        database_migrations_sql_vm::Client(self.clone())
    }
    pub fn files_client(&self) -> files::Client {
        files::Client(self.clone())
    }
    pub fn operations_client(&self) -> operations::Client {
        operations::Client(self.clone())
    }
    pub fn projects_client(&self) -> projects::Client {
        projects::Client(self.clone())
    }
    pub fn resource_skus_client(&self) -> resource_skus::Client {
        resource_skus::Client(self.clone())
    }
    pub fn service_tasks_client(&self) -> service_tasks::Client {
        service_tasks::Client(self.clone())
    }
    pub fn services_client(&self) -> services::Client {
        services::Client(self.clone())
    }
    pub fn sql_migration_services_client(&self) -> sql_migration_services::Client {
        sql_migration_services::Client(self.clone())
    }
    pub fn tasks_client(&self) -> tasks::Client {
        tasks::Client(self.clone())
    }
    pub fn usages_client(&self) -> usages::Client {
        usages::Client(self.clone())
    }
}
pub mod database_migrations_sql_mi {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieve the Database Migration resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the resource group that contains the resource. You can obtain this value from the Azure Resource Manager API or the portal."]
        #[doc = "* `target_db_name`: The name of the target database."]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            managed_instance_name: impl Into<String>,
            target_db_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                managed_instance_name: managed_instance_name.into(),
                target_db_name: target_db_name.into(),
                subscription_id: subscription_id.into(),
                migration_operation_id: None,
                expand: None,
            }
        }
        #[doc = "Create or Update Database Migration resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the resource group that contains the resource. You can obtain this value from the Azure Resource Manager API or the portal."]
        #[doc = "* `target_db_name`: The name of the target database."]
        #[doc = "* `parameters`: Details of SqlMigrationService resource."]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        pub fn create_or_update(
            &self,
            resource_group_name: impl Into<String>,
            managed_instance_name: impl Into<String>,
            target_db_name: impl Into<String>,
            parameters: impl Into<models::DatabaseMigrationSqlMi>,
            subscription_id: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                managed_instance_name: managed_instance_name.into(),
                target_db_name: target_db_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Stop migrations in progress for the database"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the resource group that contains the resource. You can obtain this value from the Azure Resource Manager API or the portal."]
        #[doc = "* `target_db_name`: The name of the target database."]
        #[doc = "* `parameters`: Required migration operation ID for which cancel will be initiated."]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        pub fn cancel(
            &self,
            resource_group_name: impl Into<String>,
            managed_instance_name: impl Into<String>,
            target_db_name: impl Into<String>,
            parameters: impl Into<models::MigrationOperationInput>,
            subscription_id: impl Into<String>,
        ) -> cancel::Builder {
            cancel::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                managed_instance_name: managed_instance_name.into(),
                target_db_name: target_db_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Initiate cutover for online migration in progress for the database."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the resource group that contains the resource. You can obtain this value from the Azure Resource Manager API or the portal."]
        #[doc = "* `target_db_name`: The name of the target database."]
        #[doc = "* `parameters`: Required migration operation ID for which cutover will be initiated."]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        pub fn cutover(
            &self,
            resource_group_name: impl Into<String>,
            managed_instance_name: impl Into<String>,
            target_db_name: impl Into<String>,
            parameters: impl Into<models::MigrationOperationInput>,
            subscription_id: impl Into<String>,
        ) -> cutover::Builder {
            cutover::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                managed_instance_name: managed_instance_name.into(),
                target_db_name: target_db_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::DatabaseMigrationSqlMi;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) managed_instance_name: String,
            pub(crate) target_db_name: String,
            pub(crate) subscription_id: String,
            pub(crate) migration_operation_id: Option<String>,
            pub(crate) expand: Option<String>,
        }
        impl Builder {
            #[doc = "Optional migration operation ID. If this is provided, then details of migration operation for that ID are retrieved. If not provided (default), then details related to most recent or current operation are retrieved."]
            pub fn migration_operation_id(mut self, migration_operation_id: impl Into<String>) -> Self {
                self.migration_operation_id = Some(migration_operation_id.into());
                self
            }
            #[doc = "The child resources to include in the response."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Sql/managedInstances/{}/providers/Microsoft.DataMigration/databaseMigrations/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . managed_instance_name , & this . target_db_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        if let Some(migration_operation_id) = &this.migration_operation_id {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("migrationOperationId", migration_operation_id);
                        }
                        if let Some(expand) = &this.expand {
                            req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DatabaseMigrationSqlMi = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::DatabaseMigrationSqlMi),
            Created201(models::DatabaseMigrationSqlMi),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) managed_instance_name: String,
            pub(crate) target_db_name: String,
            pub(crate) parameters: models::DatabaseMigrationSqlMi,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Sql/managedInstances/{}/providers/Microsoft.DataMigration/databaseMigrations/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . managed_instance_name , & this . target_db_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DatabaseMigrationSqlMi = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DatabaseMigrationSqlMi = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) resource_group_name: String,
            pub(crate) managed_instance_name: String,
            pub(crate) target_db_name: String,
            pub(crate) parameters: models::MigrationOperationInput,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Sql/managedInstances/{}/providers/Microsoft.DataMigration/databaseMigrations/{}/cancel" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . managed_instance_name , & this . target_db_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
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
    pub mod cutover {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) managed_instance_name: String,
            pub(crate) target_db_name: String,
            pub(crate) parameters: models::MigrationOperationInput,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Sql/managedInstances/{}/providers/Microsoft.DataMigration/databaseMigrations/{}/cutover" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . managed_instance_name , & this . target_db_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
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
}
pub mod database_migrations_sql_vm {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieve the Database Migration resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the resource group that contains the resource. You can obtain this value from the Azure Resource Manager API or the portal."]
        #[doc = "* `target_db_name`: The name of the target database."]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            sql_virtual_machine_name: impl Into<String>,
            target_db_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                sql_virtual_machine_name: sql_virtual_machine_name.into(),
                target_db_name: target_db_name.into(),
                subscription_id: subscription_id.into(),
                migration_operation_id: None,
                expand: None,
            }
        }
        #[doc = "Create or Update Database Migration resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the resource group that contains the resource. You can obtain this value from the Azure Resource Manager API or the portal."]
        #[doc = "* `target_db_name`: The name of the target database."]
        #[doc = "* `parameters`: Details of SqlMigrationService resource."]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        pub fn create_or_update(
            &self,
            resource_group_name: impl Into<String>,
            sql_virtual_machine_name: impl Into<String>,
            target_db_name: impl Into<String>,
            parameters: impl Into<models::DatabaseMigrationSqlVm>,
            subscription_id: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                sql_virtual_machine_name: sql_virtual_machine_name.into(),
                target_db_name: target_db_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Stop ongoing migration for the database."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the resource group that contains the resource. You can obtain this value from the Azure Resource Manager API or the portal."]
        #[doc = "* `target_db_name`: The name of the target database."]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        pub fn cancel(
            &self,
            resource_group_name: impl Into<String>,
            sql_virtual_machine_name: impl Into<String>,
            target_db_name: impl Into<String>,
            parameters: impl Into<models::MigrationOperationInput>,
            subscription_id: impl Into<String>,
        ) -> cancel::Builder {
            cancel::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                sql_virtual_machine_name: sql_virtual_machine_name.into(),
                target_db_name: target_db_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Cutover online migration operation for the database."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the resource group that contains the resource. You can obtain this value from the Azure Resource Manager API or the portal."]
        #[doc = "* `target_db_name`: The name of the target database."]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        pub fn cutover(
            &self,
            resource_group_name: impl Into<String>,
            sql_virtual_machine_name: impl Into<String>,
            target_db_name: impl Into<String>,
            parameters: impl Into<models::MigrationOperationInput>,
            subscription_id: impl Into<String>,
        ) -> cutover::Builder {
            cutover::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                sql_virtual_machine_name: sql_virtual_machine_name.into(),
                target_db_name: target_db_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::DatabaseMigrationSqlVm;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) sql_virtual_machine_name: String,
            pub(crate) target_db_name: String,
            pub(crate) subscription_id: String,
            pub(crate) migration_operation_id: Option<String>,
            pub(crate) expand: Option<String>,
        }
        impl Builder {
            #[doc = "Optional migration operation ID. If this is provided, then details of migration operation for that ID are retrieved. If not provided (default), then details related to most recent or current operation are retrieved."]
            pub fn migration_operation_id(mut self, migration_operation_id: impl Into<String>) -> Self {
                self.migration_operation_id = Some(migration_operation_id.into());
                self
            }
            #[doc = "The child resources to include in the response."]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachines/{}/providers/Microsoft.DataMigration/databaseMigrations/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . sql_virtual_machine_name , & this . target_db_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        if let Some(migration_operation_id) = &this.migration_operation_id {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("migrationOperationId", migration_operation_id);
                        }
                        if let Some(expand) = &this.expand {
                            req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DatabaseMigrationSqlVm = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::DatabaseMigrationSqlVm),
            Created201(models::DatabaseMigrationSqlVm),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) sql_virtual_machine_name: String,
            pub(crate) target_db_name: String,
            pub(crate) parameters: models::DatabaseMigrationSqlVm,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachines/{}/providers/Microsoft.DataMigration/databaseMigrations/{}" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . sql_virtual_machine_name , & this . target_db_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Put);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DatabaseMigrationSqlVm = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DatabaseMigrationSqlVm = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) resource_group_name: String,
            pub(crate) sql_virtual_machine_name: String,
            pub(crate) target_db_name: String,
            pub(crate) parameters: models::MigrationOperationInput,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachines/{}/providers/Microsoft.DataMigration/databaseMigrations/{}/cancel" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . sql_virtual_machine_name , & this . target_db_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
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
    pub mod cutover {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) sql_virtual_machine_name: String,
            pub(crate) target_db_name: String,
            pub(crate) parameters: models::MigrationOperationInput,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.SqlVirtualMachine/sqlVirtualMachines/{}/providers/Microsoft.DataMigration/databaseMigrations/{}/cutover" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . sql_virtual_machine_name , & this . target_db_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
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
}
pub mod operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the available SQL Migration REST API operations."]
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
                            azure_core::Url::parse(&format!("{}/providers/Microsoft.DataMigration/operations", this.client.endpoint(),))?;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
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
pub mod sql_migration_services {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Retrieve the Migration Service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the resource group that contains the resource. You can obtain this value from the Azure Resource Manager API or the portal."]
        #[doc = "* `sql_migration_service_name`: Name of the SQL Migration Service."]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            sql_migration_service_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                sql_migration_service_name: sql_migration_service_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Create or Update SQL Migration Service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the resource group that contains the resource. You can obtain this value from the Azure Resource Manager API or the portal."]
        #[doc = "* `sql_migration_service_name`: Name of the SQL Migration Service."]
        #[doc = "* `parameters`: Details of SqlMigrationService resource."]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        pub fn create_or_update(
            &self,
            resource_group_name: impl Into<String>,
            sql_migration_service_name: impl Into<String>,
            parameters: impl Into<models::SqlMigrationService>,
            subscription_id: impl Into<String>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                sql_migration_service_name: sql_migration_service_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Update SQL Migration Service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the resource group that contains the resource. You can obtain this value from the Azure Resource Manager API or the portal."]
        #[doc = "* `sql_migration_service_name`: Name of the SQL Migration Service."]
        #[doc = "* `parameters`: Details of SqlMigrationService resource."]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        pub fn update(
            &self,
            resource_group_name: impl Into<String>,
            sql_migration_service_name: impl Into<String>,
            parameters: impl Into<models::SqlMigrationServiceUpdate>,
            subscription_id: impl Into<String>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                sql_migration_service_name: sql_migration_service_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Delete SQL Migration Service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the resource group that contains the resource. You can obtain this value from the Azure Resource Manager API or the portal."]
        #[doc = "* `sql_migration_service_name`: Name of the SQL Migration Service."]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            sql_migration_service_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                sql_migration_service_name: sql_migration_service_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Retrieve all SQL migration services in the resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the resource group that contains the resource. You can obtain this value from the Azure Resource Manager API or the portal."]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        pub fn list_by_resource_group(
            &self,
            resource_group_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_by_resource_group::Builder {
            list_by_resource_group::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Retrieve the List of Authentication Keys for Self Hosted Integration Runtime."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the resource group that contains the resource. You can obtain this value from the Azure Resource Manager API or the portal."]
        #[doc = "* `sql_migration_service_name`: Name of the SQL Migration Service."]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        pub fn list_auth_keys(
            &self,
            resource_group_name: impl Into<String>,
            sql_migration_service_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_auth_keys::Builder {
            list_auth_keys::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                sql_migration_service_name: sql_migration_service_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Regenerate a new set of Authentication Keys for Self Hosted Integration Runtime."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the resource group that contains the resource. You can obtain this value from the Azure Resource Manager API or the portal."]
        #[doc = "* `sql_migration_service_name`: Name of the SQL Migration Service."]
        #[doc = "* `parameters`: Details of SqlMigrationService resource."]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        pub fn regenerate_auth_keys(
            &self,
            resource_group_name: impl Into<String>,
            sql_migration_service_name: impl Into<String>,
            parameters: impl Into<models::RegenAuthKeys>,
            subscription_id: impl Into<String>,
        ) -> regenerate_auth_keys::Builder {
            regenerate_auth_keys::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                sql_migration_service_name: sql_migration_service_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Delete the integration runtime node."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the resource group that contains the resource. You can obtain this value from the Azure Resource Manager API or the portal."]
        #[doc = "* `sql_migration_service_name`: Name of the SQL Migration Service."]
        #[doc = "* `parameters`: Details of SqlMigrationService resource."]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        pub fn delete_node(
            &self,
            resource_group_name: impl Into<String>,
            sql_migration_service_name: impl Into<String>,
            parameters: impl Into<models::DeleteNode>,
            subscription_id: impl Into<String>,
        ) -> delete_node::Builder {
            delete_node::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                sql_migration_service_name: sql_migration_service_name.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Retrieve the List of database migrations attached to the service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the resource group that contains the resource. You can obtain this value from the Azure Resource Manager API or the portal."]
        #[doc = "* `sql_migration_service_name`: Name of the SQL Migration Service."]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        pub fn list_migrations(
            &self,
            resource_group_name: impl Into<String>,
            sql_migration_service_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_migrations::Builder {
            list_migrations::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                sql_migration_service_name: sql_migration_service_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Retrieve the Monitoring Data."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: Name of the resource group that contains the resource. You can obtain this value from the Azure Resource Manager API or the portal."]
        #[doc = "* `sql_migration_service_name`: Name of the SQL Migration Service."]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        pub fn list_monitoring_data(
            &self,
            resource_group_name: impl Into<String>,
            sql_migration_service_name: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list_monitoring_data::Builder {
            list_monitoring_data::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                sql_migration_service_name: sql_migration_service_name.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Retrieve all SQL migration services in the subscriptions."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        pub fn list_by_subscription(&self, subscription_id: impl Into<String>) -> list_by_subscription::Builder {
            list_by_subscription::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::SqlMigrationService;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) sql_migration_service_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/sqlMigrationServices/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.sql_migration_service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SqlMigrationService = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::SqlMigrationService),
            Created201(models::SqlMigrationService),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) sql_migration_service_name: String,
            pub(crate) parameters: models::SqlMigrationService,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/sqlMigrationServices/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.sql_migration_service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SqlMigrationService = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SqlMigrationService = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::SqlMigrationService),
            Created201(models::SqlMigrationService),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) sql_migration_service_name: String,
            pub(crate) parameters: models::SqlMigrationServiceUpdate,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/sqlMigrationServices/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.sql_migration_service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SqlMigrationService = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SqlMigrationService = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) resource_group_name: String,
            pub(crate) sql_migration_service_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/sqlMigrationServices/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.sql_migration_service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
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
        type Response = models::SqlMigrationListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/sqlMigrationServices",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SqlMigrationListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_auth_keys {
        use super::models;
        type Response = models::AuthenticationKeys;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) sql_migration_service_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/sqlMigrationServices/{}/listAuthKeys",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.sql_migration_service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::AuthenticationKeys = serde_json::from_slice(&rsp_body)?;
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
    pub mod regenerate_auth_keys {
        use super::models;
        type Response = models::RegenAuthKeys;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) sql_migration_service_name: String,
            pub(crate) parameters: models::RegenAuthKeys,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/sqlMigrationServices/{}/regenerateAuthKeys" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . sql_migration_service_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::RegenAuthKeys = serde_json::from_slice(&rsp_body)?;
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
    pub mod delete_node {
        use super::models;
        type Response = models::DeleteNode;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) sql_migration_service_name: String,
            pub(crate) parameters: models::DeleteNode,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/sqlMigrationServices/{}/deleteNode",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.sql_migration_service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DeleteNode = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_migrations {
        use super::models;
        type Response = models::DatabaseMigrationListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) sql_migration_service_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/sqlMigrationServices/{}/listMigrations" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . sql_migration_service_name)) ? ;
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DatabaseMigrationListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_monitoring_data {
        use super::models;
        type Response = models::IntegrationRuntimeMonitoringData;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) sql_migration_service_name: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/sqlMigrationServices/{}/listMonitoringData" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . sql_migration_service_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::IntegrationRuntimeMonitoringData = serde_json::from_slice(&rsp_body)?;
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
    pub mod list_by_subscription {
        use super::models;
        type Response = models::SqlMigrationListResult;
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
                            "{}/subscriptions/{}/providers/Microsoft.DataMigration/sqlMigrationServices",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SqlMigrationListResult = serde_json::from_slice(&rsp_body)?;
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
pub mod resource_skus {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get supported SKUs"]
        #[doc = "The skus action returns the list of SKUs that DMS supports."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        pub fn list_skus(&self, subscription_id: impl Into<String>) -> list_skus::Builder {
            list_skus::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list_skus {
        use super::models;
        type Response = models::ResourceSkusResult;
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
                            "{}/subscriptions/{}/providers/Microsoft.DataMigration/skus",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ResourceSkusResult = serde_json::from_slice(&rsp_body)?;
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
pub mod services {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get DMS Service Instance"]
        #[doc = "The services resource is the top-level resource that represents the Database Migration Service. The GET method retrieves information about a service instance."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
            }
        }
        #[doc = "Create or update DMS Instance"]
        #[doc = "The services resource is the top-level resource that represents the Database Migration Service. The PUT method creates a new service or updates an existing one. When a service is updated, existing child resources (i.e. tasks) are unaffected. Services currently support a single kind, \"vm\", which refers to a VM-based service, although other kinds may be added in the future. This method can change the kind, SKU, and network of the service, but if tasks are currently running (i.e. the service is busy), this will fail with 400 Bad Request (\"ServiceIsBusy\"). The provider will reply when successful with 200 OK or 201 Created. Long-running operations use the provisioningState property."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `parameters`: Information about the service"]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            parameters: impl Into<models::DataMigrationService>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Create or update DMS Service Instance"]
        #[doc = "The services resource is the top-level resource that represents the Database Migration Service. The PATCH method updates an existing service. This method can change the kind, SKU, and network of the service, but if tasks are currently running (i.e. the service is busy), this will fail with 400 Bad Request (\"ServiceIsBusy\")."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `parameters`: Information about the service"]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            parameters: impl Into<models::DataMigrationService>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Delete DMS Service Instance"]
        #[doc = "The services resource is the top-level resource that represents the Database Migration Service. The DELETE method deletes a service. Any running tasks will be canceled."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                delete_running_tasks: None,
            }
        }
        #[doc = "Check service health status"]
        #[doc = "The services resource is the top-level resource that represents the Database Migration Service. This action performs a health check and returns the status of the service and virtual machine size."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        pub fn check_status(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
        ) -> check_status::Builder {
            check_status::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
            }
        }
        #[doc = "Start service"]
        #[doc = "The services resource is the top-level resource that represents the Database Migration Service. This action starts the service and the service can be used for data migration."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        pub fn start(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
        ) -> start::Builder {
            start::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
            }
        }
        #[doc = "Stop service"]
        #[doc = "The services resource is the top-level resource that represents the Database Migration Service. This action stops the service and the service cannot be used for data migration. The service owner won't be billed when the service is stopped."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        pub fn stop(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
        ) -> stop::Builder {
            stop::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
            }
        }
        #[doc = "Get compatible SKUs"]
        #[doc = "The services resource is the top-level resource that represents the Database Migration Service. The skus action returns the list of SKUs that a service resource can be updated to."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        pub fn list_skus(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
        ) -> list_skus::Builder {
            list_skus::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
            }
        }
        #[doc = "Check nested resource name validity and availability"]
        #[doc = "This method checks whether a proposed nested resource name is valid and available."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `parameters`: Requested name to validate"]
        pub fn check_children_name_availability(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            parameters: impl Into<models::NameAvailabilityRequest>,
        ) -> check_children_name_availability::Builder {
            check_children_name_availability::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Get services in resource group"]
        #[doc = "The Services resource is the top-level resource that represents the Database Migration Service. This method returns a list of service resources in a resource group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        pub fn list_by_resource_group(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
        ) -> list_by_resource_group::Builder {
            list_by_resource_group::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
            }
        }
        #[doc = "Get services in subscription"]
        #[doc = "The services resource is the top-level resource that represents the Database Migration Service. This method returns a list of service resources in a subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        pub fn list(&self, subscription_id: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Check name validity and availability"]
        #[doc = "This method checks whether a proposed top-level resource name is valid and available."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `location`: The Azure region of the operation"]
        #[doc = "* `parameters`: Requested name to validate"]
        pub fn check_name_availability(
            &self,
            subscription_id: impl Into<String>,
            location: impl Into<String>,
            parameters: impl Into<models::NameAvailabilityRequest>,
        ) -> check_name_availability::Builder {
            check_name_availability::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                location: location.into(),
                parameters: parameters.into(),
            }
        }
    }
    pub mod get {
        use super::models;
        type Response = models::DataMigrationService;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DataMigrationService = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::DataMigrationService),
            Created201(models::DataMigrationService),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) parameters: models::DataMigrationService,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DataMigrationService = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DataMigrationService = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Created201(rsp_value))
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
    pub mod update {
        use super::models;
        #[derive(Debug)]
        pub enum Response {
            Ok200(models::DataMigrationService),
            Accepted202,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) parameters: models::DataMigrationService,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DataMigrationService = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) delete_running_tasks: Option<bool>,
        }
        impl Builder {
            #[doc = "Delete the resource even if it contains running tasks"]
            pub fn delete_running_tasks(mut self, delete_running_tasks: bool) -> Self {
                self.delete_running_tasks = Some(delete_running_tasks);
                self
            }
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        if let Some(delete_running_tasks) = &this.delete_running_tasks {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("deleteRunningTasks", &delete_running_tasks.to_string());
                        }
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
    pub mod check_status {
        use super::models;
        type Response = models::DataMigrationServiceStatusResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/checkStatus",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DataMigrationServiceStatusResponse = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) group_name: String,
            pub(crate) service_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/start",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
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
            pub(crate) group_name: String,
            pub(crate) service_name: String,
        }
        impl Builder {
            #[doc = "only the first response will be fetched as long running operations are not supported yet"]
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/stop",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
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
    pub mod list_skus {
        use super::models;
        type Response = models::ServiceSkuList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/skus",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ServiceSkuList = serde_json::from_slice(&rsp_body)?;
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
    pub mod check_children_name_availability {
        use super::models;
        type Response = models::NameAvailabilityResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) parameters: models::NameAvailabilityRequest,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/checkNameAvailability",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NameAvailabilityResponse = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::DataMigrationServiceList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DataMigrationServiceList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::DataMigrationServiceList;
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
                            "{}/subscriptions/{}/providers/Microsoft.DataMigration/services",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::DataMigrationServiceList = serde_json::from_slice(&rsp_body)?;
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
    pub mod check_name_availability {
        use super::models;
        type Response = models::NameAvailabilityResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) location: String,
            pub(crate) parameters: models::NameAvailabilityRequest,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.DataMigration/locations/{}/checkNameAvailability",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.location
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::NameAvailabilityResponse = serde_json::from_slice(&rsp_body)?;
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
pub mod tasks {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get tasks in a service"]
        #[doc = "The services resource is the top-level resource that represents the Database Migration Service. This method returns a list of tasks owned by a service resource. Some tasks may have a status of Unknown, which indicates that an error occurred while querying the status of that task."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `project_name`: Name of the project"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            project_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                project_name: project_name.into(),
                task_type: None,
            }
        }
        #[doc = "Get task information"]
        #[doc = "The tasks resource is a nested, proxy-only resource representing work performed by a DMS instance. The GET method retrieves information about a task."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `project_name`: Name of the project"]
        #[doc = "* `task_name`: Name of the Task"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            project_name: impl Into<String>,
            task_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                project_name: project_name.into(),
                task_name: task_name.into(),
                expand: None,
            }
        }
        #[doc = "Create or update task"]
        #[doc = "The tasks resource is a nested, proxy-only resource representing work performed by a DMS instance. The PUT method creates a new task or updates an existing one, although since tasks have no mutable custom properties, there is little reason to update an existing one."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `project_name`: Name of the project"]
        #[doc = "* `task_name`: Name of the Task"]
        #[doc = "* `parameters`: Information about the task"]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            project_name: impl Into<String>,
            task_name: impl Into<String>,
            parameters: impl Into<models::ProjectTask>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                project_name: project_name.into(),
                task_name: task_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Create or update task"]
        #[doc = "The tasks resource is a nested, proxy-only resource representing work performed by a DMS instance. The PATCH method updates an existing task, but since tasks have no mutable custom properties, there is little reason to do so."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `project_name`: Name of the project"]
        #[doc = "* `task_name`: Name of the Task"]
        #[doc = "* `parameters`: Information about the task"]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            project_name: impl Into<String>,
            task_name: impl Into<String>,
            parameters: impl Into<models::ProjectTask>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                project_name: project_name.into(),
                task_name: task_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Delete task"]
        #[doc = "The tasks resource is a nested, proxy-only resource representing work performed by a DMS instance. The DELETE method deletes a task, canceling it first if it's running."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `project_name`: Name of the project"]
        #[doc = "* `task_name`: Name of the Task"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            project_name: impl Into<String>,
            task_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                project_name: project_name.into(),
                task_name: task_name.into(),
                delete_running_tasks: None,
            }
        }
        #[doc = "Cancel a task"]
        #[doc = "The tasks resource is a nested, proxy-only resource representing work performed by a DMS instance. This method cancels a task if it's currently queued or running."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `project_name`: Name of the project"]
        #[doc = "* `task_name`: Name of the Task"]
        pub fn cancel(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            project_name: impl Into<String>,
            task_name: impl Into<String>,
        ) -> cancel::Builder {
            cancel::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                project_name: project_name.into(),
                task_name: task_name.into(),
            }
        }
        #[doc = "Execute a command on a task"]
        #[doc = "The tasks resource is a nested, proxy-only resource representing work performed by a DMS instance. This method executes a command on a running task."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `project_name`: Name of the project"]
        #[doc = "* `task_name`: Name of the Task"]
        #[doc = "* `parameters`: Command to execute"]
        pub fn command(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            project_name: impl Into<String>,
            task_name: impl Into<String>,
            parameters: impl Into<models::CommandProperties>,
        ) -> command::Builder {
            command::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                project_name: project_name.into(),
                task_name: task_name.into(),
                parameters: parameters.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::TaskList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) project_name: String,
            pub(crate) task_type: Option<String>,
        }
        impl Builder {
            #[doc = "Filter tasks by task type"]
            pub fn task_type(mut self, task_type: impl Into<String>) -> Self {
                self.task_type = Some(task_type.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/projects/{}/tasks",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name,
                            &this.project_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                                if let Some(task_type) = &this.task_type {
                                    req.url_mut().query_pairs_mut().append_pair("taskType", task_type);
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
                                let rsp_value: models::TaskList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ProjectTask;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) project_name: String,
            pub(crate) task_name: String,
            pub(crate) expand: Option<String>,
        }
        impl Builder {
            #[doc = "Expand the response"]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/projects/{}/tasks/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name,
                            &this.project_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        if let Some(expand) = &this.expand {
                            req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProjectTask = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::ProjectTask),
            Created201(models::ProjectTask),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) project_name: String,
            pub(crate) task_name: String,
            pub(crate) parameters: models::ProjectTask,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/projects/{}/tasks/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name,
                            &this.project_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProjectTask = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProjectTask = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ProjectTask;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) project_name: String,
            pub(crate) task_name: String,
            pub(crate) parameters: models::ProjectTask,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/projects/{}/tasks/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name,
                            &this.project_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProjectTask = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) project_name: String,
            pub(crate) task_name: String,
            pub(crate) delete_running_tasks: Option<bool>,
        }
        impl Builder {
            #[doc = "Delete the resource even if it contains running tasks"]
            pub fn delete_running_tasks(mut self, delete_running_tasks: bool) -> Self {
                self.delete_running_tasks = Some(delete_running_tasks);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/projects/{}/tasks/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name,
                            &this.project_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        if let Some(delete_running_tasks) = &this.delete_running_tasks {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("deleteRunningTasks", &delete_running_tasks.to_string());
                        }
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
    pub mod cancel {
        use super::models;
        type Response = models::ProjectTask;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) project_name: String,
            pub(crate) task_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/projects/{}/tasks/{}/cancel" , this . client . endpoint () , & this . subscription_id , & this . group_name , & this . service_name , & this . project_name , & this . task_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProjectTask = serde_json::from_slice(&rsp_body)?;
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
    pub mod command {
        use super::models;
        type Response = models::CommandProperties;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) project_name: String,
            pub(crate) task_name: String,
            pub(crate) parameters: models::CommandProperties,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/projects/{}/tasks/{}/command" , this . client . endpoint () , & this . subscription_id , & this . group_name , & this . service_name , & this . project_name , & this . task_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::CommandProperties = serde_json::from_slice(&rsp_body)?;
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
pub mod service_tasks {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get service level tasks for a service"]
        #[doc = "The services resource is the top-level resource that represents the Database Migration Service. This method returns a list of service level tasks owned by a service resource. Some tasks may have a status of Unknown, which indicates that an error occurred while querying the status of that task."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                task_type: None,
            }
        }
        #[doc = "Get service task information"]
        #[doc = "The service tasks resource is a nested, proxy-only resource representing work performed by a DMS instance. The GET method retrieves information about a service task."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `task_name`: Name of the Task"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            task_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                task_name: task_name.into(),
                expand: None,
            }
        }
        #[doc = "Create or update service task"]
        #[doc = "The service tasks resource is a nested, proxy-only resource representing work performed by a DMS instance. The PUT method creates a new service task or updates an existing one, although since service tasks have no mutable custom properties, there is little reason to update an existing one."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `task_name`: Name of the Task"]
        #[doc = "* `parameters`: Information about the task"]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            task_name: impl Into<String>,
            parameters: impl Into<models::ProjectTask>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                task_name: task_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Create or update service task"]
        #[doc = "The service tasks resource is a nested, proxy-only resource representing work performed by a DMS instance. The PATCH method updates an existing service task, but since service tasks have no mutable custom properties, there is little reason to do so."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `task_name`: Name of the Task"]
        #[doc = "* `parameters`: Information about the task"]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            task_name: impl Into<String>,
            parameters: impl Into<models::ProjectTask>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                task_name: task_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Delete service task"]
        #[doc = "The service tasks resource is a nested, proxy-only resource representing work performed by a DMS instance. The DELETE method deletes a service task, canceling it first if it's running."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `task_name`: Name of the Task"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            task_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                task_name: task_name.into(),
                delete_running_tasks: None,
            }
        }
        #[doc = "Cancel a service task"]
        #[doc = "The service tasks resource is a nested, proxy-only resource representing work performed by a DMS instance. This method cancels a service task if it's currently queued or running."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `task_name`: Name of the Task"]
        pub fn cancel(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            task_name: impl Into<String>,
        ) -> cancel::Builder {
            cancel::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                task_name: task_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::TaskList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) task_type: Option<String>,
        }
        impl Builder {
            #[doc = "Filter tasks by task type"]
            pub fn task_type(mut self, task_type: impl Into<String>) -> Self {
                self.task_type = Some(task_type.into());
                self
            }
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/serviceTasks",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                                if let Some(task_type) = &this.task_type {
                                    req.url_mut().query_pairs_mut().append_pair("taskType", task_type);
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
                                let rsp_value: models::TaskList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ProjectTask;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) task_name: String,
            pub(crate) expand: Option<String>,
        }
        impl Builder {
            #[doc = "Expand the response"]
            pub fn expand(mut self, expand: impl Into<String>) -> Self {
                self.expand = Some(expand.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/serviceTasks/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        if let Some(expand) = &this.expand {
                            req.url_mut().query_pairs_mut().append_pair("$expand", expand);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProjectTask = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::ProjectTask),
            Created201(models::ProjectTask),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) task_name: String,
            pub(crate) parameters: models::ProjectTask,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/serviceTasks/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProjectTask = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProjectTask = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ProjectTask;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) task_name: String,
            pub(crate) parameters: models::ProjectTask,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/serviceTasks/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProjectTask = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) task_name: String,
            pub(crate) delete_running_tasks: Option<bool>,
        }
        impl Builder {
            #[doc = "Delete the resource even if it contains running tasks"]
            pub fn delete_running_tasks(mut self, delete_running_tasks: bool) -> Self {
                self.delete_running_tasks = Some(delete_running_tasks);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/serviceTasks/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        if let Some(delete_running_tasks) = &this.delete_running_tasks {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("deleteRunningTasks", &delete_running_tasks.to_string());
                        }
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
    pub mod cancel {
        use super::models;
        type Response = models::ProjectTask;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) task_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/serviceTasks/{}/cancel",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name,
                            &this.task_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProjectTask = serde_json::from_slice(&rsp_body)?;
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
        #[doc = "Get projects in a service"]
        #[doc = "The project resource is a nested resource representing a stored migration project. This method returns a list of projects owned by a service resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
            }
        }
        #[doc = "Get project information"]
        #[doc = "The project resource is a nested resource representing a stored migration project. The GET method retrieves information about a project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `project_name`: Name of the project"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            project_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                project_name: project_name.into(),
            }
        }
        #[doc = "Create or update project"]
        #[doc = "The project resource is a nested resource representing a stored migration project. The PUT method creates a new project or updates an existing one."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `project_name`: Name of the project"]
        #[doc = "* `parameters`: Information about the project"]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            project_name: impl Into<String>,
            parameters: impl Into<models::Project>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                project_name: project_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Update project"]
        #[doc = "The project resource is a nested resource representing a stored migration project. The PATCH method updates an existing project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `project_name`: Name of the project"]
        #[doc = "* `parameters`: Information about the project"]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            project_name: impl Into<String>,
            parameters: impl Into<models::Project>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                project_name: project_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Delete project"]
        #[doc = "The project resource is a nested resource representing a stored migration project. The DELETE method deletes a project."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `project_name`: Name of the project"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            project_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                project_name: project_name.into(),
                delete_running_tasks: None,
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::ProjectList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/projects",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProjectList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::Project;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) project_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/projects/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
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
    pub mod create_or_update {
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
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) project_name: String,
            pub(crate) parameters: models::Project,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/projects/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
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
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) project_name: String,
            pub(crate) parameters: models::Project,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/projects/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
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
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) project_name: String,
            pub(crate) delete_running_tasks: Option<bool>,
        }
        impl Builder {
            #[doc = "Delete the resource even if it contains running tasks"]
            pub fn delete_running_tasks(mut self, delete_running_tasks: bool) -> Self {
                self.delete_running_tasks = Some(delete_running_tasks);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/projects/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name,
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        if let Some(delete_running_tasks) = &this.delete_running_tasks {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("deleteRunningTasks", &delete_running_tasks.to_string());
                        }
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
pub mod usages {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get resource quotas and usage information"]
        #[doc = "This method returns region-specific quotas and resource usage information for the Database Migration Service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `location`: The Azure region of the operation"]
        pub fn list(&self, subscription_id: impl Into<String>, location: impl Into<String>) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                location: location.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::QuotaList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) location: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.DataMigration/locations/{}/usages",
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::QuotaList = serde_json::from_slice(&rsp_body)?;
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
pub mod files {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get files in a project"]
        #[doc = "The project resource is a nested resource representing a stored migration project. This method returns a list of files owned by a project resource."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `project_name`: Name of the project"]
        pub fn list(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            project_name: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                project_name: project_name.into(),
            }
        }
        #[doc = "Get file information"]
        #[doc = "The files resource is a nested, proxy-only resource representing a file stored under the project resource. This method retrieves information about a file."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `project_name`: Name of the project"]
        #[doc = "* `file_name`: Name of the File"]
        pub fn get(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            project_name: impl Into<String>,
            file_name: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                project_name: project_name.into(),
                file_name: file_name.into(),
            }
        }
        #[doc = "Create a file resource"]
        #[doc = "The PUT method creates a new file or updates an existing one."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `project_name`: Name of the project"]
        #[doc = "* `file_name`: Name of the File"]
        #[doc = "* `parameters`: Information about the file"]
        pub fn create_or_update(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            project_name: impl Into<String>,
            file_name: impl Into<String>,
            parameters: impl Into<models::ProjectFile>,
        ) -> create_or_update::Builder {
            create_or_update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                project_name: project_name.into(),
                file_name: file_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Update a file"]
        #[doc = "This method updates an existing file."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `project_name`: Name of the project"]
        #[doc = "* `file_name`: Name of the File"]
        #[doc = "* `parameters`: Information about the file"]
        pub fn update(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            project_name: impl Into<String>,
            file_name: impl Into<String>,
            parameters: impl Into<models::ProjectFile>,
        ) -> update::Builder {
            update::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                project_name: project_name.into(),
                file_name: file_name.into(),
                parameters: parameters.into(),
            }
        }
        #[doc = "Delete file"]
        #[doc = "This method deletes a file."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `project_name`: Name of the project"]
        #[doc = "* `file_name`: Name of the File"]
        pub fn delete(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            project_name: impl Into<String>,
            file_name: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                project_name: project_name.into(),
                file_name: file_name.into(),
            }
        }
        #[doc = "Request storage information for downloading the file content"]
        #[doc = "This method is used for requesting storage information using which contents of the file can be downloaded."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `project_name`: Name of the project"]
        #[doc = "* `file_name`: Name of the File"]
        pub fn read(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            project_name: impl Into<String>,
            file_name: impl Into<String>,
        ) -> read::Builder {
            read::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                project_name: project_name.into(),
                file_name: file_name.into(),
            }
        }
        #[doc = "Request information for reading and writing file content."]
        #[doc = "This method is used for requesting information for reading and writing the file content."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID that identifies an Azure subscription."]
        #[doc = "* `group_name`: Name of the resource group"]
        #[doc = "* `service_name`: Name of the service"]
        #[doc = "* `project_name`: Name of the project"]
        #[doc = "* `file_name`: Name of the File"]
        pub fn read_write(
            &self,
            subscription_id: impl Into<String>,
            group_name: impl Into<String>,
            service_name: impl Into<String>,
            project_name: impl Into<String>,
            file_name: impl Into<String>,
        ) -> read_write::Builder {
            read_write::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
                group_name: group_name.into(),
                service_name: service_name.into(),
                project_name: project_name.into(),
                file_name: file_name.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::FileList;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) project_name: String,
        }
        impl Builder {
            pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                let make_request = move |continuation: Option<String>| {
                    let this = self.clone();
                    async move {
                        let mut url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/projects/{}/files",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name,
                            &this.project_name
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
                                        .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
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
                                    .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                                let req_body = azure_core::EMPTY_BODY;
                                req.set_body(req_body);
                                this.client.send(&mut req).await?
                            }
                        };
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FileList = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ProjectFile;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) project_name: String,
            pub(crate) file_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/projects/{}/files/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name,
                            &this.project_name,
                            &this.file_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProjectFile = serde_json::from_slice(&rsp_body)?;
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
            Ok200(models::ProjectFile),
            Created201(models::ProjectFile),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) project_name: String,
            pub(crate) file_name: String,
            pub(crate) parameters: models::ProjectFile,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/projects/{}/files/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name,
                            &this.project_name,
                            &this.file_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProjectFile = serde_json::from_slice(&rsp_body)?;
                                Ok(Response::Ok200(rsp_value))
                            }
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProjectFile = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::ProjectFile;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) project_name: String,
            pub(crate) file_name: String,
            pub(crate) parameters: models::ProjectFile,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/projects/{}/files/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name,
                            &this.project_name,
                            &this.file_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ProjectFile = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) project_name: String,
            pub(crate) file_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/projects/{}/files/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name,
                            &this.project_name,
                            &this.file_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
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
    pub mod read {
        use super::models;
        type Response = models::FileStorageInfo;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) project_name: String,
            pub(crate) file_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/projects/{}/files/{}/read",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.group_name,
                            &this.service_name,
                            &this.project_name,
                            &this.file_name
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FileStorageInfo = serde_json::from_slice(&rsp_body)?;
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
    pub mod read_write {
        use super::models;
        type Response = models::FileStorageInfo;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
            pub(crate) group_name: String,
            pub(crate) service_name: String,
            pub(crate) project_name: String,
            pub(crate) file_name: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.DataMigration/services/{}/projects/{}/files/{}/readwrite" , this . client . endpoint () , & this . subscription_id , & this . group_name , & this . service_name , & this . project_name , & this . file_name)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-30-preview");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::FileStorageInfo = serde_json::from_slice(&rsp_body)?;
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
