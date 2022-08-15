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
    pub fn health_api_client(&self) -> health_api::Client {
        health_api::Client(self.clone())
    }
    pub fn web_pub_sub_client(&self) -> web_pub_sub::Client {
        web_pub_sub::Client(self.clone())
    }
}
pub mod health_api {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Get service health status."]
        pub fn get_service_status(&self) -> get_service_status::Builder {
            get_service_status::Builder { client: self.0.clone() }
        }
    }
    pub mod get_service_status {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/api/health", this.client.endpoint(),))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Head);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
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
pub mod web_pub_sub {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "Generate token for the client to connect Azure Web PubSub service."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `hub`: Target hub name, which should start with alphabetic characters and only contain alpha-numeric characters or underscore."]
        pub fn generate_client_token(&self, hub: impl Into<String>) -> generate_client_token::Builder {
            generate_client_token::Builder {
                client: self.0.clone(),
                hub: hub.into(),
                user_id: None,
                role: Vec::new(),
                minutes_to_expire: None,
            }
        }
        #[doc = "Close the connections in the hub."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `hub`: Target hub name, which should start with alphabetic characters and only contain alpha-numeric characters or underscore."]
        pub fn close_all_connections(&self, hub: impl Into<String>) -> close_all_connections::Builder {
            close_all_connections::Builder {
                client: self.0.clone(),
                hub: hub.into(),
                excluded: Vec::new(),
                reason: None,
            }
        }
        #[doc = "Broadcast content inside request body to all the connected client connections."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `hub`: Target hub name, which should start with alphabetic characters and only contain alpha-numeric characters or underscore."]
        #[doc = "* `message`: The payload body."]
        pub fn send_to_all(&self, hub: impl Into<String>, message: impl Into<String>) -> send_to_all::Builder {
            send_to_all::Builder {
                client: self.0.clone(),
                hub: hub.into(),
                message: message.into(),
                excluded: Vec::new(),
            }
        }
        #[doc = "Close the client connection."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `hub`: Target hub name, which should start with alphabetic characters and only contain alpha-numeric characters or underscore."]
        #[doc = "* `connection_id`: Target connection Id."]
        pub fn close_connection(&self, hub: impl Into<String>, connection_id: impl Into<String>) -> close_connection::Builder {
            close_connection::Builder {
                client: self.0.clone(),
                hub: hub.into(),
                connection_id: connection_id.into(),
                reason: None,
            }
        }
        #[doc = "Check if the connection with the given connectionId exists."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `hub`: Target hub name, which should start with alphabetic characters and only contain alpha-numeric characters or underscore."]
        #[doc = "* `connection_id`: The connection Id."]
        pub fn connection_exists(&self, hub: impl Into<String>, connection_id: impl Into<String>) -> connection_exists::Builder {
            connection_exists::Builder {
                client: self.0.clone(),
                hub: hub.into(),
                connection_id: connection_id.into(),
            }
        }
        #[doc = "Send content inside request body to the specific connection."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `hub`: Target hub name, which should start with alphabetic characters and only contain alpha-numeric characters or underscore."]
        #[doc = "* `connection_id`: The connection Id."]
        #[doc = "* `message`: The payload body."]
        pub fn send_to_connection(
            &self,
            hub: impl Into<String>,
            connection_id: impl Into<String>,
            message: impl Into<String>,
        ) -> send_to_connection::Builder {
            send_to_connection::Builder {
                client: self.0.clone(),
                hub: hub.into(),
                connection_id: connection_id.into(),
                message: message.into(),
            }
        }
        #[doc = "Check if there are any client connections inside the given group"]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `hub`: Target hub name, which should start with alphabetic characters and only contain alpha-numeric characters or underscore."]
        #[doc = "* `group`: Target group name, which length should be greater than 0 and less than 1025."]
        pub fn group_exists(&self, hub: impl Into<String>, group: impl Into<String>) -> group_exists::Builder {
            group_exists::Builder {
                client: self.0.clone(),
                hub: hub.into(),
                group: group.into(),
            }
        }
        #[doc = "Close connections in the specific group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `hub`: Target hub name, which should start with alphabetic characters and only contain alpha-numeric characters or underscore."]
        #[doc = "* `group`: Target group name, which length should be greater than 0 and less than 1025."]
        pub fn close_group_connections(&self, hub: impl Into<String>, group: impl Into<String>) -> close_group_connections::Builder {
            close_group_connections::Builder {
                client: self.0.clone(),
                hub: hub.into(),
                group: group.into(),
                excluded: Vec::new(),
                reason: None,
            }
        }
        #[doc = "Send content inside request body to a group of connections."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `hub`: Target hub name, which should start with alphabetic characters and only contain alpha-numeric characters or underscore."]
        #[doc = "* `group`: Target group name, which length should be greater than 0 and less than 1025."]
        #[doc = "* `message`: The payload body."]
        pub fn send_to_group(
            &self,
            hub: impl Into<String>,
            group: impl Into<String>,
            message: impl Into<String>,
        ) -> send_to_group::Builder {
            send_to_group::Builder {
                client: self.0.clone(),
                hub: hub.into(),
                group: group.into(),
                message: message.into(),
                excluded: Vec::new(),
            }
        }
        #[doc = "Add a connection to the target group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `hub`: Target hub name, which should start with alphabetic characters and only contain alpha-numeric characters or underscore."]
        #[doc = "* `group`: Target group name, which length should be greater than 0 and less than 1025."]
        #[doc = "* `connection_id`: Target connection Id"]
        pub fn add_connection_to_group(
            &self,
            hub: impl Into<String>,
            group: impl Into<String>,
            connection_id: impl Into<String>,
        ) -> add_connection_to_group::Builder {
            add_connection_to_group::Builder {
                client: self.0.clone(),
                hub: hub.into(),
                group: group.into(),
                connection_id: connection_id.into(),
            }
        }
        #[doc = "Remove a connection from the target group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `hub`: Target hub name, which should start with alphabetic characters and only contain alpha-numeric characters or underscore."]
        #[doc = "* `group`: Target group name, which length should be greater than 0 and less than 1025."]
        #[doc = "* `connection_id`: Target connection Id."]
        pub fn remove_connection_from_group(
            &self,
            hub: impl Into<String>,
            group: impl Into<String>,
            connection_id: impl Into<String>,
        ) -> remove_connection_from_group::Builder {
            remove_connection_from_group::Builder {
                client: self.0.clone(),
                hub: hub.into(),
                group: group.into(),
                connection_id: connection_id.into(),
            }
        }
        #[doc = "Check if there are any client connections connected for the given user."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `hub`: Target hub name, which should start with alphabetic characters and only contain alpha-numeric characters or underscore."]
        #[doc = "* `user_id`: Target user Id."]
        pub fn user_exists(&self, hub: impl Into<String>, user_id: impl Into<String>) -> user_exists::Builder {
            user_exists::Builder {
                client: self.0.clone(),
                hub: hub.into(),
                user_id: user_id.into(),
            }
        }
        #[doc = "Close connections for the specific user."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `hub`: Target hub name, which should start with alphabetic characters and only contain alpha-numeric characters or underscore."]
        #[doc = "* `user_id`: The user Id."]
        pub fn close_user_connections(&self, hub: impl Into<String>, user_id: impl Into<String>) -> close_user_connections::Builder {
            close_user_connections::Builder {
                client: self.0.clone(),
                hub: hub.into(),
                user_id: user_id.into(),
                excluded: Vec::new(),
                reason: None,
            }
        }
        #[doc = "Send content inside request body to the specific user."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `hub`: Target hub name, which should start with alphabetic characters and only contain alpha-numeric characters or underscore."]
        #[doc = "* `user_id`: The user Id."]
        #[doc = "* `message`: The payload body."]
        pub fn send_to_user(
            &self,
            hub: impl Into<String>,
            user_id: impl Into<String>,
            message: impl Into<String>,
        ) -> send_to_user::Builder {
            send_to_user::Builder {
                client: self.0.clone(),
                hub: hub.into(),
                user_id: user_id.into(),
                message: message.into(),
            }
        }
        #[doc = "Add a user to the target group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `hub`: Target hub name, which should start with alphabetic characters and only contain alpha-numeric characters or underscore."]
        #[doc = "* `group`: Target group name, which length should be greater than 0 and less than 1025."]
        #[doc = "* `user_id`: Target user Id."]
        pub fn add_user_to_group(
            &self,
            hub: impl Into<String>,
            group: impl Into<String>,
            user_id: impl Into<String>,
        ) -> add_user_to_group::Builder {
            add_user_to_group::Builder {
                client: self.0.clone(),
                hub: hub.into(),
                group: group.into(),
                user_id: user_id.into(),
            }
        }
        #[doc = "Remove a user from the target group."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `hub`: Target hub name, which should start with alphabetic characters and only contain alpha-numeric characters or underscore."]
        #[doc = "* `group`: Target group name, which length should be greater than 0 and less than 1025."]
        #[doc = "* `user_id`: Target user Id."]
        pub fn remove_user_from_group(
            &self,
            hub: impl Into<String>,
            group: impl Into<String>,
            user_id: impl Into<String>,
        ) -> remove_user_from_group::Builder {
            remove_user_from_group::Builder {
                client: self.0.clone(),
                hub: hub.into(),
                group: group.into(),
                user_id: user_id.into(),
            }
        }
        #[doc = "Remove a user from all groups."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `hub`: Target hub name, which should start with alphabetic characters and only contain alpha-numeric characters or underscore."]
        #[doc = "* `user_id`: Target user Id."]
        pub fn remove_user_from_all_groups(
            &self,
            hub: impl Into<String>,
            user_id: impl Into<String>,
        ) -> remove_user_from_all_groups::Builder {
            remove_user_from_all_groups::Builder {
                client: self.0.clone(),
                hub: hub.into(),
                user_id: user_id.into(),
            }
        }
        #[doc = "Grant permission to the connection."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `hub`: Target hub name, which should start with alphabetic characters and only contain alpha-numeric characters or underscore."]
        #[doc = "* `permission`: The permission: current supported actions are joinLeaveGroup and sendToGroup."]
        #[doc = "* `connection_id`: Target connection Id."]
        pub fn grant_permission(
            &self,
            hub: impl Into<String>,
            permission: impl Into<String>,
            connection_id: impl Into<String>,
        ) -> grant_permission::Builder {
            grant_permission::Builder {
                client: self.0.clone(),
                hub: hub.into(),
                permission: permission.into(),
                connection_id: connection_id.into(),
                target_name: None,
            }
        }
        #[doc = "Revoke permission for the connection."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `hub`: Target hub name, which should start with alphabetic characters and only contain alpha-numeric characters or underscore."]
        #[doc = "* `permission`: The permission: current supported actions are joinLeaveGroup and sendToGroup."]
        #[doc = "* `connection_id`: Target connection Id."]
        pub fn revoke_permission(
            &self,
            hub: impl Into<String>,
            permission: impl Into<String>,
            connection_id: impl Into<String>,
        ) -> revoke_permission::Builder {
            revoke_permission::Builder {
                client: self.0.clone(),
                hub: hub.into(),
                permission: permission.into(),
                connection_id: connection_id.into(),
                target_name: None,
            }
        }
        #[doc = "Check if a connection has permission to the specified action."]
        #[doc = ""]
        #[doc = "Arguments:"]
        #[doc = "* `hub`: Target hub name, which should start with alphabetic characters and only contain alpha-numeric characters or underscore."]
        #[doc = "* `permission`: The permission: current supported actions are joinLeaveGroup and sendToGroup."]
        #[doc = "* `connection_id`: Target connection Id."]
        pub fn check_permission(
            &self,
            hub: impl Into<String>,
            permission: impl Into<String>,
            connection_id: impl Into<String>,
        ) -> check_permission::Builder {
            check_permission::Builder {
                client: self.0.clone(),
                hub: hub.into(),
                permission: permission.into(),
                connection_id: connection_id.into(),
                target_name: None,
            }
        }
    }
    pub mod generate_client_token {
        use super::models;
        type Response = models::ClientTokenResponse;
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) hub: String,
            pub(crate) user_id: Option<String>,
            pub(crate) role: Vec<String>,
            pub(crate) minutes_to_expire: Option<i32>,
        }
        impl Builder {
            #[doc = "User Id."]
            pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
                self.user_id = Some(user_id.into());
                self
            }
            #[doc = "Roles that the connection with the generated token will have."]
            pub fn role(mut self, role: Vec<String>) -> Self {
                self.role = role;
                self
            }
            #[doc = "The expire time of the generated token."]
            pub fn minutes_to_expire(mut self, minutes_to_expire: i32) -> Self {
                self.minutes_to_expire = Some(minutes_to_expire);
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/api/hubs/{}/:generateToken", this.client.endpoint(), &this.hub))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        if let Some(user_id) = &this.user_id {
                            req.url_mut().query_pairs_mut().append_pair("userId", user_id);
                        }
                        let role = &this.role;
                        for value in &this.role {
                            req.url_mut().query_pairs_mut().append_pair("role", &value.to_string());
                        }
                        if let Some(minutes_to_expire) = &this.minutes_to_expire {
                            req.url_mut()
                                .query_pairs_mut()
                                .append_pair("minutesToExpire", &minutes_to_expire.to_string());
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Ok => {
                                let rsp_body = rsp_stream.collect().await?;
                                let rsp_value: models::ClientTokenResponse = serde_json::from_slice(&rsp_body)?;
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
    pub mod close_all_connections {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) hub: String,
            pub(crate) excluded: Vec<String>,
            pub(crate) reason: Option<String>,
        }
        impl Builder {
            #[doc = "Exclude these connectionIds when closing the connections in the hub."]
            pub fn excluded(mut self, excluded: Vec<String>) -> Self {
                self.excluded = excluded;
                self
            }
            #[doc = "The reason closing the client connection."]
            pub fn reason(mut self, reason: impl Into<String>) -> Self {
                self.reason = Some(reason.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/api/hubs/{}/:closeConnections", this.client.endpoint(), &this.hub))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let excluded = &this.excluded;
                        for value in &this.excluded {
                            req.url_mut().query_pairs_mut().append_pair("excluded", &value.to_string());
                        }
                        if let Some(reason) = &this.reason {
                            req.url_mut().query_pairs_mut().append_pair("reason", reason);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod send_to_all {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) hub: String,
            pub(crate) message: String,
            pub(crate) excluded: Vec<String>,
        }
        impl Builder {
            #[doc = "Excluded connection Ids."]
            pub fn excluded(mut self, excluded: Vec<String>) -> Self {
                self.excluded = excluded;
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!("{}/api/hubs/{}/:send", this.client.endpoint(), &this.hub))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Post);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let excluded = &this.excluded;
                        for value in &this.excluded {
                            req.url_mut().query_pairs_mut().append_pair("excluded", &value.to_string());
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.message)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Accepted => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod close_connection {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) hub: String,
            pub(crate) connection_id: String,
            pub(crate) reason: Option<String>,
        }
        impl Builder {
            #[doc = "The reason closing the client connection."]
            pub fn reason(mut self, reason: impl Into<String>) -> Self {
                self.reason = Some(reason.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/api/hubs/{}/connections/{}",
                            this.client.endpoint(),
                            &this.hub,
                            &this.connection_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        if let Some(reason) = &this.reason {
                            req.url_mut().query_pairs_mut().append_pair("reason", reason);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod connection_exists {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) hub: String,
            pub(crate) connection_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/api/hubs/{}/connections/{}",
                            this.client.endpoint(),
                            &this.hub,
                            &this.connection_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Head);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
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
    pub mod send_to_connection {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) hub: String,
            pub(crate) connection_id: String,
            pub(crate) message: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/api/hubs/{}/connections/{}/:send",
                            this.client.endpoint(),
                            &this.hub,
                            &this.connection_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.message)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Accepted => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod group_exists {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) hub: String,
            pub(crate) group: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url =
                            azure_core::Url::parse(&format!("{}/api/hubs/{}/groups/{}", this.client.endpoint(), &this.hub, &this.group))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Head);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
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
    pub mod close_group_connections {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) hub: String,
            pub(crate) group: String,
            pub(crate) excluded: Vec<String>,
            pub(crate) reason: Option<String>,
        }
        impl Builder {
            #[doc = "Exclude these connectionIds when closing the connections in the group."]
            pub fn excluded(mut self, excluded: Vec<String>) -> Self {
                self.excluded = excluded;
                self
            }
            #[doc = "The reason closing the client connection."]
            pub fn reason(mut self, reason: impl Into<String>) -> Self {
                self.reason = Some(reason.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/api/hubs/{}/groups/{}/:closeConnections",
                            this.client.endpoint(),
                            &this.hub,
                            &this.group
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let excluded = &this.excluded;
                        for value in &this.excluded {
                            req.url_mut().query_pairs_mut().append_pair("excluded", &value.to_string());
                        }
                        if let Some(reason) = &this.reason {
                            req.url_mut().query_pairs_mut().append_pair("reason", reason);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod send_to_group {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) hub: String,
            pub(crate) group: String,
            pub(crate) message: String,
            pub(crate) excluded: Vec<String>,
        }
        impl Builder {
            #[doc = "Excluded connection Ids"]
            pub fn excluded(mut self, excluded: Vec<String>) -> Self {
                self.excluded = excluded;
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/api/hubs/{}/groups/{}/:send",
                            this.client.endpoint(),
                            &this.hub,
                            &this.group
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let excluded = &this.excluded;
                        for value in &this.excluded {
                            req.url_mut().query_pairs_mut().append_pair("excluded", &value.to_string());
                        }
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.message)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Accepted => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod add_connection_to_group {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) hub: String,
            pub(crate) group: String,
            pub(crate) connection_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/api/hubs/{}/groups/{}/connections/{}",
                            this.client.endpoint(),
                            &this.hub,
                            &this.group,
                            &this.connection_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
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
    pub mod remove_connection_from_group {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) hub: String,
            pub(crate) group: String,
            pub(crate) connection_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/api/hubs/{}/groups/{}/connections/{}",
                            this.client.endpoint(),
                            &this.hub,
                            &this.group,
                            &this.connection_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod user_exists {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) hub: String,
            pub(crate) user_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/api/hubs/{}/users/{}",
                            this.client.endpoint(),
                            &this.hub,
                            &this.user_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Head);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
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
    pub mod close_user_connections {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) hub: String,
            pub(crate) user_id: String,
            pub(crate) excluded: Vec<String>,
            pub(crate) reason: Option<String>,
        }
        impl Builder {
            #[doc = "Exclude these connectionIds when closing the connections for the user."]
            pub fn excluded(mut self, excluded: Vec<String>) -> Self {
                self.excluded = excluded;
                self
            }
            #[doc = "The reason closing the client connection."]
            pub fn reason(mut self, reason: impl Into<String>) -> Self {
                self.reason = Some(reason.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/api/hubs/{}/users/{}/:closeConnections",
                            this.client.endpoint(),
                            &this.hub,
                            &this.user_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let excluded = &this.excluded;
                        for value in &this.excluded {
                            req.url_mut().query_pairs_mut().append_pair("excluded", &value.to_string());
                        }
                        if let Some(reason) = &this.reason {
                            req.url_mut().query_pairs_mut().append_pair("reason", reason);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod send_to_user {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) hub: String,
            pub(crate) user_id: String,
            pub(crate) message: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/api/hubs/{}/users/{}/:send",
                            this.client.endpoint(),
                            &this.hub,
                            &this.user_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        req.insert_header("content-type", "application/json");
                        let req_body = azure_core::to_json(&this.message)?;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::Accepted => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod add_user_to_group {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) hub: String,
            pub(crate) group: String,
            pub(crate) user_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/api/hubs/{}/users/{}/groups/{}",
                            this.client.endpoint(),
                            &this.hub,
                            &this.user_id,
                            &this.group
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
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
    pub mod remove_user_from_group {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) hub: String,
            pub(crate) group: String,
            pub(crate) user_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/api/hubs/{}/users/{}/groups/{}",
                            this.client.endpoint(),
                            &this.hub,
                            &this.user_id,
                            &this.group
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod remove_user_from_all_groups {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) hub: String,
            pub(crate) user_id: String,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/api/hubs/{}/users/{}/groups",
                            this.client.endpoint(),
                            &this.hub,
                            &this.user_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod grant_permission {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) hub: String,
            pub(crate) permission: String,
            pub(crate) connection_id: String,
            pub(crate) target_name: Option<String>,
        }
        impl Builder {
            #[doc = "The meaning of the target depends on the specific permission. For joinLeaveGroup and sendToGroup, targetName is a required parameter standing for the group name."]
            pub fn target_name(mut self, target_name: impl Into<String>) -> Self {
                self.target_name = Some(target_name.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/api/hubs/{}/permissions/{}/connections/{}",
                            this.client.endpoint(),
                            &this.hub,
                            &this.permission,
                            &this.connection_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        if let Some(target_name) = &this.target_name {
                            req.url_mut().query_pairs_mut().append_pair("targetName", target_name);
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
    pub mod revoke_permission {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) hub: String,
            pub(crate) permission: String,
            pub(crate) connection_id: String,
            pub(crate) target_name: Option<String>,
        }
        impl Builder {
            #[doc = "The meaning of the target depends on the specific permission. For joinLeaveGroup and sendToGroup, targetName is a required parameter standing for the group name."]
            pub fn target_name(mut self, target_name: impl Into<String>) -> Self {
                self.target_name = Some(target_name.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/api/hubs/{}/permissions/{}/connections/{}",
                            this.client.endpoint(),
                            &this.hub,
                            &this.permission,
                            &this.connection_id
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
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        if let Some(target_name) = &this.target_name {
                            req.url_mut().query_pairs_mut().append_pair("targetName", target_name);
                        }
                        let req_body = azure_core::EMPTY_BODY;
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            azure_core::StatusCode::NoContent => Ok(()),
                            status_code => Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse {
                                status: status_code,
                                error_code: None,
                            })),
                        }
                    }
                })
            }
        }
    }
    pub mod check_permission {
        use super::models;
        type Response = ();
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
            pub(crate) hub: String,
            pub(crate) permission: String,
            pub(crate) connection_id: String,
            pub(crate) target_name: Option<String>,
        }
        impl Builder {
            #[doc = "The meaning of the target depends on the specific permission. For joinLeaveGroup and sendToGroup, targetName is a required parameter standing for the group name."]
            pub fn target_name(mut self, target_name: impl Into<String>) -> Self {
                self.target_name = Some(target_name.into());
                self
            }
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(
                            "{}/api/hubs/{}/permissions/{}/connections/{}",
                            this.client.endpoint(),
                            &this.hub,
                            &this.permission,
                            &this.connection_id
                        ))?;
                        let mut req = azure_core::Request::new(url, azure_core::Method::Head);
                        let credential = this.client.token_credential();
                        let token_response = credential.get_token(&this.client.scopes().join(" ")).await?;
                        req.insert_header(
                            azure_core::headers::AUTHORIZATION,
                            format!("Bearer {}", token_response.token.secret()),
                        );
                        req.url_mut()
                            .query_pairs_mut()
                            .append_pair(azure_core::query_param::API_VERSION, "2021-10-01");
                        if let Some(target_name) = &this.target_name {
                            req.url_mut().query_pairs_mut().append_pair("targetName", target_name);
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
