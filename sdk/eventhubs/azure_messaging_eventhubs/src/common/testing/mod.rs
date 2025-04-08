// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use async_trait::async_trait;
use azure_core::{http::Url, Result};
use azure_core_amqp::{
    AmqpClaimsBasedSecurity, AmqpClaimsBasedSecurityApis, AmqpConnection, AmqpConnectionApis,
    AmqpConnectionOptions, AmqpSendOptions, AmqpSession, AmqpSessionApis, AmqpSessionOptions,
};
use std::sync::{Arc, RwLock, Weak};
use time::OffsetDateTime;

/// Mock implementation of AmqpConnection for testing
#[derive(Debug, Default)]
pub struct MockAmqpConnection {
    connection_id: RwLock<String>,
    url: RwLock<Option<Url>>,
    open_called: RwLock<bool>,
    close_called: RwLock<bool>,
    should_fail: RwLock<bool>,
    error_message: RwLock<Option<String>>,
}

impl MockAmqpConnection {
    pub fn new() -> Self {
        Self {
            connection_id: RwLock::new(String::new()),
            url: RwLock::new(None),
            open_called: RwLock::new(false),
            close_called: RwLock::new(false),
            should_fail: RwLock::new(false),
            error_message: RwLock::new(None),
        }
    }

    pub fn with_error(mut self, error_message: &str) -> Self {
        *self.should_fail.write().unwrap() = true;
        *self.error_message.write().unwrap() = Some(error_message.to_string());
        self
    }

    pub fn is_open_called(&self) -> bool {
        *self.open_called.read().unwrap()
    }

    pub fn is_close_called(&self) -> bool {
        *self.close_called.read().unwrap()
    }

    pub fn get_connection_id(&self) -> String {
        self.connection_id.read().unwrap().clone()
    }

    pub fn get_url(&self) -> Option<Url> {
        self.url.read().unwrap().clone()
    }
}

impl AmqpConnectionApis for MockAmqpConnection {
    async fn open(
        &self,
        connection_id: String,
        url: Url,
        _options: Option<AmqpConnectionOptions>,
    ) -> Result<()> {
        *self.open_called.write().unwrap() = true;
        *self.connection_id.write().unwrap() = connection_id;
        *self.url.write().unwrap() = Some(url);

        if *self.should_fail.read().unwrap() {
            return Err(azure_core::Error::message(
                azure_core::error::ErrorKind::Other,
                self.error_message
                    .read()
                    .unwrap()
                    .clone()
                    .unwrap_or_else(|| "Mock connection error".to_string()),
            ));
        }

        Ok(())
    }

    async fn close(&self) -> Result<()> {
        *self.close_called.write().unwrap() = true;

        if *self.should_fail.read().unwrap() {
            return Err(azure_core::Error::message(
                azure_core::error::ErrorKind::Other,
                self.error_message
                    .read()
                    .unwrap()
                    .clone()
                    .unwrap_or_else(|| "Mock connection close error".to_string()),
            ));
        }

        Ok(())
    }

    async fn close_with_error(
        &self,
        condition: azure_core_amqp::AmqpSymbol,
        description: Option<String>,
        info: Option<
            azure_core_amqp::AmqpOrderedMap<
                azure_core_amqp::AmqpSymbol,
                azure_core_amqp::AmqpValue,
            >,
        >,
    ) -> Result<()> {
        todo!()
    }
}

/// Mock implementation of AmqpSession for testing
#[derive(Debug, Default)]
pub struct MockAmqpSession {
    begin_called: RwLock<bool>,
    end_called: RwLock<bool>,
    should_fail: RwLock<bool>,
    error_message: RwLock<Option<String>>,
}

impl MockAmqpSession {
    pub fn new() -> Self {
        Self {
            begin_called: RwLock::new(false),
            end_called: RwLock::new(false),
            should_fail: RwLock::new(false),
            error_message: RwLock::new(None),
        }
    }

