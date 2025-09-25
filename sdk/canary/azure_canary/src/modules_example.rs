// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Examples of modules in Rust

/// A module for user-related functionality
pub mod user {
    /// User struct representing a user account
    #[derive(Debug, Clone)]
    pub struct User {
        pub username: String,
        pub email: String,
        pub is_active: bool,
    }

    impl User {
        /// Creates a new active user
        pub fn new(username: String, email: String) -> Self {
            Self {
                username,
                email,
                is_active: true,
            }
        }
    }

    /// A module for authentication-related functionality
    pub mod auth {
        /// Role enum for access control
        #[derive(Debug, Clone, PartialEq)]
        pub enum Role {
            Admin,
            User,
            Guest,
        }

        /// Function to verify access based on role
        pub fn verify_access(role: &Role, resource: &str) -> bool {
            match role {
                Role::Admin => true,
                Role::User => !resource.starts_with("admin/"),
                Role::Guest => resource.starts_with("public/"),
            }
        }
    }
}

/// A nested module structure demonstrating module hierarchy
pub mod networking {
    /// Represents a network connection
    pub struct Connection {
        pub address: String,
        pub port: u16,
        pub is_secure: bool,
    }

    pub mod protocols {
        /// HTTP protocol module
        pub mod http {
            /// HTTP request method enum
            #[derive(Debug, Clone, Copy)]
            pub enum Method {
                GET,
                POST,
                PUT,
                DELETE,
            }

            /// Function to create an HTTP request
            pub fn create_request(method: Method, path: &str) -> String {
                format!("{:?} {} HTTP/1.1", method, path)
            }
        }

        /// TCP protocol module
        pub mod tcp {
            use super::super::Connection;

            /// Function to establish a TCP connection
            pub fn connect(address: &str, port: u16) -> Connection {
                Connection {
                    address: address.to_string(),
                    port,
                    is_secure: false,
                }
            }
        }
    }
}
