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
    pub fn serial_ports_client(&self) -> serial_ports::Client {
        serial_ports::Client(self.clone())
    }
}
impl Client {
    #[doc = "Gets a list of Serial Console API operations."]
    pub fn list_operations(&self) -> list_operations::Builder {
        list_operations::Builder { client: self.clone() }
    }
    #[doc = "Get the disabled status for a subscription"]
    #[doc = "Gets whether or not Serial Console is disabled for a given subscription"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `subscription_id`: Subscription ID which uniquely identifies the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call requiring it."]
    #[doc = "* `default`: Default parameter. Leave the value as \"default\"."]
    pub fn get_console_status(&self, subscription_id: impl Into<String>, default: impl Into<String>) -> get_console_status::Builder {
        get_console_status::Builder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            default: default.into(),
        }
    }
    #[doc = "Disable Serial Console for a subscription"]
    #[doc = "Disables the Serial Console service for all VMs and VM scale sets in the provided subscription"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `subscription_id`: Subscription ID which uniquely identifies the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call requiring it."]
    #[doc = "* `default`: Default parameter. Leave the value as \"default\"."]
    pub fn disable_console(&self, subscription_id: impl Into<String>, default: impl Into<String>) -> disable_console::Builder {
        disable_console::Builder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            default: default.into(),
        }
    }
    #[doc = "Enable Serial Console for a subscription"]
    #[doc = "Enables the Serial Console service for all VMs and VM scale sets in the provided subscription"]
    #[doc = ""]
    #[doc = "Arguments:"]
    #[doc = "* `subscription_id`: Subscription ID which uniquely identifies the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call requiring it."]
    #[doc = "* `default`: Default parameter. Leave the value as \"default\"."]
    pub fn enable_console(&self, subscription_id: impl Into<String>, default: impl Into<String>) -> enable_console::Builder {
        enable_console::Builder {
            client: self.clone(),
            subscription_id: subscription_id.into(),
            default: default.into(),
        }
    }
}
pub mod list_operations {
    use super::models;
    type Response = models::SerialConsoleOperations;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!("{}/providers/Microsoft.SerialConsole/operations", this.client.endpoint(),))?;
                    let mut req = azure_core::Request::new(url, azure_core::Method::Get);
                    let credential = this.client.token_credential();
                    let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                    req.insert_header(
                        azure_core::headers::AUTHORIZATION,
                        format!("Bearer {}", token_response.token.secret()),
                    );
                    req.url_mut()
                        .query_pairs_mut()
                        .append_pair(azure_core::query_param::API_VERSION, "2018-05-01");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::SerialConsoleOperations = serde_json::from_slice(&rsp_body)?;
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
pub mod get_console_status {
    use super::models;
    type Response = models::SerialConsoleStatus;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) default: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/providers/Microsoft.SerialConsole/consoleServices/{}",
                        this.client.endpoint(),
                        &this.subscription_id,
                        &this.default
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
                        .append_pair(azure_core::query_param::API_VERSION, "2018-05-01");
                    let req_body = azure_core::EMPTY_BODY;
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::SerialConsoleStatus = serde_json::from_slice(&rsp_body)?;
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
pub mod disable_console {
    use super::models;
    type Response = models::DisableSerialConsoleResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) default: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/providers/Microsoft.SerialConsole/consoleServices/{}/disableConsole",
                        this.client.endpoint(),
                        &this.subscription_id,
                        &this.default
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
                        .append_pair(azure_core::query_param::API_VERSION, "2018-05-01");
                    let req_body = azure_core::EMPTY_BODY;
                    req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::DisableSerialConsoleResult = serde_json::from_slice(&rsp_body)?;
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
pub mod enable_console {
    use super::models;
    type Response = models::EnableSerialConsoleResult;
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) subscription_id: String,
        pub(crate) default: String,
    }
    impl Builder {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url = azure_core::Url::parse(&format!(
                        "{}/subscriptions/{}/providers/Microsoft.SerialConsole/consoleServices/{}/enableConsole",
                        this.client.endpoint(),
                        &this.subscription_id,
                        &this.default
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
                        .append_pair(azure_core::query_param::API_VERSION, "2018-05-01");
                    let req_body = azure_core::EMPTY_BODY;
                    req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                    req.set_body(req_body);
                    let rsp = this.client.send(&mut req).await?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        azure_core::StatusCode::Ok => {
                            let rsp_body = rsp_stream.collect().await?;
                            let rsp_value: models::EnableSerialConsoleResult = serde_json::from_slice(&rsp_body)?;
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
pub mod serial_ports {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Lists all of the configured serial ports for a parent resource "]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `resource_provider_namespace`: The namespace of the resource provider."]
        #[doc = "* `parent_resource_type`: The resource type of the parent resource.  For example: 'virtualMachines' or 'virtualMachineScaleSets'"]
        #[doc = "* `parent_resource`: The resource name, or subordinate path, for the parent of the serial port. For example: the name of the virtual machine."]
        #[doc = "* `subscription_id`: Subscription ID which uniquely identifies the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call requiring it."]
        pub fn list(
            &self,
            resource_group_name: impl Into<String>,
            resource_provider_namespace: impl Into<String>,
            parent_resource_type: impl Into<String>,
            parent_resource: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> list::Builder {
            list::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                resource_provider_namespace: resource_provider_namespace.into(),
                parent_resource_type: parent_resource_type.into(),
                parent_resource: parent_resource.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Gets the configured settings for a serial port"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `resource_provider_namespace`: The namespace of the resource provider."]
        #[doc = "* `parent_resource_type`: The resource type of the parent resource.  For example: 'virtualMachines' or 'virtualMachineScaleSets'"]
        #[doc = "* `parent_resource`: The resource name, or subordinate path, for the parent of the serial port. For example: the name of the virtual machine."]
        #[doc = "* `serial_port`: The name of the serial port to connect to."]
        #[doc = "* `subscription_id`: Subscription ID which uniquely identifies the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call requiring it."]
        pub fn get(
            &self,
            resource_group_name: impl Into<String>,
            resource_provider_namespace: impl Into<String>,
            parent_resource_type: impl Into<String>,
            parent_resource: impl Into<String>,
            serial_port: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> get::Builder {
            get::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                resource_provider_namespace: resource_provider_namespace.into(),
                parent_resource_type: parent_resource_type.into(),
                parent_resource: parent_resource.into(),
                serial_port: serial_port.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Creates or updates a serial port"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `resource_provider_namespace`: The namespace of the resource provider."]
        #[doc = "* `parent_resource_type`: The resource type of the parent resource.  For example: 'virtualMachines' or 'virtualMachineScaleSets'"]
        #[doc = "* `parent_resource`: The resource name, or subordinate path, for the parent of the serial port. For example: the name of the virtual machine."]
        #[doc = "* `serial_port`: The name of the serial port to create."]
        #[doc = "* `parameters`: Parameters supplied to create the serial port."]
        #[doc = "* `subscription_id`: Subscription ID which uniquely identifies the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call requiring it."]
        pub fn create(
            &self,
            resource_group_name: impl Into<String>,
            resource_provider_namespace: impl Into<String>,
            parent_resource_type: impl Into<String>,
            parent_resource: impl Into<String>,
            serial_port: impl Into<String>,
            parameters: impl Into<models::SerialPort>,
            subscription_id: impl Into<String>,
        ) -> create::Builder {
            create::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                resource_provider_namespace: resource_provider_namespace.into(),
                parent_resource_type: parent_resource_type.into(),
                parent_resource: parent_resource.into(),
                serial_port: serial_port.into(),
                parameters: parameters.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Deletes a serial port"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `resource_provider_namespace`: The namespace of the resource provider."]
        #[doc = "* `parent_resource_type`: The resource type of the parent resource.  For example: 'virtualMachines' or 'virtualMachineScaleSets'"]
        #[doc = "* `parent_resource`: The resource name, or subordinate path, for the parent of the serial port. For example: the name of the virtual machine."]
        #[doc = "* `serial_port`: The name of the serial port to delete."]
        #[doc = "* `subscription_id`: Subscription ID which uniquely identifies the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call requiring it."]
        pub fn delete(
            &self,
            resource_group_name: impl Into<String>,
            resource_provider_namespace: impl Into<String>,
            parent_resource_type: impl Into<String>,
            parent_resource: impl Into<String>,
            serial_port: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> delete::Builder {
            delete::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                resource_provider_namespace: resource_provider_namespace.into(),
                parent_resource_type: parent_resource_type.into(),
                parent_resource: parent_resource.into(),
                serial_port: serial_port.into(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Handles requests to list all SerialPort resources in a subscription."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `subscription_id`: Subscription ID which uniquely identifies the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call requiring it."]
        pub fn list_by_subscriptions(&self, subscription_id: impl Into<String>) -> list_by_subscriptions::Builder {
            list_by_subscriptions::Builder {
                client: self.0.clone(),
                subscription_id: subscription_id.into(),
            }
        }
        #[doc = "Connect to serial port of the target resource"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `resource_group_name`: The name of the resource group."]
        #[doc = "* `resource_provider_namespace`: The namespace of the resource provider."]
        #[doc = "* `parent_resource_type`: The resource type of the parent resource.  For example: 'virtualMachines' or 'virtualMachineScaleSets'"]
        #[doc = "* `parent_resource`: The resource name, or subordinate path, for the parent of the serial port. For example: the name of the virtual machine."]
        #[doc = "* `serial_port`: The name of the serial port to connect to."]
        #[doc = "* `subscription_id`: Subscription ID which uniquely identifies the Microsoft Azure subscription. The subscription ID forms part of the URI for every service call requiring it."]
        pub fn connect(
            &self,
            resource_group_name: impl Into<String>,
            resource_provider_namespace: impl Into<String>,
            parent_resource_type: impl Into<String>,
            parent_resource: impl Into<String>,
            serial_port: impl Into<String>,
            subscription_id: impl Into<String>,
        ) -> connect::Builder {
            connect::Builder {
                client: self.0.clone(),
                resource_group_name: resource_group_name.into(),
                resource_provider_namespace: resource_provider_namespace.into(),
                parent_resource_type: parent_resource_type.into(),
                parent_resource: parent_resource.into(),
                serial_port: serial_port.into(),
                subscription_id: subscription_id.into(),
            }
        }
    }
    pub mod list {
        use super::models;
        type Response = models::SerialPortListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) resource_provider_namespace: String,
            pub(crate) parent_resource_type: String,
            pub(crate) parent_resource: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourcegroups/{}/providers/{}/{}/{}/providers/Microsoft.SerialConsole/serialPorts",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.resource_provider_namespace,
                            &this.parent_resource_type,
                            &this.parent_resource
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SerialPortListResult = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::SerialPort;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) resource_provider_namespace: String,
            pub(crate) parent_resource_type: String,
            pub(crate) parent_resource: String,
            pub(crate) serial_port: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourcegroups/{}/providers/{}/{}/{}/providers/Microsoft.SerialConsole/serialPorts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.resource_provider_namespace,
                            &this.parent_resource_type,
                            &this.parent_resource,
                            &this.serial_port
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SerialPort = serde_json::from_slice(&rsp_body)?;
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
        type Response = models::SerialPort;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) resource_provider_namespace: String,
            pub(crate) parent_resource_type: String,
            pub(crate) parent_resource: String,
            pub(crate) serial_port: String,
            pub(crate) parameters: models::SerialPort,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourcegroups/{}/providers/{}/{}/{}/providers/Microsoft.SerialConsole/serialPorts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.resource_provider_namespace,
                            &this.parent_resource_type,
                            &this.parent_resource,
                            &this.serial_port
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-05-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.parameters)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Created => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SerialPort = serde_json::from_slice(&rsp_body)?;
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
            pub(crate) resource_group_name: String,
            pub(crate) resource_provider_namespace: String,
            pub(crate) parent_resource_type: String,
            pub(crate) parent_resource: String,
            pub(crate) serial_port: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/resourcegroups/{}/providers/{}/{}/{}/providers/Microsoft.SerialConsole/serialPorts/{}",
                            this.client.endpoint(),
                            &this.subscription_id,
                            &this.resource_group_name,
                            &this.resource_provider_namespace,
                            &this.parent_resource_type,
                            &this.parent_resource,
                            &this.serial_port
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
                            .append_pair(azure_core::query_param::API_VERSION, "2018-05-01");
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
    pub mod list_by_subscriptions {
        use super::models;
        type Response = models::SerialPortListResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/subscriptions/{}/providers/Microsoft.SerialConsole/serialPorts",
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
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SerialPortListResult = serde_json::from_slice(&rsp_body)?;
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
    pub mod connect {
        use super::models;
        type Response = models::SerialPortConnectResult;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) resource_group_name: String,
            pub(crate) resource_provider_namespace: String,
            pub(crate) parent_resource_type: String,
            pub(crate) parent_resource: String,
            pub(crate) serial_port: String,
            pub(crate) subscription_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core :: Url :: parse (& format ! ("{}/subscriptions/{}/resourcegroups/{}/providers/{}/{}/{}/providers/Microsoft.SerialConsole/serialPorts/{}/connect" , this . client . endpoint () , & this . subscription_id , & this . resource_group_name , & this . resource_provider_namespace , & this . parent_resource_type , & this . parent_resource , & this . serial_port)) ? ;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2018-05-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::SerialPortConnectResult = serde_json::from_slice(&rsp_body)?;
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