    pub fn with_error(mut self, error_message: &str) -> Self {
        *self.should_fail.write().unwrap() = true;
        *self.error_message.write().unwrap() = Some(error_message.to_string());
        self
    }

    pub fn is_begin_called(&self) -> bool {
        *self.begin_called.read().unwrap()
    }

    pub fn is_end_called(&self) -> bool {
        *self.end_called.read().unwrap()
    }
}

impl AmqpSessionApis for MockAmqpSession {
    async fn begin(
        &self,
        _connection: &AmqpConnection,
        _options: Option<AmqpSessionOptions>,
    ) -> Result<()> {
        *self.begin_called.write().unwrap() = true;

        if *self.should_fail.read().unwrap() {
            return Err(azure_core::Error::message(
                azure_core::error::ErrorKind::Other,
                self.error_message
                    .read()
                    .unwrap()
                    .clone()
                    .unwrap_or_else(|| "Mock session error".to_string()),
            ));
        }

        Ok(())
    }

    async fn end(&self) -> Result<()> {
        *self.end_called.write().unwrap() = true;

        if *self.should_fail.read().unwrap() {
            return Err(azure_core::Error::message(
                azure_core::error::ErrorKind::Other,
                self.error_message
                    .read()
                    .unwrap()
                    .clone()
                    .unwrap_or_else(|| "Mock session close error".to_string()),
            ));
        }

        Ok(())
    }
}

/// Mock implementation of AmqpClaimsBasedSecurity for testing
#[derive(Debug)]
pub struct MockAmqpClaimsBasedSecurity {
    attach_called: RwLock<bool>,
    authorize_path_called: RwLock<bool>,
    paths_authorized: RwLock<Vec<String>>,
    should_fail: RwLock<bool>,
    error_message: RwLock<Option<String>>,
}

impl MockAmqpClaimsBasedSecurity {
    pub fn new(_session: &MockAmqpSession) -> Result<Self> {
        Ok(Self {
            attach_called: RwLock::new(false),
            authorize_path_called: RwLock::new(false),
            paths_authorized: RwLock::new(Vec::new()),
            should_fail: RwLock::new(false),
            error_message: RwLock::new(None),
        })
    }

    pub fn with_error(mut self, error_message: &str) -> Self {
        *self.should_fail.write().unwrap() = true;
        *self.error_message.write().unwrap() = Some(error_message.to_string());
        self
    }

    pub fn is_attach_called(&self) -> bool {
        *self.attach_called.read().unwrap()
    }

    pub fn is_authorize_path_called(&self) -> bool {
        *self.authorize_path_called.read().unwrap()
    }

    pub fn get_authorized_paths(&self) -> Vec<String> {
        self.paths_authorized.read().unwrap().clone()
    }
}

impl AmqpClaimsBasedSecurityApis for MockAmqpClaimsBasedSecurity {
    async fn attach(&self) -> Result<()> {
        *self.attach_called.write().unwrap() = true;

        if *self.should_fail.read().unwrap() {
            return Err(azure_core::Error::message(
                azure_core::error::ErrorKind::Other,
                self.error_message
                    .read()
                    .unwrap()
                    .clone()
                    .unwrap_or_else(|| "Mock CBS attach error".to_string()),
            ));
        }

        Ok(())
    }

    async fn authorize_path(
        &self,
        path: String,
        _auth_type: Option<String>,
        _token: String,
        _expiration: OffsetDateTime,
    ) -> Result<()> {
        *self.authorize_path_called.write().unwrap() = true;
        self.paths_authorized.write().unwrap().push(path);

        if *self.should_fail.read().unwrap() {
            return Err(azure_core::Error::message(
                azure_core::error::ErrorKind::Other,
                self.error_message
                    .read()
                    .unwrap()
                    .clone()
                    .unwrap_or_else(|| "Mock CBS authorize_path error".to_string()),
            ));
        }

        Ok(())
    }

    async fn detach(self) -> Result<()> {
        todo!()
    }
}
