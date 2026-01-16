// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a service identity with federation information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServiceIdentity {
    #[serde(rename = "FederationId")]
    federation_id: String,

    #[serde(rename = "ServiceName")]
    service_name: String,

    #[serde(rename = "IsMasterService")]
    is_master_service: bool,
}

impl ServiceIdentity {
    /// Creates a new ServiceIdentity instance
    pub fn new(federation_id: String, service_name: String, is_master_service: bool) -> Self {
        Self {
            federation_id,
            service_name,
            is_master_service,
        }
    }

    /// Gets the federation ID
    pub fn federation_id(&self) -> &str {
        &self.federation_id
    }

    /// Gets the service name (URI)
    pub fn service_name(&self) -> &String {
        &self.service_name
    }

    /// Gets whether this is a master service
    pub fn is_master_service(&self) -> bool {
        self.is_master_service
    }

    /// Gets the application name by extracting the URI up to the last slash
    pub fn application_name(&self) -> String {
        let uri_str = self.service_name.as_str();
        if let Some(last_slash) = uri_str.rfind('/') {
            uri_str[..last_slash].to_string()
        } else {
            String::new()
        }
    }

    fn service_uri(&self) -> &String {
        &self.service_name
    }

    fn partition_key(&self) -> i64 {
        0
    }
}

impl fmt::Display for ServiceIdentity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FederationId:{},ServiceName:{},IsMasterService:{}",
            self.federation_id, self.service_name, self.is_master_service
        )
    }
}

impl std::hash::Hash for ServiceIdentity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.federation_id.to_lowercase().hash(state);
        self.service_name.as_str().to_lowercase().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_identity_creation() {
        let url = String::from("https://example.com/service");
        let identity = ServiceIdentity::new("fed123".to_string(), url.clone(), true);

        assert_eq!(identity.federation_id(), "fed123");
        assert_eq!(identity.service_name(), &url);
        assert!(identity.is_master_service());
    }

    #[test]
    fn application_name_validation() {
        let url = String::from("https://example.com/app/service");
        let identity = ServiceIdentity::new("fed123".to_string(), url, false);

        assert_eq!(identity.application_name(), "https://example.com/app");
    }

    #[test]
    fn display_validation() {
        let url = String::from("https://example.com/service");
        let identity = ServiceIdentity::new("fed123".to_string(), url, true);

        let display_str = format!("{}", identity);
        assert!(display_str.contains("FederationId:fed123"));
        assert!(display_str.contains("ServiceName:https://example.com/service"));
        assert!(display_str.contains("IsMasterService:true"));
    }
}
